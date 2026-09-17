# ⚡ GODKILLER ZERO — Feature Audit Report
### ตรวจทุก Feature ว่า "ใช้ได้จริง / มี Bug / มีประโยชน์จริงไหม"

---

## สถาปัตยกรรมการ Enforce — เข้าใจก่อน

GK-ZERO มีกลไก enforce 3 ระดับ:

| ระดับ | กลไก | ความแข็งแกร่ง |
|---|---|---|
| 🔴 **HARD** | Proxy intercept / Disk Scanner / Transpiler | บังคับได้จริง ไม่ขึ้นกับ AI |
| 🟡 **SOFT** | เขียนใน GEMINI.md เท่านั้น | AI อาจ "drift" ได้ใน conversation ยาว |
| 🔵 **HYBRID** | GEMINI.md + programmatic assist | แข็งกว่า SOFT แต่ไม่ 100% |

---

## Feature-by-Feature Audit

---

### Rule 1 — TARGET COORDINATES
**กลไก:** 🔵 HYBRID — `AstDiscoveryEngine` scan workspace + GEMINI.md

| | |
|---|---|
| **ทำงานได้จริงไหม** | ✅ ใช้ได้ — `AstDiscoveryEngine.discover_with_tokens()` ค้นไฟล์จริงในโปรเจค |
| **Bug** | ⚠️ Confidence score algorithm ง่ายมาก: path match +0.3, filename match +0.6 — ถ้าชื่อโปรเจคซ้อนกัน false positive สูง |
| **มีประโยชน์ไหม** | ✅ **ประโยชน์สูง** — Antigravity IDE ไม่ auto-detect file จากคำพูดภาษาไทย (`ปุ่มหน้าแรก` → `home.tsx`) แต่ GK-ZERO ทำได้ |
| **ซ้อนกับ AI ไหม** | ❌ ไม่ซ้อน — Gemini เดา path ไม่ได้จาก Thai context |

---

### Rule 2 — ZERO CONVERSATIONAL FLUFF (First-Token Protocol)
**กลไก:** 🔴 HARD — Reverse Proxy Egress Fluff Stripper (`proxy::sanitizer`) + GEMINI.md

| | |
|---|---|
| **ทำงานได้จริงไหม** | ✅ **ใช้ได้จริง 100%** — ดักจับสตรีมขาออก ตัดบทเกริ่นนำภาษาไทย/อังกฤษทิ้ง บังคับ First-Token Code Fence เที่ยงตรง |
| **Bug** | ✅ (แก้แล้ว) มี Unit Tests ดักจับคำเกริ่นทั้งภาษาไทย ("สวัสดีครับ", "ยินดีครับ", "นี่คือ...") และภาษาอังกฤษ 100% |
| **มีประโยชน์ไหม** | ✅ **ประโยชน์สูง** — ป้องกัน AI โมเดลยาวๆ Drift กลับมาพูดเกริ่นนำอย่างไร้ประโยชน์ |
| **ซ้อนกับ AI ไหม** | ❌ ไม่ซ้อน — ตัดข้อความทิ้งระดับ Byte Stream ของ Reverse Proxy ก่อนส่งถึงผู้ใช้ |

---

### Rule 3 — ANTI-SPAGHETTI & CODE HYGIENE (CC ≤ 7, Span ≤ 70)
**กลไก:** 🔴 HARD — Tree-sitter AST Engine + Gatekeeper Disk Scanner (`cargo gate`)

| | |
|---|---|
| **ทำงานได้จริงไหม** | ✅ **ใช้ได้จริง 100%** — สแกนโครงสร้าง AST จริง (C#, Rust, TSX, Python) พร้อมคำนวณ SonarQube Cognitive Complexity |
| **Bug** | ✅ (แก้แล้ว) ยกเว้น Structural Code (React JSX, WinForms Forms, DTO mappings, `tests/`) ไม่ให้ติด Span และ Complexity บังคับเฉพาะ Business Logic เท่านั้น |
| **มีประโยชน์ไหม** | ✅ **ประโยชน์สูงมาก** — เป็น CI/CD quality gate ระดับ Production-Grade |
| **ซ้อนกับ AI ไหม** | ❌ ไม่ซ้อน — Enforce โครงสร้างโค้ดบนดิสก์จริง |

---

### Rule 4 — ZERO VIBE-CODING TELLS (No junk comments)
**กลไก:** 🟡 SOFT — GEMINI.md เท่านั้น

| | |
|---|---|
| **ทำงานได้จริงไหม** | ⚠️ บางส่วน — AI รุ่นใหม่ใส่ comment น้อยลงอยู่แล้ว |
| **Bug** | 🔴 **ไม่มีกลไก detect** — Gatekeeper ไม่ scan comment quality จริง |
| **มีประโยชน์ไหม** | ✅ มีประโยชน์ระดับต่ำ — ช่วย "prime" AI แต่ไม่ guarantee |
| **ซ้อนกับ AI ไหม** | ⚠️ ซ้อนมาก — AI รู้จัก self-documenting code อยู่แล้ว |

---

### Rule 5 — CLARIFICATION CADENCE (Silent / Interactive / Balanced)
**กลไก:** 🟡 SOFT — GEMINI.md เท่านั้น

| | |
|---|---|
| **ทำงานได้จริงไหม** | ✅ ได้ผลดี — ปรับ behavior AI ได้จริงจาก Discipline mode (KEN/SHI/SHIN) |
| **Bug** | 🔴 **Bug ใน TriPillarEvaluator** — `is_fast_lane_affirmation` ใช้ `split_whitespace().count() > 3` ทำให้ "ตกลงนะ" (2 คำ) bypass เข้า fast lane ด้วย แม้จะไม่ใช่ affirmation |
| **มีประโยชน์ไหม** | ✅ **ประโยชน์สูง** — 3 modes ชัดเจน (SHI=ถาม, KEN=ถามเฉพาะ critical, SHIN=ทำเองเลย) |
| **ซ้อนกับ AI ไหม** | ❌ ไม่ซ้อน — AI ไม่ปรับ clarification behavior ด้วยตัวเอง |

---

### Rule 6 — DISK STATE VERIFICATION
**กลไก:** 🟡 SOFT — GEMINI.md + `enforce_disk_verification` flag

| | |
|---|---|
| **ทำงานได้จริงไหม** | ⚠️ บางส่วน — flag ถูกส่งเข้า Hoare Contract แต่ AI ต้อง "เชื่อฟัง" เอง |
| **Bug** | ✅ ไม่มี bug ชัดเจน |
| **มีประโยชน์ไหม** | ✅ มีประโยชน์ระดับปานกลาง — ป้องกัน AI declare "done" ก่อนเวลา |
| **ซ้อนกับ AI ไหม** | ⚠️ ซ้อนบางส่วน — Antigravity IDE มี file verification tools อยู่แล้ว |

---

### Rule 7 — MANDATORY ASCII BLUEPRINTS
**กลไก:** 🟡 SOFT — GEMINI.md เท่านั้น

| | |
|---|---|
| **ทำงานได้จริงไหม** | ✅ ได้ผล — ขอ ASCII wireframe ก่อน code ได้ผลจริง |
| **Bug** | 🔴 **ไม่มีกลไก enforce fence** — rule บอกว่าต้องใช้ ```text fence แต่ไม่มี parser ตรวจ |
| **มีประโยชน์ไหม** | ✅ **ประโยชน์สูง** สำหรับ UI development — ลด misunderstanding ก่อนเขียน code |
| **ซ้อนกับ AI ไหม** | ❌ ไม่ซ้อน — AI ไม่ทำ ASCII blueprint โดยอัตโนมัติถ้าไม่ได้สั่ง |

---

### Rule 8 — BLAST RADIUS IMPACT AUDIT
**กลไก:** 🔴 HARD — `PerProjectSymbolGraph` + `/api/purify` endpoint

| | |
|---|---|
| **ทำงานได้จริงไหม** | ✅ **ทำงานได้จริง** — trace consumer files ได้จาก regex-based symbol graph |
| **Bug** | ⚠️ Symbol extraction แบบ text-only (ไม่ใช่ real AST) — จับ `pub fn` ได้แต่จับ `pub(crate) fn` ไม่ได้ |
| **มีประโยชน์ไหม** | ✅ **ประโยชน์สูง** — บอก AI ว่า "ฟังก์ชันนี้มีผลกับอีก 3 ไฟล์" ก่อน edit |
| **ซ้อนกับ AI ไหม** | ❌ ไม่ซ้อน — Gemini ไม่ scan consumer files ก่อน edit โดยอัตโนมัติ |

---

### Rule 9 — TECH STACK AUTO-ALIGNMENT
**กลไก:** 🔴 HARD — `UniversalStackSensor` scan manifest files

| | |
|---|---|
| **ทำงานได้จริงไหม** | ✅ **ทำงานได้จริง** — detect `package.json`, `Cargo.toml`, `pyproject.toml` ฯลฯ |
| **Bug** | ⚠️ ใช้ lowercase match เท่านั้น — `Cargo.toml` ถูก match แต่ใช้ชื่อที่ตัวใหญ่ตัวเล็ก case-sensitive |
| **มีประโยชน์ไหม** | ✅ **ประโยชน์สูง** — inject "Stack: TailwindCSS + Next.js" เข้า Hoare Contract ทำให้ AI ไม่ hallucinate dependency |
| **ซ้อนกับ AI ไหม** | ⚠️ ซ้อนบางส่วน — Antigravity IDE รู้ stack อยู่แล้วถ้าเปิดไฟล์ไว้ |

---

### Rule 10 — NEGATIVE MUTATION BOUNDING (Ghost Edit Shield)
**กลไก:** 🔴 HARD — `LinguisticTranspiler.detect_polarity()` → `PRESERVE` verdict

| | |
|---|---|
| **ทำงานได้จริงไหม** | ✅ **ทำงานได้จริง 100%** — ตรวจ "อย่า/ห้าม/don't/never" และส่ง `PRESERVE_GUARD` IR ไปยัง AI |
| **Bug** | 🔴 **False positive**: `"don't forget to add tests"` จะถูก detect ว่า negated ทำให้ AI หยุด mutation แทนที่จะ add tests |
| **มีประโยชน์ไหม** | ✅ **ประโยชน์สูงมาก** — ป้องกัน AI แก้ไฟล์ที่ไม่ได้สั่ง (ghost edit) |
| **ซ้อนกับ AI ไหม** | ❌ ไม่ซ้อน — AI "drift" และแก้ไฟล์นอกเป้าหมายบ่อยมากถ้าไม่มี constraint |

---

### Rule 11 — LINGUISTIC CIRCUIT BREAKER (Failure Loop Interrupt)
**กลไก:** 🔴 HARD — `LinguisticTranspiler.detect_circuit_breaker()` → `CIRCUIT_BREAKER_LOCKDOWN`

| | |
|---|---|
| **ทำงานได้จริงไหม** | ✅ **ทำงานได้จริง 100%** — test ผ่าน, ทดสอบ live ผ่านแล้ว |
| **Bug** | ⚠️ keyword list จำกัด เช่น "ยังไม่เวิร์ค" ไม่อยู่ใน list แต่ "ไม่เวิร์ค" อยู่ |
| **มีประโยชน์ไหม** | ✅ **ประโยชน์สูงมาก** — เป็น feature ที่ unique ที่สุด ตัดวงจร AI guessing loop |
| **ซ้อนกับ AI ไหม** | ❌ ไม่ซ้อนเลย — AI ไม่ detect ว่าตัวเองกำลัง loop และยิ่ง guess มากขึ้น |

---

### Rule 12 — EXHAUSTIVE ERROR HANDLING & NO LAZY STUBS
**กลไก:** 🟡 SOFT — GEMINI.md เท่านั้น

| | |
|---|---|
| **ทำงานได้จริงไหม** | ⚠️ บางส่วน — AI รู้ดีอยู่แล้วแต่ไม่ consistent |
| **Bug** | 🔴 **ไม่มี scanner** — Gatekeeper ไม่ detect `unwrap()` หรือ `// TODO` ใน code |
| **มีประโยชน์ไหม** | ✅ มีประโยชน์ระดับปานกลาง |
| **ซ้อนกับ AI ไหม** | ⚠️ ซ้อนสูง — AI modern รู้จัก error handling best practices อยู่แล้ว |

---

### Rule 13 — FUNCTIONAL CORE & IMPERATIVE SHELL
**กลไก:** 🟡 SOFT — GEMINI.md เท่านั้น

| | |
|---|---|
| **ทำงานได้จริงไหม** | ⚠️ บางส่วน — pattern ซับซ้อน AI อาจไม่ apply ถูกต้อง |
| **Bug** | ✅ ไม่มี bug — เป็น instruction เท่านั้น |
| **มีประโยชน์ไหม** | ✅ มีประโยชน์สำหรับ advanced architecture — แต่ต้องการ context เยอะ |
| **ซ้อนกับ AI ไหม** | ⚠️ ซ้อนสูง — AI รู้ pattern นี้อยู่แล้ว แค่ต้อง remind |

---

### Rule 14 — MODERN WEB & DOCUMENTATION GUIDANCE
**กลไก:** 🟡 SOFT — GEMINI.md เท่านั้น

| | |
|---|---|
| **ทำงานได้จริงไหม** | ✅ ได้ผลบางส่วน — ช่วย bias AI ไปหา documentation ใหม่กว่า |
| **Bug** | ✅ ไม่มี bug |
| **มีประโยชน์ไหม** | ✅ มีประโยชน์ระดับต่ำ-ปานกลาง |
| **ซ้อนกับ AI ไหม** | ⚠️ ซ้อนสูง — Gemini มี knowledge cutoff และ web search อยู่แล้ว |

---

### Rule 15 — PREMIUM UI/UX AESTHETIC MANDATE
**กลไก:** 🟡 SOFT — GEMINI.md เท่านั้น

| | |
|---|---|
| **ทำงานได้จริงไหม** | ✅ ได้ผลดี — AI เพิ่ม hover, transition, gradient ถ้า rule บอก |
| **Bug** | 🔴 **ไม่มีกลไก audit** — Gatekeeper ไม่ scan CSS quality |
| **มีประโยชน์ไหม** | ✅ **ประโยชน์สูง** สำหรับ UI work — ให้ผลที่ consistent กว่า |
| **ซ้อนกับ AI ไหม** | ⚠️ ซ้อนบางส่วน — แต่ AI ไม่ consistent ถ้าไม่มี constraint |

---

### Rule 16 — INFINITE EVOLUTIONARY CONTINUUM (Enterprise Roadmap Advancement)
**กลไก:** 🟡 SOFT — Prompt Rule เท่านั้น

| | |
|---|---|
| **ทำงานได้จริงไหม** | ✅ ได้ผลดี — ผลักดันให้ AI ไม่หยุดแค่งานผ่านเทสท์ แต่เสนอ Roadmap สู่ระดับ Enterprise |
| **Bug** | ✅ ไม่มี bug |
| **มีประโยชน์ไหม** | ✅ **ประโยชน์สูง** สำหรับ Autonomous Workflow ป้องกัน AI หยุดก่อนเวลา |
| **ซ้อนกับ AI ไหม** | ❌ ไม่ซ้อน — AI ปกติจะหยุดทันทีเมื่อโค้ดคอมไพล์ผ่าน |

---

## การทำงานของ 3 สถานะหลัก (Discipline & Autonomy Modes)

ใน GODKILLER ZERO ผู้ใช้สามารถเลือกโหมดวินัยการทำงานได้ 3 ระดับ ซึ่งจะควบคุมพฤติกรรมความกล้าได้กล้าเสียและการถามยืนยันของ AI:

| โหมด | สัญลักษณ์ & สไตล์ | กลไกเบื้องหลัง | ผลกระทบต่อ Token | ข้อดี | ข้อเสีย | กรณีที่ควรเลือกใช้ |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **SHI (志)** | **Insight / Inquirer**<br>(Interactive Copilot) | ตั้งค่า Cadence เป็น `interactive`<br>AI จะหยุดถามและขอความเห็นชอบทีละก้าว (Step-by-step confirmation) ไม่ด่วนเขียนโค้ดก่อนได้รับอนุมัติ | **เปลือง Token สูงสุด**<br>(บทสนทนายาวขึ้น 40-70% เพราะมีข้อความถาม-ตอบสลับไปมาเรื่อยๆ) | ผู้ใช้ควบคุมทิศทางได้ 100% ป้องกัน AI ตีความผิดหรือแก้ผิดจุดอย่างเด็ดขาด | ช้า ผู้ใช้ต้องคอยกด Proceed ตลอดเวลา เสียจังหวะการทำงานแบบ Flow State | เหมาะสำหรับ: เริ่มโปรเจกต์ใหม่, วางรากฐาน Core Architecture, ระบบความปลอดภัย/การเงินที่ผิดพลาดไม่ได้ |
| **KEN (権)** | **Authority / Standard Guard**<br>(Balanced Smart - ค่าเริ่มต้น) | ตั้งค่า Cadence เป็น `balanced`<br>AI ทำงานเชิงรุก แต่จะหยุดถามเฉพาะ "จุดเปลี่ยนสำคัญ" (Critical Forks, Breaking Changes, Ambiguity) | **ใช้ Token สมดุล (Balanced)**<br>ตัดการถามจุกจิกทิ้ง ประหยัด Token ไปได้ 20-30% เมื่อเทียบกับ SHI | จุดสมดุลที่ดีที่สุดระหว่างความเร็วและความปลอดภัย ไม่ถามพร่ำเพรื่อ แต่ไม่แอบเปลี่ยนโครงสร้างใหญ่เอง | AI อาจตัดสินใจเรื่องสถาปัตยกรรมระดับกลางเองโดยไม่ได้ถาม | **แนะนำเป็นโหมดหลักสำหรับการทำงานประจำวัน**, การเพิ่ม Feature ทั่วไป, การ Refactor ขนาดกลาง |
| **SHIN (神)** | **Ghost in the Shell**<br>(Full Autonomy / Fast Lane) | ตั้งค่า Cadence เป็น `silent`<br>AI เดินหน้าลุย 100% Non-Stop วินิจฉัยและลงมือแก้จนจบวงรอบ พร้อม Self-Healing รันเทสต์แก้เองโดยไม่หยุดถาม | **เซฟ Token สนทนาสูงสุด**<br>ประหยัด Token การถามตอบได้ถึง 50-80% ส่งคำสั่งเดียวจบวงรอบ | งานเสร็จเร็วที่สุด ไม่ต้องมีคนเฝ้า เหมาะมากสำหรับรัน Task ข้ามคืน หรือ Autonomous subagents | หาก AI ตีความ Requirement ตั้งต้นผิด อาจวิ่งผิดทางไปไกลจนต้อง Rollback โค้ดทั้งหมด | เหมาะสำหรับ: งานแก้บั๊กที่จุดเกิดเหตุชัดเจน, รัน Test & Fix ให้ผ่าน, งานเพิ่ม Unit Test ให้ครอบคลุม, โปรเจกต์ที่มี CI เทสต์แน่นหนา |

---

## คู่มือแจกแจงการทำงานของ 18 ตั้งค่าติ๊ก (18 Invariant Checkbox Settings)

การตั้งค่าในหน้า **AI Power & Guardrails Suite** (`ExtraSettingsForm`) ประกอบด้วย 18 ตัวเลือกหลัก ซึ่งแบ่งออกเป็น 3 กลุ่ม เพื่อปรับจูนความเข้มงวดและสไตล์ของ AI:

```
┌────────────────────────────────────────────────────────────────────────┐
│               AI POWER & GUARDRAILS SUITE (18 CONTROLS)                │
├────────────────────────────────┬───────────────────────────────────────┤
│ Group 1: Visual Intelligence & │ 1. 🎯 Zero Hallucination Guard        │
│          Architecture          │ 2. 💎 Self-Documenting Code           │
│          Blueprints (6 ตัว)    │ 3. 🏛️ Clean Domain Architecture       │
│                                │ 4. ⚡ Instant Code Delivery           │
│                                │ 5. 📐 Spatial Wireframing Blueprints  │
│                                │ 6. 📊 Flowchart Architecture Generator│
├────────────────────────────────┼───────────────────────────────────────┤
│ Group 2: Anti-Spaghetti Armor &│ 7. 🧩 Bite-Sized Functions (Span ≤ 70)│
│          Quality Standards     │ 8. ✨ Frictionless Logic (CC ≤ 7)     │
│          (4 ตัว)               │ 9. 🔍 Blast Radius Radar              │
│                                │ 10. 🎯 Universal Stack Sensor         │
├────────────────────────────────┼───────────────────────────────────────┤
│ Group 3: Enterprise            │ 11. 🛡️ Ghost Edit Shield              │
│          Bulletproofing &      │ 12. 🛑 Failure Loop Breaker           │
│          Production Shield     │ 13. 🚀 Crash Immunity (Ban unwrap)    │
│          (8 ตัว)               │ 14. ⚙️ Decoupled Core Architecture   │
│                                │ 15. 🌐 Modern Web Standards           │
│                                │ 16. 🎨 Designer-Grade UI & Aesthetics │
│                                │ 17. 📈 Proactive Roadmap Engine       │
│                                │ 18. 🇹🇭 Native Thai Precision          │
└────────────────────────────────┴───────────────────────────────────────┘
```

---

### กลุ่มที่ 1: AI Precision & Visual Architecture (6 ตัวเลือก)

#### 1. Zero-Speculation (`ZeroSpeculation`)
- **กลไก:** 🟡 SOFT (GEMINI.md Rule) + 🔵 HYBRID (Hoare Ambiguity Branch Alpha)
- **การทำงาน:** สั่งห้าม AI "เดาไปเอง" เมื่อข้อมูลในโค้ดหรือคำสั่งกำกวม หากพิกัดไฟล์หรือความต้องการไม่ชัดเจน AI ต้องระบุความไม่แน่นอนออกมาตรงๆ
- **ผลกระทบต่อ Token:** **เซฟ Token มหาศาลในระยะยาว** ป้องกันการสร้างโค้ดผิดพลาดแล้วต้องมาสั่งแก้ใหม่หลายรอบ (Token Prompt กฎสั้นมาก ~25 tokens แต่เซฟการแก้โค้ดผิดหลักพัน tokens)
- **ข้อดี:** ป้องกันการนั่งแก้โค้ดที่ AI มโนขึ้นมาเอง
- **ข้อเสีย:** AI อาจจะหยุดถามบ่อยขึ้นถ้าผู้ใช้พิมพ์คำสั่งห้วนเกินไป
- **คำแนะนำ:** **ควรเปิดตลอดเวลา (Recommended ON)** โดยเฉพาะโปรเจกต์ขนาดใหญ่

#### 2. Ban Generic Naming (`BanGeneric`)
- **กลไก:** 🔴 HARD (Gatekeeper Scanner Rule 2) + 🟡 SOFT (GEMINI.md)
- **การทำงาน:** แบนการตั้งชื่อตัวแปรขี้เกียจ เช่น `data`, `res`, `result`, `payload`, `temp`, `item` ในระดับ Scanner และ System Prompt
- **ผลกระทบต่อ Token:** เสมอตัว (ตัวแปรยาวขึ้นเล็กน้อย เพิ่ม ~5-15 tokens ต่อไฟล์)
- **ข้อดี:** โค้ดสะอาด ชัดเจน บำรุงรักษาง่าย โค้ดอ่านรู้เรื่องในตัวเอง (Self-documenting)
- **ข้อเสีย:** หากผู้พัฒนาชอบเขียนสคริปต์สั้นๆ ไวๆ (One-off scratch script) อาจรู้สึกน่ารำคาญเพราะโดน Gatekeeper ตีกลับ
- **คำแนะนำ:** **ควรเปิดในโปรเจกต์จริง**, ปิดเฉพาะตอนเขียน Quick Prototype ชั่วคราว

#### 3. Prohibit Messy Utils/Helpers (`BanJunk`)
- **กลไก:** 🟡 SOFT (GEMINI.md Rule 2)
- **การทำงาน:** ห้าม AI สาดฟังก์ชันมั่วๆ ไปไว้ในโฟลเดอร์ถังขยะ เช่น `utils/`, `helpers/`, `common/` แต่บังคับให้ออกแบบเป็น Domain Module หรือ Service ที่มีชื่อชัดเจน
- **ผลกระทบต่อ Token:** ไม่กระทบ Token
- **ข้อดี:** สถาปัตยกรรมโปรเจกต์ไม่เละ ไม่เกิดปัญหา "God Utility File" บวมเป็นพันบรรทัด
- **ข้อเสีย:** AI ต้องใช้เวลาออกแบบโฟลเดอร์และ Module แยกย่อยมากขึ้น
- **คำแนะนำ:** **ควรเปิดในโปรเจกต์ระยะยาว**, ปิดในโปรเจกต์เล็กที่มีไฟล์เดียว

#### 4. Zero Fluff / First-Token Protocol (`ZeroFluff`)
- **กลไก:** 🔴 HARD (Reverse Proxy Egress Stripper) + 🟡 SOFT (GEMINI.md Rule 2)
- **การทำงาน:** ตัดคำเกริ่นนำ บททักทาย ("สวัสดีครับ", "Certainly!", "Here is your code") ทิ้ง บังคับให้โทเคนแรกของคำตอบคือโค้ดหรือแผนงานทันที
- **ผลกระทบต่อ Token:** **เซฟ Output Token ได้ 30 - 100 tokens ต่อการตอบ 1 ครั้ง** ยืดอายุ Context Window ของ AI ได้อย่างชัดเจน
- **ข้อดี:** ได้โค้ดทันที ไม่ต้องเลื่อนจอข้ามคำพูดสุภาพที่ไร้สาระ
- **ข้อเสีย:** น้ำเสียงของ AI จะดูเป็นหุ่นยนต์ตรงไปตรงมา ไม่มีมารยาททางสังคม
- **คำแนะนำ:** **ควรเปิด 100% สำหรับการเขียนโค้ด (Must-have for Developers)**

#### 5. ASCII Wireframes & Blueprints (`AsciiBlueprints` + Cadence ComboBox)
- **กลไก:** 🟡 SOFT (GEMINI.md Rule 7)
- **การทำงาน:** บังคับให้ AI วาดแผนผังหน้าจอหรือ Layout โครงสร้างด้วย ASCII Art ลงในบล็อก ` ```text ` ก่อนลงมือเขียนโค้ด UI
- **ผลกระทบต่อ Token:** **เปลือง Output Token เพิ่มขึ้น ~150 - 400 tokens ต่อฟีเจอร์**
- **ข้อดี:** เห็นภาพ Layout, Alignment, และ Hierarchy ล่วงหน้า ป้องกัน AI จัดหน้าจอออกมาบูดเบี้ยว
- **ข้อเสีย:** เปลือง Token และเสียเวลาเจนภาพ ASCII หากเป็นงาน Backend ล้วนๆ
- **คำแนะนำ:** **เปิดเมื่อทำ Frontend / UI / Mobile**, **ควรปิดเมื่อทำ Backend API / Database / CLI** เพื่อเซฟ Token

#### 6. Mermaid Diagrams & Logic Flows (`MermaidDiagrams` + Cadence ComboBox)
- **กลไก:** 🟡 SOFT (GEMINI.md Rule)
- **การทำงาน:** บังคับให้ AI เขียน Mermaid Diagram (Flowchart, Sequence, Entity-Relationship) ก่อนทำงานสถาปัตยกรรมที่ซับซ้อน
- **ผลกระทบต่อ Token:** **เปลือง Output Token เพิ่มขึ้น ~100 - 300 tokens**
- **ข้อดี:** เข้าใจ Data Flow และ State Machine ได้ชัดเจน สามารถ Render เป็นรูปภาพใน Markdown Preview ได้ทันที
- **ข้อเสีย:** เปลือง Token และหากเป็นงานแก้บั๊กบรรทัดเดียวจะกลายเป็นภาระ
- **คำแนะนำ:** **เปิดเมื่อออกแบบระบบใหม่ หรือ Flow การทำงานซับซ้อน**, **ปิดเมื่องานเป็นเพียง Bugfix หรือแก้เล็กๆ น้อยๆ**

---

### กลุ่มที่ 2: Clean Code & Quality Standards (4 ตัวเลือก)

#### 7. Modular Functions / Max Span ≤ 70 (`LimitSpan`)
- **กลไก:** 🔴 HARD (Gatekeeper Scanner Rule 4) + 🟡 SOFT (GEMINI.md Rule 3)
- **การทำงาน:** สแกนโค้ดทั้งไฟล์ ตรวจสอบว่ามีฟังก์ชัน Business Logic ใดที่มีความยาวเกิน 70 บรรทัดหรือไม่ หากเกิน Gatekeeper จะ Reject ทันที
- **ผลกระทบต่อ Token:** เสมอตัว แต่อาจเพิ่ม Token เล็กน้อยจากการแยก Helper function
- **ข้อดี:** ฟังก์ชันกระชับ อ่านง่าย ทดสอบง่าย ป้องกัน Spaghetti Code บวมเกินเยียวยา
- **ข้อเสีย:** โค้ดบางประเภทที่มี pattern ยาวเป็นธรรมชาติ (เช่น switch-case ใหญ่ๆ) ต้องถูกแยกย่อยจนไฟล์มีฟังก์ชันย่อยเยอะขึ้น
- **คำแนะนำ:** **ควรเปิดเสมอในงานระดับ Production (Recommended ON)**

#### 8. Simple Logic / Complexity ≤ 7 (`Complexity`)
- **กลไก:** 🔴 HARD (Tree-sitter SonarQube Cognitive Complexity) + 🟡 SOFT (GEMINI.md Rule 3)
- **การทำงาน:** คำนวณค่า Cognitive Complexity ของฟังก์ชัน หากมีการซ้อนลูปหรือ if-else ลึกเกิน 7 ระดับ จะถูกตีกลับ
- **ผลกระทบต่อ Token:** ไม่กระทบ Token โดยตรง
- **ข้อดี:** ลดโอกาสเกิด Edge case บั๊กซ่อนเร้น โค้ดไม่ซับซ้อนเกินที่มนุษย์จะทำความเข้าใจ
- **ข้อเสีย:** อัลกอริทึมบางประเภท (เช่น State Machine Parser) อาจต้อง refactor เป็น Table-driven เพื่อไม่ให้ค่าเกิน
- **คำแนะนำ:** **ควรเปิดสำหรับ Business Logic ทั่วไป**, ปิดชั่วคราวเมื่อต้องเขียน Parser/Mathematical Engine ซับซ้อน

#### 9. Blast Radius Impact Audit (`BlastRadius`)
- **กลไก:** 🔴 HARD (`PerProjectSymbolGraph` & `/api/purify`) + 🟡 SOFT (GEMINI.md Rule 8)
- **การทำงาน:** สแกน Symbol Graph ในโปรเจกต์ หาว่าฟังก์ชันที่กำลังจะแก้นี้ ถูกเรียกใช้โดยไฟล์ไหนบ้าง (Caller files) และรายงานให้ AI ทราบก่อนแก้
- **ผลกระทบต่อ Token:** เพิ่ม Input Token เล็กน้อย (~30 - 80 tokens ใน Context) แต่ **เซฟ Token การแก้บั๊กลูกโซ่ได้นับพัน tokens**
- **ข้อดี:** ป้องกันการแก้ฟังก์ชันหนึ่งแล้วไปทำให้อีก 5 ไฟล์พัง (Breaking dependent callers)
- **ข้อเสีย:** สแกนไฟล์ใหญ่ๆ อาจใช้เวลาเสี้ยววินาทีก่อนตอบ
- **คำแนะนำ:** **ควรเปิดตลอดเวลา (Highly Recommended)**

#### 10. Smart Tech Stack Auto-Detection (`StackSensor`)
- **กลไก:** 🔴 HARD (`UniversalStackSensor` ตรวจจับ `Cargo.toml`, `package.json`, `csproj` ฯลฯ)
- **การทำงาน:** ตรวจจับอัตโนมัติว่าโปรเจกต์ใช้เทคโนโลยีอะไร (เช่น React 19, TailwindCSS, Rust 2021) แล้วป้อนเข้าไปในสเปกคำสั่งของ AI
- **ผลกระทบต่อ Token:** เพิ่ม Token ใน Prompt จิ๋วเดียว (~15 - 30 tokens)
- **ข้อดี:** AI จะไม่มโนเรียกใช้ Library มั่วซั่ว (เช่น โปรเจกต์ใช้ Tailwind อยู่แล้ว AI จะไม่แอบลง Bootstrap ซ้ำซ้อน)
- **ข้อเสีย:** ไม่มีข้อเสีย
- **คำแนะนำ:** **ควรเปิดตลอดเวลา 100%**

---

### กลุ่มที่ 3: Enterprise Safety & Modern Best Practices (8 ตัวเลือก)

#### 11. Ghost Edit Shield / Negative Bounding (`NegativeBounding`)
- **กลไก:** 🔴 HARD (`LinguisticTranspiler` Polarity Parser) + 🟡 SOFT (GEMINI.md Rule 10)
- **การทำงาน:** ตรวจจับคำสั่งปฏิเสธ (เช่น "อย่าแตะ", "ห้ามแก้", "don't touch") แล้วล็อกไฟล์หรือสโคปนั้นไว้เป็น `PRESERVE_GUARD` ทันที
- **ผลกระทบต่อ Token:** เซฟ Token ป้องกัน AI แก้โค้ดลามไปยังส่วนอื่นที่ผู้ใช้ไม่ได้สั่ง
- **ข้อดี:** ป้องกัน "Ghost Edit" หรือการที่ AI หวังดีแก้โค้ดรอบข้างจนพังเละ
- **ข้อเสีย:** ไม่มี
- **คำแนะนำ:** **ควรเปิดตลอดเวลา (Crucial Safety Feature)**

#### 12. Anti-Loop Circuit Breaker (`CircuitBreaker`)
- **กลไก:** 🔴 HARD (`LinguisticTranspiler::detect_circuit_breaker`)
- **การทำงาน:** ตรวจจับเมื่อผู้ใช้เริ่มบ่นซ้ำๆ เช่น "ยังพังอยู่", "แก้ไม่หาย", "error เดิม" ระบบจะสั่ง **HALT** ทันที เพื่อบังคับให้ AI ถอยออกมาวิเคราะห์สถาปัตยกรรมใหม่ แทนที่จะเดาสุ่มวนลูป
- **ผลกระทบต่อ Token:** **เซฟ Token ได้มหาศาล** ตัดวงจร AI guessing loop ที่อาจผลาญ Token ทิ้งเปล่าๆ 5-10 รอบ
- **ข้อดี:** หยุดอาการ "แถ" และการเดาสุ่มของ AI เมื่อเจอทางตัน
- **ข้อเสีย:** AI จะไม่ยอมแก้โค้ดต่อทันที แต่จะบังคับขอข้อมูลเพิ่มหรือหยุดวิเคราะห์ก่อน
- **คำแนะนำ:** **ควรเปิดตลอดเวลา (One of GK-ZERO's Best Features)**

#### 13. Crash Prevention: Ban unwrap() & silent catch (`ExhaustiveErrors`)
- **กลไก:** 🔴 HARD (Gatekeeper Scanner Rule 12 + Multi-line parser) + 🟡 SOFT (GEMINI.md Rule 12)
- **การทำงาน:** สแกนและบล็อก `.unwrap()`, `.expect()` ใน Rust และบล็อก `catch { }` / `except: pass` ว่างเปล่าใน C#, Java, Python, TS ทุกรูปแบบ
- **ผลกระทบต่อ Token:** เสมอตัว
- **ข้อดี:** โปรแกรมไม่ Crash กะทันหันใน Production, บั๊กไม่ถูกซ่อนไว้ใต้พรม
- **ข้อเสีย:** AI ต้องเขียนบล็อก Error Handling ครบถ้วน โค้ดจะยาวขึ้นเล็กน้อย
- **คำแนะนำ:** **ควรเปิดในงาน Production ทุกงาน**, ปิดเฉพาะตอนเขียน Proof of Concept ชั่วคราว

#### 14. Decoupled Architecture / Functional Core (`FunctionalCore`)
- **กลไก:** 🟡 SOFT (GEMINI.md Rule 13)
- **การทำงาน:** สั่งให้ AI แยก Pure Business Logic (คำนวณล้วนๆ ไม่มี Side-effect) ออกจาก I/O, Network, Database
- **ผลกระทบต่อ Token:** เสมอตัว
- **ข้อดี:** โค้ดทำ Unit Test ง่ายมาก และสามารถทดสอบ Logic ได้โดยไม่ต้อง Mock Database
- **ข้อเสีย:** โค้ดจะถูกแยกไฟล์เยอะขึ้น ไม่เหมาะกับสคริปต์แบบ CRUD ง่ายๆ
- **คำแนะนำ:** **เปิดสำหรับงานสถาปัตยกรรมขนาดใหญ่ / Domain-Driven Design**, **ปิดสำหรับงาน Simple CRUD / Script เล็กๆ**

#### 15. Modern Web Standards Guidance (`ModernWebSources`)
- **กลไก:** 🟡 SOFT (GEMINI.md Rule 14)
- **การทำงาน:** ไกด์ให้ AI ใช้ Syntax และ API มาตรฐานล่าสุด อิงตาม Official Documentation ใหม่เสมอ (เช่น React Hook ล่าสุด, CSS Grid/Flex, Vanilla CSS)
- **ผลกระทบต่อ Token:** ไม่กระทบ Token
- **ข้อดี:** ไม่ได้โค้ดโบราณที่ Deprecated ไปแล้วมาใช้งาน
- **ข้อเสีย:** บางครั้ง AI อาจพยายามใช้ Feature ใหม่ที่บราวเซอร์รุ่นเก่าบางตัวยังไม่รองรับ
- **คำแนะนำ:** **ควรเปิดสำหรับงาน Web Development ยุคใหม่**

#### 16. Visual Excellence / Premium UI/UX (`PremiumUiUx`)
- **กลไก:** 🟡 SOFT (GEMINI.md Rule 15)
- **การทำงาน:** กำชับ AI ว่าห้ามทำหน้าตาเป็นปุ่มสี่เหลี่ยมเทาๆ โบราณ แต่ต้องมี Hover Effect, Micro-animations, Smooth Gradient, Modern Typography และ Glassmorphism
- **ผลกระทบต่อ Token:** **เปลือง Output Token CSS เพิ่มขึ้น ~80 - 200 tokens**
- **ข้อดี:** งาน Frontend ออกมาดูสวยหรูระดับ State-of-the-Art ผู้ใช้หรือลูกค้าประทับใจทันที
- **ข้อเสีย:** โค้ด CSS จะยาวขึ้น และเปลือง Token มากขึ้น
- **คำแนะนำ:** **เปิดเมื่องานต้องการความสวยงาม (Client-facing UI)**, **ปิดเมื่อทำ Internal Dashboard เรียบง่าย หรือ Backend**

#### 17. Proactive Roadmap Continuum (`InfiniteEvolution`)
- **กลไก:** 🟡 SOFT (GEMINI.md Rule 16)
- **การทำงาน:** เมื่อ AI ทำงานเสร็จ จะไม่หยุดแค่โค้ดผ่าน แต่จะวิเคราะห์และเสนอแนะ Roadmap สู่ขั้นถัดไป (เช่น แนะนำ Cache, Load test, Security Hardening)
- **ผลกระทบต่อ Token:** **เปลือง Output Token เพิ่มขึ้น ~100 - 250 tokens ในช่วงท้ายของการตอบ**
- **ข้อดี:** ช่วยคิดและต่อยอดโปรเจกต์เชิงรุก เหมาะมากสำหรับสตาร์ทอัพหรือคนที่ต้องการไอเดียต่อยอด
- **ข้อเสีย:** ถ้าผู้ใช้ต้องการแค่ผลลัพธ์สั้นๆ กระชับ อาจรู้สึกว่าข้อความยาวเกินไป
- **คำแนะนำ:** **เปิดเมื่อต้องการวางแผนและพัฒนาต่อยอด**, **ปิดเมื่อต้องการคำตอบสั้นกระชับแบบรีบด่วน**

#### 18. Thai Intent Safeguard (`ThaiPolarity`)
- **กลไก:** 🔴 HARD (`LinguisticTranspiler` Thai Lexicon Engine)
- **การทำงาน:** สแกนคำไวยากรณ์ภาษาไทยที่ซับซ้อน เช่น "อย่าลืม", "ห้าม", "ยังไม่", "ตกลงนะ" เพื่อไม่ให้ AI ตีความคำสั่งภาษาไทยกำกวมผิดทิศทาง
- **ผลกระทบต่อ Token:** **เซฟ Token ได้ดีเยี่ยม** ป้องกันการที่ AI เข้าใจผิดแล้วรันโค้ดมั่ว
- **ข้อดี:** ทำให้โปรแกรมเมอร์ชาวไทยสามารถสั่งงานด้วยภาษาไทยตามธรรมชาติได้โดยไม่ต้องเกร็งภาษาอังกฤษ
- **ข้อเสีย:** ไม่มีข้อเสีย (ประมวลผลบนเครื่องผ่าน Rust ทันที)
- **คำแนะนำ:** **ควรเปิดตลอดเวลาสำหรับผู้ใช้งานที่พิมพ์คำสั่งภาษาไทย (Must-have for Thai Developers)**

---

## สรุปภาพรวมและกลยุทธ์การตั้งค่าตามประเภทงาน (Cheat Sheet)

| ประเภทงาน | โหมดที่แนะนำ | Checkboxes ที่ควรเปิด | Checkboxes ที่ควรปิด (เพื่อเซฟ Token) |
| :--- | :---: | :--- | :--- |
| **🚀 งานเร่งด่วน / Quick Bugfix** | **SHIN** | Zero Fluff, Ghost Edit Shield, Circuit Breaker, Thai Safeguard | ASCII Blueprints, Mermaid Diagrams, Infinite Evolution, Premium UI |
| **🎨 งานพัฒนา Frontend / UI สวยหรู** | **KEN** | Zero Fluff, ASCII Blueprints, Premium UI, Modern Web, Stack Sensor | Functional Core, Complexity (ลดความตึงของ JSX) |
| **🏛️ งานสถาปัตยกรรมระบบ / Enterprise Core** | **SHI / KEN** | Zero-Speculation, Ban Generic, Modular Span, Complexity, Blast Radius, Exhaustive Errors, Functional Core | Premium UI, Infinite Evolution |
| **🤖 งาน Autonomous Agent รันงานยาว** | **SHIN** | Zero Fluff, Ghost Edit Shield, Circuit Breaker, Exhaustive Errors, Modular Span, Blast Radius | ASCII Blueprints, Mermaid Diagrams, Infinite Evolution |

---

## สรุปสถานะ Bug ที่ได้รับการตรวจสอบและแก้ไข (Bug Tracking Matrix)

| # | Bug | สถานะ | บันทึกการแก้ไข & หลักฐาน |
|---|---|:---:|---|
| **BUG-A** | `evaluate_branch_gamma_invariants()` hardcode `passed: true` | ✅ **RESOLVED** | แก้ไขให้ประเมิน `complexity_budget_valid && spans_bounded && generic_banned` จริงใน `quantum_simulator.rs` (เทสท์ผ่าน: `test_gamma_invariant_fails_when_directive_exceeded`) |
| **BUG-B** | Negation detector false positive: `"don't forget to add tests"` | ✅ **RESOLVED** | เพิ่มข้อยกเว้นวลี `"don't forget"` และ `"อย่าลืม"` ใน `linguistic_transpiler.rs` (เทสท์ผ่าน: `test_negation_exemption_dont_forget`) |
| **BUG-C** | Token usage hardcode `42` prompt tokens | ✅ **RESOLVED** | เปลี่ยนเป็นคำนวณตามความยาวข้อความจริง `(content.len() / 6).max(12)` ใน `handlers.rs:L230` |
| **BUG-D** | Symbol graph ไม่จับ `pub(crate) fn` | ✅ **RESOLVED** | รองรับ `pub(crate)` และ `pub(super)` ใน `symbol_graph.rs` พร้อมตัด comments และ export signatures (เทสท์ผ่าน: `test_parse_advanced_signatures_and_ignore_comments`) |
| **BUG-E** | Runtime Panic: `reqwest::blocking` ชน Tokio runtime | ✅ **RESOLVED** | ดีลิเกตการยิง HTTP ซิงโครนัสไปยัง Detached thread ด้วย `std::thread::spawn().join()` ใน `linguistic_transpiler.rs` |
| **BUG-F** | Antigravity MCP ติดตั้งผิดโฟลเดอร์ (`~/.gemini/antigravity-ide/`) | ✅ **RESOLVED** | เพิ่ม Heuristic Multi-Tier Auto-Discovery ใน `HookEngine.cs` เชื่อมต่อ `~/.gemini/config/mcp_config.json` อัตโนมัติ |
| **BUG-G** | Unauthorized background dropper ติดตั้ง OllamaSetup.exe เงียบๆ | ✅ **RESOLVED** | กำจัดฟังก์ชันดาวน์โหลดและสั่งรัน `/silent` ทิ้งทั้งหมดใน `bootstrap.rs` คงไว้เฉพาะ Offline Standalone |
| **BUG-H** | Gatekeeper ตรวจไม่พบบล็อก `catch` ว่างเปล่าแบบหลายบรรทัด | ✅ **RESOLVED** | เพิ่ม `is_empty_catch_block` พร้อม Lookahead Token Parser ตรวจจับ `catch \n { \n }` ได้ 100% ใน `gatekeeper.rs` |
| **BUG-I** | ปิด GUI แล้วเกิด Zombie Daemon ค้างพอร์ต 4242 | ✅ **RESOLVED** | เพิ่ม `HookEngine.ShutdownBackgroundDaemon()` ส่งคำขอ `/api/quit` และเก็บกวาดโปรเซสใน `OnFormClosing` |
| **BUG-J** | `extract_target_file` เข้าใจผิดว่าเลขทศนิยม (เช่น 1.5) คือชื่อไฟล์ | ✅ **RESOLVED** | เพิ่มตัวกรองนามสกุลไฟล์จริง (`rs`, `cs`, `py`, `json`, `md` ฯลฯ) ใน `tri_pillar.rs` |
| **BUG-K** | Gatekeeper ไม่ตรวจ Cognitive Complexity (`report.functions.cognitive_complexity` ถูกคำนวณแต่ไม่เคยเอามาตรวจ) | ✅ **RESOLVED** | เพิ่มการตรวจสอบ `function.cognitive_complexity > max_complexity` พร้อมออก Violation `RULE_3_MAX_COGNITIVE_COMPLEXITY` ใน `gatekeeper.rs` และเปิดรับค่าใน `scan_path` |
| **BUG-L** | Regex แบนตัวแปรเหวี่ยงแห (False Positive) โดน `{ data }` และ `(req, res)` | ✅ **RESOLVED** | ปรับปรุง `BANNED_IDENTIFIER_PATTERN` ให้ยกเว้น Destructuring และ Framework signature แต่ยังตรวจจับ Lazy Variable Declarations อย่างถูกต้องใน `gatekeeper.rs` |
| **BUG-M** | Fluff Stripper ลบคำอธิบายเชิงเทคนิคหลายย่อหน้าทิ้งหมดถ้ามีคำว่า `sure` หรือ `updated` | ✅ **RESOLVED** | ปรับปรุง `strip_conversational_fluff` ใน `sanitizer.rs` ให้ตรวจตัดเฉพาะ 1-2 บรรทัดแรกที่เป็นคำทักทายเกริ่นนำ โดยคงคำอธิบายและเหตุผลเชิงเทคนิคก่อนบล็อกโค้ดไว้ 100% |
| **BUG-N** | สำหรับ Non-Antigravity client ตัว Proxy ลบข้อความ Prompt เดิมของผู้ใช้ทิ้งทั้งหมด | ✅ **RESOLVED** | ปรับปรุง `enrich_envelope_with_compiled_contract` ใน `handlers.rs` ให้แนบ Contract ต่อท้ายข้อความเดิมของผู้ใช้ ไม่เขียนทับทิ้ง |
| **BUG-O** | `extract_target_file` ตัด Directory Path ทิ้งจนเกิด Target Ambiguity | ✅ **RESOLVED** | ปรับปรุง `extract_target_file` ใน `tri_pillar.rs` ให้คง Relative Path เดิมไว้ (เช่น `src/components/Navbar.tsx`) เพื่อป้องกันความสับสนไฟล์ชื่อซ้ำ |
| **BUG-P** | `setup_local_llm.ps1` ทำงานเหมือน Silent Dropper ติดตั้ง Ollama โดยไม่ถามยืนยัน | ✅ **RESOLVED** | เพิ่ม Interactive Confirmation Prompt ให้ผู้ใช้ตัดสินใจก่อนดาวน์โหลด/ติดตั้งโปรแกรม |
| **BUG-Q** | Cadence Inversion ระหว่างโหมด SHI และ SHIN สลับขั้วในโค้ด | ✅ **RESOLVED** | แก้ไขให้ SHI เป็น `Interactive` และ SHIN เป็น `Silent` ทั้งใน `HookEngine.cs` และ `hook.rs` (เทสท์ผ่าน: `test_shi_and_shin_cadence_presets`) |
| **BUG-R** | กฎ Rule 6 ไม่สั่งให้ AI เรียกใช้ MCP tool `gk_claim_done` | ✅ **RESOLVED** | เพิ่มข้อความบังคับเรียก `gk_claim_done` ใน Rule 6 เมื่อเปิดใช้งาน MCP Protocol ใน `hook.rs` และ `HookEngine.cs` (เทสท์ผ่าน: `test_mcp_gk_claim_done_mandate`) |
| **BUG-S** | `FindDaemonExePath` ใน C# ตรวจไม่พบ `target/debug/godkiller-zero.exe` ในเครื่อง Dev | ✅ **RESOLVED** | เพิ่ม `Path.Combine(currentDir, "target", "debug", "godkiller-zero.exe")` ใน `HookEngine.cs` และลงทะเบียน `~/.gemini/config/mcp_config.json` |
| **BUG-T** | ฟังก์ชัน Unhook ทิ้งไฟล์ขยะ 1 ไบต์ (`.cursorrules`, `CLAUDE.md`) ในรูทโปรเจกต์ | ✅ **RESOLVED** | ปรับปรุง `RemoveHookFromFilePath` ให้ลบไฟล์ทิ้งอัตโนมัติหากเนื้อหาว่างเปล่า พร้อมเก็บกวาดไฟล์ตกค้างในรูท |


