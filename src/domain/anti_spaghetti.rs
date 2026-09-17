use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AntiSpaghettiDirective {
    pub max_cyclomatic_complexity: u8,
    pub max_function_span_lines: u8,
    pub forbidden_generic_identifiers: Vec<String>,
    pub forbidden_comment_patterns: Vec<String>,
    pub require_atomic_diff_emission: bool,
    #[serde(default = "default_true", alias = "enforce_godkiller_mcp_verification")]
    pub enforce_disk_verification: bool,
}

fn default_true() -> bool {
    true
}

impl Default for AntiSpaghettiDirective {
    fn default() -> Self {
        Self {
            max_cyclomatic_complexity: 7,
            max_function_span_lines: 70,
            forbidden_generic_identifiers: vec![
                "data".into(),
                "res".into(),
                "req".into(),
                "item".into(),
                "val".into(),
                "temp".into(),
                "obj".into(),
                "info".into(),
                "payload".into(),
                "result".into(),
                "handleData".into(),
                "processData".into(),
                "doAction".into(),
                "updateItem".into(),
            ],
            forbidden_comment_patterns: vec![
                "Trivial restatements of code operations".into(),
                "Conversational prefaces or apologetic remarks".into(),
                "AI watermarks or disclaimers".into(),
            ],
            require_atomic_diff_emission: true,
            enforce_disk_verification: true,
        }
    }
}

pub struct AntiSpaghettiGuard;

impl AntiSpaghettiGuard {
    #[must_use]
    pub fn build_system_invariant_block(directive: &AntiSpaghettiDirective) -> String {
        let mut invariant_lines = Vec::new();

        invariant_lines.push("=== GODKILLER ZERO : ARCHITECTURAL INVARIANTS ===".to_string());
        invariant_lines.push(format!(
            "- COMPLEXITY: Cyclomatic complexity MUST NOT exceed {}. Maximum function length: {} lines.",
            directive.max_cyclomatic_complexity, directive.max_function_span_lines
        ));
        invariant_lines.push(
            "- CODE_HYGIENE: Strictly FORBIDDEN generic identifiers: [".to_string()
                + &directive.forbidden_generic_identifiers.join(", ")
                + "]. Use explicit domain nomenclature.",
        );
        invariant_lines.push(
            "- COMMENT_DISCIPLINE: Zero conversational comments. Self-documenting code only."
                .to_string(),
        );
        invariant_lines.push(
            "- ARCHITECTURAL_BOUNDARIES: Ban creation of utils/ or helpers/ junk drawers."
                .to_string(),
        );
        invariant_lines.push("- ZERO_SPECULATION: If specifications, coordinates, or types are underspecified, PROHIBITED from speculative assumptions. MUST pause and prompt user with targeted clarifying questions.".to_string());

        if directive.require_atomic_diff_emission {
            invariant_lines.push("- OUTPUT_FORMAT: Emit strictly unified diff patches or targeted file replacements. No conversational fluff.".to_string());
        }

        if directive.enforce_disk_verification {
            invariant_lines.push("- VERIFICATION_PROTOCOL: Verify target files on disk and ensure build or unit tests pass before concluding.".to_string());
        }

        invariant_lines.join("\n")
    }
}
