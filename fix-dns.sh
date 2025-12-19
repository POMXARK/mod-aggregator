#!/bin/bash

# Исправление DNS проблем в WSL2
echo "=== Исправление DNS в WSL2 ==="

# Вариант 1: Установить статический DNS
echo "1. Устанавливаю статический DNS (Google)..."
sudo bash -c 'cat > /etc/resolv.conf << EOF
nameserver 8.8.8.8
nameserver 8.8.4.4
nameserver 1.1.1.1
EOF'
echo "✓ DNS настроен на 8.8.8.8"
echo ""

# Вариант 2: Проверить и создать /etc/wsl.conf
echo "2. Настройка WSL конфигурации..."
if [ ! -f /etc/wsl.conf ]; then
    sudo bash -c 'cat > /etc/wsl.conf << EOF
[network]
generateResolvConf = false
EOF'
    echo "✓ Создан /etc/wsl.conf"
else
    echo "✓ /etc/wsl.conf уже существует"
fi
echo ""

# Вариант 3: Тест DNS
echo "3. Тестирую DNS..."
sleep 2

echo "Проверяю google.com..."
nslookup google.com >/dev/null 2>&1 && echo "✓ DNS работает" || echo "❌ DNS все еще не работает"

echo ""
echo "=== Следующие шаги ==="
echo "1. Перезапусти WSL: wsl --shutdown && wsl -d Ubuntu"
echo "2. Проверь: ./diagnose-network.sh"
echo "3. Если не помогло - попробуй IP адрес VPN сервера"
echo ""
echo "IP адрес vpn.erkapharm.com можно узнать командой:"
echo "nslookup vpn.erkapharm.com 8.8.8.8"
