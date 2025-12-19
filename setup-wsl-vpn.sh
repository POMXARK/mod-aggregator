#!/bin/bash

# Скрипт настройки WSL2 для VPN с SOCKS5 прокси
# Запускать с sudo: sudo ./setup-wsl-vpn.sh

echo "=== Обновление системы ==="
apt update && apt upgrade -y

echo "=== Установка необходимых пакетов ==="
# OpenConnect для Cisco AnyConnect
# OpenVPN для OpenVPN соединений
# Dante для SOCKS5 прокси
# Network tools для диагностики
apt install -y openconnect openvpn dante-server curl wget net-tools iproute2

echo "=== Настройка SOCKS5 прокси ==="
# Создаем конфиг для Dante
cat > /etc/danted.conf << 'EOF'
logoutput: /var/log/dante.log
internal: 0.0.0.0 port = 1080
external: eth0
clientmethod: none
socksmethod: none
user.privileged: root
user.unprivileged: nobody
client pass {
    from: 0.0.0.0/0 to: 0.0.0.0/0
}
socks pass {
    from: 0.0.0.0/0 to: 0.0.0.0/0
}
EOF

echo "=== Запуск Dante SOCKS5 прокси ==="
# В WSL2 systemctl не работает, запускаем напрямую
sudo danted &
sleep 2

echo "=== Проверка статуса ==="
if pgrep -x "danted" > /dev/null; then
    echo "✓ Dante SOCKS5 прокси запущен"
else
    echo "✗ Не удалось запустить Dante"
    exit 1
fi

netstat -tlnp | grep :1080

echo "=== Настройка завершена! ==="
echo "SOCKS5 прокси доступен на localhost:1080"
echo "Из Windows подключайся к: 127.0.0.1:1080"
echo ""
echo "Примеры использования:"
echo "1. OpenConnect: openconnect --socks-proxy localhost:1080 vpn.example.com"
echo "2. OpenVPN: добавь в конфиг: socks-proxy 127.0.0.1 1080"
