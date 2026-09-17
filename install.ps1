# GODKILLER ZERO — Enterprise One-Liner PowerShell Installer
# Usage: irm https://raw.githubusercontent.com/taurus42119-stack/godkiller-zero/main/install.ps1 | iex

$ErrorActionPreference = "Stop"

Write-Host "⚡ [GODKILLER ZERO] Initializing Enterprise Setup..." -ForegroundColor Cyan

$InstallDir = "$env:LOCALAPPDATA\Programs\godkiller-zero"
$ExePath = "$InstallDir\godkiller-zero.exe"

if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

$CurrentExe = "$PSScriptRoot\godkiller-zero.exe"
$CurrentGui = "$PSScriptRoot\GodkillerZeroGui.exe"
$GuiPath = "$InstallDir\GodkillerZeroGui.exe"

if (Test-Path $CurrentExe) {
    Copy-Item $CurrentExe -Destination $ExePath -Force
}
if (Test-Path $CurrentGui) {
    Copy-Item $CurrentGui -Destination $GuiPath -Force
}

if (-not (Test-Path $ExePath)) {
    Write-Host "📥 Downloading latest release binary from GitHub..." -ForegroundColor Yellow
    $DownloadUrl = "https://github.com/taurus42119-stack/godkiller-zero/releases/latest/download/godkiller-zero-windows-x86_64.zip"
    $ZipPath = "$env:TEMP\godkiller-zero.zip"
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $ZipPath -UseBasicParsing
    Expand-Archive -Path $ZipPath -DestinationPath $InstallDir -Force
    Remove-Item $ZipPath -Force
}

# Add to User PATH if not present
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
    Write-Host "✓ Added $InstallDir to User PATH" -ForegroundColor Green
}

# Hook Antigravity with default Balanced (KEN) discipline
Write-Host "⛩️  Hooking Google Antigravity with Zero-Vibe Invariants..." -ForegroundColor Cyan
& "$ExePath" --hook

# Launch compact desktop card
Write-Host "🚀 Launching GODKILLER ZERO Desktop Controller..." -ForegroundColor Green
if (Test-Path $GuiPath) {
    Start-Process -FilePath "$GuiPath"
} else {
    Start-Process -FilePath "$ExePath"
}

Write-Host "✅ GODKILLER ZERO is active and protecting Antigravity IDE & CLI." -ForegroundColor Green
