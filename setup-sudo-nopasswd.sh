#!/bin/bash

# Настройка sudo без пароля для openconnect
# Нужно для создания TUN устройства в WSL2

echo "=== Настройка sudo без пароля для openconnect ==="

# Проверяем есть ли уже настройка
if sudo grep -q "openconnect" /etc/sudoers; then
    echo "✓ Настройка уже существует"
else
    echo "Добавляем правило в /etc/sudoers..."
    echo "$USER ALL=(ALL) NOPASSWD: /usr/sbin/openconnect" | sudo tee -a /etc/sudoers > /dev/null
    echo "✓ Настройка добавлена"
fi

echo ""
echo "Теперь openconnect можно запускать без sudo пароля:"
echo "sudo openconnect --authgroup RKF --user dmitriy.maksimov vpn.erkapharm.com"
echo ""
echo "Пароль VPN нужно будет ввести вручную"
