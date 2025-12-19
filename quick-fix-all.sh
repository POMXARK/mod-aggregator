#!/bin/bash

# Быстрое исправление всех проблем WSL2 VPN
echo "=== Быстрое исправление WSL2 VPN ==="
echo ""

# 1. Создаем resolv.conf
echo "1. Создание DNS конфигурации..."
sudo rm -f /etc/resolv.conf
sudo bash -c 'cat > /etc/resolv.conf << EOF
nameserver 8.8.8.8
nameserver 8.8.4.4
nameserver 1.1.1.1
EOF'
echo "✓ DNS настроен"

# 2. Проверяем sudo для openconnect
echo ""
echo "2. Проверка sudo для openconnect..."
if sudo -n true 2>/dev/null; then
    echo "✓ sudo NOPASSWD работает"
else
    echo "⚠️  sudo требует пароль"
    echo "Запусти: sudo ./setup-sudo-nopasswd.sh"
fi

# 3. Тест DNS
echo ""
echo "3. Тест DNS..."
if nslookup google.com >/dev/null 2>&1; then
    echo "✓ DNS работает"
else
    echo "❌ DNS не работает"
    echo "Используй подключение по IP"
fi

# 4. Тест VPN сервера
echo ""
echo "4. Тест VPN сервера..."
VPN_IP="89.107.140.6"
if ping -c 1 $VPN_IP >/dev/null 2>&1; then
    echo "✓ VPN сервер доступен ($VPN_IP)"
else
    echo "❌ VPN сервер недоступен"
fi

echo ""
echo "=== Готово! ==="
echo ""
echo "Для подключения:"
if nslookup google.com >/dev/null 2>&1; then
    echo "./connect-rkf-manual.sh"
else
    echo "./connect-auto-ip.sh  # (обход DNS)"
fi
echo ""
echo "Затем в Windows: .\\setup-routing.ps1"
