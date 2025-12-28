#!/bin/bash
# Скрипт для проверки предварительных требований для SpecKit

set -e

echo "Проверка предварительных требований для SpecKit..."

# Проверка Node.js
if ! command -v node &> /dev/null; then
    echo "❌ Node.js не установлен"
    exit 1
else
    echo "✅ Node.js установлен: $(node --version)"
fi

# Проверка npm
if ! command -v npm &> /dev/null; then
    echo "❌ npm не установлен"
    exit 1
else
    echo "✅ npm установлен: $(npm --version)"
fi

# Проверка Rust
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust не установлен"
    exit 1
else
    echo "✅ Rust установлен: $(rustc --version)"
fi

# Проверка структуры директорий
if [ ! -d "specs" ]; then
    echo "❌ Директория specs/ не найдена"
    exit 1
else
    echo "✅ Директория specs/ существует"
fi

if [ ! -d "memory" ]; then
    echo "❌ Директория memory/ не найдена"
    exit 1
else
    echo "✅ Директория memory/ существует"
fi

if [ ! -d "templates" ]; then
    echo "❌ Директория templates/ не найдена"
    exit 1
else
    echo "✅ Директория templates/ существует"
fi

# Проверка конституции
if [ ! -f "memory/constitution.md" ]; then
    echo "❌ Файл memory/constitution.md не найден"
    exit 1
else
    echo "✅ Файл memory/constitution.md существует"
fi

echo ""
echo "✅ Все предварительные требования выполнены!"
























