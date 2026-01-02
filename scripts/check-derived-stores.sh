#!/bin/bash

# Проверка вызова derived stores как функций в Svelte файлах

echo "=================================================="
echo " Проверка Derived Stores в Svelte файлах"
echo "=================================================="

# Проверяем аргументы
if [ $# -lt 1 ]; then
    echo "❌ Использование: $0 <путь> [--fix]"
    echo "  <путь> - файл или директория для анализа"
    echo "  --fix  - автоматически исправить проблемы"
    exit 1
fi

PATH_ARG="$1"
FIX_MODE=false

if [ "$2" = "--fix" ]; then
    FIX_MODE=true
fi

# Проверяем существование пути
if [ ! -e "$PATH_ARG" ]; then
    echo "❌ Путь не существует: $PATH_ARG"
    exit 1
fi

# Получаем список файлов
if [ -f "$PATH_ARG" ]; then
    # Это файл
    files=("$PATH_ARG")
elif [ -d "$PATH_ARG" ]; then
    # Это директория
    echo "ℹ️  Поиск Svelte файлов в директории: $PATH_ARG"
    mapfile -t files < <(find "$PATH_ARG" -name "*.svelte" -type f 2>/dev/null)
    if [ ${#files[@]} -eq 0 ]; then
        echo "⚠️  Svelte файлы не найдены"
        exit 0
    fi
    echo "ℹ️  Найдено ${#files[@]} Svelte файлов"
else
    echo "❌ Неверный тип пути: $PATH_ARG"
    exit 1
fi

problems_found=0
fixes_applied=0

echo "ℹ️  Начинаем анализ..."
echo

# Анализируем каждый файл
for file_path in "${files[@]}"; do
    if [ ! -f "$file_path" ]; then
        continue
    fi

    # Читаем содержимое файла
    content=$(cat "$file_path" 2>/dev/null)
    if [ -z "$content" ]; then
        continue
    fi

    # Ищем проблемы с помощью grep
    problems=$(echo "$content" | grep -n '\$derived\s*(\s*[a-zA-Z_][a-zA-Z0-9_]*\s*(\s*)\s*)' || true)

    if [ -n "$problems" ]; then
        # Считаем количество проблем
        problem_count=$(echo "$problems" | wc -l)
        echo "⚠️  Найдено $problem_count проблем в файле: $file_path"

        # Обрабатываем каждую проблему
        echo "$problems" | while IFS=: read -r line_num line_content; do
            # Извлекаем имя функции из паттерна
            function_name=$(echo "$line_content" | sed -n 's/.*\$derived\s*(\s*\([a-zA-Z_][a-zA-Z0-9_]*\)\s*(\s*)\s*).*/\1/p')

            if [ -n "$function_name" ]; then
                echo "  Строка $line_num: $line_content"
                echo "  Проблема: Вызов derived store '$function_name' как функции"
                echo "  Исправление: \$derived($function_name) вместо \$derived($function_name())"
                echo

                if [ "$FIX_MODE" = true ]; then
                    # Создаем исправление
                    old_pattern="\$derived($function_name())"
                    new_pattern="\$derived($function_name)"

                    # Исправляем в файле
                    sed -i "s/\$derived($function_name())/\$derived($function_name)/g" "$file_path"
                    echo "✅ Исправлено: $old_pattern → $new_pattern"
                    ((fixes_applied++))
                fi

                ((problems_found++))
            fi
        done

        if [ "$FIX_MODE" = true ]; then
            echo "✅ Файл $file_path сохранен"
        fi
    fi
done

echo
echo "=================================================="
echo " ИТОГИ АНАЛИЗА"
echo "=================================================="

if [ $problems_found -eq 0 ]; then
    echo "✅ Проблем не найдено! Все derived stores используются правильно."
else
    echo "⚠️  Найдено проблем: $problems_found"
    if [ "$FIX_MODE" = true ] && [ $fixes_applied -gt 0 ]; then
        echo "✅ Исправлено: $fixes_applied"
    elif [ "$FIX_MODE" = false ]; then
        echo "ℹ️  Используйте --fix для автоматического исправления"
    fi
fi

echo
echo "ℹ️  Анализ завершен"

