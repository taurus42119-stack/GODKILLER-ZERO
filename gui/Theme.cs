using System;
using System.Drawing;
using System.Drawing.Drawing2D;
using System.Runtime.InteropServices;
using System.Windows.Forms;

namespace GodkillerZeroGui;

public static class Theme
{
    public static readonly Color BgCanvas = Color.FromArgb(22, 23, 26);
    public static readonly Color BgCard = Color.FromArgb(32, 34, 39);
    public static readonly Color BorderCard = Color.FromArgb(43, 46, 54);
    
    public static readonly Color BgTile = Color.FromArgb(38, 41, 48);
    public static readonly Color BorderTile = Color.FromArgb(55, 58, 67);
    public static readonly Color BgTileHover = Color.FromArgb(46, 50, 59);
    public static readonly Color BgTileActive = Color.FromArgb(27, 40, 56);
    public static readonly Color BorderActive = Color.FromArgb(0, 148, 255);
    
    public static readonly Color BgUnhookActive = Color.FromArgb(45, 26, 26);
    public static readonly Color BorderUnhook = Color.FromArgb(239, 68, 68);

    public static readonly Color TextTitle = Color.FromArgb(241, 245, 249);
    public static readonly Color TextBody = Color.FromArgb(203, 213, 225);
    public static readonly Color TextMuted = Color.FromArgb(148, 163, 184);

    public static readonly Font FontTitle = new Font("Segoe UI", 9.5f, FontStyle.Bold);
    public static readonly Font FontRegular = new Font("Segoe UI", 9f, FontStyle.Regular);
    public static readonly Font FontSemibold = new Font("Segoe UI", 9f, FontStyle.Bold);
    public static readonly Font FontSmall = new Font("Segoe UI", 8f, FontStyle.Regular);

    [DllImport("dwmapi.dll")]
    private static extern int DwmSetWindowAttribute(IntPtr hwnd, int attr, ref int attrValue, int attrSize);

    private const int DWMWA_USE_IMMERSIVE_DARK_MODE = 20;

    public static void EnableDarkMode(IntPtr handle)
    {
        try
        {
            int darkMode = 1;
            DwmSetWindowAttribute(handle, DWMWA_USE_IMMERSIVE_DARK_MODE, ref darkMode, sizeof(int));
        }
        catch (Exception ex)
        {
            System.Diagnostics.Debug.WriteLine(ex.Message);
        }
    }

    public static GraphicsPath GetRoundedPath(Rectangle rect, int radius)
    {
        var path = new GraphicsPath();
        float diameter = radius * 2f;
        path.AddArc(rect.X, rect.Y, diameter, diameter, 180, 90);
        path.AddArc(rect.Right - diameter, rect.Y, diameter, diameter, 270, 90);
        path.AddArc(rect.Right - diameter, rect.Bottom - diameter, diameter, diameter, 0, 90);
        path.AddArc(rect.X, rect.Bottom - diameter, diameter, diameter, 90, 90);
        path.CloseFigure();
        return path;
    }
}

public class GSegmentedButton : Control
{
    [System.ComponentModel.DesignerSerializationVisibility(System.ComponentModel.DesignerSerializationVisibility.Hidden)]
    public string PrimaryText { get; set; } = string.Empty;

    [System.ComponentModel.DesignerSerializationVisibility(System.ComponentModel.DesignerSerializationVisibility.Hidden)]
    public string SubText { get; set; } = string.Empty;

    [System.ComponentModel.DesignerSerializationVisibility(System.ComponentModel.DesignerSerializationVisibility.Hidden)]
    public bool IsActive { get; set; }

    [System.ComponentModel.DesignerSerializationVisibility(System.ComponentModel.DesignerSerializationVisibility.Hidden)]
    public bool IsDangerStyle { get; set; }

    private bool _isHovered;

    public GSegmentedButton()
    {
        SetStyle(ControlStyles.AllPaintingInWmPaint | ControlStyles.UserPaint | ControlStyles.OptimizedDoubleBuffer | ControlStyles.ResizeRedraw, true);
        Cursor = Cursors.Hand;
    }

    protected override void OnMouseEnter(EventArgs e)
    {
        _isHovered = true;
        Invalidate();
        base.OnMouseEnter(e);
    }

    protected override void OnMouseLeave(EventArgs e)
    {
        _isHovered = false;
        Invalidate();
        base.OnMouseLeave(e);
    }

    protected override void OnPaint(PaintEventArgs e)
    {
        e.Graphics.SmoothingMode = SmoothingMode.AntiAlias;
        var rect = new Rectangle(1, 1, Width - 3, Height - 3);

        Color backColor;
        Color borderColor;
        int borderWidth = 1;

        if (IsActive)
        {
            if (IsDangerStyle)
            {
                backColor = Theme.BgUnhookActive;
                borderColor = Theme.BorderUnhook;
                borderWidth = 2;
            }
            else
            {
                backColor = Theme.BgTileActive;
                borderColor = Theme.BorderActive;
                borderWidth = 2;
            }
        }
        else if (_isHovered)
        {
            backColor = Theme.BgTileHover;
            borderColor = Color.FromArgb(75, 80, 92);
        }
        else
        {
            backColor = Theme.BgTile;
            borderColor = Theme.BorderTile;
        }

        using (var path = Theme.GetRoundedPath(rect, 5))
        {
            using (var brush = new SolidBrush(backColor))
            {
                e.Graphics.FillPath(brush, path);
            }
            using (var pen = new Pen(borderColor, borderWidth))
            {
                e.Graphics.DrawPath(pen, path);
            }
        }

        // Draw Primary and Sub Text
        int totalHeight = Height;
        bool hasSub = !string.IsNullOrEmpty(SubText);

        var sf = new StringFormat
        {
            Alignment = StringAlignment.Center,
            LineAlignment = StringAlignment.Center
        };

        if (hasSub)
        {
            var primaryRect = new Rectangle(0, (totalHeight / 2) - 15, Width, 16);
            var subRect = new Rectangle(0, (totalHeight / 2) + 1, Width, 14);

            using var primaryBrush = new SolidBrush(IsActive ? Theme.TextTitle : Theme.TextBody);
            using var subBrush = new SolidBrush(IsActive ? (IsDangerStyle ? Color.FromArgb(248, 113, 113) : Theme.BorderActive) : Theme.TextMuted);

            e.Graphics.DrawString(PrimaryText, Theme.FontSemibold, primaryBrush, primaryRect, sf);
            e.Graphics.DrawString(SubText, Theme.FontSmall, subBrush, subRect, sf);
        }
        else
        {
            using var primaryBrush = new SolidBrush(IsActive ? Theme.TextTitle : Theme.TextBody);
            e.Graphics.DrawString(PrimaryText, Theme.FontSemibold, primaryBrush, ClientRectangle, sf);
        }
    }
}
