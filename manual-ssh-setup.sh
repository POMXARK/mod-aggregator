#!/bin/bash

# Ручная настройка SSH ключа на сервере ErkaPharm
SSH_HOST="xcom@xcom-01.dev.erkapharm.ru"
PUBKEY_FILE="$HOME/.ssh/id_rsa_erkapharm.pub"

echo "=== РУЧНАЯ НАСТРОЙКА SSH КЛЮЧА ==="
echo "Сервер: $SSH_HOST"
echo ""

if [ ! -f "$PUBKEY_FILE" ]; then
    echo "❌ Публичный ключ не найден: $PUBKEY_FILE"
    exit 1
fi

echo "Ваш публичный ключ:"
echo "=================="
cat "$PUBKEY_FILE"
echo "=================="
echo ""

echo "ВЫПОЛНИТЕ ЭТИ КОМАНДЫ НА СЕРВЕРЕ (после подключения с паролем):"
echo ""
echo "1. Создайте директорию SSH:"
echo "   mkdir -p ~/.ssh"
echo ""
echo "2. Установите правильные права:"
echo "   chmod 700 ~/.ssh"
echo ""
echo "3. Добавьте публичный ключ в authorized_keys:"
echo "   nano ~/.ssh/authorized_keys"
echo "   (вставьте ключ из раздела выше и сохраните)"
echo ""
echo "4. Установите права на файл:"
echo "   chmod 600 ~/.ssh/authorized_keys"
echo ""
echo "5. Проверьте владельца:"
echo "   chown -R \$USER:\$USER ~/.ssh"
echo ""
echo "6. Проверьте SSH конфигурацию сервера:"
echo "   sudo grep -E 'PubkeyAuthentication|AuthorizedKeysFile' /etc/ssh/sshd_config"
echo ""
echo "7. Перезагрузите SSH службу:"
echo "   sudo systemctl restart sshd  # или sudo service ssh restart"
echo ""

echo "ПОСЛЕ НАСТРОЙКИ НА СЕРВЕРЕ:"
echo "Тестируйте: ssh $SSH_HOST 'echo SSH без пароля работает!'"
echo ""
echo "Если работает - запускайте туннель: ./start-ssh-tunnel.sh"
