#!/bin/bash

# Тест подключения к VPN серверу
VPN_HOST="vpn.erkapharm.com"
VPN_IP="89.107.140.6"

echo "=== Тест VPN сервера ==="

echo "1. Тест DNS разрешения:"
echo "Хост: $VPN_HOST"
nslookup $VPN_HOST 2>/dev/null | grep "Address" || echo "❌ DNS failed"

echo ""
echo "2. Тест подключения по IP:"
echo "IP: $VPN_IP"
timeout 5 bash -c "</dev/tcp/$VPN_IP/443" && echo "✓ Порт 443 открыт" || echo "❌ Порт 443 закрыт"

echo ""
echo "3. Тест HTTPS подключения:"
curl -k -I https://$VPN_IP/ --connect-timeout 5 2>/dev/null | head -1 || echo "❌ HTTPS failed"

echo ""
echo "=== Рекомендации ==="
if nslookup $VPN_HOST >/dev/null 2>&1; then
    echo "✓ DNS работает - используй ./connect-rkf-manual.sh"
else
    echo "❌ DNS не работает - используй ./connect-rkf-ip.sh"
fi
