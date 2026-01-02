# Запуск Mullvad Browser в безопасном режиме
# Отключает все расширения и использует чистый профиль

Write-Host "=== Запуск Mullvad Browser в безопасном режиме ===" -ForegroundColor Green

# Ищем Mullvad Browser
$mullvadPaths = @(
    "C:\Program Files\Mullvad Browser\Browser\firefox.exe",
    "C:\Program Files (x86)\Mullvad Browser\Browser\firefox.exe",
    "${env:LOCALAPPDATA}\Programs\Mullvad Browser\Browser\firefox.exe",
    "${env:PROGRAMFILES}\Mullvad Browser\Browser\firefox.exe"
)

$mullvadPath = $null
foreach ($path in $mullvadPaths) {
    if (Test-Path $path) {
        $mullvadPath = $path
        break
    }
}

if (!$mullvadPath) {
    Write-Host "Mullvad Browser не найден!" -ForegroundColor Red
    Write-Host "Установите его с: https://mullvad.net/en/browser" -ForegroundColor Yellow
    exit 1
}

Write-Host "Найден браузер: $mullvadPath" -ForegroundColor Cyan

# Параметры безопасного режима
$safeArgs = @(
    "--safe-mode",
    "--new-instance",
    "--profile", "${env:TEMP}\MullvadSafeProfile",
    "https://www.google.com"
)

Write-Host "Запуск в безопасном режиме..." -ForegroundColor Yellow

try {
    Start-Process -FilePath $mullvadPath -ArgumentList $safeArgs
    Write-Host "✓ Mullvad Browser запущен в безопасном режиме!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Если страницы загружаются - проблема в расширениях или профиле." -ForegroundColor Cyan
    Write-Host "Если нет - проблема в сетевых настройках или антивирусе." -ForegroundColor Cyan
} catch {
    Write-Host "✗ Ошибка запуска: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host ""
Write-Host "=== ДОПОЛНИТЕЛЬНЫЕ ВАРИАНТЫ ===" -ForegroundColor Yellow
Write-Host ""
Write-Host "1. Полная очистка профиля:" -ForegroundColor White
Write-Host '   Remove-Item "$env:APPDATA\Mullvad Browser" -Recurse -Force' -ForegroundColor Gray
Write-Host ""
Write-Host "2. Временное отключение Windows Defender:" -ForegroundColor White
Write-Host '   Set-MpPreference -DisableRealtimeMonitoring $true' -ForegroundColor Gray
Write-Host ""
Write-Host "3. Сброс сетевых настроек:" -ForegroundColor White
Write-Host '   netsh winsock reset && netsh int ip reset && ipconfig /flushdns' -ForegroundColor Gray



































