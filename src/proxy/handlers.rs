use crate::domain::{
    AntiSpaghettiDirective, IngressEvaluationVerdict, InvariantContractEvaluator, PruneResult,
    TerminalPruner, TriPillarEvaluator,
};
use crate::proxy::sanitizer::{EgressFluffStripper, IngressSecuritySanitizer};
use crate::upstream::{ChatCompletionInboundRequest, OllamaBootstrapManager, UpstreamLlmGateway};
use axum::{
    body::Body,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct ProxyRuntimeState {
    pub upstream_gateway: Arc<dyn UpstreamLlmGateway>,
    pub anti_spaghetti_directive: AntiSpaghettiDirective,
    pub tokens_preserved_counter: Arc<AtomicU64>,
    pub halts_enforced_counter: Arc<AtomicU64>,
    pub requests_intercepted_counter: Arc<AtomicU64>,
    pub interpreter_active: Arc<AtomicBool>,
    pub bootstrap_manager: Option<Arc<OllamaBootstrapManager>>,
    /// Port the loopback listener was actually bound to, so telemetry reports the
    /// live endpoint instead of the documented default.
    pub bound_port: u16,
}

pub async fn list_models_handler() -> Json<serde_json::Value> {
    Json(json!({
        "object": "list",
        "data": [
            {
                "id": "godkiller-zero",
                "object": "model",
                "created": 1773489600,
                "owned_by": "godkiller-zero-local"
            }
        ]
    }))
}

pub async fn chat_completions_handler(
    State(runtime_state): State<ProxyRuntimeState>,
    headers: HeaderMap,
    Json(mut inbound_invocation_envelope): Json<ChatCompletionInboundRequest>,
) -> Result<Response, (StatusCode, String)> {
    runtime_state
        .requests_intercepted_counter
        .fetch_add(1, Ordering::Relaxed);

    let origin_str = headers.get("origin").and_then(|h| h.to_str().ok());
    if IngressSecuritySanitizer::is_browser_origin_forbidden(origin_str) {
        return Err((
            StatusCode::FORBIDDEN,
            "Cross-Origin browser requests to GODKILLER ZERO are rejected for security.".into(),
        ));
    }

    let target_message_index = find_user_message_index(&inbound_invocation_envelope)?;

    if runtime_state.interpreter_active.load(Ordering::Relaxed) {
        if let Some(immediate_response) = process_interpreter_enrichment(
            &mut inbound_invocation_envelope,
            target_message_index,
            &runtime_state,
        ) {
            return Ok(immediate_response);
        }
    }

    let is_streaming = inbound_invocation_envelope.stream;
    let target_model_identifier = inbound_invocation_envelope.model.clone();

    let upstream_result = runtime_state
        .upstream_gateway
        .forward_request(inbound_invocation_envelope)
        .await
        .map_err(|gateway_error| (StatusCode::BAD_GATEWAY, gateway_error))?;

    let purified_content = EgressFluffStripper::strip_conversational_fluff(&upstream_result);

    Ok(format_immediate_text_response(
        &target_model_identifier,
        &purified_content,
        is_streaming,
    ))
}

fn process_interpreter_enrichment(
    envelope: &mut ChatCompletionInboundRequest,
    target_message_index: usize,
    runtime_state: &ProxyRuntimeState,
) -> Option<Response> {
    let unprocessed_prompt_stream =
        extract_prompt_stream(&envelope.messages[target_message_index].content);
    let sanitized_prompt_stream =
        IngressSecuritySanitizer::sanitize_prompt_tokens(&unprocessed_prompt_stream);
    let (isolated_request_opt, is_antigravity) =
        IngressSecuritySanitizer::extract_isolated_user_request(&sanitized_prompt_stream);
    let prompt_to_evaluate = isolated_request_opt
        .as_deref()
        .unwrap_or(&sanitized_prompt_stream);

    let fast_evaluation = TriPillarEvaluator::evaluate_prompt(prompt_to_evaluate);
    if fast_evaluation == IngressEvaluationVerdict::ConversationalGreeting {
        let greeting_reply =
            "สวัสดีครับ GODKILLER ZERO ประจำการอยู่บนเครื่อง Local ของคุณแล้ว (0 Cloud Tokens)\n\
             พร้อมเป็นล่ามแปลภาษาและตีกรอบบริบทสำหรับงานพัฒนาโค้ด\n\
             ระบุงานที่ต้องการได้เลยครับ เช่น \"แก้ปุ่มหน้าแรก\" หรือวาง Cursor ไว้ที่โค้ดใน Editor";
        return Some(format_immediate_text_response(
            &envelope.model,
            greeting_reply,
            envelope.stream,
        ));
    }

    if fast_evaluation != IngressEvaluationVerdict::BypassFastLane {
        enrich_envelope_with_compiled_contract(
            envelope,
            target_message_index,
            prompt_to_evaluate,
            &sanitized_prompt_stream,
            unprocessed_prompt_stream.len(),
            is_antigravity,
            runtime_state,
        );
    }

    None
}

fn find_user_message_index(
    envelope: &ChatCompletionInboundRequest,
) -> Result<usize, (StatusCode, String)> {
    for (message_sequence_index, user_message) in envelope.messages.iter().enumerate().rev() {
        if user_message.role == "user" {
            return Ok(message_sequence_index);
        }
    }
    Err((
        StatusCode::BAD_REQUEST,
        "Request must contain at least one message with role 'user'".into(),
    ))
}

fn extract_prompt_stream(content_value: &serde_json::Value) -> String {
    content_value.as_str().unwrap_or("").to_string()
}

fn enrich_envelope_with_compiled_contract(
    envelope: &mut ChatCompletionInboundRequest,
    target_message_index: usize,
    prompt_to_evaluate: &str,
    sanitized_prompt_stream: &str,
    original_length: usize,
    is_antigravity: bool,
    runtime_state: &ProxyRuntimeState,
) {
    let simulation_receipt = InvariantContractEvaluator::evaluate(
        prompt_to_evaluate,
        &runtime_state.anti_spaghetti_directive,
    );

    if !simulation_receipt.dispatch_allowed {
        runtime_state
            .halts_enforced_counter
            .fetch_add(1, Ordering::Relaxed);
    }

    let contract_spec_opt = simulation_receipt
        .diagnostic_spec
        .as_ref()
        .or(simulation_receipt.contract_spec.as_ref());

    if let Some(target_contract_spec) = contract_spec_opt {
        let compressed_text = if is_antigravity {
            IngressSecuritySanitizer::inject_compiled_request_into_antigravity_envelope(
                sanitized_prompt_stream,
                target_contract_spec,
            )
        } else {
            format!(
                "{}\n\n/* [GODKILLER ZERO COMPLIANCE CONTRACT] */\n{}",
                sanitized_prompt_stream, target_contract_spec
            )
        };

        let compressed_length = target_contract_spec.len();
        if original_length > compressed_length {
            let saved_estimated_tokens = ((original_length - compressed_length) / 4) as u64;
            runtime_state
                .tokens_preserved_counter
                .fetch_add(saved_estimated_tokens, Ordering::Relaxed);
        }

        envelope.messages[target_message_index].content =
            serde_json::Value::String(compressed_text);
    }
}

fn format_immediate_text_response(model: &str, content: &str, stream: bool) -> Response {
    let now_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    if stream {
        let chunk_json = json!({
            "id": format!("chatcmpl-gk-{}", now_epoch),
            "object": "chat.completion.chunk",
            "created": now_epoch,
            "model": model,
            "choices": [{
                "index": 0,
                "delta": {
                    "role": "assistant",
                    "content": content
                },
                "finish_reason": "stop"
            }]
        });

        let sse_body = format!("data: {}\n\ndata: [DONE]\n\n", chunk_json);

        Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "text/event-stream")
            .header("Cache-Control", "no-cache")
            .header("Connection", "keep-alive")
            .body(Body::from(sse_body))
            .unwrap_or_else(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "SSE response build failed",
                )
                    .into_response()
            })
    } else {
        let prompt_tokens = (content.len() / 6).max(12);
        let completion_tokens = (content.len() / 4).max(1);
        let completion_json = json!({
            "id": format!("chatcmpl-gk-{}", now_epoch),
            "object": "chat.completion",
            "created": now_epoch,
            "model": model,
            "choices": [{
                "index": 0,
                "message": {
                    "role": "assistant",
                    "content": content
                },
                "finish_reason": "stop"
            }],
            "usage": {
                "prompt_tokens": prompt_tokens,
                "completion_tokens": completion_tokens,
                "total_tokens": prompt_tokens + completion_tokens
            }
        });

        Json(completion_json).into_response()
    }
}

pub async fn metrics_handler(State(state): State<ProxyRuntimeState>) -> Json<serde_json::Value> {
    let is_hooked = crate::antigravity::is_antigravity_hooked();
    Json(json!({
        "status": "ACTIVE",
        "port": state.bound_port,
        "antigravity_hooked": is_hooked,
        "metrics": {
            "requests_intercepted": state.requests_intercepted_counter.load(Ordering::Relaxed),
            "tokens_preserved": state.tokens_preserved_counter.load(Ordering::Relaxed),
            "halts_enforced": state.halts_enforced_counter.load(Ordering::Relaxed),
        }
    }))
}

#[derive(serde::Deserialize, Default)]
pub struct HookRequestPayload {
    pub discipline: Option<String>,
    pub rules: Option<crate::antigravity::InvariantRulesSelection>,
}

#[derive(serde::Deserialize)]
pub struct StartupTogglePayload {
    pub enabled: bool,
}

#[derive(serde::Deserialize)]
pub struct PurifyRequestPayload {
    pub prompt: String,
    pub workspace_path: Option<String>,
}

pub async fn default_workspace_api_handler() -> Json<serde_json::Value> {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let project_name = current_dir
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(".");

    let is_git = std::path::Path::new(".git").exists();

    Json(json!({
        "current_workspace": project_name,
        "is_git": is_git
    }))
}

pub async fn get_startup_api_handler() -> Json<serde_json::Value> {
    let startup_enabled = crate::antigravity::query_windows_startup_status();
    Json(json!({
        "enabled": startup_enabled
    }))
}

pub async fn toggle_startup_api_handler(
    Json(toggle_config): Json<StartupTogglePayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match crate::antigravity::configure_windows_startup(toggle_config.enabled) {
        Ok(success) => Ok(Json(json!({
            "success": success,
            "enabled": toggle_config.enabled
        }))),
        Err(failure) => Err((StatusCode::INTERNAL_SERVER_ERROR, failure)),
    }
}

pub async fn hook_status_api_handler() -> Json<crate::antigravity::HookStateDetails> {
    Json(crate::antigravity::query_antigravity_hook_state())
}

pub async fn inspect_rules_api_handler() -> Json<serde_json::Value> {
    let hook_state = crate::antigravity::query_antigravity_hook_state();
    let rule_content = crate::antigravity::generate_custom_antigravity_rule_block(
        &hook_state.discipline,
        Some(&hook_state.rules),
    );
    Json(json!({
        "hooked": hook_state.hooked,
        "discipline": hook_state.discipline,
        "rules": hook_state.rules,
        "content": rule_content
    }))
}

pub async fn hook_antigravity_api_handler(
    invocation_parameters: Option<Json<HookRequestPayload>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let hook_config = invocation_parameters.map(|p| p.0).unwrap_or_default();
    let discipline_mode = hook_config.discipline.unwrap_or_else(|| "KEN".to_string());

    match crate::antigravity::hook_antigravity_with_rules(
        &discipline_mode,
        hook_config.rules.as_ref(),
    ) {
        Ok(hook_dispatch_receipt) => Ok(Json(json!({
            "success": true,
            "message": hook_dispatch_receipt.message,
            "path": hook_dispatch_receipt.path_modified.to_string_lossy(),
        }))),
        Err(hook_execution_fault) => Err((StatusCode::INTERNAL_SERVER_ERROR, hook_execution_fault)),
    }
}

pub async fn unhook_antigravity_api_handler(
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match crate::antigravity::unhook_antigravity() {
        Ok(unhook_dispatch_receipt) => Ok(Json(json!({
            "success": true,
            "message": unhook_dispatch_receipt.message,
            "path": unhook_dispatch_receipt.path_modified.to_string_lossy(),
        }))),
        Err(unhook_execution_fault) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, unhook_execution_fault))
        }
    }
}

pub async fn quit_app_api_handler() -> Json<serde_json::Value> {
    tokio::spawn(async {
        tokio::time::sleep(std::time::Duration::from_millis(150)).await;
        std::process::exit(0);
    });
    Json(json!({
        "success": true,
        "message": "GODKILLER ZERO shutting down cleanly."
    }))
}

pub async fn purify_prompt_api_handler(
    State(runtime_state): State<ProxyRuntimeState>,
    Json(purification_target_envelope): Json<PurifyRequestPayload>,
) -> Json<serde_json::Value> {
    let sanitized_lexical_input =
        IngressSecuritySanitizer::sanitize_prompt_tokens(&purification_target_envelope.prompt);

    let target_workspace = purification_target_envelope
        .workspace_path
        .as_deref()
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from("."));

    let transpiled =
        crate::domain::LinguisticTranspiler::transpile_async(&sanitized_lexical_input).await;

    let (discovered_coords, clarifier_options) =
        crate::domain::AstDiscoveryEngine::discover_with_tokens(
            &sanitized_lexical_input,
            &target_workspace,
            &transpiled.candidate_search_tokens,
        );

    if let Some(early_exit_envelope) = evaluate_early_purify_exit(
        &purification_target_envelope.prompt,
        &sanitized_lexical_input,
        &transpiled,
        &clarifier_options,
        &discovered_coords,
    ) {
        return Json(early_exit_envelope);
    }

    let simulation_receipt = InvariantContractEvaluator::evaluate(
        &sanitized_lexical_input,
        &runtime_state.anti_spaghetti_directive,
    );

    let stack_context = crate::domain::UniversalStackSensor::inspect_workspace(&target_workspace);
    let symbol_impact = transpiled
        .candidate_search_tokens
        .iter()
        .find_map(|token_slice| {
            crate::domain::PerProjectSymbolGraph::locate_symbol_and_impact(
                token_slice,
                &target_workspace,
            )
        });

    let response_envelope = build_purified_simulation_response(
        &purification_target_envelope,
        &transpiled,
        &simulation_receipt,
        &stack_context,
        symbol_impact,
        clarifier_options,
        discovered_coords,
    );
    Json(response_envelope)
}

fn evaluate_early_purify_exit(
    prompt_text: &str,
    sanitized_input: &str,
    transpiled: &crate::domain::TranspiledIntent,
    clarifier_options: &[crate::domain::ClarifierOption],
    discovered_coords: &[crate::domain::DiscoveredCoordinate],
) -> Option<serde_json::Value> {
    if transpiled.circuit_breaker_triggered {
        return Some(build_circuit_breaker_purify_envelope(
            sanitized_input,
            transpiled,
            clarifier_options.to_vec(),
            discovered_coords.to_vec(),
        ));
    }

    if transpiled.is_greeting {
        return Some(build_greeting_purify_envelope(
            transpiled,
            clarifier_options.to_vec(),
            discovered_coords.to_vec(),
        ));
    }

    let fast_evaluation = TriPillarEvaluator::evaluate_prompt(sanitized_input);
    if fast_evaluation == IngressEvaluationVerdict::BypassFastLane {
        return Some(build_fast_lane_purify_envelope(
            prompt_text,
            clarifier_options.to_vec(),
            discovered_coords.to_vec(),
        ));
    }

    None
}

fn build_greeting_purify_envelope(
    transpiled: &crate::domain::TranspiledIntent,
    clarifiers: Vec<crate::domain::ClarifierOption>,
    coords: Vec<crate::domain::DiscoveredCoordinate>,
) -> serde_json::Value {
    json!({
        "verdict": "CONVERSATIONAL_BYPASS",
        "dense_ir": transpiled.concise_english,
        "concise_english": transpiled.concise_english,
        "branches": [],
        "quick_fixes": [],
        "clarifiers": clarifiers,
        "discovered_coordinates": coords,
        "tokens_saved": 0,
        "is_greeting": true,
    })
}

fn build_circuit_breaker_purify_envelope(
    sanitized_input: &str,
    transpiled: &crate::domain::TranspiledIntent,
    clarifiers: Vec<crate::domain::ClarifierOption>,
    coords: Vec<crate::domain::DiscoveredCoordinate>,
) -> serde_json::Value {
    let breaker_spec = crate::domain::HoareContract::render_circuit_breaker_ir(
        "RepeatedFailureScope",
        sanitized_input,
    );
    json!({
        "verdict": "CIRCUIT_BREAKER_LOCKDOWN",
        "dense_ir": breaker_spec,
        "concise_english": transpiled.technical_action_summary,
        "circuit_breaker": true,
        "branches": [],
        "quick_fixes": [],
        "clarifiers": clarifiers,
        "discovered_coordinates": coords,
        "tokens_saved": 0,
    })
}

fn build_fast_lane_purify_envelope(
    prompt_text: &str,
    clarifiers: Vec<crate::domain::ClarifierOption>,
    coords: Vec<crate::domain::DiscoveredCoordinate>,
) -> serde_json::Value {
    json!({
        "verdict": "FAST_LANE",
        "dense_ir": prompt_text,
        "concise_english": prompt_text,
        "branches": [],
        "quick_fixes": [],
        "clarifiers": clarifiers,
        "discovered_coordinates": coords,
        "tokens_saved": 0,
    })
}

fn resolve_target_coordinate_and_blast(
    symbol_impact: &Option<(
        crate::domain::SymbolNode,
        crate::domain::BlastRadiusAnalysis,
    )>,
    discovered_coords: &[crate::domain::DiscoveredCoordinate],
    detected_components: &[String],
) -> (String, Vec<String>) {
    if let Some((symbol_node, blast_analysis)) = symbol_impact {
        (
            format!(
                "{}:L{} (<{}>)",
                symbol_node.relative_path, symbol_node.line_number, symbol_node.symbol_name
            ),
            blast_analysis.direct_consumers.clone(),
        )
    } else if let Some(top_coord) = discovered_coords.first() {
        (top_coord.file_path.clone(), Vec::new())
    } else if !detected_components.is_empty() {
        (detected_components.join(" / "), Vec::new())
    } else {
        ("Active workspace focus".to_string(), Vec::new())
    }
}

#[allow(clippy::too_many_arguments)]
fn render_allowed_purify_payload(
    simulation_receipt: &crate::domain::ContractEvaluationReceipt,
    clarifier_options: Vec<crate::domain::ClarifierOption>,
    discovered_coords: Vec<crate::domain::DiscoveredCoordinate>,
    dense_ir_output: &str,
    hoare_contract_output: &str,
    transpiled: &crate::domain::TranspiledIntent,
    stack_context: &crate::domain::ProjectStackContext,
    blast_consumers: &[String],
    tokens_saved: usize,
    original_chars: usize,
) -> serde_json::Value {
    json!({
        "verdict": "DISPATCH_ALLOWED",
        "satisfied_branch_ratio": simulation_receipt.satisfied_branch_ratio,
        "branches": simulation_receipt.branches,
        "quick_fixes": simulation_receipt.quick_fixes,
        "clarifiers": clarifier_options,
        "discovered_coordinates": discovered_coords,
        "dense_ir": transpiled.concise_english,
        "formal_contract": dense_ir_output,
        "hoare_ir": hoare_contract_output,
        "concise_english": transpiled.concise_english,
        "detected_components": transpiled.detected_components,
        "styling_tokens": transpiled.styling_tokens,
        "stack": stack_context,
        "blast_radius": blast_consumers,
        "tokens_saved": tokens_saved,
        "original_chars": original_chars,
        "compiled_chars": transpiled.concise_english.len(),
    })
}

fn compute_dense_ir_output(
    transpiled: &crate::domain::TranspiledIntent,
    target_coordinate: &str,
    styling_paradigm: &str,
    blast_consumers: &[String],
) -> String {
    if transpiled.is_negated {
        crate::domain::HoareContract::render_negation_dense_ir(
            target_coordinate,
            &transpiled.technical_action_summary,
            styling_paradigm,
            blast_consumers,
        )
    } else {
        crate::domain::HoareContract::render_context_aware_dense_ir(
            target_coordinate,
            &transpiled.technical_action_summary,
            styling_paradigm,
            blast_consumers,
        )
    }
}

fn build_purified_simulation_response(
    purification_target_envelope: &PurifyRequestPayload,
    transpiled: &crate::domain::TranspiledIntent,
    simulation_receipt: &crate::domain::ContractEvaluationReceipt,
    stack_context: &crate::domain::ProjectStackContext,
    symbol_impact: Option<(
        crate::domain::SymbolNode,
        crate::domain::BlastRadiusAnalysis,
    )>,
    clarifier_options: Vec<crate::domain::ClarifierOption>,
    discovered_coords: Vec<crate::domain::DiscoveredCoordinate>,
) -> serde_json::Value {
    if simulation_receipt.dispatch_allowed {
        let (target_coordinate, blast_consumers) = resolve_target_coordinate_and_blast(
            &symbol_impact,
            &discovered_coords,
            &transpiled.detected_components,
        );

        let dense_ir_output = compute_dense_ir_output(
            transpiled,
            &target_coordinate,
            &stack_context.styling_paradigm,
            &blast_consumers,
        );

        let hoare_contract_output = simulation_receipt.contract_spec.clone().unwrap_or_default();

        let original_chars = purification_target_envelope.prompt.len();
        let concise_chars = transpiled.concise_english.len();
        let tokens_saved = if original_chars > concise_chars {
            (original_chars - concise_chars) / 4
        } else {
            0
        };

        render_allowed_purify_payload(
            simulation_receipt,
            clarifier_options,
            discovered_coords,
            &dense_ir_output,
            &hoare_contract_output,
            transpiled,
            stack_context,
            &blast_consumers,
            tokens_saved,
            original_chars,
        )
    } else {
        json!({
            "verdict": "HALT_INQUISITIVE",
            "satisfied_branch_ratio": simulation_receipt.satisfied_branch_ratio,
            "branches": simulation_receipt.branches,
            "quick_fixes": simulation_receipt.quick_fixes,
            "clarifiers": clarifier_options,
            "discovered_coordinates": discovered_coords,
            "dense_ir": simulation_receipt.diagnostic_spec,
            "message": simulation_receipt
                .technical_action_summary
                .as_deref()
                .unwrap_or("Contract evaluation halted before dispatch."),
        })
    }
}

#[derive(Debug, serde::Deserialize, Default)]
pub struct ProjectConfigExportPayload {
    pub discipline: Option<String>,
    pub rules: Option<crate::antigravity::InvariantRulesSelection>,
    pub workspace_path: Option<String>,
}

fn validate_safe_workspace_directory(
    candidate_path: Option<&str>,
) -> Result<std::path::PathBuf, (StatusCode, String)> {
    let raw_path = candidate_path.unwrap_or(".").trim();
    let effective_path = if raw_path.is_empty() { "." } else { raw_path };
    let target = std::path::PathBuf::from(effective_path);

    let canonical = target.canonicalize().map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            format!(
                "Target workspace '{}' does not exist or is not accessible.",
                target.display()
            ),
        )
    })?;

    if !canonical.is_dir() {
        return Err((
            StatusCode::BAD_REQUEST,
            format!(
                "Target workspace '{}' is not a directory.",
                canonical.display()
            ),
        ));
    }

    let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let cwd_canonical = cwd.canonicalize().unwrap_or(cwd);

    let is_within_cwd = canonical.starts_with(&cwd_canonical);
    let is_project_repo = canonical.join(".git").exists() || canonical.join(".agents").exists();

    if !is_within_cwd && !is_project_repo {
        return Err((
            StatusCode::FORBIDDEN,
            format!(
                "Access denied: '{}' is outside active project boundary and lacks project markers (.git/.agents).",
                canonical.display()
            ),
        ));
    }

    Ok(canonical)
}

pub async fn install_gate_api_handler(
    body: axum::body::Bytes,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let raw_workspace = if !body.is_empty() {
        serde_json::from_slice::<serde_json::Value>(&body)
            .ok()
            .and_then(|val| {
                val.get("workspace_path")
                    .and_then(|p| p.as_str())
                    .map(String::from)
            })
    } else {
        None
    };

    let target_dir = validate_safe_workspace_directory(raw_workspace.as_deref())?;

    match crate::domain::GatekeeperScanner::install_git_pre_commit_hook(&target_dir) {
        Ok(path) => Ok(Json(json!({
            "success": true,
            "message": format!("Git pre-commit gatekeeper installed at {}", path.display())
        }))),
        Err(err) => Err((StatusCode::BAD_REQUEST, err)),
    }
}

pub async fn export_project_config_api_handler(
    Json(export_config): Json<ProjectConfigExportPayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let target_dir = validate_safe_workspace_directory(export_config.workspace_path.as_deref())?;

    let config_json = serde_json::to_string_pretty(&json!({
        "project": "godkiller-zero",
        "discipline": export_config.discipline.unwrap_or_else(|| "KEN".to_string()),
        "rules": export_config.rules.unwrap_or_else(|| crate::antigravity::InvariantRulesSelection::for_discipline("KEN")),
    })).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let out_file = target_dir.join(".godkiller.json");
    std::fs::write(&out_file, config_json).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to write {}: {}", out_file.display(), e),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": format!("Saved project configuration to {}", out_file.display())
    })))
}

pub async fn interpreter_status_api_handler(
    State(runtime_state): State<ProxyRuntimeState>,
) -> Json<serde_json::Value> {
    let is_active = runtime_state.interpreter_active.load(Ordering::Relaxed);
    Json(json!({
        "status": "ok",
        "interpreter_active": is_active
    }))
}

pub async fn toggle_interpreter_api_handler(
    State(runtime_state): State<ProxyRuntimeState>,
) -> Json<serde_json::Value> {
    let previous_state = runtime_state
        .interpreter_active
        .fetch_xor(true, Ordering::Relaxed);
    let new_state = !previous_state;

    if new_state {
        let active_discipline = match runtime_state
            .anti_spaghetti_directive
            .max_cyclomatic_complexity
        {
            10 => "SHI",
            5 => "SHIN",
            _ => {
                let hook_state = crate::antigravity::query_antigravity_hook_state();
                match hook_state.discipline.to_uppercase().as_str() {
                    "SHI" => "SHI",
                    "SHIN" => "SHIN",
                    _ => "KEN",
                }
            }
        };
        let _ = crate::antigravity::hook_antigravity(active_discipline);
    } else {
        let _ = crate::antigravity::unhook_antigravity();
    }

    Json(json!({
        "status": "ok",
        "interpreter_active": new_state
    }))
}

pub async fn engine_status_api_handler(
    State(runtime_state): State<ProxyRuntimeState>,
) -> Json<serde_json::Value> {
    if let Some(bootstrap) = &runtime_state.bootstrap_manager {
        let current_engine_status = bootstrap.get_status().await;
        Json(json!({
            "target_model": bootstrap.target_model_name(),
            "status": current_engine_status,
        }))
    } else {
        Json(json!({
            "target_model": "qwen2.5-coder:1.5b",
            "status": {
                "state": "OfflineStandalone",
                "details": { "reason": "Standalone Engine Active" }
            }
        }))
    }
}

pub async fn engine_bootstrap_api_handler(
    State(runtime_state): State<ProxyRuntimeState>,
) -> Json<serde_json::Value> {
    if let Some(bootstrap) = &runtime_state.bootstrap_manager {
        bootstrap.clone().start_background_bootstrap();
        Json(json!({ "success": true, "message": "Bootstrap sequence initiated" }))
    } else {
        Json(json!({ "success": false, "message": "No bootstrap manager instance active" }))
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct PruneTerminalRequestPayload {
    pub raw_output: String,
}

pub async fn prune_terminal_api_handler(
    State(runtime_state): State<ProxyRuntimeState>,
    Json(prune_request): Json<PruneTerminalRequestPayload>,
) -> Json<PruneResult> {
    let prune_verdict = TerminalPruner::prune(&prune_request.raw_output);
    if prune_verdict.original_tokens_est > prune_verdict.pruned_tokens_est {
        let saved = (prune_verdict.original_tokens_est - prune_verdict.pruned_tokens_est) as u64;
        runtime_state
            .tokens_preserved_counter
            .fetch_add(saved, Ordering::Relaxed);
    }
    Json(prune_verdict)
}
