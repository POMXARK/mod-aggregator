# 🚀 Запуск ErkaPharm VPN

## ⚡ НОВЫЙ ПОДХОД - Прямое подключение

OpenConnect не поддерживает SOCKS5 прокси. Используем прямое подключение с маршрутизацией.

## ⚡ Быстрый старт (3 шага)

```bash
# 1. Зайди в WSL
wsl -d Ubuntu
cd /mnt/c/Users/User/mod-aggregator

# 2. Подключи VPN
./connect-rkf-direct.sh

# 3. В Windows настрой маршрутизацию
# Запусти setup-routing.ps1 от имени администратора
```

## 📋 Пошагово

### Шаг 1: Первый запуск (установка)
```bash
sudo ./setup-wsl-vpn.sh
# В WSL2 systemctl не работает, поэтому скрипт запускает danted напрямую
```

### Шаг 2: Подключение VPN
```bash
./connect-rkf.sh
```

### Шаг 3: Настройка в Windows
В браузере или системных настройках:
- **SOCKS5 прокси**: `127.0.0.1`
- **Порт**: `1080`

## 🔧 Альтернативные команды

### Прямое подключение
```bash
./vpn-connect.sh openconnect vpn.erkapharm.com --authgroup RKF --user dmitriy.maksimov
```

### Ручная команда (как у тебя)
```bash
sudo openconnect --authgroup RKF vpn.erkapharm.com --user dmitriy.maksimov --passwd-on-stdin <<< "QQLnT82CHjO1"
```

### С SOCKS5 прокси
```bash
openconnect --socks-proxy localhost:1080 --authgroup RKF vpn.erkapharm.com --user dmitriy.maksimov --passwd-on-stdin <<< "QQLnT82CHjO1"
```

## ✅ Проверка работы

После подключения:
- В браузере Windows перейди на `whatismyipaddress.com`
- Должен показаться IP ErkaPharm

## 🛑 Отключение VPN

Нажми `Ctrl+C` в терминале WSL или:
```bash
pkill openconnect
```

## 📁 Файлы проекта

- `quick-start.sh` - быстрый старт всего
- `setup-wsl-vpn.sh` - установка компонентов
- `connect-rkf.sh` - подключение к ErkaPharm
- `test-vpn-bridge.sh` - тестирование моста
- `README-WSL-VPN.md` - подробная документация
