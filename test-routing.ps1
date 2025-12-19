# Проверка маршрутизации VPN в Windows
# Запускать в PowerShell

Write-Host "=== Проверка маршрутизации VPN ==="

# Получить текущие маршруты
Write-Host "Текущие маршруты:"
route print | Select-String "0.0.0.0"

Write-Host ""
Write-Host "WSL интерфейсы:"
wsl -d Ubuntu -- ip addr show | Select-String "inet"

Write-Host ""
Write-Host "Тест подключения:"
try {
    $response = Invoke-WebRequest -Uri "https://ifconfig.me" -TimeoutSec 10
    Write-Host "Текущий IP: $($response.Content)"
} catch {
    Write-Host "❌ Ошибка подключения: $($_.Exception.Message)"
}

Write-Host ""
Write-Host "Если IP не из ErkaPharm, проверь:"
Write-Host "1. VPN подключен в WSL (ip addr show)"
Write-Host "2. Маршрутизация настроена (route print)"
Write-Host "3. Перезапусти setup-routing.ps1"
