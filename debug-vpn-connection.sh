#!/bin/bash

# Отладка VPN подключения
echo "=== Отладка VPN подключения ==="

VPN_IP="89.107.140.6"

# 1. Проверяем сетевые настройки
echo "1. Сетевые интерфейсы:"
ip addr show | grep -E "(inet|eth|tun)" | head -10
echo ""

# 2. Проверяем маршруты
echo "2. Таблица маршрутизации:"
ip route show | head -5
echo ""

# 3. Проверяем DNS
echo "3. DNS настройки:"
cat /etc/resolv.conf 2>/dev/null || echo "resolv.conf не найден"
echo ""

# 4. Тестируем базовое подключение
echo "4. Базовое подключение к VPN серверу:"
echo "Пингуем $VPN_IP..."
ping -c 2 $VPN_IP >/dev/null 2>&1 && echo "✓ Ping успешен" || echo "❌ Ping неудачен"

echo "Тестируем TCP соединение..."
timeout 5 bash -c "</dev/tcp/$VPN_IP/443" >/dev/null 2>&1 && echo "✓ TCP соединение установлено" || echo "❌ TCP соединение неудачно"
echo ""

# 5. Тестируем openconnect без авторизации
echo "5. Тест openconnect (без авторизации):"
timeout 10 openconnect --cookie-only --no-cert-check $VPN_IP 2>&1 | head -5 || echo "❌ OpenConnect тест неудачен"
echo ""

# 6. Проверяем sudo
echo "6. Проверка sudo:"
sudo -n true 2>/dev/null && echo "✓ sudo работает без пароля" || echo "❌ sudo требует пароль"
echo ""

echo "=== Возможные решения ==="
echo "1. Если ping работает но TCP нет - firewall блокирует"
echo "2. Если openconnect тест неудачен - проблема с openconnect"
echo "3. Если sudo требует пароль - настрой sudo NOPASSWD"
echo "4. Попробуй: ./connect-rkf-ip.sh (прямое подключение по IP)"
