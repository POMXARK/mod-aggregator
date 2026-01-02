@echo off
echo === Тестирование API с правильным Host header ===

REM Проверяем curl в WSL
wsl -d Ubuntu -- bash -c "which curl" >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ curl не найден в WSL
    echo Установите curl: sudo apt install curl
    pause
    exit /b 1
)

echo ✓ curl найден в WSL

echo.
echo === ТЕСТИРОВАНИЕ API ===
echo.

echo 1. Тест без Host header (ожидаем 403):
wsl -d Ubuntu -- bash -c "curl -k -s -I https://localhost:9443 | head -1"

echo.
echo 2. Тест с правильным Host header (ожидаем 200):
wsl -d Ubuntu -- bash -c "curl -k -s -I -H 'Host: api.erkapharm.com' https://localhost:9443 | head -1"

echo.
echo 3. Полный ответ API:
wsl -d Ubuntu -- bash -c "curl -k -s -H 'Host: api.erkapharm.com' https://localhost:9443 | head -10"

echo.
echo === ДЛЯ БРАУЗЕРА ===
echo.
echo Проблема: браузер отправляет Host: localhost:9443
echo Решение: нужно расширение для подмены Host header
echo.
echo Рекомендуемые расширения Chrome:
echo - "ModHeader" - для подмены headers
echo - "Requestly" - для перезаписи запросов
echo.
echo Настройка ModHeader:
echo 1. Установить расширение
echo 2. Добавить правило:
echo    - URL pattern: https://localhost:9443/*
echo    - Header name: Host
echo    - Header value: api.erkapharm.com
echo.

pause



































