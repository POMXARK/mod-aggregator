@echo off
echo === Установка curl в Windows ===

REM Проверяем, установлен ли уже curl
where curl >nul 2>&1
if %errorlevel% equ 0 (
    echo ✓ curl уже установлен
    curl --version
    goto :test
)

echo Проверяем наличие winget...
where winget >nul 2>&1
if %errorlevel% equ 0 (
    echo ✓ winget найден, устанавливаем curl...
    winget install curl --accept-source-agreements --accept-package-agreements
    goto :test
)

echo Проверяем наличие chocolatey...
where choco >nul 2>&1
if %errorlevel% equ 0 (
    echo ✓ chocolatey найден, устанавливаем curl...
    choco install curl -y
    goto :test
)

echo.
echo ❌ Не найдены winget или chocolatey
echo.
echo === РУЧНАЯ УСТАНОВКА ===
echo.
echo Скачайте curl с официального сайта:
echo https://curl.se/windows/
echo.
echo 1. Перейдите по ссылке выше
echo 2. Скачайте zip архив для Windows
echo 3. Распакуйте в папку (например, C:\curl)
echo 4. Добавьте в PATH: C:\curl\bin
echo.
echo Или установите Git for Windows - curl идет в комплекте:
echo https://gitforwindows.org/
echo.

goto :end

:test
echo.
echo === ТЕСТИРОВАНИЕ ===
where curl >nul 2>&1
if %errorlevel% equ 0 (
    echo ✓ curl успешно установлен
    echo Версия:
    curl --version | findstr "curl"
    echo.
    echo === ТЕСТ ЗАПРОСА ===
    echo Тестируем curl на api.erkapharm.com:
    curl -k -I https://api.erkapharm.com --connect-timeout 5 2>nul | findstr "HTTP/"
    if %errorlevel% equ 0 (
        echo ✓ curl работает корректно
    ) else (
        echo ❌ curl не может подключиться (возможно, нужен VPN)
    )
) else (
    echo ❌ curl не найден после установки
    echo Попробуйте перезапустить командную строку
)

:end
echo.
pause
























