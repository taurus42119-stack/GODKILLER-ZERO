# 🛡️ GODKILLER ZERO : SECURITY REMEDIATION CHECKLIST (บันทึกงานความปลอดภัย)

บันทึกรายการตรวจสอบและงานความปลอดภัยที่ดำเนินการจัดการเพื่อเปิดเป็น Open Source สาธารณะ และส่งมอบโปรดักชัน

---

## 1. ล้างประวัติอีเมลส่วนตัวใน Git Commit History (ก่อน Push ขึ้น GitHub สาธารณะ)
* **สถานะ:** ✅ **RESOLVED**
* **รายละเอียด:** รวบประวัติ Git Commit ทั้งหมดให้เป็น Commit สะอาด และใช้ No-Reply GitHub Email (`taurus42119@users.noreply.github.com`)
* **การดำเนินการ:**
  1. ตั้งค่าอีเมล Git ให้เป็น GitHub No-Reply:
     ```powershell
     git config user.name "taurus42119"
     git config user.email "taurus42119@users.noreply.github.com"
     ```
  2. รวบประวัติ (Squash) เป็น Commit เดียวที่สะอาดเอี่ยมก่อน Push:
     ```powershell
     git reset $(git commit-tree HEAD^{tree} -m "feat(core): production release of GODKILLER ZERO v1.0.0")
     ```

---

## 2. การจัดการไฟล์ไบนารีและ Debug Symbols ใน Git Tracking
* **สถานะ:** ✅ **RESOLVED**
* **การดำเนินการ:**
  1. ปลดการแทร็กไฟล์ไบนารีออกจาก Git: `git rm --cached godkiller-zero.exe`
  2. อัปเดต [.gitignore](.gitignore) ครอบคลุม:
     * `*.exe`, `*.dll`, `*.pdb`, `*.deps.json`, `*.runtimeconfig.json`
     * `godkiller-zero.exe`
     * `publish/`, `gui-csharp/bin/`, `gui-csharp/obj/`, `bin/`, `obj/`
  3. ปล่อยดาวน์โหลดไฟล์ `.exe` ผ่านระบบ GitHub Actions Release (`.github/workflows/release.yml`) แทนการใส่ในซอร์สโค้ด

---

## 3. การป้องกัน Path Leak ในไฟล์ไบนารี (Filesystem Path Remapping)
* **สถานะ:** ✅ **RESOLVED**
* **Rust Native Binary:**
  * กำหนดค่าผ่าน `--remap-path-prefix` และ `CFLAGS = "-DNDEBUG"` เพื่อแปลง Absolute Path ของเครื่องผู้ใช้เป็น `/godkiller-zero` และ `/user` พร้อมตัด C assert paths ทิ้งทั้งหมด
* **C# GUI Binary (.NET 9):**
  * กำหนดค่าใน [gui-csharp/GodkillerZeroGui.csproj](gui-csharp/GodkillerZeroGui.csproj) โดยเปิดใช้งาน:
    * `<Deterministic>true</Deterministic>`
    * `<PathMap>$(MSBuildProjectDirectory)=/gui-csharp</PathMap>`
  * ป้องกันการรั่วไหลของ Windows Username และ Absolute Directory ใน `.pdb` และ `.dll`

---

## 4. สถาปัตยกรรม Local-First & การแยกขาดจากภายนอก (Zero Cloud Token Leak)
* **สถานะ:** ✅ **RESOLVED**
* **รายละเอียด:**
  * สถาปัตยกรรม Upstream ปัจจุบันทำงานผ่าน **Local Zero Engine** และ **Ollama Bootstrap Manager (`qwen2.5-coder:1.5b`)** ในเครื่องคอมพิวเตอร์ของผู้ใช้ (Offline-First)
  * ยกเลิกการพึ่งพา Cloud API Key ภายนอก ทำให้ไม่มีความเสี่ยงเรื่อง API Key รั่วไหลใน RAM หรือ Terminal Command History
  * Strict Loopback Binding (`127.0.0.1:4242`) ปฏิเสธการเชื่อมต่อจากภายนอกเครือข่าย LAN (`0.0.0.0`)
  * Host-Authority Origin Guard ตรวจสอบ Header `Origin` และ `Referer` สกัดกั้นการโจมตีข้ามโดเมน (CSRF / Subdomain Spoofing)

---

## 5. การเปิดเผยตัวตนผู้พัฒนา (Creator Attribution)
* **สถานะ:** ℹ️ **INTENTIONAL**
* **รายละเอียด:**
  * การแสดงชื่อผู้พัฒนา `taurus42119` และลิงก์ Instagram `@kayvins.th` ใน [README.md](README.md) เป็นความตั้งใจของผู้พัฒนาสำหรับการเผยแพร่สู่สาธารณะ (Public Attribution)
  * ส่วนข้อมูลส่วนตัวอื่น (อีเมลส่วนบุคคล, พาธไฟล์ในระบบปฏิบัติการ) ได้รับการปกป้องอย่างเคร่งครัดตามข้อ 1, 2, และ 3
