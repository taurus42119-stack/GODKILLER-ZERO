use godkiller_zero::domain::{
    AntiSpaghettiDirective, HoareContract, InvariantContractEvaluator, ZeroBridge,
};
use std::borrow::Cow;

#[test]
fn test_hoare_contract_rendering() {
    let contract = HoareContract::new(
        Cow::Borrowed("src/components/ThemeToggle.tsx#ThemeButton:L30-L50"),
        Cow::Borrowed("Button fails to toggle dark class on click"),
        Cow::Borrowed("Button toggles dark class correctly with aria-pressed"),
        vec![
            Cow::Borrowed("Complexity <= 7"),
            Cow::Borrowed("Banned generic symbols: [data, res, req]"),
        ],
    );

    let rendered = contract.render_hoare_specification();
    assert!(rendered.contains("!GK_ZERO:HOARE_CONTRACT"));
    assert!(rendered
        .contains("TARGET_COORDINATE  :: src/components/ThemeToggle.tsx#ThemeButton:L30-L50"));
    assert!(rendered.contains("PRE_CONDITION {P} :: Button fails to toggle dark class on click"));
    assert!(rendered
        .contains("POST_CONDITION {Q}:: Button toggles dark class correctly with aria-pressed"));
    assert!(rendered.contains("Complexity <= 7"));
}

#[test]
fn test_contract_evaluation_all_branches_pass() {
    let directive = AntiSpaghettiDirective::default();
    let valid_intent = "แก้บั๊กใน src/components/ThemeToggle.tsx ปุ่มสลับธีมไม่ยอมเปลี่ยนคลาส dark";

    let receipt = InvariantContractEvaluator::evaluate(valid_intent, &directive);
    assert!(receipt.dispatch_allowed, "Valid intent must allow dispatch");
    assert_eq!(
        receipt.branches.len(),
        4,
        "Must evaluate all 4 invariant branches"
    );
    assert!(
        receipt.branches[0].passed,
        "Branch Alpha (Coordinate) must pass"
    );
    assert!(
        receipt.branches[1].passed,
        "Branch Beta (Transition) must pass"
    );
    assert!(
        receipt.branches[2].passed,
        "Branch Gamma (Invariants) must pass"
    );
    assert!(
        receipt.branches[3].passed,
        "Branch Delta (Containment) must pass"
    );

    let contract_text = receipt.contract_spec.expect("Contract must be synthesized");
    assert!(contract_text.contains("ThemeToggle.tsx"));
    assert!(contract_text.contains(
        "INVARIANT: IF UNDERSPECIFIED, NEVER GUESS. MUST TRIGGER INTERACTIVE CLARIFICATION."
    ));
}

#[test]
fn test_contract_evaluation_missing_coordinate_anchors_domain_scope() {
    let directive = AntiSpaghettiDirective::default();
    let natural_intent = "ช่วยแก้หน่อย สีปุ่มไม่สวยเลย";

    let receipt = InvariantContractEvaluator::evaluate(natural_intent, &directive);
    assert!(
        receipt.dispatch_allowed,
        "Natural intent must allow dispatch via domain-anchored scope"
    );
    assert!(
        receipt.branches[0].passed,
        "Branch Alpha must pass via domain scope fallback"
    );
    let contract = receipt
        .contract_spec
        .expect("Must emit contract with domain scope");
    assert!(contract.contains("Domain(About:"));
}

#[test]
fn test_contract_evaluation_junk_drawer_guides_containment() {
    let directive = AntiSpaghettiDirective::default();
    let junk_intent = "สร้างฟังก์ชันใน src/utils/stringHelper.ts เพื่อแปลงค่า";

    let receipt = InvariantContractEvaluator::evaluate(junk_intent, &directive);
    assert!(
        receipt.dispatch_allowed,
        "Interpreter allows execution with silent guidance rather than police halt"
    );
    assert!(
        receipt.branches[3]
            .diagnostic_message
            .contains("Containment guidance"),
        "Branch Delta must guide away from utils/ violation"
    );
}

#[test]
fn test_zero_bridge_compilation() {
    let directive = AntiSpaghettiDirective::default();
    let valid_intent = "อัปเดตไฟล์ src/components/Navbar.tsx เพื่อเพิ่มปุ่ม Logout";
    let compiled_res = ZeroBridge::compile_intent_to_contract(valid_intent, &directive);

    assert!(compiled_res.is_ok(), "ZeroBridge must compile valid intent");
    let contract = compiled_res.unwrap();
    assert!(contract.contains("Navbar.tsx"));

    let junk_intent = "สร้าง helper ใน utils/math.ts";
    let guided_res = ZeroBridge::compile_intent_to_contract(junk_intent, &directive);
    assert!(
        guided_res.is_ok(),
        "ZeroBridge as interpreter guides and compiles without crashing"
    );
}

#[test]
fn test_exploratory_inquiry_bypasses_coordinate_requirement() {
    let directive = AntiSpaghettiDirective::default();
    let inquiry_prompts = [
        "Rust คืออะไร อธิบายให้ฟังหน่อย",
        "What is Hoare Logic in computer science?",
        "ช่วยอธิบายความแตกต่างระหว่าง TCP กับ UDP",
    ];

    for prompt in inquiry_prompts {
        let receipt = InvariantContractEvaluator::evaluate(prompt, &directive);
        assert!(
            receipt.dispatch_allowed,
            "Conceptual inquiry '{}' must pass without demanding file coordinate",
            prompt
        );
        assert!(
            receipt.branches[0].passed,
            "Branch Alpha must pass for inquiry"
        );
        let contract = receipt.contract_spec.expect("Contract should be generated");
        assert!(contract.contains("ConceptualArchitecture"));
    }
}

#[test]
fn test_multilanguage_coordinates_pass_branch_alpha() {
    let directive = AntiSpaghettiDirective::default();
    let multi_lang_intents = [
        ("แก้การตั้งค่าใน Cargo.toml เพื่ออัปเดต dependency", "Cargo.toml"),
        ("อัปเดตสคริปต์ใน package.json เพิ่มคำสั่ง build", "package.json"),
        ("แก้ template ใน src/App.vue เพิ่มปุ่ม", "src/App.vue"),
        (
            "ปรับปรุงคลาส UserService.java ให้รองรับ cache",
            "UserService.java",
        ),
        (
            "แก้ memory leak ใน src/engine/core.cpp ให้เรียบร้อย",
            "src/engine/core.cpp",
        ),
        ("อัปเดตคอนฟิกใน Dockerfile ให้ใช้ Alpine", "Dockerfile"),
    ];

    for (intent, expected_coord) in multi_lang_intents {
        let receipt = InvariantContractEvaluator::evaluate(intent, &directive);
        assert!(
            receipt.dispatch_allowed,
            "Multi-language intent '{}' must allow dispatch",
            intent
        );
        assert!(
            receipt.branches[0].passed,
            "Branch Alpha must pass for coordinate '{}'",
            expected_coord
        );
        let contract = receipt.contract_spec.expect("Contract must be generated");
        assert!(
            contract.contains(expected_coord),
            "Contract must contain expected coordinate '{}'",
            expected_coord
        );
    }
}

#[test]
fn test_negative_mutation_dense_ir_generation() {
    let directive = AntiSpaghettiDirective::default();
    let negated_intent = "อย่าแก้ ThemeToggle.tsx แต่ให้คงเดิมไว้";

    let receipt = InvariantContractEvaluator::evaluate(negated_intent, &directive);
    assert!(receipt.dispatch_allowed);
    let dense_ir = receipt.diagnostic_spec.expect("Diagnostic IR must exist");
    assert!(dense_ir.contains("[GK0:PRESERVE_GUARD]"));
    assert!(dense_ir.contains("NegativeMutationBounding:STRICT"));
    assert!(dense_ir.contains("PreserveUntouchedBitForBit"));
}

#[test]
fn test_circuit_breaker_lockdown_on_looping_intent() {
    let directive = AntiSpaghettiDirective::default();
    let looping_intent = "ยังไม่ได้ พังเหมือนเดิม แก้ไม่หายสักที";

    let receipt = InvariantContractEvaluator::evaluate(looping_intent, &directive);
    assert!(
        !receipt.dispatch_allowed,
        "Circuit breaker must halt dispatch"
    );
    assert_eq!(receipt.overall_compliance, 0.0);
    let breaker_spec = receipt.diagnostic_spec.expect("Breaker IR must exist");
    assert!(breaker_spec.contains("[GK0:DIAGNOSTIC_CIRCUIT_BREAKER]"));
    assert!(breaker_spec.contains("REPEATED_FAILURE_LOCKDOWN"));
}

#[test]
fn test_invariant_rules_generation_omega() {
    let shin_rules = godkiller_zero::antigravity::generate_antigravity_rule_block("SHIN");
    assert!(shin_rules.contains("NEGATIVE MUTATION BOUNDING"));
    assert!(shin_rules.contains("LINGUISTIC CIRCUIT BREAKER"));
    assert!(shin_rules.contains("EXHAUSTIVE ERROR HANDLING"));
    assert!(shin_rules.contains("FUNCTIONAL CORE & IMPERATIVE SHELL"));
}

#[test]
fn test_negation_exemption_dont_forget() {
    use godkiller_zero::domain::LinguisticTranspiler;
    let intent = LinguisticTranspiler::transpile("don't forget to add tests for theme toggle");
    assert!(
        !intent.is_negated,
        "Positive reminder 'don't forget' must not trigger negation PRESERVE"
    );
    assert_ne!(intent.primary_action, "PRESERVE");

    let thai_intent = LinguisticTranspiler::transpile("อย่าลืมเขียนเทสด้วยนะ");
    assert!(
        !thai_intent.is_negated,
        "Thai reminder 'อย่าลืม' must not trigger negation PRESERVE"
    );
}

#[test]
fn test_gamma_invariant_fails_when_directive_exceeded() {
    let bad_directive = AntiSpaghettiDirective {
        max_cyclomatic_complexity: 99, // Exceeds CC <= 7
        ..Default::default()
    };
    let receipt = InvariantContractEvaluator::evaluate(
        "แก้ปุ่มใน src/components/ThemeToggle.tsx",
        &bad_directive,
    );
    let gamma_branch = receipt
        .branches
        .iter()
        .find(|b| b.branch_identifier == "GAMMA_INVARIANT_BUDGET")
        .unwrap();
    assert!(
        !gamma_branch.passed,
        "Gamma branch must report passed: false when CC > 7"
    );
}

#[tokio::test]
async fn test_ollama_bootstrap_manager_initial_state() {
    use godkiller_zero::upstream::{EngineStatus, OllamaBootstrapManager};
    let manager = OllamaBootstrapManager::new("qwen2.5-coder:1.5b");
    assert_eq!(manager.target_model_name(), "qwen2.5-coder:1.5b");
    let status = manager.get_status().await;
    assert_eq!(status, EngineStatus::StartingService);
}

#[tokio::test]
async fn test_standalone_contract_synthesis_output() {
    use godkiller_zero::upstream::{
        ChatCompletionInboundRequest, ChatMessagePayload, LocalZeroEngine, UpstreamLlmGateway,
    };
    let engine = LocalZeroEngine::default();
    let request = ChatCompletionInboundRequest {
        model: "godkiller-zero".to_string(),
        messages: vec![ChatMessagePayload {
            role: "user".to_string(),
            content: serde_json::Value::String("แก้ปุ่มเปลี่ยนธีม dark mode".to_string()),
        }],
        stream: false,
        temperature: None,
        passthrough_fields: serde_json::Map::new(),
    };

    let synthesized_contract = engine
        .forward_request(request)
        .await
        .expect("Forward should produce contract");
    assert!(synthesized_contract.contains("COGNITIVE PRE-FLIGHT COMPILER CONTRACT"));
    assert!(synthesized_contract.contains("GUARDRAILS & NEGATIVE INVARIANTS"));
}
