<p align="center">
  <img src="assets/banner.png" alt="GODKILLER ZERO — The Guardrail Layer for AI Coding" width="100%" />
</p>

<h1 align="center">GODKILLER ZERO</h1>

<p align="center">
  <strong>The Guardrail Layer for AI Coding</strong><br/>
  <em>Let AI <b>BUILD</b>. Make GODKILLER <b>VERIFY</b>.</em>
</p>

<p align="center">
  <code>DONE IS NOT EVIDENCE.</code>
  &nbsp;·&nbsp;
  <code>Less tokens. More control.</code>
</p>

<p align="center">
  <a href="https://github.com/taurus42119-stack/GODKILLER-ZERO/releases/latest"><img src="https://img.shields.io/github/v/release/taurus42119-stack/GODKILLER-ZERO?style=for-the-badge&label=DOWNLOAD&color=00d4ff" alt="Download" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/LICENSE-MIT-white?style=for-the-badge" alt="MIT" /></a>
  <img src="https://img.shields.io/badge/PLATFORM-Windows%20x64-0a0a0a?style=for-the-badge" alt="Windows" />
  <img src="https://img.shields.io/badge/MCP-gk__*-0a0a0a?style=for-the-badge" alt="MCP" />
</p>

<p align="center">
  <b>Works with</b><br/>
  <img src="https://img.shields.io/badge/Cursor-00d4ff?style=flat-square&logoColor=white" alt="Cursor" />
  <img src="https://img.shields.io/badge/Claude-00d4ff?style=flat-square" alt="Claude" />
  <img src="https://img.shields.io/badge/Gemini-00d4ff?style=flat-square" alt="Gemini" />
  <img src="https://img.shields.io/badge/Antigravity-00d4ff?style=flat-square" alt="Antigravity" />
</p>

<p align="center">
  <a href="https://www.instagram.com/kayvins.th">@kayvins.th</a>
</p>

---

## Why it exists

AI can code.  
It should not be allowed to break your project — then claim **done**.

<table>
  <tr>
    <th width="50%">Without ZERO</th>
    <th width="50%">With ZERO</th>
  </tr>
  <tr>
    <td align="center">vibe edits</td>
    <td align="center"><b>targeted scope</b></td>
  </tr>
  <tr>
    <td align="center">“finished” on hope</td>
    <td align="center"><b>disk verification</b></td>
  </tr>
  <tr>
    <td align="center">silent spaghetti</td>
    <td align="center"><b>complexity / stub gates</b></td>
  </tr>
  <tr>
    <td align="center">dump the whole repo</td>
    <td align="center"><b>deep project radar</b></td>
  </tr>
  <tr>
    <td align="center">long, unfocused agent loops</td>
    <td align="center"><b>tighter aim · fewer wasted turns</b></td>
  </tr>
</table>

---

## Field signal (Cursor Usage)

Same model. New chat both sides. Same task prompt.

| | Tokens |
| :---: | :---: |
| Before SHIELD | **585K** |
| After SHIELD | **241.1K** |
| Delta | **~59% lower** on that run |

Model: `cursor-grok-4.6-high` · Source: Cursor Usage log

> Not a lab benchmark. One controlled session. Your mileage will vary — but the direction matched what Zero Fluff + Target + Scope are built for: less wandering, more hit.

---

## How it works

```mermaid
flowchart LR
  A[You] --> B[AI Agent]
  B --> C[GODKILLER ZERO]
  C -->|SHIELD| D[Soft Rules<br/>Extra Tuning]
  C -->|MCP / CLI| E[Disk Gatekeeper]
  E --> F[(Your Files)]
  E -->|PASS / FAIL| B
  D --> B
```

```mermaid
stateDiagram-v2
  [*] --> Coding: Agent starts
  Coding --> ClaimDone: Model says done
  ClaimDone --> Verify: gk_claim_done / --gate
  Verify --> Pass: Clean disk
  Verify --> Fail: Stub / CC / span / scope
  Fail --> Coding: Fix again
  Pass --> [*]
```

Three planes:

| Plane | What | Force |
| :--- | :--- | :---: |
| **Soft rules** | Injected into Cursor / Claude / Gemini hosts | Behavioral |
| **HARD gate** | `godkiller-console --gate` / `gk_gatekeeper_scan` | Disk |
| **MCP tools** | `gk_claim_done` · `gk_gatekeeper_scan` · `gk_get_repo_map` | Protocol |

---

## Control surface

| Mode | Name | Feel |
| :---: | :--- | :--- |
| **SHI** | Silent | Max autonomy |
| **KEN** | Balanced | Confirm breaking changes |
| **SHIN** | Turbo | Deep architect pressure |
| **EXTRA** | Tuning | Flip every invariant |

| Action | Effect |
| :--- | :--- |
| **SHIELD** | Arm rules + MCP across hosts |
| **RESTORE** | Strip ZERO only — peer MCPs stay |

---

## Feature pillars

<table>
<tr>
<td width="50%" valign="top">

### 1. AI Guardrails
Set boundaries **before** the model touches code.

`Target · Scope · Stack · Architecture`

| Extra toggle | Job |
| :--- | :--- |
| Zero Hallucination | Halt assumptions |
| Ghost Edit Shield | Never touch untouched code |
| Stack Sensor | Align to real manifests |
| Thai Precision | Honor ห้าม / อย่า |

</td>
<td width="50%" valign="top">

### 2. Verify Before Done
Real filesystem check — not a chat claim.

`Code · Scope · Complexity · Stub`

```text
VERIFY
 ✓ Code Quality
 ✓ Scope Check
 ✓ Complexity   (CC ≤ 7)
 ✓ No Stub
 ✓ Disk State
 ✓ Span ≤ 70
```

| Extra toggle | Job |
| :--- | :--- |
| Bite-Sized Functions | Cap span |
| Frictionless Logic | Kill if-else pyramids |
| Crash Immunity | Ban unwrap / empty catch |
| Clean Architecture | No junk drawers |

</td>
</tr>
<tr>
<td width="50%" valign="top">

### 3. Blast Radius Radar
See what breaks **before** you edit.

`Dependency · Caller · Impact`

| Extra toggle | Job |
| :--- | :--- |
| Blast Radius Radar | Audit callers first |
| Self-Documenting Code | Ban `data` / `res` / `req` |

</td>
<td width="50%" valign="top">

### 4. Loop Breaker
Stop repeat “still failing” loops.

`Log · Diagnose · Fix`

| Extra toggle | Job |
| :--- | :--- |
| Loop Breaker | Halt guess cycles |
| Instant Delivery | Zero fluff / chitchat |

</td>
</tr>
<tr>
<td width="50%" valign="top">

### 5. Deep Project Radar
Understand the repo without dumping it.

`Map · Target · Context · Plan`

| Extra toggle | Job |
| :--- | :--- |
| Deep Project Radar | `gk_get_repo_map` |
| Spatial Wireframing | ASCII plans |
| Flowchart Generator | Mermaid flows |

Cadence: **Fast** (plans only) · **Normal** (more diagrams)

</td>
<td width="50%" valign="top">

### Bonus architecture knobs

| Extra toggle | Job |
| :--- | :--- |
| Decoupled Core | Pure logic vs I/O |
| Modern Web Standards | Prefer current docs |
| Designer-Grade UI | Tokens / motion / dark |
| Proactive Roadmap | Next phases after green |

</td>
</tr>
</table>

```mermaid
flowchart TB
  subgraph Agent["AI Agent"]
    P[Plan]
    E[Edit]
    C[Claim done]
  end
  subgraph ZERO["GODKILLER ZERO"]
    R[Rules Soft + Extra]
    M[Repo Map]
    G[Gatekeeper HARD]
  end
  P --> R
  P --> M
  E --> R
  C --> G
  G -->|evidence| C
```

---

## Extra Tuning — full board

One SHIELD. Nineteen levers.

| # | Toggle | Pillar |
| ---: | :--- | :--- |
| 1 | Zero Hallucination | Guardrails |
| 2 | Self-Documenting Code | Blast / Quality |
| 3 | Clean Architecture | Verify |
| 4 | Instant Delivery | Loop / Tokens |
| 5 | Spatial Wireframing | Radar / Plan |
| 6 | Flowchart Generator | Radar / Plan |
| 7 | Bite-Sized Functions | Verify |
| 8 | Frictionless Logic | Verify |
| 9 | Blast Radius Radar | Blast |
| 10 | Stack Sensor | Guardrails |
| 11 | Ghost Edit Shield | Guardrails |
| 12 | Loop Breaker | Loop |
| 13 | Crash Immunity | Verify |
| 14 | Decoupled Core | Architecture |
| 15 | Modern Web Standards | Architecture |
| 16 | Designer-Grade UI | Architecture |
| 17 | Proactive Roadmap | Architecture |
| 18 | Thai Precision | Guardrails |
| 19 | Deep Project Radar | Radar |

Hard floor on disk (always available via CLI / MCP):

- Cyclomatic complexity **≤ 7**
- Function span **≤ 70**
- No lazy stubs / banned generics

---

## Get it

### Download

**[→ Latest Windows release](https://github.com/taurus42119-stack/GODKILLER-ZERO/releases/latest)**

1. Unzip `godkiller-zero-windows-x86_64.zip`  
2. Run `GodkillerZero.exe`  
3. Click **SHIELD**  
4. Open **EXTRA** — flip only what you need

### One-liner

```powershell
irm https://raw.githubusercontent.com/taurus42119-stack/GODKILLER-ZERO/main/install.ps1 | iex
```

---

## Build from source

Needs **Rust** + **.NET SDK 9+**

```powershell
.\build.ps1
```

```text
publish\
  GodkillerZero.exe
  godkiller-console.exe
```

```powershell
godkiller-console --gate .
godkiller-console --repo-map .
godkiller-console --mcp
```

| MCP tool | Role |
| :--- | :--- |
| `gk_gatekeeper_scan` | Hygiene / CC / span / stubs |
| `gk_get_repo_map` | Compact project skeleton |
| `gk_claim_done` | Outbound “done” with disk proof |

---

## What you get

| | |
| :--- | :--- |
| **Stronger security** | Block risky / out-of-scope edits |
| **Lower wasted usage** | Fewer unfocused loops · smarter context |
| **Better code quality** | CC / span / stub discipline on disk |
| **Full control** | Your rules. Your Extra board. |
| **Open source** | MIT |

---

## License

MIT — free to use, fork, and ship. See [LICENSE](LICENSE).

---

<p align="center">
  <strong>Less chaos. More control.</strong><br/>
  <sub>Built to kill vibe-coding — not creativity.</sub><br/>
  <sub>@kayvins.th</sub>
</p>
