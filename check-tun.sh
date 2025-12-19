#!/bin/bash

# Проверка TUN устройства и прав для VPN
echo "=== Диагностика TUN устройства ==="

# Проверяем права
echo "1. Права пользователя:"
id
echo ""

# Проверяем TUN устройство
echo "2. TUN устройство:"
if [ -c /dev/net/tun ]; then
    echo "  ✓ /dev/net/tun существует"
else
    echo "  ❌ /dev/net/tun не существует"
fi
ls -la /dev/net/tun 2>/dev/null || echo "  TUN устройство недоступно"
echo ""

# Проверяем группы
echo "3. Группы пользователя:"
groups
echo ""

# Проверяем sudo для openconnect
echo "4. Sudo настройка:"
if sudo -l | grep -q openconnect; then
    echo "  ✓ openconnect разрешен без пароля в sudo"
else
    echo "  ❌ openconnect требует пароль в sudo"
    echo "  Запусти: sudo ./setup-sudo-nopasswd.sh"
fi
echo ""

# Тест создания TUN (только для root)
echo "5. Тест TUN устройства:"
if [ "$EUID" -eq 0 ]; then
    if ip tuntap add mode tun tun0 &>/dev/null; then
        echo "  ✓ TUN устройство создано успешно"
        ip link delete tun0 2>/dev/null
    else
        echo "  ❌ Не удалось создать TUN устройство"
    fi
else
    echo "  ⚠️  Запусти с sudo для полного теста"
fi

echo ""
echo "=== Рекомендации ==="
echo "Если проблемы с TUN:"
echo "1. sudo ./setup-sudo-nopasswd.sh"
echo "2. Перезапусти WSL: wsl --shutdown && wsl -d Ubuntu"
echo "3. Проверь группы: sudo usermod -aG sudo $USER"
