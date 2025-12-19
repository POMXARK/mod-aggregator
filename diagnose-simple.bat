@echo off
echo === ПРОСТАЯ ДИАГНОСТИКА VPN ===
echo.

echo 1. Проверяем WSL статус:
wsl -l -v | findstr Ubuntu
echo.

echo 2. Проверяем VPN процесс в WSL:
wsl -d Ubuntu -- ps aux | findstr openconnect | findstr /v grep
if %errorlevel% equ 0 (
    echo   ✓ OpenConnect запущен
) else (
    echo   ❌ OpenConnect НЕ запущен
)
echo.

echo 3. Проверяем TUN интерфейс в WSL:
wsl -d Ubuntu -- ip addr show | findstr tun
if %errorlevel% equ 0 (
    echo   ✓ TUN интерфейс найден
) else (
    echo   ❌ TUN интерфейс НЕ найден
)
echo.

echo 4. Проверяем маршруты в Windows:
route print | findstr "0.0.0.0"
if %errorlevel% equ 0 (
    echo   ✓ Маршрут 0.0.0.0 найден
) else (
    echo   ❌ Маршрут 0.0.0.0 НЕ найден
)
echo.

echo 5. Текущий IP (может занять время):
curl -s --connect-timeout 10 ifconfig.me/ip
if %errorlevel% equ 0 (
    echo   ✓ IP получен
) else (
    echo   ❌ Ошибка получения IP
)
echo.

echo === РЕЗУЛЬТАТ ===
echo Если все пункты зеленые (✓) - VPN работает!
echo Если есть красные (❌) - есть проблемы.
echo.
pause
