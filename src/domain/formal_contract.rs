use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HoareContract<'a> {
    pub target_coordinate: Cow<'a, str>,
    pub pre_condition: Cow<'a, str>,
    pub post_condition: Cow<'a, str>,
    pub invariant_assertions: Vec<Cow<'a, str>>,
}

impl<'a> HoareContract<'a> {
    #[must_use]
    pub fn new(
        target_coordinate: Cow<'a, str>,
        pre_condition: Cow<'a, str>,
        post_condition: Cow<'a, str>,
        invariant_assertions: Vec<Cow<'a, str>>,
    ) -> Self {
        Self {
            target_coordinate,
            pre_condition,
            post_condition,
            invariant_assertions,
        }
    }

    #[must_use]
    pub fn render_hoare_specification(&self) -> String {
        let mut contract_lines = Vec::new();

        contract_lines.push("```tla\n!GK_ZERO:HOARE_CONTRACT".to_string());
        contract_lines.push(format!("TARGET_COORDINATE  :: {}", self.target_coordinate));
        contract_lines.push(format!("PRE_CONDITION {{P}} :: {}", self.pre_condition));
        contract_lines.push(format!("POST_CONDITION {{Q}}:: {}", self.post_condition));
        contract_lines.push("INVARIANT_ASSERTIONS [I] :: [".to_string());

        for assertion in &self.invariant_assertions {
            contract_lines.push(format!(r"  /\ {}", assertion));
        }

        contract_lines.push("]".to_string());
        contract_lines.push("```".to_string());

        contract_lines.join("\n")
    }

    #[must_use]
    pub fn render_dense_symbolic_ir(&self, technical_action: &str) -> String {
        format!(
            "[TARGET]: {}\n[ACTION]: {}",
            self.target_coordinate, technical_action
        )
    }

    #[must_use]
    pub fn render_context_aware_dense_ir(
        target_coordinate: &str,
        technical_action: &str,
        stack_paradigm: &str,
        blast_radius: &[String],
    ) -> String {
        let mut contract_lines = Vec::new();
        contract_lines.push(format!("[TARGET]: {}", target_coordinate));
        if !stack_paradigm.is_empty() && stack_paradigm != "Universal / Agnostic" {
            contract_lines.push(format!("[STACK]: {}", stack_paradigm));
        }
        if !blast_radius.is_empty() {
            contract_lines.push(format!("[BLAST_RADIUS]: [{}]", blast_radius.join(", ")));
        }
        contract_lines.push(format!("[ACTION]: {}", technical_action));
        contract_lines.join("\n")
    }

    #[must_use]
    pub fn render_circuit_breaker_ir(target_coordinate: &str, diagnostic_message: &str) -> String {
        format!(
            "[GK0:DIAGNOSTIC_CIRCUIT_BREAKER]\nTARGET: {}\nSTATUS: REPEATED_FAILURE_LOCKDOWN\nTRIGGER: {}\nMANDATORY_INVARIANTS: Strictly FORBIDDEN from claiming success or guessing code. MUST demand runtime terminal error log/stack trace or emit minimal hypothesis trace diagram.",
            target_coordinate, diagnostic_message
        )
    }

    #[must_use]
    pub fn render_negation_dense_ir(
        target_coordinate: &str,
        technical_action: &str,
        stack_paradigm: &str,
        blast_radius: &[String],
    ) -> String {
        let blast_info = if blast_radius.is_empty() {
            "Isolated scope".to_string()
        } else {
            blast_radius.join(", ")
        };

        format!(
            "[GK0:PRESERVE_GUARD]\nTARGET: {}\nSTACK: {}\nPROTECTED_SCOPE: [{}]\nACTION: {}\nINVARIANTS: NegativeMutationBounding:STRICT | ZeroGhostEdits | PreserveUntouchedBitForBit | NoUnsolicitedMutations",
            target_coordinate, stack_paradigm, blast_info, technical_action
        )
    }
}
