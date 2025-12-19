#!/bin/bash

echo "=== Тестирование API endpoint через туннель ==="

# Тестируем endpoint через туннель
curl -k -X POST "https://localhost:9443/user/login-sms-prestep" \
  -H "Host: api.erkapharm.com" \
  -H "Content-Type: application/json" \
  -H "X-API-Key: ONWb3aDVFtQuDb4S5l" \
  -d '{"phone":"79141888576","marketplace":"superapteka"}' \
  -v

echo ""
echo "=== Результат ==="
echo "Если получили ответ - API работает через туннель"
echo "Если ERR_CONNECTION_REFUSED - проблема с туннелем"

