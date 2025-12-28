# Быстрый запуск тестов коллекций

## Способ 1: Через DevTools (Рекомендуется)

1. **Дождитесь запуска приложения** (окно Tauri должно открыться)

2. **Откройте DevTools:**
   - Нажмите `F12` или
   - `Ctrl+Shift+I` (Windows/Linux) или `Cmd+Option+I` (Mac)
   - Или через меню: View → Developer Tools

3. **Откройте вкладку Console**

4. **Скопируйте и вставьте** содержимое файла `test-collection-commands.js` в консоль

5. **Запустите тесты:**
   ```javascript
   runFullTest()
   ```

## Способ 2: Через HTML страницу

1. Откройте файл `test-collections-runner.html` в браузере
2. Нажмите кнопку "Запустить полный тест"

## Способ 3: Прямо в консоли (быстрый тест)

Если приложение уже запущено, выполните в консоли DevTools:

```javascript
// Быстрый тест создания коллекции
const { invoke } = window.__TAURI__.core;

async function quickTest() {
    try {
        // Создать коллекцию
        const collection = await invoke('create_collection', {
            params: {
                name: "QuickTestCollection",
                description: "Quick test"
            }
        });
        console.log("✅ Collection created:", collection);
        
        // Получить все коллекции
        const collections = await invoke('get_collections');
        console.log("✅ All collections:", collections);
        
        console.log("✅ Quick test passed!");
    } catch (e) {
        console.error("❌ Error:", e);
    }
}

quickTest();
```

## Что проверяется

- ✅ Создание коллекций
- ✅ Добавление файлов в коллекции
- ✅ Создание правил логики
- ✅ Оценка логики коллекций
- ✅ Объединение коллекций
- ✅ Получение файлов из нескольких коллекций

## Ожидаемый результат

При успешном выполнении вы увидите:

```
=========================================
✓ All tests completed successfully!
=========================================
```

## Если что-то не работает

1. **Проверьте, что приложение запущено:**
   ```bash
   # Если не запущено, выполните:
   npm run tauri dev
   ```

2. **Проверьте консоль на ошибки:**
   - Откройте DevTools → Console
   - Ищите сообщения об ошибках

3. **Проверьте, что команды зарегистрированы:**
   - Откройте `src-tauri/src/main.rs`
   - Убедитесь, что команды коллекций в списке `tauri::generate_handler!`

4. **Проверьте базу данных:**
   - Убедитесь, что таблицы созданы (миграции выполнены)
   - Проверьте файл `mod_aggregator.db` в директории приложения

## Отладка

Если тесты падают с ошибкой:

1. **Ошибка "Command not found":**
   - Проверьте регистрацию команды в `main.rs`
   - Перезапустите приложение

2. **Ошибка "Collection not found":**
   - Проверьте, что коллекция создана
   - Используйте `get_collections()` для проверки

3. **Ошибка "Database error":**
   - Проверьте подключение к БД
   - Убедитесь, что миграции выполнены

## Полезные команды для отладки

```javascript
// Получить все коллекции
await invoke('get_collections')

// Получить файлы коллекции
await invoke('get_collection_files', { collection_id: 1 })

// Получить правила логики
await invoke('get_collection_logic_rules', { collection_id: 1 })

// Оценить логику
await invoke('evaluate_collection_logic', { collection_id: 1 })
```
























