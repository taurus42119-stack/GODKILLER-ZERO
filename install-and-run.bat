@echo off
title GODKILLER ZERO - 1-Click Local AI Launcher
cd /d "%~dp0"

echo ===================================================
echo   GODKILLER ZERO: 1-Click Local AI Launcher
echo ===================================================
echo.

powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0setup_local_llm.ps1"

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo Launching GodkillerZeroGui directly...
    start "" "%~dp0GodkillerZeroGui.exe"
)
