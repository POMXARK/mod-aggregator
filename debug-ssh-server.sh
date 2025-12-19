#!/bin/bash

# Диагностика SSH проблем на сервере ErkaPharm
SSH_HOST="xcom@xcom-01.dev.erkapharm.ru"
SSH_KEY="$HOME/.ssh/id_rsa_erkapharm"

echo "=== Диагностика SSH на сервере ErkaPharm ==="
echo "Сервер: $SSH_HOST"
echo "Ключ: $SSH_KEY"
echo ""

# 1. Проверяем локальный ключ
echo "1. Проверяем локальный SSH ключ:"
if [ -f "$SSH_KEY" ]; then
    echo "  ✓ Файл ключа существует"
    ls -la "$SSH_KEY"
else
    echo "  ❌ Файл ключа не найден"
    exit 1
fi

if [ -f "${SSH_KEY}.pub" ]; then
    echo "  ✓ Публичный ключ существует"
else
    echo "  ❌ Публичный ключ не найден"
    exit 1
fi
echo ""

# 2. Проверяем подключение с паролем
echo "2. Тестируем подключение с паролем:"
echo "  (Введите пароль когда попросят)"
ssh -o ConnectTimeout=10 "$SSH_HOST" 'echo "SSH с паролем работает"' 2>/dev/null
if [ $? -eq 0 ]; then
    echo "  ✓ SSH с паролем работает"
else
    echo "  ❌ SSH с паролем не работает"
    echo "  Проверьте сетевые настройки и доступность сервера"
    exit 1
fi
echo ""

# 3. Проверяем настройки на сервере
echo "3. Проверяем настройки SSH на сервере:"
echo "  Выполните эти команды на сервере (введите пароль):"
echo ""
echo "  # Проверить права на директорию"
echo "  ls -la ~/.ssh/"
echo ""
echo "  # Проверить authorized_keys"
echo "  cat ~/.ssh/authorized_keys"
echo ""
echo "  # Проверить права на файлы"
echo "  ls -la ~/.ssh/authorized_keys"
echo ""
echo "  # Проверить SSH конфигурацию"
echo "  sudo grep -E '^(PubkeyAuthentication|AuthorizedKeysFile|PasswordAuthentication)' /etc/ssh/sshd_config"
echo ""

# 4. Тестируем с verbose
echo "4. Детальная диагностика SSH:"
echo "  Выполните: ssh -vvv $SSH_HOST"
echo "  И посмотрите где происходит ошибка"
echo ""

# 5. Альтернативная настройка
echo "5. Если ничего не помогает - ручная настройка:"
echo "  Выполните на сервере:"
echo ""
echo "  # Создать/проверить директорию"
echo "  mkdir -p ~/.ssh"
echo "  chmod 700 ~/.ssh"
echo ""
echo "  # Добавить ключ (замените YOUR_PUBKEY на содержимое вашего публичного ключа)"
echo "  echo 'YOUR_PUBKEY' >> ~/.ssh/authorized_keys"
echo "  chmod 600 ~/.ssh/authorized_keys"
echo ""
echo "  # Проверить владельца"
echo "  chown -R \$USER:\$USER ~/.ssh"
echo ""

echo "Ваш публичный ключ:"
cat "${SSH_KEY}.pub"
echo ""

echo "=== ПОПРОБУЙТЕ РУЧНУЮ НАСТРОЙКУ ==="
echo "Если автоматическая настройка не работает,"
echo "выполните ручные команды на сервере."
