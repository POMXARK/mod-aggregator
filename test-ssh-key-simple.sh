#!/bin/bash

# Простой тест SSH ключа
SSH_HOST="xcom@xcom-01.dev.erkapharm.ru"
SSH_KEY="$HOME/.ssh/id_rsa_erkapharm"

echo "=== Тест SSH ключа ==="
echo "Сервер: $SSH_HOST"
echo "Ключ: $SSH_KEY"
echo ""

if [ ! -f "$SSH_KEY" ]; then
    echo "❌ Ключ не найден: $SSH_KEY"
    exit 1
fi

echo "Тестируем подключение без пароля..."
ssh -i "$SSH_KEY" -o PasswordAuthentication=no -o ConnectTimeout=10 "$SSH_HOST" 'echo "SSH без пароля работает на $(hostname)!"'

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ SSH ключ работает!"
else
    echo ""
    echo "❌ SSH ключ НЕ работает"
    echo ""
    echo "Возможные причины:"
    echo "1. Ключ не добавлен в authorized_keys на сервере"
    echo "2. Неправильные права на сервере"
    echo "3. SSH сервер не принимает pubkey аутентификацию"
    echo ""
    echo "Проверьте на сервере:"
    echo "  cat ~/.ssh/authorized_keys"
    echo "  ls -la ~/.ssh/"
    echo "  sudo grep PubkeyAuthentication /etc/ssh/sshd_config"
fi
