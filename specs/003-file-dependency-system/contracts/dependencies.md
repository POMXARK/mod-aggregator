# API Contracts: Dependencies

**Feature**: 003-file-dependency-system  
**Date**: 2025-12-19  
**Status**: Design Complete

## Tauri Commands: Dependencies

### `get_file_dependencies(file_id: i64) -> Result<Vec<FileDependency>, String>`

Получить все зависимости файла.

**Request**:
```rust
file_id: i64
```

**Response**:
```rust
Ok(Vec<FileDependency>)
// или
Err(String) // описание ошибки
```

**FileDependency**:
```rust
{
  id: i64,
  source_file_id: i64,
  target_file_name: String,
  target_file_version: Option<String>,
  dependency_type: "required" | "optional" | "peer",
  created_at: String // ISO 8601
}
```

**Errors**:
- `"File not found"` - файл с указанным ID не существует

---

### `add_file_dependency(params: AddDependencyParams) -> Result<FileDependency, String>`

Добавить зависимость к файлу.

**Request**:
```rust
{
  source_file_id: i64,
  target_file_name: String,
  target_file_version: Option<String>, // NULL = любая версия
  dependency_type: "required" | "optional" | "peer"
}
```

**Response**:
```rust
Ok(FileDependency)
// или
Err(String)
```

**Errors**:
- `"Source file not found"` - исходный файл не существует
- `"Circular dependency detected"` - обнаружена циклическая зависимость
- `"Self-dependency not allowed"` - файл не может зависеть от самого себя
- `"Target version not found"` - указанная версия зависимости не существует (если version указан)

**Validation**:
- Проверка на циклические зависимости через граф
- Проверка существования target файла (если version указан)

---

### `remove_file_dependency(dependency_id: i64) -> Result<(), String>`

Удалить зависимость.

**Request**:
```rust
dependency_id: i64
```

**Response**:
```rust
Ok(())
// или
Err(String)
```

**Errors**:
- `"Dependency not found"` - зависимость не существует

---

### `check_dependencies(file_id: i64) -> Result<DependencyCheckResult, String>`

Проверить состояние зависимостей файла.

**Request**:
```rust
file_id: i64
```

**Response**:
```rust
Ok(DependencyCheckResult)
// или
Err(String)
```

**DependencyCheckResult**:
```rust
{
  file_id: i64,
  missing_dependencies: Vec<MissingDependency>,
  satisfied_dependencies: Vec<i64>, // dependency IDs
  version_conflicts: Vec<VersionConflict>
}
```

**MissingDependency**:
```rust
{
  dependency_id: i64,
  target_file_name: String,
  target_file_version: Option<String>,
  dependency_type: "required" | "optional" | "peer",
  available_versions: Vec<String> // доступные версии, если есть
}
```

**VersionConflict**:
```rust
{
  dependency_id: i64,
  target_file_name: String,
  required_version: Option<String>,
  available_version: String // версия, которая есть, но не подходит
}
```

**Errors**:
- `"File not found"` - файл не существует

---

### `check_circular_dependencies(file_id: i64, target_file_name: String, target_file_version: Option<String>) -> Result<bool, String>`

Проверить, создаст ли добавление зависимости циклическую зависимость.

**Request**:
```rust
{
  file_id: i64,
  target_file_name: String,
  target_file_version: Option<String>
}
```

**Response**:
```rust
Ok(bool) // true = циклическая зависимость обнаружена
// или
Err(String)
```

**Errors**:
- `"File not found"` - файл не существует

---

### `get_dependent_files(file_id: i64) -> Result<Vec<File>, String>`

Получить все файлы, которые зависят от данного файла.

**Request**:
```rust
file_id: i64
```

**Response**:
```rust
Ok(Vec<File>)
// или
Err(String)
```

**File**:
```rust
{
  id: i64,
  name: String,
  version: String,
  path: Option<String>,
  metadata: serde_json::Value,
  created_at: String,
  updated_at: String
}
```

**Errors**:
- `"File not found"` - файл не существует

---

### `resolve_dependency_version(file_id: i64, dependency_id: i64, selected_version: String) -> Result<(), String>`

Разрешить конфликт версии зависимости, выбрав конкретную версию.

**Request**:
```rust
{
  file_id: i64,
  dependency_id: i64,
  selected_version: String
}
```

**Response**:
```rust
Ok(())
// или
Err(String)
```

**Errors**:
- `"File not found"` - файл не существует
- `"Dependency not found"` - зависимость не существует
- `"Version not found"` - выбранная версия не существует

---

### `get_dependency_graph(file_ids: Option<Vec<i64>>) -> Result<DependencyGraph, String>`

Получить граф зависимостей для указанных файлов (или всех, если None).

**Request**:
```rust
file_ids: Option<Vec<i64>> // None = все файлы
```

**Response**:
```rust
Ok(DependencyGraph)
// или
Err(String)
```

**DependencyGraph**:
```rust
{
  nodes: Vec<GraphNode>,
  edges: Vec<GraphEdge>
}
```

**GraphNode**:
```rust
{
  file_id: i64,
  name: String,
  version: String,
  has_missing_dependencies: bool
}
```

**GraphEdge**:
```rust
{
  from_file_id: i64,
  to_file_id: i64,
  dependency_id: i64,
  dependency_type: "required" | "optional" | "peer",
  satisfied: bool
}
```

---

### `get_installation_order(file_ids: Vec<i64>) -> Result<Vec<i64>, String>`

Получить порядок установки файлов с учетом зависимостей (топологическая сортировка).

**Request**:
```rust
file_ids: Vec<i64>
```

**Response**:
```rust
Ok(Vec<i64>) // порядок file_id для установки
// или
Err(String)
```

**Errors**:
- `"Circular dependencies detected"` - обнаружены циклические зависимости
- `"File not found"` - один из файлов не существует

**Note**: Возвращает порядок, при котором все зависимости устанавливаются перед зависимыми файлами.



































