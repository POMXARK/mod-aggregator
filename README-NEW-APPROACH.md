# 🔄 Новый подход к VPN в WSL2

## 🚨 Проблемы с WSL2

1. **OpenConnect НЕ поддерживает SOCKS5 прокси** (опция `--socks-proxy` не существует)
2. **OpenConnect требует root прав** для создания TUN устройства в WSL2
3. **Systemctl не работает** в WSL2 (используем прямой запуск служб)

## ✅ Решение: Прямое подключение + маршрутизация

1. **VPN подключается напрямую в WSL**
2. **Маршрутизация перенаправляет трафик из Windows через WSL VPN**

## 🚀 Как использовать

### Шаг 1: Настрой sudo (один раз)
```bash
wsl -d Ubuntu
cd /mnt/c/Users/User/mod-aggregator
sudo ./setup-sudo-nopasswd.sh
```

### Шаг 2: Подключи VPN в WSL
```bash
./connect-rkf-manual.sh
# Или ./connect-rkf-direct.sh (после настройки sudo NOPASSWD)
```

### Шаг 2: Настрой маршрутизацию в Windows
Открой PowerShell **от имени администратора**:
```powershell
cd C:\Users\User\mod-aggregator
.\setup-routing.ps1
```

### Шаг 3: Проверь работу
- Зайди на `whatismyipaddress.com`
- IP должен быть из сети ErkaPharm

## 📁 Файлы

- `connect-rkf-direct.sh` - прямое подключение к VPN
- `setup-routing.ps1` - настройка маршрутизации Windows
- `commands-only.txt` - краткие команды

## 🔧 Как работает маршрутизация

1. **WSL получает VPN интерфейс** (обычно tun0)
2. **Windows добавляет маршрут**: весь трафик → WSL IP
3. **WSL перенаправляет трафик через VPN**

## 🛑 Отключение

### VPN:
- В WSL терминале нажми `Ctrl+C`

### Маршрутизация:
```powershell
# В PowerShell от имени администратора
route delete 0.0.0.0 mask 0.0.0.0
```

## 💡 Преимущества нового подхода

- ✅ Работает с любой версией OpenConnect
- ✅ Не требует SOCKS5 прокси
- ✅ Простая настройка
- ✅ Надежная маршрутизация

## 🔍 Диагностика

### Проверить VPN интерфейс:
```bash
ip addr show
# Ищи tun0 или подобный интерфейс
```

### Проверить маршруты Windows:
```cmd
route print
```

### Проверить подключение:
```bash
curl ifconfig.me
# Должен вернуть IP ErkaPharm
```
