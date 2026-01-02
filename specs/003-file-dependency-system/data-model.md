# Data Model: File Dependency Management and Advanced Collections

**Feature**: 003-file-dependency-system  
**Date**: 2025-12-19  
**Status**: Design Complete

## Overview

Модель данных расширяет существующую схему БД для поддержки зависимостей файлов, продвинутых коллекций с логикой, и сохранения состояния сессии.

## Core Entities

### 1. File (Расширение существующей модели)

**Идентификация**: `name@version` (комбинация имени и версии)

**Атрибуты**:
- `id`: INTEGER PRIMARY KEY (существующее)
- `name`: TEXT NOT NULL (существующее, теперь часть идентификатора)
- `version`: TEXT NOT NULL (новое, обязательное поле)
- `path`: TEXT (существующее)
- `metadata`: JSON (существующее, расширяется)
- `created_at`: TIMESTAMP (существующее)
- `updated_at`: TIMESTAMP (существующее)

**Ограничения**:
- Уникальность: `(name, version)` - комбинация должна быть уникальной
- `version` не может быть пустым (NOT NULL)
- Множественные версии одного `name` могут сосуществовать

**Состояния**:
- `installed`: файл установлен в системе
- `available`: файл доступен, но не установлен
- `missing_dependencies`: файл имеет отсутствующие зависимости

**Связи**:
- Один ко многим: File → FileDependency (файл может иметь множество зависимостей)
- Многие ко многим: File ↔ Collection (через промежуточную таблицу)

### 2. FileDependency

**Описание**: Представляет зависимость одного файла от другого.

**Атрибуты**:
- `id`: INTEGER PRIMARY KEY
- `source_file_id`: INTEGER NOT NULL (FK → files.id)
- `target_file_name`: TEXT NOT NULL (имя зависимости)
- `target_file_version`: TEXT (версия зависимости, NULL = любая версия)
- `dependency_type`: TEXT NOT NULL (required | optional | peer)
- `created_at`: TIMESTAMP

**Ограничения**:
- `source_file_id` должен существовать в таблице `files`
- `dependency_type` должен быть одним из: 'required', 'optional', 'peer'
- Если `target_file_version` указан, должна существовать запись `files` с таким `name` и `version`

**Валидация**:
- Не допускаются циклические зависимости (проверка через граф)
- `source_file_id` не может быть равен `target_file_id` (самозависимость)

**Состояния**:
- `satisfied`: зависимость разрешена (существует файл с нужным name@version)
- `missing`: зависимость отсутствует
- `version_conflict`: существует файл с нужным name, но другой версией

### 3. DependencyGraph

**Описание**: Виртуальная структура (не таблица БД), представляющая граф зависимостей.

**Структура**:
- **Узлы**: File (идентифицируются как `name@version`)
- **Рёбра**: FileDependency (направленные от source к target)

**Алгоритмы**:
- **Обнаружение циклов**: DFS с отслеживанием посещенных узлов
- **Топологическая сортировка**: Kahn's algorithm для определения порядка установки
- **Поиск зависимых файлов**: BFS от заданного файла для поиска всех зависимых

**Операции**:
- `get_dependents(file_id)`: получить все файлы, зависящие от данного
- `get_dependencies(file_id)`: получить все зависимости файла
- `check_circular(file_id, target_id)`: проверить наличие циклической зависимости
- `resolve_installation_order(files)`: определить порядок установки файлов

### 4. Collection

**Описание**: Коллекция файлов с опциональной логикой включения/выключения.

**Атрибуты**:
- `id`: INTEGER PRIMARY KEY
- `name`: TEXT NOT NULL UNIQUE
- `description`: TEXT
- `created_at`: TIMESTAMP
- `updated_at`: TIMESTAMP

**Связи**:
- Многие ко многим: Collection ↔ File (через `collection_files`)
- Один ко многим: Collection → CollectionLogicRule

**Состояния**:
- `active`: коллекция активна (файлы применяются)
- `inactive`: коллекция неактивна

### 5. CollectionFile

**Описание**: Промежуточная таблица для связи коллекций и файлов с логикой.

**Атрибуты**:
- `id`: INTEGER PRIMARY KEY
- `collection_id`: INTEGER NOT NULL (FK → collections.id)
- `file_id`: INTEGER NOT NULL (FK → files.id)
- `logic_rule_id`: INTEGER (FK → collection_logic_rules.id, NULL = всегда включен)
- `order`: INTEGER (порядок файла в коллекции)
- `created_at`: TIMESTAMP

**Ограничения**:
- Уникальность: `(collection_id, file_id)` - файл может быть в коллекции только один раз
- `order` используется для сортировки файлов в коллекции

### 6. CollectionLogicRule

**Описание**: Правило логики для включения/выключения файла в коллекции.

**Атрибуты**:
- `id`: INTEGER PRIMARY KEY
- `collection_id`: INTEGER NOT NULL (FK → collections.id)
- `name`: TEXT NOT NULL (название правила для UI)
- `condition_type`: TEXT NOT NULL (boolean | collection_check | file_check | and | or)
- `condition_params`: JSON NOT NULL (параметры условия)
- `action`: TEXT NOT NULL (enable | disable)
- `created_at`: TIMESTAMP

**Типы условий**:
- `boolean`: простое включено/выключено (параметры: `{"value": true/false}`)
- `collection_check`: проверка активности другой коллекции (параметры: `{"collection_id": 123}`)
- `file_check`: проверка установки файла (параметры: `{"file_name": "mod", "file_version": "1.0"}`)
- `and`: логическое И (параметры: `{"rules": [rule_id1, rule_id2]}`)
- `or`: логическое ИЛИ (параметры: `{"rules": [rule_id1, rule_id2]}`)

**Валидация**:
- `condition_params` должен быть валидным JSON
- Для `and`/`or`: `rules` должен содержать существующие `rule_id`
- Для `collection_check`: `collection_id` должен существовать
- Для `file_check`: `file_name` и `file_version` должны существовать

**Вычисление**:
- Рекурсивное вычисление условий для определения состояния файла
- Кэширование результатов для производительности

### 7. SessionState

**Описание**: Сохраненное состояние приложения для восстановления сессии.

**Атрибуты**:
- `id`: INTEGER PRIMARY KEY
- `file_order`: JSON NOT NULL (массив `file_id` в порядке отображения)
- `ui_preferences`: JSON NOT NULL (объект с настройками UI)
- `open_collections`: JSON (массив `collection_id` открытых коллекций)
- `selected_files`: JSON (массив `file_id` выбранных файлов)
- `recent_actions`: JSON (массив последних действий, максимум 50)
- `last_updated`: TIMESTAMP

**Структура JSON полей**:

`file_order`:
```json
[1, 5, 3, 7, 2]
```

`ui_preferences`:
```json
{
  "view_mode": "list" | "tiles",
  "window_width": 1200,
  "window_height": 800,
  "sidebar_collapsed": false,
  "theme": "dark"
}
```

`open_collections`:
```json
[1, 3, 5]
```

`selected_files`:
```json
[2, 4, 6]
```

`recent_actions`:
```json
[
  {
    "type": "file_added" | "file_deleted" | "collection_created" | "dependency_added",
    "target_id": 123,
    "target_name": "mod-name@1.0",
    "timestamp": "2025-12-19T10:30:00Z"
  }
]
```

**Ограничения**:
- Только одна запись в таблице (singleton pattern)
- `recent_actions` ограничен 50 записями (при добавлении удаляется самая старая)
- Все JSON поля должны быть валидными

## Database Schema

### Миграции

**003_add_dependencies.sql**:
```sql
-- Добавление поля version в таблицу files (если еще не добавлено)
ALTER TABLE files ADD COLUMN version TEXT NOT NULL DEFAULT '1.0.0';

-- Создание уникального индекса для name@version
CREATE UNIQUE INDEX idx_files_name_version ON files(name, version);

-- Таблица зависимостей файлов
CREATE TABLE file_dependencies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    source_file_id INTEGER NOT NULL,
    target_file_name TEXT NOT NULL,
    target_file_version TEXT,
    dependency_type TEXT NOT NULL CHECK(dependency_type IN ('required', 'optional', 'peer')),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (source_file_id) REFERENCES files(id) ON DELETE CASCADE
);

CREATE INDEX idx_file_dependencies_source ON file_dependencies(source_file_id);
CREATE INDEX idx_file_dependencies_target ON file_dependencies(target_file_name, target_file_version);
```

**004_add_collections.sql**:
```sql
-- Таблица коллекций
CREATE TABLE collections (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Промежуточная таблица коллекций и файлов
CREATE TABLE collection_files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    collection_id INTEGER NOT NULL,
    file_id INTEGER NOT NULL,
    logic_rule_id INTEGER,
    order_index INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (collection_id) REFERENCES collections(id) ON DELETE CASCADE,
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE,
    FOREIGN KEY (logic_rule_id) REFERENCES collection_logic_rules(id) ON DELETE SET NULL,
    UNIQUE(collection_id, file_id)
);

-- Таблица правил логики коллекций
CREATE TABLE collection_logic_rules (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    collection_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    condition_type TEXT NOT NULL CHECK(condition_type IN ('boolean', 'collection_check', 'file_check', 'and', 'or')),
    condition_params JSON NOT NULL,
    action TEXT NOT NULL CHECK(action IN ('enable', 'disable')),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (collection_id) REFERENCES collections(id) ON DELETE CASCADE
);

CREATE INDEX idx_collection_files_collection ON collection_files(collection_id);
CREATE INDEX idx_collection_files_file ON collection_files(file_id);
CREATE INDEX idx_collection_logic_rules_collection ON collection_logic_rules(collection_id);
```

**005_add_session_state.sql**:
```sql
-- Таблица состояния сессии
CREATE TABLE session_state (
    id INTEGER PRIMARY KEY CHECK(id = 1),
    file_order JSON NOT NULL DEFAULT '[]',
    ui_preferences JSON NOT NULL DEFAULT '{}',
    open_collections JSON DEFAULT '[]',
    selected_files JSON DEFAULT '[]',
    recent_actions JSON DEFAULT '[]',
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Вставка начальной записи
INSERT INTO session_state (id) VALUES (1);
```

## TypeScript Types (Frontend)

```typescript
// File identification
type FileId = string; // format: "name@version"

// File model
interface File {
  id: number;
  name: string;
  version: string;
  path?: string;
  metadata: Record<string, unknown>;
  createdAt: string;
  updatedAt: string;
}

// Dependency model
interface FileDependency {
  id: number;
  sourceFileId: number;
  targetFileName: string;
  targetFileVersion?: string;
  dependencyType: 'required' | 'optional' | 'peer';
  createdAt: string;
}

// Collection model
interface Collection {
  id: number;
  name: string;
  description?: string;
  createdAt: string;
  updatedAt: string;
}

// Collection logic rule
interface CollectionLogicRule {
  id: number;
  collectionId: number;
  name: string;
  conditionType: 'boolean' | 'collection_check' | 'file_check' | 'and' | 'or';
  conditionParams: Record<string, unknown>;
  action: 'enable' | 'disable';
  createdAt: string;
}

// Session state
interface SessionState {
  fileOrder: number[];
  uiPreferences: {
    viewMode: 'list' | 'tiles';
    windowWidth?: number;
    windowHeight?: number;
    sidebarCollapsed?: boolean;
    theme?: 'dark' | 'light';
  };
  openCollections: number[];
  selectedFiles: number[];
  recentActions: Array<{
    type: 'file_added' | 'file_deleted' | 'collection_created' | 'dependency_added';
    targetId: number;
    targetName: string;
    timestamp: string;
  }>;
}
```

## Rust Types (Backend)

```rust
// File identification
pub type FileId = String; // format: "name@version"

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub id: i64,
    pub name: String,
    pub version: String,
    pub path: Option<String>,
    pub metadata: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDependency {
    pub id: i64,
    pub source_file_id: i64,
    pub target_file_name: String,
    pub target_file_version: Option<String>,
    pub dependency_type: DependencyType,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyType {
    Required,
    Optional,
    Peer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionLogicRule {
    pub id: i64,
    pub collection_id: i64,
    pub name: String,
    pub condition_type: ConditionType,
    pub condition_params: serde_json::Value,
    pub action: Action,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionType {
    Boolean,
    CollectionCheck,
    FileCheck,
    And,
    Or,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    Enable,
    Disable,
}
```

## Validation Rules

### File
- `name` не может быть пустым
- `version` не может быть пустым
- Комбинация `(name, version)` должна быть уникальной

### FileDependency
- `source_file_id` должен существовать
- `target_file_name` не может быть пустым
- Если `target_file_version` указан, должен существовать файл с таким `name` и `version`
- Не допускаются циклические зависимости
- Не допускается самозависимость (`source_file_id` != `target_file_id`)

### Collection
- `name` должен быть уникальным
- `name` не может быть пустым

### CollectionLogicRule
- `condition_params` должен быть валидным JSON
- Для `and`/`or`: все `rule_id` в `condition_params.rules` должны существовать
- Для `collection_check`: `collection_id` должен существовать
- Для `file_check`: `file_name` и `file_version` должны существовать

### SessionState
- Только одна запись (id = 1)
- Все JSON поля должны быть валидными
- `recent_actions` ограничен 50 записями

## State Transitions

### File States
```
available → installed (при установке)
installed → available (при удалении)
installed → missing_dependencies (при удалении зависимости)
missing_dependencies → installed (при установке зависимостей)
```

### Dependency States
```
satisfied → missing (при удалении зависимости)
missing → satisfied (при установке зависимости)
satisfied → version_conflict (при изменении версии)
```

## Relationships Summary

```
File (1) ──< (many) FileDependency (source)
File (1) ──< (many) FileDependency (target, по name@version)
File (many) ──< (many) Collection (через CollectionFile)
Collection (1) ──< (many) CollectionFile
Collection (1) ──< (many) CollectionLogicRule
CollectionFile (many) ──> (1) CollectionLogicRule (опционально)
SessionState (1) - singleton
```



































