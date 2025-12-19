#!/bin/bash

# Диагностика сети и DNS в WSL2
echo "=== Диагностика сети WSL2 ==="

# 1. Информация о сети
echo "1. Сетевые интерфейсы:"
ip addr show | grep -E "(inet|eth|tun)"
echo ""

# 2. Маршруты
echo "2. Таблица маршрутизации:"
ip route show
echo ""

# 3. DNS настройки
echo "3. DNS конфигурация:"
cat /etc/resolv.conf
echo ""

# 4. Тест DNS
echo "4. Тест DNS разрешения:"
echo "Проверяем vpn.erkapharm.com..."
nslookup vpn.erkapharm.com 2>/dev/null || echo "❌ nslookup failed"

echo "Проверяем google.com..."
nslookup google.com 2>/dev/null | grep "Address" || echo "❌ nslookup google failed"
echo ""

# 5. Тест подключения
echo "5. Тест подключения к хосту:"
echo "Пингуем 8.8.8.8..."
ping -c 2 8.8.8.8 >/dev/null 2>&1 && echo "✓ Ping 8.8.8.8 OK" || echo "❌ Ping 8.8.8.8 failed"

echo "Пингуем google.com..."
ping -c 2 google.com >/dev/null 2>&1 && echo "✓ Ping google.com OK" || echo "❌ Ping google.com failed"
echo ""

# 6. Проверить /etc/hosts
echo "6. Файл hosts:"
grep -v "^#" /etc/hosts | grep -v "^$"
echo ""

# 7. WSL версия и настройки
echo "7. WSL информация:"
echo "WSL версия: $(cat /proc/version | grep -o "WSL2")"
echo "DNS suffix: $(nmcli device show eth0 2>/dev/null | grep IP4.DNS || echo "nmcli не установлен")"

echo ""
echo "=== Рекомендации ==="
echo "Если DNS не работает:"
echo "1. Проверь интернет в Windows"
echo "2. Перезапусти WSL: wsl --shutdown && wsl -d Ubuntu"
echo "3. Попробуй статический DNS: echo 'nameserver 8.8.8.8' | sudo tee /etc/resolv.conf"
echo "4. Проверь firewall Windows"
