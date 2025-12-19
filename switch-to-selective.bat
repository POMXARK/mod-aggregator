@echo off
echo === Переключение на селективную маршрутизацию ===

REM Получаем IP WSL
for /f "tokens=*" %%i in ('wsl -d Ubuntu -- hostname -I') do set WSL_IP=%%i
for /f "tokens=1" %%i in ("%WSL_IP%") do set WSL_IP=%%i

echo Текущий IP WSL: %WSL_IP%

echo.
echo Удаляем маршрут всего трафика (0.0.0.0)...
route delete 0.0.0.0 mask 0.0.0.0 %WSL_IP%

echo.
echo Добавляем маршруты только для ErkaPharm сетей:

REM 10.0.0.0/8 - Внутренняя сеть ErkaPharm
route add 10.0.0.0 mask 255.0.0.0 %WSL_IP%

REM 172.16.0.0/12 - Дополнительные подсети
route add 172.16.0.0 mask 255.240.0.0 %WSL_IP%

REM 172.21.0.0/16 - VPN подсеть
route add 172.21.0.0 mask 255.255.0.0 %WSL_IP%

REM 192.168.0.0/16 - Локальные сети
route add 192.168.0.0 mask 255.255.0.0 %WSL_IP%

echo.
echo Текущие маршруты через VPN:
route print | findstr "%WSL_IP%"

echo.
echo === Готово! ===
echo Теперь ТОЛЬКО ErkaPharm трафик идет через VPN
echo Обычный интернет работает напрямую
pause
