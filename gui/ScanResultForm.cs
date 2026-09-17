using System;
using System.Drawing;
using System.Windows.Forms;

namespace GodkillerZeroGui;

public class ScanResultForm : Form
{
    public ScanResultForm(string targetPath, bool success, string reportText)
    {
        Text = "Invariant Gatekeeper Audit Report";
        Size = new Size(520, 580);
        MinimumSize = new Size(420, 400);
        StartPosition = FormStartPosition.CenterParent;
        BackColor = Theme.BgCanvas;
        ForeColor = Theme.TextTitle;

        Theme.EnableDarkMode(Handle);

        var pnlHeader = new Panel
        {
            Dock = DockStyle.Top,
            Height = 64,
            BackColor = Color.FromArgb(28, 30, 35),
            Padding = new Padding(14, 10, 14, 10)
        };

        var lblTitle = new Label
        {
            Text = success ? "[PASS] INVARIANTS SATISFIED" : "[WARN] INVARIANT VIOLATIONS DETECTED",
            Font = Theme.FontTitle,
            ForeColor = success ? Color.FromArgb(52, 211, 153) : Color.FromArgb(248, 113, 113),
            Location = new Point(12, 10),
            AutoSize = true
        };

        var lblPath = new Label
        {
            Text = $"Target: {targetPath}",
            Font = Theme.FontSmall,
            ForeColor = Theme.TextMuted,
            Location = new Point(14, 36),
            Size = new Size(480, 18),
            AutoEllipsis = true
        };

        pnlHeader.Controls.AddRange(new Control[] { lblTitle, lblPath });

        var pnlFooter = new Panel
        {
            Dock = DockStyle.Bottom,
            Height = 50,
            BackColor = Color.FromArgb(25, 26, 29),
            Padding = new Padding(12, 8, 12, 8)
        };

        var btnCopy = new Button
        {
            Text = "Copy Report",
            Font = Theme.FontSemibold,
            ForeColor = Theme.TextTitle,
            BackColor = Theme.BgTile,
            FlatStyle = FlatStyle.Flat,
            Size = new Size(110, 32),
            Location = new Point(14, 9),
            Cursor = Cursors.Hand
        };
        btnCopy.FlatAppearance.BorderColor = Theme.BorderTile;
        btnCopy.Click += (s, e) =>
        {
            try
            {
                Clipboard.SetText(reportText);
                btnCopy.Text = "Copied!";
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine(ex.Message);
            }
        };

        var btnClose = new Button
        {
            Text = "Close",
            Font = Theme.FontSemibold,
            ForeColor = Theme.TextTitle,
            BackColor = Theme.BgTile,
            FlatStyle = FlatStyle.Flat,
            Size = new Size(90, 32),
            Anchor = AnchorStyles.Top | AnchorStyles.Right,
            Location = new Point(pnlFooter.Width - 104, 9),
            Cursor = Cursors.Hand
        };
        btnClose.FlatAppearance.BorderColor = Theme.BorderTile;
        btnClose.Click += (s, e) => Close();
        pnlFooter.Resize += (s, e) => { btnClose.Left = pnlFooter.Width - 104; };

        pnlFooter.Controls.AddRange(new Control[] { btnCopy, btnClose });

        var txtReport = new TextBox
        {
            Dock = DockStyle.Fill,
            Multiline = true,
            ReadOnly = true,
            ScrollBars = ScrollBars.Both,
            WordWrap = false,
            Font = new Font("Consolas", 9f, FontStyle.Regular),
            BackColor = Color.FromArgb(20, 21, 24),
            ForeColor = Color.FromArgb(226, 232, 240),
            BorderStyle = BorderStyle.None,
            Text = reportText
        };

        var pnlTextContainer = new Panel
        {
            Dock = DockStyle.Fill,
            Padding = new Padding(12)
        };
        pnlTextContainer.Controls.Add(txtReport);

        Controls.Add(pnlTextContainer);
        Controls.Add(pnlFooter);
        Controls.Add(pnlHeader);
    }
}
