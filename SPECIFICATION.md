# ⚡ GODKILLER ZERO (GK-ZERO)
### Official Production-Grade Technical Specification & System Blueprint
**Version:** 1.0.0-GA | **Codename:** Zero-Entropy (零) / ZERO 1.0 | **Default Port:** `4242`  
**Philosophy:** Zen Minimalist Architecture (日本のミニマリズム) × Zero-Overhead Compact Utility  
**Target:** Native Core Engine (`godkiller-console.exe`) + GODKILLER ZERO 1.0 Desktop App (`GodkillerZero.exe`) with System Tray & Desktop Shortcut | **License:** MIT (100% Free & Open Source)

---

## 1. Executive Vision & The Universal Invariant Shield

**GODKILLER ZERO** คือ *Universal Cognitive Pre-flight Firewall & Invariant Architecture Gatekeeper* ประจำการที่เครื่องของผู้พัฒนา ทำหน้าที่ดักจับภาษาธรรมชาติ (High-Entropy Natural Language) เพื่อควบคุมโมเดลภาษาขนาดใหญ่ (LLMs) และกำกับวินัยของ AI Coding Assistants ข้ามทุกแพลตฟอร์ม:
* **Google Antigravity IDE** & **Google Antigravity CLI (`agy`) / Antigravity 2.0**
* **Cursor IDE (`.cursorrules`)**
* **Anthropic Claude Code (`CLAUDE.md`)**
* **VS Code GitHub Copilot (`.github/copilot-instructions.md`)**

```text
       [ ทั่วไป / Vibe Coding ]                        [ GODKILLER ZERO : UNIVERSAL SUITE ]
                                               ┌──────────────────────────────────────┐
ภาษามนุษย์เยิ่นเย้อ (High Entropy) ─────────────►│ GODKILLER ZERO                       │
                                               │ (1-Click Universal Hook / Crucible)  │
                                               └──────────────────┬───────────────────┘
                                                                  │ (Hoare-Logic IR + 16 Invariants)
                                                                  ▼
                                               ┌──────────────────────────────────────┐
                                               │ ANTIGRAVITY / CLAUDE / CURSOR / COPILOT
                                               │ (Planning Mode / Autonomous Engine)  │
                                               └──────────────────┬───────────────────┘
                                                                  │ (Tool Calling / Disk Edits)
                                                                  ▼
                                               ┌──────────────────────────────────────┐
                                               │ GATEKEEPER & MCP DISK GATE           │
                                               │ (CC <= 7, Span <= 70, claim_done)    │
                                               └──────────────────────────────────────┘
```

### ปรัชญา Contrarian Arbitrage & Local-First Ingress
* **ความเชื่อกระแสหลัก:** ผู้คนมองว่าโมเดลประหยัดอย่าง Gemini Flash หรือ Local Models "ชอบแถและเขียนโค้ดสปาเกตตี้" จึงแห่ไปจ่ายเงินแพงๆ ให้โมเดลตัวท็อป
* **ความจริงทางวิศวกรรม:** โมเดลความเร็วสูงล้มเหลวเพราะ "Entropy ของภาษามนุษย์สูงเกินไป" และขาด Guardrail ด้านลบ (Negative Invariants)
* **พันธกิจของ GK-ZERO:** แปลงเจตจำนงกำกวมให้กลายเป็นสัญญาคณิตศาสตร์ Zero-Entropy บังคับสัจพจน์ด้านลบ 16 ข้อ และรัน **Local Zero Engine (Ollama `qwen2.5-coder:1.5b`)** ประหยัดงบ Cloud Token 100%

---

## 2. การรักษาความปลอดภัยและการป้องกันระบบ (Security & Hardening)

```text
               ┌────────────────────────────────────────┐
               │         SECURITY PERIMETER             │
               │                                        │
AI Clients ───►│  [Strict Loopback Binding: 127.0.0.1]  │ (ปฏิเสธ 0.0.0.0 ถาวร)
               │  [Origin Guard: Block External CSRF]   │ (บล็อก Origin ภายนอกทั้ง HTTP/HTTPS)
               │  [Zero Telemetry / Zero Cloud Storage] │ (ไม่มี Analytics ใดๆ)
               │  [Local-First Engine: Zero Cloud Keys] │ (ไม่ต้องใช้ API Key เมฆ)
               │  [Path Remapping: Clean Binary Symbols]│ (ซ่อนชื่อผู้ใช้และ Local Path จากไบนารี)
               └────────────────────────────────────────┘
```

### 2.1 Network & Socket Isolation
* **Strict Loopback Binding:** ผูกกับ `127.0.0.1:4242` เท่านั้น **ห้ามผูกกับ `0.0.0.0` เด็ดขาด** เพื่อป้องกันอุปกรณ์อื่นในเครือข่าย LAN ยิง Request เข้ามา
* **Hardened Host-Authority Origin Guard:** ตรวจสอบ Authority/Host ของ Header `Origin` และ `Referer` ผ่าน Axum Middleware ทุก Request ป้องกัน Subdomain Spoofing (`localhost.evil.com`) โดยอนุญาตเฉพาะ Local Hostname แท้จริง (`localhost`, `127.0.0.1`) และตัด CORS Wildcard ทิ้ง 100%
* **Zero Privilege Escalation:** รันในระดับสิทธิ์ผู้ใช้ปกติ (Standard User) ไม่ต้องใช้สิทธิ์ Administrator

### 2.2 Local-First & Zero Cloud Leak Policy
* **Local Neural Engine:** ใช้ **Local Zero Engine** ร่วมกับ **Autonomous Ollama Bootstrap (`qwen2.5-coder:1.5b`)** ทำงานแบบ Offline-First 100% ไร้ความเสี่ยงเรื่อง API Key รั่วไหล
* **Binary Path Remapping:**
  * ฝั่ง Rust: ตั้งค่า `--remap-path-prefix` เพื่อตัดพาธไดเรกทอรีในเครื่องผู้พัฒนาให้เหลือ `/godkiller-zero` และ `/user`
  * ฝั่ง C# GUI: กำหนด `<Deterministic>true</Deterministic>` และ `<PathMap>` ใน `gui/GodkillerZero.csproj` ป้องกันการรั่วไหลของ Absolute Path ใน `.pdb` และ `.dll`

### 2.3 Prompt Injection & Malicious Content Sanitizer
* ตรวจจับและ Escape โทเคนพิเศษที่อาจใช้ Hijack System Prompt เช่น `<|endoftext|>`, `[INST]`, `[/INST]`, `system:`, `<fim_prefix>`, `<fim_suffix>`
* บังคับใช้ Context Boundary Tags แยก "คำสั่งของผู้ใช้" ออกจาก "ข้อมูลโค้ด"

---

## 3. สถาปัตยกรรมสัญญา Isomorphic & กฎเหล็ก 16 สัจพจน์ (The 16 Invariant Rules)

```text
[ เจตจำนงมนุษย์ ] ──► [ LINGUISTIC TRANSPILER ] ──► [ 4-BRANCH INVARIANT EVALUATOR ]
                                │
               ┌────────────────┴────────────────┐
               ▼                                 ▼
      [ Exploratory Inquiry ]          [ Architectural Mutation ]
      • ถามความรู้ / อธิบายทฤษฎี        • สั่งแก้โค้ด / ลบ / เพิ่มฟังก์ชัน
      • ยกเว้นพิกัดไฟล์ (Domain Scope)  • บังคับ Invariant Contract {P} Target {Q}
      • บังคับตอบกระชับ ไร้น้ำ         • บังคับ 16 Invariant Rules & CC <= 7
```

### 3.1 Structural Invariant Contract (Pre/Post-Condition Specification)
The evaluator structures intent through structural pre- and post-condition invariants:
$$\{ \mathcal{P}_{\text{pre}} \} \quad \mathcal{C}[\text{Coordinate}] \quad \{ \mathcal{Q}_{\text{post}} \} \quad \text{bound by} \quad \mathcal{I}_{\text{invariants}}$$

### 3.2 บัญญัติ 16 สัจพจน์สถาปัตยกรรม (The 16 Ironclad Rules)

| # | ชื่อกฎ (Invariant Rule) | กลไกการบังคับใช้ | รายละเอียดการคุ้มครอง |
|---|---|:---:|---|
| **1** | **Target Coordinates** | 🔵 Hybrid (AST Discovery + Hook) | ตรวจจับพิกัดไฟล์เป้าหมายจากเจตจำนงภาษาไทย/อังกฤษ |
| **2** | **Zero Conversational Fluff** | 🟡 Soft (First-Token Protocol) | บังคับ Token แรกเป็น Diff หรือ Code ทันที ไร้น้ำ ไร้คำทักทายเยิ่นเย้อ |
| **3** | **Anti-Spaghetti & Code Hygiene** | 🔴 Hard (Gatekeeper Disk Scanner) | Cyclomatic Complexity $\le 7$, Function Span $\le 70$ บรรทัด |
| **4** | **Zero Vibe-Coding Tells** | 🔴 Hard (Gatekeeper Identifiers) | แบนชื่อตัวแปรขี้เกียจ: `data`, `res`, `req`, `item`, `temp`, `val`, `payload` |
| **5** | **Clarification Cadence** | 🟡 Soft (Tri-Pillar Discipline) | 3 โหมดชัดเจน: SHI (เงียบ), KEN (ถามเฉพาะ critical), SHIN (สถาปัตย์ลึก) |
| **6** | **Disk State Verification** | 🔴 Hard (MCP / Gatekeeper) | ตรวจสอบไฟล์บนดิสก์จริงและผลคอมไพล์ก่อนสรุปจบงาน ห้ามส่ง Mock หลุด |
| **7** | **Mandatory ASCII Blueprints** | 🟡 Soft (Prompt Invariant) | บังคับวาดพิมพ์เขียว ASCII ใน Code Fence (```text) สำหรับโครงสร้าง UI 2 มิติ (เลือกระดับได้: Fast = เฉพาะตอนวางแผน/UI, Normal = ทุกการตอบ) |
| **7b** | **Mandatory Mermaid Diagrams** | 🟡 Soft (Prompt Invariant) | แผนภาพ Flowchart / State Diagram ใน Code Fence (```mermaid) สำหรับ Flow การทำงาน (เลือกระดับได้: Fast = เฉพาะ Plan, Normal = ทุกคำตอบ) |
| **8** | **Blast Radius Impact Audit** | 🔴 Hard (Symbol Graph) | ตรวจสอบ Inbound/Outbound Callers และ Dependency ก่อนแก้ไขฟังก์ชัน |
| **9** | **Tech Stack Auto-Alignment** | 🔴 Hard (Universal Stack Sensor) | สแกน Manifest (`package.json`, `Cargo.toml`) ป้องกันการหลอน Dependency |
| **10** | **Negative Mutation Bounding** | 🔴 Hard (Linguistic Polarity Guard) | ตรวจจับคำสั่งห้าม ("อย่า", "don't", "preserve") ป้องกัน Ghost Edit 100% |
| **11** | **Linguistic Circuit Breaker** | 🔴 Hard (Failure Loop Interrupt) | ตัดวงจร AI เดามั่วเมื่อพบคำว่า "ยังไม่ได้", "เหมือนเดิม", "looping" บังคับขอดู Log |
| **12** | **Exhaustive Error Handling** | 🟡 Soft (Prompt Invariant) | ห้าม `unwrap()`, `expect()` ดิบ, `// TODO`, `silent catch {}` |
| **13** | **Functional Core & Shell** | 🟡 Soft (Architecture Rule) | แยก Pure Business Logic ออกจาก I/O เพื่อให้เทสท์ง่ายโดยไม่ต้อง Mock |
| **14** | **Modern Web & Doc Sources** | 🟡 Soft (Source Guidance) | บังคับอ้างอิงเอกสารและ Framework ยุคใหม่ เลิกใช้รูปแบบ Legacy |
| **15** | **Premium UI/UX Mandate** | 🟡 Soft (Design System Rule) | แบนปุ่ม Browser ค่าเริ่มต้น บังคับ Design Tokens, Micro-animations, Dark Mode |
| **16** | **Infinite Evolutionary Continuum** | 🟡 Soft (Enterprise Advancement) | การผ่านเทสท์ไม่ใช่จุดสิ้นสุด ผลักดันสถาปัตย์สู่ระดับ Enterprise ต่อเนื่อง |

---

## 4. สถาปัตยกรรม UI / UX: Zen Minimalist & Zero-Overhead Compact Card Utility

### 4.1 Window Dimensions & Form Factors
1. **Edge App Mode (Rust Embedded Web UI):** ขนาดหน้าต่างมาตรฐาน **`380px × 560px`** รันผ่าน Microsoft Edge `--app` mode ปราศจาก Browser Shell
2. **Native C# WinForms Suite (`GodkillerZero.exe`):** ขนาดหน้าต่างมาตรฐาน **`380px × 610px`** พิกัด Bottom-Right สไตล์ Compact Tray Utility พร้อม System Tray

---

## 5. สถาปัตยกรรมภายในแบบ Hexagonal Clean Architecture

```text
┌────────────────────────────────────────────────────────┐
│                   1. ADAPTERS (UI / CLI)               │
│      (Embedded Web UI / C# Desktop GUI / CLI Engine)   │
└───────────────────────────┬────────────────────────────┘
                            │
┌───────────────────────────▼────────────────────────────┐
│               2. INGRESS & PROXY ENGINE                │
│       (Axum Loopback Server, Origin Sanitizer, SSE)    │
└───────────────────────────┬────────────────────────────┘
                            │
┌───────────────────────────▼────────────────────────────┐
│               3. CORE DOMAIN & COMPILER                │
│   (Gatekeeper Scanner, Linguistic Transpiler, Hoare)   │
└───────────────────────────┬────────────────────────────┘
                            │
┌───────────────────────────▼────────────────────────────┐
│              4. UPSTREAM ADAPTERS (Local Engines)      │
│     (Local Zero Engine with Autonomous Ollama Bootstrap)│
└────────────────────────────────────────────────────────┘
```

---

## 6. Continuous Integration & Quality Gates

* **สถานะชุดทดสอบปัจจุบัน:** **66 / 66 Tests Passed (100% Pass Rate)**
  * `src/lib.rs`: 44 Unit Tests (รวม Rule 12 Multi-Language Stubs, Smart Terminal Pruning, Semantic AST, และ Banned Identifiers)
  * `tests/antigravity_isolation_test.rs`: 8 Integration Tests (รวม Loopback Security, Thai Polarity, และ Envelope Isolation)
  * `tests/contract_evaluation_test.rs`: 14 Domain & Contract Tests (รวม Invariant Contract, Multi-language Coordinates, และ Intent Circuit Breaker)
* **CI Pipeline (`.github/workflows/ci.yml` & `release.yml`):** รัน `cargo fmt`, `cargo clippy -D warnings`, `cargo test`, และคอมไพล์ release binary พร้อม sanitize path ผ่าน `--remap-path-prefix` ใน CI

---

## 7. แผนที่โครงสร้างไฟล์โปรเจกต์จริง (Verified Project Directory Tree)

```text
GODKILLER ZERO/
├── .github/
│   └── workflows/
│       ├── ci.yml                     # Automated Test, Clippy, Security Audit
│       ├── release.yml                # Automated Windows Release (.exe) Pipeline with Path Remapping
│       └── godkiller-gate.template.yml# CI Quality Gate Action Template for Teams
├── assets/                            # Application icons & branding assets
│   ├── app.ico
│   ├── app.png
│   └── logo.png
├── src/                               # แกนสมองกลภาษา Rust (Zero-Spaghetti Architecture)
│   ├── domain/
│   │   ├── anti_spaghetti.rs          # Complexity <= 7 & architecture invariant builder
│   │   ├── ast_discovery.rs           # Token-based workspace file & symbol locator
│   │   ├── formal_contract.rs         # Zero-Copy Cow<'a, str> HoareContract {P} Target {Q}
│   │   ├── gatekeeper.rs              # Disk Scanner: Multi-Language CC, Span & Rule 12 Stub auditor
│   │   ├── linguistic_transpiler.rs   # Thai->English IR, Negation Guard & Circuit Breaker
│   │   ├── contract_evaluator.rs      # 4-Branch Invariant Evaluation & Semantic Post-Condition validator
│   │   ├── repo_map.rs                # Repository topology tree generator
│   │   ├── stack_sensor.rs            # Universal Tech Stack Sensor (package.json, Cargo.toml)
│   │   ├── symbol_graph.rs            # Semantic Symbol graph & Blast Radius impact auditor
│   │   ├── terminal_pruner.rs         # Smart Terminal Pruner for Noise Filtering
│   │   ├── tri_pillar.rs              # Fast-lane, Spatial Anchors & Inquiry classifier
│   │   ├── zero_bridge.rs             # Post-Prompt Isomorphic Bridge
│   │   └── mod.rs
│   ├── proxy/
│   │   ├── handlers.rs                # /v1 OpenAI endpoints, Intent Crucible & Hook routes
│   │   ├── mcp.rs                     # Native Model Context Protocol (JSON-RPC stdio: gk_claim_done)
│   │   ├── sanitizer.rs               # Prompt injection sanitizer & Origin Host Guard
│   │   ├── server.rs                  # Loopback server with embedded UI & desktop card launcher
│   │   └── mod.rs
│   ├── antigravity/
│   │   ├── hook.rs                    # 1-Click Universal Hook (Antigravity, Cursor, Claude, Copilot)
│   │   ├── startup.rs                 # Windows Run-on-Startup registry helper
│   │   └── mod.rs
│   ├── upstream/
│   │   ├── bootstrap.rs               # Autonomous Ollama Bootstrap Manager (qwen2.5-coder:1.5b)
│   │   ├── provider.rs                # LocalZeroEngine & Upstream LLM Gateway
│   │   └── mod.rs
│   ├── lib.rs
│   └── main.rs                        # CLI parser (--hook, --unhook, --gate, --install_hook, --purify)
├── gui/                               # C# Windows Forms Compact Card Suite (.NET 9)
│   ├── GodkillerZero.csproj           # Deterministic Build & PathMap configured
│   ├── MainForm.cs                    # Compact card UI, System Tray & Telemetry
│   ├── HookEngine.cs                  # Universal multi-IDE Hook manager
│   ├── ExtraSettingsForm.cs           # Checkbox matrix for all 16 Invariants
│   ├── Theme.cs                       # Dark Acrylic Theme palette
│   └── Program.cs
├── src-ui/                            # Japanese Zen Minimalist UI (ฝังในตัวไบนารีผ่าน include_str!)
│   ├── index.html                     # Neural Engine Card & Intent Crucible
│   ├── style.css                      # โทนสี Sumi Ink, Washi, Koke Moss, Urushi
│   ├── app.js                         # 1-Click Hook, Crucible Dispatcher & Engine Sync
│   └── favicon.svg
├── tests/
│   ├── antigravity_isolation_test.rs  # Loopback security, fast-lane & envelope tests (8 tests)
│   └── contract_evaluation_test.rs    # Invariant contract, Multi-language coords & Circuit breaker (14 tests)
├── Cargo.toml                         # Tokio, Axum, Serde, Clap, Regex
├── CONTRIBUTING.md                    # Community contribution guidelines
├── LICENSE                            # MIT License
├── README.md                          # Production-grade cognitive pre-flight documentation
├── SECURITY.md                        # Security policy and vulnerability reporting
├── SPECIFICATION.md                   # เอกสารพิมพ์เขียวฉบับสมบูรณ์ (ไฟล์นี้)
├── build.ps1                          # PowerShell build and packaging pipeline
├── install-and-run.bat                # 1-Click launcher script
└── install.ps1                        # 1-Liner PowerShell installer
```

---

## 8. Zero-Friction Ingress Architecture
1. **1-Click Native Hook:** เขียนกฎ 16 สัจพจน์ลง `~/.gemini/GEMINI.md`, `.cursorrules`, `CLAUDE.md`, และ `.github/copilot-instructions.md` ทันทีโดยไม่ต้องแก้ Settings
2. **Intent Crucible:** ฟอกภาษาธรรมชาติภาษาไทย บีบอัดเป็น Technical English Invariant Contract และคัดลอกลง Clipboard อัตโนมัติ
3. **Local Gatekeeper CLI (`--gate`):** สแกนโปรเจกต์บนดิสก์จริง บล็อกสปาเกตตี้โค้ดก่อน Commit

---

## 9. Pessimistic Red-Team & Failure Recovery
* **Exploratory Inquiry Exemption:** คำถามเชิงทฤษฎี ("คืออะไร", "explain") ปล่อยผ่านเป็น `ConceptualArchitecture` ตอบกระชับ ไร้น้ำ
* **Fast Lane Affirmations:** คำยืนยันสั้น ("ok", "yes", "ทำต่อเลย") ผ่านฉลุย ไม่ติดขัด
* **Offline Local Engine Resilience:** หากไม่มีเน็ต ระบบสลับใช้ Local Zero Engine / Ollama ทันที ไม่ติด 429 Rate Limit
* **Double-Click Process Protection:** หากเปิดโปรแกรมซ้ำ ตัวที่สองส่ง handshake และปิดตัวอย่างนุ่มนวล (Exit Code 0)
