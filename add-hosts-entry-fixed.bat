@echo off
chcp 65001 >nul
echo === Добавление записи в hosts файл ===

REM Проверяем права администратора
net session >nul 2>&1
if %errorLevel% == 0 (
    echo ✓ Запущено от имени администратора
) else (
    echo ❌ Требуются права администратора!
    echo Запустите командную строку ОТ ИМЕНИ АДМИНИСТРАТОРА
    echo Или щелкните правой кнопкой по файлу и "Запуск от имени администратора"
    pause
    exit /b 1
)

REM Путь к hosts файлу
set HOSTS_FILE=%windir%\System32\drivers\etc\hosts

echo Добавляем запись: 127.0.0.1 api.erkapharm.com

REM Создаем резервную копию
copy "%HOSTS_FILE%" "%HOSTS_FILE%.backup" >nul 2>&1

REM Проверяем, существует ли уже такая запись
findstr /C:"127.0.0.1 api.erkapharm.com" "%HOSTS_FILE%" >nul 2>&1
if %errorlevel% equ 0 (
    echo ⚠️  Запись уже существует в hosts файле
    goto :check
) else (
    REM Добавляем запись
    echo. >> "%HOSTS_FILE%"
    echo # ErkaPharm API tunnel >> "%HOSTS_FILE%"
    echo 127.0.0.1 api.erkapharm.com >> "%HOSTS_FILE%"

    if %errorlevel% equ 0 (
        echo ✓ Запись успешно добавлена
    ) else (
        echo ❌ Ошибка добавления записи
        REM Восстанавливаем резервную копию
        copy "%HOSTS_FILE%.backup" "%HOSTS_FILE%" >nul 2>&1
        pause
        exit /b 1
    )
)

:check
echo.
echo === ПРОВЕРКА ===
echo Текущий hosts файл:
findstr /C:"api.erkapharm.com" "%HOSTS_FILE%"

echo.
echo === ТЕСТИРОВАНИЕ ===
echo Теперь попробуйте открыть в браузере:
echo https://api.erkapharm.com
echo.
echo Если API заработает - отлично!
echo Если нет - возможно нужны дополнительные headers или аутентификация.

pause
























