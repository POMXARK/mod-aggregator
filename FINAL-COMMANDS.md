# 🎯 ФИНАЛЬНЫЕ КОМАНДЫ - ErkaPharm VPN

## ✅ ГОТОВАЯ СИСТЕМА:

**БЫСТРЫЙ ЗАПУСК:**
```cmd
cd C:\Users\User\mod-aggregator
.\start-all.bat
```

**БЫСТРАЯ ОСТАНОВКА:**
```cmd
.\stop-all.bat
```

## ✅ Что работает:

- OpenConnect v8.05-1 НЕ поддерживает SOCKS5 прокси
- Решение: прямое подключение VPN + селективная маршрутизация Windows

## 🚀 ЗАПУСК (3 шага):

### Шаг 1: Настрой sudo (один раз)
```bash
wsl -d Ubuntu
cd /mnt/c/Users/User/mod-aggregator
sudo ./setup-sudo-nopasswd.sh
```

### Шаг 2: Исправь DNS (если проблемы)
```bash
./fix-dns.sh
# Потом перезапусти WSL: wsl --shutdown && wsl -d Ubuntu
```

### Шаг 3: Подключи VPN в WSL
```bash
./connect-rkf-manual.sh
# Или по IP (если DNS не работает):
./connect-rkf-ip.sh
# Или автоматический вариант после настройки sudo NOPASSWD:
# ./connect-rkf-direct.sh
```

### Шаг 2: Настрой маршрутизацию в Windows
Открой **PowerShell от имени администратора**:
```powershell
cd C:\Users\User\mod-aggregator
.\setup-routing.ps1
```

## 🔍 ПРОВЕРКА:
```powershell
.\test-routing.ps1
```
Или зайди на `whatismyipaddress.com`

## 🛑 ОТКЛЮЧЕНИЕ:

### VPN:
В WSL терминале нажми `Ctrl+C`

### Маршрутизация:
```powershell
route delete 0.0.0.0 mask 0.0.0.0
```

## 📁 ФАЙЛЫ ПРОЕКТА:

### ОСНОВНЫЕ СКРИПТЫ:
- `start-all.bat` - **АВТОМАТИЧЕСКИЙ ЗАПУСК ВСЕЙ СИСТЕМЫ**
- `stop-all.bat` - **АВТОМАТИЧЕСКАЯ ОСТАНОВКА ВСЕЙ СИСТЕМЫ**
- `auto-connect.sh` - автоматическое подключение VPN
- `setup-selective-admin.bat` - настройка маршрутизации

### ДИАГНОСТИКА:
- `test-full-system.bat` - полный тест системы
- `check-vpn-status.sh` - проверка VPN статуса
- `diagnose-network.sh` - диагностика сети

### ДОКУМЕНТАЦИЯ:
- `STARTUP-GUIDE.md` - подробный гайд запуска/остановки
- `QUICK-START-CHEATSHEET.md` - шпаргалка команд
- `README-FINAL.md` - финальное руководство
