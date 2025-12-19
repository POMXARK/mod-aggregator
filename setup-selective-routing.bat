@echo off
echo === Настройка селективной маршрутизации ErkaPharm ===

REM Получаем IP WSL
for /f "tokens=*" %%i in ('wsl -d Ubuntu -- hostname -I') do set WSL_IP=%%i
for /f "tokens=1" %%i in ("%WSL_IP%") do set WSL_IP=%%i

echo IP WSL: %WSL_IP%

echo.
echo Добавляем маршруты для ErkaPharm сетей:

REM 10.0.0.0/8 - Внутренняя сеть ErkaPharm
echo Добавляем 10.0.0.0/8 via %WSL_IP%
route delete 10.0.0.0 mask 255.0.0.0 %WSL_IP% >nul 2>&1
route add 10.0.0.0 mask 255.0.0.0 %WSL_IP%

REM 172.16.0.0/12 - Дополнительные подсети
echo Добавляем 172.16.0.0/12 via %WSL_IP%
route delete 172.16.0.0 mask 255.240.0.0 %WSL_IP% >nul 2>&1
route add 172.16.0.0 mask 255.240.0.0 %WSL_IP%

REM 172.21.0.0/16 - VPN подсеть
echo Добавляем 172.21.0.0/16 via %WSL_IP%
route delete 172.21.0.0 mask 255.255.0.0 %WSL_IP% >nul 2>&1
route add 172.21.0.0 mask 255.255.0.0 %WSL_IP%

REM 192.168.0.0/16 - Локальные сети
echo Добавляем 192.168.0.0/16 via %WSL_IP%
route delete 192.168.0.0 mask 255.255.0.0 %WSL_IP% >nul 2>&1
route add 192.168.0.0 mask 255.255.0.0 %WSL_IP%

echo.
echo Текущие маршруты через VPN:
route print | findstr "%WSL_IP%"

echo.
echo === Готово! ===
echo Теперь только трафик к ErkaPharm сетям идет через VPN
echo Остальной интернет работает напрямую
pause
