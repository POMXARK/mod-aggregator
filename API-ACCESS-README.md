# 🚀 ДОСТУП К API ERKAPHARM

## 🎯 БЫСТРЫЙ ЗАПУСК

### 1. Запуск API в браузере
```cmd
# В папке mod-aggregator
.\open-api-browser.bat
```

### 2. Ручной запуск
```bash
# В WSL
cd /mnt/c/Users/User/mod-aggregator
./start-ssh-tunnel.sh

# Затем в браузере
https://localhost:9443
```

## 📋 ЧТО ПРОИСХОДИТ

1. **SSH туннель** подключает `localhost:9443` → `api.erkapharm.com:443`
2. **Браузер** открывает `https://localhost:9443`
3. **SSL предупреждение** - принимаем (сертификат для localhost)
4. **Swagger UI** должен загрузиться

## 🔧 РЕШЕНИЕ ПРОБЛЕМ

### ❌ 403 Forbidden
- Обновите страницу: `Ctrl+F5`
- Очистите кэш браузера
- Попробуйте режим инкогнито

### ❌ SSL ошибка / NET::ERR_CERT_INVALID
- Нажмите "Дополнительно" → "Перейти на localhost (небезопасно)"
- Или добавьте исключение для `localhost:9443`

### ❌ Туннель не работает
```bash
# Проверить
netstat -ano | findstr ":9443"

# Перезапустить
wsl -d Ubuntu -- bash -c "cd /mnt/c/Users/User/mod-aggregator && ./start-ssh-tunnel.sh"
```

## 🌐 АЛЬТЕРНАТИВНЫЕ СПОСОБЫ

### Через прокси сервер
```bash
# Запуск Node.js прокси
wsl -d Ubuntu -- bash -c "cd ~ && node api-proxy.js"

# Открыть
http://localhost:3000
```

### Через PowerShell
```powershell
.\test-api-fixed.ps1
```

### Через curl
```bash
# Из WSL
curl -k -H 'Host: api.erkapharm.com' https://localhost:9443
```

## ✅ ГОТОВО!

**Запустите `.\open-api-browser.bat` - API ErkaPharm откроется в браузере! 🚀**



































