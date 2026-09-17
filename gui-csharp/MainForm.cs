using System;
using System.Diagnostics;
using System.Drawing;
using System.Net.Http;
using System.Runtime.InteropServices;
using System.Text;
using System.Text.Json;
using System.Text.RegularExpressions;
using System.Windows.Forms;
using Microsoft.Win32;

namespace GodkillerZeroGui;

public class MainForm : Form
{
    private string _activeDiscipline = "KEN";
    private InvariantOptions _options;
    private TargetEngine _currentTargetEngine = TargetEngine.Universal;

    // Header & Status
    private readonly Label _lblStatusBadge;
    private readonly Label _lblDisciplineHeader;
    private readonly Label _lblDisciplineTelemetry;
    private readonly Label _lblRulesStatus;
    private readonly ComboBox _cboTargetEngine;

    // Mode Buttons
    private readonly GSegmentedButton _btnModeShi;
    private readonly GSegmentedButton _btnModeKen;
    private readonly GSegmentedButton _btnModeShin;
    private readonly GSegmentedButton _btnModeExtra;

    // Rules Hook / Unhook Controller (Section 2)
    private readonly GSegmentedButton _btnHook;
    private readonly GSegmentedButton _btnUnhook;

    // Compiler Box
    private readonly TextBox _txtCrucible;
    private readonly Label _lblScope;
    private readonly Label _lblPolarity;
    private readonly Button _btnCompile;

    // Startup & Footer
    private readonly CheckBox _chkStartup;
    private readonly Button _btnFootHook;
    private readonly Button _btnFootUnhook;
    private readonly Button _btnFootUpdates;
    private readonly Button _btnFootQuit;

    private NotifyIcon? _trayIcon;
    private EventWaitHandle? _wakeUpEvent;
    private RegisteredWaitHandle? _registeredWait;
    private System.Threading.CancellationTokenSource? _pipeCts;

    private readonly HttpClient _httpClient = new HttpClient { Timeout = TimeSpan.FromSeconds(30) };

    public MainForm()
    {
        _activeDiscipline = HookEngine.GetCurrentDiscipline();
        _options = HookEngine.LoadOptionsFromDisk();

        Text = "GODKILLER ZORO 1.0";
        ClientSize = new Size(380, 485);
        FormBorderStyle = FormBorderStyle.FixedSingle;
        MaximizeBox = false;
        StartPosition = FormStartPosition.CenterScreen;
        BackColor = Theme.BgCanvas;
        ForeColor = Theme.TextTitle;
        ShowInTaskbar = true;

        Icon = CreateAppIcon();
        InitializeTrayIcon();

        StartPipeListener();
        InitializeWakeUpEvent();

        // Footer Bar
        var pnlFooter = new Panel
        {
            Dock = DockStyle.Bottom,
            Height = 44,
            BackColor = Color.FromArgb(25, 26, 29)
        };

        _btnFootHook = CreateFooterButton("SHIELD", 8, (s, e) => ApplyHook(_activeDiscipline));
        _btnFootUnhook = CreateFooterButton("RESTORE", 96, (s, e) => ApplyUnhook(), isDanger: true);
        _btnFootUpdates = CreateFooterButton("Scan", 184, (s, e) => RunProjectGatekeeperScan());
        _btnFootQuit = CreateFooterButton("Quit", 272, (s, e) => Close(), isDanger: true);

        pnlFooter.Controls.AddRange(new Control[] { _btnFootHook, _btnFootUnhook, _btnFootUpdates, _btnFootQuit });

        // Main Container
        var pnlMain = new Panel
        {
            Dock = DockStyle.Fill,
            AutoScroll = false,
            Padding = new Padding(12, 8, 12, 8)
        };

        int top = 8;

        // SECTION 1: MODE SELECTOR
        var pnlSectionMode = CreateCardBox(ref top, 58);
        _lblDisciplineHeader = new Label { Visible = false };
        _lblDisciplineTelemetry = new Label { Visible = false };

        _btnModeShi = new GSegmentedButton
        {
            PrimaryText = "Silent",
            SubText = "SHI",
            Location = new Point(6, 6),
            Size = new Size(80, 46)
        };
        _btnModeShi.Click += (s, e) => SetDiscipline("SHI");

        _btnModeKen = new GSegmentedButton
        {
            PrimaryText = "Balanced",
            SubText = "KEN",
            IsActive = true,
            Location = new Point(90, 6),
            Size = new Size(80, 46)
        };
        _btnModeKen.Click += (s, e) => SetDiscipline("KEN");

        _btnModeShin = new GSegmentedButton
        {
            PrimaryText = "Turbo",
            SubText = "SHIN",
            Location = new Point(174, 6),
            Size = new Size(80, 46)
        };
        _btnModeShin.Click += (s, e) => SetDiscipline("SHIN");

        _btnModeExtra = new GSegmentedButton
        {
            PrimaryText = "Extra",
            SubText = "Tuning",
            Location = new Point(258, 6),
            Size = new Size(80, 46)
        };
        _btnModeExtra.Click += (s, e) => OpenExtraSettings();

        pnlSectionMode.Controls.AddRange(new Control[] { _lblDisciplineHeader, _lblDisciplineTelemetry, _btnModeShi, _btnModeKen, _btnModeShin, _btnModeExtra });
        pnlMain.Controls.Add(pnlSectionMode);

        // SECTION 2: IDE RULES CONTROLLER (SHIELD / RESTORE)
        top += 8;
        var pnlSectionRules = CreateCardBox(ref top, 114);
        _lblRulesStatus = new Label
        {
            Text = "RULE CONTROLLER",
            Font = Theme.FontSemibold,
            ForeColor = Theme.TextTitle,
            Location = new Point(8, 6),
            AutoSize = true
        };
        _lblStatusBadge = new Label
        {
            Text = "● Shielded",
            Font = Theme.FontSemibold,
            ForeColor = Theme.BorderActive,
            Location = new Point(220, 6),
            Size = new Size(120, 18),
            TextAlign = ContentAlignment.MiddleRight
        };
        var lblRulesTelemetry = new Label { Visible = false };

        _cboTargetEngine = new ComboBox
        {
            Location = new Point(6, 26),
            Size = new Size(332, 24),
            DropDownStyle = ComboBoxStyle.DropDownList,
            BackColor = Color.FromArgb(28, 30, 36),
            ForeColor = Theme.TextTitle,
            Font = Theme.FontRegular,
            FlatStyle = FlatStyle.Flat
        };
        _cboTargetEngine.Items.AddRange(new object[]
        {
            "ALL",
            "Antigravity",
            "Cursor",
            "Claude",
            "VS Code Copilot"
        });
        _cboTargetEngine.SelectedIndex = 0;
        _cboTargetEngine.SelectedIndexChanged += (s, e) =>
        {
            _currentTargetEngine = (TargetEngine)_cboTargetEngine.SelectedIndex;
            SyncState();
        };

        _btnHook = new GSegmentedButton
        {
            PrimaryText = "SHIELD",
            SubText = string.Empty,
            Location = new Point(6, 56),
            Size = new Size(164, 48)
        };
        _btnHook.Click += (s, e) => ApplyHook(_activeDiscipline);

        _btnUnhook = new GSegmentedButton
        {
            PrimaryText = "RESTORE",
            SubText = string.Empty,
            IsDangerStyle = true,
            Location = new Point(174, 56),
            Size = new Size(164, 48)
        };
        _btnUnhook.Click += (s, e) => ApplyUnhook();

        pnlSectionRules.Controls.AddRange(new Control[] { _lblRulesStatus, _lblStatusBadge, lblRulesTelemetry, _cboTargetEngine, _btnHook, _btnUnhook });
        pnlMain.Controls.Add(pnlSectionRules);

        // SECTION 3: PRE-FLIGHT COMPILER
        top += 8;
        var pnlSectionCrucible = CreateCardBox(ref top, 140);
        var lblCrucibleTitle = new Label
        {
            Text = "Pre-Flight Contract Compiler",
            Font = Theme.FontSemibold,
            ForeColor = Theme.TextTitle,
            Location = new Point(8, 6),
            AutoSize = true
        };
        var lblLlmBadge = new Label
        {
            Text = "● Local AI: qwen2.5-coder",
            Font = Theme.FontSmall,
            ForeColor = Theme.BorderActive,
            Location = new Point(180, 7),
            Size = new Size(160, 18),
            TextAlign = ContentAlignment.MiddleRight
        };

        _txtCrucible = new TextBox
        {
            Multiline = true,
            Location = new Point(8, 26),
            Size = new Size(330, 68),
            BackColor = Color.FromArgb(24, 25, 29),
            ForeColor = Theme.TextTitle,
            BorderStyle = BorderStyle.FixedSingle,
            Font = Theme.FontRegular,
            PlaceholderText = "Input intent to compile into zero-vibe contract (e.g. Update Navbar)..."
        };
        _txtCrucible.TextChanged += TxtCrucible_TextChanged;

        _lblScope = new Label
        {
            Text = "Scope: Auto-Detected",
            Font = Theme.FontSmall,
            ForeColor = Theme.TextMuted,
            Location = new Point(8, 104),
            Size = new Size(205, 20),
            TextAlign = ContentAlignment.MiddleLeft
        };

        _lblPolarity = new Label
        {
            Text = string.Empty,
            Font = Theme.FontSmall,
            ForeColor = Theme.BorderActive,
            Location = new Point(140, 104),
            Size = new Size(80, 20),
            TextAlign = ContentAlignment.MiddleLeft,
            Visible = false
        };

        _btnCompile = new Button
        {
            Text = "Compile & Copy",
            Font = Theme.FontSemibold,
            Size = new Size(118, 26),
            Location = new Point(220, 102),
            BackColor = Theme.BgTile,
            ForeColor = Theme.TextTitle,
            FlatStyle = FlatStyle.Flat,
            Cursor = Cursors.Hand,
            UseMnemonic = false
        };
        _btnCompile.FlatAppearance.BorderSize = 1;
        _btnCompile.FlatAppearance.BorderColor = Theme.BorderTile;
        _btnCompile.Click += BtnCompile_Click;

        pnlSectionCrucible.Controls.AddRange(new Control[] { lblCrucibleTitle, lblLlmBadge, _txtCrucible, _lblScope, _lblPolarity, _btnCompile });
        pnlMain.Controls.Add(pnlSectionCrucible);

        // SECTION 4: STARTUP ROW
        top += 8;
        _chkStartup = new CheckBox
        {
            Text = "Run on Startup",
            Font = Theme.FontRegular,
            ForeColor = Theme.TextBody,
            Location = new Point(16, top),
            Size = new Size(140, 22),
            Checked = QueryStartupStatus(),
            Cursor = Cursors.Hand
        };
        _chkStartup.CheckedChanged += ChkStartup_CheckedChanged;

        var lblVersion = new Label
        {
            Text = "Version: 1.0.0 • @kayvin.th",
            Font = Theme.FontSmall,
            ForeColor = Theme.TextMuted,
            Location = new Point(170, top + 2),
            Size = new Size(180, 18),
            TextAlign = ContentAlignment.MiddleRight,
            Cursor = Cursors.Hand
        };
        lblVersion.MouseEnter += (s, e) => lblVersion.ForeColor = Theme.BorderActive;
        lblVersion.MouseLeave += (s, e) => lblVersion.ForeColor = Theme.TextMuted;
        lblVersion.Click += (s, e) =>
        {
            try
            {
                Process.Start(new ProcessStartInfo
                {
                    FileName = "https://www.instagram.com/kayvin.th?stkn=MXR0ZW96OWRkY3V1dA%3D%3D&utm_source=qr",
                    UseShellExecute = true
                });
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine(ex.Message);
            }
        };

        pnlMain.Controls.AddRange(new Control[] { _chkStartup, lblVersion });

        Controls.Add(pnlMain);
        Controls.Add(pnlFooter);

        // Initial state sync
        SyncState();

        var syncTimer = new System.Windows.Forms.Timer { Interval = 2000 };
        syncTimer.Tick += (s, e) => SyncState();
        syncTimer.Start();
    }

    private Panel CreateCardBox(ref int top, int height)
    {
        var pnl = new Panel
        {
            Location = new Point(8, top),
            Size = new Size(348, height),
            BackColor = Theme.BgCard
        };
        pnl.Paint += (s, e) =>
        {
            var rect = new Rectangle(0, 0, pnl.Width - 1, pnl.Height - 1);
            using var pen = new Pen(Theme.BorderCard);
            e.Graphics.DrawRectangle(pen, rect);
        };
        top += height;
        return pnl;
    }

    private Button CreateFooterButton(string text, int x, EventHandler onClick, bool isDanger = false)
    {
        var btn = new Button
        {
            Text = text,
            Font = Theme.FontSemibold,
            Size = new Size(82, 28),
            Location = new Point(x, 8),
            BackColor = Theme.BgTile,
            ForeColor = isDanger ? Color.FromArgb(248, 113, 113) : Theme.TextTitle,
            FlatStyle = FlatStyle.Flat,
            Cursor = Cursors.Hand
        };
        btn.FlatAppearance.BorderSize = 1;
        btn.FlatAppearance.BorderColor = Theme.BorderTile;
        btn.Click += onClick;
        return btn;
    }

    private void SyncState()
    {
        bool isHooked = HookEngine.IsHooked(_currentTargetEngine);
        _activeDiscipline = HookEngine.GetCurrentDiscipline();
        _options = HookEngine.LoadOptionsFromDisk();

        if (isHooked)
        {
            _lblStatusBadge.Text = $"● Shielded ({_activeDiscipline})";
            _lblStatusBadge.ForeColor = Theme.BorderActive;
            _lblRulesStatus.Text = "RULE CONTROLLER";

            _btnHook.IsActive = true;
            _btnUnhook.IsActive = false;
        }
        else
        {
            _lblStatusBadge.Text = "● Restored";
            _lblStatusBadge.ForeColor = Theme.BorderUnhook;
            _lblRulesStatus.Text = "RULE CONTROLLER";

            _btnHook.IsActive = false;
            _btnUnhook.IsActive = true;
        }

        UpdateModeTiles();
        _btnHook.Invalidate();
        _btnUnhook.Invalidate();
    }

    private void SetDiscipline(string discipline)
    {
        _activeDiscipline = discipline.ToUpper();
        _options = HookEngine.GetOptionsForDiscipline(_activeDiscipline);

        UpdateModeTiles();
        ApplyHook(_activeDiscipline);
    }

    private void UpdateModeTiles()
    {
        _btnModeShi.IsActive = _activeDiscipline == "SHI";
        _btnModeKen.IsActive = _activeDiscipline == "KEN";
        _btnModeShin.IsActive = _activeDiscipline == "SHIN";

        _btnModeShi.Invalidate();
        _btnModeKen.Invalidate();
        _btnModeShin.Invalidate();
    }

    private void ApplyHook(string discipline)
    {
        bool success = HookEngine.Hook(discipline, _options, _currentTargetEngine);
        SyncState();
        if (success)
        {
            MessageBox.Show(this, "Cognitive Invariants hooked successfully across target engines.", "GODKILLER ZERO", MessageBoxButtons.OK, MessageBoxIcon.Information);
        }
        else
        {
            MessageBox.Show(this, "Failed to hook invariants into target engines.", "GODKILLER ZERO", MessageBoxButtons.OK, MessageBoxIcon.Error);
        }
    }

    private void ApplyUnhook()
    {
        bool success = HookEngine.Unhook(_currentTargetEngine);
        SyncState();
        if (success)
        {
            MessageBox.Show(this, "Cognitive Invariants unhooked. Environment restored to default.", "GODKILLER ZERO", MessageBoxButtons.OK, MessageBoxIcon.Information);
        }
    }

    private void OpenExtraSettings()
    {
        using var form = new ExtraSettingsForm(_options);
        form.ShowDialog(this);
        if (form.Saved)
        {
            HookEngine.Hook(_activeDiscipline, _options, _currentTargetEngine);
            SyncState();
        }
        else if (form.UnhookRequested)
        {
            ApplyUnhook();
        }
    }

    private void TxtCrucible_TextChanged(object? sender, EventArgs e)
    {
        string rawText = _txtCrucible.Text.Trim();
        if (string.IsNullOrEmpty(rawText))
        {
            _lblScope.Text = "Scope: Auto-Detected";
            _lblPolarity.Visible = false;
            return;
        }

        string lower = rawText.ToLower();
        string[] failureKeywords = { "ยังไม่ได้", "ยังไม่หาย", "พังเหมือนเดิม", "เหมือนเดิม", "still fails", "same error", "looping" };
        string[] negationKeywords = { "อย่า", "ห้าม", "ไม่ต้อง", "ไม่ควร", "don't", "avoid", "never", "preserve" };

        bool isBreaker = Array.Exists(failureKeywords, kw => lower.Contains(kw));
        bool isNegated = Array.Exists(negationKeywords, kw => lower.Contains(kw));

        if (isBreaker)
        {
            _lblPolarity.Text = "Circuit Breaker";
            _lblPolarity.ForeColor = Color.FromArgb(245, 158, 11);
            _lblPolarity.Visible = true;
        }
        else if (isNegated)
        {
            _lblPolarity.Text = "Ghost Edit Shield";
            _lblPolarity.ForeColor = Color.FromArgb(34, 197, 94);
            _lblPolarity.Visible = true;
        }
        else
        {
            _lblPolarity.Visible = false;
        }

        var match = Regex.Match(rawText, @"[a-zA-Z0-9_\-\./\\]+\.[a-zA-Z0-9]+");
        if (match.Success)
        {
            _lblScope.Text = $"File: {match.Value}";
        }
        else
        {
            _lblScope.Text = "Scope: Domain-Anchored";
        }
    }

    private async void BtnCompile_Click(object? sender, EventArgs e)
    {
        string prompt = _txtCrucible.Text.Trim();
        if (string.IsNullOrEmpty(prompt))
        {
            _lblScope.Text = "Please enter prompt first";
            _lblScope.ForeColor = Color.FromArgb(245, 158, 11);
            _txtCrucible.Focus();
            return;
        }

        _btnCompile.Enabled = false;
        _btnCompile.Text = "AI Translating...";
        _lblScope.Text = "AI Translating (qwen2.5)...";
        _lblScope.ForeColor = Theme.BorderActive;

        try
        {
            string compiled = await QueryLocalLlmAsync(prompt);
            _txtCrucible.Text = compiled;
            _txtCrucible.SelectAll();
            Clipboard.SetText(compiled);

            _lblScope.Text = "Copied to clipboard (AI)";
            _lblScope.ForeColor = Color.FromArgb(34, 197, 94);
        }
        catch
        {
            Clipboard.SetText(prompt);
            _lblScope.Text = "Fallback: Copied input";
            _lblScope.ForeColor = Color.FromArgb(239, 68, 68);
        }
        finally
        {
            _btnCompile.Enabled = true;
            _btnCompile.Text = "Compile & Copy";
        }
    }

    private async Task<string> QueryLocalLlmAsync(string prompt)
    {
        string? ollamaOutput = await QueryOllamaLlmAsync(prompt);
        if (!string.IsNullOrWhiteSpace(ollamaOutput))
        {
            return ollamaOutput;
        }

        string? daemonOutput = await QueryLocalDaemonPurifyAsync(prompt);
        if (!string.IsNullOrWhiteSpace(daemonOutput))
        {
            return daemonOutput;
        }

        return prompt;
    }

    private async Task<string?> QueryOllamaLlmAsync(string prompt)
    {
        try
        {
            const string systemPrompt = "You are an expert technical translator. Translate any Thai input directly into natural, concise English for software developers and AI assistants. Keep negative constraints like 'อย่า' or 'ห้าม' as 'Do not modify/touch'. For casual greetings, translate naturally. Return ONLY the direct English translation without explanations, quotes, or markdown.";
            var ollamaReq = new
            {
                model = "qwen2.5-coder:1.5b",
                messages = new object[]
                {
                    new { role = "system", content = systemPrompt },
                    new { role = "user", content = "ช่วยแก้ปุ่มตรงคลังสินค้าหน่อย" },
                    new { role = "assistant", content = "Update the warehouse button component." },
                    new { role = "user", content = "กินข้าวหรือยัง" },
                    new { role = "assistant", content = "Have you eaten yet?" },
                    new { role = "user", content = "อย่าแตะต้องไฟล์ config.json" },
                    new { role = "assistant", content = "Do not modify the config.json file." },
                    new { role = "user", content = prompt }
                },
                stream = false,
                options = new { temperature = 0.1, num_predict = 128 }
            };
            string reqJson = JsonSerializer.Serialize(ollamaReq);
            var content = new StringContent(reqJson, Encoding.UTF8, "application/json");
            var resp = await _httpClient.PostAsync("http://127.0.0.1:11434/api/chat", content);
            if (resp.IsSuccessStatusCode)
            {
                string resJson = await resp.Content.ReadAsStringAsync();
                using var doc = JsonDocument.Parse(resJson);
                if (doc.RootElement.TryGetProperty("message", out var msg) && msg.TryGetProperty("content", out var c))
                {
                    string cleanedText = c.GetString()?.Trim() ?? prompt;
                    if (cleanedText.StartsWith("```") && cleanedText.EndsWith("```"))
                    {
                        var lines = cleanedText.Split('\n');
                        if (lines.Length >= 2)
                            cleanedText = string.Join("\n", lines.Skip(1).Take(lines.Length - 2)).Trim();
                    }
                    if ((cleanedText.StartsWith("\"") && cleanedText.EndsWith("\"")) || (cleanedText.StartsWith("'") && cleanedText.EndsWith("'")))
                    {
                        if (cleanedText.Length >= 2)
                            cleanedText = cleanedText.Substring(1, cleanedText.Length - 2).Trim();
                    }
                    if (!string.IsNullOrWhiteSpace(cleanedText)) return cleanedText;
                }
            }
        }
        catch (Exception exOllama)
        {
            System.Diagnostics.Debug.WriteLine($"Ollama translation error: {exOllama.Message}");
        }
        return null;
    }

    private async Task<string?> QueryLocalDaemonPurifyAsync(string prompt)
    {
        try
        {
            string requestPayload = $"{{\"prompt\":\"{prompt.Replace("\"", "\\\"")}\",\"workspace_path\":\".\"}}";
            var content = new StringContent(requestPayload, Encoding.UTF8, "application/json");
            var response = await _httpClient.PostAsync("http://127.0.0.1:4242/api/purify", content);
            if (response.IsSuccessStatusCode)
            {
                string json = await response.Content.ReadAsStringAsync();
                using var doc = JsonDocument.Parse(json);
                if (doc.RootElement.TryGetProperty("concise_english", out var eng) && !string.IsNullOrWhiteSpace(eng.GetString()))
                {
                    return eng.GetString()!;
                }
                if (doc.RootElement.TryGetProperty("dense_ir", out var ir) && !string.IsNullOrWhiteSpace(ir.GetString()))
                {
                    return ir.GetString()!;
                }
            }
        }
        catch (Exception exDaemon)
        {
            System.Diagnostics.Debug.WriteLine($"Local daemon purify error: {exDaemon.Message}");
        }
        return null;
    }

    private static bool QueryStartupStatus()
    {
        try
        {
            using var key = Registry.CurrentUser.OpenSubKey(@"Software\Microsoft\Windows\CurrentVersion\Run", false);
            return key?.GetValue("GodkillerZero") != null;
        }
        catch
        {
            return false;
        }
    }

    private void ChkStartup_CheckedChanged(object? sender, EventArgs e)
    {
        try
        {
            using var key = Registry.CurrentUser.OpenSubKey(@"Software\Microsoft\Windows\CurrentVersion\Run", true);
            if (key == null) return;

            if (_chkStartup.Checked)
            {
                string exePath = Application.ExecutablePath;
                key.SetValue("GodkillerZero", $"\"{exePath}\"");
            }
            else
            {
                key.DeleteValue("GodkillerZero", false);
            }
        }
        catch (Exception ex)
        {
            System.Diagnostics.Debug.WriteLine(ex.Message);
        }
    }

    private void CheckUpdates()
    {
        MessageBox.Show(this, "Version 1.0.0 is up to date.\nRepository: taurus42119-stack/godkiller-zero", "Updates", MessageBoxButtons.OK, MessageBoxIcon.Information);
    }

    private void RunProjectGatekeeperScan()
    {
        using var fbd = new FolderBrowserDialog
        {
            Description = "Select Project Directory for GODKILLER ZERO Invariant Audit",
            UseDescriptionForTitle = true
        };

        if (fbd.ShowDialog(this) == DialogResult.OK)
        {
            string selectedPath = fbd.SelectedPath;
            string? exePath = HookEngine.FindDaemonExePath();
            if (!string.IsNullOrEmpty(exePath) && File.Exists(exePath))
            {
                var psi = new ProcessStartInfo
                {
                    FileName = exePath,
                    Arguments = $"--gate \"{selectedPath}\"",
                    UseShellExecute = false,
                    RedirectStandardOutput = true,
                    RedirectStandardError = true,
                    CreateNoWindow = true
                };

                using var proc = Process.Start(psi);
                if (proc != null)
                {
                    string output = proc.StandardOutput.ReadToEnd();
                    string error = proc.StandardError.ReadToEnd();
                    proc.WaitForExit(15000);
                    string display = string.IsNullOrWhiteSpace(output) ? error : output;
                    using var resultDialog = new ScanResultForm(selectedPath, proc.ExitCode == 0, display);
                    resultDialog.ShowDialog(this);
                    return;
                }
            }
            MessageBox.Show(this, "Could not locate godkiller-zero.exe engine. Please ensure it is compiled and present in the project directory.", "Audit Error", MessageBoxButtons.OK, MessageBoxIcon.Error);
        }
    }

    [DllImport("user32.dll", SetLastError = true)]
    private static extern bool ChangeWindowMessageFilter(uint message, uint dwFlag);

    [DllImport("user32.dll")]
    private static extern bool ShowWindow(IntPtr hWnd, int nCmdShow);

    [DllImport("user32.dll")]
    private static extern bool SetForegroundWindow(IntPtr hWnd);

    [DllImport("user32.dll")]
    private static extern uint GetWindowThreadProcessId(IntPtr hWnd, IntPtr ProcessId);

    [DllImport("user32.dll")]
    private static extern bool AttachThreadInput(uint idAttach, uint idAttachTo, bool fAttach);

    [DllImport("kernel32.dll")]
    private static extern uint GetCurrentThreadId();

    [DllImport("user32.dll")]
    private static extern IntPtr GetForegroundWindow();

    private const int SW_RESTORE = 9;
    private const int SW_SHOW = 5;

    protected override void OnHandleCreated(EventArgs e)
    {
        base.OnHandleCreated(e);
        Program.Log($"OnHandleCreated: Handle = 0x{Handle.ToInt64():X}");
        Theme.EnableDarkMode(Handle);
        try
        {
            ChangeWindowMessageFilter(WmActivateWindow, 1);
        }
        catch (Exception exFilter)
        {
            System.Diagnostics.Debug.WriteLine($"ChangeWindowMessageFilter error: {exFilter.Message}");
        }
    }

    protected override void OnShown(EventArgs e)
    {
        base.OnShown(e);
        Program.Log("OnShown called.");
        BringToFront();
        Activate();
        Task.Run(() =>
        {
            HookEngine.EnsureBackgroundDaemon();
            if (HookEngine.IsHooked())
            {
                HookEngine.ProvisionMcpConfig();
            }
            EnsureDesktopShortcut();
        });
    }

    private void StartPipeListener()
    {
        _pipeCts = new System.Threading.CancellationTokenSource();
        var token = _pipeCts.Token;
        Task.Run(async () =>
        {
            while (!token.IsCancellationRequested && !IsDisposed)
            {
                try
                {
                    using var server = new System.IO.Pipes.NamedPipeServerStream(
                        Program.ScopedPipeName,
                        System.IO.Pipes.PipeDirection.In,
                        1,
                        System.IO.Pipes.PipeTransmissionMode.Byte,
                        System.IO.Pipes.PipeOptions.Asynchronous);

                    await server.WaitForConnectionAsync(token);
                    int b = server.ReadByte();
                    if (b != -1 && !IsDisposed)
                    {
                        try 
                        { 
                            BeginInvoke(new Action(ShowAndActivate)); 
                        } 
                        catch (Exception exPipeShow)
                        {
                            System.Diagnostics.Debug.WriteLine($"Pipe invoke error: {exPipeShow.Message}");
                        }
                    }
                }
                catch (OperationCanceledException) { break; }
                catch (ObjectDisposedException) { break; }
                catch (Exception exPipeLoop)
                {
                    System.Diagnostics.Debug.WriteLine($"Pipe listener iteration error: {exPipeLoop.Message}");
                    try { await Task.Delay(300, token); } catch { break; }
                }
            }
        }, token);
    }

    [DllImport("user32.dll")]
    private static extern bool SetWindowPos(IntPtr hWnd, IntPtr hWndInsertAfter, int X, int Y, int cx, int cy, uint uFlags);

    public void ShowAndActivate()
    {
        if (InvokeRequired)
        {
            try 
            { 
                BeginInvoke(new Action(ShowAndActivate)); 
            } 
            catch (Exception exInvokeShow)
            {
                System.Diagnostics.Debug.WriteLine($"ShowAndActivate invoke error: {exInvokeShow.Message}");
            }
            return;
        }

        try
        {
            if (WindowState == FormWindowState.Minimized)
            {
                WindowState = FormWindowState.Normal;
            }
            Show();
            ShowWindow(Handle, 9);
            SetWindowPos(Handle, new IntPtr(-1), 0, 0, 0, 0, 0x0001 | 0x0002 | 0x0040);
            SetWindowPos(Handle, new IntPtr(-2), 0, 0, 0, 0, 0x0001 | 0x0002 | 0x0040);
            SetForegroundWindow(Handle);
            BringToFront();
            Activate();
        }
        catch (Exception exActivate)
        {
            System.Diagnostics.Debug.WriteLine($"ShowAndActivate error: {exActivate.Message}");
        }
    }

    private void InitializeTrayIcon()
    {
        _trayIcon = new NotifyIcon
        {
            Text = "GODKILLER ZORO 1.0",
            Icon = CreateAppIcon(),
            Visible = true
        };

        var menu = new ContextMenuStrip();
        menu.Items.Add("Open GODKILLER ZORO", null, (s, e) => ShowAndActivate());
        menu.Items.Add("SHIELD", null, (s, e) => ApplyHook(_activeDiscipline));
        menu.Items.Add("RESTORE", null, (s, e) => ApplyUnhook());
        menu.Items.Add(new ToolStripSeparator());
        menu.Items.Add("Exit", null, (s, e) => Close());

        _trayIcon.ContextMenuStrip = menu;
        _trayIcon.DoubleClick += (s, e) => ShowAndActivate();
    }

    private static Bitmap? LoadAppBitmap()
    {
        string baseDirectory = AppDomain.CurrentDomain.BaseDirectory;
        string imageFilePath = Path.Combine(baseDirectory, "app.png");
        if (File.Exists(imageFilePath))
        {
            return new Bitmap(imageFilePath);
        }

        var executingAssembly = typeof(MainForm).Assembly;
        using var stream = executingAssembly.GetManifestResourceStream("GodkillerZeroGui.app.png");
        if (stream != null)
        {
            return new Bitmap(stream);
        }

        return null;
    }

    public static Icon CreateAppIcon()
    {
        try
        {
            string baseDirectory = AppDomain.CurrentDomain.BaseDirectory;
            string iconFilePath = Path.Combine(baseDirectory, "app.ico");
            if (File.Exists(iconFilePath))
            {
                return new Icon(iconFilePath, 32, 32);
            }

            var executingAssembly = typeof(MainForm).Assembly;
            using var iconStream = executingAssembly.GetManifestResourceStream("GodkillerZeroGui.app.ico");
            if (iconStream != null)
            {
                return new Icon(iconStream, 32, 32);
            }

            string executablePath = Process.GetCurrentProcess().MainModule?.FileName ?? string.Empty;
            if (!string.IsNullOrEmpty(executablePath) && File.Exists(executablePath))
            {
                var associatedExecutableIcon = Icon.ExtractAssociatedIcon(executablePath);
                if (associatedExecutableIcon != null)
                {
                    return associatedExecutableIcon;
                }
            }
        }
        catch
        {
            // Graceful fallback to embedded geometric glyph
        }

        using var fallbackSurface = new Bitmap(16, 16);
        using var graphicsSurface = Graphics.FromImage(fallbackSurface);
        graphicsSurface.SmoothingMode = System.Drawing.Drawing2D.SmoothingMode.AntiAlias;
        graphicsSurface.Clear(Color.Transparent);
        using var brandBrush = new SolidBrush(Color.FromArgb(16, 185, 129));
        graphicsSurface.FillPolygon(brandBrush, new Point[] {
            new Point(8, 1),
            new Point(14, 4),
            new Point(12, 11),
            new Point(8, 15),
            new Point(4, 11),
            new Point(2, 4)
        });
        IntPtr nativeIconHandle = fallbackSurface.GetHicon();
        return Icon.FromHandle(nativeIconHandle);
    }

    private static void EnsureDesktopShortcut()
    {
        try
        {
            string desktopDirectory = Environment.GetFolderPath(Environment.SpecialFolder.Desktop);
            string desktopShortcutPath = Path.Combine(desktopDirectory, "GODKILLER ZORO 1.0.lnk");
            if (File.Exists(desktopShortcutPath))
            {
                return;
            }

            string executablePath = Process.GetCurrentProcess().MainModule?.FileName ?? string.Empty;
            if (string.IsNullOrEmpty(executablePath) || !File.Exists(executablePath))
            {
                return;
            }

            string baseDirectory = Path.GetDirectoryName(executablePath) ?? string.Empty;
            string iconPathCandidate = Path.Combine(baseDirectory, "app.ico");
            string iconCoordinate = File.Exists(iconPathCandidate) ? iconPathCandidate : executablePath;

            string escapedShortcut = desktopShortcutPath.Replace("'", "''");
            string escapedTarget = executablePath.Replace("'", "''");
            string escapedWorkingDir = baseDirectory.Replace("'", "''");
            string escapedIcon = iconCoordinate.Replace("'", "''");

            string powershellScript = $"$ws = New-Object -ComObject WScript.Shell; $s = $ws.CreateShortcut('{escapedShortcut}'); $s.TargetPath = '{escapedTarget}'; $s.WorkingDirectory = '{escapedWorkingDir}'; $s.IconLocation = '{escapedIcon},0'; $s.Description = 'GODKILLER ZORO 1.0'; $s.Save()";
            var processStartCoordinates = new ProcessStartInfo
            {
                FileName = "powershell",
                Arguments = $"-NoProfile -WindowStyle Hidden -Command \"{powershellScript}\"",
                CreateNoWindow = true,
                UseShellExecute = false
            };
            Process.Start(processStartCoordinates)?.WaitForExit(3000);
        }
        catch (Exception exceptionTrace)
        {
            System.Diagnostics.Debug.WriteLine(exceptionTrace.Message);
        }
    }

    [DllImport("user32.dll", SetLastError = true, CharSet = CharSet.Auto)]
    private static extern uint RegisterWindowMessage(string lpString);

    private static readonly uint WmActivateWindow = RegisterWindowMessage(Program.ActivateMessageName);

    protected override void WndProc(ref Message m)
    {
        if (m.Msg == WmActivateWindow && WmActivateWindow != 0)
        {
            ShowAndActivate();
            return;
        }
        base.WndProc(ref m);
    }

    protected override void OnFormClosing(FormClosingEventArgs e)
    {
        try
        {
            _pipeCts?.Cancel();
            _pipeCts?.Dispose();
            _registeredWait?.Unregister(null);
            _wakeUpEvent?.Dispose();
            _trayIcon?.Dispose();
            HookEngine.ShutdownBackgroundDaemon();
        }
        catch (Exception exClosing)
        {
            System.Diagnostics.Debug.WriteLine($"Teardown error: {exClosing.Message}");
        }
        base.OnFormClosing(e);
    }

    private void InitializeWakeUpEvent()
    {
        try
        {
            _wakeUpEvent = new EventWaitHandle(false, EventResetMode.AutoReset, Program.ScopedWakeUpEventName);
            _registeredWait = ThreadPool.RegisterWaitForSingleObject(
                _wakeUpEvent,
                (state, timedOut) =>
                {
                    if (!IsDisposed)
                    {
                        try
                        {
                            BeginInvoke(new Action(ShowAndActivate));
                        }
                        catch (Exception exInvoke)
                        {
                            System.Diagnostics.Debug.WriteLine($"WakeUpEvent invoke error: {exInvoke.Message}");
                        }
                    }
                },
                null,
                -1,
                false
            );
        }
        catch (Exception exWakeInit)
        {
            System.Diagnostics.Debug.WriteLine($"WakeUpEvent setup error: {exWakeInit.Message}");
        }
    }
}

