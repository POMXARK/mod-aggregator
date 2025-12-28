# Тестирование команд коллекций

## Быстрый старт

### 1. Запуск приложения

```bash
npm run tauri dev
```

### 2. Открытие DevTools

- Нажмите `F12` или
- Через меню приложения: View → Developer Tools

### 3. Запуск тестов

Скопируйте содержимое `test-collection-commands.js` в консоль DevTools и выполните:

```javascript
runFullTest()
```

## Что тестируется

### ✅ Базовые операции
- Создание коллекций
- Получение списка коллекций
- Обновление коллекций
- Удаление коллекций

### ✅ Работа с файлами
- Добавление файлов в коллекции
- Получение файлов коллекции
- Удаление файлов из коллекций
- Изменение порядка файлов

### ✅ Правила логики
- Создание правил логики (boolean, file_check, collection_check)
- Получение правил логики
- Оценка логики коллекций

### ✅ Продвинутые операции
- Объединение коллекций
- Получение файлов из нескольких коллекций

## Ожидаемый результат

При успешном выполнении вы увидите:

```
=========================================
✓ All tests completed successfully!
=========================================
```

## Отдельные тесты

Вы можете запускать отдельные тесты:

```javascript
// Создать коллекцию
const collection = await testCreateCollection();

// Добавить файл в коллекцию
await testAddFileToCollection(collection.collection1.id, fileId);

// Создать правило логики
await testCreateLogicRule(
  collection.collection1.id,
  "My Rule",
  "boolean",
  { value: true },
  "enable"
);

// Оценить логику
await testEvaluateCollectionLogic(collection.collection1.id);
```

## Тестирование в браузере

Если используете моки (`tauri-mock.ts`), тесты также работают в браузере:

```bash
npm run dev
```

Откройте консоль браузера и выполните те же команды.

## Отладка

### Проблемы с командами

Если команда не найдена:
1. Проверьте, что команда зарегистрирована в `src-tauri/src/main.rs`
2. Убедитесь, что модуль `collections` включен в `src-tauri/src/commands/mod.rs`

### Проблемы с базой данных

Если возникают ошибки БД:
1. Проверьте, что миграции выполнены (таблицы `collections`, `collection_files`, `collection_logic_rules`)
2. Убедитесь, что база данных инициализирована

### Проблемы с логикой

Если оценка логики не работает:
1. Проверьте, что правила логики созданы правильно
2. Убедитесь, что типы условий соответствуют ожидаемым форматам

## Примеры использования

### Создание коллекции с логикой

```javascript
// 1. Создать коллекцию
const collection = await invoke('create_collection', {
  params: {
    name: "MyCollection",
    description: "Collection with logic"
  }
});

// 2. Добавить файлы
await invoke('add_file_to_collection', {
  params: {
    collection_id: collection.id,
    file_id: 1,
    order_index: 0
  }
});

// 3. Создать правило логики
const rule = await invoke('create_collection_logic_rule', {
  params: {
    collection_id: collection.id,
    name: "Enable if file exists",
    condition_type: "file_check",
    condition_params: {
      file_name: "TestMod",
      file_version: "1.0.0"
    },
    action: "enable"
  }
});

// 4. Оценить логику
const evaluation = await invoke('evaluate_collection_logic', {
  collection_id: collection.id
});

console.log("Enabled files:", evaluation.enabled_files);
console.log("Disabled files:", evaluation.disabled_files);
```

### Объединение коллекций

```javascript
// Объединить две коллекции
const combined = await invoke('combine_collections', {
  params: {
    name: "CombinedCollection",
    source_collection_ids: [1, 2],
    selected_file_ids: null // Все файлы
  }
});
```

## Примечания

- Тестовые данные сохраняются в базе данных
- Для очистки данных используйте `testDeleteCollection()`
- Моки в браузере не сохраняют данные между перезагрузками
























