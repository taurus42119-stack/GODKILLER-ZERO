# GODKILLER ZERO: 1-Click Local AI Setup & Launcher
$ErrorActionPreference = "Continue"

Write-Host "===================================================" -ForegroundColor Cyan
Write-Host "   GODKILLER ZERO: 1-Click Local AI Setup" -ForegroundColor Green
Write-Host "===================================================" -ForegroundColor Cyan

# 1. Check if Ollama is installed
$ollamaCmd = Get-Command "ollama" -ErrorAction SilentlyContinue
$ollamaPath = $null

if ($ollamaCmd) {
    $ollamaPath = $ollamaCmd.Source
} else {
    $candidates = @(
        "$env:LOCALAPPDATA\Programs\Ollama\ollama.exe",
        "$env:ProgramFiles\Ollama\ollama.exe",
        "$env:LOCALAPPDATA\Ollama\ollama.exe"
    )
    foreach ($cand in $candidates) {
        if (Test-Path $cand) {
            $ollamaPath = $cand
            break
        }
    }
}

if (-not $ollamaPath) {
    Write-Host "[1/3] Ollama was not detected on this machine." -ForegroundColor Yellow
    Write-Host "      GODKILLER ZERO operates in 100% Offline Standalone Mode without it." -ForegroundColor Gray
    Write-Host "      Optional: Would you like to download and install Ollama now? (Y/N, default N): " -NoNewline -ForegroundColor Cyan
    $userConsent = Read-Host
    if ($userConsent -match '^[Yy]') {
        Write-Host "      Downloading official Ollama installer (https://ollama.com)..." -ForegroundColor Yellow
        $installerUrl = "https://ollama.com/download/OllamaSetup.exe"
        $tempInstaller = Join-Path $env:TEMP "OllamaSetup.exe"
        
        try {
            [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12 -bor [Net.SecurityProtocolType]::Tls13
            Invoke-WebRequest -Uri $installerUrl -OutFile $tempInstaller -UseBasicParsing
            Write-Host "      Launching Ollama installer window..." -ForegroundColor Green
            Start-Process -FilePath $tempInstaller -Wait
            Remove-Item $tempInstaller -Force -ErrorAction SilentlyContinue
            
            # Locate newly installed binary
            $ollamaPath = "$env:LOCALAPPDATA\Programs\Ollama\ollama.exe"
        } catch {
            Write-Host "Warning: Could not download or run Ollama installer: $_" -ForegroundColor Red
        }
    } else {
        Write-Host "      Skipping Ollama installation. Running in Standalone Mode." -ForegroundColor Green
    }
} else {
    Write-Host "[1/3] Ollama is installed: $ollamaPath" -ForegroundColor Green
}

# 2. Check if Ollama service is running on 11434
$serviceRunning = $false
try {
    $check = Test-NetConnection -ComputerName 127.0.0.1 -Port 11434 -InformationLevel Quiet -WarningAction SilentlyContinue
    if ($check) { $serviceRunning = $true }
} catch { }

if (-not $serviceRunning) {
    Write-Host "[2/3] Starting Ollama daemon in background..." -ForegroundColor Yellow
    if ($ollamaPath -and (Test-Path $ollamaPath)) {
        Start-Process -FilePath $ollamaPath -ArgumentList "serve" -WindowStyle Hidden
        Start-Sleep -Seconds 2
    }
} else {
    Write-Host "[2/3] Ollama service is running." -ForegroundColor Green
}

# 3. Check and Pull qwen2.5-coder:1.5b model
Write-Host "[3/3] Verifying Local AI model (qwen2.5-coder:1.5b)..." -ForegroundColor Yellow
$modelPresent = $false
try {
    $modelsJson = Invoke-RestMethod -Uri "http://127.0.0.1:11434/api/tags" -Method Get -TimeoutSec 5 -ErrorAction Stop
    foreach ($m in $modelsJson.models) {
        if ($m.name -like "*qwen2.5-coder:1.5b*") {
            $modelPresent = $true
            break
        }
    }
} catch { }

if (-not $modelPresent) {
    Write-Host "[3/3] Pulling qwen2.5-coder:1.5b (one-time download ~980MB)..." -ForegroundColor Cyan
    try {
        if ($ollamaPath) {
            & $ollamaPath pull qwen2.5-coder:1.5b
        } else {
            ollama pull qwen2.5-coder:1.5b
        }
        Write-Host "[3/3] Model qwen2.5-coder:1.5b ready!" -ForegroundColor Green
    } catch {
        Write-Host "Notice: Model pull will continue in background." -ForegroundColor Yellow
    }
} else {
    Write-Host "[3/3] Local AI Model qwen2.5-coder:1.5b is ready!" -ForegroundColor Green
}

# 4. Launch GUI Application
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$guiExe = if (Test-Path (Join-Path $scriptDir "GodkillerZero.exe")) { Join-Path $scriptDir "GodkillerZero.exe" } else { Join-Path $scriptDir "GodkillerZeroGui.exe" }

Write-Host "Launching GODKILLER ZERO..." -ForegroundColor Green
if (Test-Path $guiExe) {
    Start-Process -FilePath $guiExe
} else {
    Write-Host "GodkillerZero.exe ready at $guiExe" -ForegroundColor Yellow
}
