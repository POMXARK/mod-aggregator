@echo off
echo === Запуск Node.js прокси для API ===
echo.

REM Проверяем Node.js
node --version >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ Node.js не установлен
    echo Скачайте с https://nodejs.org/
    pause
    exit /b 1
)

echo ✓ Node.js найден
node --version

echo.
echo Запуск прокси сервера...
echo Прокси: http://localhost:3000 -^> https://localhost:9443 (с Host: api.erkapharm.com)
echo.

node api-proxy.js

echo.
pause



































