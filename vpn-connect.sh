#!/bin/bash

# Скрипт для подключения к VPN через SOCKS5 прокси
# Использование: ./vpn-connect.sh <тип_vpn> <сервер> [опции]

VPN_TYPE=$1
SERVER=$2
shift 2

if [ -z "$VPN_TYPE" ] || [ -z "$SERVER" ]; then
    echo "Использование: $0 <openconnect|openvpn> <сервер> [опции]"
    echo ""
    echo "Примеры:"
    echo "OpenConnect: $0 openconnect vpn.company.com --user=your_user"
    echo "OpenVPN: $0 openvpn config.ovpn"
    exit 1
fi

echo "=== Подключение к VPN: $VPN_TYPE -> $SERVER ==="

case $VPN_TYPE in
    "openconnect")
        echo "Запуск OpenConnect с SOCKS5 прокси..."

        # Специальный случай для ErkaPharm RKF
        if [ "$SERVER" = "vpn.erkapharm.com" ] && [[ "$@" == *"--authgroup RKF"* ]]; then
            echo "Обнаружена конфигурация ErkaPharm RKF - использую сохраненный пароль"
            # Запускаем с сохраненным паролем
            openconnect --socks-proxy localhost:1080 "$SERVER" "$@" <<< "QQLnT82CHjO1" &
        else
            # Обычное подключение
            openconnect --socks-proxy localhost:1080 "$SERVER" "$@" &
        fi

        VPN_PID=$!
        echo "OpenConnect запущен (PID: $VPN_PID)"
        echo "Для отключения: kill $VPN_PID"
        ;;

    "openvpn")
        echo "Запуск OpenVPN с SOCKS5 прокси..."
        # Добавляем socks-proxy в конфиг если это файл
        if [ -f "$SERVER" ]; then
            # Создаем временный конфиг с socks-proxy
            TEMP_CONFIG="/tmp/openvpn_temp.conf"
            cp "$SERVER" "$TEMP_CONFIG"
            echo "socks-proxy 127.0.0.1 1080" >> "$TEMP_CONFIG"
            openvpn "$TEMP_CONFIG" "$@" &
            VPN_PID=$!
            echo "OpenVPN запущен (PID: $VPN_PID)"
            echo "Временный конфиг: $TEMP_CONFIG"
        else
            echo "Ошибка: для OpenVPN нужен файл конфигурации"
            exit 1
        fi
        ;;

    *)
        echo "Ошибка: неподдерживаемый тип VPN: $VPN_TYPE"
        echo "Поддерживаемые: openconnect, openvpn"
        exit 1
        ;;
esac

echo "=== VPN подключен ==="
echo "Проверь статус: curl --socks5 localhost:1080 ifconfig.me"
echo "Или в браузере Windows: настрой SOCKS5 прокси на 127.0.0.1:1080"
