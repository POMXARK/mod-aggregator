# API Contracts: Session State

**Feature**: 003-file-dependency-system  
**Date**: 2025-12-19  
**Status**: Design Complete

## Tauri Commands: Session State

### `get_session_state() -> Result<SessionState, String>`

Получить сохраненное состояние сессии.

**Response**:
```rust
Ok(SessionState)
// или
Err(String)
```

**SessionState**:
```rust
{
  file_order: Vec<i64>, // file_ids в порядке отображения
  ui_preferences: {
    view_mode: "list" | "tiles",
    window_width: Option<i32>,
    window_height: Option<i32>,
    sidebar_collapsed: Option<bool>,
    theme: Option<"dark" | "light">
  },
  open_collections: Vec<i64>, // collection_ids
  selected_files: Vec<i64>, // file_ids
  recent_actions: Vec<RecentAction>
}
```

**RecentAction**:
```rust
{
  type: "file_added" | "file_deleted" | "collection_created" | "dependency_added" | "dependency_removed" | "collection_updated",
  target_id: i64,
  target_name: String, // name@version для файлов, name для коллекций
  timestamp: String // ISO 8601
}
```

**Note**: Если состояние не существует, возвращает состояние по умолчанию.

---

### `update_file_order(file_order: Vec<i64>) -> Result<(), String>`

Обновить порядок файлов.

**Request**:
```rust
file_order: Vec<i64> // file_ids в новом порядке
```

**Response**:
```rust
Ok(())
// или
Err(String)
```

**Errors**:
- `"Invalid file IDs"` - один из file_id не существует

**Note**: Автоматически сохраняется в БД.

---

### `update_ui_preferences(preferences: UiPreferences) -> Result<(), String>`

Обновить настройки UI.

**Request**:
```rust
{
  view_mode: Option<"list" | "tiles">,
  window_width: Option<i32>,
  window_height: Option<i32>,
  sidebar_collapsed: Option<bool>,
  theme: Option<"dark" | "light">
}
```

**Response**:
```rust
Ok(())
// или
Err(String)
```

**Note**: Обновляет только указанные поля, остальные остаются без изменений.

---

### `update_open_collections(collection_ids: Vec<i64>) -> Result<(), String>`

Обновить список открытых коллекций.

**Request**:
```rust
collection_ids: Vec<i64>
```

**Response**:
```rust
Ok(())
// или
Err(String)
```

**Errors**:
- `"Collection not found"` - одна из коллекций не существует

---

### `update_selected_files(file_ids: Vec<i64>) -> Result<(), String>`

Обновить список выбранных файлов.

**Request**:
```rust
file_ids: Vec<i64>
```

**Response**:
```rust
Ok(())
// или
Err(String)
```

**Errors**:
- `"File not found"` - один из файлов не существует

---

### `add_recent_action(action: RecentAction) -> Result<(), String>`

Добавить действие в историю.

**Request**:
```rust
{
  type: "file_added" | "file_deleted" | "collection_created" | "dependency_added" | "dependency_removed" | "collection_updated",
  target_id: i64,
  target_name: String
}
```

**Response**:
```rust
Ok(())
// или
Err(String)
```

**Note**: 
- Автоматически ограничивает историю до 50 записей (удаляет самые старые)
- Автоматически добавляет текущий timestamp

---

### `get_recent_actions(limit: Option<usize>) -> Result<Vec<RecentAction>, String>`

Получить последние действия.

**Request**:
```rust
limit: Option<usize> // по умолчанию 50
```

**Response**:
```rust
Ok(Vec<RecentAction>)
// или
Err(String)
```

**Note**: Возвращает действия, отсортированные по времени (новые первыми).

---

### `clear_recent_actions() -> Result<(), String>`

Очистить историю действий.

**Response**:
```rust
Ok(())
// или
Err(String)
```

---

### `restore_session() -> Result<SessionRestoreResult, String>`

Восстановить сессию при запуске приложения.

**Response**:
```rust
Ok(SessionRestoreResult)
// или
Err(String)
```

**SessionRestoreResult**:
```rust
{
  restored: bool,
  file_order: Vec<i64>,
  ui_preferences: UiPreferences,
  open_collections: Vec<i64>,
  selected_files: Vec<i64>,
  warnings: Vec<String> // предупреждения о недоступных файлах/коллекциях
}
```

**Behavior**:
- Загружает сохраненное состояние из БД
- Проверяет существование файлов и коллекций
- Если файл/коллекция не существует: удаляет из состояния и добавляет предупреждение
- Возвращает восстановленное состояние с предупреждениями

---

### `reset_session_state() -> Result<(), String>`

Сбросить состояние сессии к значениям по умолчанию.

**Response**:
```rust
Ok(())
// или
Err(String)
```

**Note**: Удаляет все сохраненные настройки и возвращает к начальному состоянию.

