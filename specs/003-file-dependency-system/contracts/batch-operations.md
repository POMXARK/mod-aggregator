# API Contracts: Batch Operations

**Feature**: 003-file-dependency-system  
**Date**: 2025-12-19  
**Status**: Design Complete

## Tauri Commands: Batch Operations

### `batch_delete_files(file_ids: Vec<i64>, force: Option<bool>) -> Result<BatchOperationResult, String>`

Удалить несколько файлов одновременно.

**Request**:
```rust
{
  file_ids: Vec<i64>,
  force: Option<bool> // true = удалить даже если есть зависимые файлы
}
```

**Response**:
```rust
Ok(BatchOperationResult)
// или
Err(String)
```

**BatchOperationResult**:
```rust
{
  success_count: usize,
  failed_count: usize,
  results: Vec<FileOperationResult>,
  warnings: Vec<String>,
  blocked_files: Vec<BlockedFile> // если force=false
}
```

**FileOperationResult**:
```rust
{
  file_id: i64,
  success: bool,
  error: Option<String>
}
```

**BlockedFile**:
```rust
{
  file_id: i64,
  file_name: String,
  dependent_files: Vec<i64> // file_ids файлов, зависящих от данного
}
```

**Errors**:
- `"No files specified"` - не указаны файлы для удаления

**Behavior**:
- Если `force=false`: проверяет зависимости для каждого файла
- Файлы с зависимостями не удаляются, возвращаются в `blocked_files`
- Остальные файлы удаляются
- Возвращает детальные результаты для каждого файла

---

### `batch_move_files_to_collection(file_ids: Vec<i64>, collection_id: i64) -> Result<BatchOperationResult, String>`

Переместить несколько файлов в коллекцию.

**Request**:
```rust
{
  file_ids: Vec<i64>,
  collection_id: i64
}
```

**Response**:
```rust
Ok(BatchOperationResult)
// или
Err(String)
```

**Errors**:
- `"Collection not found"` - коллекция не существует
- `"No files specified"` - не указаны файлы

**Behavior**:
- Добавляет файлы в коллекцию
- Если файл уже в коллекции: пропускает с предупреждением
- Возвращает результаты для каждого файла

---

### `batch_update_file_properties(file_ids: Vec<i64>, properties: UpdateFileProperties) -> Result<BatchOperationResult, String>`

Обновить свойства нескольких файлов.

**Request**:
```rust
{
  file_ids: Vec<i64>,
  properties: {
    metadata: Option<serde_json::Value>,
    path: Option<String>
    // name и version не обновляются пакетно (требуют проверки уникальности)
  }
}
```

**Response**:
```rust
Ok(BatchOperationResult)
// или
Err(String)
```

**Errors**:
- `"No files specified"` - не указаны файлы

**Behavior**:
- Обновляет указанные свойства для всех файлов
- Если свойство не указано: не изменяет его
- Возвращает результаты для каждого файла

---

### `batch_add_dependencies(file_id: i64, dependencies: Vec<AddDependencyParams>) -> Result<BatchOperationResult, String>`

Добавить несколько зависимостей к файлу.

**Request**:
```rust
{
  file_id: i64,
  dependencies: Vec<{
    target_file_name: String,
    target_file_version: Option<String>,
    dependency_type: "required" | "optional" | "peer"
  }>
}
```

**Response**:
```rust
Ok(BatchOperationResult)
// или
Err(String)
```

**Errors**:
- `"File not found"` - файл не существует
- `"No dependencies specified"` - не указаны зависимости

**Behavior**:
- Добавляет зависимости по одной
- Если зависимость уже существует: пропускает с предупреждением
- Проверяет циклические зависимости для каждой
- Возвращает результаты для каждой зависимости

---

### `batch_check_dependencies(file_ids: Vec<i64>) -> Result<BatchDependencyCheckResult, String>`

Проверить зависимости для нескольких файлов.

**Request**:
```rust
file_ids: Vec<i64>
```

**Response**:
```rust
Ok(BatchDependencyCheckResult)
// или
Err(String)
```

**BatchDependencyCheckResult**:
```rust
{
  results: Vec<{
    file_id: i64,
    file_name: String,
    missing_dependencies: Vec<MissingDependency>,
    satisfied_dependencies_count: usize,
    version_conflicts: Vec<VersionConflict>
  }>,
  summary: {
    total_files: usize,
    files_with_missing_deps: usize,
    files_with_conflicts: usize,
    total_missing_deps: usize
  }
}
```

**Errors**:
- `"No files specified"` - не указаны файлы

---

### `batch_export_files(file_ids: Vec<i64>) -> Result<String, String>`

Экспортировать несколько файлов в один JSON.

**Request**:
```rust
file_ids: Vec<i64>
```

**Response**:
```rust
Ok(String) // JSON строка
// или
Err(String)
```

**JSON Format**:
```json
{
  "version": "1.0",
  "type": "files",
  "data": {
    "files": [
      {
        "file": {...},
        "dependencies": [...]
      }
    ]
  }
}
```

**Errors**:
- `"No files specified"` - не указаны файлы
- `"File not found"` - один из файлов не существует

---

### `validate_batch_operation(operation_type: String, file_ids: Vec<i64>) -> Result<BatchValidationResult, String>`

Валидировать пакетную операцию перед выполнением.

**Request**:
```rust
{
  operation_type: "delete" | "move" | "update" | "export",
  file_ids: Vec<i64>
}
```

**Response**:
```rust
Ok(BatchValidationResult)
// или
Err(String)
```

**BatchValidationResult**:
```rust
{
  valid: bool,
  can_proceed: bool,
  warnings: Vec<String>,
  errors: Vec<String>,
  affected_dependencies: Vec<{
    file_id: i64,
    dependent_files: Vec<i64>
  }>
}
```

**Behavior**:
- Проверяет существование всех файлов
- Для операций удаления: проверяет зависимости
- Для операций перемещения: проверяет существование коллекции
- Возвращает предупреждения и ошибки без выполнения операции



































