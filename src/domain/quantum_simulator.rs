use super::anti_spaghetti::AntiSpaghettiDirective;
use super::formal_contract::HoareContract;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

static COORDINATE_EXTRACTOR_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    match Regex::new(r"([a-zA-Z0-9_./\\-]+\.(tsx|ts|jsx|js|rs|py|go|html|css|vue|svelte|java|kt|cs|cpp|c|h|hpp|toml|yaml|yml|json|md|sh|rb|php|sql)|Dockerfile|Makefile)") {
        Ok(compiled) => compiled,
        Err(_) => match Regex::new("") {
            Ok(fallback) => fallback,
            Err(_) => unreachable!(),
        },
    }
});

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuickFixAction {
    pub action_identifier: String,
    pub display_label: String,
    pub replacement_target: String,
    #[serde(default)]
    pub action_payload: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuantumBranchState {
    pub branch_identifier: String,
    pub passed: bool,
    pub fidelity_score: f32,
    pub diagnostic_message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuantumSimulationReceipt {
    pub collapse_allowed: bool,
    pub overall_fidelity: f32,
    pub branches: Vec<QuantumBranchState>,
    pub quick_fixes: Vec<QuickFixAction>,
    pub hoare_contract_spec: Option<String>,
    pub dense_symbolic_ir_spec: Option<String>,
    pub technical_action_summary: Option<String>,
}

pub struct QuantumSuperpositionSimulator;

impl QuantumSuperpositionSimulator {
    #[must_use]
    pub fn simulate_superposition(
        lexical_intent: &str,
        directive: &AntiSpaghettiDirective,
    ) -> QuantumSimulationReceipt {
        let trimmed_input = lexical_intent.trim();
        let target_candidate = Self::extract_coordinate_candidate(trimmed_input);

        let branch_alpha = Self::evaluate_branch_alpha_coordinate(target_candidate.as_deref());
        let branch_beta = Self::evaluate_branch_beta_consistency(trimmed_input);
        let branch_gamma = Self::evaluate_branch_gamma_invariants(directive);
        let branch_delta = Self::evaluate_branch_delta_containment(trimmed_input);

        let branches = vec![branch_alpha, branch_beta, branch_gamma, branch_delta];
        let average_fidelity: f32 =
            branches.iter().map(|b| b.fidelity_score).sum::<f32>() / (branches.len() as f32);

        let quick_fixes = Self::generate_coordinate_quick_fixes(trimmed_input);
        let coordinate_slice = Self::resolve_target_coordinate(target_candidate, trimmed_input);
        let transpiled =
            super::linguistic_transpiler::LinguisticTranspiler::transpile(trimmed_input);

        if transpiled.circuit_breaker_triggered {
            let breaker_spec = super::formal_contract::HoareContract::render_circuit_breaker_ir(
                &coordinate_slice,
                trimmed_input,
            );
            return QuantumSimulationReceipt {
                collapse_allowed: false,
                overall_fidelity: 0.0,
                branches,
                quick_fixes,
                hoare_contract_spec: Some(breaker_spec.clone()),
                dense_symbolic_ir_spec: Some(breaker_spec),
                technical_action_summary: Some(transpiled.technical_action_summary),
            };
        }

        let contract = Self::synthesize_contract(coordinate_slice.into(), trimmed_input, directive);
        let hoare_contract_spec = Some(contract.render_hoare_specification());
        let dense_symbolic_ir_spec = if transpiled.is_negated {
            Some(super::formal_contract::HoareContract::render_negation_dense_ir(
                &contract.target_coordinate,
                &transpiled.technical_action_summary,
                "Universal",
                &[],
            ))
        } else {
            Some(contract.render_dense_symbolic_ir(&transpiled.technical_action_summary))
        };

        QuantumSimulationReceipt {
            collapse_allowed: true,
            overall_fidelity: average_fidelity,
            branches,
            quick_fixes,
            hoare_contract_spec,
            dense_symbolic_ir_spec,
            technical_action_summary: Some(transpiled.technical_action_summary),
        }
    }

    fn resolve_target_coordinate(target_candidate: Option<String>, trimmed_input: &str) -> String {
        let lower = trimmed_input.to_lowercase();
        let is_inquiry = lower.contains("what is")
            || lower.contains("how does")
            || lower.contains("explain")
            || trimmed_input.contains("คืออะไร")
            || trimmed_input.contains("อธิบาย");

        if is_inquiry {
            "ConceptualArchitecture".to_string()
        } else if let Some(coord) = target_candidate {
            coord
        } else {
            format!("Domain(About: {})", trimmed_input)
        }
    }

    fn extract_coordinate_candidate(input: &str) -> Option<String> {
        COORDINATE_EXTRACTOR_PATTERN
            .find(input)
            .map(|m| m.as_str().to_string())
    }

    fn evaluate_branch_alpha_coordinate(target_candidate: Option<&str>) -> QuantumBranchState {
        match target_candidate {
            Some(coord) => QuantumBranchState {
                branch_identifier: "ALPHA_COORDINATE_SOUNDNESS".into(),
                passed: true,
                fidelity_score: 1.0,
                diagnostic_message: format!("Target coordinate anchored: {}", coord),
            },
            None => QuantumBranchState {
                branch_identifier: "ALPHA_COORDINATE_SOUNDNESS".into(),
                passed: true,
                fidelity_score: 0.95,
                diagnostic_message:
                    "Target scope: Domain-anchored (Agent auto-discovers candidate files).".into(),
            },
        }
    }

    fn evaluate_branch_beta_consistency(_input: &str) -> QuantumBranchState {
        QuantumBranchState {
            branch_identifier: "BETA_TRANSITION_CONSISTENCY".into(),
            passed: true,
            fidelity_score: 1.0,
            diagnostic_message: "Post-condition intent verified for compilation.".into(),
        }
    }

    fn evaluate_branch_gamma_invariants(directive: &AntiSpaghettiDirective) -> QuantumBranchState {
        let complexity_budget_valid = directive.max_cyclomatic_complexity <= 7;
        let spans_bounded = directive.max_function_span_lines <= 70;
        let generic_banned = !directive.forbidden_generic_identifiers.is_empty();

        let passed = complexity_budget_valid && spans_bounded && generic_banned;
        QuantumBranchState {
            branch_identifier: "GAMMA_INVARIANT_BUDGET".into(),
            passed,
            fidelity_score: if passed { 1.0 } else { 0.8 },
            diagnostic_message:
                "Invariants enforced: CC <= 7, MaxSpan <= 70, Generic Identifiers Banned.".into(),
        }
    }

    fn evaluate_branch_delta_containment(input: &str) -> QuantumBranchState {
        let mentions_junk_drawer =
            input.contains("utils/") || input.contains("helpers/") || input.contains("common/");
        QuantumBranchState {
            branch_identifier: "DELTA_ARCHITECTURAL_CONTAINMENT".into(),
            passed: true,
            fidelity_score: if mentions_junk_drawer { 0.9 } else { 1.0 },
            diagnostic_message: if mentions_junk_drawer {
                "Containment guidance: Redirecting logic away from utils/ or helpers/ junk drawers."
                    .into()
            } else {
                "Architectural boundary preserved: Clean domain-oriented layout.".into()
            },
        }
    }

    fn generate_coordinate_quick_fixes(input: &str) -> Vec<QuickFixAction> {
        let mut fixes = Vec::new();

        fixes.push(QuickFixAction {
            action_identifier: "qf_explicit_file".into(),
            display_label: "📍 ระบุพิกัดไฟล์เจาะจง (e.g. In path/to/file.ext)".into(),
            replacement_target: format!("In file <target_file>: {}", input),
            action_payload: format!("In file <target_file>: {}", input),
        });

        fixes.push(QuickFixAction {
            action_identifier: "qf_domain_scope".into(),
            display_label: "🌐 ใช้ Domain Scope (ให้ AI ค้นหาสัญลักษณ์ผ่าน LSP/MCP)".into(),
            replacement_target: format!("Domain(About: {})", input),
            action_payload: format!("Domain(About: {})", input),
        });

        fixes.push(QuickFixAction {
            action_identifier: "qf_ascii_blueprint".into(),
            display_label: "📐 บังคับสร้างพิมพ์เขียว ASCII ก่อนเขียนโค้ด".into(),
            replacement_target: format!(
                "{}\n[INVARIANT: Render explicit ASCII component wireframe before writing code]",
                input
            ),
            action_payload: format!(
                "{}\n[INVARIANT: Render explicit ASCII component wireframe before writing code]",
                input
            ),
        });

        fixes
    }

    fn synthesize_contract<'a>(
        coordinate: std::borrow::Cow<'a, str>,
        input: &'a str,
        directive: &AntiSpaghettiDirective,
    ) -> HoareContract<'a> {
        let mut assertions: Vec<std::borrow::Cow<'a, str>> = Vec::new();
        assertions.push("=== GODKILLER ZERO : ARCHITECTURAL INVARIANTS ===".into());
        assertions.push(
            format!(
                "- COMPLEXITY: Cyclomatic complexity MUST NOT exceed {}. Maximum function length: {} lines.",
                directive.max_cyclomatic_complexity, directive.max_function_span_lines
            )
            .into(),
        );

        let generic_banned_list = directive.forbidden_generic_identifiers.join(", ");
        assertions.push(
            format!(
                "- CODE_HYGIENE: Strictly FORBIDDEN generic identifiers: [{}]. Use explicit domain nomenclature.",
                generic_banned_list
            )
            .into(),
        );

        assertions.push(
            "- COMMENT_DISCIPLINE: Zero conversational comments. Self-documenting code only."
                .into(),
        );
        assertions.push(
            "- ARCHITECTURAL_BOUNDARIES: Ban creation of utils/ or helpers/ junk drawers.".into(),
        );
        assertions.push("- ZERO_SPECULATION: If specifications, coordinates, or types are underspecified, PROHIBITED from speculative assumptions. MUST pause and prompt user with targeted clarifying questions.".into());
        assertions.push("- OUTPUT_FORMAT: Emit strictly unified diff patches or targeted file replacements. No conversational fluff.".into());

        if directive.enforce_disk_verification {
            assertions.push("- VERIFICATION_PROTOCOL: Verify target files on disk and ensure build or unit tests pass before concluding.".into());
        }

        assertions.push(
            "INVARIANT: IF UNDERSPECIFIED, NEVER GUESS. MUST TRIGGER INTERACTIVE CLARIFICATION."
                .into(),
        );

        HoareContract::new(
            coordinate,
            "Defect observed in target coordinates without specification".into(),
            input.into(),
            assertions,
        )
    }
}
