use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use std::path::Path;

use crate::domain::GatekeeperScanner;

pub struct McpServer;

impl McpServer {
    pub fn run_stdio_loop() -> io::Result<()> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        let reader = stdin.lock();

        for line_result in reader.lines() {
            let line = match line_result {
                Ok(l) => l,
                Err(ref e)
                    if e.kind() == io::ErrorKind::BrokenPipe
                        || e.raw_os_error() == Some(232)
                        || e.raw_os_error() == Some(109) =>
                {
                    break;
                }
                Err(e) => return Err(e),
            };
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            let Ok(request) = serde_json::from_str::<Value>(trimmed) else {
                continue;
            };

            if let Some(response) = Self::handle_request(&request) {
                let serialized = serde_json::to_string(&response)?;
                writeln!(stdout, "{}", serialized)?;
                stdout.flush()?;
            }
        }

        Ok(())
    }

    fn handle_request(request: &Value) -> Option<Value> {
        let method = request.get("method")?.as_str()?;
        // JSON-RPC notifications have no id — never reply.
        let id = request.get("id")?;

        match method {
            "initialize" => Some(Self::handle_initialize(id)),
            "tools/list" => Some(Self::handle_tools_list(id)),
            "tools/call" => Some(Self::handle_tools_call(id, request.get("params"))),
            "prompts/list" => Some(json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": { "prompts": [] }
            })),
            "resources/list" => Some(json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": { "resources": [] }
            })),
            "resources/templates/list" => Some(json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": { "resourceTemplates": [] }
            })),
            "ping" => Some(json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {}
            })),
            // Never silent-drop a request with an id — clients (Antigravity) hang on refresh.
            _ => Some(json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": {
                    "code": -32601,
                    "message": format!("Method not found: {method}")
                }
            })),
        }
    }

    fn handle_initialize(id: &Value) -> Value {
        json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "godkiller-zero",
                    "version": "1.0.0"
                }
            }
        })
    }

    fn handle_tools_list(id: &Value) -> Value {
        json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "tools": [
                    {
                        "name": "gk_claim_done",
                        "description": "Mandatory Outbound Gatekeeper: AI must call this to claim completion. Verifies that all target code on disk adheres to Cyclomatic Complexity <= 7, Span <= 70 lines, zero generic identifiers, and zero lazy stubs.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "workspace_path": {
                                    "type": "string",
                                    "description": "Absolute path to workspace root or target project directory"
                                },
                                "completion_summary": {
                                    "type": "string",
                                    "description": "Short summary of changes made"
                                }
                            },
                            "required": ["workspace_path"]
                        }
                    },
                    {
                        "name": "gk_gatekeeper_scan",
                        "description": "Scan a project directory or file for code hygiene, complexity budget (CC <= 7), span limits (<= 70 lines), and lazy stubs (TODO, unwrap).",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "path": {
                                    "type": "string",
                                    "description": "Directory or file path to audit"
                                }
                            },
                            "required": ["path"]
                        }
                    },
                    {
                        "name": "gk_get_repo_map",
                        "description": "Generate an Aider-grade project skeleton map of the entire codebase. Returns grouped classes, structs, and function signatures with line numbers, compressed to minimal tokens to avoid dumping large files.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "workspace_path": {
                                    "type": "string",
                                    "description": "Path to workspace root directory (defaults to current directory)"
                                },
                                "max_tokens": {
                                    "type": "integer",
                                    "description": "Token budget cap for the repo map (default: 2048)"
                                }
                            }
                        }
                    }
                ]
            }
        })
    }

    fn handle_tools_call(id: &Value, params: Option<&Value>) -> Value {
        let tool_name = params
            .and_then(|p| p.get("name"))
            .and_then(|n| n.as_str())
            .unwrap_or("");
        let arguments = params
            .and_then(|p| p.get("arguments"))
            .cloned()
            .unwrap_or(json!({}));

        match tool_name {
            "gk_claim_done" => Self::handle_claim_done(id, &arguments),
            "gk_gatekeeper_scan" => Self::handle_gatekeeper_scan(id, &arguments),
            "gk_get_repo_map" => Self::handle_get_repo_map(id, &arguments),
            _ => json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": {
                    "code": -32601,
                    "message": format!("Tool not found: {}", tool_name)
                }
            }),
        }
    }

    /// Resolves a caller-supplied path argument. A missing or non-existent path
    /// is rejected instead of silently falling back to the server's own working
    /// directory, which would let a caller obtain a verification verdict for a
    /// tree it never named.
    fn resolve_required_path<'a>(
        arguments: &'a Value,
        argument_key: &str,
    ) -> Result<&'a str, String> {
        let supplied = arguments
            .get(argument_key)
            .and_then(|value| value.as_str())
            .map(str::trim)
            .unwrap_or_default();

        if supplied.is_empty() {
            return Err(format!(
                "Missing required argument '{}'. Supply an absolute path to the target project root.",
                argument_key
            ));
        }
        if !Path::new(supplied).exists() {
            return Err(format!(
                "Path '{}' does not exist on disk. Verification cannot be granted for an unresolvable target.",
                supplied
            ));
        }
        Ok(supplied)
    }

    fn invalid_arguments_response(id: &Value, message: &str) -> Value {
        json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": {
                "code": -32602,
                "message": message
            }
        })
    }

    fn handle_get_repo_map(id: &Value, arguments: &Value) -> Value {
        let workspace_path = arguments
            .get("workspace_path")
            .and_then(|p| p.as_str())
            .map(str::trim)
            .filter(|candidate| !candidate.is_empty())
            .unwrap_or(".");
        if !Path::new(workspace_path).exists() {
            return Self::invalid_arguments_response(
                id,
                &format!(
                    "Workspace path '{}' does not exist on disk.",
                    workspace_path
                ),
            );
        }
        let max_tokens = arguments
            .get("max_tokens")
            .and_then(|t| t.as_u64())
            .unwrap_or(2048) as usize;

        let report =
            crate::domain::RepoMapGenerator::generate(Path::new(workspace_path), max_tokens);
        json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "content": [
                    {
                        "type": "text",
                        "text": report.map_content
                    }
                ]
            }
        })
    }

    fn handle_claim_done(id: &Value, arguments: &Value) -> Value {
        let workspace_path = match Self::resolve_required_path(arguments, "workspace_path") {
            Ok(resolved) => resolved,
            Err(rejection) => return Self::invalid_arguments_response(id, &rejection),
        };

        let audit = GatekeeperScanner::scan_path(Path::new(workspace_path), 70, 7);
        if audit.total_files_scanned == 0 {
            return Self::invalid_arguments_response(
                id,
                &format!(
                    "No auditable source files found under '{}'. Completion cannot be claimed against an unverified tree.",
                    workspace_path
                ),
            );
        }
        if audit.passed {
            json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "content": [
                        {
                            "type": "text",
                            "text": format!("[OK] [CLAIM_DONE GRANTED] Gatekeeper verification passed. {} files scanned with zero invariant violations. Task concluded successfully.", audit.total_files_scanned)
                        }
                    ]
                }
            })
        } else {
            let mut violations_text = String::new();
            for (idx, v) in audit.violations.iter().take(10).enumerate() {
                violations_text.push_str(&format!(
                    "\n{}. [{}] {}:{} — {} ({})",
                    idx + 1,
                    v.rule_identifier,
                    v.file_path,
                    v.line_number,
                    v.description,
                    v.snippet
                ));
            }
            if audit.violations.len() > 10 {
                violations_text.push_str(&format!(
                    "\n... and {} more violations",
                    audit.violations.len() - 10
                ));
            }

            json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "isError": true,
                    "content": [
                        {
                            "type": "text",
                            "text": format!("[FAIL] [CLAIM_DONE REJECTED] Invariant violations detected on disk (Total: {}). You MUST refactor and fix all violations before concluding:{violations_text}", audit.violations.len())
                        }
                    ]
                }
            })
        }
    }

    fn handle_gatekeeper_scan(id: &Value, arguments: &Value) -> Value {
        let target_path = match Self::resolve_required_path(arguments, "path") {
            Ok(resolved) => resolved,
            Err(rejection) => return Self::invalid_arguments_response(id, &rejection),
        };

        let audit = GatekeeperScanner::scan_path(Path::new(target_path), 70, 7);
        let summary = if audit.passed {
            format!(
                "[OK] [GATEKEEPER PASS] Scanned {} files. Zero invariant violations detected.",
                audit.total_files_scanned
            )
        } else {
            let mut details = format!(
                "[FAIL] [GATEKEEPER REJECT] {} violations detected across {} files:\n",
                audit.violations.len(),
                audit.total_files_scanned
            );
            for (idx, v) in audit.violations.iter().take(15).enumerate() {
                details.push_str(&format!(
                    "\n{}. [{}] {}:{} — {}",
                    idx + 1,
                    v.rule_identifier,
                    v.file_path,
                    v.line_number,
                    v.description
                ));
            }
            details
        };

        json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "content": [
                    {
                        "type": "text",
                        "text": summary
                    }
                ]
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn call_tool(tool_name: &str, arguments: Value) -> Value {
        McpServer::handle_tools_call(
            &json!(1),
            Some(&json!({ "name": tool_name, "arguments": arguments })),
        )
    }

    #[test]
    fn test_claim_done_rejects_missing_workspace_path() {
        let response = call_tool("gk_claim_done", json!({ "completion_summary": "all done" }));
        assert!(
            response.get("error").is_some(),
            "an unnamed target must not receive a completion verdict: {response}"
        );
        assert!(response.get("result").is_none());
    }

    #[test]
    fn test_claim_done_rejects_nonexistent_workspace_path() {
        let response = call_tool(
            "gk_claim_done",
            json!({ "workspace_path": "Z:/definitely/not/here" }),
        );
        assert_eq!(response["error"]["code"], -32602);
    }

    #[test]
    fn test_gatekeeper_scan_rejects_missing_path() {
        let response = call_tool("gk_gatekeeper_scan", json!({}));
        assert_eq!(response["error"]["code"], -32602);
    }

    #[test]
    fn test_prompts_list_returns_empty_array() {
        let response = McpServer::handle_request(&json!({
            "jsonrpc": "2.0",
            "id": 7,
            "method": "prompts/list"
        }))
        .expect("prompts/list must reply");
        assert!(response.get("error").is_none(), "{response}");
        assert_eq!(response["result"]["prompts"], json!([]));
    }

    #[test]
    fn test_resources_list_returns_empty_array() {
        let response = McpServer::handle_request(&json!({
            "jsonrpc": "2.0",
            "id": 8,
            "method": "resources/list"
        }))
        .expect("resources/list must reply");
        assert_eq!(response["result"]["resources"], json!([]));
    }

    #[test]
    fn test_unknown_method_returns_error_instead_of_silence() {
        let response = McpServer::handle_request(&json!({
            "jsonrpc": "2.0",
            "id": 9,
            "method": "does/not/exist"
        }))
        .expect("unknown methods with id must reply");
        assert_eq!(response["error"]["code"], -32601);
    }

    #[test]
    fn test_notification_without_id_is_ignored() {
        let response = McpServer::handle_request(&json!({
            "jsonrpc": "2.0",
            "method": "notifications/initialized"
        }));
        assert!(response.is_none());
    }
}
