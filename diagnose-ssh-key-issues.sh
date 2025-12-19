#!/bin/bash

# Полная диагностика проблем с SSH ключом
SSH_HOST="xcom@xcom-01.dev.erkapharm.ru"

# Определяем какой ключ использовать
SSH_KEYS=(
    "$HOME/.ssh/id_rsa_erkapharm_new"
    "$HOME/.ssh/id_rsa_erkapharm"
    "$HOME/.ssh/id_rsa"
)

echo "=== ДИАГНОСТИКА SSH КЛЮЧА ==="
echo "Сервер: $SSH_HOST"
echo ""

# 1. Проверяем локальные ключи
echo "1. Проверяем локальные SSH ключи:"
for key in "${SSH_KEYS[@]}"; do
    if [ -f "$key" ]; then
        echo "  ✓ $key существует"
        # Проверяем права
        perms=$(stat -c "%a" "$key" 2>/dev/null || echo "unknown")
        if [ "$perms" = "600" ]; then
            echo "    ✓ Правильные права (600)"
        else
            echo "    ❌ Неправильные права: $perms (должно быть 600)"
        fi
    else
        echo "  ❌ $key не найден"
    fi
done
echo ""

# 2. Проверяем fingerprint ключей
echo "2. Fingerprint локальных ключей:"
for key in "${SSH_KEYS[@]}"; do
    if [ -f "$key" ]; then
        echo "  $key:"
        ssh-keygen -lf "$key" 2>/dev/null || echo "    ❌ Ошибка чтения ключа"
    fi
done
echo ""

# 3. Тестируем базовое подключение
echo "3. Тестируем базовое подключение:"
echo "  Подключение с паролем (введите пароль):"
ssh -o ConnectTimeout=10 "$SSH_HOST" 'echo "SSH с паролем работает"' 2>/dev/null
if [ $? -eq 0 ]; then
    echo "  ✓ SSH с паролем работает"
else
    echo "  ❌ SSH с паролем НЕ работает"
    echo "  Проверьте сеть или доступность сервера"
    exit 1
fi
echo ""

# 4. Проверяем authorized_keys на сервере
echo "4. Проверяем authorized_keys на сервере:"
echo "  Выполните на сервере (введите пароль):"
echo ""
echo "  # Проверить файл"
echo "  cat ~/.ssh/authorized_keys"
echo ""
echo "  # Проверить права"
echo "  ls -la ~/.ssh/authorized_keys"
echo ""
echo "  # Проверить содержимое"
echo "  ssh-keygen -lf ~/.ssh/authorized_keys 2>/dev/null || echo 'Ошибка чтения ключей'"
echo ""

# 5. Тестируем pubkey аутентификацию
echo "5. Тестируем pubkey аутентификацию:"
for key in "${SSH_KEYS[@]}"; do
    if [ -f "$key" ]; then
        echo "  Тестируем $key:"
        ssh -i "$key" -o PasswordAuthentication=no -o ConnectTimeout=5 "$SSH_HOST" 'echo "SSH без пароля работает!"' 2>/dev/null
        if [ $? -eq 0 ]; then
            echo "    ✓ Ключ $key работает!"
            break
        else
            echo "    ❌ Ключ $key не работает"
        fi
    fi
done
echo ""

# 6. Проверяем SSH конфигурацию сервера
echo "6. Проверьте SSH конфигурацию на сервере:"
echo "  Выполните на сервере:"
echo ""
echo "  # Проверить pubkey аутентификацию"
echo "  sudo grep -E '^(PubkeyAuthentication|AuthorizedKeysFile|PasswordAuthentication)' /etc/ssh/sshd_config"
echo ""
echo "  # Проверить права на .ssh"
echo "  ls -la ~/.ssh/"
echo "  stat -c '%a %U:%G' ~/.ssh/"
echo ""

# 7. Детальная диагностика
echo "7. Детальная диагностика SSH:"
echo "  Запустите с локальной машины:"
echo "  ssh -vvv -i ~/.ssh/id_rsa_erkapharm $SSH_HOST"
echo "  И посмотрите где происходит ошибка"
echo ""

echo "=== ВОЗМОЖНЫЕ ПРОБЛЕМЫ ==="
echo "❌ Неправильные права на файлы (~/.ssh/ должен быть 700, ключи 600)"
echo "❌ Ключ поврежден или неправильный формат"
echo "❌ SSH сервер не принимает pubkey аутентификацию"
echo "❌ Ключ не добавлен в authorized_keys"
echo "❌ Проблемы с SSH агентом"
echo ""
echo "=== РЕШЕНИЯ ==="
echo "1. Проверьте права: chmod 700 ~/.ssh && chmod 600 ~/.ssh/*"
echo "2. Перегенерируйте ключ: ./generate-new-ssh-key.sh"
echo "3. Проверьте authorized_keys на сервере"
echo "4. Перезапустите SSH: sudo systemctl restart sshd"
