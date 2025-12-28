@echo off
echo === БЫСТРАЯ ПРОВЕРКА API ===
echo.

echo Проверяем SSH туннель...
netstat -ano | findstr ":9443" >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ SSH туннель не работает
    echo Запустите: wsl -d Ubuntu -- bash -c "cd /mnt/c/Users/User/mod-aggregator && ./start-ssh-tunnel.sh"
) else (
    echo ✅ SSH туннель работает
)

echo.
echo Тестируем API...
powershell -ExecutionPolicy Bypass -Command "
try {
    [System.Net.ServicePointManager]::ServerCertificateValidationCallback = {$true}
    $request = [System.Net.WebRequest]::Create('https://localhost:9443')
    $request.Method = 'GET'
    $request.Host = 'api.erkapharm.com'
    $request.Timeout = 5000
    $response = $request.GetResponse()
    $response.Close()
    Write-Host '✅ API ДОСТУПЕН через туннель!'
} catch {
    Write-Host '❌ API НЕДОСТУПЕН:' $_.Exception.Message
}
"

echo.
echo === ИНСТРУКЦИИ ПО ДОСТУПУ ===
echo.
echo 1. Установите расширение ModHeader для браузера
echo 2. Добавьте правило для https://localhost:9443/*
echo    Host: api.erkapharm.com
echo 3. Откройте https://localhost:9443
echo 4. Примите SSL предупреждение
echo.
echo ✅ ГОТОВО!

pause
























