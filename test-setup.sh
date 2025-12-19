#!/bin/bash

# Тестовый скрипт для проверки установки
echo "=== Тестирование установки ErkaPharm VPN ==="

# Проверка команд
echo "1. Проверка наличия команд:"
commands=("openconnect" "danted" "curl")
for cmd in "${commands[@]}"; do
    if command -v "$cmd" &> /dev/null; then
        echo "  ✓ $cmd установлен"
    else
        echo "  ❌ $cmd не найден"
    fi
done
echo ""

# Проверка SOCKS5
echo "2. Проверка SOCKS5 прокси:"
if pgrep -x "danted" > /dev/null; then
    echo "  ✓ danted запущен"
else
    echo "  ❌ danted не запущен"
fi

if netstat -tln 2>/dev/null | grep -q ":1080"; then
    echo "  ✓ Порт 1080 прослушивается"
else
    echo "  ❌ Порт 1080 не прослушивается"
fi
echo ""

# Тест подключения
echo "3. Тест SOCKS5 подключения:"
if curl --socks5 localhost:1080 -s ifconfig.me > /dev/null 2>&1; then
    IP=$(curl --socks5 localhost:1080 -s ifconfig.me)
    echo "  ✓ SOCKS5 работает, текущий IP: $IP"
else
    echo "  ❌ SOCKS5 не работает"
fi

echo ""
echo "=== Тест завершен ==="
