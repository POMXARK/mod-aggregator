@echo off
echo ========================================
echo    ЗАПУСК API ПРОКСИ СЕРВЕРА
echo ========================================
echo.

REM Проверяем PowerShell
powershell -Command "Write-Host 'PowerShell OK'" >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ PowerShell недоступен
    pause
    exit /b 1
)

echo ✓ PowerShell найден

REM Проверяем SSH туннель
echo Проверяем SSH туннель...
netstat -ano | findstr ":9443" >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ SSH туннель не найден (порт 9443)
    echo Запускаем туннель...
    wsl -d Ubuntu -- bash -c "cd /mnt/c/Users/User/mod-aggregator && ./start-ssh-tunnel.sh" >nul 2>&1
    timeout /t 3 >nul
)

REM Проверяем ещё раз
netstat -ano | findstr ":9443" >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ Не удалось запустить SSH туннель
    echo Запустите вручную: wsl -d Ubuntu -- bash -c "cd /mnt/c/Users/User/mod-aggregator && ./start-ssh-tunnel.sh"
    pause
    exit /b 1
)

echo ✓ SSH туннель работает

echo.
echo ========================================
echo     ЗАПУСК ПРОКСИ СЕРВЕРА
echo ========================================
echo.
echo Прокси сервер будет доступен на:
echo http://localhost:8080
echo.
echo Он автоматически добавляет правильный Host header
echo и проксирует запросы на https://localhost:9443
echo.
echo Откройте браузер и перейдите по адресу выше
echo.
echo Остановка: закройте окно PowerShell
echo.

REM Запускаем прокси сервер
powershell -ExecutionPolicy Bypass -File "api-proxy-server.ps1"

echo.
pause
























