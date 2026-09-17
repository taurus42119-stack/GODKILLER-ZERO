use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetCoordinate(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DefectGoalDescription(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstraintBoundary(pub Vec<String>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IngressEvaluationVerdict {
    BypassFastLane,
    ConversationalGreeting {
        greeting_text: String,
    },
    ProceedToCompilation {
        target: TargetCoordinate,
        defect: DefectGoalDescription,
        constraints: ConstraintBoundary,
    },
}

#[derive(Debug, Clone)]
pub struct TriPillarEvaluator;

impl TriPillarEvaluator {
    #[must_use]
    pub fn evaluate_prompt(raw_prompt_text: &str) -> IngressEvaluationVerdict {
        let trimmed_prompt = raw_prompt_text.trim();

        if trimmed_prompt.is_empty() {
            return IngressEvaluationVerdict::BypassFastLane;
        }

        if Self::is_fast_lane_affirmation(trimmed_prompt) {
            return IngressEvaluationVerdict::BypassFastLane;
        }

        if Self::is_conversational_greeting(trimmed_prompt) {
            return IngressEvaluationVerdict::ConversationalGreeting {
                greeting_text: trimmed_prompt.to_string(),
            };
        }

        let target_file = Self::extract_target_file(trimmed_prompt);
        let target_coordinate = match target_file {
            Some(file_name) => TargetCoordinate(file_name),
            None if trimmed_prompt.contains("ตรงนี้") || trimmed_prompt.contains("this") => {
                TargetCoordinate("Spatial(FocusedActiveNode)".to_string())
            }
            None => TargetCoordinate(format!("Domain(About: {})", trimmed_prompt)),
        };

        IngressEvaluationVerdict::ProceedToCompilation {
            target: target_coordinate,
            defect: DefectGoalDescription(trimmed_prompt.to_string()),
            constraints: ConstraintBoundary(vec!["Silent Context Enrichment Active".into()]),
        }
    }

    fn is_fast_lane_affirmation(prompt_text: &str) -> bool {
        let word_count = prompt_text.split_whitespace().count();
        let char_count = prompt_text.chars().count();
        if word_count > 3 || char_count > 15 {
            return false;
        }
        let lower = prompt_text.to_lowercase();
        let stripped = lower.trim_end_matches(['.', '!', '?', ' ']);
        matches!(
            stripped,
            "yes"
                | "ok"
                | "okay"
                | "proceed"
                | "go"
                | "continue"
                | "looks good"
                | "ตกลง"
                | "ใช่"
                | "โอเค"
                | "จัดไป"
                | "ทำต่อ"
                | "ตามนั้น"
        )
    }

    fn is_conversational_greeting(prompt_text: &str) -> bool {
        let action_markers = [
            "แก้",
            "เพิ่ม",
            "ลบ",
            "สร้าง",
            "ช่วย",
            "ดู",
            "ทำ",
            "fix",
            "add",
            "remove",
            "create",
            "help",
            "debug",
            "refactor",
            "run",
            "test",
            "check",
        ];
        let lower = prompt_text.to_lowercase();
        if action_markers.iter().any(|&marker| lower.contains(marker)) {
            return false;
        }

        let word_count = prompt_text.split_whitespace().count();
        if word_count > 4 {
            return false;
        }
        let stripped = lower.trim_end_matches(['.', '!', '?', ' ']);
        matches!(
            stripped,
            "hello"
                | "hi"
                | "hey"
                | "greetings"
                | "good morning"
                | "good afternoon"
                | "good evening"
                | "สวัสดี"
                | "สวัสดีครับ"
                | "สวัสดีค่ะ"
                | "หวัดดี"
                | "หวัดดีครับ"
                | "หวัดดีค่ะ"
                | "ดีครับ"
                | "ดีค่ะ"
                | "อรุณสวัสดิ์"
        )
    }

    fn extract_target_file(prompt_text: &str) -> Option<String> {
        let valid_extensions = [
            "rs", "cs", "ts", "tsx", "js", "jsx", "py", "go", "toml", "json", "md", "html", "css",
            "sql", "yaml", "yml", "sh", "bat", "ps1", "txt", "c", "cpp", "h", "hpp", "java", "kt",
            "swift", "rb", "php",
        ];

        for token in prompt_text.split_whitespace() {
            let clean = token.trim_matches([
                '"', '\'', '(', ')', '[', ']', '{', '}', ':', ',', ';', '<', '>',
            ]);
            let Some((_, extension_candidate)) = clean.rsplit_once('.') else {
                continue;
            };
            let extension_lower = extension_candidate.to_lowercase();
            if !valid_extensions.contains(&extension_lower.as_str()) {
                continue;
            }
            let normalized_path = clean.replace('\\', "/");
            let Some(basename_candidate) = normalized_path.split('/').next_back() else {
                continue;
            };
            if !basename_candidate.is_empty() && basename_candidate.contains('.') {
                return Some(normalized_path);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_lane_affirmation() {
        assert_eq!(
            TriPillarEvaluator::evaluate_prompt("ok"),
            IngressEvaluationVerdict::BypassFastLane
        );
        assert_eq!(
            TriPillarEvaluator::evaluate_prompt("จัดไป"),
            IngressEvaluationVerdict::BypassFastLane
        );
    }

    #[test]
    fn test_conversational_greeting() {
        assert_eq!(
            TriPillarEvaluator::evaluate_prompt("สวัสดี"),
            IngressEvaluationVerdict::ConversationalGreeting {
                greeting_text: "สวัสดี".to_string()
            }
        );
        assert_eq!(
            TriPillarEvaluator::evaluate_prompt("hello"),
            IngressEvaluationVerdict::ConversationalGreeting {
                greeting_text: "hello".to_string()
            }
        );
    }

    #[test]
    fn test_mixed_greeting_with_action_does_not_trap() {
        let verdict = TriPillarEvaluator::evaluate_prompt("สวัสดี ช่วยแก้บั๊กตรงนี้หน่อย");
        assert!(matches!(
            verdict,
            IngressEvaluationVerdict::ProceedToCompilation { .. }
        ));
    }

    #[test]
    fn test_extract_target_file_ignores_decimals_and_finds_real_files() {
        assert_eq!(
            TriPillarEvaluator::extract_target_file("update to v1.5 now"),
            None
        );
        assert_eq!(
            TriPillarEvaluator::extract_target_file("คะแนน 3.14 ใน calc.py"),
            Some("calc.py".to_string())
        );
        assert_eq!(
            TriPillarEvaluator::extract_target_file("check src/proxy/server.rs please"),
            Some("src/proxy/server.rs".to_string())
        );
    }

    #[test]
    fn test_natural_language_intent_proceeds_cleanly() {
        let verdict = TriPillarEvaluator::evaluate_prompt("แก้ปุ่มหน้าแรกหน่อย");
        match verdict {
            IngressEvaluationVerdict::ProceedToCompilation { defect, .. } => {
                assert_eq!(defect.0, "แก้ปุ่มหน้าแรกหน่อย");
            }
            _ => panic!("Should proceed to compilation without word-matching rejection"),
        }
    }

    #[test]
    fn test_spatial_anchor_for_vague_requests() {
        let verdict = TriPillarEvaluator::evaluate_prompt("แก้ปุ่มตรงนี้หน่อย");
        match verdict {
            IngressEvaluationVerdict::ProceedToCompilation { target, .. } => {
                assert_eq!(target.0, "Spatial(FocusedActiveNode)");
            }
            _ => panic!("Should proceed to compilation with spatial auto-anchoring"),
        }
    }
}
