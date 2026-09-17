# GODKILLER ZERO v1.0.0-GA - Build from Source + Auto Package
# Author  : taurus42119 (@kayvins.th)
# Repo    : https://github.com/taurus42119-stack/godkiller-zero
# Usage   : .\build.ps1
# Requirements: Rust (rustup.rs) + .NET SDK 9+ (dotnet.microsoft.com)

$ErrorActionPreference = "Stop"

$Version    = "1.0.0"
$RepoOwner  = "taurus42119-stack"
$RepoName   = "godkiller-zero"
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

# -- Step 1: Rust Core Engine -------------------------------------------------
Write-Host "[1/3] Building Rust core engine (release)..." -ForegroundColor Cyan
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "[FAIL] Rust build failed." -ForegroundColor Red
    exit 1
}
Write-Host "[OK] Core engine built -> target\release\godkiller-console.exe" -ForegroundColor Green
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
Copy-Item "target\release\godkiller-console.exe" -Destination "publish" -Force
Write-Host "[OK] Copied godkiller-console.exe -> publish" -ForegroundColor Green

if (Test-Path "assets\app.ico") { Copy-Item "assets\app.ico" -Destination "publish" -Force }
if (Test-Path "assets\app.png") { Copy-Item "assets\app.png" -Destination "publish" -Force }

# Clean up debug symbols (.pdb) for clean release package
Get-ChildItem -Path "publish" -Filter "*.pdb" -Recurse | Remove-Item -Force
Write-Host ""

# -- Step 3: Auto-Zip for GitHub Release --------------------------------------
Write-Host "[3/3] Packaging release zip..." -ForegroundColor Cyan
$ZipPath = ".\$ZipName"
if (Test-Path $ZipPath) { Remove-Item $ZipPath -Force }
Compress-Archive -Path "publish\*" -DestinationPath $ZipPath -Force
Write-Host "[OK] Zip created -> $ZipName" -ForegroundColor Green
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
