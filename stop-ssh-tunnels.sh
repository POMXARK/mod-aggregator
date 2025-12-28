#!/bin/bash

echo "=== Остановка SSH туннелей ==="

# Останавливаем все SSH туннели
echo "Останавливаем SSH процессы..."
pkill -f "ssh.*xcom@xcom-01.dev.erkapharm.ru" 2>/dev/null && echo "✓ SSH туннели остановлены" || echo "⚠️  SSH туннели не найдены"

echo ""
echo "Проверка портов:"
netstat -tln 2>/dev/null | grep -E ":8194|:9443|:9996|:9998|:11006|:9999" && echo "❌ Некоторые порты все еще открыты" || echo "✓ Все порты освобождены"

echo ""
echo "=== ГОТОВО ==="
echo "SSH туннели остановлены"
























