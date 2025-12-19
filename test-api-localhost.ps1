# Тест API через localhost с правильным Host header
Write-Host "=== Тест API ErkaPharm через localhost ===" -ForegroundColor Green

$url = "https://localhost:9443"

try {
    # Создаем HTTP запрос с правильным Host header
    $request = [System.Net.WebRequest]::Create($url)
    $request.Method = "GET"
    $request.Timeout = 10000
    $request.Host = "api.erkapharm.com"

    # Игнорируем SSL сертификаты
    [System.Net.ServicePointManager]::ServerCertificateValidationCallback = {$true}

    $response = $request.GetResponse()
    $reader = New-Object System.IO.StreamReader($response.GetResponseStream())
    $content = $reader.ReadToEnd()
    $reader.Close()
    $response.Close()

    Write-Host "API доступен!" -ForegroundColor Green
    Write-Host "Статус: $($response.StatusCode)" -ForegroundColor Cyan
    Write-Host "Content-Type: $($response.ContentType)" -ForegroundColor Cyan

    # Проверяем что это Swagger UI
    if ($content -like "*swagger*") {
        Write-Host "Получен Swagger UI - API работает!" -ForegroundColor Green
    }
}
catch {
    Write-Host "Ошибка доступа к API" -ForegroundColor Red
    Write-Host "Ошибка: $($_.Exception.Message)" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Использование в браузере:" -ForegroundColor Green
Write-Host "https://localhost:9443" -ForegroundColor Cyan
Write-Host "(браузер автоматически передаст правильный Host header)"
Write-Host ""
Write-Host "Использование в приложении:" -ForegroundColor Green
Write-Host "Замените https://api.erkapharm.com/ на https://localhost:9443/" -ForegroundColor Cyan
