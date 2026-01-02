# Простой тест API
Write-Host "=== Простой тест API ===" -ForegroundColor Green

try {
    [System.Net.ServicePointManager]::ServerCertificateValidationCallback = {$true}

    $request = [System.Net.WebRequest]::Create("https://localhost:9443")
    $request.Method = "GET"
    $request.Timeout = 5000
    $request.Host = "api.erkapharm.com"

    $response = $request.GetResponse()
    $reader = New-Object System.IO.StreamReader($response.GetResponseStream())
    $content = $reader.ReadToEnd()
    $reader.Close()
    $response.Close()

    Write-Host "API доступен!" -ForegroundColor Green
    Write-Host "Статус: $($response.StatusCode)" -ForegroundColor Cyan

    if ($content -like "*swagger*") {
        Write-Host "Swagger UI получен - API работает!" -ForegroundColor Green
    } else {
        Write-Host "Получен ответ, но не Swagger UI" -ForegroundColor Yellow
    }
}
catch {
    Write-Host "Ошибка доступа к API" -ForegroundColor Red
    Write-Host "Причина: $($_.Exception.Message)" -ForegroundColor Yellow

    if ($_.Exception.Message -like "*403*") {
        Write-Host ""
        Write-Host "403 Forbidden - возможные решения:" -ForegroundColor Yellow
        Write-Host "1. API требует специальной аутентификации" -ForegroundColor Cyan
        Write-Host "2. API доступен только из внутренней сети" -ForegroundColor Cyan
        Write-Host "3. Обратитесь к IT ErkaPharm" -ForegroundColor Cyan
    }
}

Write-Host ""
Write-Host "Тест в браузере: https://localhost:9443" -ForegroundColor Green



































