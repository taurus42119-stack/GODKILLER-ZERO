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
                Err(ref e) if e.kind() == io::ErrorKind::BrokenPipe || e.raw_os_error() == Some(232) || e.raw_os_error() == Some(109) => {
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
        let id = request.get("id")?;
        let method = request.get("method")?.as_str()?;

        match method {
            "initialize" => Some(Self::handle_initialize(id)),
            "tools/list" => Some(Self::handle_tools_list(id)),
            "tools/call" => Some(Self::handle_tools_call(id, request.get("params"))),
            "ping" => Some(json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {}
            })),
            _ => None,
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
        let tool_name = params.and_then(|p| p.get("name")).and_then(|n| n.as_str()).unwrap_or("");
        let arguments = params.and_then(|p| p.get("arguments")).cloned().unwrap_or(json!({}));

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

    fn handle_get_repo_map(id: &Value, arguments: &Value) -> Value {
        let workspace_path = arguments
            .get("workspace_path")
            .and_then(|p| p.as_str())
            .unwrap_or(".");
        let max_tokens = arguments
            .get("max_tokens")
            .and_then(|t| t.as_u64())
            .unwrap_or(2048) as usize;

        let report = crate::domain::RepoMapGenerator::generate(Path::new(workspace_path), max_tokens);
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
        let workspace_path = arguments
            .get("workspace_path")
            .and_then(|p| p.as_str())
            .unwrap_or(".");

        let audit = GatekeeperScanner::scan_path(Path::new(workspace_path), 70, 7);
        if audit.passed {
            json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "content": [
                        {
                            "type": "text",
                            "text": format!("✅ [CLAIM_DONE GRANTED] Gatekeeper verification passed. {} files scanned with zero invariant violations. Task concluded successfully.", audit.total_files_scanned)
                        }
                    ]
                }
            })
        } else {
            let mut violations_text = String::new();
            for (idx, v) in audit.violations.iter().take(10).enumerate() {
                violations_text.push_str(&format!("\n{}. [{}] {}:{} — {} ({})", idx + 1, v.rule_identifier, v.file_path, v.line_number, v.description, v.snippet));
            }
            if audit.violations.len() > 10 {
                violations_text.push_str(&format!("\n... and {} more violations", audit.violations.len() - 10));
            }

            json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "isError": true,
                    "content": [
                        {
                            "type": "text",
                            "text": format!("❌ [CLAIM_DONE REJECTED] Invariant violations detected on disk (Total: {}). You MUST refactor and fix all violations before concluding:{violations_text}", audit.violations.len())
                        }
                    ]
                }
            })
        }
    }

    fn handle_gatekeeper_scan(id: &Value, arguments: &Value) -> Value {
        let target_path = arguments
            .get("path")
            .and_then(|p| p.as_str())
            .unwrap_or(".");

        let audit = GatekeeperScanner::scan_path(Path::new(target_path), 70, 7);
        let summary = if audit.passed {
            format!("✓ [GATEKEEPER PASS] Scanned {} files. Zero invariant violations detected.", audit.total_files_scanned)
        } else {
            let mut details = format!("❌ [GATEKEEPER REJECT] {} violations detected across {} files:\n", audit.violations.len(), audit.total_files_scanned);
            for (idx, v) in audit.violations.iter().take(15).enumerate() {
                details.push_str(&format!("\n{}. [{}] {}:{} — {}", idx + 1, v.rule_identifier, v.file_path, v.line_number, v.description));
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
