@echo off
echo === Исправление проблем Mullvad Browser ===
echo.

echo Выберите вариант исправления:
echo.
echo 1. Быстрая диагностика (рекомендуется сначала)
echo 2. Запуск в безопасном режиме
echo 3. Очистка профиля браузера
echo 4. Отключение системного прокси
echo 5. Сброс сетевых настроек
echo 6. Переустановка браузера (ссылка)
echo.
set /p choice="Введите номер варианта (1-6): "

if "%choice%"=="1" goto diagnose
if "%choice%"=="2" goto safe_mode
if "%choice%"=="3" goto clear_profile
if "%choice%"=="4" goto disable_proxy
if "%choice%"=="5" goto reset_network
if "%choice%"=="6" goto reinstall
echo Неверный выбор!
pause
exit /b 1

:diagnose
echo.
echo === БЫСТРАЯ ДИАГНОСТИКА ===
echo.

echo 1. Проверка наличия браузера...
if exist "C:\Program Files\Mullvad Browser\Browser\firefox.exe" (
    echo   Найден: C:\Program Files\Mullvad Browser\Browser\firefox.exe
) else if exist "C:\Program Files (x86)\Mullvad Browser\Browser\firefox.exe" (
    echo   Найден: C:\Program Files (x86)\Mullvad Browser\Browser\firefox.exe
) else (
    echo   Mullvad Browser НЕ НАЙДЕН!
    echo   Скачайте: https://mullvad.net/en/browser
    goto end
)

echo.
echo 2. Проверка интернет подключения...
ping -n 1 google.com >nul 2>&1
if %errorlevel% equ 0 (
    echo   Интернет работает
) else (
    echo   ПРОБЛЕМЫ С ИНТЕРНЕТОМ!
)

echo.
echo 3. Проверка DNS...
nslookup google.com 8.8.8.8 | findstr "Address" >nul 2>&1
if %errorlevel% equ 0 (
    echo   DNS работает
) else (
    echo   ПРОБЛЕМЫ С DNS!
)

echo.
echo 4. Проверка системного прокси...
reg query "HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings" /v ProxyEnable | findstr "0x1" >nul 2>&1
if %errorlevel% equ 0 (
    echo   СИСТЕМНЫЙ ПРОКСИ ВКЛЮЧЕН - может конфликтовать!
) else (
    echo   Системный прокси отключен
)

echo.
echo === РЕЗУЛЬТАТЫ ДИАГНОСТИКИ ===
echo Если браузер не грузит страницы, попробуйте:
echo - Отключить системный прокси (вариант 4)
echo - Очистить профиль (вариант 3)
echo - Запустить в безопасном режиме (вариант 2)
goto end

:safe_mode
echo.
echo Запуск в безопасном режиме...
powershell -ExecutionPolicy Bypass -File ".\launch-mullvad-safe.ps1"
goto end

:clear_profile
echo.
echo Очистка профиля браузера...
echo Это удалит все закладки, историю и настройки!
set /p confirm="Продолжить? (y/n): "
if /i not "%confirm%"=="y" goto end

if exist "%APPDATA%\Mullvad Browser" (
    rmdir /s /q "%APPDATA%\Mullvad Browser"
    echo ✓ Профиль очищен
) else (
    echo Профиль не найден
)
goto end

:disable_proxy
echo.
echo Отключение системного прокси...
reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings" /v ProxyEnable /t REG_DWORD /d 0 /f
echo ✓ Прокси отключен
echo Перезапустите браузер
goto end

:reset_network
echo.
echo Сброс сетевых настроек...
netsh winsock reset
netsh int ip reset
ipconfig /flushdns
echo ✓ Сетевые настройки сброшены
echo Перезагрузите компьютер для применения изменений
goto end

:reinstall
echo.
echo Переустановка Mullvad Browser:
echo 1. Удалите текущую установку через Панель управления
echo 2. Скачайте свежую версию:
echo    https://mullvad.net/en/browser
echo 3. Установите заново
echo.
start https://mullvad.net/en/browser
goto end

:end
echo.
echo === ГОТОВО ===
pause
