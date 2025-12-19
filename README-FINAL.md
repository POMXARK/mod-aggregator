# 🎯 ErkaPharm VPN - Полное руководство

## 🚨 Текущие проблемы и решения

### 1. **DNS не работает в WSL2**
```
getaddrinfo failed for host 'vpn.erkapharm.com': Temporary failure in name resolution
```
**Решение:** Использовать IP адрес или исправить DNS

### 2. **OpenConnect требует root**
```
To configure local networking, openconnect must be running as root
```
**Решение:** Настроить sudo NOPASSWD

### 3. **SOCKS5 не поддерживается**
```
unrecognized option '--socks-proxy'
```
**Решение:** Прямое подключение + маршрутизация Windows

## 🚀 Полная настройка (шаг за шагом)

### Шаг 1: Первый запуск - настройка
```bash
wsl -d Ubuntu
cd /mnt/c/Users/User/mod-aggregator

# Сделай скрипты исполняемыми
chmod +x *.sh

# Настрой sudo без пароля
sudo ./setup-sudo-nopasswd.sh
```

### Шаг 2: Исправь DNS (если проблемы)
```bash
./fix-dns.sh
# Перезапусти WSL
wsl --shutdown && wsl -d Ubuntu
```

### Шаг 3: Протестируй сеть
```bash
./diagnose-network.sh
./test-vpn-server.sh
```

### Шаг 4: Подключи VPN
```bash
# Если DNS работает:
./connect-rkf-manual.sh

# Если DNS не работает:
./connect-rkf-ip.sh
```

### Шаг 5: Настрой маршрутизацию Windows

**Вариант 1: Весь трафик через VPN**
```powershell
cd C:\Users\User\mod-aggregator
.\setup-routing.ps1
```

**Вариант 2: Только ErkaPharm сети (рекомендуется)**
```powershell
.\setup-selective-routing.ps1
```
или
```cmd
.\setup-selective-routing.bat
```

## 📁 Все скрипты

| Скрипт | Назначение |
|--------|------------|
| `setup-sudo-nopasswd.sh` | Настройка sudo без пароля |
| `fix-dns.sh` | Исправление DNS проблем |
| `diagnose-network.sh` | Диагностика сети WSL2 |
| `test-vpn-server.sh` | Тест подключения к VPN |
| `connect-rkf-manual.sh` | VPN с ручным вводом |
| `connect-rkf-ip.sh` | VPN по IP (обход DNS) |
| `check-tun.sh` | Диагностика TUN устройства |
| `setup-routing.ps1` | Маршрутизация Windows |
| `test-routing.ps1` | Проверка маршрутизации |

## 🔍 Диагностика проблем

### DNS не работает
```bash
./diagnose-network.sh
./fix-dns.sh
wsl --shutdown && wsl -d Ubuntu
```

### TUN устройство не создается
```bash
./check-tun.sh
sudo ./setup-sudo-nopasswd.sh
```

### VPN не подключается
```bash
./test-vpn-server.sh
# Попробуй ./connect-rkf-ip.sh
```

## 💡 Быстрый старт (для опытных)

```bash
# Один раз:
sudo ./setup-sudo-nopasswd.sh
./fix-dns.sh
wsl --shutdown && wsl -d Ubuntu

# Каждый раз:
./connect-rkf-manual.sh
# Потом в Windows: .\setup-routing.ps1
```

## ✅ Проверка работы

После подключения:
- В WSL: `curl ifconfig.me` → IP ErkaPharm
- В Windows: зайди на `whatismyipaddress.com` → IP ErkaPharm

## 🛑 Отключение

### VPN:
`Ctrl+C` в WSL терминале

### Маршрутизация:
```powershell
route delete 0.0.0.0 mask 0.0.0.0
```

## 🎉 Готово!

Теперь у тебя полнофункциональная VPN система ErkaPharm в WSL2 с маршрутизацией в Windows! 🚀
