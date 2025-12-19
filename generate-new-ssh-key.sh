#!/bin/bash

# Генерация нового SSH ключа для ErkaPharm
echo "=== Генерация нового SSH ключа для ErkaPharm ==="

SSH_HOST="xcom@xcom-01.dev.erkapharm.ru"
SSH_KEY_FILE="$HOME/.ssh/id_rsa_erkapharm_new"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

echo "SSH сервер: $SSH_HOST"
echo "Новый файл ключа: $SSH_KEY_FILE"
echo "Метка времени: $TIMESTAMP"
echo ""

# Проверяем существует ли уже новый ключ
if [ -f "$SSH_KEY_FILE" ]; then
    echo "⚠️  Новый ключ уже существует: $SSH_KEY_FILE"
    echo "Удалить его? (y/n)"
    read -r response
    if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
        rm -f "$SSH_KEY_FILE" "$SSH_KEY_FILE.pub"
        echo "✓ Старый новый ключ удален"
    else
        echo "Отмена операции"
        exit 0
    fi
fi

# Генерируем новый SSH ключ
echo "Генерируем новый SSH ключ..."
ssh-keygen -t rsa -b 4096 -f "$SSH_KEY_FILE" -N "" -C "erkapharm-dev-$TIMESTAMP"

if [ $? -eq 0 ]; then
    echo "✓ Новый SSH ключ создан: $SSH_KEY_FILE"
else
    echo "❌ Ошибка создания ключа"
    exit 1
fi

echo ""
echo "Публичный ключ:"
echo "==============="
cat "${SSH_KEY_FILE}.pub"
echo "==============="
echo ""

echo "ВЫПОЛНИТЕ НА СЕРВЕРЕ (в SSH сессии с паролем):"
echo ""
echo "1. Резервная копия старых ключей:"
echo "   cp ~/.ssh/authorized_keys ~/.ssh/authorized_keys.backup"
echo ""
echo "2. Очистить authorized_keys:"
echo "   > ~/.ssh/authorized_keys"
echo ""
echo "3. Добавить НОВЫЙ публичный ключ:"
echo "   nano ~/.ssh/authorized_keys"
echo "   (вставьте ключ из раздела выше)"
echo ""
echo "4. Установить права:"
echo "   chmod 600 ~/.ssh/authorized_keys"
echo "   chmod 700 ~/.ssh"
echo ""
echo "5. Перезапустить SSH:"
echo "   sudo systemctl restart sshd"
echo ""

echo "ЛОКАЛЬНЫЕ ДЕЙСТВИЯ:"
echo ""
echo "1. Обновить скрипт start-ssh-tunnel.sh:"
echo "   sed -i 's/id_rsa_erkapharm/id_rsa_erkapharm_new/g' start-ssh-tunnel.sh"
echo ""
echo "2. Протестировать:"
echo "   ssh -i ~/.ssh/id_rsa_erkapharm_new $SSH_HOST 'echo \"Новый ключ работает!\"'"
echo ""
echo "3. Если работает - удалить старый ключ:"
echo "   rm ~/.ssh/id_rsa_erkapharm ~/.ssh/id_rsa_erkapharm.pub"
echo ""

echo "НОВЫЙ ПУБЛИЧНЫЙ КЛЮЧ (скопируйте на сервер):"
echo "=========================================="
cat "${SSH_KEY_FILE}.pub"
echo "=========================================="
