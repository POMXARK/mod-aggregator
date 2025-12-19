#!/bin/bash

# HTTP туннель для API ErkaPharm (обход SSL проблем)
echo "=== HTTP туннель для API ErkaPharm ==="

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

# Запускаем HTTP туннель
echo "🚀 Запускаем HTTP туннель для API..."
echo "✓ Используем SSH ключ: $SSH_KEY"

# Останавливаем существующий туннель
pkill -f "ssh.*8080" 2>/dev/null || true

# Туннель HTTP (80) на локальный порт 8080
ssh -i "$SSH_KEY" -f -N \
  -L 8080:api.erkapharm.com:80 \
  xcom@xcom-01.dev.erkapharm.ru

# Проверяем что туннель запустился
sleep 2

echo ""
echo "Проверка порта:"
netstat -tln 2>/dev/null | grep -E ":8080" && echo "✓ Порт 8080 открыт (HTTP туннель)" || echo "❌ Порт 8080 не открыт"

echo ""
echo "=== ГОТОВО! ==="
echo "API ErkaPharm доступен на: http://localhost:8080"
echo ""
echo "Использование в браузере:"
echo "http://localhost:8080/  (HTTP вместо HTTPS)"
echo ""
echo "Остановка: pkill -f 'ssh.*8080'"

