#!/bin/bash

# Подключение к ErkaPharm VPN без проверки сертификата
# Используем --insecure для обхода проблем с сертификатом

VPN_IP="89.107.140.6"

echo "=== Подключение ErkaPharm VPN (без проверки сертификата) ==="
echo "IP: $VPN_IP"
echo "⚠️  ВНИМАНИЕ: Без проверки сертификата (менее безопасно)"
echo ""

echo "🚀 Подключаемся к VPN..."
echo "Пароль будет введен автоматически"
echo "Нажми Ctrl+C для отключения"
echo ""

# Подключаемся без проверки сертификата
echo "QQLnT82CHjO1" | sudo openconnect \
    --authgroup RKF \
    --user dmitriy.maksimov \
    --passwd-on-stdin \
    --no-cert-check \
    $VPN_IP

echo ""
echo "VPN отключен"
