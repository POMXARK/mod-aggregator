#!/bin/bash

# Получаем IP адреса для разработки ErkaPharm
echo "=== Получение IP адресов для ErkaPharm разработки ==="

# Список хостов из SSH команды
HOSTS=(
    "basket-01.dev.erkapharm.ru"
    "fndr-01.prod.shop.local"
    "ws.erkapharm.com"
    "mbsdevcrm15sp1.manzanagroup.ru"
    "mnz-cosn.erkapharm.com"
    "xcom-01.dev.erkapharm.ru"
)

echo "Проверяем DNS разрешение..."
for host in "${HOSTS[@]}"; do
    echo -n "$host: "
    # Пробуем разные DNS серверы
    ip=$(nslookup $host 8.8.8.8 2>/dev/null | grep "Address" | tail -1 | awk '{print $2}')
    if [ -z "$ip" ]; then
        ip=$(nslookup $host 1.1.1.1 2>/dev/null | grep "Address" | tail -1 | awk '{print $2}')
    fi
    if [ -n "$ip" ]; then
        echo "$ip"
    else
        echo "не найден"
    fi
done

echo ""
echo "=== Внутренние подсети ErkaPharm ==="
echo "На основе анализа трафика:"
echo "10.0.0.0/8"
echo "172.16.0.0/12"
echo "192.168.0.0/16"
echo "172.21.0.0/16 (VPN подсеть)"
