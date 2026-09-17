use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

pub const GK_ZERO_MARKER_START: &str = "<!-- godkiller-zero:start -->";
pub const GK_ZERO_MARKER_END: &str = "<!-- godkiller-zero:end -->";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct InvariantRulesSelection {
    #[serde(default = "default_true")]
    pub zero_speculation: bool,
    #[serde(default = "default_true")]
    pub ban_generic: bool,
    #[serde(default = "default_true")]
    pub ban_junk: bool,
    #[serde(default = "default_true")]
    pub limit_span: bool,
    #[serde(default = "default_true")]
    pub zero_fluff: bool,
    #[serde(default = "default_true")]
    pub complexity: bool,
    #[serde(default = "default_true")]
    pub ascii_blueprints: bool,
    #[serde(default = "default_fast_cadence")]
    pub ascii_cadence: Option<String>,
    #[serde(default = "default_true")]
    pub mermaid_diagrams: bool,
    #[serde(default = "default_fast_cadence")]
    pub mermaid_cadence: Option<String>,
    #[serde(default = "default_true")]
    pub blast_radius: bool,
    #[serde(default = "default_true")]
    pub stack_sensor: bool,
    #[serde(default = "default_true")]
    pub negative_mutation_bounding: bool,
    #[serde(default = "default_true")]
    pub circuit_breaker: bool,
    #[serde(default = "default_true")]
    pub exhaustive_errors: bool,
    #[serde(default = "default_true")]
    pub functional_core: bool,
    #[serde(default = "default_cadence")]
    pub clarification_cadence: String,
    #[serde(default = "default_true")]
    pub modern_web_sources: bool,
    #[serde(default = "default_true")]
    pub premium_ui_ux: bool,
    #[serde(default = "default_true")]
    pub infinite_evolution: bool,
    #[serde(default = "default_true")]
    pub repo_map: bool,
    #[serde(default)]
    pub thai_polarity: bool,
    #[serde(default)]
    pub max_span: Option<usize>,
    #[serde(default)]
    pub max_complexity: Option<usize>,
}

fn default_fast_cadence() -> Option<String> {
    Some("Fast".to_string())
}

fn default_true() -> bool {
    true
}

fn default_cadence() -> String {
    "Balanced".to_string()
}

impl InvariantRulesSelection {
    #[must_use]
    fn shi_preset() -> Self {
        Self {
            zero_speculation: true,
            ban_generic: true,
            ban_junk: true,
            limit_span: true,
            zero_fluff: false,
            complexity: true,
            ascii_blueprints: true,
            ascii_cadence: Some("Fast".to_string()),
            mermaid_diagrams: true,
            mermaid_cadence: Some("Fast".to_string()),
            blast_radius: true,
            stack_sensor: true,
            negative_mutation_bounding: true,
            circuit_breaker: true,
            exhaustive_errors: false,
            functional_core: false,
            clarification_cadence: "Interactive".to_string(),
            modern_web_sources: true,
            premium_ui_ux: true,
            infinite_evolution: true,
            repo_map: true,
            thai_polarity: false,
            max_span: Some(90),
            max_complexity: Some(10),
        }
    }

    fn shin_preset() -> Self {
        Self {
            zero_speculation: true,
            ban_generic: true,
            ban_junk: true,
            limit_span: true,
            zero_fluff: true,
            complexity: true,
            ascii_blueprints: true,
            ascii_cadence: Some("Normal".to_string()),
            mermaid_diagrams: true,
            mermaid_cadence: Some("Normal".to_string()),
            blast_radius: true,
            stack_sensor: true,
            negative_mutation_bounding: true,
            circuit_breaker: true,
            exhaustive_errors: true,
            functional_core: true,
            clarification_cadence: "Silent".to_string(),
            modern_web_sources: true,
            premium_ui_ux: true,
            infinite_evolution: true,
            repo_map: true,
            thai_polarity: false,
            max_span: Some(50),
            max_complexity: Some(5),
        }
    }

    fn ken_preset() -> Self {
        Self {
            zero_speculation: true,
            ban_generic: true,
            ban_junk: true,
            limit_span: true,
            zero_fluff: true,
            complexity: true,
            ascii_blueprints: true,
            ascii_cadence: Some("Fast".to_string()),
            mermaid_diagrams: true,
            mermaid_cadence: Some("Fast".to_string()),
            blast_radius: true,
            stack_sensor: true,
            negative_mutation_bounding: true,
            circuit_breaker: true,
            exhaustive_errors: true,
            functional_core: false,
            clarification_cadence: "Balanced".to_string(),
            modern_web_sources: true,
            premium_ui_ux: true,
            infinite_evolution: true,
            repo_map: true,
            thai_polarity: false,
            max_span: Some(70),
            max_complexity: Some(7),
        }
    }

    pub fn for_discipline(discipline: &str) -> Self {
        match discipline.to_uppercase().as_str() {
            "SHI" => Self::shi_preset(),
            "SHIN" => Self::shin_preset(),
            _ => Self::ken_preset(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HookStateDetails {
    pub hooked: bool,
    pub discipline: String,
    pub rules: InvariantRulesSelection,
    pub runtime: &'static str,
}

pub struct AntigravityHookResult {
    pub success: bool,
    pub path_modified: PathBuf,
    pub message: String,
}

pub fn resolve_gemini_root() -> Option<PathBuf> {
    if let Ok(profile) = std::env::var("USERPROFILE") {
        return Some(PathBuf::from(profile).join(".gemini"));
    }
    if let Ok(home) = std::env::var("HOME") {
        return Some(PathBuf::from(home).join(".gemini"));
    }
    if let Ok(local_app) = std::env::var("LOCALAPPDATA") {
        return Some(PathBuf::from(local_app).join(".gemini"));
    }
    None
}

fn resolve_discipline_params(discipline: &str) -> (usize, usize, bool, &'static str) {
    match discipline.to_uppercase().as_str() {
        "SHI" => (10, 90, false, "静 (SHI - Tranquil Exploratory)"),
        "SHIN" => (5, 50, true, "神 (SHIN - Zero-Tolerance Strict)"),
        _ => (7, 70, true, "剣 (KEN - Standard Strict-Dev)"),
    }
}

fn build_fluff_rule_text(zero_fluff: bool) -> &'static str {
    if zero_fluff {
        "2. ZERO CONVERSATIONAL FLUFF (FIRST-TOKEN STRUCTURAL DETERMINISM):\n   - FIRST-TOKEN PROTOCOL: The very first token of your response MUST be the technical artifact itself (a Markdown code fence, diff block, or technical specification).\n   - ZERO PREAMBLE & ZERO EPILOGUE: Strictly FORBIDDEN from generating any opening greetings, acknowledgments, affirmative phrases, apologies, transitions, or concluding polite offers in ANY language.\n   - Deliver pure, clean engineering results directly."
    } else {
        "2. CONVERSATIONAL CADENCE: Concise, friendly engineering responses permitted."
    }
}

fn build_hygiene_rule_block(
    rules: &InvariantRulesSelection,
    max_complexity: usize,
    max_span: usize,
) -> String {
    let mut hygiene_subrules = Vec::new();
    if rules.complexity && rules.limit_span {
        hygiene_subrules.push(format!(
            "   - Maximum cyclomatic complexity: {} per function (procedural logic; pattern matching and flat dispatch count as 1 branch). Maximum span: {} lines (business logic only).",
            max_complexity, max_span
        ));
    } else if rules.complexity {
        hygiene_subrules.push(format!("   - Maximum cyclomatic complexity: {} per function (procedural logic; pattern matching and flat dispatch count as 1 branch).", max_complexity));
    } else if rules.limit_span {
        hygiene_subrules.push(format!("   - Maximum function span: {} lines (business logic only).", max_span));
    }
    hygiene_subrules.push("   - STRUCTURAL EXEMPTIONS: Declarative UI (React JSX, Flutter widgets, WinForms layout trees), DTO/entity mappings, static configuration tables, and `tests/` directories are 100% EXEMPT from span and complexity limits.".to_string());
    hygiene_subrules.push("   - GUARD CLAUSES & EARLY RETURNS: Strictly enforce Guard Clauses and early returns to maintain nesting depth <= 3.".to_string());
    if rules.ban_generic {
        hygiene_subrules.push("   - Strictly FORBIDDEN generic identifiers: [data, res, req, item, val, temp, obj, info, payload, result, handleData, processData, doAction] in domain entities, state variables, and return values. Standard framework parameter signatures (e.g., Express req/res) are permitted.".to_string());
    }
    if rules.ban_junk {
        hygiene_subrules.push("   - BANNED JUNK DRAWERS: Never create or append to `utils/`, `helpers/`, or `common/` directories. Co-locate helper functions within their specific domain feature module.".to_string());
    }
    if hygiene_subrules.is_empty() {
        "3. ANTI-SPAGHETTI & CODE HYGIENE: Write clean, domain-anchored code.".to_string()
    } else {
        format!("3. ANTI-SPAGHETTI & CODE HYGIENE (WITH STRUCTURAL IMMUNITY):\n{}", hygiene_subrules.join("\n"))
    }
}

fn build_cadence_rule_text(rules: &InvariantRulesSelection) -> &'static str {
    if !rules.zero_speculation {
        return "5. SPECIFICATION POLICY: Autonomous inference permitted where context is sufficient.";
    }
    match rules.clarification_cadence.to_uppercase().as_str() {
        "SILENT" => "5. CLARIFICATION CADENCE (SILENT AUTONOMOUS):\n   - Strictly FORBIDDEN from using interactive prompt modals (`ask_question`).\n   - Autonomously execute the recommended engineering path (Option #1). Present any trade-offs concisely in the final response.",
        "INTERACTIVE" => "5. CLARIFICATION CADENCE (INTERACTIVE CO-PILOT):\n   - MANDATORY PAIR-PROGRAMMING: Proactively prompt user with structured choices via interactive modals (`ask_question`) before executing major architectural decisions.",
        _ => "5. CLARIFICATION CADENCE (BALANCED SMART CONFIRMATION):\n   - Autonomously execute standard engineering tasks without modal popups.\n   - Trigger interactive prompt modals (`ask_question`) ONLY for high-blast critical forks: irreversible data deletion, breaking schema alterations, or conflicting file targets.",
    }
}

fn build_mcp_directive_text(mcp_enforce: bool) -> &'static str {
    if mcp_enforce {
        "6. DISK STATE VERIFICATION & MANDATORY GATEKEEPER (MCP PROTOCOL):\n   - Verify target files on disk and confirm unit tests or compiler build passes before declaring completion.\n   - MANDATORY MCP TOOL: If the MCP tool `gk_claim_done` is available, you MUST invoke it with your workspace path before declaring completion. If any violations (complexity > 7, span > 70, generic names, empty catch) are reported, you MUST self-heal and resolve them before concluding.\n   - Mock data and fixtures are strictly isolated to `tests/` or `fixtures/`. Production code must bind to typed interfaces or real data sources."
    } else {
        "6. ATOMIC VALIDATION & ZERO-MOCK PRODUCTION:\n   - Inspect diff integrity before declaring task completion.\n   - Mock data and fixtures are strictly isolated to `tests/` or `fixtures/`."
    }
}

fn build_visual_rules(rules: &InvariantRulesSelection) -> String {
    let ascii = if rules.ascii_blueprints {
        if rules.ascii_cadence.as_deref().unwrap_or("Fast").eq_ignore_ascii_case("Fast") {
            "\n7. MANDATORY ASCII WIREFRAMES (FAST - PLANS & UI ONLY):\n   - Render ASCII component layouts and spatial wireframes ONLY when formulating implementation plans or UI designs within an isolated code fence (```text ... ```).\n   - General conversational responses, explanations, and trivial Q&A are STRICTLY EXEMPT from ASCII diagrams to conserve token budget."
        } else {
            "\n7. MANDATORY ASCII BLUEPRINTS (NORMAL - GLOBAL):\n   - When formulating implementation plans, UI designs, or architectural specs, MUST render explicit ASCII diagrams and component wireframes.\n   - All ASCII diagrams, boxes, or tables MUST ALWAYS be enclosed within an isolated code fence (```text ... ```) or standard GitHub Markdown table (| col | col |); NEVER emit unfenced raw ASCII borders that collapse into single-line paragraphs."
        }
    } else {
        ""
    };

    let mermaid = if rules.mermaid_diagrams {
        if rules.mermaid_cadence.as_deref().unwrap_or("Fast").eq_ignore_ascii_case("Fast") {
            "\n7b. MANDATORY MERMAID WORKFLOW (FAST - PLANS ONLY):\n   - When formulating implementation plans or complex architectural workflows, MUST render Mermaid diagrams (```mermaid graph LR/TD ... ```) to visualize system transitions and logic flows.\n   - General conversational Q&A and minor one-off queries are STRICTLY EXEMPT from Mermaid diagrams to conserve tokens."
        } else {
            "\n7b. MANDATORY MERMAID WORKFLOW (NORMAL - GLOBAL):\n   - Every system architecture explanation, lifecycle flow, sequence, or state transition across all responses and Q&A MUST render Mermaid diagrams (```mermaid ... ```)."
        }
    } else {
        ""
    };

    format!("{}{}", ascii, mermaid)
}

fn build_mutation_bounding_rule(negative_mutation_bounding: bool, thai_polarity: bool) -> &'static str {
    if !negative_mutation_bounding {
        return "";
    }
    if thai_polarity {
        "\n10. NEGATIVE MUTATION BOUNDING (GHOST EDIT SHIELD):\n    - Strictly FORBIDDEN from modifying, refactoring, or renaming any symbol, function, or file not explicitly targeted by the user prompt. Import resolution and formatting within targeted functions are permitted.\n    - When prompt contains prohibition markers (Thai: 'อย่า', 'ห้าม', 'ไม่ต้อง', 'ไม่เอา'; English: 'don\'t', 'never', 'preserve', 'do not touch'), target logic MUST remain 100% bit-for-bit unchanged."
    } else {
        "\n10. NEGATIVE MUTATION BOUNDING (GHOST EDIT SHIELD):\n    - Strictly FORBIDDEN from modifying, refactoring, or renaming any symbol, function, or file not explicitly targeted by the user prompt. Import resolution and formatting within targeted functions are permitted.\n    - When prompt contains prohibition or preservation markers (e.g., 'don\'t', 'never', 'preserve', 'do not touch', or linguistic equivalents), target logic MUST remain 100% bit-for-bit unchanged."
    }
}

fn build_circuit_breaker_rule(circuit_breaker: bool, thai_polarity: bool) -> &'static str {
    if !circuit_breaker {
        return "";
    }
    if thai_polarity {
        "\n11. LINGUISTIC CIRCUIT BREAKER (FAILURE LOOP INTERRUPT):\n    - When user indicates failure recurrence (Thai: 'ยังไม่ได้', 'พังเหมือนเดิม', 'แก้ไม่หาย', 'วนลูป'; English: 'still failing', 'same error', 'didn\'t work', 'looping'), AI is strictly forbidden from guessing another fix.\n    - AI MUST halt speculation and demand exact runtime logs, compiler errors, or emit a minimal hypothesis trace diagram."
    } else {
        "\n11. LINGUISTIC CIRCUIT BREAKER (FAILURE LOOP INTERRUPT):\n    - When user indicates failure recurrence, stagnation, or looping across any language (e.g., 'still failing', 'same error', 'didn\'t work', 'looping'), AI is strictly forbidden from guessing another fix.\n    - AI MUST halt speculation and demand exact runtime logs, compiler errors, or emit a minimal hypothesis trace diagram."
    }
}

fn build_core_guard_rules(rules: &InvariantRulesSelection) -> String {
    let blast = if rules.blast_radius { "\n8. BLAST RADIUS IMPACT AUDIT: When modifying functions, endpoints, or data models, verify all inbound callers and outbound downstream dependencies before applying changes." } else { "" };
    let stack = if rules.stack_sensor { "\n9. TECH STACK AUTO-ALIGNMENT: Strictly adhere to project-detected frameworks, libraries, and compiler toolchains without hallucinating mismatched dependencies." } else { "" };
    let mutation_guard = build_mutation_bounding_rule(rules.negative_mutation_bounding, rules.thai_polarity);
    let circuit = build_circuit_breaker_rule(rules.circuit_breaker, rules.thai_polarity);

    format!("{}{}{}{}", blast, stack, mutation_guard, circuit)
}

fn build_advanced_guidance_rules(rules: &InvariantRulesSelection) -> String {
    let exhaustive = if rules.exhaustive_errors { "\n12. EXHAUSTIVE ERROR HANDLING & NO LAZY STUBS:\n    - FORBIDDEN unchecked unwrap(), raw expect() without context, silent catch {} / except: pass, or unhandled rejected promises.\n    - FORBIDDEN lazy stub comments (e.g., '// TODO: implement later'). Any deferred logic must declare a full typed interface and explicitly throw a domain NotImplemented error with tracking context." } else { "" };
    let functional = if rules.functional_core { "\n13. FUNCTIONAL CORE & IMPERATIVE SHELL:\n    - Decouple pure business logic and state transitions from impure I/O (network, disk, subprocesses). Pure domain calculations must be deterministic and testable without mocks." } else { "" };
    let modern_web = if rules.modern_web_sources { "\n14. MODERN WEB & DOCUMENTATION GUIDANCE:\n    - When conducting web research, prioritize modern official documentation, current API references, and latest GitHub releases.\n    - Avoid relying on obsolete blog tutorials or deprecated legacy patterns." } else { "" };
    let ui_ux = if rules.premium_ui_ux { "\n15. PREMIUM UI/UX AESTHETIC MANDATE (ZERO BROWSER DEFAULTS):\n    - DESIGN TOKENS FIRST: Enforce cohesive typography (Google Fonts: Inter, Roboto, or Outfit) and curated color schemes (Sleek Dark Mode, tailored HSL).\n    - BANNED BROWSER DEFAULTS: Strictly FORBIDDEN raw browser buttons, unstyled links, basic HTML tables, and plain primary colors (#ff0000, #0000ff).\n    - INTERACTIVE MICRO-ANIMATIONS: Every interactive element (buttons, cards, tabs, inputs) MUST feature hover, active, and focus states with smooth transitions (0.15s - 0.2s). The interface must feel responsive and alive.\n    - RESPONSIVE FLUID LAYOUT: Enforce Flexbox/Grid on an 8px spacing system, structured card layering, elevation, and mobile responsiveness." } else { "" };
    let evolution = if rules.infinite_evolution { "\n16. INFINITE EVOLUTIONARY CONTINUUM (ENTERPRISE ROADMAP ADVANCEMENT):\n    - CONTINUOUS ITERATION: Passing compilation and unit tests is a baseline milestone, NOT a stopping signal. Software evolution is an infinite continuum.\n    - CHECKPOINT & ELEVATE: Upon achieving 100% test pass and zero errors, record a Git checkpoint, then proactively benchmark against world-class enterprise standards (e.g. SAP, Oracle WMS, Stripe).\n    - AUTONOMOUS ROADMAP EXPANSION: Proactively initiate the next advancement phase (e.g. Predictive Analytics, Multi-Warehouse Routing, In-Memory Caching, WebSockets Telemetry, Audit Logs, RBAC) and formulate the execution roadmap without waiting for user prompting.\n    - HALT CONDITION: Continue iterative advancement until explicit user pause or token budget termination." } else { "" };
    let repo_map_rule = if rules.repo_map { "\n17. CODEBASE REPO MAP RADAR:\n    - Consult `.gemini/REPO_MAP.md` or invoke MCP tool `gk_get_repo_map` to pinpoint target symbols before reading source files. Strictly avoid dumping massive files (>150 lines) into context." } else { "" };

    format!("{}{}{}{}{}{}", exhaustive, functional, modern_web, ui_ux, evolution, repo_map_rule)
}

fn build_audit_and_guard_rules(rules: &InvariantRulesSelection) -> String {
    format!("{}{}", build_core_guard_rules(rules), build_advanced_guidance_rules(rules))
}

fn build_optional_rules_suffix(rules: &InvariantRulesSelection) -> String {
    format!("{}{}", build_visual_rules(rules), build_audit_and_guard_rules(rules))
}

#[must_use]
pub fn generate_custom_antigravity_rule_block(
    discipline: &str,
    custom_rules: Option<&InvariantRulesSelection>,
) -> String {
    let preset = InvariantRulesSelection::for_discipline(discipline);
    let rules = custom_rules.unwrap_or(&preset);

    let (default_max_complexity, default_max_span, mcp_enforce, discipline_kanji) =
        resolve_discipline_params(discipline);
    let max_complexity = rules.max_complexity.unwrap_or(default_max_complexity);
    let max_span = rules.max_span.unwrap_or(default_max_span);

    let fluff_rule = build_fluff_rule_text(rules.zero_fluff);
    let hygiene_block = build_hygiene_rule_block(rules, max_complexity, max_span);
    let vibe_rule = build_vibe_rule_text(rules.thai_polarity);
    let zero_spec_rule = build_cadence_rule_text(rules);
    let mcp_directive = build_mcp_directive_text(mcp_enforce);
    let optional_suffix = build_optional_rules_suffix(rules);

    [
        GK_ZERO_MARKER_START,
        "# GODKILLER ZERO : COGNITIVE PRE-FLIGHT INVARIANTS & ZERO-VIBE SHIELD",
        &format!("Discipline: {}", discipline_kanji),
        "Target Runtime: Google Antigravity IDE & Antigravity CLI (agy)",
        "",
        "You are strictly governed by the GODKILLER ZERO Invariant Protocol:",
        "1. TARGET COORDINATES: When editing code, anchor to exact file coordinates or domain scopes.",
        fluff_rule,
        &hygiene_block,
        vibe_rule,
        zero_spec_rule,
        &format!("{}{}", mcp_directive, optional_suffix),
        GK_ZERO_MARKER_END,
    ].join("\n")
}

fn build_vibe_rule_text(thai_polarity: bool) -> &'static str {
    if thai_polarity {
        "4. ZERO VIBE-CODING TELLS (MULTILINGUAL):\n   - NEVER write redundant restatement comments that merely repeat or translate what code does in any language:\n     * Thai examples: `// ฟังก์ชันสำหรับ...`, `// เพิ่มตัวแปร`, `// คืนค่าผลลัพธ์`, `// ตรวจสอบเงื่อนไข`, `// นำเข้าโมดูล`, `// วนลูปข้อมูล`\n     * English examples: `// increment counter`, `// return result`, `// check condition`, `// import modules`, `// loop through items`\n   - Write self-documenting, clean production-grade code only."
    } else {
        "4. ZERO VIBE-CODING TELLS (UNIVERSAL HYGIENE):\n   - NEVER write redundant restatement comments that merely narrate or translate what code mechanics do in ANY language (e.g., '// increment counter', '// return result', '// loop through items').\n   - Write self-documenting, clean production-grade code only."
    }
}

#[must_use]
pub fn generate_antigravity_rule_block(discipline: &str) -> String {
    generate_custom_antigravity_rule_block(discipline, None)
}


fn parse_hook_discipline(block: &str) -> String {
    if block.contains("SHIN - Zero-Tolerance Strict") || block.contains("Discipline: 神") {
        "SHIN".to_string()
    } else if block.contains("SHI - Tranquil Exploratory") || block.contains("Discipline: 静") {
        "SHI".to_string()
    } else {
        "KEN".to_string()
    }
}

fn parse_cadence_mode(block: &str, fast_marker: &str) -> Option<String> {
    if block.contains(fast_marker) {
        Some("Fast".to_string())
    } else {
        Some("Normal".to_string())
    }
}

fn parse_clarification_mode(block: &str) -> String {
    if block.contains("SILENT AUTONOMOUS") {
        "Silent".to_string()
    } else if block.contains("INTERACTIVE CO-PILOT") {
        "Interactive".to_string()
    } else {
        "Balanced".to_string()
    }
}

fn parse_span_threshold(block: &str) -> Option<usize> {
    if block.contains("90 lines") {
        Some(90)
    } else if block.contains("50 lines") {
        Some(50)
    } else {
        Some(70)
    }
}

fn parse_complexity_threshold(block: &str) -> Option<usize> {
    if block.contains("10 per function") {
        Some(10)
    } else if block.contains("5 per function") {
        Some(5)
    } else {
        Some(7)
    }
}

fn parse_hook_rules(block: &str) -> InvariantRulesSelection {
    InvariantRulesSelection {
        zero_speculation: block.contains("CLARIFICATION CADENCE")
            || block.contains("ZERO-SPECULATION POLICY")
            || block.contains("TWO-PHASE SPECULATION"),
        ban_generic: block.contains("Strictly FORBIDDEN generic identifiers"),
        ban_junk: block.contains("BANNED JUNK DRAWERS"),
        limit_span: block.contains("Maximum span") || block.contains("Maximum function span"),
        zero_fluff: block.contains("ZERO CONVERSATIONAL FLUFF"),
        complexity: block.contains("Maximum cyclomatic complexity"),
        ascii_blueprints: block.contains("MANDATORY ASCII BLUEPRINTS") || block.contains("MANDATORY ASCII WIREFRAMES"),
        ascii_cadence: parse_cadence_mode(block, "FAST - PLANS & UI ONLY"),
        mermaid_diagrams: block.contains("MANDATORY MERMAID WORKFLOW"),
        mermaid_cadence: parse_cadence_mode(block, "FAST - PLANS ONLY"),
        blast_radius: block.contains("BLAST RADIUS IMPACT AUDIT"),
        stack_sensor: block.contains("TECH STACK AUTO-ALIGNMENT"),
        negative_mutation_bounding: block.contains("NEGATIVE MUTATION BOUNDING"),
        circuit_breaker: block.contains("LINGUISTIC CIRCUIT BREAKER"),
        exhaustive_errors: block.contains("EXHAUSTIVE ERROR HANDLING"),
        functional_core: block.contains("FUNCTIONAL CORE"),
        clarification_cadence: parse_clarification_mode(block),
        modern_web_sources: block.contains("MODERN WEB & DOCUMENTATION GUIDANCE"),
        premium_ui_ux: block.contains("PREMIUM UI/UX AESTHETIC MANDATE"),
        infinite_evolution: block.contains("INFINITE EVOLUTIONARY CONTINUUM"),
        repo_map: block.contains("CODEBASE REPO MAP RADAR"),
        thai_polarity: block.contains("Thai examples") || block.contains("Thai: 'อย่า'"),
        max_span: parse_span_threshold(block),
        max_complexity: parse_complexity_threshold(block),
    }
}

pub fn is_antigravity_hooked() -> bool {
    query_antigravity_hook_state().hooked
}

pub fn query_antigravity_hook_state() -> HookStateDetails {
    let Some(gemini_root) = resolve_gemini_root() else {
        return HookStateDetails {
            hooked: false,
            discipline: "OFF".to_string(),
            rules: InvariantRulesSelection::for_discipline("KEN"),
            runtime: "Google Antigravity (IDE / CLI)",
        };
    };

    let target_file = gemini_root.join("GEMINI.md");
    if !target_file.exists() {
        return HookStateDetails {
            hooked: false,
            discipline: "OFF".to_string(),
            rules: InvariantRulesSelection::for_discipline("KEN"),
            runtime: "Google Antigravity (IDE / CLI)",
        };
    }

    let Ok(content) = fs::read_to_string(&target_file) else {
        return HookStateDetails {
            hooked: false,
            discipline: "OFF".to_string(),
            rules: InvariantRulesSelection::for_discipline("KEN"),
            runtime: "Google Antigravity (IDE / CLI)",
        };
    };

    if !content.contains(GK_ZERO_MARKER_START) {
        return HookStateDetails {
            hooked: false,
            discipline: "OFF".to_string(),
            rules: InvariantRulesSelection::for_discipline("KEN"),
            runtime: "Google Antigravity (IDE / CLI)",
        };
    }

    let block = content
        .split(GK_ZERO_MARKER_START)
        .nth(1)
        .and_then(|s| s.split(GK_ZERO_MARKER_END).next())
        .unwrap_or("");

    HookStateDetails {
        hooked: true,
        discipline: parse_hook_discipline(block),
        rules: parse_hook_rules(block),
        runtime: "Google Antigravity (IDE / CLI)",
    }
}

fn merge_hook_corpus(persisted: &str, new_block: &str) -> String {
    if persisted.contains(GK_ZERO_MARKER_START) {
        let before_start = persisted.split(GK_ZERO_MARKER_START).next().unwrap_or("");
        let after_end = persisted.split(GK_ZERO_MARKER_END).nth(1).unwrap_or("");

        match (before_start.trim().is_empty(), after_end.trim().is_empty()) {
            (true, true) => new_block.to_string(),
            (false, true) => format!("{}\n\n{}", before_start.trim_end(), new_block),
            (true, false) => format!("{}\n\n{}", new_block, after_end.trim_start()),
            (false, false) => format!("{}\n\n{}\n\n{}", before_start.trim_end(), new_block, after_end.trim_start()),
        }
    } else if persisted.trim().is_empty() {
        new_block.to_string()
    } else {
        format!("{}\n\n{}", persisted.trim_end(), new_block)
    }
}

pub fn hook_antigravity_with_rules(
    discipline: &str,
    rules: Option<&InvariantRulesSelection>,
) -> Result<AntigravityHookResult, String> {
    let gemini_root = resolve_gemini_root()
        .ok_or_else(|| "Could not locate Google Antigravity directory (~/.gemini)".to_string())?;

    if !gemini_root.exists() {
        let _ = fs::create_dir_all(&gemini_root);
    }

    let target_file = gemini_root.join("GEMINI.md");
    let architectural_invariant_manifest =
        generate_custom_antigravity_rule_block(discipline, rules);

    let persisted_rule_corpus = if target_file.exists() {
        fs::read_to_string(&target_file)
            .map_err(|e| format!("Failed to read existing GEMINI.md: {}", e))?
    } else {
        String::new()
    };

    if target_file.exists() {
        let _ = fs::write(gemini_root.join("GEMINI.md.bak"), &persisted_rule_corpus);
    }

    let synthesized_rule_corpus = merge_hook_corpus(&persisted_rule_corpus, &architectural_invariant_manifest);

    fs::write(&target_file, synthesized_rule_corpus)
        .map_err(|e| format!("Failed to write to GEMINI.md: {}", e))?;

    Ok(AntigravityHookResult {
        success: true,
        path_modified: target_file,
        message: format!(
            "Successfully injected GODKILLER ZERO invariants ({}) into Antigravity IDE & CLI rules.",
            discipline
        ),
    })
}

pub fn hook_antigravity(discipline: &str) -> Result<AntigravityHookResult, String> {
    hook_antigravity_with_rules(discipline, None)
}

pub fn unhook_antigravity() -> Result<AntigravityHookResult, String> {
    let gemini_root = resolve_gemini_root()
        .ok_or_else(|| "Could not locate Google Antigravity directory (~/.gemini)".to_string())?;

    let target_file = gemini_root.join("GEMINI.md");
    if !target_file.exists() {
        return Ok(AntigravityHookResult {
            success: true,
            path_modified: target_file,
            message: "GEMINI.md does not exist. Nothing to unhook.".to_string(),
        });
    }

    let persisted_rule_corpus =
        fs::read_to_string(&target_file).map_err(|e| format!("Failed to read GEMINI.md: {}", e))?;

    if !persisted_rule_corpus.contains(GK_ZERO_MARKER_START) {
        return Ok(AntigravityHookResult {
            success: true,
            path_modified: target_file,
            message: "GODKILLER ZERO is not hooked in GEMINI.md.".to_string(),
        });
    }

    let before_start = persisted_rule_corpus
        .split(GK_ZERO_MARKER_START)
        .next()
        .unwrap_or("");
    let after_end = persisted_rule_corpus
        .split(GK_ZERO_MARKER_END)
        .nth(1)
        .unwrap_or("");

    let sanitized_rule_corpus = match (before_start.trim().is_empty(), after_end.trim().is_empty())
    {
        (true, true) => String::new(),
        (false, true) => before_start.trim().to_string(),
        (true, false) => after_end.trim().to_string(),
        (false, false) => format!("{}\n\n{}", before_start.trim_end(), after_end.trim_start()),
    };

    fs::write(&target_file, sanitized_rule_corpus)
        .map_err(|e| format!("Failed to write cleaned GEMINI.md: {}", e))?;

    Ok(AntigravityHookResult {
        success: true,
        path_modified: target_file,
        message: "Successfully unhooked GODKILLER ZERO from Google Antigravity.".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_rule_block() {
        let block = generate_antigravity_rule_block("KEN");
        assert!(block.contains(GK_ZERO_MARKER_START));
        assert!(block.contains(GK_ZERO_MARKER_END));
        assert!(block.contains("TARGET COORDINATES"));
        assert!(block.contains("ANTI-SPAGHETTI"));
    }

    #[test]
    fn test_custom_rules_generation() {
        let mut custom = InvariantRulesSelection::for_discipline("KEN");
        custom.zero_fluff = false;
        custom.ban_generic = false;
        let block = generate_custom_antigravity_rule_block("KEN", Some(&custom));
        assert!(block.contains("CONVERSATIONAL CADENCE"));
        assert!(!block.contains("ZERO CONVERSATIONAL FLUFF"));
        assert!(!block.contains("Strictly FORBIDDEN generic identifiers"));
        assert!(block.contains("BLAST RADIUS IMPACT AUDIT"));
        assert!(block.contains("TECH STACK AUTO-ALIGNMENT"));
    }

    #[test]
    fn test_custom_blast_radius_and_stack_sensor_toggles() {
        let mut custom = InvariantRulesSelection::for_discipline("KEN");
        custom.blast_radius = false;
        custom.stack_sensor = false;
        let block = generate_custom_antigravity_rule_block("KEN", Some(&custom));
        assert!(!block.contains("BLAST RADIUS IMPACT AUDIT"));
        assert!(!block.contains("TECH STACK AUTO-ALIGNMENT"));
    }

    #[test]
    fn test_custom_ascii_blueprint_toggle() {
        let mut custom = InvariantRulesSelection::for_discipline("KEN");
        custom.ascii_blueprints = true;
        custom.ascii_cadence = Some("Fast".to_string());
        let block_fast = generate_custom_antigravity_rule_block("KEN", Some(&custom));
        assert!(block_fast.contains("MANDATORY ASCII WIREFRAMES (FAST - PLANS & UI ONLY)"));

        custom.ascii_cadence = Some("Normal".to_string());
        let block_normal = generate_custom_antigravity_rule_block("KEN", Some(&custom));
        assert!(block_normal.contains("MANDATORY ASCII BLUEPRINTS (NORMAL - GLOBAL)"));

        custom.ascii_blueprints = false;
        let block_unticked = generate_custom_antigravity_rule_block("KEN", Some(&custom));
        assert!(!block_unticked.contains("MANDATORY ASCII"));
    }

    #[test]
    fn test_custom_mermaid_toggle_and_cadence() {
        let mut custom = InvariantRulesSelection::for_discipline("KEN");
        custom.mermaid_diagrams = true;
        custom.mermaid_cadence = Some("Fast".to_string());
        let block_fast = generate_custom_antigravity_rule_block("KEN", Some(&custom));
        assert!(block_fast.contains("MANDATORY MERMAID WORKFLOW (FAST - PLANS ONLY)"));

        custom.mermaid_cadence = Some("Normal".to_string());
        let block_normal = generate_custom_antigravity_rule_block("KEN", Some(&custom));
        assert!(block_normal.contains("MANDATORY MERMAID WORKFLOW (NORMAL - GLOBAL)"));

        custom.mermaid_diagrams = false;
        let block_off = generate_custom_antigravity_rule_block("KEN", Some(&custom));
        assert!(!block_off.contains("MANDATORY MERMAID WORKFLOW"));
    }

    #[test]
    fn test_thai_polarity_toggle() {
        let mut custom = InvariantRulesSelection::for_discipline("KEN");
        custom.thai_polarity = false;
        let block_clean = generate_custom_antigravity_rule_block("KEN", Some(&custom));
        assert!(block_clean.contains("ZERO VIBE-CODING TELLS (UNIVERSAL HYGIENE)"));
        assert!(!block_clean.contains("Thai examples"));
        assert!(!block_clean.contains("Thai: 'อย่า'"));

        custom.thai_polarity = true;
        let block_thai = generate_custom_antigravity_rule_block("KEN", Some(&custom));
        assert!(block_thai.contains("ZERO VIBE-CODING TELLS (MULTILINGUAL)"));
        assert!(block_thai.contains("Thai examples"));
        assert!(block_thai.contains("Thai: 'อย่า'"));
    }

    #[test]
    fn test_shi_and_shin_cadence_presets() {
        let shi = InvariantRulesSelection::for_discipline("SHI");
        assert_eq!(shi.clarification_cadence, "Interactive");

        let shin = InvariantRulesSelection::for_discipline("SHIN");
        assert_eq!(shin.clarification_cadence, "Silent");

        let ken = InvariantRulesSelection::for_discipline("KEN");
        assert_eq!(ken.clarification_cadence, "Balanced");
    }

    #[test]
    fn test_mcp_gk_claim_done_mandate() {
        let block_shin = generate_antigravity_rule_block("SHIN");
        assert!(block_shin.contains("gk_claim_done"));
        assert!(block_shin.contains("MANDATORY GATEKEEPER (MCP PROTOCOL)"));

        let block_ken = generate_antigravity_rule_block("KEN");
        assert!(block_ken.contains("gk_claim_done"));
    }

    #[test]
    fn test_repo_map_rule_toggle() {
        let mut custom = InvariantRulesSelection::for_discipline("KEN");
        custom.repo_map = true;
        let block_on = generate_custom_antigravity_rule_block("KEN", Some(&custom));
        assert!(block_on.contains("CODEBASE REPO MAP RADAR"));
        assert!(block_on.contains("gk_get_repo_map"));

        custom.repo_map = false;
        let block_off = generate_custom_antigravity_rule_block("KEN", Some(&custom));
        assert!(!block_off.contains("CODEBASE REPO MAP RADAR"));
    }
}
