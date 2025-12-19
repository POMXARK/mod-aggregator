@echo off
echo ========================================
echo      ЗАПУСК API ERKAPHARM
echo ========================================
echo.

REM Проверяем WSL
wsl -l -q >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ WSL не установлен
    echo Установите WSL: wsl --install
    pause
    exit /b 1
)

REM Проверяем что SSH туннель работает
echo Проверяем SSH туннель...
netstat -ano | findstr ":9443" >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ SSH туннель не найден
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

REM Запускаем API прокси
echo Запускаем API прокси сервер...
start /B wsl -d Ubuntu -- bash -c "cd ~ && node api-proxy.js" >nul 2>&1

REM Ждём запуска
timeout /t 2 >nul

echo.
echo ========================================
echo         API ГОТОВ!
echo ========================================
echo.
echo 🌐 Откройте в браузере:
echo    http://localhost:3000
echo.
echo 📖 Должен загрузиться Swagger UI
echo.
echo 🔧 Или напрямую:
echo    https://localhost:9443
echo    (примите SSL предупреждение)
echo.
echo 📁 Открыть браузер: api-browser.html
echo.

REM Открываем HTML файл в браузере
start api-browser.html

echo Нажмите любую клавишу для выхода...
pause >nul

