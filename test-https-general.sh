#!/bin/bash

# Тест HTTPS соединений в целом
echo "=== Тест HTTPS соединений ==="

# Тест 1: Google
echo "1. Тест Google HTTPS:"
curl -k -I https://www.google.com --connect-timeout 5 --max-time 10 2>/dev/null | head -1 || echo "❌ Google недоступен"
echo ""

# Тест 2: GitHub
echo "2. Тест GitHub HTTPS:"
curl -k -I https://github.com --connect-timeout 5 --max-time 10 2>/dev/null | head -1 || echo "❌ GitHub недоступен"
echo ""

# Тест 3: VPN сервер
VPN_IP="89.107.140.6"
echo "3. Тест VPN сервера HTTPS:"
curl -k -I https://$VPN_IP --connect-timeout 5 --max-time 15 2>/dev/null | head -1 || echo "❌ VPN сервер недоступен"
echo ""

# Тест 4: Проверка TCP соединения
echo "4. Тест TCP соединений:"
echo "Google (443):"
timeout 3 bash -c "</dev/tcp/8.8.8.8/443" >/dev/null && echo "✓ Google TCP OK" || echo "❌ Google TCP FAIL"

echo "VPN сервер (443):"
timeout 3 bash -c "</dev/tcp/$VPN_IP/443" >/dev/null && echo "✓ VPN TCP OK" || echo "❌ VPN TCP FAIL"
echo ""

echo "=== Анализ ==="
if curl -k -I https://www.google.com --connect-timeout 5 --max-time 10 >/dev/null 2>&1; then
    echo "✓ HTTPS в целом работает"
    if curl -k -I https://$VPN_IP --connect-timeout 5 --max-time 15 >/dev/null 2>&1; then
        echo "✓ VPN сервер отвечает на HTTPS"
        echo "Проблема в другом (возможно, в openconnect или авторизации)"
    else
        echo "❌ VPN сервер не отвечает на HTTPS"
        echo "Сервер недоступен или блокирует соединения"
    fi
else
    echo "❌ HTTPS не работает вообще"
    echo "Проблема с интернетом в WSL"
fi
