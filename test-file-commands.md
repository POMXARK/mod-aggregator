# Тестирование команд управления файлами

## Быстрый старт

### Вариант 1: Тестирование через Tauri Dev Mode

1. Запустите приложение в режиме разработки:
```bash
npm run tauri:dev
```

2. Откройте DevTools (F12 или Ctrl+Shift+I)

3. В консоли выполните:
```javascript
// Импортируем invoke
const { invoke } = window.__TAURI__.core;

// Запускаем тесты
await testFileCommands();
```

### Вариант 2: Тестирование через браузер (Mock)

1. Запустите dev сервер:
```bash
npm run dev
```

2. Откройте консоль браузера

3. Выполните команды из `test-file-commands.js` вручную

## Ручное тестирование команд

### 1. Создание файла

```javascript
const file = await invoke('create_file', {
  params: {
    name: 'my-mod',
    version: '1.0.0',
    path: '/path/to/mod',
    metadata: { author: 'Author Name', description: 'Mod description' }
  }
});
console.log('Создан файл:', file);
```

### 2. Создание файла с зависимостями

```javascript
const file = await invoke('create_file', {
  params: {
    name: 'dependent-mod',
    version: '1.0.0',
    dependencies: [{
      target_file_name: 'my-mod',
      target_file_version: '1.0.0',
      dependency_type: 'required'
    }]
  }
});
```

### 3. Получение файла по name@version

```javascript
const file = await invoke('get_file_by_name_version', {
  name: 'my-mod',
  version: '1.0.0'
});
```

### 4. Получение всех версий файла

```javascript
const versions = await invoke('get_file_versions', { name: 'my-mod' });
```

### 5. Обновление файла

```javascript
const updated = await invoke('update_file', {
  params: {
    id: file.id,
    path: '/new/path',
    metadata: { updated: true }
  }
});
```

### 6. Загрузка новой версии

```javascript
const newVersion = await invoke('upload_file_version', {
  params: {
    name: 'my-mod',
    version: '2.0.0',
    path: '/path/to/v2'
  }
});
```

### 7. Удаление файла

```javascript
// Без force (проверка зависимостей)
const result = await invoke('delete_file', {
  file_id: file.id,
  force: false
});

// С force (принудительное удаление)
const result = await invoke('delete_file', {
  file_id: file.id,
  force: true
});
```

## Проверка ошибок

### Дубликат name@version

```javascript
try {
  await invoke('create_file', {
    params: {
      name: 'my-mod',
      version: '1.0.0' // Уже существует
    }
  });
} catch (error) {
  console.log('Ошибка:', error); // Должна быть ошибка о дубликате
}
```

### Пустые поля

```javascript
try {
  await invoke('create_file', {
    params: {
      name: '', // Пустое имя
      version: '1.0.0'
    }
  });
} catch (error) {
  console.log('Ошибка валидации:', error);
}
```

### Циклические зависимости

```javascript
// Создаем файл A, зависящий от B
const fileA = await invoke('create_file', {
  params: {
    name: 'mod-a',
    version: '1.0.0',
    dependencies: [{ target_file_name: 'mod-b', dependency_type: 'required' }]
  }
});

// Пытаемся создать файл B, зависящий от A (цикл!)
try {
  await invoke('create_file', {
    params: {
      name: 'mod-b',
      version: '1.0.0',
      dependencies: [{ target_file_name: 'mod-a', dependency_type: 'required' }]
    }
  });
} catch (error) {
  console.log('Обнаружена циклическая зависимость:', error);
}
```

## Тестирование через UI

1. Запустите приложение: `npm run tauri:dev`
2. Откройте страницу управления файлами (если есть)
3. Используйте форму `FileForm.svelte` для создания/редактирования файлов
4. Проверьте:
   - Создание файла с разными параметрами
   - Добавление зависимостей через форму
   - Редактирование существующего файла
   - Валидацию полей формы
   - Обработку ошибок

## Ожидаемые результаты

✅ Все команды должны работать корректно
✅ Валидация должна предотвращать некорректные данные
✅ Зависимости должны проверяться на циклы
✅ Удаление должно проверять зависимые файлы
✅ Ошибки должны быть информативными

## Известные проблемы

Если обнаружите проблемы, проверьте:
- База данных инициализирована (миграции выполнены)
- Команды зарегистрированы в `main.rs`
- Типы данных соответствуют между frontend и backend
























