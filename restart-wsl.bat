@echo off
REM Перезапуск WSL из Windows
echo Останавливаю WSL...
wsl --shutdown

echo Запускаю Ubuntu...
wsl -d Ubuntu
