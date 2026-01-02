@echo off
echo ========================================
echo    ПРОВЕРКА API ERKAPHARM
echo ========================================
echo.

REM Проверяем SSH туннель
echo 1. Проверяем SSH туннель...
netstat -ano | findstr ":9443" >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ SSH туннель не найден (порт 9443)
    echo.
    echo Запускаем туннель...
    wsl -d Ubuntu -- bash -c "cd /mnt/c/Users/User/mod-aggregator && ./start-ssh-tunnel.sh" >nul 2>&1
    timeout /t 3 >nul
)

netstat -ano | findstr ":9443" >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ Не удалось запустить SSH туннель
    echo Запустите вручную: wsl -d Ubuntu -- bash -c "cd /mnt/c/Users/User/mod-aggregator && ./start-ssh-tunnel.sh"
    pause
    exit /b 1
)

echo ✅ SSH туннель работает

echo.
echo 2. Тестируем API через туннель...
wsl -d Ubuntu -- bash -c "curl -k -s -I -H 'Host: api.erkapharm.com' https://localhost:9443 | grep 'HTTP/'" > temp_api_test.txt 2>&1

if exist temp_api_test.txt (
    set /p API_RESULT=<temp_api_test.txt
    if "%API_RESULT%"=="" (
        echo ❌ API НЕДОСТУПЕН
        echo Возможные причины:
        echo - VPN не работает
        echo - SSH туннель сломан
        echo - API сервер недоступен
    ) else (
        echo ✅ API ДОСТУПЕН
        echo Результат: %API_RESULT%
        echo.
        echo ========================================
        echo         ДОСТУП К API
        echo ========================================
        echo.
        echo В БРАУЗЕРЕ (с расширением ModHeader):
        echo 1. Установите: https://chromewebstore.google.com/detail/modheader/idgpnmonknjnojddfkpgkljpfnnfcklj
        echo 2. Добавьте правило:
        echo    - Request URL: https://localhost:9443/*
        echo    - Header Name: Host
        echo    - Header Value: api.erkapharm.com
        echo 3. Откройте: https://localhost:9443
        echo 4. Примите SSL предупреждение
        echo.
        echo В КОДЕ ПРИЛОЖЕНИЯ:
        echo Используйте: curl -k -H "Host: api.erkapharm.com" https://localhost:9443
        echo.
        echo ✅ ГОТОВО! API работает через SSH туннель!
    )
) else (
    echo ❌ Ошибка тестирования
)

del temp_api_test.txt 2>nul

echo.
pause



































