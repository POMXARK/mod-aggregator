#!/bin/bash

# Скрипт для копирования SSH ключа на другую машину
echo "=== Копирование SSH ключа на другую машину ==="

# Проверяем существующие ключи
SSH_KEYS=(
    "$HOME/.ssh/id_rsa_erkapharm_new"
    "$HOME/.ssh/id_rsa_erkapharm"
    "$HOME/.ssh/id_rsa"
)

echo "Доступные SSH ключи:"
for i in "${!SSH_KEYS[@]}"; do
    if [ -f "${SSH_KEYS[$i]}" ]; then
        echo "  $((i+1)). ${SSH_KEYS[$i]} ✓"
    else
        echo "  $((i+1)). ${SSH_KEYS[$i]} ❌"
    fi
done
echo ""

# Выбираем ключ для копирования
read -p "Выберите номер ключа для копирования (1-${#SSH_KEYS[@]}): " key_num

if ! [[ "$key_num" =~ ^[0-9]+$ ]] || [ "$key_num" -lt 1 ] || [ "$key_num" -gt "${#SSH_KEYS[@]}" ]; then
    echo "❌ Неверный номер"
    exit 1
fi

SELECTED_KEY="${SSH_KEYS[$((key_num-1))]}"

if [ ! -f "$SELECTED_KEY" ]; then
    echo "❌ Выбранный ключ не существует: $SELECTED_KEY"
    exit 1
fi

echo "Выбран ключ: $SELECTED_KEY"
echo ""

# Запрашиваем данные целевой машины
read -p "IP или hostname целевой машины: " TARGET_HOST
read -p "Пользователь на целевой машине: " TARGET_USER

TARGET="${TARGET_USER}@${TARGET_HOST}"

echo ""
echo "Копируем ключ на: $TARGET"
echo "Пароль потребуется для первоначального подключения"
echo ""

# Копируем ключ
ssh-copy-id -i "$SELECTED_KEY" "$TARGET"

if [ $? -eq 0 ]; then
    echo ""
    echo "✓ Ключ успешно скопирован!"
    echo ""
    echo "Тестируем подключение:"
    ssh -i "$SELECTED_KEY" -o ConnectTimeout=5 "$TARGET" 'echo "SSH без пароля работает на $(hostname)!"'

    if [ $? -eq 0 ]; then
        echo ""
        echo "🎉 ГОТОВО! SSH ключ работает на новой машине!"
        echo ""
        echo "Теперь можно использовать этот ключ с:"
        echo "  ssh -i $SELECTED_KEY $TARGET"
    else
        echo ""
        echo "❌ Тест не прошел"
    fi
else
    echo ""
    echo "❌ Ошибка копирования ключа"
fi
