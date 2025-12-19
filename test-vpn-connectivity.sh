#!/bin/bash

# Проверка доступности VPN сервера
VPN_IP="89.107.140.6"
VPN_HOST="vpn.erkapharm.com"

echo "=== Проверка доступности VPN сервера ==="
echo "IP: $VPN_IP"
echo "Хост: $VPN_HOST"
echo ""

# 1. Проверка ping
echo "1. Ping VPN сервера:"
ping -c 3 $VPN_IP
echo ""

# 2. Проверка TCP соединения на порт 443
echo "2. Проверка HTTPS порта:"
timeout 5 bash -c "</dev/tcp/$VPN_IP/443" && echo "✓ Порт 443 открыт" || echo "❌ Порт 443 закрыт"
echo ""

# 3. Проверка с curl
echo "3. Тест HTTPS соединения:"
curl -k -I https://$VPN_IP/ --connect-timeout 10 --max-time 15 2>/dev/null | head -1 || echo "❌ HTTPS недоступен"
echo ""

# 4. Проверка разрешения DNS
echo "4. DNS разрешение:"
nslookup $VPN_HOST 8.8.8.8 2>/dev/null | grep "Address" || echo "❌ DNS не работает"
echo ""

# 5. Проверка интернет соединения в целом
echo "5. Общий интернет:"
ping -c 2 8.8.8.8 >/dev/null 2>&1 && echo "✓ Интернет работает" || echo "❌ Интернет не работает"

echo ""
echo "=== Диагностика ==="
if ping -c 1 $VPN_IP >/dev/null 2>&1; then
    if timeout 5 bash -c "</dev/tcp/$VPN_IP/443" >/dev/null 2>&1; then
        echo "✓ VPN сервер доступен - проблема в другом"
        echo "Возможно, нужно проверить настройки OpenConnect"
    else
        echo "❌ VPN сервер не отвечает на порт 443"
        echo "Проверь настройки firewall или VPN сервер недоступен"
    fi
else
    echo "❌ VPN сервер не пингуется"
    echo "Проверь интернет соединение или IP адрес сервера"
fi
