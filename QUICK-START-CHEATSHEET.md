# ⚡ ШПАРГАЛКА: ЗАПУСК ERKAPHARM VPN

## 🚀 БЫСТРЫЙ ЗАПУСК (4 КОМАНДЫ)

```bash
# 0. Настроить SSH ключ (один раз)
wsl -d Ubuntu && cd /mnt/c/Users/User/mod-aggregator && ./setup-ssh-key.sh

# 1. Запустить WSL + VPN
wsl -d Ubuntu && cd /mnt/c/Users/User/mod-aggregator && ./auto-connect.sh

# 2. Настроить маршруты (в PowerShell от админа)
cd C:\Users\User\mod-aggregator && .\setup-selective-admin.bat

# 3. Запустить SSH туннель
cd /mnt/c/Users/User/mod-aggregator && ./start-ssh-tunnel.sh
```

## ✅ ПРОВЕРКА РАБОТЫ

```cmd
cd C:\Users\User\mod-aggregator && .\test-full-system.bat
```

## 🛑 БЫСТРАЯ ОСТАНОВКА

```bash
# Остановить VPN и SSH
pkill openconnect && pkill ssh
```

## 📋 ЧЕК-ЛИСТ ЗАПУСКА

- [ ] WSL: `wsl -d Ubuntu`
- [ ] VPN: `./auto-connect.sh`
- [ ] Маршруты: `.\setup-selective-admin.bat` (админ)
- [ ] SSH: длинная команда с -L портами
- [ ] Тест: `.\test-full-system.bat`
- [ ] Переменные окружения в проекте

## 🔧 ПОЛЕЗНЫЕ КОМАНДЫ

### Диагностика
```bash
# Статус VPN
wsl -d Ubuntu -- ip addr show tun0

# Маршруты Windows
route print | findstr "172.18"

# SSH процессы
ps aux | grep ssh
```

### Перезапуск компонентов
```bash
# Перезапуск WSL
wsl --shutdown && wsl -d Ubuntu

# Перезапуск маршрутов
.\setup-selective-admin.bat
```

### Очистка
```bash
# Удалить маршруты
route delete 10.0.0.0 mask 255.0.0.0
route delete 172.16.0.0 mask 255.240.0.0
route delete 172.21.0.0 mask 255.255.0.0
route delete 192.168.0.0 mask 255.255.0.0
```

## 📱 ДОСТУП К СЕРВИСАМ

После запуска доступны:
- **Basket API:** http://localhost:8194
- **MongoDB:** mongodb://localhost:9999
- **ManZana:** http://localhost:11006
- **Fndr:** localhost:9996
- **WS:** localhost:9998

## 🚨 ЕСЛИ ЧТО-ТО НЕ РАБОТАЕТ

1. Проверить VPN: `./check-vpn-status.sh`
2. Проверить маршруты: `route print`
3. Проверить SSH: `netstat -ano | findstr :8194`
4. Перезапустить: `wsl --shutdown && wsl -d Ubuntu`
