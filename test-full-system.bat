@echo off
echo === ПОЛНЫЙ ТЕСТ СИСТЕМЫ ERKAPHARM VPN ===

echo.
echo 1. Проверяем VPN в WSL...
wsl -d Ubuntu -- bash -c "ip addr show tun0 >/dev/null 2>&1 && echo '   ✓ VPN подключен' || echo '   ❌ VPN не подключен'"

echo.
echo 2. Проверяем маршрутизацию Windows...
route print | findstr "172.18.58.253" >nul 2>&1
if %errorlevel% equ 0 (
    echo    ✓ Маршруты ErkaPharm настроены
    route print | findstr "172.18.58.253" | findstr /c:"10.0.0.0" >nul 2>&1 && echo    ✓ 10.0.0.0/8
    route print | findstr "172.18.58.253" | findstr /c:"172.16.0.0" >nul 2>&1 && echo    ✓ 172.16.0.0/12
    route print | findstr "172.18.58.253" | findstr /c:"172.21.0.0" >nul 2>&1 && echo    ✓ 172.21.0.0/16
    route print | findstr "172.18.58.253" | findstr /c:"192.168.0.0" >nul 2>&1 && echo    ✓ 192.168.0.0/16
) else (
    echo    ❌ Маршруты ErkaPharm НЕ настроены
)

echo.
echo 3. Проверяем обычный интернет...
ping -n 1 google.com >nul 2>&1
if %errorlevel% equ 0 (
    echo    ✓ Обычный интернет работает
) else (
    echo    ❌ Обычный интернет НЕ работает
)

echo.
echo 4. Тестируем SSH доступ к ErkaPharm...
wsl -d Ubuntu -- bash -c "timeout 3 ssh -o StrictHostKeyChecking=no -o ConnectTimeout=3 xcom@xcom-01.dev.erkapharm.ru 'echo SSH OK' 2>/dev/null" 2>nul | findstr "SSH OK" >nul 2>&1
if %errorlevel% equ 0 (
    echo    ✓ SSH к ErkaPharm работает
) else (
    echo    ❌ SSH к ErkaPharm НЕ работает
)

echo.
echo 5. Проверяем SSH туннели...
netstat -ano | findstr ":8194" >nul 2>&1 && echo "   ✓ Порт 8194 открыт (basket API)" || echo "   ❌ Порт 8194 закрыт (basket API)"
netstat -ano | findstr ":9999" >nul 2>&1 && echo "   ✓ Порт 9999 открыт (MongoDB)" || echo "   ❌ Порт 9999 закрыт (MongoDB)"

echo.
echo === РЕЗУЛЬТАТ ===
echo Если VPN и маршруты работают - система готова!
echo Если SSH работает - можно запускать туннели.
echo.
pause
