# ⚡ GODKILLER ZERO (零)
> **Universal Cognitive Pre-flight Firewall & Invariant Gatekeeper for AI Coding Assistants**  
> *Target Runtimes: Google Antigravity, Cursor IDE, Anthropic Claude Code, VS Code Copilot | Native Windows Binary (`.exe`) | 100% Free & Open Source*

[![Zero-Entropy CI](https://github.com/taurus42119-stack/godkiller-zero/actions/workflows/ci.yml/badge.svg)](https://github.com/taurus42119-stack/godkiller-zero/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)
[![Port: 4242](https://img.shields.io/badge/Port-4242-blueviolet.svg)](#)
[![Tests: 52 Passed](https://img.shields.io/badge/Tests-52%2F52%20Passed-brightgreen.svg)](#)

---

## 零 What is GODKILLER ZERO?

**Stop prompting with vibes.** Casual prompting produces spaghetti architecture, bloated functions, lazy stubs (`// TODO`), generic identifiers (`data`, `res`, `req`), and hallucinated files.

**GODKILLER ZERO** is a standalone, enterprise-grade cognitive micro-utility. It shields **Google Antigravity, Cursor, Claude Code, and GitHub Copilot** with mathematically sound invariant rules across your entire machine and development workflows.

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
│ TIER 3: CI/CD Quality Gate (`godkiller-zero --gate .`)                  │
│ • Drops into GitHub Actions / GitLab CI pipelines                       │
│ • Rejects Pull Requests that breach complexity or architecture rules    │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## ⚡ 1-Second Enterprise Quickstart

### Option A: One-Liner PowerShell (Windows)
Open PowerShell and run:
```powershell
irm https://raw.githubusercontent.com/taurus42119-stack/godkiller-zero/main/install.ps1 | iex
```

### Option B: Portable Desktop App (No Installation Required)
1. Download or launch `GodkillerZeroGui.exe` from `publish/` or project directory.
2. Double-click `GodkillerZeroGui.exe`.
3. The **GODKILLER ZORO 1.0** dashboard appears, creates a Desktop Shortcut, and docks to the Windows System Tray (beside the language switcher).
4. Click **`SHIELD`** to activate invariant guardrails across all AI clients (Cursor, Claude, Antigravity, Copilot)!

---

## 🛡️ The 16 Invariant Rules

### Core Architecture Discipline (Rules 1 - 7)
| # | Rule | Description | Enforced By |
| :---: | :--- | :--- | :---: |
| **1** | **Target Coordinates** | Auto-detects target files from prompt context & AST discovery. | Hybrid Engine |
| **2** | **Zero Fluff** | Pure code and diffs. Zero conversational filler, pleasantries, or apologies. | First-Token Protocol |
| **3** | **Anti-Spaghetti (CC $\le$ 7)** | Cyclomatic complexity bounded to $\le$ 7 and spans $\le$ 70 lines. | Gatekeeper Disk Scanner |
| **4** | **Zero Vibe Identifiers** | Strictly forbids generic names (`data`, `res`, `req`, `item`, `temp`, `val`, `payload`). | Gatekeeper & Hook |
| **5** | **Clarification Cadence** | Controlled clarification cadence: SHI (Silent), KEN (Balanced), SHIN (Deep Architect). | Tri-Pillar Engine |
| **6** | **Disk State Verification** | Verify real disk state and compiler build before declaring completion. | MCP & Gatekeeper |
| **7** | **Mandatory ASCII Blueprints**| Output explicit ASCII component wireframes inside code fences (```text) before code. | Global Hook |

### Advanced Cognitive Invariants (Rules 8 - 16)
| # | Rule | Description | Enforced By |
| :---: | :--- | :--- | :---: |
| **8** | **Blast Radius Impact Audit** | Verifies all inbound callers & downstream dependencies before modifying symbols. | Symbol Graph Engine |
| **9** | **Tech Stack Auto-Alignment** | Scans workspace manifests (`package.json`, `Cargo.toml`) to prevent library hallucination. | Universal Stack Sensor |
| **10** | **Ghost Edit Shield** | Prohibits mutating files not targeted. Respects negative directives ("don't", "ห้าม"). | Linguistic Polarity Guard |
| **11** | **Linguistic Circuit Breaker**| Halts AI guessing loops when user indicates failure recurrence ("still failing", "ยังไม่ได้"). | Circuit Breaker Lockdown |
| **12** | **Exhaustive Error Handling** | Forbids unchecked `unwrap()`, raw `expect()`, `// TODO`, or silent `catch {}`. | Code Hygiene Gate |
| **13** | **Functional Core & Shell** | Decouples deterministic domain logic from impure I/O operations. | Architecture Invariant |
| **14** | **Modern Web & Doc Guidance** | Mandates modern official documentation; deprecates obsolete blog patterns. | Research Prompt Invariant |
| **15** | **Premium UI/UX Mandate** | Bans raw browser defaults; enforces cohesive design tokens, micro-animations & dark mode. | Design System Invariant |
| **16** | **Infinite Evolution** | Proactively identifies next-stage enterprise milestones upon test completion. | Evolutionary Continuum |

---

## 🏢 CI/CD & Team Quality Gate Integration
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
          ./godkiller-zero.exe --gate .
```

---

## 💻 CLI Commands (For Automation & Terminals)

```powershell
# Run Disk Gatekeeper check on any project directory
godkiller-zero --gate "D:\Projects\MyProject"

# Install Git pre-commit hook into target project
godkiller-zero --install-hook "D:\Projects\MyProject"

# Manually hook/unhook IDEs globally
godkiller-zero --hook
godkiller-zero --unhook

# Launch without opening the GUI card window
godkiller-zero --no-open
```

---

## 🔒 Enterprise Security & Privacy Guarantee

* **100% Offline-First & Local:** Operates on local loopback `127.0.0.1:4242`. Features Local Zero Engine with autonomous Ollama support (`qwen2.5-coder:1.5b`).
* **Zero Telemetry / Zero Cloud Storage:** No external tracking, no Google Analytics, no third-party telemetry.
* **Path-Remapped Binaries:** Executable symbols are sanitized with `--remap-path-prefix` and C# `<PathMap>` to protect local development paths.
* **RAM Footprint:** Less than 15 MB RAM, 0% idle CPU usage.

---

## 📜 License & Author
MIT License — 100% Free & Open Source for developers and enterprises worldwide.  
Crafted with discipline by [taurus42119](https://github.com/taurus42119-stack).  
Follow the Developer on Instagram: [**@kayvins.th**](https://www.instagram.com/kayvins.th) 📸
