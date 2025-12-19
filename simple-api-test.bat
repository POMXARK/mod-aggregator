@echo off
echo === ПРОСТОЙ ТЕСТ API ===
echo.

echo Тестируем API напрямую через туннель:
wsl -d Ubuntu -- bash -c "curl -k -s -I -H 'Host: api.erkapharm.com' https://localhost:9443 | grep 'HTTP/'" > temp_result.txt 2>&1
set /p RESULT=<temp_result.txt
if "%RESULT%"=="" (
    echo ❌ API НЕДОСТУПЕН через туннель
    echo Проверьте SSH туннель и VPN.
) else (
    echo ✅ API РАБОТАЕТ через туннель!
    echo Результат: %RESULT%
)
del temp_result.txt 2>nul
echo.

echo === РЕКОМЕНДАЦИИ ===
echo.

echo 1. Используйте curl для тестирования:
echo    wsl -d Ubuntu -- bash -c "curl -k -H 'Host: api.erkapharm.com' https://localhost:9443"
echo.

echo 2. Для браузера установите расширение ModHeader:
echo    - URL: https://localhost:9443/*
echo    - Header: Host = api.erkapharm.com
echo    - Откройте: https://localhost:9443
echo.

echo 3. Или используйте приложение напрямую через curl/wget
echo.

pause
