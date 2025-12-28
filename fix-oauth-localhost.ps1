# Исправление проблемы OAuth с localhost в Mullvad Browser

Write-Host "=== Диагностика OAuth localhost проблемы ===" -ForegroundColor Green
Write-Host ""

# 1. Проверяем приложение на порту 53920
Write-Host "1. Проверка приложения на порту 53920..." -ForegroundColor Yellow
try {
    $response = Invoke-WebRequest -Uri "http://localhost:53920/" -Method Get -TimeoutSec 5
    Write-Host "✓ Приложение отвечает (статус: $($response.StatusCode))" -ForegroundColor Green
} catch {
    Write-Host "✗ Приложение не отвечает: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}

# 2. Проверяем разные способы доступа
Write-Host ""
Write-Host "2. Тестирование разных способов доступа..." -ForegroundColor Yellow

$urls = @(
    "http://localhost:53920/oauth-callback?test=1",
    "http://127.0.0.1:53920/oauth-callback?test=1",
    "http://[::1]:53920/oauth-callback?test=1"
)

foreach ($url in $urls) {
    try {
        $response = Invoke-WebRequest -Uri $url -Method Get -TimeoutSec 3 -ErrorAction Stop
        Write-Host "✓ $url - работает (статус: $($response.StatusCode))" -ForegroundColor Green
    } catch {
        Write-Host "✗ $url - не работает" -ForegroundColor Red
    }
}

# 3. Рекомендации
Write-Host ""
Write-Host "=== РЕКОМЕНДАЦИИ ДЛЯ ИСПРАВЛЕНИЯ ===" -ForegroundColor Green
Write-Host ""

Write-Host "Вариант 1: Изменить DNS настройки в Mullvad Browser" -ForegroundColor White
Write-Host "  1. Открыть Настройки → Privacy & Security → DNS Protection" -ForegroundColor Gray
Write-Host "  2. Изменить 'Max Protection' на 'Standard'" -ForegroundColor Gray
Write-Host "  3. Попробовать авторизацию снова" -ForegroundColor Gray
Write-Host ""

Write-Host "Вариант 2: Использовать другой браузер для OAuth" -ForegroundColor White
Write-Host "  1. Открыть Chrome/Google Chrome" -ForegroundColor Gray
Write-Host "  2. Перейти по ссылке авторизации" -ForegroundColor Gray
Write-Host "  3. Завершить авторизацию там" -ForegroundColor Gray
Write-Host ""

Write-Host "Вариант 3: Временно отключить DNS over HTTPS" -ForegroundColor White
Write-Host "  1. В Mullvad Browser: Настройки → DNS Protection" -ForegroundColor Gray
Write-Host "  2. Снять галочку 'Enable DNS over HTTPS'" -ForegroundColor Gray
Write-Host "  3. Перезапустить браузер" -ForegroundColor Gray
Write-Host ""

Write-Host "Вариант 4: Добавить исключение для localhost" -ForegroundColor White
Write-Host "  1. В Firefox-based браузерах это обычно работает по умолчанию" -ForegroundColor Gray
Write-Host "  2. Проверить в about:config: network.dns.disableIPv6 = false" -ForegroundColor Gray
Write-Host ""

Write-Host "Вариант 5: Использовать IP вместо localhost" -ForegroundColor White
Write-Host "  Заменить localhost на 127.0.0.1 в URL авторизации" -ForegroundColor Gray
Write-Host ""

# 4. Создаем тестовый HTML файл
Write-Host "Вариант 6: Создание тестового HTML файла" -ForegroundColor White
$testHtml = @"
<!DOCTYPE html>
<html>
<head>
    <title>OAuth Localhost Test</title>
</head>
<body>
    <h1>Тест подключения к localhost</h1>
    <button onclick="testConnection()">Тестировать подключение</button>
    <div id="result"></div>

    <script>
        async function testConnection() {
            const result = document.getElementById('result');
            try {
                const response = await fetch('http://localhost:53920/oauth-callback?test=1');
                result.innerHTML = '<p style="color: green;">✓ Подключение работает! Статус: ' + response.status + '</p>';
            } catch (error) {
                result.innerHTML = '<p style="color: red;">✗ Ошибка: ' + error.message + '</p>';

                // Попробовать 127.0.0.1
                try {
                    const response2 = await fetch('http://127.0.0.1:53920/oauth-callback?test=1');
                    result.innerHTML += '<p style="color: green;">✓ 127.0.0.1 работает! Статус: ' + response2.status + '</p>';
                } catch (error2) {
                    result.innerHTML += '<p style="color: red;">✗ 127.0.0.1 тоже не работает</p>';
                }
            }
        }
    </script>
</body>
</html>
"@

$testHtml | Out-File -FilePath "oauth-test.html" -Encoding UTF8
Write-Host "  Создан файл oauth-test.html для тестирования" -ForegroundColor Gray
Write-Host "  Откройте его в Mullvad Browser и нажмите кнопку" -ForegroundColor Gray

Write-Host ""
Write-Host "Какой вариант хотите попробовать?" -ForegroundColor Cyan
























