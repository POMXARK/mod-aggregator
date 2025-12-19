#!/bin/bash

# Запуск SSH туннеля для ErkaPharm разработки
echo "=== Запуск SSH туннеля ErkaPharm ==="

# Проверяем что VPN работает
if ! ip addr show tun1 >/dev/null 2>&1 && ! ip addr show tun0 >/dev/null 2>&1; then
    echo "❌ VPN не подключен! Запустите VPN сначала:"
    echo "./auto-connect.sh"
    exit 1
fi

echo "✓ VPN работает"

# Определяем SSH ключ заранее
SSH_KEYS=(
    "$HOME/.ssh/id_rsa_erkapharm_new"
    "$HOME/.ssh/id_rsa_erkapharm"
    "$HOME/.ssh/id_rsa"
)

SSH_KEY=""
for key in "${SSH_KEYS[@]}"; do
    if [ -f "$key" ]; then
        SSH_KEY="$key"
        break
    fi
done

# Проверяем что SSH ключ работает
echo "Проверяем SSH ключ..."
if [ -n "$SSH_KEY" ] && ssh -i "$SSH_KEY" -o PasswordAuthentication=no -o ConnectTimeout=5 xcom@xcom-01.dev.erkapharm.ru 'echo SSH OK' >/dev/null 2>&1; then
    echo "✓ SSH ключ работает"
else
    echo "⚠️  SSH ключ не настроен или не работает"
    echo "Запустите: ./diagnose-ssh-key-issues.sh"
    echo "Или настройте: ./setup-ssh-key.sh"
    exit 1
fi

# Запускаем SSH туннель
echo "🚀 Запускаем SSH туннель..."
echo "✓ Используем SSH ключ: $SSH_KEY"

# Запускаем туннель
ssh -i "$SSH_KEY" -f -N \
  -L 8194:basket-01.dev.erkapharm.ru:8194 \
  -L 9443:api.erkapharm.com:443 \
  -L 9996:fndr-01.prod.shop.local:3030 \
  -L 9998:ws.erkapharm.com:8990 \
  -L 11006:mbsdevcrm15sp1.manzanagroup.ru:11006 \
  -L 9999:localhost:27017 \
  -p 22 xcom@xcom-01.dev.erkapharm.ru

# Проверяем что туннель запустился
sleep 2

echo ""
echo "Проверка портов:"
netstat -tln 2>/dev/null | grep -E ":8194|:9443|:9996|:9998|:11006|:9999" && echo "✓ Порты открыты" || echo "❌ Порты не открыты"

echo ""
echo "=== ГОТОВО! ==="
echo "SSH туннель запущен"
echo ""
echo "Доступные сервисы:"
echo "- Basket API: http://localhost:8194"
echo "- ErkaPharm API: https://localhost:9443"
echo "- MongoDB: mongodb://localhost:9999"
echo "- ManZana: http://localhost:11006"
echo "- Fndr: http://localhost:9996"
echo "- WS: http://localhost:9998"
