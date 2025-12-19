# Тест прокси сервера
Write-Host "=== Тест API Proxy Server ===" -ForegroundColor Green

$url = "http://localhost:8080"

try {
    Write-Host "Тестируем подключение к прокси..." -ForegroundColor Cyan

    $request = [System.Net.WebRequest]::Create($url)
    $request.Method = "GET"
    $request.Timeout = 5000

    $response = $request.GetResponse()
    $reader = New-Object System.IO.StreamReader($response.GetResponseStream())
    $content = $reader.ReadToEnd()
    $reader.Close()
    $response.Close()

    Write-Host "✅ Прокси работает!" -ForegroundColor Green
    Write-Host "Статус: $($response.StatusCode)" -ForegroundColor Cyan

    if ($content -like "*swagger*") {
        Write-Host "✓ Swagger UI получен через прокси!" -ForegroundColor Green
    } elseif ($content -like "*API*") {
        Write-Host "✓ API ответ получен" -ForegroundColor Green
    } else {
        Write-Host "⚠️ Получен ответ, но формат не распознан" -ForegroundColor Yellow
    }

} catch {
    Write-Host "❌ Ошибка подключения к прокси" -ForegroundColor Red
    Write-Host "Ошибка: $($_.Exception.Message)" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Возможные причины:" -ForegroundColor Cyan
    Write-Host "1. Прокси сервер не запущен" -ForegroundColor White
    Write-Host "2. SSH туннель не работает" -ForegroundColor White
    Write-Host "3. Порт 8080 занят другим процессом" -ForegroundColor White
    Write-Host ""
    Write-Host "Запустите: .\start-api-proxy.bat" -ForegroundColor Green
}

Write-Host ""
Write-Host "Для браузера откройте: http://localhost:8080" -ForegroundColor Green

