@echo off
REM Quick start script for Windows

echo Starting Monitoring System...
echo.

REM Check if Rust is installed
where cargo >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo Error: Rust is not installed. Please install from https://rustup.rs/
    exit /b 1
)

echo ✓ Rust installation found
echo.

REM Build all components
echo Building all components...
cargo build --release --all

if %ERRORLEVEL% NEQ 0 (
    echo Error: Build failed
    exit /b 1
)

echo.
echo ✓ Build complete!
echo.

REM Set up environment
set JWT_SECRET=dev-secret-1234
set MONITORING_AUTH_TOKEN=dev-token-1234

echo Generated credentials:
echo   JWT_SECRET=%JWT_SECRET%
echo   MONITORING_AUTH_TOKEN=%MONITORING_AUTH_TOKEN%
echo.

REM Create log directory
if not exist logs mkdir logs

REM Start collector
echo Starting collector server on ws://localhost:8080...
start "Monitoring Collector" /MIN cmd /c "set JWT_SECRET=%JWT_SECRET% && target\release\monitoring-collector.exe --config config\collector.toml > logs\collector.log 2>&1"

timeout /t 3 /nobreak >nul

REM Start agent
echo Starting monitoring agent...
start "Monitoring Agent" /MIN cmd /c "set MONITORING_AUTH_TOKEN=%MONITORING_AUTH_TOKEN% && target\release\monitoring-agent.exe --config config\agent.toml > logs\agent.log 2>&1"

timeout /t 2 /nobreak >nul

echo.
echo ============================================
echo   Monitoring System is Running!
echo ============================================
echo.
echo Collector:  ws://localhost:8080/ingest
echo Health:     http://localhost:8080/health
echo.
echo Logs:
echo   Collector:  logs\collector.log
echo   Agent:      logs\agent.log
echo.
echo Press any key to view logs...
pause >nul

type logs\collector.log
type logs\agent.log
