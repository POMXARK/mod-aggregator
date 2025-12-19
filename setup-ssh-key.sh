#!/bin/bash

# Настройка SSH ключа для автоматического входа на ErkaPharm сервер
echo "=== Настройка SSH ключа для ErkaPharm ==="

SSH_HOST="xcom@xcom-01.dev.erkapharm.ru"
SSH_KEY_FILE="$HOME/.ssh/id_rsa_erkapharm"

echo "SSH сервер: $SSH_HOST"
echo "Файл ключа: $SSH_KEY_FILE"
echo ""

# Шаг 1: Проверяем существует ли уже ключ
if [ -f "$SSH_KEY_FILE" ]; then
    echo "✓ SSH ключ уже существует: $SSH_KEY_FILE"
else
    echo "Генерируем новый SSH ключ..."
    ssh-keygen -t rsa -b 4096 -f "$SSH_KEY_FILE" -N "" -C "erkapharm-dev-$(date +%Y%m%d)"

    if [ $? -eq 0 ]; then
        echo "✓ SSH ключ создан: $SSH_KEY_FILE"
    else
        echo "❌ Ошибка создания ключа"
        exit 1
    fi
fi

echo ""
echo "Публичный ключ:"
cat "${SSH_KEY_FILE}.pub"
echo ""

# Шаг 2: Копируем ключ на сервер
echo "Копируем ключ на сервер..."
echo "Введите пароль от SSH сервера когда попросят:"

ssh-copy-id -i "$SSH_KEY_FILE" "$SSH_HOST"

if [ $? -eq 0 ]; then
    echo ""
    echo "✓ Ключ успешно скопирован на сервер!"
else
    echo ""
    echo "❌ Ошибка копирования ключа"
    echo ""
    echo "Альтернативный способ (вручную):"
    echo "1. Подключитесь: ssh $SSH_HOST"
    echo "2. Создайте директорию: mkdir -p ~/.ssh"
    echo "3. Добавьте ключ: echo '$(cat ${SSH_KEY_FILE}.pub)' >> ~/.ssh/authorized_keys"
    echo "4. Установите права: chmod 600 ~/.ssh/authorized_keys && chmod 700 ~/.ssh"
    exit 1
fi

# Шаг 3: Тестируем вход без пароля
echo ""
echo "Тестируем вход без пароля..."
ssh -o PasswordAuthentication=no -o ConnectTimeout=5 "$SSH_HOST" 'echo "SSH без пароля работает!" && uptime'

if [ $? -eq 0 ]; then
    echo ""
    echo "🎉 УСПЕХ! SSH ключ настроен и работает!"
    echo ""
    echo "Теперь можно использовать SSH туннели без пароля:"
    echo "./start-ssh-tunnel.sh"
else
    echo ""
    echo "❌ Тест не прошел. Проверьте настройки на сервере."
    exit 1
fi
