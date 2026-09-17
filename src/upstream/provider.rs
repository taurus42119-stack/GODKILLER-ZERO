use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::domain::{
    AntiSpaghettiDirective, AstDiscoveryEngine, LinguisticTranspiler,
    QuantumSuperpositionSimulator,
};
use crate::upstream::bootstrap::{EngineStatus, OllamaBootstrapManager};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessagePayload {
    pub role: String,
    pub content: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionInboundRequest {
    pub model: String,
    pub messages: Vec<ChatMessagePayload>,
    #[serde(default)]
    pub stream: bool,
    #[serde(default)]
    pub temperature: Option<f32>,
    #[serde(flatten)]
    pub passthrough_fields: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionStreamChunk {
    pub id: String,
    pub object: &'static str,
    pub created: u64,
    pub model: String,
    pub choices: Vec<StreamChoiceDelta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChoiceDelta {
    pub index: u32,
    pub delta: DeltaContent,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeltaContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

#[async_trait]
pub trait UpstreamLlmGateway: Send + Sync {
    async fn forward_request(
        &self,
        request: ChatCompletionInboundRequest,
    ) -> Result<String, String>;
}

#[derive(Default, Clone)]
pub struct LocalZeroEngine {
    pub bootstrap_manager: Option<Arc<OllamaBootstrapManager>>,
}

impl LocalZeroEngine {
    pub fn new(bootstrap_manager: Option<Arc<OllamaBootstrapManager>>) -> Self {
        Self { bootstrap_manager }
    }
}

#[async_trait]
impl UpstreamLlmGateway for LocalZeroEngine {
    async fn forward_request(
        &self,
        request: ChatCompletionInboundRequest,
    ) -> Result<String, String> {
        let user_query = extract_latest_user_text(&request);

        if is_thai_or_english_greeting(&user_query) {
            return Ok(build_greeting_response());
        }

        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(3))
            .build()
            .unwrap_or_default();

        if let Ok(local_text) = try_local_llm_forward(&http_client, &request).await {
            return Ok(local_text);
        }

        if let Some(bootstrap_ref) = &self.bootstrap_manager {
            let current_status = bootstrap_ref.get_status().await;
            if let Some(progress_msg) = build_bootstrap_notice_if_busy(&current_status) {
                return Ok(progress_msg);
            }
        }

        Ok(synthesize_real_preflight_contract(&user_query))
    }
}

fn extract_latest_user_text(request: &ChatCompletionInboundRequest) -> String {
    request
        .messages
        .last()
        .map(|message| {
            if let Some(text_slice) = message.content.as_str() {
                text_slice.to_string()
            } else {
                message.content.to_string()
            }
        })
        .unwrap_or_else(|| "Empty intent".into())
}

fn is_thai_or_english_greeting(query: &str) -> bool {
    let lower = query.to_lowercase();
    let trimmed = lower.trim().trim_end_matches(['.', '!', '?', ' ']);
    matches!(
        trimmed,
        "hello"
            | "hi"
            | "hey"
            | "greetings"
            | "สวัสดี"
            | "สวัสดีครับ"
            | "สวัสดีค่ะ"
            | "หวัดดี"
            | "หวัดดีครับ"
            | "หวัดดีค่ะ"
            | "ดีครับ"
            | "ดีค่ะ"
    )
}

fn build_greeting_response() -> String {
    "⛩️ [GODKILLER ZERO]\n\
     Cognitive Pre-flight Firewall & Zero-Vibe Invariants Active (100% Local Loopback).\n\
     Awaiting instructions."
        .to_string()
}

async fn try_local_llm_forward(
    client: &reqwest::Client,
    request: &ChatCompletionInboundRequest,
) -> Result<String, ()> {
    let local_endpoints = [
        ("http://127.0.0.1:11434/v1/chat/completions", "Ollama"),
        ("http://127.0.0.1:1234/v1/chat/completions", "LM Studio"),
    ];

    for (endpoint, provider_name) in local_endpoints {
        if let Ok(network_resp) = client.post(endpoint).json(request).send().await {
            if network_resp.status().is_success() {
                if let Ok(json_envelope) = network_resp.json::<serde_json::Value>().await {
                    if let Some(choice_str) = json_envelope["choices"][0]["message"]["content"].as_str() {
                        tracing::info!("Forwarded to local {} engine successfully", provider_name);
                        return Ok(choice_str.to_string());
                    }
                }
            }
        }
    }
    Err(())
}



fn build_bootstrap_notice_if_busy(status: &EngineStatus) -> Option<String> {
    match status {
        EngineStatus::PullingModel {
            model,
            percent,
            status_text,
        } => Some(format!(
            "⏳ [GODKILLER ZERO : AI ENGINE INITIALIZING]\n\
             กำลังจัดเตรียมโมเดล AI บนเครื่องของคุณ: {} ({:.1}%)\n\
             สถานะปัจจุบัน: {}\n\n\
             กรุณารอสักครู่ เมื่อดาวน์โหลดเสร็จ คำสั่งถัดไปจะประมวลผลด้วยโมเดล Neural จริง 100% โดยอัตโนมัติ",
            model, percent, status_text
        )),
        EngineStatus::DownloadingInstaller { percent } => Some(format!(
            "⏳ [GODKILLER ZERO : SETUP IN PROGRESS]\n\
             กำลังดาวน์โหลดตัวติดตั้ง AI Engine บนเครื่องของคุณ ({:.1}%)\n\
             ระบบกำลังตั้งค่าให้อัตโนมัติ ไม่ต้องคลิกอะไรเพิ่มเติม",
            percent
        )),
        EngineStatus::StartingService | EngineStatus::InstallingOllama => Some(
            "⏳ [GODKILLER ZERO : STARTING LOCAL AI]\n\
             กำลังเริ่มการทำงานของ Service ในเบื้องหลัง กรุณารอสักครู่..."
                .to_string(),
        ),
        _ => None,
    }
}

fn synthesize_real_preflight_contract(raw_query: &str) -> String {
    let intent_analysis = LinguisticTranspiler::transpile(raw_query);
    let workspace_path = std::path::Path::new(".");
    let (discovered_coords, clarifiers) =
        AstDiscoveryEngine::discover_coordinates_and_clarifications(raw_query, workspace_path);

    let simulation_receipt = QuantumSuperpositionSimulator::simulate_superposition(
        raw_query,
        &AntiSpaghettiDirective::default(),
    );

    let contract_text = simulation_receipt
        .hoare_contract_spec
        .unwrap_or_else(|| "HOARE SPEC: Invariants Preserved".to_string());

    let mut target_files_section = String::new();
    if discovered_coords.is_empty() {
        target_files_section.push_str("• Scope: Active Editor Selection / Current Working Workspace\n");
    } else {
        for coord in &discovered_coords {
            target_files_section.push_str(&format!(
                "• Target: {} (Confidence: {:.0}%)\n",
                coord.file_path,
                coord.match_confidence * 100.0
            ));
        }
    }

    let mut clarifiers_section = String::new();
    for clarifier in clarifiers {
        clarifiers_section.push_str(&format!("  - [{}] {}\n", clarifier.action_id, clarifier.display_label));
    }

    format!(
        "/* ⛩️ [GODKILLER ZERO : COGNITIVE PRE-FLIGHT COMPILER CONTRACT] */\n\
         /* Mode: Standalone Semantic Synthesis (Offline Zero-Dependency Engine) */\n\n\
         [TARGET SCOPE & DISCOVERED COORDINATES]\n\
         {}\n\
         [TRANSPILER ACTION]: {}\n\
         [SUMMARY]: {}\n\n\
         {}\n\n\
         [GUARDRAILS & NEGATIVE INVARIANTS]\n\
         • Function Span: <= 70 lines\n\
         • Cyclomatic Complexity: <= 7\n\
         • Generic Identifiers Banned: data, res, req, temp, val, payload\n\
         • Ghost Edit Shield: Strictly preserve untouched functions\n\
         {}",
        target_files_section,
        intent_analysis.primary_action,
        intent_analysis.technical_action_summary,
        contract_text,
        if clarifiers_section.is_empty() {
            String::new()
        } else {
            format!("[SUGGESTED CLARIFICATIONS]\n{}", clarifiers_section)
        }
    )
}
