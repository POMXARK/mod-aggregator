#!/bin/bash

# Скрипт тестирования VPN моста через SOCKS5 прокси
echo "=== Тестирование VPN моста WSL2 -> Windows ==="

# Проверка статуса SOCKS5 прокси (WSL совместимо)
echo "1. Проверка SOCKS5 прокси..."
if pgrep -x "danted" > /dev/null; then
    echo "✓ Dante SOCKS5 прокси работает"
else
    echo "✗ Dante SOCKS5 прокси не запущен"
    echo "Запускаю автоматически..."
    sudo danted &
    sleep 2

    if pgrep -x "danted" > /dev/null; then
        echo "✓ Dante SOCKS5 прокси запущен"
    else
        echo "✗ Не удалось запустить прокси"
        echo "Запустите вручную: sudo danted"
        exit 1
    fi
fi

# Проверка прослушки порта
if netstat -tlnp 2>/dev/null | grep -q ":1080"; then
    echo "✓ Порт 1080 прослушивается"
else
    echo "✗ Порт 1080 не прослушивается"
    exit 1
fi

# Тест подключения к SOCKS5 из WSL
echo ""
echo "2. Тест подключения из WSL..."
if curl --socks5 localhost:1080 -s ifconfig.me > /dev/null 2>&1; then
    echo "✓ SOCKS5 прокси доступен из WSL"
    IP_WSL=$(curl --socks5 localhost:1080 -s ifconfig.me)
    echo "  Текущий IP через SOCKS5: $IP_WSL"
else
    echo "✗ SOCKS5 прокси недоступен из WSL"
fi

# Информация для Windows
echo ""
echo "3. Инструкция для Windows:"
echo "В браузере или системных настройках укажи SOCKS5 прокси:"
echo "  Адрес: 127.0.0.1"
echo "  Порт: 1080"
echo ""
echo "Тест в браузере: https://whatismyipaddress.com/"
echo ""
echo "4. Тест с VPN:"
echo "Подключи VPN через скрипт vpn-connect.sh"
echo "Затем проверь IP через браузер в Windows"

# Проверка маршрутизации
echo ""
echo "5. Информация о сети:"
ip route show
echo ""
echo "Текущий IP WSL: $(hostname -I | awk '{print $1}')"
echo "Шлюз WSL: $(ip route | grep default | awk '{print $3}')"
