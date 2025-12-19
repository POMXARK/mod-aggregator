# 🔧 ModHeader - исправление CORS конфигурации

## 📊 АНАЛИЗ НОВОГО HAR ЛОГА:

**Проблема найдена!** В OPTIONS запросе есть неправильный заголовок:
```
"access-control-allow-origin": "*"
```

Это **response header**, который нельзя отправлять в request!

## ❌ ЧТО НЕПРАВИЛЬНО:

ModHeader добавляет CORS response headers в request, что ломает запрос.

## ✅ ПРАВИЛЬНАЯ КОНФИГУРАЦИЯ:

### Правило 1: Filter (обязательно!)
```
Type: Filter
Request URL: https://localhost:9443/*
Comment: Ограничивает область действия
```

### Правило 2: Request Header (главное!)
```
Type: Request header
Name: Host
Value: api.erkapharm.com
Comment: Исправляет 403 ошибку
```

### Правило 3: Redirect URL (работает!)
```
Type: Redirect URL
Original URL: https://api.erkapharm.com/(.*)
Redirect URL: https://localhost:9443/$1
Comment: Перехватывает API вызовы
```

## 🚨 УБЕРИТЕ ВСЕ CORS HEADERS ИЗ MODHEADER!

**Удалите все правила типа:**
- ❌ `Access-Control-Allow-Origin`
- ❌ `Access-Control-Allow-Methods`
- ❌ `Access-Control-Allow-Headers`

Эти headers должны приходить **из ответа сервера**, а не отправляться в запросе!

## 🧪 ТЕСТИРОВАНИЕ:

1. **Удалите все CORS правила** из ModHeader
2. **Оставьте только**: Filter + Host header + Redirect URL
3. **Обновите страницу** `https://localhost:9443`
4. **Протестируйте API** в Swagger UI

## 🔧 АЛЬТЕРНАТИВА:

```cmd
.\start-api-proxy.bat
# http://localhost:3000
```

## ✅ ВЫВОД:

**Удалите CORS headers из request - они ломают запрос!** Сервер сам отправит правильные CORS headers в ответе.

**Оставьте только Host header в Request Headers! 🚀**
