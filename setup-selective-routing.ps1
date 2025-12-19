# Настройка селективной маршрутизации для ErkaPharm разработки
# Только конкретные подсети, а не весь трафик

Write-Host "=== Настройка селективной маршрутизации ErkaPharm ===" -ForegroundColor Green

# Получаем IP WSL
$wsl_ip = wsl -d Ubuntu -- hostname -I | ForEach-Object { $_.Trim() -split ' ' | Select-Object -First 1 }
Write-Host "IP WSL: $wsl_ip" -ForegroundColor Cyan

# Определяем маршруты: network, mask
$routes = @(
    @("10.0.0.0", "255.0.0.0"),        # 10.0.0.0/8
    @("172.16.0.0", "255.240.0.0"),    # 172.16.0.0/12
    @("172.21.0.0", "255.255.0.0"),    # 172.21.0.0/16
    @("192.168.0.0", "255.255.0.0"),   # 192.168.0.0/16
    @("5.172.178.51", "255.255.255.255")  # api.erkapharm.com
)

Write-Host ""
Write-Host "Добавляем маршруты для ErkaPharm сетей:" -ForegroundColor Yellow

foreach ($route in $routes) {
    $network = $route[0]
    $mask = $route[1]

    Write-Host "Добавляем маршрут: $network/$mask via $wsl_ip" -ForegroundColor Cyan

    # Удаляем существующий маршрут (если есть)
    route delete $network mask $mask $wsl_ip 2>$null | Out-Null

    # Добавляем новый маршрут
    $result = route add $network mask $mask $wsl_ip
    if ($LASTEXITCODE -eq 0) {
        Write-Host "  ✓ Маршрут добавлен" -ForegroundColor Green
    } else {
        Write-Host "  ❌ Ошибка добавления маршрута" -ForegroundColor Red
    }
}

Write-Host ""
Write-Host "Текущие маршруты через VPN:" -ForegroundColor Yellow
route print | Select-String $wsl_ip | ForEach-Object { Write-Host "  $_" -ForegroundColor Cyan }

Write-Host ""
Write-Host "=== Готово! ===" -ForegroundColor Green
Write-Host "Теперь только трафик к ErkaPharm сетям идет через VPN" -ForegroundColor Cyan
Write-Host "Остальной интернет работает напрямую" -ForegroundColor Cyan
