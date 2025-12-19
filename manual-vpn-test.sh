#!/bin/bash

# Ручное тестирование VPN подключения
VPN_IP="89.107.140.6"

echo "=== РУЧНОЕ ТЕСТИРОВАНИЕ VPN ==="
echo "IP сервера: $VPN_IP"
echo ""

echo "Шаг 1: Проверяем интернет"
ping -c 2 8.8.8.8 >/dev/null && echo "✓ Интернет работает" || echo "❌ Интернет не работает"
echo ""

echo "Шаг 2: Проверяем доступность VPN сервера"
ping -c 2 $VPN_IP >/dev/null && echo "✓ VPN сервер пингуется" || echo "❌ VPN сервер не пингуется"
echo ""

echo "Шаг 3: Проверяем HTTPS порт"
timeout 5 bash -c "</dev/tcp/$VPN_IP/443" >/dev/null && echo "✓ HTTPS порт открыт" || echo "❌ HTTPS порт закрыт"
echo ""

echo "Шаг 4: Тестируем OpenConnect (5 сек)"
echo "Если зависнет - нажми Ctrl+C"
timeout 5 sudo openconnect --cookie-only --no-cert-check $VPN_IP 2>&1 | head -3
echo ""

echo "=== РЕЗУЛЬТАТЫ ==="
if ping -c 1 $VPN_IP >/dev/null && timeout 3 bash -c "</dev/tcp/$VPN_IP/443" >/dev/null; then
    echo "✅ Сервер доступен - пробуй подключение:"
    echo "sudo openconnect --authgroup RKF --user dmitriy.maksimov vpn.erkapharm.com"
    echo "Или по IP: sudo openconnect --authgroup RKF --user dmitriy.maksimov --no-cert-check $VPN_IP"
else
    echo "❌ Сервер недоступен"
    echo "Проверь интернет или время работы сервера"
fi
