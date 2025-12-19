@echo off
echo ========================================
echo      ОТКРЫТИЕ API В БРАУЗЕРЕ
echo ========================================
echo.

REM Проверяем SSH туннель
echo Проверяем SSH туннель...
netstat -ano | findstr ":9443" >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ SSH туннель не найден (порт 9443)
    echo.
    echo Запускаем туннель...
    wsl -d Ubuntu -- bash -c "cd /mnt/c/Users/User/mod-aggregator && ./start-ssh-tunnel.sh" >nul 2>&1
    timeout /t 3 >nul
)

REM Проверяем ещё раз
netstat -ano | findstr ":9443" >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ Не удалось запустить SSH туннель
    echo.
    echo Запустите вручную в WSL:
    echo cd /mnt/c/Users/User/mod-aggregator
    echo ./start-ssh-tunnel.sh
    pause
    exit /b 1
)

echo ✓ SSH туннель работает

echo.
echo ========================================
echo     ОТКРЫТИЕ API В БРАУЗЕРЕ
echo ========================================
echo.
echo Открываем браузер...
echo.
echo 🔗 Ссылка: https://localhost:9443
echo.
echo ⚠️  В браузере:
echo    1. Примите SSL предупреждение
echo    2. Нажмите "Дополнительно" → "Перейти на сайт"
echo    3. Должен загрузиться Swagger UI
echo.
echo 📖 Если 403 Forbidden - обновите страницу (Ctrl+F5)
echo.

REM Открываем браузер
start https://localhost:9443

REM Открываем HTML файл с инструкциями
start api-browser.html

echo.
echo Готово! Проверьте браузер.
echo.
pause

