#!/bin/bash

# Исправление маршрутизации в WSL для VPN
echo "=== Исправление маршрутизации WSL ==="

# 1. Проверяем TUN интерфейс
echo "1. Ищем TUN интерфейс..."
TUN_IF=$(ip addr show | grep -o "tun[0-9]*" | head -1)
if [ -n "$TUN_IF" ]; then
    echo "   ✓ Найден TUN интерфейс: $TUN_IF"

    # 2. Получаем IP TUN интерфейса
    TUN_IP=$(ip addr show $TUN_IF | grep "inet " | awk '{print $2}' | cut -d'/' -f1)
    if [ -n "$TUN_IP" ]; then
        echo "   ✓ IP TUN интерфейса: $TUN_IP"

        # 3. Проверяем маршруты
        echo ""
        echo "2. Текущие маршруты:"
        ip route show

        # 4. Добавляем маршрут по умолчанию через TUN (если нужно)
        echo ""
        echo "3. Настройка маршрутов..."
        # Удаляем старый маршрут по умолчанию
        sudo ip route del default 2>/dev/null || true

        # Добавляем маршрут через TUN
        sudo ip route add default via $TUN_IP dev $TUN_IF 2>/dev/null || echo "   ⚠️  Не удалось добавить маршрут"

        echo ""
        echo "4. Обновленные маршруты:"
        ip route show

        # 5. Тест
        echo ""
        echo "5. Тест подключения:"
        ping -c 2 8.8.8.8 >/dev/null 2>&1 && echo "   ✓ Интернет работает" || echo "   ❌ Интернет не работает"

    else
        echo "   ❌ Не удалось получить IP TUN интерфейса"
    fi

else
    echo "   ❌ TUN интерфейс не найден"
    echo "   Возможно VPN не подключен"
    echo "   Запусти: ./auto-connect.sh"
fi

echo ""
echo "=== Готово ==="
echo "Теперь проверь в Windows: .\\test-vpn-final.ps1"
