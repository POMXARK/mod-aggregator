#!/bin/bash

# Примеры запуска OpenConnect для разных VPN серверов
# Замени значения на свои!

echo "=== Примеры запуска OpenConnect ==="
echo ""

# 1. Базовое подключение с запросом пароля
echo "1. Базовое подключение (запросит пароль):"
echo "./vpn-connect.sh openconnect vpn.company.com --user=your_username"
echo ""

# 2. С указанием группы/реалма
echo "2. С указанием группы аутентификации:"
echo "./vpn-connect.sh openconnect vpn.company.com --user=user --authgroup=GROUP_NAME"
echo ""

# 3. Без проверки сертификата (для самоподписанных)
echo "3. Без проверки сертификата:"
echo "./vpn-connect.sh openconnect vpn.company.com --user=user --no-cert-check"
echo ""

# 4. С PIN сертификата сервера
echo "4. С PIN сертификата сервера:"
echo "./vpn-connect.sh openconnect vpn.company.com --user=user --servercert pin-sha256:YOUR_CERT_PIN"
echo ""

# 5. С протоколом DTLS
echo "5. С DTLS для лучшей производительности:"
echo "./vpn-connect.sh openconnect vpn.company.com --user=user --no-dtls"
echo ""

# 6. С кастомными заголовками (для некоторых серверов)
echo "6. С кастомными заголовками:"
echo "./vpn-connect.sh openconnect vpn.company.com --user=user --header='X-Custom-Header: value'"
echo ""

# 7. Через прокси (если VPN сервер за прокси)
echo "7. Через HTTP прокси:"
echo "./vpn-connect.sh openconnect vpn.company.com --user=user --proxy=http://proxy.company.com:8080"
echo ""

echo "=== Что нужно предоставить для запуска ==="
echo ""
echo "Обязательные параметры:"
echo "- VPN_SERVER: адрес VPN сервера (vpn.company.com или IP)"
echo "- USERNAME: имя пользователя для аутентификации"
echo ""
echo "Опциональные параметры:"
echo "- PASSWORD: пароль (или будет запрошен интерактивно)"
echo "- AUTH_GROUP: группа аутентификации (если требуется)"
echo "- CERT_PIN: PIN сертификата сервера (для безопасности)"
echo "- PROXY: HTTP прокси если VPN за прокси"
echo ""
echo "Пример полной команды:"
echo "./vpn-connect.sh openconnect vpn.office.com --user=john.doe --authgroup=Employees --servercert pin-sha256:abc123..."
