@echo off
echo === Быстрая диагностика Mullvad Browser ===
echo.

echo 1. Ищем Mullvad Browser...
set BROWSER_PATH=
if exist "C:\Program Files\Mullvad Browser\Browser\firefox.exe" (
    set BROWSER_PATH=C:\Program Files\Mullvad Browser\Browser\firefox.exe
    echo   ✓ Найден в Program Files
) else if exist "C:\Program Files (x86)\Mullvad Browser\Browser\firefox.exe" (
    set BROWSER_PATH=C:\Program Files (x86)\Mullvad Browser\Browser\firefox.exe
    echo   ✓ Найден в Program Files (x86)
) else (
    echo   ✗ MULLVAD BROWSER НЕ НАЙДЕН!
    echo.
    echo   Скачайте с официального сайта:
    echo   https://mullvad.net/en/browser
    echo.
    goto :end
)

echo.
echo 2. Проверяем интернет...
ping -n 1 -w 2000 google.com >nul 2>&1
if %errorlevel% equ 0 (
    echo   ✓ Интернет подключен
) else (
    echo   ✗ ИНТЕРНЕТ НЕ РАБОТАЕТ!
    goto :problems
)

echo.
echo 3. Проверяем DNS...
nslookup google.com 8.8.8.8 | findstr /C:"Address" | findstr /V "127.0.0.1" >nul 2>&1
if %errorlevel% equ 0 (
    echo   ✓ DNS разрешение работает
) else (
    echo   ✗ ПРОБЛЕМЫ С DNS!
    goto :problems
)

echo.
echo 4. Проверяем системный прокси...
reg query "HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings" /v ProxyEnable 2>nul | findstr "0x1" >nul 2>&1
if %errorlevel% equ 0 (
    echo   ⚠ СИСТЕМНЫЙ ПРОКСИ ВКЛЮЧЕН!
    echo     Это может блокировать Mullvad Browser
    goto :problems
) else (
    echo   ✓ Системный прокси отключен
)

echo.
echo 5. Проверяем антивирус...
sc query WinDefend | findstr "RUNNING" >nul 2>&1
if %errorlevel% equ 0 (
    echo   ⚠ Windows Defender активен
    echo     Может блокировать браузер
) else (
    echo   ✓ Windows Defender отключен или не найден
)

echo.
echo === ВСЕ СИСТЕМНЫЕ ПРОВЕРКИ ПРОЙДЕНЫ ===
echo.
echo Если Mullvad Browser все равно не грузит страницы:
echo.
echo ВАРИАНТ 1: Запустите в безопасном режиме
echo   %BROWSER_PATH% --safe-mode
echo.
echo ВАРИАНТ 2: Очистите профиль
echo   Удалите папку: %%APPDATA%%\Mullvad Browser
echo.
echo ВАРИАНТ 3: Отключите VPN/прокси если используется
echo.
goto :end

:problems
echo.
echo === ОБНАРУЖЕНЫ ПРОБЛЕМЫ ===
echo.
echo Возможные решения:
echo.
echo 1. Отключите системный прокси:
echo    Win + R → inetcpl.cpl → Подключения → снять галочку прокси
echo.
echo 2. Очистите DNS кэш:
echo    ipconfig /flushdns
echo.
echo 3. Перезагрузите маршрутизатор
echo.
echo 4. Временно отключите антивирус
echo.
echo 5. Попробуйте другой DNS: 1.1.1.1 или 8.8.8.8
echo.

:end
echo.
pause



































