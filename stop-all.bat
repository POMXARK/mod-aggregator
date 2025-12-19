@echo off
echo === ОСТАНОВКА СИСТЕМЫ ERKAPHARM VPN ===

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
echo 1. Останавливаем SSH туннели...
wsl -d Ubuntu -- bash -c "pkill -f ssh" 2>nul
echo ✓ SSH туннели остановлены

echo.
echo 2. Останавливаем VPN...
wsl -d Ubuntu -- bash -c "sudo pkill openconnect" 2>nul
echo ✓ VPN остановлен

echo.
echo 3. Очищаем маршруты...
route delete 10.0.0.0 mask 255.0.0.0 >nul 2>&1
route delete 172.16.0.0 mask 255.240.0.0 >nul 2>&1
route delete 172.21.0.0 mask 255.255.0.0 >nul 2>&1
route delete 192.168.0.0 mask 255.255.0.0 >nul 2>&1
echo ✓ Маршруты очищены

echo.
echo 4. Проверяем что все остановлено...
wsl -d Ubuntu -- bash -c "ps aux | grep -E '(openconnect|ssh)' | grep -v grep" 2>nul | findstr "." >nul 2>&1
if %errorlevel% neq 0 (
    echo ✓ Все процессы остановлены
) else (
    echo ⚠️ Некоторые процессы могут еще работать
)

echo.
echo === СИСТЕМА ОСТАНОВЛЕНА ===
echo VPN отключен, маршруты очищены.
echo.
pause
