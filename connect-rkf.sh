#!/bin/bash

# Скрипт для подключения к VPN ErkaPharm (RKF группа)
# На основе предоставленной команды пользователя

echo "=== Подключение к ErkaPharm VPN (RKF) ==="
echo "Сервер: vpn.erkapharm.com"
echo "Группа: RKF"
echo "Пользователь: dmitriy.maksimov"
echo ""

# Проверяем что SOCKS5 прокси запущен (для WSL без systemd)
if ! pgrep -x "danted" > /dev/null; then
    echo "❌ SOCKS5 прокси не запущен!"
    echo "Запускаю автоматически..."
    sudo danted
    sleep 2

    if ! pgrep -x "danted" > /dev/null; then
        echo "❌ Не удалось запустить danted!"
        echo "Запусти вручную: sudo danted"
        exit 1
    fi
fi

echo "✅ SOCKS5 прокси работает"
echo ""

# Подключаемся к VPN с SOCKS5 прокси
echo "🚀 Подключаемся к VPN..."
openconnect \
    --socks-proxy localhost:1080 \
    --authgroup RKF \
    --user dmitriy.maksimov \
    --passwd-on-stdin \
    vpn.erkapharm.com <<< "QQLnT82CHjO1"

echo ""
echo "VPN отключен"
