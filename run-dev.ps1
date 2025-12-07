# PowerShell script to run Tauri dev with verbose logging
$env:RUST_LOG = "debug"
Write-Host "Starting Tauri dev with verbose logging..." -ForegroundColor Green
Write-Host "RUST_LOG=$env:RUST_LOG" -ForegroundColor Yellow
npm run tauri:dev








