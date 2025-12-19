#!/bin/bash

# Полная диагностика VPN статуса
echo "=== Диагностика VPN статуса ==="

# 1. Проверяем TUN интерфейсы
echo "1. TUN интерфейсы в WSL:"
ip addr show | grep -E "(tun|vpn)" || echo "   Нет TUN интерфейсов"

# 2. Проверяем маршруты в WSL
echo ""
echo "2. Маршруты в WSL:"
ip route show | head -10

# 3. Проверяем процессы VPN
echo ""
echo "3. VPN процессы:"
ps aux | grep -E "(openconnect|vpn)" | grep -v grep || echo "   Нет VPN процессов"

# 4. Проверяем подключение к VPN серверу
echo ""
echo "4. Тест VPN сервера:"
VPN_IP="89.107.140.6"
if ping -c 1 $VPN_IP >/dev/null 2>&1; then
    echo "   ✓ VPN сервер доступен"
else
    echo "   ❌ VPN сервер недоступен"
fi

# 5. Тест внешнего IP через curl
echo ""
echo "5. Текущий внешний IP:"
EXTERNAL_IP=$(curl -s --connect-timeout 5 ifconfig.me/ip 2>/dev/null || echo "failed")
if [ "$EXTERNAL_IP" != "failed" ]; then
    echo "   Внешний IP: $EXTERNAL_IP"
    # Проверяем, является ли IP внутренним (примерная проверка для ErkaPharm)
    if [[ $EXTERNAL_IP =~ ^(10\.|172\.1[6-9]\.|172\.2[0-9]\.|172\.3[0-1]\.|192\.168\.) ]]; then
        echo "   ✓ Вероятно VPN работает (внутренний IP)"
    else
        echo "   ❌ VPN не работает (внешний IP)"
    fi
else
    echo "   ❌ Не удалось получить IP"
fi

echo ""
echo "=== Рекомендации ==="
echo "Если VPN не работает:"
echo "1. Проверь что VPN подключен: ps aux | grep openconnect"
echo "2. Проверь TUN интерфейс: ip addr show"
echo "3. Переподключи VPN: ./auto-connect.sh"
echo "4. Проверь маршрутизацию в Windows: route print"
