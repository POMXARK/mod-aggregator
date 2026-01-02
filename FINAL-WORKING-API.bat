@echo off
echo ========================================
echo    РАБОЧИЙ ДОСТУП К API ERKAPHARM
echo ========================================
echo.

REM Проверяем SSH туннель
echo Проверяем SSH туннель...
netstat -ano | findstr ":9443" >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ SSH туннель не найден
    echo Запускаем...
    wsl -d Ubuntu -- bash -c "cd /mnt/c/Users/User/mod-aggregator && ./start-ssh-tunnel.sh" >nul 2>&1
    timeout /t 3 >nul
)

netstat -ano | findstr ":9443" >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ Не удалось запустить SSH туннель
    pause
    exit /b 1
)

echo ✓ SSH туннель работает

echo.
echo ========================================
echo    ВЫБЕРИТЕ СПОСОБ ДОСТУПА К API
echo ========================================
echo.

echo 1. ТЕСТИРОВАНИЕ через curl (рекомендуется):
echo    wsl -d Ubuntu -- bash -c "curl -k -H 'Host: api.erkapharm.com' https://localhost:9443"
echo.

echo 2. БРАУЗЕР с расширением ModHeader:
echo    - Установите расширение: https://chromewebstore.google.com/detail/modheader/idgpnmonknjnojddfkpgkljpfnnfcklj
echo    - Добавьте правило для https://localhost:9443/* :
echo      Header Name: Host
echo      Header Value: api.erkapharm.com
echo    - Откройте: https://localhost:9443
echo    - Примите SSL предупреждение
echo.

echo 3. ПРЯМОЙ ДОСТУП (с ручным добавлением Host):
echo    - Откройте: https://localhost:9443
echo    - Используйте браузерные инструменты разработчика
echo    - В Network tab вручную добавьте Host header
echo.

echo 4. ПРОКСИ СЕРВЕРА (если работают):
echo    - PowerShell прокси: http://localhost:8080
echo    - Node.js прокси: http://localhost:3000
echo.

echo ========================================
echo    ТЕСТИРОВАНИЕ API
echo ========================================
echo.

echo Тестируем API через curl:
wsl -d Ubuntu -- bash -c "curl -k -s -I -H 'Host: api.erkapharm.com' https://localhost:9443 | findstr 'HTTP/'"

echo.
echo ЕСЛИ ВЫШЕ "200 OK" - API ДОСТУПЕН!
echo.

echo Для доступа в приложении используйте:
echo curl -k -H "Host: api.erkapharm.com" https://localhost:9443
echo.

pause



































