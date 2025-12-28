# Тестирование команд коллекций

## Описание

Скрипт `test-collection-commands.js` предназначен для тестирования всех команд работы с коллекциями в backend.

## Использование

### В Tauri DevTools

1. Запустите приложение в режиме разработки:
   ```bash
   npm run tauri dev
   ```

2. Откройте DevTools (обычно F12 или через меню)

3. Скопируйте содержимое `test-collection-commands.js` в консоль

4. Запустите полный тест:
   ```javascript
   runFullTest()
   ```

### В браузере (с моками)

Если используете `tauri-mock.ts`, скрипт также должен работать в браузере.

## Тестовые функции

### Базовые операции с коллекциями

- `testCreateCollection()` - создание коллекций
- `testGetCollections()` - получение всех коллекций
- `testUpdateCollection(collectionId, newName, newDescription)` - обновление коллекции
- `testDeleteCollection(collectionId)` - удаление коллекции

### Работа с файлами в коллекциях

- `testAddFileToCollection(collectionId, fileId)` - добавление файла в коллекцию
- `testGetCollectionFiles(collectionId)` - получение файлов коллекции
- `testRemoveFileFromCollection(collectionId, fileId)` - удаление файла из коллекции
- `testReorderCollectionFiles(collectionId, fileOrders)` - изменение порядка файлов

### Правила логики

- `testCreateLogicRule(collectionId, ruleName, conditionType, conditionParams, action)` - создание правила логики
- `testGetLogicRules(collectionId)` - получение правил логики коллекции
- `testEvaluateCollectionLogic(collectionId)` - оценка логики коллекции

### Продвинутые операции

- `testCombineCollections(collectionIds, newName)` - объединение коллекций
- `testGetFilesFromMultipleCollections(collectionIds)` - получение файлов из нескольких коллекций

## Полный тестовый сценарий

Функция `runFullTest()` выполняет полный набор тестов:

1. **Создание коллекций** - создает две тестовые коллекции
2. **Получение коллекций** - проверяет получение списка всех коллекций
3. **Создание файлов** - создает тестовые файлы (или использует существующие)
4. **Добавление файлов** - добавляет файлы в коллекции
5. **Получение файлов** - проверяет получение файлов коллекций
6. **Создание правил логики** - создает правила разных типов:
   - Boolean правило
   - File check правило
   - Collection check правило
7. **Получение правил** - проверяет получение правил логики
8. **Оценка логики** - проверяет оценку логики коллекций
9. **Объединение коллекций** - объединяет две коллекции в одну
10. **Получение из нескольких коллекций** - проверяет получение файлов из нескольких коллекций
11. **Изменение порядка** - проверяет изменение порядка файлов
12. **Обновление коллекции** - обновляет коллекцию
13. **Удаление файла** - удаляет файл из коллекции

## Ожидаемые результаты

### Успешное выполнение

При успешном выполнении вы увидите:
```
=========================================
✓ All tests completed successfully!
=========================================
```

### Примеры вывода

**Создание коллекции:**
```javascript
Created collection1: {
  id: 1,
  name: "TestCollectionA",
  description: "Test collection A",
  created_at: "2025-12-19T10:30:00Z",
  updated_at: "2025-12-19T10:30:00Z"
}
```

**Оценка логики:**
```javascript
Evaluation result: {
  collection_id: 1,
  enabled_files: [1, 2],
  disabled_files: [],
  evaluation_details: [
    { file_id: 1, enabled: true, applied_rules: [1] },
    { file_id: 2, enabled: true, applied_rules: [2] }
  ]
}
```

## Типы условий для правил логики

### Boolean
```javascript
{
  condition_type: "boolean",
  condition_params: { value: true },
  action: "enable"
}
```

### File Check
```javascript
{
  condition_type: "file_check",
  condition_params: { 
    file_name: "TestModB", 
    file_version: "1.0.0" 
  },
  action: "enable"
}
```

### Collection Check
```javascript
{
  condition_type: "collection_check",
  condition_params: { collection_id: 1 },
  action: "enable"
}
```

### And/Or
```javascript
{
  condition_type: "and",
  condition_params: { rules: [1, 2] },
  action: "enable"
}
```

## Отладка

Если тест падает с ошибкой:

1. Проверьте, что backend команды зарегистрированы в `main.rs`
2. Убедитесь, что база данных инициализирована
3. Проверьте консоль на наличие ошибок компиляции
4. Убедитесь, что все зависимости установлены

## Примечания

- Тестовые данные не удаляются автоматически (закомментировано в `runFullTest`)
- Для очистки данных можно вручную вызвать `testDeleteCollection()`
- Скрипт использует существующие файлы, если они уже созданы
























