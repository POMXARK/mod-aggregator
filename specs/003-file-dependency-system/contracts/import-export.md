# API Contracts: Import/Export

**Feature**: 003-file-dependency-system  
**Date**: 2025-12-19  
**Status**: Design Complete

## Tauri Commands: Import/Export

### `export_file(file_id: i64) -> Result<String, String>`

Экспортировать файл в JSON формат.

**Request**:
```rust
file_id: i64
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
  "type": "file",
  "data": {
    "file": {
      "id": 123,
      "name": "mod-name",
      "version": "1.0.0",
      "path": "/path/to/file",
      "metadata": {}
    },
    "dependencies": [
      {
        "target_file_name": "dependency-name",
        "target_file_version": "2.0.0",
        "dependency_type": "required"
      }
    ]
  }
}
```

**Errors**:
- `"File not found"` - файл не существует

---

### `export_collection(collection_id: i64) -> Result<String, String>`

Экспортировать коллекцию в JSON формат.

**Request**:
```rust
collection_id: i64
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
  "type": "collection",
  "data": {
    "collection": {
      "id": 456,
      "name": "collection-name",
      "description": "Description"
    },
    "files": [
      {
        "file_id": 123,
        "name": "mod-name",
        "version": "1.0.0",
        "order_index": 0,
        "logic_rule": {
          "name": "rule-name",
          "condition_type": "boolean",
          "condition_params": {"value": true},
          "action": "enable"
        }
      }
    ],
    "logic_rules": [
      {
        "id": 789,
        "name": "rule-name",
        "condition_type": "boolean",
        "condition_params": {"value": true},
        "action": "enable"
      }
    ]
  }
}
```

**Errors**:
- `"Collection not found"` - коллекция не существует

---

### `import_file(json_data: String) -> Result<ImportResult, String>`

Импортировать файл из JSON.

**Request**:
```rust
json_data: String // JSON строка в формате export_file
```

**Response**:
```rust
Ok(ImportResult)
// или
Err(String)
```

**ImportResult**:
```rust
{
  file_id: i64,
  created: bool, // true = создан новый, false = обновлен существующий
  missing_dependencies: Vec<MissingDependency>,
  warnings: Vec<String>
}
```

**MissingDependency**:
```rust
{
  target_file_name: String,
  target_file_version: Option<String>,
  available_versions: Vec<String>
}
```

**Errors**:
- `"Invalid JSON format"` - невалидный JSON
- `"Invalid schema version"` - неподдерживаемая версия схемы
- `"Invalid type"` - тип не "file"
- `"Name cannot be empty"` - имя файла пустое
- `"Version cannot be empty"` - версия пустая

**Behavior**:
- Если файл с таким name@version существует: обновляет его
- Если не существует: создает новый
- Зависимости импортируются, но проверяются на наличие
- Если зависимости отсутствуют: возвращает их в `missing_dependencies`

---

### `import_collection(json_data: String) -> Result<ImportResult, String>`

Импортировать коллекцию из JSON.

**Request**:
```rust
json_data: String // JSON строка в формате export_collection
```

**Response**:
```rust
Ok(ImportResult)
// или
Err(String)
```

**ImportResult**:
```rust
{
  collection_id: i64,
  created: bool,
  missing_files: Vec<{
    file_name: String,
    file_version: String
  }>,
  missing_dependencies: Vec<MissingDependency>,
  warnings: Vec<String>
}
```

**Errors**:
- `"Invalid JSON format"` - невалидный JSON
- `"Invalid schema version"` - неподдерживаемая версия схемы
- `"Invalid type"` - тип не "collection"
- `"Collection name already exists"` - коллекция с таким именем уже существует

**Behavior**:
- Создает новую коллекцию
- Импортирует файлы (если они существуют в системе)
- Если файлы отсутствуют: возвращает их в `missing_files`
- Импортирует правила логики
- Сохраняет порядок файлов

---

### `export_build(build_name: String, collection_ids: Vec<i64>, file_ids: Option<Vec<i64>>) -> Result<String, String>`

Экспортировать сборку (комбинацию коллекций и/или файлов).

**Request**:
```rust
{
  build_name: String,
  collection_ids: Vec<i64>,
  file_ids: Option<Vec<i64>> // дополнительные файлы
}
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
  "type": "build",
  "data": {
    "build_name": "my-build",
    "collections": [
      {
        "collection_id": 456,
        "collection_name": "collection-name"
      }
    ],
    "files": [
      {
        "file_id": 123,
        "name": "mod-name",
        "version": "1.0.0"
      }
    ]
  }
}
```

**Errors**:
- `"Collection not found"` - одна из коллекций не существует
- `"File not found"` - один из файлов не существует

---

### `import_build(json_data: String) -> Result<ImportResult, String>`

Импортировать сборку из JSON.

**Request**:
```rust
json_data: String // JSON строка в формате export_build
```

**Response**:
```rust
Ok(ImportResult)
// или
Err(String)
```

**ImportResult**:
```rust
{
  build_id: Option<i64>, // если создана коллекция для сборки
  created: bool,
  missing_collections: Vec<i64>,
  missing_files: Vec<{file_name: String, file_version: String}>,
  missing_dependencies: Vec<MissingDependency>,
  warnings: Vec<String>
}
```

**Behavior**:
- Импортирует все файлы из сборки
- Если указаны коллекции: проверяет их наличие
- Если создается новая коллекция для сборки: возвращает её ID
- Предупреждает о недостающих зависимостях

---

### `validate_import_json(json_data: String) -> Result<ImportValidationResult, String>`

Валидировать JSON перед импортом (без фактического импорта).

**Request**:
```rust
json_data: String
```

**Response**:
```rust
Ok(ImportValidationResult)
// или
Err(String)
```

**ImportValidationResult**:
```rust
{
  valid: bool,
  type: Option<String>, // "file" | "collection" | "build"
  version: Option<String>,
  errors: Vec<String>,
  warnings: Vec<String>,
  preview: Option<ImportPreview>
}
```

**ImportPreview**:
```rust
{
  files_count: usize,
  dependencies_count: usize,
  collections_count: usize,
  missing_dependencies: Vec<MissingDependency>
}
```

**Errors**:
- `"Invalid JSON format"` - невалидный JSON
- `"Missing required fields"` - отсутствуют обязательные поля
























