# Скрипт для запуска браузера с подменой данных на USA
# Использование: .\launch-usa-chromium.ps1

param(
    [string]$Url = "https://whatismyipaddress.com/"
)

Write-Host "=== Запуск браузера с данными USA ===" -ForegroundColor Green

# Проверяем наличие браузеров в порядке предпочтения
$chromePaths = @(
    "C:\Program Files\Ungoogled-Chromium\Application\chrome.exe",
    "C:\Program Files\Google\Chrome\Application\chrome.exe",
    "C:\Program Files (x86)\Google\Chrome\Application\chrome.exe"
)

$chromePath = $null
$browserName = ""

foreach ($path in $chromePaths) {
    if (Test-Path $path) {
        $chromePath = $path
        if ($path -like "*ungoogled*") {
            $browserName = "Ungoogled Chromium"
        } else {
            $browserName = "Google Chrome"
        }
        break
    }
}
$userAgent = 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36'

# Проверяем существование браузера
if (!$chromePath) {
    Write-Host "Ошибка: Ни Ungoogled Chromium, ни Google Chrome не найдены!" -ForegroundColor Red
    Write-Host ""
    Write-Host "Варианты решения:" -ForegroundColor Yellow
    Write-Host "1. Установите Ungoogled Chromium: https://ungoogled-software.github.io/ungoogled-chromium-binaries/" -ForegroundColor Cyan
    Write-Host "2. Используйте обычный Google Chrome (найден в системе)" -ForegroundColor Cyan
    Write-Host "3. Установите другой Chromium-based браузер" -ForegroundColor Cyan
    exit 1
}

Write-Host "Найден браузер: $browserName" -ForegroundColor Green
Write-Host "Путь: $chromePath" -ForegroundColor Gray

Write-Host "Запуск браузера с параметрами USA..." -ForegroundColor Cyan

# Запускаем браузер с параметрами
$arguments = @(
    "--user-agent=$userAgent",
    "--lang=en-US",
    "--disable-web-security",
    "--disable-features=VizDisplayCompositor",
    "--disable-ipc-flooding-protection",
    "--new-window",
    $Url
)

try {
    Start-Process -FilePath $chromePath -ArgumentList $arguments
    Write-Host "✓ Браузер запущен успешно!" -ForegroundColor Green
    Write-Host "Проверьте геолокацию на сайте: $Url" -ForegroundColor Cyan
} catch {
    Write-Host "Ошибка при запуске браузера: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host ""
Write-Host "=== ДОПОЛНИТЕЛЬНЫЕ ВАРИАНТЫ ЗАПУСКА ===" -ForegroundColor Yellow
Write-Host ""
Write-Host "Вариант 1 - Через командную строку CMD:" -ForegroundColor White
Write-Host "start `"`" `"$chromePath`" --user-agent=`"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36`" --lang=en-US" -ForegroundColor Gray
Write-Host ""
Write-Host "Вариант 2 - Создать ярлык на рабочем столе:" -ForegroundColor White
Write-Host "В поле `"Расположение объекта`": `"$chromePath`" --user-agent=`"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36`" --lang=en-US" -ForegroundColor Gray
Write-Host ""
Write-Host "Вариант 3 - С дополнительными параметрами приватности:" -ForegroundColor White
Write-Host '.\launch-usa-chromium.ps1 -Url "https://api.erkapharm.com"' -ForegroundColor Gray
