# Quick Start: File Dependency Management and Advanced Collections

**Feature**: 003-file-dependency-system  
**Date**: 2025-12-19

## Быстрый старт

Это руководство поможет быстро начать работу с системой управления зависимостями файлов и продвинутыми коллекциями.

## Основные концепции

### Идентификация файлов

Файлы идентифицируются комбинацией **name@version** (например, `my-mod@1.0.0`), аналогично npm пакетам. Это позволяет:
- Хранить несколько версий одного файла одновременно
- Точное указание версий в зависимостях
- Разрешение конфликтов версий

### Зависимости файлов

Файлы могут зависеть от других файлов. Типы зависимостей:
- **required**: обязательная зависимость (файл не может работать без неё)
- **optional**: опциональная зависимость (файл может работать без неё)
- **peer**: зависимость на уровне (как в npm peer dependencies)

### Коллекции с логикой

Коллекции могут содержать файлы с условной логикой включения/выключения:
- Простые условия (boolean)
- Проверка активности других коллекций
- Проверка установки файлов
- Комбинации условий (AND/OR)

## Типичные сценарии использования

### 1. Добавление файла с зависимостями

```typescript
// Frontend (Svelte 5)
import { invoke } from '@tauri-apps/api/core';

// Создать файл
const file = await invoke('create_file', {
  name: 'my-mod',
  version: '1.0.0',
  path: '/path/to/file',
  dependencies: [
    {
      target_file_name: 'base-mod',
      target_file_version: '2.0.0',
      dependency_type: 'required'
    }
  ]
});

// Проверить зависимости
const check = await invoke('check_dependencies', { file_id: file.id });
if (check.missing_dependencies.length > 0) {
  console.warn('Отсутствующие зависимости:', check.missing_dependencies);
}
```

### 2. Создание коллекции с логикой

```typescript
// Создать коллекцию
const collection = await invoke('create_collection', {
  name: 'My Collection',
  description: 'Collection with logic'
});

// Добавить файл с правилом логики
const logicRule = await invoke('create_collection_logic_rule', {
  collection_id: collection.id,
  name: 'Enable if base mod installed',
  condition_type: 'file_check',
  condition_params: {
    file_name: 'base-mod',
    file_version: '2.0.0'
  },
  action: 'enable'
});

await invoke('add_file_to_collection', {
  collection_id: collection.id,
  file_id: file.id,
  logic_rule_id: logicRule.id
});
```

### 3. Импорт/экспорт файла

```typescript
// Экспорт файла
const json = await invoke('export_file', { file_id: 123 });
// Сохранить в файл или поделиться

// Импорт файла
const result = await invoke('import_file', { json_data: json });
if (result.missing_dependencies.length > 0) {
  // Предложить пользователю установить зависимости
  console.warn('Недостающие зависимости:', result.missing_dependencies);
}
```

### 4. Пакетные операции

```typescript
// Выбрать несколько файлов
const selectedFiles = [1, 2, 3, 4, 5];

// Проверить зависимости для всех
const batchCheck = await invoke('batch_check_dependencies', {
  file_ids: selectedFiles
});

// Удалить файлы (с проверкой зависимостей)
const deleteResult = await invoke('batch_delete_files', {
  file_ids: selectedFiles,
  force: false // false = блокировать удаление если есть зависимые файлы
});

if (deleteResult.blocked_files.length > 0) {
  // Показать предупреждение о заблокированных файлах
  console.warn('Не удалось удалить:', deleteResult.blocked_files);
}
```

### 5. Сохранение и восстановление состояния

```typescript
// Сохранить порядок файлов после drag & drop
await invoke('update_file_order', {
  file_order: [3, 1, 5, 2, 4] // новый порядок file_ids
});

// Сохранить настройки UI
await invoke('update_ui_preferences', {
  view_mode: 'tiles',
  sidebar_collapsed: false
});

// При запуске приложения - восстановить состояние
const restored = await invoke('restore_session');
console.log('Восстановлено:', restored);
```

## UI Компоненты

### DependencyEditor

Компонент для редактирования зависимостей файла:

```svelte
<script lang="ts">
  import DependencyEditor from '$lib/components/dependencies/DependencyEditor.svelte';
  
  let fileId = $state(123);
</script>

<DependencyEditor {fileId} />
```

### CollectionLogicBuilder

UI-конструктор для создания правил логики коллекций:

```svelte
<script lang="ts">
  import CollectionLogicBuilder from '$lib/components/collections/CollectionLogicBuilder.svelte';
  
  let collectionId = $state(456);
</script>

<CollectionLogicBuilder {collectionId} />
```

### FileList с множественным выбором

Список файлов с поддержкой множественного выбора и drag & drop:

```svelte
<script lang="ts">
  import FileList from '$lib/components/files/FileList.svelte';
  
  let selectedFiles = $state<number[]>([]);
  
  function handleSelection(ids: number[]) {
    selectedFiles = ids;
  }
</script>

<FileList on:select={handleSelection} />
```

## Backend примеры (Rust)

### Создание зависимости

```rust
// src-tauri/src/commands/dependencies.rs
#[tauri::command]
pub async fn add_file_dependency(
    params: AddDependencyParams,
    state: State<'_, AppState>,
) -> Result<FileDependency, String> {
    // Проверка на циклические зависимости
    let has_cycle = check_circular_dependencies(
        &state.db,
        params.source_file_id,
        &params.target_file_name,
        params.target_file_version.as_deref(),
    ).await?;
    
    if has_cycle {
        return Err("Circular dependency detected".to_string());
    }
    
    // Добавление зависимости
    // ...
}
```

### Вычисление логики коллекции

```rust
// src-tauri/src/services/collection_service.rs
pub async fn evaluate_collection_logic(
    db: &Database,
    collection_id: i64,
) -> Result<CollectionEvaluationResult, String> {
    let rules = get_collection_logic_rules(db, collection_id).await?;
    let files = get_collection_files(db, collection_id).await?;
    
    let mut enabled_files = Vec::new();
    let mut disabled_files = Vec::new();
    
    for file in files {
        let enabled = evaluate_file_logic(&rules, file.logic_rule_id, db).await?;
        if enabled {
            enabled_files.push(file.file_id);
        } else {
            disabled_files.push(file.file_id);
        }
    }
    
    Ok(CollectionEvaluationResult {
        collection_id,
        enabled_files,
        disabled_files,
        // ...
    })
}
```

## Миграции БД

Применение миграций:

```bash
# Миграции применяются автоматически при запуске приложения
# Или вручную через SQLx CLI:

sqlx migrate run
```

Миграции:
1. `003_add_dependencies.sql` - таблицы зависимостей
2. `004_add_collections.sql` - таблицы коллекций и логики
3. `005_add_session_state.sql` - таблица состояния сессии

## Тестирование

### Unit тесты (Rust)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_circular_dependency_detection() {
        // Тест обнаружения циклических зависимостей
    }
    
    #[tokio::test]
    async fn test_dependency_resolution() {
        // Тест разрешения версий зависимостей
    }
}
```

### Component тесты (Frontend)

```typescript
// tests/frontend/components/dependencies/DependencyEditor.test.ts
import { render, screen } from '@testing-library/svelte';
import DependencyEditor from '$lib/components/dependencies/DependencyEditor.svelte';

test('renders dependency editor', () => {
  render(DependencyEditor, { fileId: 123 });
  expect(screen.getByText('Dependencies')).toBeInTheDocument();
});
```

## Отладка

### Проверка графа зависимостей

```typescript
const graph = await invoke('get_dependency_graph', { file_ids: null });
console.log('Graph nodes:', graph.nodes);
console.log('Graph edges:', graph.edges);
```

### Валидация импорта

```typescript
const validation = await invoke('validate_import_json', { json_data: json });
if (!validation.valid) {
  console.error('Ошибки валидации:', validation.errors);
} else {
  console.log('Preview:', validation.preview);
}
```

## Следующие шаги

1. Изучите [data-model.md](./data-model.md) для понимания структуры данных
2. Ознакомьтесь с [contracts/](./contracts/) для деталей API
3. Посмотрите примеры компонентов в `src/components/dependencies/`
4. Изучите сервисы в `src-tauri/src/services/`

## Полезные ссылки

- [Спецификация](./spec.md) - полная спецификация функциональности
- [План реализации](./plan.md) - технический план
- [Исследования](./research.md) - принятые технические решения



































