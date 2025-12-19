#!/bin/bash

# Тест подключения к API ErkaPharm
API_HOST="api.erkapharm.com"
API_IP="5.172.178.51"

echo "=== Тест API ErkaPharm ==="
echo "Хост: $API_HOST"
echo "IP: $API_IP"
echo ""

# Проверяем DNS
echo "1. Проверяем DNS разрешение:"
nslookup $API_HOST 8.8.8.8 2>/dev/null | grep "Address" || echo "❌ DNS не работает"

echo ""
echo "2. Проверяем HTTPS соединение:"
curl -k -I https://$API_IP --connect-timeout 5 --max-time 10 2>/dev/null | head -1 || echo "❌ HTTPS недоступен"

echo ""
echo "3. Проверяем маршрут в Windows:"
route print | grep "$API_IP" && echo "✓ Маршрут настроен через VPN" || echo "❌ Маршрут НЕ настроен"

echo ""
echo "=== РЕЗУЛЬТАТ ==="
echo "Если все проверки зеленые - API доступен через VPN!"
echo ""
echo "Для тестирования в приложении:"
echo "https://$API_IP"
echo "mongodb://localhost:9999 (через SSH туннель)"
