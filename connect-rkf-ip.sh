#!/bin/bash

# Подключение к ErkaPharm VPN по IP адресу
# Обход проблем с DNS разрешением

VPN_IP="89.107.140.6"
VPN_HOST="vpn.erkapharm.com"

echo "=== Подключение ErkaPharm VPN по IP ==="
echo "IP адрес: $VPN_IP"
echo "Хост: $VPN_HOST"
echo "Группа: RKF"
echo "Пользователь: dmitriy.maksimov"
echo ""

echo "🚀 Подключаемся к VPN..."
echo "Используем IP адрес для обхода DNS проблем"
echo "Нажми Ctrl+C для отключения"
echo ""

# Подключаемся по IP с проверкой сертификата
sudo openconnect \
    --authgroup RKF \
    --user dmitriy.maksimov \
    --servercert pin-sha256:9O77kp/OtuHwv0+lmcS6KSnKos4tJHqbURPVcWVOdEg= \
    $VPN_IP

echo ""
echo "VPN отключен"
