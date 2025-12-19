#!/bin/bash

# Быстрый старт ErkaPharm VPN
echo "=== Быстрый старт ErkaPharm VPN ==="
echo ""

# Шаг 1: Проверка и установка компонентов
echo "1. Проверяем установку компонентов..."
if ! command -v openconnect &> /dev/null; then
    echo "❌ OpenConnect не установлен!"
    echo "Запусти: sudo ./setup-wsl-vpn.sh"
    exit 1
fi

if ! pgrep -x "danted" > /dev/null; then
    echo "❌ SOCKS5 прокси не запущен!"
    echo "Запускаю автоматически..."
    sudo danted &
    sleep 2

    if ! pgrep -x "danted" > /dev/null; then
        echo "❌ Не удалось запустить прокси!"
        exit 1
    fi
fi

echo "✅ Все компоненты установлены"
echo ""

# Шаг 2: Тестирование моста
echo "2. Тестируем мост WSL -> Windows..."
./test-vpn-bridge.sh
echo ""

# Шаг 3: Подключение VPN
echo "3. Подключаемся к VPN ErkaPharm..."
echo "Нажми Ctrl+C для отключения"
echo ""

./connect-rkf.sh
