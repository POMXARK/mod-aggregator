#!/bin/bash

# Ручное подключение к ErkaPharm VPN с sudo
# Пароль VPN вводится вручную

echo "=== Ручное подключение ErkaPharm VPN ==="
echo "Сервер: vpn.erkapharm.com"
echo "Группа: RKF"
echo "Пользователь: dmitriy.maksimov"
echo ""
echo "⚠️  ВНИМАНИЕ: openconnect должен работать с правами root"
echo "   Если попросит sudo пароль - введи его"
echo "   Потом введи пароль VPN: QQLnT82CHjO1"
echo ""
echo "🚀 Запускаем подключение..."
echo ""

sudo openconnect \
    --authgroup RKF \
    --user dmitriy.maksimov \
    vpn.erkapharm.com

echo ""
echo "VPN отключен"
