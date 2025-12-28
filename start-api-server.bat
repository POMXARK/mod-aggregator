@echo off
echo ========================================
echo    ЗАПУСК API ERKAPHARM СЕРВЕРА
echo ========================================
echo.

REM Проверяем WSL
wsl -l -q >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ WSL не установлен или не работает
    echo Установите WSL: wsl --install
    pause
    exit /b 1
)

echo ✓ WSL найден

REM Проверяем Node.js в WSL
wsl -d Ubuntu -- bash -c "node --version" >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ Node.js не установлен в WSL
    echo Установите: wsl -d Ubuntu -- bash -c "sudo apt update && sudo apt install -y nodejs npm"
    pause
    exit /b 1
)

echo ✓ Node.js найден в WSL

REM Копируем прокси файл
echo Копируем API прокси...
wsl -d Ubuntu -- bash -c "cp /mnt/c/Users/User/mod-aggregator/api-proxy.js ~/ 2>/dev/null; echo 'Файл скопирован'"

REM Проверяем SSH туннель
echo.
echo Проверяем SSH туннели...
netstat -ano | findstr ":9443" >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ SSH туннель не найден (порт 9443)
    echo Запустите сначала: .\start-ssh-tunnel.sh
    pause
    exit /b 1
)

echo ✓ SSH туннель работает

REM Запускаем прокси сервер
echo.
echo ========================================
echo    ЗАПУСК ПРОКСИ СЕРВЕРА
echo ========================================
echo.
echo Сервер будет доступен на:
echo http://localhost:3000
echo.
echo Откройте браузер и перейдите по адресу выше
echo.
echo Остановка: Ctrl+C или закройте окно
echo.

wsl -d Ubuntu -- bash -c "cd ~ && node api-proxy.js"

echo.
pause
























