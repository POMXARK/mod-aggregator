@echo off
echo === ЗАПУСК ВСЕЙ СИСТЕМЫ ERKAPHARM VPN ===

REM Проверяем права администратора
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo ❌ Требуются права администратора!
    echo Запустите командную строку или PowerShell ОТ ИМЕНИ АДМИНИСТРАТОРА
    pause
    exit /b 1
)

echo ✓ Права администратора подтверждены

echo.
echo 1. Запускаем WSL и VPN...
wsl -d Ubuntu -- bash -c "cd /mnt/c/Users/User/mod-aggregator && ./auto-connect.sh"
if %errorlevel% neq 0 (
    echo ❌ Ошибка запуска VPN
    pause
    exit /b 1
)

echo ✓ VPN запущен

echo.
echo 2. Настраиваем маршрутизацию...
call .\setup-selective-admin.bat
if %errorlevel% neq 0 (
    echo ❌ Ошибка настройки маршрутов
    pause
    exit /b 1
)

echo ✓ Маршруты настроены

echo.
echo 3. Запускаем SSH туннель...
wsl -d Ubuntu -- bash -c "cd /mnt/c/Users/User/mod-aggregator && ./start-ssh-tunnel.sh"
timeout /t 3 /nobreak >nul

REM Проверяем что SSH запустился
netstat -ano | findstr ":8194" >nul 2>&1
if %errorlevel% equ 0 (
    echo ✓ SSH туннель запущен
) else (
    echo ❌ SSH туннель не запустился
    echo Возможно, SSH ключ не настроен: ./setup-ssh-key.sh
)

echo.
echo 4. Финальная проверка...
call .\test-full-system.bat

echo.
echo === ГОТОВО! ===
echo Система ErkaPharm VPN запущена и готова к работе!
echo.
echo Доступные сервисы:
echo - Basket API: http://localhost:8194
echo - MongoDB: mongodb://localhost:9999
echo - ManZana: http://localhost:11006
echo.
pause
