@echo off
echo === Диагностика проблемы OAuth с localhost ===
echo.

echo 1. Проверка приложения на порту 53920...
curl -s http://localhost:53920/ >nul 2>&1
if %errorlevel% equ 0 (
    echo   ✓ Приложение отвечает
) else (
    echo   ✗ Приложение не отвечает
    echo   Возможно, Antigravity не запущено
    goto :end
)

echo.
echo 2. Тестирование подключения...
echo   Попробуйте открыть в Mullvad Browser:
echo   http://localhost:53920/oauth-callback?test=1
echo.
echo   Если не работает, попробуйте:
echo   http://127.0.0.1:53920/oauth-callback?test=1

echo.
echo === РЕКОМЕНДАЦИИ ===
echo.
echo ВАРИАНТ 1: Изменить DNS настройки в Mullvad Browser
echo   1. Настройки → Privacy & Security → DNS Protection
echo   2. Изменить "Max Protection" на "Standard"
echo   3. Перезапустить браузер
echo   4. Повторить авторизацию
echo.
echo ВАРИАНТ 2: Использовать обычный Chrome для OAuth
echo   1. Открыть Google Chrome
echo   2. Перейти по ссылке авторизации
echo   3. Завершить процесс там
echo.
echo ВАРИАНТ 3: Временно отключить DNS over HTTPS
echo   1. В Mullvad Browser отключить "Enable DNS over HTTPS"
echo   2. Попробовать авторизацию
echo   3. Включить обратно после
echo.
echo ВАРИАНТ 4: Проверить настройки NoScript/расширений
echo   1. Если установлены блокировщики, добавить исключение для localhost
echo.

:end
echo.
pause

