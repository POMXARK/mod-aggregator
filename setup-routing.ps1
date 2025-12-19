# PowerShell скрипт для настройки маршрутизации VPN в Windows
# Запускать от имени администратора

Write-Host "=== Настройка маршрутизации VPN в Windows ==="

# Получаем IP WSL
$wsl_ip = wsl -d Ubuntu -- hostname -I | ForEach-Object { $_.Trim() -split ' ' | Select-Object -First 1 }
Write-Host "IP WSL: $wsl_ip"

# Получаем шлюз WSL
$wsl_gateway = wsl -d Ubuntu -- ip route | Select-String "default" | ForEach-Object { ($_ -split ' ')[2] }
Write-Host "Шлюз WSL: $wsl_gateway"

# Добавляем маршрут для всех IP через WSL VPN
# Это перенаправит весь трафик через VPN
Write-Host "Добавляем маршрут для всего трафика через VPN..."
route add 0.0.0.0/0 $wsl_ip metric 1

Write-Host "✓ Маршрутизация настроена"
Write-Host "Теперь весь трафик идет через VPN"
Write-Host ""
Write-Host "Для отмены маршрутизации:"
Write-Host "route delete 0.0.0.0 mask 0.0.0.0 $wsl_ip"
