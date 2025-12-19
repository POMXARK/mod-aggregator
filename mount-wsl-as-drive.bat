@echo off
echo === Монтирование WSL Ubuntu как сетевой диск ===

REM Проверяем что WSL запущен
wsl -l -v | findstr Ubuntu | findstr Running >nul
if %errorlevel% neq 0 (
    echo ❌ WSL Ubuntu не запущен
    echo Запустите: wsl -d Ubuntu
    pause
    exit /b 1
)

echo ✓ WSL Ubuntu запущен

REM Создаем сетевой диск W:
echo Создаем сетевой диск W: для WSL...
net use W: "\\wsl$\Ubuntu" /persistent:yes 2>nul

if %errorlevel% equ 0 (
    echo ✓ Диск W: успешно создан
    echo Теперь можно открывать W: в проводнике
) else (
    echo ❌ Ошибка создания диска W:
    echo Возможно, диск W: уже используется
    echo Попробуйте другую букву диска
)

echo.
echo === ДОСТУП К ФАЙЛАМ ===
echo W:\ - корень WSL Ubuntu
echo W:\mnt\c\Users\User\mod-aggregator - ваша папка проекта
echo.

echo === УПРАВЛЕНИЕ ДИСКОМ ===
echo Отключить диск: net use W: /delete
echo Проверить диски: net use
echo.

pause
