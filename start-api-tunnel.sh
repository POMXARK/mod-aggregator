#!/bin/bash

# SSH туннель для API ErkaPharm
echo "=== SSH туннель для API ErkaPharm ==="

# Проверяем что VPN работает
if ! ip addr show tun1 >/dev/null 2>&1; then
    echo "❌ VPN не подключен! Запустите VPN сначала:"
    echo "./auto-connect.sh"
    exit 1
fi

echo "✓ VPN работает"

# Определяем SSH ключ
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

# Проверяем SSH ключ
if [ -n "$SSH_KEY" ] && ssh -i "$SSH_KEY" -o PasswordAuthentication=no -o ConnectTimeout=5 xcom@xcom-01.dev.erkapharm.ru 'echo SSH OK' >/dev/null 2>&1; then
    echo "✓ SSH ключ работает"
else
    echo "⚠️  SSH ключ не настроен"
    echo "Запустите: ./setup-ssh-key.sh"
    exit 1
fi

# Запускаем туннель для API
echo "🚀 Запускаем SSH туннель для API..."
echo "✓ Используем SSH ключ: $SSH_KEY"

# Туннель HTTPS (443) на локальный порт 9443
ssh -i "$SSH_KEY" -f -N \
  -L 9443:api.erkapharm.com:443 \
  xcom@xcom-01.dev.erkapharm.ru

# Проверяем что туннель запустился
sleep 2

echo ""
echo "Проверка порта:"
netstat -tln 2>/dev/null | grep -E ":9443" && echo "✓ Порт 9443 открыт (API туннель)" || echo "❌ Порт 9443 не открыт"

echo ""
echo "=== ГОТОВО! ==="
echo "API ErkaPharm доступен на: https://localhost:9443"
echo ""
echo "Использование в приложении:"
echo "https://localhost:9443/  (вместо https://api.erkapharm.com/)"
echo ""
echo "Остановка: pkill -f 'ssh.*9443'"
























