@echo off
echo === Отладка прокси сервера ===

echo 1. Проверяем что прокси сервер работает...
netstat -ano | findstr ":8080" >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ Прокси сервер не запущен (порт 8080)
    echo Запустите: .\start-api-proxy.bat
    pause
    exit /b 1
)
echo ✓ Прокси сервер работает

echo.
echo 2. Проверяем SSH туннель...
netstat -ano | findstr ":9443" >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ SSH туннель не работает (порт 9443)
    echo Запустите: wsl -d Ubuntu -- bash -c "cd /mnt/c/Users/User/mod-aggregator && ./start-ssh-tunnel.sh"
    pause
    exit /b 1
)
echo ✓ SSH туннель работает

echo.
echo 3. Тестируем API напрямую через туннель...
wsl -d Ubuntu -- bash -c "curl -k -s -I -H 'Host: api.erkapharm.com' https://localhost:9443 | findstr 'HTTP/'" 2>nul
if %errorlevel% neq 0 (
    echo ❌ API не доступен через туннель
) else (
    echo ✓ API доступен через туннель
)

echo.
echo 4. Проверяем прокси сервер вручную...
echo Попробуйте в браузере: http://localhost:8080
echo Если 403 - проблема в прокси сервере
echo.

echo 5. Альтернативный тест - через curl...
echo Тестируем прокси через curl:
powershell -ExecutionPolicy Bypass -Command "try { $response = Invoke-WebRequest -Uri 'http://localhost:8080' -TimeoutSec 10; Write-Host 'Прокси работает! Статус:' $response.StatusCode } catch { Write-Host 'Ошибка прокси:' $_.Exception.Message }"

echo.
echo === ВОЗМОЖНЫЕ ПРОБЛЕМЫ ===
echo.
echo 1. Прокси сервер не добавляет Host header
echo 2. Прокси не правильно проксирует HTTPS
echo 3. PowerShell ограничения на HTTPS прокси
echo.
echo === РЕШЕНИЯ ===
echo.
echo 1. Используйте Node.js прокси:
echo    wsl -d Ubuntu -- bash -c "cd ~ && node api-proxy.js"
echo    Затем откройте: http://localhost:3000
echo.
echo 2. Используйте расширение браузера ModHeader
echo    для https://localhost:9443 с Host: api.erkapharm.com
echo.

pause
























