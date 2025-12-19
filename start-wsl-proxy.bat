@echo off
echo === Запуск Node.js прокси через WSL ===

REM Копируем файл в WSL
wsl -d Ubuntu -- bash -c "cp /mnt/c/Users/User/mod-aggregator/api-proxy.js ~/ 2>/dev/null || echo 'File copied'"

echo Запуск прокси через WSL...
echo Прокси будет доступен на: http://localhost:3000
echo.

wsl -d Ubuntu -- bash -c "cd ~ && node api-proxy.js"

echo.
pause

