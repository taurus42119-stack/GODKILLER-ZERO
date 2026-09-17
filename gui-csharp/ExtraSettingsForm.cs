using System;
using System.Drawing;
using System.Windows.Forms;

namespace GodkillerZeroGui;

public class ExtraSettingsForm : Form
{
    private readonly InvariantOptions _options;
    public bool Saved { get; private set; }
    public bool UnhookRequested { get; private set; }

    private readonly ComboBox _cboCadence;
    private readonly CheckBox _chkZeroSpeculation;
    private readonly CheckBox _chkBanGeneric;
    private readonly CheckBox _chkBanJunk;
    private readonly CheckBox _chkZeroFluff;
    private readonly CheckBox _chkAsciiBlueprints;
    private readonly ComboBox _cboAsciiCadence;
    private readonly CheckBox _chkMermaidDiagrams;
    private readonly ComboBox _cboMermaidCadence;
    private readonly CheckBox _chkLimitSpan;
    private readonly CheckBox _chkComplexity;
    private readonly CheckBox _chkBlastRadius;
    private readonly CheckBox _chkStackSensor;
    private readonly CheckBox _chkNegativeBounding;
    private readonly CheckBox _chkCircuitBreaker;
    private readonly CheckBox _chkExhaustiveErrors;
    private readonly CheckBox _chkFunctionalCore;
    private readonly CheckBox _chkModernWeb;
    private readonly CheckBox _chkPremiumUiUx;
    private readonly CheckBox _chkInfiniteEvolution;
    private readonly CheckBox _chkThaiPolarity;
    private readonly CheckBox _chkRepoMap;

    public ExtraSettingsForm(InvariantOptions currentOptions)
    {
        _options = currentOptions;

        Text = "AI Power & Guardrails Suite";
        Icon = MainForm.CreateAppIcon();
        Size = new Size(540, 790);
        FormBorderStyle = FormBorderStyle.FixedSingle;
        MaximizeBox = false;
        MinimizeBox = false;
        StartPosition = FormStartPosition.CenterParent;
        BackColor = Theme.BgCanvas;
        ForeColor = Theme.TextTitle;

        Theme.EnableDarkMode(Handle);

        var panel = new Panel
        {
            Dock = DockStyle.Fill,
            AutoScroll = true,
            Padding = new Padding(12)
        };

        int top = 8;

        var lblCadenceGroup = CreateHeaderLabel("🚀 Autonomy & Collaboration Engine", ref top);
        panel.Controls.Add(lblCadenceGroup);

        _cboCadence = new ComboBox
        {
            Location = new Point(16, top),
            Size = new Size(485, 24),
            DropDownStyle = ComboBoxStyle.DropDownList,
            BackColor = Color.FromArgb(28, 30, 36),
            ForeColor = Theme.TextTitle,
            Font = Theme.FontRegular,
            FlatStyle = FlatStyle.Flat
        };
        _cboCadence.Items.AddRange(new object[]
        {
            "⚡ Full Autonomy: Non-stop execution with intelligent self-healing",
            "⚖️ Smart Balanced (Recommended): Confirms only breaking & high-impact changes",
            "💬 Interactive Copilot: Step-by-step guidance for absolute human control"
        });
        _cboCadence.SelectedIndex = _options.ClarificationCadence.ToLowerInvariant() switch
        {
            "silent" => 0,
            "interactive" => 2,
            _ => 1
        };
        top += 30;
        panel.Controls.Add(_cboCadence);

        top += 8;
        var lblGroup1 = CreateHeaderLabel("✨ Visual Intelligence & Architecture Blueprints", ref top);
        panel.Controls.Add(lblGroup1);

        _chkZeroSpeculation = CreateCheckBox("🎯 Zero Hallucination: Halts wild assumptions & demands clarity when ambiguous", _options.ZeroSpeculation, ref top);
        _chkBanGeneric = CreateCheckBox("💎 Self-Documenting Code: Bans lazy variable names (data, res, req, temp)", _options.BanGeneric, ref top);
        _chkBanJunk = CreateCheckBox("🏛️ Clean Domain Architecture: Bans sloppy junk drawers & messy helper dumps", _options.BanJunk, ref top);
        _chkZeroFluff = CreateCheckBox("⚡ Instant Code Delivery: 100% pure code output, zero polite chitchat", _options.ZeroFluff, ref top);

        _chkAsciiBlueprints = CreateCheckBox("📐 Spatial Wireframing: Generates beautiful ASCII component layouts & UI blueprints", _options.AsciiBlueprints, ref top);
        _cboAsciiCadence = CreateCadenceComboBox(_options.AsciiCadence, ref top);
        _cboAsciiCadence.Enabled = _options.AsciiBlueprints;
        _chkAsciiBlueprints.CheckedChanged += (s, e) => _cboAsciiCadence.Enabled = _chkAsciiBlueprints.Checked;

        _chkMermaidDiagrams = CreateCheckBox("📊 Flowchart Generator: Auto-maps architecture & complex state transitions", _options.MermaidDiagrams, ref top);
        _cboMermaidCadence = CreateCadenceComboBox(_options.MermaidCadence, ref top);
        _cboMermaidCadence.Enabled = _options.MermaidDiagrams;
        _chkMermaidDiagrams.CheckedChanged += (s, e) => _cboMermaidCadence.Enabled = _chkMermaidDiagrams.Checked;

        panel.Controls.AddRange(new Control[] { _chkZeroSpeculation, _chkBanGeneric, _chkBanJunk, _chkZeroFluff, _chkAsciiBlueprints, _cboAsciiCadence, _chkMermaidDiagrams, _cboMermaidCadence });

        top += 8;
        var lblGroup2 = CreateHeaderLabel("🛡️ Anti-Spaghetti Armor & Quality Standards", ref top);
        panel.Controls.Add(lblGroup2);

        _chkLimitSpan = CreateCheckBox("🧩 Bite-Sized Functions: Caps function lengths to guarantee readability", _options.LimitSpan, ref top);
        _chkComplexity = CreateCheckBox("✨ Frictionless Logic: Enforces clean flow & eliminates nested if-else pyramids", _options.Complexity, ref top);
        _chkBlastRadius = CreateCheckBox("🔍 Blast Radius Radar: Audits affected callers before making breaking edits", _options.BlastRadius, ref top);
        _chkStackSensor = CreateCheckBox("🎯 Universal Stack Sensor: Auto-aligns with your exact frameworks & libraries", _options.StackSensor, ref top);

        panel.Controls.AddRange(new Control[] { _chkLimitSpan, _chkComplexity, _chkBlastRadius, _chkStackSensor });

        top += 8;
        var lblGroup3 = CreateHeaderLabel("🔒 Enterprise Bulletproofing & Production Shield", ref top);
        panel.Controls.Add(lblGroup3);

        _chkNegativeBounding = CreateCheckBox("🛡️ Ghost Edit Shield: Absolute guarantee AI never alters untouched code", _options.NegativeBounding, ref top);
        _chkCircuitBreaker = CreateCheckBox("🛑 Failure Loop Breaker: Auto-interrupts repetitive trial-and-error guessing", _options.CircuitBreaker, ref top);
        _chkExhaustiveErrors = CreateCheckBox("🚀 Crash Immunity: Prohibits risky unwrap() & silent catch block swallows", _options.ExhaustiveErrors, ref top);
        _chkFunctionalCore = CreateCheckBox("⚙️ Decoupled Core: Separates pure business logic from side-effects & I/O", _options.FunctionalCore, ref top);
        _chkModernWeb = CreateCheckBox("🌐 Modern Web Standards: Uses latest official best practices & modern APIs", _options.ModernWebSources, ref top);
        _chkPremiumUiUx = CreateCheckBox("🎨 Designer-Grade UI: Crafts smooth micro-animations, glassmorphism & gradients", _options.PremiumUiUx, ref top);
        _chkInfiniteEvolution = CreateCheckBox("📈 Proactive Roadmap: Inspires scalable future phases & enterprise upgrades", _options.InfiniteEvolution, ref top);
        _chkThaiPolarity = CreateCheckBox("🇹🇭 Native Thai Precision: 100% fluent understanding of 'ห้าม', 'อย่า', 'ยังไม่'", _options.ThaiPolarity, ref top);

        panel.Controls.AddRange(new Control[] { _chkNegativeBounding, _chkCircuitBreaker, _chkExhaustiveErrors, _chkFunctionalCore, _chkModernWeb, _chkPremiumUiUx, _chkInfiniteEvolution, _chkThaiPolarity });

        top += 8;
        var lblGroup4 = CreateHeaderLabel("🗺️ Codebase Radar & Context Armor", ref top);
        panel.Controls.Add(lblGroup4);

        _chkRepoMap = CreateCheckBox("🌐 Deep Project Radar: Instant full-codebase awareness with zero token waste", _options.RepoMap, ref top);
        panel.Controls.Add(_chkRepoMap);

        // Footer Action Panel
        var footerPanel = new Panel
        {
            Dock = DockStyle.Bottom,
            Height = 44,
            BackColor = Color.FromArgb(25, 26, 29)
        };

        var btnSave = new Button
        {
            Text = "Save & Apply",
            Font = Theme.FontSemibold,
            Size = new Size(120, 28),
            Location = new Point(16, 8),
            BackColor = Color.FromArgb(0, 120, 215),
            ForeColor = Color.White,
            FlatStyle = FlatStyle.Flat,
            Cursor = Cursors.Hand
        };
        btnSave.FlatAppearance.BorderSize = 0;
        btnSave.Click += (s, e) =>
        {
            SaveOptions();
            Saved = true;
            Close();
        };

        var btnUnhook = new Button
        {
            Text = "Unhook IDE",
            Font = Theme.FontSemibold,
            Size = new Size(110, 28),
            Location = new Point(146, 8),
            BackColor = Color.FromArgb(127, 29, 29),
            ForeColor = Color.FromArgb(254, 202, 202),
            FlatStyle = FlatStyle.Flat,
            Cursor = Cursors.Hand
        };
        btnUnhook.FlatAppearance.BorderSize = 0;
        btnUnhook.Click += (s, e) =>
        {
            UnhookRequested = true;
            Close();
        };

        var btnCancel = new Button
        {
            Text = "Close",
            Font = Theme.FontRegular,
            Size = new Size(90, 28),
            Location = new Point(420, 8),
            BackColor = Theme.BgTile,
            ForeColor = Theme.TextBody,
            FlatStyle = FlatStyle.Flat,
            Cursor = Cursors.Hand
        };
        btnCancel.FlatAppearance.BorderSize = 1;
        btnCancel.FlatAppearance.BorderColor = Theme.BorderTile;
        btnCancel.Click += (s, e) => Close();

        footerPanel.Controls.AddRange(new Control[] { btnSave, btnUnhook, btnCancel });

        Controls.Add(panel);
        Controls.Add(footerPanel);
    }

    private Label CreateHeaderLabel(string text, ref int top)
    {
        var lbl = new Label
        {
            Text = text,
            UseMnemonic = false,
            Font = Theme.FontTitle,
            ForeColor = Theme.BorderActive,
            Location = new Point(12, top),
            AutoSize = true
        };
        top += 24;
        return lbl;
    }

    private CheckBox CreateCheckBox(string text, bool isChecked, ref int top)
    {
        var chk = new CheckBox
        {
            Text = text,
            UseMnemonic = false,
            Checked = isChecked,
            Font = Theme.FontRegular,
            ForeColor = Theme.TextBody,
            Location = new Point(16, top),
            Size = new Size(485, 22),
            Cursor = Cursors.Hand
        };
        top += 25;
        return chk;
    }

    private ComboBox CreateCadenceComboBox(string cadence, ref int top)
    {
        var cbo = new ComboBox
        {
            Location = new Point(34, top),
            Size = new Size(467, 24),
            DropDownStyle = ComboBoxStyle.DropDownList,
            BackColor = Color.FromArgb(28, 30, 36),
            ForeColor = Theme.TextTitle,
            Font = Theme.FontSmall,
            FlatStyle = FlatStyle.Flat
        };
        cbo.Items.AddRange(new object[]
        {
            "⚡ Fast: Plans Only (Converse/Q&A Exempt to save tokens)",
            "🛡️ Normal: Always (Mandatory across Plans & Q&A)"
        });
        cbo.SelectedIndex = string.Equals(cadence, "normal", StringComparison.OrdinalIgnoreCase) ? 1 : 0;
        top += 30;
        return cbo;
    }

    private void SaveOptions()
    {
        _options.ClarificationCadence = _cboCadence.SelectedIndex switch
        {
            0 => "Silent",
            2 => "Interactive",
            _ => "Balanced"
        };
        _options.ZeroSpeculation = _chkZeroSpeculation.Checked;
        _options.BanGeneric = _chkBanGeneric.Checked;
        _options.BanJunk = _chkBanJunk.Checked;
        _options.ZeroFluff = _chkZeroFluff.Checked;
        _options.AsciiBlueprints = _chkAsciiBlueprints.Checked;
        _options.AsciiCadence = _cboAsciiCadence.SelectedIndex == 1 ? "Normal" : "Fast";
        _options.MermaidDiagrams = _chkMermaidDiagrams.Checked;
        _options.MermaidCadence = _cboMermaidCadence.SelectedIndex == 1 ? "Normal" : "Fast";
        _options.LimitSpan = _chkLimitSpan.Checked;
        _options.Complexity = _chkComplexity.Checked;
        _options.BlastRadius = _chkBlastRadius.Checked;
        _options.StackSensor = _chkStackSensor.Checked;
        _options.NegativeBounding = _chkNegativeBounding.Checked;
        _options.CircuitBreaker = _chkCircuitBreaker.Checked;
        _options.ExhaustiveErrors = _chkExhaustiveErrors.Checked;
        _options.FunctionalCore = _chkFunctionalCore.Checked;
        _options.ModernWebSources = _chkModernWeb.Checked;
        _options.PremiumUiUx = _chkPremiumUiUx.Checked;
        _options.InfiniteEvolution = _chkInfiniteEvolution.Checked;
        _options.ThaiPolarity = _chkThaiPolarity.Checked;
        _options.RepoMap = _chkRepoMap.Checked;
    }
}
