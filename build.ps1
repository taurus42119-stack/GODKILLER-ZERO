# GODKILLER ZERO v1.0.0-GA - Build from Source + Auto Package
# Author  : taurus42119 (@kayvins.th)
# Repo    : https://github.com/taurus42119-stack/GODKILLER-ZERO
# Usage   : .\build.ps1
# Requirements: Rust (rustup.rs) + .NET SDK 9+ (dotnet.microsoft.com)

$ErrorActionPreference = "Stop"

function Sync-BetaDistributionFolder {
    $betaRoot = Join-Path $PSScriptRoot "GK0-BETA"
    if (-not (Test-Path -LiteralPath $betaRoot)) {
        Write-Host "[SKIP] No GK0-BETA folder to sync." -ForegroundColor DarkGray
        return
    }

    Write-Host "[4/4] Syncing GK0-BETA to match this repo..." -ForegroundColor Cyan

    $excludeDirs = @(
        "GK0-BETA",
        "GODKILLER ZERO (BETA)",
        "target",
        "publish",
        ".git",
        "bin",
        "obj",
        ".idea",
        ".vscode",
        ".gemini",
        ".agents",
        ".claude",
        ".codex",
        ".cursor"
    )

    $previousPreference = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    & robocopy $PSScriptRoot $betaRoot /E /NFL /NDL /NJH /NJS /NP /IS /IT `
        /XD $excludeDirs `
        /XF "_gst.txt" "_gste.txt" "_st.txt" "_ste.txt"
    $robo = $LASTEXITCODE
    $ErrorActionPreference = $previousPreference
    if ($robo -ge 8) {
        throw "robocopy failed while syncing BETA (exit $robo)"
    }

    $betaPublish = Join-Path $betaRoot "publish"
    New-Item -ItemType Directory -Path $betaPublish -Force | Out-Null
    Copy-Item (Join-Path $PSScriptRoot "publish\*") -Destination $betaPublish -Force
    Copy-Item (Join-Path $PSScriptRoot $ZipName) -Destination $betaRoot -Force
    Copy-Item (Join-Path $PSScriptRoot "publish\SHA256SUMS.txt") -Destination (Join-Path $betaRoot "SHA256SUMS.txt") -Force

    Write-Host "[OK] BETA folder matches source + publish binaries" -ForegroundColor Green
}

$Version    = "1.0.0"
$RepoOwner  = "taurus42119-stack"
$RepoName   = "GODKILLER-ZERO"
$ZipName    = "godkiller-zero-windows-x86_64.zip"

Write-Host ""
Write-Host "GODKILLER ZERO v$Version-GA - Build from Source" -ForegroundColor Cyan
Write-Host "--------------------------------------------------------" -ForegroundColor DarkGray
Write-Host ""

$running = Get-Process -Name "GodkillerZero", "godkiller-console" -ErrorAction SilentlyContinue
if ($running) {
    Write-Host "[WARN] GodkillerZero or godkiller-console is currently running." -ForegroundColor Yellow
    Write-Host "       Please close them before running build.ps1 to avoid file lock errors." -ForegroundColor Yellow
    Write-Host ""
}

# -- Preflight: Rust ----------------------------------------------------------
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "[FAIL] Rust / Cargo not found." -ForegroundColor Red
    Write-Host "   Install from: https://rustup.rs" -ForegroundColor Yellow
    exit 1
}
Write-Host "[OK] Rust: $(cargo --version)" -ForegroundColor Green

# -- Preflight: .NET SDK ------------------------------------------------------
if (-not (Get-Command dotnet -ErrorAction SilentlyContinue)) {
    Write-Host "[FAIL] .NET SDK not found." -ForegroundColor Red
    Write-Host "   Install from: https://dotnet.microsoft.com/download" -ForegroundColor Yellow
    exit 1
}
Write-Host "[OK] .NET:  $(dotnet --version)" -ForegroundColor Green
Write-Host ""

# Always write Rust output into this repo's target\, not a sandbox cache.
$env:CARGO_TARGET_DIR = Join-Path $PSScriptRoot "target"

# -- Step 1: Rust Core Engine -------------------------------------------------
Write-Host "[1/3] Building Rust core engine (release)..." -ForegroundColor Cyan
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "[FAIL] Rust build failed." -ForegroundColor Red
    exit 1
}
$ConsoleExe = Join-Path $env:CARGO_TARGET_DIR "release\godkiller-console.exe"
Write-Host "[OK] Core engine built -> $ConsoleExe" -ForegroundColor Green
Write-Host ""

# -- Step 2: C# GUI -----------------------------------------------------------
Write-Host "[2/3] Publishing C# GUI (self-contained, win-x64)..." -ForegroundColor Cyan
dotnet publish gui\GodkillerZero.csproj `
    -c Release `
    -r win-x64 `
    --self-contained true `
    -p:PublishSingleFile=true `
    -p:IncludeNativeLibrariesForSelfExtract=true `
    -o publish
if ($LASTEXITCODE -ne 0) {
    Write-Host "[FAIL] C# GUI build failed." -ForegroundColor Red
    exit 1
}
Write-Host "[OK] GUI published -> publish\GodkillerZero.exe" -ForegroundColor Green

# Copy core engine into publish
Copy-Item $ConsoleExe -Destination "publish" -Force
Write-Host "[OK] Copied godkiller-console.exe -> publish" -ForegroundColor Green

if (Test-Path "assets\app.ico") { Copy-Item "assets\app.ico" -Destination "publish" -Force }
if (Test-Path "assets\app.png") { Copy-Item "assets\app.png" -Destination "publish" -Force }

Get-ChildItem -Path "publish" -Filter "*.pdb" -Recurse | Remove-Item -Force

$GuiHash = (Get-FileHash "publish\GodkillerZero.exe" -Algorithm SHA256).Hash.ToLower()
$CliHash = (Get-FileHash "publish\godkiller-console.exe" -Algorithm SHA256).Hash.ToLower()
$ChecksumBody = "$GuiHash  GodkillerZero.exe`r`n$CliHash  godkiller-console.exe`r`n"
Set-Content -Path "publish\SHA256SUMS.txt" -Value $ChecksumBody -Encoding ascii -NoNewline
Write-Host "[OK] Wrote publish\SHA256SUMS.txt" -ForegroundColor Green
Write-Host ""

# -- Step 3: Auto-Zip for GitHub Release --------------------------------------
Write-Host "[3/4] Packaging release zip..." -ForegroundColor Cyan
$ZipPath = ".\$ZipName"
if (Test-Path $ZipPath) { Remove-Item $ZipPath -Force }
Compress-Archive -Path "publish\*" -DestinationPath $ZipPath -Force
Write-Host "[OK] Zip created -> $ZipName" -ForegroundColor Green
Write-Host ""

# -- Step 4: Keep the BETA distribution folder identical ---------------------
Sync-BetaDistributionFolder
Write-Host ""

# -- Done ---------------------------------------------------------------------
Write-Host "--------------------------------------------------------" -ForegroundColor DarkGray
Write-Host "[OK] Build complete!" -ForegroundColor Green
Write-Host ""
Write-Host "  Run now :  .\publish\GodkillerZero.exe" -ForegroundColor White
Write-Host "  CLI test:  .\publish\godkiller-console.exe --hook" -ForegroundColor White
Write-Host ""
Write-Host "--------------------------------------------------------" -ForegroundColor DarkGray
Write-Host "NEXT STEP - Upload to GitHub Releases:" -ForegroundColor Yellow
Write-Host ""
Write-Host "  1. git add . && git commit -m 'Release v$Version'" -ForegroundColor White
Write-Host "  2. git tag v$Version && git push origin main --tags" -ForegroundColor White
Write-Host "  3. Go to: https://github.com/$RepoOwner/$RepoName/releases/new" -ForegroundColor Cyan
Write-Host "  4. Tag: v$Version | Title: GODKILLER ZERO v$Version-GA" -ForegroundColor White
Write-Host "  5. Upload: $ZipName  (file is ready in this folder)" -ForegroundColor White
Write-Host "  6. Publish Release [OK]" -ForegroundColor Green
Write-Host ""
Write-Host "  One-liner install will then work:" -ForegroundColor DarkGray
Write-Host "  irm https://raw.githubusercontent.com/$RepoOwner/$RepoName/main/install.ps1 | iex" -ForegroundColor DarkGray
Write-Host ""
