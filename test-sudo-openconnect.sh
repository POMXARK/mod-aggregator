#!/bin/bash

# Тест sudo для openconnect
echo "=== Тест sudo для openconnect ==="

# Проверяем путь
OPENCONNECT_PATH=$(which openconnect)
echo "Путь к openconnect: $OPENCONNECT_PATH"

# Проверяем sudo настройку
echo ""
echo "Текущая sudo конфигурация:"
sudo grep openconnect /etc/sudoers || echo "Правило не найдено в /etc/sudoers"

# Тест запуска с sudo -n (без пароля)
echo ""
echo "Тест запуска без пароля:"
if sudo -n true 2>/dev/null; then
    echo "✓ sudo NOPASSWD работает"
else
    echo "❌ sudo требует пароль"
fi

# Тест openconnect --version с sudo
echo ""
echo "Тест openconnect с sudo:"
if sudo -n $OPENCONNECT_PATH --version >/dev/null 2>&1; then
    echo "✓ openconnect работает с sudo без пароля"
else
    echo "❌ openconnect не работает с sudo"
fi

echo ""
echo "Если проблемы - проверь /etc/sudoers:"
echo "sudo visudo  # и добавь строку:"
echo "$USER ALL=(ALL) NOPASSWD: /usr/sbin/openconnect"
