@echo off
echo === Проверка доступа к API ErkaPharm ===

REM Получаем IP WSL
for /f "tokens=*" %%i in ('wsl -d Ubuntu -- hostname -I') do set WSL_IP=%%i
for /f "tokens=1" %%i in ("%WSL_IP%") do set WSL_IP=%%i

echo IP WSL: %WSL_IP%
echo API IP: 5.172.178.51
echo.

echo 1. Проверяем маршрут для API:
route print | findstr "5.172.178.51" >nul 2>&1
if %errorlevel% equ 0 (
    echo    ✓ Маршрут настроен
) else (
    echo    ❌ МАРШРУТ НЕ НАСТРОЕН!
    echo    Запустите .\setup-selective-admin.bat от имени администратора
    goto :error
)

echo.
echo 2. Проверяем VPN:
wsl -d Ubuntu -- bash -c "ip addr show tun0 >/dev/null 2>&1 && echo '   ✓ VPN работает' || echo '   ❌ VPN не работает'"

echo.
echo 3. Тестируем API через VPN:
curl -k https://api.erkapharm.com --connect-timeout 5 --max-time 10 >nul 2>&1
if %errorlevel% equ 0 (
    echo    ✓ API доступен через VPN
) else (
    echo    ❌ API недоступен
)

echo.
echo 4. Тестируем прямой доступ к API:
curl -k https://5.172.178.51 --connect-timeout 5 --max-time 10 >nul 2>&1
if %errorlevel% equ 0 (
    echo    ✓ API доступен напрямую
) else (
    echo    ❌ API недоступен даже напрямую
)

echo.
echo === РЕКОМЕНДАЦИИ ===
echo Если API не загружается в браузере:
echo 1. Проверьте что маршрут настроен (шаг 1 выше)
echo 2. Очистите кэш браузера (Ctrl+F5)
echo 3. Попробуйте в режиме инкогнито
echo 4. Проверьте firewall Windows
echo.
echo Для настройки маршрута:
echo .\setup-selective-admin.bat (ОТ ИМЕНИ АДМИНИСТРАТОРА)

goto :end

:error
echo.
echo ОШИБКА: Маршрут не настроен!
echo Запустите PowerShell ОТ ИМЕНИ АДМИНИСТРАТОРА и выполните:
echo cd C:\Users\User\mod-aggregator
echo .\setup-selective-admin.bat

:end
pause
