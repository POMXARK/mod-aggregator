# Полная диагностика VPN проблемы
Write-Host "=== Диагностика VPN проблемы ===" -ForegroundColor Red

# 1. Проверяем WSL статус
Write-Host "1. Проверяем WSL:" -ForegroundColor Yellow
$wsl_running = wsl -l -v | Select-String "Ubuntu" | Select-String "Running"
if ($wsl_running) {
    Write-Host "  ✓ WSL Ubuntu запущен" -ForegroundColor Green
} else {
    Write-Host "  ❌ WSL Ubuntu не запущен" -ForegroundColor Red
}

# 2. Проверяем VPN процесс в WSL
Write-Host ""
Write-Host "2. Проверяем VPN в WSL:" -ForegroundColor Yellow
try {
    $vpc_process = wsl -d Ubuntu -- ps aux | Select-String "openconnect" | Where-Object { $_ -notmatch "grep" }
    if ($vpc_process) {
        Write-Host "  ✓ OpenConnect запущен" -ForegroundColor Green
        Write-Host "  Детали: $($vpc_process)" -ForegroundColor Cyan
    } else {
        Write-Host "  ❌ OpenConnect не запущен" -ForegroundColor Red
    }
}
catch {
    Write-Host "  ❌ Ошибка проверки WSL" -ForegroundColor Red
}

# 3. Проверяем TUN интерфейс в WSL
Write-Host ""
Write-Host "3. Проверяем TUN интерфейс в WSL:" -ForegroundColor Yellow
try {
    $tun_interface = wsl -d Ubuntu -- ip addr show | Select-String "tun"
    if ($tun_interface) {
        Write-Host "  ✓ TUN интерфейс найден" -ForegroundColor Green
        Write-Host "  Детали: $($tun_interface)" -ForegroundColor Cyan
    } else {
        Write-Host "  ❌ TUN интерфейс не найден" -ForegroundColor Red
    }
}
catch {
    Write-Host "  ❌ Ошибка проверки TUN" -ForegroundColor Red
}

# 4. Проверяем маршруты в WSL
Write-Host ""
Write-Host "4. Маршруты в WSL:" -ForegroundColor Yellow
try {
    $wsl_routes = wsl -d Ubuntu -- ip route show
    $wsl_routes | ForEach-Object { Write-Host "  $_" -ForegroundColor Cyan }
}
catch {
    Write-Host "  ❌ Ошибка получения маршрутов WSL" -ForegroundColor Red
}

# 5. Проверяем маршруты в Windows
Write-Host ""
Write-Host "5. Маршруты в Windows:" -ForegroundColor Yellow
$win_routes = route print | Select-String "0.0.0.0"
if ($win_routes) {
    Write-Host "  ✓ Маршрут 0.0.0.0 найден" -ForegroundColor Green
    $win_routes | ForEach-Object { Write-Host "  $_" -ForegroundColor Cyan }
} else {
    Write-Host "  ❌ Маршрут 0.0.0.0 не найден" -ForegroundColor Red
}

# 6. Тест IP
Write-Host ""
Write-Host "6. Тест текущего IP:" -ForegroundColor Yellow
try {
    $response = Invoke-WebRequest -Uri "http://ifconfig.me/ip" -TimeoutSec 10 -ErrorAction Stop
    $ip = $response.Content.Trim()
    Write-Host "  Текущий IP: $ip" -ForegroundColor Cyan
    if ($ip -match "^(10\.|172\.|192\.168\.)") {
        Write-Host "  ✓ IP внутренний - VPN работает!" -ForegroundColor Green
    } else {
        Write-Host "  ❌ IP внешний - VPN не работает" -ForegroundColor Red
    }
}
catch {
    Write-Host "  ❌ Ошибка получения IP: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host ""
Write-Host "=== РЕШЕНИЯ ===" -ForegroundColor Green

if (-not $vpc_process) {
    Write-Host "Решение 1: Запусти VPN" -ForegroundColor Yellow
    Write-Host "  wsl -d Ubuntu" -ForegroundColor Cyan
    Write-Host "  cd /mnt/c/Users/User/mod-aggregator" -ForegroundColor Cyan
    Write-Host "  ./auto-connect.sh" -ForegroundColor Cyan
}

if (-not $tun_interface) {
    Write-Host "Решение 2: Проверь TUN устройство" -ForegroundColor Yellow
    Write-Host "  В WSL: ./check-tun.sh" -ForegroundColor Cyan
}

if (-not $win_routes) {
    Write-Host "Решение 3: Настрой маршрутизацию" -ForegroundColor Yellow
    Write-Host "  .\setup-routing.ps1" -ForegroundColor Cyan
}

Write-Host ""
Write-Host "Запусти этот скрипт после исправлений!" -ForegroundColor Green
