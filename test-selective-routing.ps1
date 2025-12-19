# Проверка селективной маршрутизации ErkaPharm
Write-Host "=== Проверка селективной маршрутизации ===" -ForegroundColor Green

# Получаем IP WSL
$wsl_ip = wsl -d Ubuntu -- hostname -I | ForEach-Object { $_.Trim() -split ' ' | Select-Object -First 1 }
Write-Host "IP WSL: $wsl_ip" -ForegroundColor Cyan

Write-Host ""
Write-Host "Проверяем маршруты ErkaPharm сетей:" -ForegroundColor Yellow

$networks = @(
    "10.0.0.0/255.0.0.0",
    "172.16.0.0/255.240.0.0",
    "172.21.0.0/255.255.0.0",
    "192.168.0.0/255.255.0.0",
    "5.172.178.51/255.255.255.255"
)

foreach ($net in $networks) {
    $parts = $net.Split('/')
    $network = $parts[0]
    $mask = $parts[1]

    $route_exists = route print | Select-String $network | Select-String $wsl_ip
    if ($route_exists) {
        if ($network -eq "5.172.178.51") {
            Write-Host "  ✓ api.erkapharm.com ($network) через VPN" -ForegroundColor Green
        } else {
            Write-Host "  ✓ $network через VPN" -ForegroundColor Green
        }
    } else {
        if ($network -eq "5.172.178.51") {
            Write-Host "  ❌ api.erkapharm.com ($network) НЕ через VPN" -ForegroundColor Red
        } else {
            Write-Host "  ❌ $network НЕ через VPN" -ForegroundColor Red
        }
    }
}

Write-Host ""
Write-Host "Тест подключения:" -ForegroundColor Yellow

# Тест обычного интернета (не должен идти через VPN)
try {
    $google_ip = (Resolve-DnsName "google.com" -ErrorAction Stop).IPAddress | Select-Object -First 1
    Write-Host "  Google IP: $google_ip (должен быть внешний)" -ForegroundColor Cyan
} catch {
    Write-Host "  ❌ DNS Google не работает" -ForegroundColor Red
}

# Тест ErkaPharm (должен идти через VPN)
try {
    $mnz_ip = (Resolve-DnsName "mnz-cosn.erkapharm.com" -ErrorAction Stop).IPAddress | Select-Object -First 1
    Write-Host "  ErkaPharm IP: $mnz_ip" -ForegroundColor Cyan
    if ($mnz_ip -match "^(10\.|172\.|192\.168\.)") {
        Write-Host "  ✓ ErkaPharm трафик идет через VPN!" -ForegroundColor Green
    } else {
        Write-Host "  ⚠️  ErkaPharm IP внешний - проверь маршруты" -ForegroundColor Yellow
    }
} catch {
    Write-Host "  ❌ DNS ErkaPharm не работает" -ForegroundColor Red
}

Write-Host ""
Write-Host "=== Результат ===" -ForegroundColor Green
Write-Host "Если зеленые ✓ - селективная маршрутизация работает!" -ForegroundColor Cyan
Write-Host "Обычный интернет работает напрямую, ErkaPharm через VPN." -ForegroundColor Cyan
