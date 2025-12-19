# API Contracts: Collections

**Feature**: 003-file-dependency-system  
**Date**: 2025-12-19  
**Status**: Design Complete

## Tauri Commands: Collections

### `get_collections() -> Result<Vec<Collection>, String>`

Получить все коллекции.

**Response**:
```rust
Ok(Vec<Collection>)
// или
Err(String)
```

**Collection**:
```rust
{
  id: i64,
  name: String,
  description: Option<String>,
  created_at: String,
  updated_at: String
}
```

---

### `create_collection(params: CreateCollectionParams) -> Result<Collection, String>`

Создать новую коллекцию.

**Request**:
```rust
{
  name: String,
  description: Option<String>
}
```

**Response**:
```rust
Ok(Collection)
// или
Err(String)
```

**Errors**:
- `"Collection name already exists"` - коллекция с таким именем уже существует
- `"Name cannot be empty"` - имя не может быть пустым

---

### `update_collection(params: UpdateCollectionParams) -> Result<Collection, String>`

Обновить коллекцию.

**Request**:
```rust
{
  id: i64,
  name: Option<String>,
  description: Option<String>
}
```

**Response**:
```rust
Ok(Collection)
// или
Err(String)
```

**Errors**:
- `"Collection not found"` - коллекция не существует
- `"Collection name already exists"` - новое имя уже занято

---

### `delete_collection(collection_id: i64) -> Result<(), String>`

Удалить коллекцию.

**Request**:
```rust
collection_id: i64
```

**Response**:
```rust
Ok(())
// или
Err(String)
```

**Errors**:
- `"Collection not found"` - коллекция не существует

**Note**: Каскадное удаление связанных записей (collection_files, collection_logic_rules).

---

### `get_collection_files(collection_id: i64) -> Result<Vec<CollectionFile>, String>`

Получить файлы коллекции с их логикой.

**Request**:
```rust
collection_id: i64
```

**Response**:
```rust
Ok(Vec<CollectionFile>)
// или
Err(String)
```

**CollectionFile**:
```rust
{
  id: i64,
  collection_id: i64,
  file_id: i64,
  file: File, // полная информация о файле
  logic_rule_id: Option<i64>,
  logic_rule: Option<CollectionLogicRule>, // если есть
  order_index: i64,
  created_at: String
}
```

**Errors**:
- `"Collection not found"` - коллекция не существует

---

### `add_file_to_collection(params: AddFileToCollectionParams) -> Result<CollectionFile, String>`

Добавить файл в коллекцию.

**Request**:
```rust
{
  collection_id: i64,
  file_id: i64,
  logic_rule_id: Option<i64>, // опциональное правило логики
  order_index: Option<i64> // опциональный порядок
}
```

**Response**:
```rust
Ok(CollectionFile)
// или
Err(String)
```

**Errors**:
- `"Collection not found"` - коллекция не существует
- `"File not found"` - файл не существует
- `"File already in collection"` - файл уже в коллекции
- `"Logic rule not found"` - правило логики не существует (если указано)

---

### `remove_file_from_collection(collection_id: i64, file_id: i64) -> Result<(), String>`

Удалить файл из коллекции.

**Request**:
```rust
{
  collection_id: i64,
  file_id: i64
}
```

**Response**:
```rust
Ok(())
// или
Err(String)
```

**Errors**:
- `"Collection not found"` - коллекция не существует
- `"File not in collection"` - файл не в коллекции

---

### `reorder_collection_files(collection_id: i64, file_orders: Vec<FileOrder>) -> Result<(), String>`

Изменить порядок файлов в коллекции.

**Request**:
```rust
{
  collection_id: i64,
  file_orders: Vec<{
    file_id: i64,
    order_index: i64
  }>
}
```

**Response**:
```rust
Ok(())
// или
Err(String)
```

**Errors**:
- `"Collection not found"` - коллекция не существует
- `"File not in collection"` - один из файлов не в коллекции

---

### `get_collection_logic_rules(collection_id: i64) -> Result<Vec<CollectionLogicRule>, String>`

Получить правила логики коллекции.

**Request**:
```rust
collection_id: i64
```

**Response**:
```rust
Ok(Vec<CollectionLogicRule>)
// или
Err(String)
```

**CollectionLogicRule**:
```rust
{
  id: i64,
  collection_id: i64,
  name: String,
  condition_type: "boolean" | "collection_check" | "file_check" | "and" | "or",
  condition_params: serde_json::Value,
  action: "enable" | "disable",
  created_at: String
}
```

**Errors**:
- `"Collection not found"` - коллекция не существует

---

### `create_collection_logic_rule(params: CreateLogicRuleParams) -> Result<CollectionLogicRule, String>`

Создать правило логики для коллекции.

**Request**:
```rust
{
  collection_id: i64,
  name: String,
  condition_type: "boolean" | "collection_check" | "file_check" | "and" | "or",
  condition_params: serde_json::Value,
  action: "enable" | "disable"
}
```

**Response**:
```rust
Ok(CollectionLogicRule)
// или
Err(String)
```

**Errors**:
- `"Collection not found"` - коллекция не существует
- `"Invalid condition params"` - невалидные параметры условия
- `"Referenced rule not found"` - для and/or: правило не существует
- `"Referenced collection not found"` - для collection_check: коллекция не существует
- `"Referenced file not found"` - для file_check: файл не существует

**Validation**:
- Проверка структуры `condition_params` в зависимости от `condition_type`
- Валидация ссылок на другие сущности

---

### `update_collection_logic_rule(params: UpdateLogicRuleParams) -> Result<CollectionLogicRule, String>`

Обновить правило логики.

**Request**:
```rust
{
  id: i64,
  name: Option<String>,
  condition_type: Option<String>,
  condition_params: Option<serde_json::Value>,
  action: Option<String>
}
```

**Response**:
```rust
Ok(CollectionLogicRule)
// или
Err(String)
```

**Errors**:
- `"Rule not found"` - правило не существует
- `"Invalid condition params"` - невалидные параметры

---

### `delete_collection_logic_rule(rule_id: i64) -> Result<(), String>`

Удалить правило логики.

**Request**:
```rust
rule_id: i64
```

**Response**:
```rust
Ok(())
// или
Err(String)
```

**Errors**:
- `"Rule not found"` - правило не существует

**Note**: Удаление правила не удаляет файлы из коллекции, только отвязывает логику.

---

### `evaluate_collection_logic(collection_id: i64) -> Result<CollectionEvaluationResult, String>`

Вычислить состояние коллекции на основе правил логики.

**Request**:
```rust
collection_id: i64
```

**Response**:
```rust
Ok(CollectionEvaluationResult)
// или
Err(String)
```

**CollectionEvaluationResult**:
```rust
{
  collection_id: i64,
  enabled_files: Vec<i64>, // file_ids
  disabled_files: Vec<i64>, // file_ids
  evaluation_details: Vec<FileEvaluationDetail>
}
```

**FileEvaluationDetail**:
```rust
{
  file_id: i64,
  enabled: bool,
  applied_rules: Vec<i64> // rule_ids, которые повлияли на решение
}
```

**Errors**:
- `"Collection not found"` - коллекция не существует

---

### `combine_collections(params: CombineCollectionsParams) -> Result<Collection, String>`

Объединить несколько коллекций в новую.

**Request**:
```rust
{
  name: String,
  description: Option<String>,
  source_collection_ids: Vec<i64>,
  selected_file_ids: Option<Vec<i64>> // если None, берутся все файлы
}
```

**Response**:
```rust
Ok(Collection)
// или
Err(String)
```

**Errors**:
- `"Collection name already exists"` - имя уже занято
- `"Source collection not found"` - одна из исходных коллекций не существует
- `"File not found"` - один из выбранных файлов не существует
- `"No files selected"` - не выбрано ни одного файла

**Note**: Создает новую коллекцию с файлами из исходных коллекций. Логика исходных коллекций не копируется.

---

### `get_files_from_multiple_collections(collection_ids: Vec<i64>) -> Result<Vec<CollectionFileView>, String>`

Получить файлы из нескольких коллекций с указанием принадлежности.

**Request**:
```rust
collection_ids: Vec<i64>
```

**Response**:
```rust
Ok(Vec<CollectionFileView>)
// или
Err(String)
```

**CollectionFileView**:
```rust
{
  file_id: i64,
  file: File,
  collections: Vec<{
    collection_id: i64,
    collection_name: String,
    order_index: i64,
    logic_rule_id: Option<i64>
  }>
}
```

**Errors**:
- `"Collection not found"` - одна из коллекций не существует

