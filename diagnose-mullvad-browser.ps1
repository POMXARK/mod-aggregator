# Диагностика проблем Mullvad Browser
# Запуск: .\diagnose-mullvad-browser.ps1

Write-Host "=== Диагностика Mullvad Browser ===" -ForegroundColor Green
Write-Host ""

# 1. Проверяем наличие Mullvad Browser
Write-Host "1. Поиск Mullvad Browser..." -ForegroundColor Yellow
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
        Write-Host "Найден: $path" -ForegroundColor Green
        break
    }
}

if (!$mullvadPath) {
    Write-Host "Mullvad Browser не найден!" -ForegroundColor Red
    Write-Host ""
    Write-Host "Скачайте и установите Mullvad Browser:" -ForegroundColor Cyan
    Write-Host "https://mullvad.net/en/browser" -ForegroundColor White
    exit 1
}

# 2. Проверяем интернет подключение
Write-Host ""
Write-Host "2. Проверка интернет подключения..." -ForegroundColor Yellow
try {
    $response = Invoke-WebRequest -Uri "https://www.google.com" -Method Head -TimeoutSec 10
    Write-Host "Интернет работает (HTTP $($response.StatusCode))" -ForegroundColor Green
} catch {
    Write-Host "Проблемы с интернетом: $($_.Exception.Message)" -ForegroundColor Red
}

# 3. Проверяем DNS
Write-Host ""
Write-Host "3. Проверка DNS..." -ForegroundColor Yellow
try {
    $dnsResult = Resolve-DnsName "google.com" -Server "8.8.8.8" -ErrorAction Stop
    Write-Host "DNS работает: $($dnsResult.IPAddress)" -ForegroundColor Green
} catch {
    Write-Host "DNS проблемы: $($_.Exception.Message)" -ForegroundColor Red
}

# 4. Проверяем прокси настройки
Write-Host ""
Write-Host "4. Проверка прокси настроек..." -ForegroundColor Yellow
try {
    $proxyEnabled = (Get-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Internet Settings" -Name ProxyEnable -ErrorAction Stop).ProxyEnable
    if ($proxyEnabled -eq 1) {
        $proxyServer = (Get-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Internet Settings" -Name ProxyServer -ErrorAction Stop).ProxyServer
        Write-Host "Системный прокси включен: $proxyServer" -ForegroundColor Yellow
        Write-Host "  Это может конфликтовать с Mullvad Browser" -ForegroundColor Yellow
    } else {
        Write-Host "Системный прокси отключен" -ForegroundColor Green
    }
} catch {
    Write-Host "Не удалось проверить прокси настройки" -ForegroundColor Gray
}

# 5. Проверяем антивирус
Write-Host ""
Write-Host "5. Проверка антивируса..." -ForegroundColor Yellow
$avServices = Get-Service | Where-Object { $_.DisplayName -like "*antivirus*" -or $_.DisplayName -like "*defender*" -or $_.DisplayName -like "*security*" } | Select-Object DisplayName, Status
if ($avServices) {
    Write-Host "Обнаружены службы безопасности:" -ForegroundColor Cyan
    $avServices | ForEach-Object {
        $statusColor = if ($_.Status -eq "Running") { "Yellow" } else { "Green" }
        Write-Host "  $($_.DisplayName): $($_.Status)" -ForegroundColor $statusColor
    }
} else {
    Write-Host "Антивирусные службы не найдены" -ForegroundColor Green
}

# 6. Рекомендации
Write-Host ""
Write-Host "=== РЕКОМЕНДАЦИИ ПО ИСПРАВЛЕНИЮ ===" -ForegroundColor Green
Write-Host ""

Write-Host "Вариант 1: Очистка профиля браузера" -ForegroundColor White
Write-Host "  1. Закройте Mullvad Browser" -ForegroundColor Gray
Write-Host "  2. Удалите папку: %APPDATA%\Mullvad Browser" -ForegroundColor Gray
Write-Host "  3. Запустите браузер заново" -ForegroundColor Gray
Write-Host ""

Write-Host "Вариант 2: Отключение прокси в системе" -ForegroundColor White
Write-Host "  1. Win + R -> inetcpl.cpl -> вкладка Подключения" -ForegroundColor Gray
Write-Host "  2. Снимите галочку 'Использовать прокси-сервер'" -ForegroundColor Gray
Write-Host "  3. Перезагрузите браузер" -ForegroundColor Gray
Write-Host ""

Write-Host "Вариант 3: Временное отключение антивируса" -ForegroundColor White
Write-Host "  1. Отключите Windows Defender или другой АВ" -ForegroundColor Gray
Write-Host "  2. Проверьте работу браузера" -ForegroundColor Gray
Write-Host "  3. Включите АВ обратно" -ForegroundColor Gray
Write-Host ""

Write-Host "Вариант 4: Запуск в безопасном режиме" -ForegroundColor White
Write-Host "  .\launch-mullvad-safe.ps1" -ForegroundColor Gray
Write-Host ""

Write-Host "Вариант 5: Переустановка браузера" -ForegroundColor White
Write-Host "  1. Удалите Mullvad Browser" -ForegroundColor Gray
Write-Host "  2. Скачайте свежую версию: https://mullvad.net/en/browser" -ForegroundColor Gray
Write-Host "  3. Установите заново" -ForegroundColor Gray

Write-Host ""
Write-Host "Запустить диагностику в браузере? (y/n): " -ForegroundColor Cyan -NoNewline
$response = Read-Host

if ($response -eq 'y' -or $response -eq 'Y') {
    Write-Host ""
    Write-Host "Запуск Mullvad Browser для тестирования..." -ForegroundColor Yellow
    try {
        Start-Process -FilePath $mullvadPath -ArgumentList "--new-window", "https://www.google.com", "--safe-mode"
        Write-Host "Браузер запущен в безопасном режиме" -ForegroundColor Green
    } catch {
        Write-Host "Ошибка запуска: $($_.Exception.Message)" -ForegroundColor Red
    }
}

