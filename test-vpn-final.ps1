# Финальная проверка VPN в Windows
Write-Host "=== Финальная проверка ErkaPharm VPN ===" -ForegroundColor Green

# Проверяем маршруты
Write-Host "1. Проверяем маршруты:" -ForegroundColor Yellow
route print | Select-String "0.0.0.0" | ForEach-Object { Write-Host "  $_" }

Write-Host ""
Write-Host "2. Тестируем подключение:" -ForegroundColor Yellow
try {
    $response = Invoke-WebRequest -Uri "http://ifconfig.me/ip" -TimeoutSec 10
    $ip = $response.Content.Trim()
    Write-Host "  Текущий IP: $ip" -ForegroundColor Cyan

    # Проверяем, является ли IP из диапазона ErkaPharm (примерная проверка)
    if ($ip -match "^(10\.|172\.|192\.168\.)") {
        Write-Host "  ✓ Вероятно VPN работает (внутренний IP)" -ForegroundColor Green
    } else {
        Write-Host "  ⚠️  Возможно VPN не работает (внешний IP)" -ForegroundColor Yellow
    }
} catch {
    Write-Host "  ❌ Ошибка подключения: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host ""
Write-Host "3. Проверка DNS:" -ForegroundColor Yellow
try {
    $dnsResult = Resolve-DnsName "vpn.erkapharm.com" -ErrorAction Stop
    Write-Host "  ✓ DNS работает" -ForegroundColor Green
} catch {
    Write-Host "  ❌ DNS не работает" -ForegroundColor Red
}

Write-Host ""
Write-Host "=== Готово! ===" -ForegroundColor Green
Write-Host ""
Write-Host "Если IP показывает внутренний адрес ErkaPharm - VPN работает!" -ForegroundColor Cyan
Write-Host "Если внешний IP - проверь подключение в WSL" -ForegroundColor Yellow
