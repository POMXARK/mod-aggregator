# Тест API через PowerShell с правильными настройками
Write-Host "=== Тест API через PowerShell ===" -ForegroundColor Green

$url = "https://localhost:9443"

try {
    # Игнорируем SSL проверки
    [System.Net.ServicePointManager]::ServerCertificateValidationCallback = {$true}

    # Создаем запрос с правильным Host header
    $request = [System.Net.WebRequest]::Create($url)
    $request.Method = "GET"
    $request.Timeout = 10000
    $request.Host = "api.erkapharm.com"

    # Добавляем User-Agent как в браузере
    $request.UserAgent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"

    $response = $request.GetResponse()
    $reader = New-Object System.IO.StreamReader($response.GetResponseStream())
    $content = $reader.ReadToEnd()
    $reader.Close()
    $response.Close()

    Write-Host "✅ API доступен!" -ForegroundColor Green
    Write-Host "Статус: $($response.StatusCode)" -ForegroundColor Cyan
    Write-Host "Content-Type: $($response.ContentType)" -ForegroundColor Cyan

    # Проверяем что это Swagger UI
    if ($content -like "*swagger*") {
        Write-Host "✓ Получен Swagger UI - API работает!" -ForegroundColor Green
    } elseif ($content -like "*API*") {
        Write-Host "✓ Получен API ответ" -ForegroundColor Green
    } else {
        Write-Host "⚠️  Получен ответ, но не распознан" -ForegroundColor Yellow
    }

} catch {
    Write-Host "❌ Ошибка доступа к API" -ForegroundColor Red
    Write-Host "Ошибка: $($_.Exception.Message)" -ForegroundColor Yellow

    # Детальная диагностика
    if ($_.Exception.Message -like "*SSL*" -or $_.Exception.Message -like "*certificate*") {
        Write-Host ""
        Write-Host "🔍 Причина: SSL сертификат" -ForegroundColor Yellow
        Write-Host "Решение: Сертификат api.erkapharm.com не доверен для localhost" -ForegroundColor Cyan
    } elseif ($_.Exception.Message -like "*403*") {
        Write-Host ""
        Write-Host "🔍 Причина: 403 Forbidden" -ForegroundColor Yellow
        Write-Host "Решение: API блокирует запросы без правильного Host header" -ForegroundColor Cyan
    } elseif ($_.Exception.Message -like "*connection*") {
        Write-Host ""
        Write-Host "🔍 Причина: Сетевая ошибка" -ForegroundColor Yellow
        Write-Host "Решение: Проверьте что SSH туннель работает (порт 9443)" -ForegroundColor Cyan
    }
}

Write-Host ""
Write-Host "=== СРАВНЕНИЕ ===" -ForegroundColor Green
Write-Host "Из WSL (работает): curl -k -H 'Host: api.erkapharm.com' https://localhost:9443" -ForegroundColor Cyan
Write-Host "Из PowerShell: Требует игнорирования SSL и правильного Host header" -ForegroundColor Cyan

Write-Host ""
Write-Host "Для браузера: Добавьте api.erkapharm.com -> 127.0.0.1 в hosts" -ForegroundColor Green
























