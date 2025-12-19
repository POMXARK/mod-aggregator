@echo off
echo === Запуск браузера с данными USA ===
echo.

REM Проверяем существование браузера (сначала Ungoogled Chromium, потом обычный Chrome)
set BROWSER_PATH=
if exist "C:\Program Files\Ungoogled-Chromium\Application\chrome.exe" (
    set BROWSER_PATH="C:\Program Files\Ungoogled-Chromium\Application\chrome.exe"
    set BROWSER_NAME=Ungoogled Chromium
) else if exist "C:\Program Files\Google\Chrome\Application\chrome.exe" (
    set BROWSER_PATH="C:\Program Files\Google\Chrome\Application\chrome.exe"
    set BROWSER_NAME=Google Chrome
) else (
    echo ОШИБКА: Ни Ungoogled Chromium, ни Google Chrome не найдены!
    echo.
    echo Варианты решения:
    echo 1. Установите Ungoogled Chromium: https://ungoogled-software.github.io/ungoogled-chromium-binaries/
    echo 2. Или используйте обычный Google Chrome (уже установлен)
    echo.
    pause
    exit /b 1
)

echo Найден браузер: %BROWSER_NAME%
echo Путь: %BROWSER_PATH%

echo Запускаем браузер с подменой данных на USA...
echo.

REM Запускаем браузер с параметрами
start "" %BROWSER_PATH% ^
--user-agent="Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36" ^
--lang=en-US ^
--disable-web-security ^
--disable-features=VizDisplayCompositor ^
--disable-ipc-flooding-protection ^
--new-window ^
"https://whatismyipaddress.com/"

echo.
echo ✓ Браузер запущен!
echo.
echo Для тестирования геолокации откройте: https://whatismyipaddress.com/
echo Для API erkapharm используйте: https://api.erkapharm.com
echo.
echo === ВАРИАНТЫ ЗАПУСКА ===
echo.
echo 1. Через PowerShell (если BAT не работает):
echo Start-Process "C:\Program Files\Ungoogled-Chromium\Application\chrome.exe" -ArgumentList '--user-agent="Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"', '--lang=en-US'
echo.
echo 2. Прямая команда в CMD:
echo "C:\Program Files\Ungoogled-Chromium\Application\chrome.exe" --user-agent="Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36" --lang=en-US
echo.
pause
