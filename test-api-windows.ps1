# Тест API из Windows с правильными headers
Write-Host "=== Тест API из Windows ===" -ForegroundColor Green

$url = "https://localhost:9443"

try {
    # Игнорируем SSL проверки
    [System.Net.ServicePointManager]::ServerCertificateValidationCallback = {$true}

    # Создаем запрос с полными headers как в браузере
    $webClient = New-Object System.Net.WebClient
    $webClient.Headers.Add("Host", "api.erkapharm.com")
    $webClient.Headers.Add("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
    $webClient.Headers.Add("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
    $webClient.Headers.Add("Accept-Language", "ru-RU,ru;q=0.9,en-US;q=0.8,en;q=0.7")
    $webClient.Headers.Add("Connection", "keep-alive")

    $response = $webClient.DownloadString($url)

    Write-Host "API доступен из Windows!" -ForegroundColor Green
    if ($response -like "*swagger*") {
        Write-Host "Получен Swagger UI" -ForegroundColor Green
    }
}
catch {
    Write-Host "Ошибка: $($_.Exception.Message)" -ForegroundColor Red

    # Дополнительная информация
    if ($_.Exception.Message -like "*403*" -or $_.Exception.Message -like "*Forbidden*") {
        Write-Host ""
        Write-Host "Возможные причины 403 Forbidden:" -ForegroundColor Yellow
        Write-Host "1. API блокирует запросы без правильного Host header" -ForegroundColor Cyan
        Write-Host "2. API требует аутентификации" -ForegroundColor Cyan
        Write-Host "3. API проверяет IP адрес или другие параметры" -ForegroundColor Cyan
        Write-Host "4. API доступен только из внутренней сети ErkaPharm" -ForegroundColor Cyan
    }
}

Write-Host ""
Write-Host "Альтернативный тест - через браузер:" -ForegroundColor Green
Write-Host "1. Откройте https://localhost:9443" -ForegroundColor Cyan
Write-Host "2. Если 403 - попробуйте добавить в hosts файл:" -ForegroundColor Cyan
Write-Host "   127.0.0.1 api.erkapharm.com" -ForegroundColor Cyan
Write-Host "3. Или обратитесь к IT ErkaPharm за доступом" -ForegroundColor Cyan
