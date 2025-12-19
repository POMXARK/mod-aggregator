#!/bin/bash

# Попытка подключения к VPN прямо сейчас
VPN_IP="89.107.140.6"

echo "=== Попытка подключения к VPN ==="
echo "Сервер отвечает на HTTPS - пробуем подключиться"
echo ""

# Проверяем еще раз доступность
echo "Проверка доступности сервера..."
if timeout 5 bash -c "</dev/tcp/$VPN_IP/443" >/dev/null; then
    echo "✓ Сервер доступен, пробуем подключиться"
    echo ""
else
    echo "❌ Сервер недоступен"
    exit 1
fi

# Подключаемся
echo "🚀 Подключаемся к VPN..."
echo "Пароль будет запрошен автоматически"
echo "Если зависнет - подожди 10-20 секунд"
echo ""

sudo openconnect \
    --authgroup RKF \
    --user dmitriy.maksimov \
    --servercert pin-sha256:9O77kp/OtuHwv0+lmcS6KSnKos4tJHqbURPVcWVOdEg= \
    $VPN_IP << 'EOF'
QQLnT82CHjO1
EOF

echo ""
echo "VPN отключен"
