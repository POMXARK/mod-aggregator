# Тест API в браузере через PowerShell
Write-Host "=== Тест API ErkaPharm в браузере ===" -ForegroundColor Green

Write-Host "Проверяем маршруты:" -ForegroundColor Yellow
$route = route print | Select-String "5.172.178.51"
if ($route) {
    Write-Host "  ✓ Маршрут для API настроен" -ForegroundColor Green
} else {
    Write-Host "  ❌ Маршрут для API НЕ настроен" -ForegroundColor Red
}

Write-Host ""
Write-Host "Проверяем VPN:" -ForegroundColor Yellow
try {
    $vpn_check = wsl -d Ubuntu -- bash -c "ip addr show tun0 >/dev/null 2>&1 && echo 'works' || echo 'failed'"
    if ($vpn_check -eq "works") {
        Write-Host "  ✓ VPN работает" -ForegroundColor Green
    } else {
        Write-Host "  ❌ VPN не работает" -ForegroundColor Red
    }
} catch {
    Write-Host "  ❌ Ошибка проверки VPN" -ForegroundColor Red
}

Write-Host ""
Write-Host "Тест API через PowerShell:" -ForegroundColor Yellow
try {
    $response = Invoke-WebRequest -Uri "https://api.erkapharm.com" -TimeoutSec 10 -SkipCertificateCheck
    Write-Host "  ✓ API доступен через PowerShell" -ForegroundColor Green
    Write-Host "  Статус: $($response.StatusCode)" -ForegroundColor Cyan
} catch {
    Write-Host "  ❌ API недоступен через PowerShell" -ForegroundColor Red
    Write-Host "  Ошибка: $($_.Exception.Message)" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "=== ИНСТРУКЦИИ ДЛЯ БРАУЗЕРА ===" -ForegroundColor Green
Write-Host "1. Откройте браузер" -ForegroundColor Cyan
Write-Host "2. Очистите кэш: Ctrl+Shift+R или Ctrl+F5" -ForegroundColor Cyan
Write-Host "3. Перейдите: https://api.erkapharm.com" -ForegroundColor Cyan
Write-Host "4. Должно загрузиться через VPN!" -ForegroundColor Green

Write-Host ""
Write-Host "Если не работает:" -ForegroundColor Yellow
Write-Host "- Проверьте что VPN подключен в WSL" -ForegroundColor Cyan
Write-Host "- Очистите кэш браузера полностью" -ForegroundColor Cyan
Write-Host "- Попробуйте в режиме инкогнито" -ForegroundColor Cyan
Write-Host "- Проверьте firewall Windows" -ForegroundColor Cyan
