# Настройка WSL2 VPN с SOCKS5 мостом в Windows 11

## Быстрый старт

### 1. Установка компонентов в WSL
```bash
# Сделай скрипт исполняемым и запусти с sudo
chmod +x setup-wsl-vpn.sh
sudo ./setup-wsl-vpn.sh
```

### 2. Тестирование моста
```bash
./test-vpn-bridge.sh
```

### 3. Подключение VPN
```bash
# OpenConnect (Cisco AnyConnect)
./vpn-connect.sh openconnect vpn.company.com --user=your_username

# OpenVPN
./vpn-connect.sh openvpn your_config.ovpn
```

## Настройка в Windows 11

### Браузер (Chrome/Firefox)
- Настройки → Дополнительные → Система → Открыть параметры прокси
- Включить "Использовать прокси-сервер"
- Адрес: `127.0.0.1`, Порт: `1080`, Тип: SOCKS5

### Системный прокси (для всех приложений)
```powershell
# В PowerShell от имени администратора:
netsh winhttp set proxy proxy-server="socks5://127.0.0.1:1080"
```

### Отключение системного прокси
```powershell
netsh winhttp reset proxy
```

## Примеры использования

### OpenConnect с аутентификацией
```bash
./vpn-connect.sh openconnect vpn.company.com \
  --user=myuser \
  --no-cert-check \
  --servercert pin-sha256:cert_hash_here
```

### OpenVPN с конфигом
```bash
# Предварительно положи конфиг в папку с скриптами
./vpn-connect.sh openvpn client.ovpn
```

### Проверка работы
```bash
# В WSL
curl --socks5 localhost:1080 ifconfig.me

# В браузере Windows
# Перейди на whatismyipaddress.com
# Должен показывать IP VPN сервера
```

## Управление VPN

### Просмотр активных соединений
```bash
ps aux | grep -E "(openconnect|openvpn)"
```

### Отключение VPN
```bash
# Найди PID процесса
ps aux | grep -E "(openconnect|openvpn)"

# Убей процесс
kill <PID>
```

### Перезапуск SOCKS5 прокси
```bash
sudo systemctl restart danted
```

## Структура файлов

```
├── setup-wsl-vpn.sh      # Установка и настройка
├── vpn-connect.sh        # Подключение VPN
├── test-vpn-bridge.sh    # Тестирование
└── README-WSL-VPN.md     # Эта документация
```

## Устранение неполадок

### SOCKS5 прокси не работает
```bash
# Проверь статус
sudo systemctl status danted

# Проверь логи
sudo journalctl -u danted -f

# Перезапусти
sudo systemctl restart danted
```

### VPN не подключается
```bash
# Проверь маршрут
ip route show

# Проверь DNS
cat /etc/resolv.conf

# Тест подключения без VPN
curl ifconfig.me
```

### Windows не видит прокси
- Убедись что WSL запущен: `wsl -l -v`
- Проверь порт: `netstat -ano | findstr :1080`
- Firewall: разреши порт 1080 в Windows Defender

## Продвинутые настройки

### Несколько VPN одновременно
```bash
# Запусти несколько экземпляров с разными портами
# Измени порт в danted.conf и vpn-connect.sh
```

### Автозапуск при старте WSL
```bash
# Добавь в ~/.bashrc
sudo systemctl start danted
```

### Кастомные маршруты
```bash
# Только трафик к определенным IP через VPN
ip route add 192.168.1.0/24 dev tun0
```
