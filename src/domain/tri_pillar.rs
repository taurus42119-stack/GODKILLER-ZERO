use serde::{Deserialize, Serialize};

/// Which ingress lane a raw prompt belongs to. Callers act on the lane alone:
/// affirmations pass through untouched, greetings get an immediate local reply,
/// everything else is compiled into a contract by the evaluator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IngressEvaluationVerdict {
    BypassFastLane,
    ConversationalGreeting,
    ProceedToCompilation,
}

#[derive(Debug, Clone)]
pub struct TriPillarEvaluator;

impl TriPillarEvaluator {
    #[must_use]
    pub fn evaluate_prompt(raw_prompt_text: &str) -> IngressEvaluationVerdict {
        let trimmed_prompt = raw_prompt_text.trim();

        if trimmed_prompt.is_empty() || Self::is_fast_lane_affirmation(trimmed_prompt) {
            return IngressEvaluationVerdict::BypassFastLane;
        }

        if Self::is_conversational_greeting(trimmed_prompt) {
            return IngressEvaluationVerdict::ConversationalGreeting;
        }

        IngressEvaluationVerdict::ProceedToCompilation
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
            IngressEvaluationVerdict::ConversationalGreeting
        );
        assert_eq!(
            TriPillarEvaluator::evaluate_prompt("hello"),
            IngressEvaluationVerdict::ConversationalGreeting
        );
    }

    #[test]
    fn test_mixed_greeting_with_action_does_not_trap() {
        assert_eq!(
            TriPillarEvaluator::evaluate_prompt("สวัสดี ช่วยแก้บั๊กตรงนี้หน่อย"),
            IngressEvaluationVerdict::ProceedToCompilation
        );
    }

    #[test]
    fn test_natural_language_intent_proceeds_cleanly() {
        assert_eq!(
            TriPillarEvaluator::evaluate_prompt("แก้ปุ่มหน้าแรกหน่อย"),
            IngressEvaluationVerdict::ProceedToCompilation
        );
    }

    #[test]
    fn test_long_affirmation_is_not_fast_laned() {
        assert_eq!(
            TriPillarEvaluator::evaluate_prompt("ok but first rename the field"),
            IngressEvaluationVerdict::ProceedToCompilation
        );
    }
}
