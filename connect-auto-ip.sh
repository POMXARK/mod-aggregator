#!/bin/bash

# Автоматическое подключение к ErkaPharm VPN по IP
# Обход всех DNS проблем

VPN_IP="89.107.140.6"
VPN_HOST="vpn.erkapharm.com"

echo "=== Автоматическое подключение ErkaPharm VPN ==="
echo "IP: $VPN_IP (обход DNS проблем)"
echo "Группа: RKF"
echo "Пользователь: dmitriy.maksimov"
echo ""

# Проверяем сеть (HTTPS, не ping)
echo "Проверяю сеть..."
timeout 5 bash -c "</dev/tcp/$VPN_IP/443" >/dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✓ VPN сервер доступен (HTTPS)"
else
    echo "❌ VPN сервер недоступен"
    echo "Проверь интернет соединение"
    exit 1
fi

echo ""
echo "🚀 Подключаюсь к VPN..."
echo "Пароль будет введен автоматически"
echo "Нажми Ctrl+C для отключения"
echo ""

# Подключаемся по IP с автоматическим вводом пароля
echo "QQLnT82CHjO1" | sudo openconnect \
    --authgroup RKF \
    --user dmitriy.maksimov \
    --passwd-on-stdin \
    --servercert pin-sha256:9O77kp/OtuHwv0+lmcS6KSnKos4tJHqbURPVcWVOdEg= \
    $VPN_IP

echo ""
echo "VPN отключен"
