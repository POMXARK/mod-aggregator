#!/bin/bash

# Подключение ErkaPharm VPN с sudo (для TUN устройства)
# Использует expect для автоматизации ввода

echo "=== Подключение ErkaPharm VPN с sudo ==="
echo "Сервер: vpn.erkapharm.com"
echo "Группа: RKF"
echo "Пользователь: dmitriy.maksimov"
echo ""

# Проверяем установлен ли expect
if ! command -v expect &> /dev/null; then
    echo "❌ expect не установлен. Устанавливаю..."
    sudo apt update && sudo apt install -y expect
fi

echo "🚀 Подключаемся к VPN с правами root..."
echo "Пароль VPN будет введен автоматически"
echo "Нажми Ctrl+C для отключения"
echo ""

# Используем expect для автоматизации
expect << EOF
spawn sudo openconnect --authgroup RKF --user dmitriy.maksimov vpn.erkapharm.com

# Ждем запроса пароля sudo (если есть)
expect {
    "*password*" {
        send "your_sudo_password_here\r"
        exp_continue
    }
    "*Password:*" {
        send "QQLnT82CHjO1\r"
        exp_continue
    }
    "*username*" {
        send "dmitriy.maksimov\r"
        exp_continue
    }
    eof
}

interact
EOF

echo ""
echo "VPN отключен"
