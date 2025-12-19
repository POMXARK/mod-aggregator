#!/bin/bash

# Исправление прав доступа для SSH ключей
echo "=== Исправление прав доступа SSH ==="

SSH_DIR="$HOME/.ssh"

# Создаем директорию если не существует
if [ ! -d "$SSH_DIR" ]; then
    mkdir -p "$SSH_DIR"
    echo "✓ Создана директория $SSH_DIR"
fi

# Устанавливаем правильные права на директорию
chmod 700 "$SSH_DIR"
echo "✓ Права 700 на $SSH_DIR"

# Устанавливаем правильные права на все файлы в .ssh
for file in "$SSH_DIR"/*; do
    if [ -f "$file" ]; then
        if [[ "$file" == *".pub" ]]; then
            # Публичные ключи - 644
            chmod 644 "$file"
            echo "✓ Права 644 на $file"
        elif [[ "$file" == *"known_hosts"* ]]; then
            # known_hosts - 644
            chmod 644 "$file"
            echo "✓ Права 644 на $file"
        elif [[ "$file" == *"config"* ]]; then
            # config - 644
            chmod 644 "$file"
            echo "✓ Права 644 на $file"
        else
            # Приватные ключи - 600
            chmod 600 "$file"
            echo "✓ Права 600 на $file (приватный ключ)"
        fi
    fi
done

# Проверяем финальные права
echo ""
echo "Финальные права:"
ls -la "$SSH_DIR"

echo ""
echo "Тестируем SSH ключи:"
for key in "$SSH_DIR"/id_*; do
    if [ -f "$key" ] && [[ "$key" != *".pub" ]]; then
        echo "Проверяем $key:"
        ssh-keygen -lf "$key" 2>/dev/null && echo "  ✓ Ключ валиден" || echo "  ❌ Ключ поврежден"
    fi
done

echo ""
echo "=== ГОТОВО ==="
echo "Теперь попробуйте: ssh xcom@xcom-01.dev.erkapharm.ru"
echo "Если не работает - запустите: ./diagnose-ssh-key-issues.sh"
