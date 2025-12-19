#!/bin/bash

# Специфичная настройка для Cisco AnyConnect VPN
# Запуск: ./cisco-anyconnect-setup.sh <server> <group> <user>

SERVER=$1
GROUP=$2
USER=$3

if [ -z "$SERVER" ] || [ -z "$GROUP" ] || [ -z "$USER" ]; then
    echo "Использование: $0 <server> <group> <user>"
    echo ""
    echo "Пример: $0 vpn.company.com Employees john.doe@company.com"
    echo ""
    echo "Как получить эти данные:"
    echo "1. Server: адрес VPN сервера (обычно vpn.company.com)"
    echo "2. Group: группа аутентификации (спроси у IT)"
    echo "3. User: твой логин/email"
    exit 1
fi

echo "=== Настройка Cisco AnyConnect для: $SERVER ==="
echo "Группа: $GROUP"
echo "Пользователь: $USER"
echo ""

# Создаем скрипт быстрого подключения
cat > connect-cisco.sh << EOF
#!/bin/bash
echo "=== Подключение к Cisco AnyConnect ==="
echo "Сервер: $SERVER"
echo "Группа: $GROUP"
echo "Пользователь: $USER"
echo ""

# Запуск openconnect с параметрами для Cisco
openconnect \\
  --socks-proxy localhost:1080 \\
  --user="$USER" \\
  --authgroup="$GROUP" \\
  --no-cert-check \\
  "$SERVER"

echo ""
echo "Для отключения нажми Ctrl+C"
EOF

chmod +x connect-cisco.sh

echo "✓ Создан скрипт connect-cisco.sh"
echo ""
echo "=== Как подключиться ==="
echo "1. Запусти: ./connect-cisco.sh"
echo "2. Введи пароль когда попросят"
echo "3. В Windows настрой SOCKS5 прокси: 127.0.0.1:1080"
echo ""
echo "=== Тестирование ==="
echo "В браузере Windows перейди на: https://whatismyipaddress.com/"
echo "Должен показаться IP твоей компании"
