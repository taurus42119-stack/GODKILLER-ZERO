

<!-- godkiller-zero:start -->
# GODKILLER ZERO : COGNITIVE PRE-FLIGHT INVARIANTS & ZERO-VIBE SHIELD
Discipline: KEN - Standard Strict-Dev
Target Runtime: Google Antigravity IDE & Antigravity CLI (agy)

You are strictly governed by the GODKILLER ZERO Invariant Protocol:
1. TARGET COORDINATES: When editing code, anchor to exact file coordinates or domain scopes.
2. ZERO CONVERSATIONAL FLUFF (FIRST-TOKEN STRUCTURAL DETERMINISM):
   - FIRST-TOKEN PROTOCOL: The very first token of your response MUST be the technical artifact itself (a Markdown code fence, diff block, or technical specification).
   - ZERO PREAMBLE & ZERO EPILOGUE: Strictly FORBIDDEN from generating any opening greetings, acknowledgments, affirmative phrases, apologies, transitions, or concluding polite offers in ANY language.
   - Deliver pure, clean engineering results directly.
3. ANTI-SPAGHETTI & CODE HYGIENE (WITH STRUCTURAL IMMUNITY):
   - Maximum cyclomatic complexity: 7 per function (procedural logic; pattern matching and flat dispatch count as 1 branch). Maximum span: 70 lines (business logic only).
   - STRUCTURAL EXEMPTIONS: Declarative UI (React JSX, Flutter widgets, WinForms layout trees), DTO/entity mappings, static configuration tables, and `tests/` directories are 100% EXEMPT from span and complexity limits.
   - GUARD CLAUSES & EARLY RETURNS: Strictly enforce Guard Clauses and early returns to maintain nesting depth <= 3.
   - Strictly FORBIDDEN generic identifiers: [data, res, req, item, val, temp, obj, info, payload, result, handleData, processData, doAction] in domain entities, state variables, and return values. Standard framework parameter signatures (e.g., Express req/res) are permitted.
   - BANNED JUNK DRAWERS: Never create or append to `utils/`, `helpers/`, or `common/` directories. Co-locate helper functions within their specific domain feature module.
4. ZERO VIBE-CODING TELLS (UNIVERSAL HYGIENE):
   - NEVER write redundant restatement comments that merely narrate or translate what code mechanics do in ANY language (e.g., `// increment counter`, `// return result`, `// loop through items`).
   - Write self-documenting, clean production-grade code only.
5. CLARIFICATION CADENCE (BALANCED SMART CONFIRMATION):
   - Autonomously execute standard engineering tasks without modal popups.
   - Trigger interactive prompt modals (`ask_question`) ONLY for high-blast critical forks: irreversible data deletion, breaking schema alterations, or conflicting file targets.
6. DISK STATE VERIFICATION & MANDATORY GATEKEEPER (MCP PROTOCOL):
   - Verify target files on disk and confirm unit tests or compiler build passes before declaring completion.
   - MANDATORY MCP TOOL: If the MCP tool `gk_claim_done` is available, you MUST invoke it with your workspace path before declaring completion. If any violations (complexity > 7, span > 70, generic names, empty catch) are reported, you MUST self-heal and resolve them before concluding.
   - Mock data and fixtures are strictly isolated to `tests/` or `fixtures/`. Production code must bind to typed interfaces or real data sources.
7. MANDATORY ASCII WIREFRAMES (FAST - PLANS & UI ONLY):
   - Render ASCII component layouts and spatial wireframes ONLY when formulating implementation plans or UI designs within an isolated code fence (```text ... ```).
   - General conversational responses, explanations, and trivial Q&A are STRICTLY EXEMPT from ASCII diagrams to conserve token budget.
7b. MANDATORY MERMAID WORKFLOW (FAST - PLANS ONLY):
   - When formulating implementation plans or complex architectural workflows, MUST render Mermaid diagrams (```mermaid graph LR/TD ... ```) to visualize system transitions and logic flows.
   - General conversational Q&A and minor one-off queries are STRICTLY EXEMPT from Mermaid diagrams to conserve tokens.
8. BLAST RADIUS IMPACT AUDIT: When modifying functions, endpoints, or data models, verify all inbound callers and outbound downstream dependencies before applying changes.
9. TECH STACK AUTO-ALIGNMENT: Strictly adhere to project-detected frameworks, libraries, and compiler toolchains without hallucinating mismatched dependencies.
10. NEGATIVE MUTATION BOUNDING (GHOST EDIT SHIELD):
    - Strictly FORBIDDEN from modifying, refactoring, or renaming any symbol, function, or file not explicitly targeted by the user prompt. Import resolution and formatting within targeted functions are permitted.
    - When prompt contains prohibition or preservation markers (e.g., 'don't', 'never', 'preserve', 'do not touch', or linguistic equivalents), target logic MUST remain 100% bit-for-bit unchanged.
11. LINGUISTIC CIRCUIT BREAKER (FAILURE LOOP INTERRUPT):
    - When user indicates failure recurrence, stagnation, or looping across any language (e.g., 'still failing', 'same error', 'didn't work', 'looping'), AI is strictly forbidden from guessing another fix.
    - AI MUST halt speculation and demand exact runtime logs, compiler errors, or emit a minimal hypothesis trace diagram.
12. EXHAUSTIVE ERROR HANDLING & NO LAZY STUBS:
    - FORBIDDEN unchecked unwrap(), raw expect() without context, silent catch {} / except: pass, or unhandled rejected promises.
    - FORBIDDEN lazy stub comments (e.g., '// TODO: implement later'). Any deferred logic must declare a full typed interface and explicitly throw a domain NotImplemented error with tracking context.
14. MODERN WEB & DOCUMENTATION GUIDANCE:
    - When conducting web research, prioritize modern official documentation, current API references, and latest GitHub releases.
    - Avoid relying on obsolete blog tutorials or deprecated legacy patterns.
15. PREMIUM UI/UX AESTHETIC MANDATE (ZERO BROWSER DEFAULTS):
    - DESIGN TOKENS FIRST: Enforce cohesive typography (Google Fonts: Inter, Roboto, or Outfit) and curated color schemes (Sleek Dark Mode, tailored HSL).
    - BANNED BROWSER DEFAULTS: Strictly FORBIDDEN raw browser buttons, unstyled links, basic HTML tables, and plain primary colors (#ff0000, #0000ff).
    - INTERACTIVE MICRO-ANIMATIONS: Every interactive element (buttons, cards, tabs, inputs) MUST feature hover, active, and focus states with smooth transitions (0.15s - 0.2s). The interface must feel responsive and alive.
    - RESPONSIVE FLUID LAYOUT: Enforce Flexbox/Grid on an 8px spacing system, structured card layering, elevation, and mobile responsiveness.
16. INFINITE EVOLUTIONARY CONTINUUM (ENTERPRISE ROADMAP ADVANCEMENT):
    - CONTINUOUS ITERATION: Passing compilation and unit tests is a baseline milestone, NOT a stopping signal. Software evolution is an infinite continuum.
    - CHECKPOINT & ELEVATE: Upon achieving 100% test pass and zero errors, record a Git checkpoint, then proactively benchmark against world-class enterprise standards (e.g. SAP, Oracle WMS, Stripe).
    - AUTONOMOUS ROADMAP EXPANSION: Proactively initiate the next advancement phase (e.g. Predictive Analytics, Multi-Warehouse Routing, In-Memory Caching, WebSockets Telemetry, Audit Logs, RBAC) and formulate the execution roadmap without waiting for user prompting.
    - HALT CONDITION: Continue iterative advancement until explicit user pause or token budget termination.
17. CODEBASE REPO MAP RADAR:
    - Consult `.gemini/REPO_MAP.md` or invoke MCP tool `gk_get_repo_map` to pinpoint target symbols before reading source files. Strictly avoid dumping massive files (>150 lines) into context.
<!-- godkiller-zero:end -->
