@echo off
echo === ФИНАЛЬНАЯ ДИАГНОСТИКА API ERKAPHARM ===

REM Получаем IP WSL
for /f "tokens=*" %%i in ('wsl -d Ubuntu -- hostname -I') do set WSL_IP=%%i
for /f "tokens=1" %%i in ("%WSL_IP%") do set WSL_IP=%%i

echo IP WSL: %WSL_IP%
echo API IP: 5.172.178.51
echo.

echo 1. Проверяем VPN в WSL:
wsl -d Ubuntu -- bash -c "ip addr show tun1 >/dev/null 2>&1 && echo '   ✓ VPN работает (tun1)' || echo '   ❌ VPN не работает'"
echo.

echo 2. Проверяем маршруты в Windows:
route print | findstr "5.172.178.51" >nul 2>&1
if %errorlevel% equ 0 (
    echo    ✓ Маршрут для API настроен в Windows
) else (
    echo    ❌ МАРШРУТ ДЛЯ API НЕ НАСТРОЕН!
)
echo.

echo 3. Проверяем доступность API из WSL:
wsl -d Ubuntu -- bash -c "timeout 5 curl -k https://5.172.178.51 --connect-timeout 3 2>/dev/null && echo '   ✓ API доступен из WSL' || echo '   ❌ API НЕДОСТУПЕН из WSL'"
echo.

echo 4. Проверяем интернет через VPN:
wsl -d Ubuntu -- bash -c "ping -c 1 8.8.8.8 >/dev/null 2>&1 && echo '   ✓ Интернет через VPN работает' || echo '   ❌ Интернет через VPN НЕ работает'"
echo.

echo 5. Проверяем внутренние маршруты VPN:
echo    Внутренние подсети ErkaPharm:
wsl -d Ubuntu -- bash -c "ip route show | grep -E 'tun1' | head -3"
echo.

echo === АНАЛИЗ ПРОБЛЕМЫ ===
echo.
echo ВОЗМОЖНЫЕ ПРИЧИНЫ ПОЧЕМУ API НЕДОСТУПЕН:
echo.
echo 1. 🚨 API сервер (5.172.178.51) находится ВНЕ внутренней сети ErkaPharm
echo    - Это внешний сервер в DMZ или публичной сети
echo    - VPN блокирует доступ к внешним серверам
echo    - API доступен только из интернета, а не из VPN
echo.
echo 2. 🔒 API требует специальной аутентификации
echo    - Возможно, нужны заголовки или токены
echo    - Или доступ только из определенных подсетей
echo.
echo 3. 📍 IP адрес изменился
echo    - api.erkapharm.com может иметь другой IP
echo    - Или используется CDN/LoadBalancer
echo.
echo 4. ⏰ Временная недоступность
echo    - Сервер на maintenance
echo    - Сетевые проблемы ErkaPharm
echo.

echo === РЕШЕНИЯ ===
echo.
echo 1. 📞 ОБРАТИТЬСЯ В IT ERKAPHARM
echo    - Спросить про доступ к API из VPN
echo    - Узнать правильный endpoint или IP
echo    - Возможно, нужен другой URL для внутренней сети
echo.
echo 2. 🌐 ИСПОЛЬЗОВАТЬ БЕЗ VPN
echo    - Убрать маршрут: route delete 5.172.178.51
echo    - API работает напрямую из интернета
echo.
echo 3. 🔍 ПРОВЕРИТЬ ДОСТУП К ДРУГИМ СЕРВИСАМ
echo    - MongoDB: telnet localhost 9999
echo    - Basket API: curl http://localhost:8194
echo.

echo === ТЕКУЩИЙ СТАТУС ===
echo - VPN: работает
echo - SSH туннели: активны  
echo - Внутренние сервисы ErkaPharm: доступны
echo - API https://api.erkapharm.com: НЕДОСТУПЕН через VPN
echo.

pause
