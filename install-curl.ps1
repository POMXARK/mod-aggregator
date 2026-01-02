# Установка curl в Windows
Write-Host "=== Установка curl в Windows ===" -ForegroundColor Green

# Проверяем, установлен ли уже curl
$curlExists = Get-Command curl -ErrorAction SilentlyContinue
if ($curlExists) {
    Write-Host "curl уже установлен" -ForegroundColor Green
    curl --version | Select-Object -First 1
} else {
    Write-Host "curl не найден, пытаемся установить..." -ForegroundColor Yellow

    # Проверяем winget
    $wingetExists = Get-Command winget -ErrorAction SilentlyContinue
    if ($wingetExists) {
        Write-Host "Устанавливаем curl через winget..." -ForegroundColor Cyan
        try {
            winget install curl --accept-source-agreements --accept-package-agreements
            Write-Host "curl установлен через winget" -ForegroundColor Green
        } catch {
            Write-Host "Ошибка установки через winget: $($_.Exception.Message)" -ForegroundColor Red
        }
    } else {
        # Проверяем chocolatey
        $chocoExists = Get-Command choco -ErrorAction SilentlyContinue
        if ($chocoExists) {
            Write-Host "Устанавливаем curl через chocolatey..." -ForegroundColor Cyan
            try {
                choco install curl -y
                Write-Host "curl установлен через chocolatey" -ForegroundColor Green
            } catch {
                Write-Host "Ошибка установки через chocolatey: $($_.Exception.Message)" -ForegroundColor Red
            }
        } else {
            Write-Host "Не найдены winget или chocolatey" -ForegroundColor Red
            Write-Host ""
            Write-Host "=== РУЧНАЯ УСТАНОВКА ===" -ForegroundColor Yellow
            Write-Host ""
            Write-Host "Вариант 1 - Git for Windows (рекомендуется):" -ForegroundColor Cyan
            Write-Host "Скачайте и установите: https://gitforwindows.org/" -ForegroundColor White
            Write-Host "curl будет доступен автоматически" -ForegroundColor Green
            Write-Host ""
            Write-Host "Вариант 2 - Официальный curl:" -ForegroundColor Cyan
            Write-Host "1. Перейдите: https://curl.se/windows/" -ForegroundColor White
            Write-Host "2. Скачайте zip архив" -ForegroundColor White
            Write-Host "3. Распакуйте в C:\curl" -ForegroundColor White
            Write-Host "4. Добавьте C:\curl\bin в PATH" -ForegroundColor White
        }
    }
}

# Финальная проверка
Write-Host ""
Write-Host "=== ПРОВЕРКА УСТАНОВКИ ===" -ForegroundColor Green
$curlAfter = Get-Command curl -ErrorAction SilentlyContinue
if ($curlAfter) {
    Write-Host "✓ curl найден" -ForegroundColor Green
    $version = curl --version 2>$null | Select-Object -First 1
    Write-Host "Версия: $version" -ForegroundColor Cyan

    Write-Host ""
    Write-Host "=== ТЕСТ ЗАПРОСА ===" -ForegroundColor Green
    Write-Host "Тестируем на api.erkapharm.com:" -ForegroundColor Cyan
    try {
        $response = curl -k -I https://api.erkapharm.com --connect-timeout 5 --max-time 10 2>$null | Select-String "HTTP/"
        if ($response) {
            Write-Host "✓ curl работает: $($response.Line)" -ForegroundColor Green
        } else {
            Write-Host "⚠️  curl установлен, но не может подключиться к API" -ForegroundColor Yellow
            Write-Host "Возможно, нужен VPN или другая сеть" -ForegroundColor Cyan
        }
    } catch {
        Write-Host "❌ Ошибка тестирования: $($_.Exception.Message)" -ForegroundColor Red
    }
} else {
    Write-Host "❌ curl не найден после установки" -ForegroundColor Red
    Write-Host "Попробуйте перезапустить PowerShell или командную строку" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Готово!" -ForegroundColor Green



































