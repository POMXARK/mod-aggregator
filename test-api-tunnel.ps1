# Проверка API туннеля
Write-Host "=== Проверка API туннеля ===" -ForegroundColor Green

# Игнорируем SSL проверки
[System.Net.ServicePointManager]::ServerCertificateValidationCallback = {$true}

try {
    $request = [System.Net.WebRequest]::Create("https://localhost:9443")
    $request.Method = "HEAD"
    $request.Timeout = 5000

    $response = $request.GetResponse()
    Write-Host "✅ API доступен через туннель!" -ForegroundColor Green
    Write-Host "Статус: $($response.StatusCode)" -ForegroundColor Cyan
    Write-Host "Сервер: $($response.Server)" -ForegroundColor Cyan
    $response.Close()
} catch {
    Write-Host "❌ API недоступен через туннель" -ForegroundColor Red
    Write-Host "Ошибка: $($_.Exception.Message)" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Если работает - используйте в приложении:" -ForegroundColor Green
Write-Host "https://localhost:9443/" -ForegroundColor Cyan
Write-Host ""
Write-Host "Вместо:" -ForegroundColor Yellow
Write-Host "https://api.erkapharm.com/" -ForegroundColor Cyan
























