# GODKILLER ZERO
> **Universal Cognitive Pre-flight Firewall & Invariant Gatekeeper for AI Coding Assistants**  
> *Target Runtimes: Google Antigravity, Cursor IDE, Anthropic Claude Code, VS Code Copilot | Native Windows Binary (`.exe`) | Free & Open Source*

[![Zero-Entropy CI](https://github.com/taurus42119-stack/godkiller-zero/actions/workflows/ci.yml/badge.svg)](https://github.com/taurus42119-stack/godkiller-zero/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)

---

## What is GODKILLER ZERO?

Unconstrained agent prompting often leads to architectural drift: bloated functions, lazy stubs (`// TODO`), generic identifiers (`data`, `res`, `req`), and hallucinated dependencies.

**GODKILLER ZERO** is a standalone, production-grade cognitive pre-flight firewall. It equips **Google Antigravity, Cursor, Claude Code, and GitHub Copilot** with structural invariant rules across local machines, pre-commit workflows, and CI/CD pipelines.

```text
┌─────────────────────────────────────────────────────────────────────────┐
│ TIER 1: Machine-Wide Cognitive Shield (Antigravity / Cursor / Claude)    │
│ • 1-Click Hook: Injects 16 Invariants into GEMINI.md, .cursorrules, etc.│
│ • Enforces Zero-Speculation, Banned Generics, Max 70 Lines, ASCII Plans │
└────────────────────────────────────┬────────────────────────────────────┘
                                     │
┌────────────────────────────────────▼────────────────────────────────────┐
│ TIER 2: Local Project Gatekeeper (cargo gate / Git Pre-commit Hook)     │
│ • 1-Click Workspace Targeting with Native Gatekeeper Scanner            │
│ • Blocks non-compliant code (CC > 7, Span > 70) before disk commit      │
└────────────────────────────────────┬────────────────────────────────────┘
                                     │
┌────────────────────────────────────▼────────────────────────────────────┐
│ TIER 3: CI/CD Quality Gate (`godkiller-console --gate .`)               │
│ • Drops into GitHub Actions / GitLab CI pipelines                       │
│ • Rejects Pull Requests that breach complexity or architecture rules    │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Quickstart

### Option A: One-Liner PowerShell (Windows)

> [!NOTE]
> This script downloads the latest release archive, verifies its SHA256 checksum, extracts binaries to `%LOCALAPPDATA%\Programs\godkiller-zero`, adds the directory to your User `PATH`, and launches the desktop card.

Open PowerShell and run:
```powershell
irm https://raw.githubusercontent.com/taurus42119-stack/godkiller-zero/main/install.ps1 | iex
```

### Option B: Portable Desktop App (No Installation Required)
1. Download or launch `GodkillerZero.exe` from `publish/` or GitHub Releases.
2. Double-click `GodkillerZero.exe`.
3. The **GODKILLER ZERO** dashboard appears, creates a Desktop Shortcut, and docks to the Windows System Tray.
4. Click **`SHIELD`** to activate invariant guardrails across all AI clients (Cursor, Claude, Antigravity, Copilot).

---

## The 16 Invariant Rules

### Enforcement Tiers

- **HARD:** Deterministically enforced by static analysis, AST scanning, or proxy circuit breakers. Violations reject commits, fail CI checks, or halt requests.
- **SOFT:** Behavioral and stylistic invariant injected into AI rules files (`GEMINI.md`, `.cursorrules`, etc.). Adhered to via model system instruction conditioning.
- **HYBRID:** Combines static AST/symbol discovery on disk with injected system contract constraints.

### Architecture & Hygiene Invariants (Rules 1 - 7)

| # | Rule | Description | Enforcement | Mechanism |
| :---: | :--- | :--- | :---: | :--- |
| **1** | **Target Coordinates** | Detects target file coordinates from prompt context and AST discovery. | **HYBRID** | AST Discovery Engine + Prompt Hook |
| **2** | **Zero Fluff** | Pure code and diffs. Strips conversational filler, pleasantries, and apologies. | **SOFT** | First-Token Protocol Invariant |
| **3** | **Anti-Spaghetti (CC <= 7)** | Cyclomatic complexity bounded to <= 7 and function spans bounded to <= 70 lines. | **HARD** | Gatekeeper Static Disk Scanner |
| **4** | **Zero Vibe Identifiers** | Forbids generic names (`data`, `res`, `req`, `item`, `temp`, `val`, `payload`). | **HARD** | Gatekeeper Identifier Scanner |
| **5** | **Clarification Cadence** | Regulated interactive clarification cadence: SHI (Silent), KEN (Balanced), SHIN (Deep Architect). | **SOFT** | Tri-Pillar Discipline Prompt |
| **6** | **Disk State Verification** | Verifies on-disk files and compiler builds before marking tasks complete. | **HYBRID** | MCP Protocol Mandate + Gatekeeper |
| **7** | **Mandatory ASCII Blueprints** | Demands explicit ASCII component wireframes inside code fences before UI modifications. | **SOFT** | System Instruction Hook |

### Cognitive & Safety Invariants (Rules 8 - 16)

| # | Rule | Description | Enforcement | Mechanism |
| :---: | :--- | :--- | :---: | :--- |
| **8** | **Blast Radius Impact Audit** | Traces inbound callers and downstream dependents before symbol mutations. | **HARD** | Symbol Graph Engine |
| **9** | **Tech Stack Auto-Alignment** | Scans workspace manifests (`package.json`, `Cargo.toml`) to prevent dependency hallucinations. | **HARD** | Universal Stack Sensor |
| **10** | **Ghost Edit Shield** | Prohibits editing untouched files; bounds negative instructions ("don't", "ห้าม"). | **HARD** | Linguistic Polarity Guard |
| **11** | **Linguistic Circuit Breaker** | Halts speculative guessing loops when repeated failure cues ("still failing", "ยังไม่ได้") are detected. | **HARD** | Failure Loop Interrupt |
| **12** | **Exhaustive Error Handling** | Forbids unchecked `unwrap()`, raw `expect()`, `// TODO`, or silent `catch {}`. | **SOFT** | Prompt Hygiene Invariant |
| **13** | **Functional Core & Shell** | Decouples deterministic domain logic from external I/O side effects. | **SOFT** | Architecture Prompt Invariant |
| **14** | **Modern Web & Doc Sources** | Mandates modern official documentation references over deprecated patterns. | **SOFT** | Research Guidance Invariant |
| **15** | **Design System Mandate** | Rejects browser default styling; requires design tokens, dark mode, and micro-interactions. | **SOFT** | Design System Invariant |
| **16** | **Evolutionary Continuum** | Proactively outlines production readiness milestones upon test completion. | **SOFT** | Advancement Invariant |

---

## CI/CD Quality Gate Integration

Drop `.github/workflows/godkiller-gate.template.yml` into your team repository:
```yaml
name: Invariant Architecture Gate
on: [push, pull_request]

jobs:
  anti-spaghetti-audit:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run GODKILLER ZERO Gatekeeper
        run: |
          ./godkiller-console.exe --gate .
```

---

## CLI Commands (For Automation & Terminals)

```powershell
# Run Disk Gatekeeper check on any project directory
godkiller-console --gate "D:\Projects\MyProject"

# Install Git pre-commit hook into target project
godkiller-console --install-hook "D:\Projects\MyProject"

# Manually hook/unhook IDEs globally
godkiller-console --hook
godkiller-console --unhook

# Launch without opening the GUI card window
godkiller-console --no-open
```

---

## Security & Privacy Guarantee

* **100% Offline-First & Local:** Operates on local loopback `127.0.0.1:4242`. Features Local Zero Engine with autonomous Ollama support (`qwen2.5-coder:1.5b`).
* **Zero Telemetry / Zero Cloud Storage:** No external tracking, no analytics, no third-party telemetry.
* **Path-Remapped Binaries:** Executable symbols are sanitized with `--remap-path-prefix` and C# `<PathMap>` to protect local development paths.
* **RAM Footprint:** Less than 15 MB RAM, 0% idle CPU usage.

---

## License & Author

MIT License - Free and Open Source. See [LICENSE](LICENSE) for details.  
Maintained by [taurus42119](https://github.com/taurus42119-stack).  
Instagram: [@kayvins.th](https://www.instagram.com/kayvins.th)
