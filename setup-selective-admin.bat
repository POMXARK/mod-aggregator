@echo off
echo === Настройка селективной маршрутизации (админ) ===

REM Получаем IP WSL
for /f "tokens=*" %%i in ('wsl -d Ubuntu -- hostname -I') do set WSL_IP=%%i
for /f "tokens=1" %%i in ("%WSL_IP%") do set WSL_IP=%%i

echo IP WSL: %WSL_IP%

echo.
echo Проверяем права администратора...
net session >nul 2>&1
if %errorLevel% == 0 (
    echo ✓ Запущено от имени администратора
) else (
    echo ❌ НЕТ прав администратора!
    echo Запустите командную строку или PowerShell ОТ ИМЕНИ АДМИНИСТРАТОРА
    pause
    exit /b 1
)

echo.
echo Удаляем старые маршруты...
route delete 0.0.0.0 mask 0.0.0.0 %WSL_IP% >nul 2>&1
route delete 10.0.0.0 mask 255.0.0.0 %WSL_IP% >nul 2>&1
route delete 172.16.0.0 mask 255.240.0.0 %WSL_IP% >nul 2>&1
route delete 172.21.0.0 mask 255.255.0.0 %WSL_IP% >nul 2>&1
route delete 192.168.0.0 mask 255.255.0.0 %WSL_IP% >nul 2>&1
route delete 5.172.178.51 mask 255.255.255.255 %WSL_IP% >nul 2>&1

echo.
echo Добавляем селективные маршруты ErkaPharm:

echo Добавляем 10.0.0.0/8...
route add 10.0.0.0 mask 255.0.0.0 %WSL_IP%
if %errorlevel% equ 0 (echo   ✓ Успешно) else (echo   ❌ Ошибка)

echo Добавляем 172.16.0.0/12...
route add 172.16.0.0 mask 255.240.0.0 %WSL_IP%
if %errorlevel% equ 0 (echo   ✓ Успешно) else (echo   ❌ Ошибка)

echo Добавляем 172.21.0.0/16...
route add 172.21.0.0 mask 255.255.0.0 %WSL_IP%
if %errorlevel% equ 0 (echo   ✓ Успешно) else (echo   ❌ Ошибка)

echo Добавляем 192.168.0.0/16...
route add 192.168.0.0 mask 255.255.0.0 %WSL_IP%
if %errorlevel% equ 0 (echo   ✓ Успешно) else (echo   ❌ Ошибка)

echo Добавляем api.erkapharm.com (5.172.178.51)...
route add 5.172.178.51 mask 255.255.255.255 %WSL_IP%
if %errorlevel% equ 0 (echo   ✓ Успешно) else (echo   ❌ Ошибка)

echo.
echo Финальные маршруты через VPN:
route print | findstr "%WSL_IP%"

echo.
echo === ГОТОВО! ===
echo Теперь ТОЛЬКО ErkaPharm трафик идет через VPN
echo SSH туннель должен заработать!
echo.
echo Для проверки: .\test-selective-routing.ps1
pause
