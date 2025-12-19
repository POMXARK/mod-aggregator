#!/bin/bash

# Прямое подключение к ErkaPharm VPN без SOCKS5 прокси
# VPN будет работать напрямую в WSL

echo "=== Прямое подключение к ErkaPharm VPN (RKF) ==="
echo "Сервер: vpn.erkapharm.com"
echo "Группа: RKF"
echo "Пользователь: dmitriy.maksimov"
echo ""

echo "🚀 Подключаемся к VPN..."
echo "После подключения настрой маршрутизацию в Windows"
echo "Нажми Ctrl+C для отключения"
echo ""

# Подключаемся с sudo (нужно для создания TUN устройства)
echo "Запускаем с правами root для создания TUN устройства..."
sudo openconnect \
    --authgroup RKF \
    --user dmitriy.maksimov \
    --passwd-on-stdin \
    vpn.erkapharm.com <<< "QQLnT82CHjO1"

echo ""
echo "VPN отключен"
