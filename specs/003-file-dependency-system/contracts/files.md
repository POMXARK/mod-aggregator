# API Contracts: Files (Extended)

**Feature**: 003-file-dependency-system  
**Date**: 2025-12-19  
**Status**: Design Complete

## Tauri Commands: Files (Расширение существующих команд)

### `create_file(params: CreateFileParams) -> Result<File, String>`

Создать файл вручную через форму.

**Request**:
```rust
{
  name: String,
  version: String,
  path: Option<String>,
  metadata: Option<serde_json::Value>,
  dependencies: Option<Vec<{
    target_file_name: String,
    target_file_version: Option<String>,
    dependency_type: "required" | "optional" | "peer"
  }>>
}
```

**Response**:
```rust
Ok(File)
// или
Err(String)
```

**Errors**:
- `"File with name@version already exists"` - файл с таким name@version уже существует
- `"Name cannot be empty"` - имя не может быть пустым
- `"Version cannot be empty"` - версия не может быть пустой
- `"Circular dependency detected"` - обнаружена циклическая зависимость
- `"Dependency not found"` - зависимость не существует (если version указан)

---

### `update_file(params: UpdateFileParams) -> Result<File, String>`

Обновить параметры файла (любое поле редактируемо).

**Request**:
```rust
{
  id: i64,
  name: Option<String>,
  version: Option<String>,
  path: Option<String>,
  metadata: Option<serde_json::Value>
}
```

**Response**:
```rust
Ok(File)
// или
Err(String)
```

**Errors**:
- `"File not found"` - файл не существует
- `"File with name@version already exists"` - новое name@version уже занято
- `"Name cannot be empty"` - имя не может быть пустым
- `"Version cannot be empty"` - версия не может быть пустой

**Note**: При изменении name или version проверяется уникальность новой комбинации.

---

### `delete_file(file_id: i64, force: Option<bool>) -> Result<DeleteFileResult, String>`

Удалить файл с проверкой зависимостей.

**Request**:
```rust
{
  file_id: i64,
  force: Option<bool> // true = удалить даже если есть зависимые файлы
}
```

**Response**:
```rust
Ok(DeleteFileResult)
// или
Err(String)
```

**DeleteFileResult**:
```rust
{
  deleted: bool,
  dependent_files: Option<Vec<File>>, // если force=false и есть зависимые
  message: String
}
```

**Errors**:
- `"File not found"` - файл не существует

**Behavior**:
- Если `force=false` (по умолчанию) и есть файлы, зависящие от данного:
  - Возвращает `DeleteFileResult { deleted: false, dependent_files: [...], message: "..." }`
  - Файл НЕ удаляется
- Если `force=true`:
  - Удаляет файл и все его зависимости
  - Предупреждает о зависимых файлах, но удаляет

---

### `upload_file_version(params: UploadFileVersionParams) -> Result<File, String>`

Загрузить новую версию файла вручную.

**Request**:
```rust
{
  name: String, // имя существующего файла
  version: String, // новая версия
  path: Option<String>,
  metadata: Option<serde_json::Value>
}
```

**Response**:
```rust
Ok(File)
// или
Err(String)
```

**Errors**:
- `"File with name@version already exists"` - версия уже существует
- `"Version cannot be empty"` - версия не может быть пустой

**Note**: Создает новый файл с тем же именем, но другой версией. Зависимости не копируются автоматически.

---

### `get_file_by_name_version(name: String, version: String) -> Result<File, String>`

Получить файл по name@version.

**Request**:
```rust
{
  name: String,
  version: String
}
```

**Response**:
```rust
Ok(File)
// или
Err(String)
```

**Errors**:
- `"File not found"` - файл с таким name@version не существует

---

### `get_file_versions(name: String) -> Result<Vec<File>, String>`

Получить все версии файла с указанным именем.

**Request**:
```rust
name: String
```

**Response**:
```rust
Ok(Vec<File>)
// или
Err(String)
```

**Note**: Возвращает все файлы с указанным именем, отсортированные по версии.



































