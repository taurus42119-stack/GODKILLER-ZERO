# GODKILLER ZERO - Enterprise One-Liner PowerShell Installer
# Usage: irm https://raw.githubusercontent.com/taurus42119-stack/GODKILLER-ZERO/main/install.ps1 | iex

$ErrorActionPreference = "Stop"

Write-Host "[GODKILLER ZERO] Initializing Enterprise Setup..." -ForegroundColor Cyan

$InstallDir = "$env:LOCALAPPDATA\Programs\godkiller-zero"
$ExePath = "$InstallDir\godkiller-console.exe"
$GuiPath = "$InstallDir\GodkillerZero.exe"

if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

$CurrentExe = "$PSScriptRoot\godkiller-console.exe"
$CurrentGui = "$PSScriptRoot\GodkillerZero.exe"

if (Test-Path $CurrentExe) {
    Copy-Item $CurrentExe -Destination $ExePath -Force
}
if (Test-Path $CurrentGui) {
    Copy-Item $CurrentGui -Destination $GuiPath -Force
}

if (-not (Test-Path $ExePath)) {
    Write-Host "Downloading latest release binary from GitHub..." -ForegroundColor Yellow
    $DownloadUrl = "https://github.com/taurus42119-stack/GODKILLER-ZERO/releases/latest/download/godkiller-zero-windows-x86_64.zip"
    $ZipPath = "$env:TEMP\godkiller-zero.zip"
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $ZipPath -UseBasicParsing

    $ChecksumUrl = "https://github.com/taurus42119-stack/GODKILLER-ZERO/releases/latest/download/SHA256SUMS.txt"
    try {
        $ExpectedContent = (Invoke-WebRequest -Uri $ChecksumUrl -UseBasicParsing).Content.Trim()
        $ExpectedHash = ($ExpectedContent -split '\s+')[0].Trim().ToUpper()
        $ActualHash = (Get-FileHash $ZipPath -Algorithm SHA256).Hash.ToUpper()
        if ($ActualHash -ne $ExpectedHash) {
            Remove-Item $ZipPath -Force
            throw "SHA256 mismatch — download may be corrupted or tampered. Expected: $ExpectedHash, Got: $ActualHash"
        }
        Write-Host "[OK] SHA256 verified: $ActualHash" -ForegroundColor Green
    } catch [System.Management.Automation.RuntimeException] {
        throw $_
    } catch {
        Write-Warning "Checksum file unavailable on remote release. Proceeding with caution: $_"
    }

    Expand-Archive -Path $ZipPath -DestinationPath $InstallDir -Force
    Remove-Item $ZipPath -Force
}


# Add to User PATH if not present
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
    Write-Host "[OK] Added $InstallDir to User PATH" -ForegroundColor Green
}

# Hook Antigravity with default Balanced (KEN) discipline
if (Test-Path $ExePath) {
    Write-Host "Hooking Google Antigravity with Zero-Vibe Invariants..." -ForegroundColor Cyan
    & "$ExePath" --hook
}

# Launch compact desktop card
Write-Host "Launching GODKILLER ZERO Desktop Controller..." -ForegroundColor Green
if (Test-Path $GuiPath) {
    Start-Process -FilePath "$GuiPath"
} elseif (Test-Path $ExePath) {
    Start-Process -FilePath "$ExePath"
}

Write-Host "[OK] GODKILLER ZERO is active and protecting Antigravity IDE & CLI." -ForegroundColor Green
