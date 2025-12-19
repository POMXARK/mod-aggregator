#!/bin/bash

# Создание resolv.conf для WSL2
echo "=== Создание /etc/resolv.conf ==="

# Удаляем старый файл
sudo rm -f /etc/resolv.conf

# Создаем новый с несколькими DNS серверами
sudo bash -c 'cat > /etc/resolv.conf << EOF
nameserver 8.8.8.8
nameserver 8.8.4.4
nameserver 1.1.1.1
nameserver 208.67.222.222
EOF'

echo "✓ Создан /etc/resolv.conf"
cat /etc/resolv.conf

echo ""
echo "Тестирую DNS..."
nslookup google.com >/dev/null && echo "✓ DNS работает" || echo "❌ DNS все еще не работает"
