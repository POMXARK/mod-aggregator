#!/bin/bash

# Простое подключение к ErkaPharm VPN без лишних опций
VPN_IP="89.107.140.6"

echo "=== Простое подключение ErkaPharm VPN ==="
echo "IP: $VPN_IP"
echo "Группа: RKF"
echo "Пользователь: dmitriy.maksimov"
echo ""

echo "🚀 Подключаемся к VPN..."
echo "Пароль: QQLnT82CHjO1"
echo "Нажми Ctrl+C для отключения"
echo ""

# Простое подключение - пусть openconnect сам запросит пароль и сертификат
sudo openconnect \
    --authgroup RKF \
    --user dmitriy.maksimov \
    $VPN_IP

echo ""
echo "VPN отключен"
