use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranspiledIntent {
    pub primary_action: String,
    pub detected_components: Vec<String>,
    pub styling_tokens: Vec<String>,
    pub technical_action_summary: String,
    pub candidate_search_tokens: Vec<String>,
    pub is_negated: bool,
    pub circuit_breaker_triggered: bool,
    #[serde(default)]
    pub concise_english: String,
    #[serde(default)]
    pub is_greeting: bool,
}

pub struct LinguisticTranspiler;

const LOCAL_LLM_MODEL: &str = "qwen2.5-coder:1.5b";
const OLLAMA_CHAT_ENDPOINT: &str = "http://127.0.0.1:11434/api/chat";

const SYSTEM_TRANSLATOR_PROMPT: &str = "\
You are an expert technical translator. \
Translate any Thai input directly into natural, concise English for software developers and AI assistants. \
Keep negative constraints like 'อย่า' or 'ห้าม' as 'Do not modify/touch'. \
For casual greetings, translate naturally. \
Return ONLY the direct English translation without explanations, quotes, or markdown.";

impl LinguisticTranspiler {
    /// Synchronous transpilation entry point using Local LLM (Ollama qwen2.5-coder).
    #[must_use]
    pub fn transpile(raw_input: &str) -> TranspiledIntent {
        let normalized = raw_input.trim();
        if normalized.is_empty() {
            return Self::empty_intent();
        }

        if Self::detect_circuit_breaker(normalized) {
            return Self::circuit_breaker_intent();
        }

        // Call Local LLM via blocking HTTP
        match Self::query_local_llm_sync(normalized) {
            Ok(translated_text) if !translated_text.is_empty() => {
                Self::build_intent_from_llm(normalized, &translated_text)
            }
            _ => {
                // If Local LLM is bootstrapping or unreachable, pass through cleanly
                Self::build_passthrough_intent(normalized)
            }
        }
    }

    /// Asynchronous transpilation entry point using Local LLM (Ollama qwen2.5-coder).
    pub async fn transpile_async(raw_input: &str) -> TranspiledIntent {
        let normalized = raw_input.trim();
        if normalized.is_empty() {
            return Self::empty_intent();
        }

        if Self::detect_circuit_breaker(normalized) {
            return Self::circuit_breaker_intent();
        }

        match Self::query_local_llm_async(normalized).await {
            Ok(translated_text) if !translated_text.is_empty() => {
                Self::build_intent_from_llm(normalized, &translated_text)
            }
            _ => Self::build_passthrough_intent(normalized),
        }
    }

    fn query_local_llm_sync(prompt: &str) -> Result<String, String> {
        if tokio::runtime::Handle::try_current().is_ok() {
            let prompt_owned = prompt.to_string();
            return std::thread::spawn(move || Self::query_local_llm_sync_direct(&prompt_owned))
                .join()
                .map_err(|_| "Transpilation worker thread failed".to_string())?;
        }
        Self::query_local_llm_sync_direct(prompt)
    }

    fn query_local_llm_sync_direct(prompt: &str) -> Result<String, String> {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(12))
            .build()
            .map_err(|e| e.to_string())?;

        let chat_prompt_envelope = serde_json::json!({
            "model": LOCAL_LLM_MODEL,
            "messages": [
                { "role": "system", "content": SYSTEM_TRANSLATOR_PROMPT },
                { "role": "user", "content": "ช่วยแก้ปุ่มตรงคลังสินค้าหน่อย" },
                { "role": "assistant", "content": "Update the warehouse button component." },
                { "role": "user", "content": "กินข้าวหรือยัง" },
                { "role": "assistant", "content": "Have you eaten yet?" },
                { "role": "user", "content": "อย่าแตะต้องไฟล์ config.json" },
                { "role": "assistant", "content": "Do not modify the config.json file." },
                { "role": "user", "content": prompt }
            ],
            "stream": false,
            "options": {
                "temperature": 0.1,
                "num_predict": 128
            }
        });

        let response = client
            .post(OLLAMA_CHAT_ENDPOINT)
            .json(&chat_prompt_envelope)
            .send()
            .map_err(|e| e.to_string())?;

        if !response.status().is_success() {
            return Err("Ollama service non-success response".into());
        }

        let json_body: serde_json::Value = response.json().map_err(|e| e.to_string())?;
        let content = json_body["message"]["content"]
            .as_str()
            .unwrap_or("")
            .trim();

        Ok(Self::clean_llm_output(content))
    }

    async fn query_local_llm_async(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(12))
            .build()?;

        let chat_prompt_envelope = serde_json::json!({
            "model": LOCAL_LLM_MODEL,
            "messages": [
                { "role": "system", "content": SYSTEM_TRANSLATOR_PROMPT },
                { "role": "user", "content": "ช่วยแก้ปุ่มตรงคลังสินค้าหน่อย" },
                { "role": "assistant", "content": "Update the warehouse button component." },
                { "role": "user", "content": "กินข้าวหรือยัง" },
                { "role": "assistant", "content": "Have you eaten yet?" },
                { "role": "user", "content": "อย่าแตะต้องไฟล์ config.json" },
                { "role": "assistant", "content": "Do not modify the config.json file." },
                { "role": "user", "content": prompt }
            ],
            "stream": false,
            "options": {
                "temperature": 0.1,
                "num_predict": 128
            }
        });

        let response = client
            .post(OLLAMA_CHAT_ENDPOINT)
            .json(&chat_prompt_envelope)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err("Ollama service non-success response".into());
        }

        let json_body: serde_json::Value = response.json().await?;
        let content = json_body["message"]["content"]
            .as_str()
            .unwrap_or("")
            .trim();

        Ok(Self::clean_llm_output(content))
    }

    fn clean_llm_output(output: &str) -> String {
        let mut cleaned = output.trim().to_string();
        if cleaned.starts_with("```") && cleaned.ends_with("```") {
            let lines: Vec<&str> = cleaned.lines().collect();
            if lines.len() >= 2 {
                cleaned = lines[1..lines.len() - 1].join("\n").trim().to_string();
            }
        }
        if (cleaned.starts_with('"') && cleaned.ends_with('"'))
            || (cleaned.starts_with('\'') && cleaned.ends_with('\''))
        {
            if cleaned.len() >= 2 {
                cleaned = cleaned[1..cleaned.len() - 1].trim().to_string();
            }
        }
        cleaned.trim().to_string()
    }

    fn detect_is_greeting(lower_input: &str, lower_english: &str) -> bool {
        let has_action_marker = lower_english.contains("fix")
            || lower_english.contains("update")
            || lower_english.contains("add")
            || lower_english.contains("modify")
            || lower_english.contains("implement")
            || lower_input.contains("แก้")
            || lower_input.contains("เพิ่ม")
            || lower_input.contains("ลบ")
            || lower_input.contains("สร้าง");
        if has_action_marker {
            return false;
        }
        lower_english.starts_with("have you eaten")
            || lower_english.starts_with("hello")
            || lower_english.starts_with("hi")
            || lower_english.starts_with("how are you")
            || lower_english.starts_with("good morning")
            || lower_english.starts_with("good afternoon")
            || lower_input == "สวัสดี"
            || lower_input.contains("กินข้าว")
            || lower_input.contains("สบายดี")
            || lower_input.contains("ทำอะไรอยู่")
    }

    fn detect_is_negated(lower_input: &str, lower_english: &str) -> bool {
        let is_dont_forget = lower_input.contains("อย่าลืม")
            || lower_english.contains("don't forget")
            || lower_english.contains("dont forget");
        if is_dont_forget {
            return false;
        }
        lower_english.contains("preserve")
            || lower_english.contains("do not modify")
            || lower_english.contains("strictly keep")
            || lower_english.contains("don't modify")
            || lower_english.contains("forbid")
            || lower_input.contains("อย่า")
            || lower_input.contains("ห้าม")
            || lower_input.contains("ไม่ต้อง")
    }

    fn classify_primary_action(lower_english: &str, is_greeting: bool, is_negated: bool) -> String {
        if is_greeting {
            "CONVERSATIONAL".to_string()
        } else if is_negated {
            "PRESERVE".to_string()
        } else if lower_english.starts_with("fix") {
            "FIX".to_string()
        } else if lower_english.starts_with("implement") || lower_english.starts_with("create") || lower_english.starts_with("add") {
            "IMPLEMENT".to_string()
        } else if lower_english.starts_with("remove") || lower_english.starts_with("delete") {
            "REMOVE".to_string()
        } else if lower_english.starts_with("inspect") || lower_english.starts_with("audit") || lower_english.starts_with("check") {
            "INSPECT".to_string()
        } else {
            "UPDATE".to_string()
        }
    }

    fn extract_candidate_tokens(lower_english: &str) -> Vec<String> {
        let mut candidate_search_tokens = Vec::new();
        let stop_words = ["the", "a", "an", "with", "into", "from", "for", "and", "or", "to", "in", "on", "at", "by", "of", "component", "effect", "include"];
        for word in lower_english.split(|c: char| !c.is_alphanumeric() && c != '_') {
            let trimmed_word = word.trim();
            if trimmed_word.len() >= 3 && !stop_words.contains(&trimmed_word) {
                candidate_search_tokens.push(trimmed_word.to_string());
            }
        }
        candidate_search_tokens.dedup();
        candidate_search_tokens
    }

    fn extract_components_and_styles(lower_english: &str) -> (Vec<String>, Vec<String>) {
        let mut detected_components = Vec::new();
        let known_components = ["button", "table", "form", "card", "modal", "dialog", "navbar", "sidebar", "input", "dropdown", "menu", "header", "footer", "chart"];
        for comp in known_components {
            if lower_english.contains(comp) {
                detected_components.push(format!("{} component", comp));
            }
        }

        let mut styling_tokens = Vec::new();
        let known_styles = ["3d", "depth", "isometric", "gradient", "neon", "glow", "shadow", "glassmorphism", "blur"];
        for st in known_styles {
            if lower_english.contains(st) {
                styling_tokens.push(format!("styling: {}", st));
            }
        }

        (detected_components, styling_tokens)
    }

    fn build_intent_from_llm(raw_input: &str, concise_english: &str) -> TranspiledIntent {
        let lower_english = concise_english.to_lowercase();
        let lower_input = raw_input.to_lowercase();

        let is_greeting = Self::detect_is_greeting(&lower_input, &lower_english);
        let is_negated = Self::detect_is_negated(&lower_input, &lower_english);
        let primary_action = Self::classify_primary_action(&lower_english, is_greeting, is_negated);
        let candidate_search_tokens = Self::extract_candidate_tokens(&lower_english);
        let (detected_components, styling_tokens) = Self::extract_components_and_styles(&lower_english);

        let technical_action_summary = if is_greeting {
            "Conversational Chitchat (Bypassed)".to_string()
        } else {
            concise_english.to_string()
        };

        TranspiledIntent {
            primary_action,
            detected_components,
            styling_tokens,
            technical_action_summary,
            candidate_search_tokens,
            is_negated,
            circuit_breaker_triggered: false,
            concise_english: concise_english.to_string(),
            is_greeting,
        }
    }

    fn build_passthrough_intent(raw_input: &str) -> TranspiledIntent {
        let lower_input = raw_input.to_lowercase();
        let is_dont_forget = raw_input.contains("อย่าลืม")
            || lower_input.contains("don't forget")
            || lower_input.contains("dont forget");

        let has_action_marker = raw_input.contains("แก้")
            || raw_input.contains("เพิ่ม")
            || raw_input.contains("ลบ")
            || raw_input.contains("สร้าง")
            || raw_input.contains("เขียน");

        let is_greeting = !has_action_marker && (
            raw_input == "สวัสดี"
            || raw_input.contains("กินข้าว")
            || raw_input.contains("สบายดี")
        );

        let is_negated = !is_dont_forget && (raw_input.contains("อย่า") || raw_input.contains("ห้าม"));

        TranspiledIntent {
            primary_action: if is_greeting { "CONVERSATIONAL".to_string() } else if is_negated { "PRESERVE".to_string() } else { "UPDATE".to_string() },
            detected_components: Vec::new(),
            styling_tokens: Vec::new(),
            technical_action_summary: raw_input.to_string(),
            candidate_search_tokens: Vec::new(),
            is_negated,
            circuit_breaker_triggered: false,
            concise_english: raw_input.to_string(),
            is_greeting,
        }
    }

    fn detect_circuit_breaker(normalized: &str) -> bool {
        let lower = normalized.to_lowercase();
        let triggers = [
            "ยังไม่ได้",
            "ยังไม่หาย",
            "พังเหมือนเดิม",
            "เหมือนเดิม",
            "still fails",
            "same error",
            "looping",
            "loop failure",
            "repeated failure",
        ];
        triggers.iter().any(|&trigger| lower.contains(trigger))
    }

    fn circuit_breaker_intent() -> TranspiledIntent {
        TranspiledIntent {
            primary_action: "CIRCUIT_BREAKER_LOCKDOWN".to_string(),
            detected_components: Vec::new(),
            styling_tokens: Vec::new(),
            technical_action_summary: "LOCKDOWN: Repeated fix failed. FORBIDDEN from generating speculative code. DEMAND runtime log/stack trace.".to_string(),
            candidate_search_tokens: Vec::new(),
            is_negated: false,
            circuit_breaker_triggered: true,
            concise_english: "Diagnostic lockdown: repeated failure detected, request error log".to_string(),
            is_greeting: false,
        }
    }

    fn empty_intent() -> TranspiledIntent {
        TranspiledIntent {
            primary_action: "NOOP".to_string(),
            detected_components: Vec::new(),
            styling_tokens: Vec::new(),
            technical_action_summary: String::new(),
            candidate_search_tokens: Vec::new(),
            is_negated: false,
            circuit_breaker_triggered: false,
            concise_english: String::new(),
            is_greeting: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_breaker_activation() {
        let outcome = LinguisticTranspiler::transpile("แก้มาสามรอบแล้วยังไม่ได้ พังเหมือนเดิม");
        assert!(outcome.circuit_breaker_triggered);
        assert_eq!(outcome.primary_action, "CIRCUIT_BREAKER_LOCKDOWN");
    }

    #[test]
    fn test_local_llm_3d_warehouse_button() {
        let outcome = LinguisticTranspiler::transpile("พอดีจะเปลี่ยนปุ่ม คลังสินค้าเป็นแบบ สามมิติ");
        assert!(!outcome.circuit_breaker_triggered);
        assert!(!outcome.concise_english.is_empty());
        let lower = outcome.concise_english.to_lowercase();
        assert!(lower.contains("warehouse"));
        assert!(lower.contains("button"));
        assert!(lower.contains("3d") || lower.contains("three-dimensional"));
    }

    #[test]
    fn test_local_llm_chitchat_eating() {
        let outcome = LinguisticTranspiler::transpile("กินข้าวหรือยัง");
        assert!(!outcome.circuit_breaker_triggered);
        assert!(outcome.is_greeting);
        let lower = outcome.concise_english.to_lowercase();
        assert!(lower.contains("eaten") || lower.contains("meal") || lower.contains("eat") || lower.contains("กินข้าว"));
    }

    #[test]
    fn test_local_llm_negation_preservation() {
        let outcome = LinguisticTranspiler::transpile("อย่าแก้โค้ดส่วนอื่นนะ");
        assert!(outcome.is_negated);
    }
}
