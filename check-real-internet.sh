#!/bin/bash

# Проверка реального интернет-соединения (не только ping)
echo "=== Проверка интернет-соединения ==="

# Тест 1: curl к надежному сайту
echo "1. Тест HTTPS соединения к Google:"
curl -k -I https://www.google.com --connect-timeout 5 --max-time 10 2>/dev/null | head -1
if [ $? -eq 0 ]; then
    echo "✓ HTTPS работает"
else
    echo "❌ HTTPS не работает"
fi
echo ""

# Тест 2: curl к надежному сайту (другой)
echo "2. Тест HTTPS соединения к Cloudflare:"
curl -k -I https://1.1.1.1 --connect-timeout 5 --max-time 10 2>/dev/null | head -1
if [ $? -eq 0 ]; then
    echo "✓ HTTPS работает (Cloudflare)"
else
    echo "❌ HTTPS не работает (Cloudflare)"
fi
echo ""

# Тест 3: DNS разрешение
echo "3. Тест DNS:"
nslookup google.com 8.8.8.8 2>/dev/null | grep -q "Address" && echo "✓ DNS работает" || echo "❌ DNS не работает"
echo ""

# Тест 4: TCP соединения
echo "4. Тест TCP портов:"
timeout 3 bash -c "</dev/tcp/8.8.8.8/53" >/dev/null 2>&1 && echo "✓ TCP 53 (DNS) открыт" || echo "❌ TCP 53 (DNS) закрыт"
timeout 3 bash -c "</dev/tcp/8.8.8.8/443" >/dev/null 2>&1 && echo "✓ TCP 443 (HTTPS) открыт" || echo "❌ TCP 443 (HTTPS) закрыт"
echo ""

echo "=== Вывод ==="
if curl -k -I https://www.google.com --connect-timeout 5 --max-time 10 >/dev/null 2>&1; then
    echo "✅ Интернет работает!"
    echo "Ping может быть заблокирован, но HTTPS работает нормально"
else
    echo "❌ Интернет не работает"
    echo "Проверь сетевые настройки WSL"
fi
