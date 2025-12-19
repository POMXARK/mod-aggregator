@echo off
echo === Исправление доступа к API в браузере ===
echo.

REM Очищаем DNS кэш Windows
echo 1. Очищаем DNS кэш Windows...
ipconfig /flushdns
echo ✓ DNS кэш очищен

REM Очищаем ARP кэш
echo 2. Очищаем ARP кэш...
arp -d *
echo ✓ ARP кэш очищен

REM Перезапускаем сетевые службы
echo 3. Перезапускаем DNS клиент...
net stop dnscache
net start dnscache
echo ✓ DNS клиент перезапущен

REM Проверяем маршруты
echo 4. Проверяем маршруты...
route print | findstr "5.172.178.51" >nul 2>&1
if %errorlevel% equ 0 (
    echo ✓ Маршрут для API настроен
) else (
    echo ❌ МАРШРУТ ДЛЯ API НЕ НАСТРОЕН!
    echo Запустите .\setup-selective-admin.bat от имени администратора
    goto :error
)

REM Проверяем доступность API по IP
echo 5. Тестируем API по IP...
curl -k -I https://5.172.178.51 --connect-timeout 5 --max-time 10 >nul 2>&1
if %errorlevel% equ 0 (
    echo ✓ API доступен по IP через VPN
) else (
    echo ❌ API недоступен по IP
)

REM Проверяем DNS разрешение
echo 6. Проверяем DNS разрешение...
nslookup api.erkapharm.com 8.8.8.8 | findstr "Address" | findstr -v "127.0.0.1" >nul 2>&1
if %errorlevel% equ 0 (
    echo ✓ DNS разрешение работает
    for /f "tokens=2" %%i in ('nslookup api.erkapharm.com 8.8.8.8 ^| findstr "Address" ^| findstr -v "127.0.0.1"') do set API_IP=%%i
    echo   IP: %API_IP%
) else (
    echo ❌ DNS разрешение не работает
    set API_IP=5.172.178.51
    echo   Используем закешированный IP: %API_IP%
)

echo.
echo === ТЕСТИРОВАНИЕ В БРАУЗЕРЕ ===
echo.
echo Откройте браузер и перейдите по адресу:
echo https://%API_IP%/
echo.
echo Если не работает:
echo 1. Очистите кэш браузера: Ctrl+Shift+R
echo 2. Попробуйте режим инкогнито
echo 3. Отключите VPN временно и проверьте
echo 4. Проверьте firewall Windows
echo.
echo Для диагностики запустите: .\test-browser-api.ps1

goto :end

:error
echo.
echo ОШИБКА: Маршрутизация не настроена!
pause
exit /b 1

:end
echo.
echo === ДОПОЛНИТЕЛЬНЫЕ ИНСТРУМЕНТЫ ===
echo.
echo Для запуска Ungoogled Chromium с подменой данных на USA:
echo 1. Запустите PowerShell и выполните:
echo Start-Process "C:\Program Files\Ungoogled-Chromium\Application\chrome.exe" -ArgumentList '--user-agent="Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"', '--lang=en-US', '--disable-web-security', '--disable-features=VizDisplayCompositor'
echo.
echo 2. Или используйте ярлык с этими параметрами
echo.
echo 3. Для тестирования геолокации: https://whatismyipaddress.com/
echo.
echo === ГОТОВО ===
pause
