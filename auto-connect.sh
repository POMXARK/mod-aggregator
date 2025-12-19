#!/bin/bash

# Полностью автоматическое подключение к ErkaPharm VPN
echo "=== Автоматическое подключение ErkaPharm VPN ==="

# 1. Создаем DNS если нужно
if [ ! -f /etc/resolv.conf ] || ! grep -q "nameserver" /etc/resolv.conf; then
    echo "Создаю DNS конфигурацию..."
    sudo rm -f /etc/resolv.conf
    sudo bash -c 'cat > /etc/resolv.conf << EOF
nameserver 8.8.8.8
nameserver 8.8.4.4
nameserver 1.1.1.1
EOF'
    echo "✓ DNS настроен"
fi

# 2. Проверяем и настраиваем sudo
if ! sudo -n true 2>/dev/null; then
    echo "Настраиваю sudo NOPASSWD..."
    ./setup-sudo-nopasswd.sh
fi

# 3. Выбираем метод подключения
VPN_IP="89.107.140.6"
echo ""
echo "Проверяю DNS..."
if nslookup vpn.erkapharm.com >/dev/null 2>&1; then
    echo "✓ DNS работает - использую домен"
    CONNECT_CMD="./connect-rkf-direct.sh"
else
    echo "❌ DNS не работает - использую IP"
    CONNECT_CMD="./connect-auto-ip.sh"
fi

# 4. Тест VPN сервера
echo ""
echo "Проверяю VPN сервер..."
if ping -c 1 $VPN_IP >/dev/null 2>&1; then
    echo "✓ VPN сервер доступен"
else
    echo "❌ VPN сервер недоступен"
    echo "Проверь интернет соединение"
    exit 1
fi

# 5. Подключаемся
echo ""
echo "🚀 Подключаюсь к VPN..."
echo "После подключения настрой маршрутизацию в Windows"
echo "Нажми Ctrl+C для отключения"
echo ""

$CONNECT_CMD
