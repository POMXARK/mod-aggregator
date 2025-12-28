# Спецификация интеграции: Моды и Файлы

**Feature**: 003-file-dependency-system  
**Date**: 2025-12-19  
**Status**: Draft  
**Purpose**: Объяснение связи между модами и файлами, интеграция UI компонентов, добавление новых функциональных блоков

## Проблема

Пользователь не видит изменений в UI и не понимает:
1. Как связаны моды (старая система) и файлы (новая система)
2. Как интегрировать новые UI компоненты
3. Как добавлять новые функциональные блоки

## Текущее состояние

### Две отдельные системы

#### 1. Старая система: Моды (Mods)
- **Таблица БД**: `mods`
- **Модель**: `Mod` (src-tauri/src/models.rs)
- **Компоненты UI**: `ModsList.svelte`, `ModCard.svelte`
- **Tauri команды**: `get_mods`, `check_updates`
- **Источник данных**: Парсинг с сайтов через ParserBuilder
- **Использование**: Отображение модов, полученных с сайтов

#### 2. Новая система: Файлы (Files)
- **Таблица БД**: `files` (создана миграцией 003_add_dependencies.sql)
- **Модель**: `File` (src-tauri/src/models/file.rs)
- **Компоненты UI**: `FileList.svelte`, `FileForm.svelte`, `BatchOperations.svelte`
- **Tauri команды**: `get_all_files`, `create_file`, `update_file`, `delete_file`
- **Источник данных**: Ручное добавление через UI или импорт
- **Использование**: Управление зависимостями, коллекциями, пакетные операции

### Проблема разделения

**Моды и файлы - это концептуально одна сущность, но реализованы как две разные:**

- **Моды** - это файлы, полученные автоматически через парсинг сайтов
- **Файлы** - это моды, добавленные вручную или импортированные

**Но они не связаны между собой:**
- Нет связи между `mods.id` и `files.id`
- Нет автоматической миграции модов в файлы
- Нет единого интерфейса для работы с обеими сущностями

## Решение: Единая модель данных

### Вариант 1: Моды как источник, Файлы как рабочая копия (Рекомендуется)

**Концепция**: Моды - это "каталог доступных файлов", Файлы - это "рабочий набор файлов"

**Связь**:
- Мод может быть преобразован в Файл (импорт мода в систему файлов)
- Файл может быть создан независимо (ручное добавление)
- Один мод может быть преобразован в несколько файлов (разные версии)

**Реализация**:
```rust
// Добавить в таблицу files поле mod_id (опциональное)
ALTER TABLE files ADD COLUMN mod_id INTEGER REFERENCES mods(id);

// При преобразовании мода в файл:
// 1. Создать File из Mod
// 2. Установить mod_id для связи
// 3. Сохранить метаданные мода в File.metadata
```

**UI Flow**:
1. Пользователь видит список модов (ModsList)
2. Пользователь может "импортировать" мод в систему файлов (кнопка "Добавить в файлы")
3. Мод преобразуется в File, создается связь через `mod_id`
4. Файл появляется в FileList с возможностью управления зависимостями

### Вариант 2: Унифицированная модель (Более сложный)

**Концепция**: Одна таблица для всех файлов/модов

**Реализация**:
- Объединить `mods` и `files` в одну таблицу `files`
- Добавить поле `source_type`: 'parsed' | 'manual' | 'imported'
- Добавить поле `site_id` для связи с сайтом (для парсенных модов)
- Мигрировать все данные из `mods` в `files`

**Преимущества**:
- Единая модель данных
- Упрощенная логика работы с файлами

**Недостатки**:
- Требует миграции данных
- Более сложная схема БД

## Интеграция UI компонентов

### Текущее состояние App.svelte

```svelte
// App.svelte использует только старые компоненты:
<ModsList selectedSiteId={selectedSiteId} />
```

### План интеграции

#### Шаг 1: Добавить новую страницу "Files" в App.svelte

```svelte
<script lang="ts">
  type Page = 'mods' | 'sites' | 'parser' | 'notifications' | 'files' | 'collections';
  let currentPage: Page = $state('mods');
</script>

{#if currentPage === 'files'}
  <FileList />
{/if}
```

#### Шаг 2: Добавить навигацию в Sidebar

```svelte
// В Sidebar.svelte добавить пункты меню:
<button onclick={() => handlePageChange('files')}>
  Файлы
</button>
<button onclick={() => handlePageChange('collections')}>
  Коллекции
</button>
```

#### Шаг 3: Интегрировать компоненты зависимостей

```svelte
// В FileList.svelte или отдельной странице:
<DependencyGraph fileId={selectedFileId} />
<DependencyEditor file={selectedFile} />
```

#### Шаг 4: Добавить пакетные операции

```svelte
// В FileList.svelte:
<BatchOperations 
  selectedFiles={selectedFiles}
  onBatchDelete={handleBatchDelete}
  onBatchMove={handleBatchMove}
/>
```

### Структура страниц

```
App.svelte
├── Mods (старая страница) - список модов с сайтов
│   └── ModsList.svelte
│   └── ModCard.svelte
│   └── [НОВОЕ] Кнопка "Импортировать в файлы"
│
├── Files (новая страница) - управление файлами
│   └── FileList.svelte
│   └── FileForm.svelte (для создания/редактирования)
│   └── BatchOperations.svelte
│   └── DependencyGraph.svelte
│   └── DependencyEditor.svelte
│
├── Collections (новая страница) - управление коллекциями
│   └── CollectionViewer.svelte
│   └── CollectionComposer.svelte
│   └── CollectionLogicBuilder.svelte
│
├── Import/Export (новая страница или модальное окно)
│   └── ImportDialog.svelte
│   └── ExportDialog.svelte
│
└── Sites, Parser, Notifications (существующие страницы)
```

## Добавление новых функциональных блоков

### Архитектура расширяемости

Система спроектирована для добавления новых функциональных блоков через:

1. **Новые Tauri команды** (Backend)
2. **Новые Svelte компоненты** (Frontend)
3. **Новые типы данных** (Models)
4. **Новые страницы/разделы** (Navigation)

### Шаблон добавления нового функционального блока

#### Шаг 1: Определить требования

Создать файл спецификации:
```
specs/004-new-feature/spec.md
```

#### Шаг 2: Создать модели данных (Backend)

```rust
// src-tauri/src/models/new_feature.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewFeature {
    pub id: i64,
    pub name: String,
    // ... другие поля
}
```

#### Шаг 3: Создать миграцию БД

```sql
-- src-tauri/src/database/migrations/004_add_new_feature.sql
CREATE TABLE IF NOT EXISTS new_feature (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    -- ... другие поля
);
```

#### Шаг 4: Создать Tauri команды

```rust
// src-tauri/src/commands/new_feature.rs
#[tauri::command]
pub async fn create_new_feature(params: CreateNewFeatureParams) -> Result<NewFeature, String> {
    // Реализация
}
```

#### Шаг 5: Зарегистрировать команды в main.rs

```rust
// src-tauri/src/main.rs
mod commands {
    pub mod new_feature;
}

// В функции setup():
.invoke_handler(tauri::generate_handler![
    // ... существующие команды
    commands::new_feature::create_new_feature,
])
```

#### Шаг 6: Создать TypeScript типы

```typescript
// src/types/new-feature.ts
export interface NewFeature {
  id: number;
  name: string;
  // ... другие поля
}
```

#### Шаг 7: Создать Svelte компоненты

```svelte
<!-- src/components/new-feature/NewFeatureList.svelte -->
<script lang="ts">
  import { invoke } from '../../lib/tauri-wrapper.js';
  import type { NewFeature } from '../../types/new-feature.js';
  
  let features = $state<NewFeature[]>([]);
  
  async function loadFeatures() {
    features = await invoke<NewFeature[]>('get_all_new_features');
  }
</script>
```

#### Шаг 8: Интегрировать в App.svelte

```svelte
// src/App.svelte
type Page = 'mods' | 'sites' | 'parser' | 'notifications' | 'files' | 'collections' | 'new-feature';

{#if currentPage === 'new-feature'}
  <NewFeatureList />
{/if}
```

#### Шаг 9: Добавить навигацию

```svelte
// src/components/Sidebar.svelte
<button onclick={() => handlePageChange('new-feature')}>
  Новый функционал
</button>
```

### Пример: Добавление функционального блока "Теги"

#### 1. Спецификация

```markdown
# Feature: Теги для файлов

**Цель**: Добавить возможность помечать файлы тегами для организации

**Требования**:
- Файл может иметь множество тегов
- Тег может быть применен к множеству файлов
- Фильтрация файлов по тегам
- Создание/удаление тегов
```

#### 2. Модель данных

```rust
// src-tauri/src/models/tag.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub color: Option<String>,
    pub created_at: DateTime<Utc>,
}

// Связь многие-ко-многим через промежуточную таблицу
// file_tags (file_id, tag_id)
```

#### 3. Миграция БД

```sql
-- src-tauri/src/database/migrations/004_add_tags.sql
CREATE TABLE IF NOT EXISTS tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    color TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS file_tags (
    file_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (file_id, tag_id),
    FOREIGN KEY (file_id) REFERENCES files(id),
    FOREIGN KEY (tag_id) REFERENCES tags(id)
);
```

#### 4. Tauri команды

```rust
// src-tauri/src/commands/tags.rs
#[tauri::command]
pub async fn create_tag(name: String, color: Option<String>) -> Result<Tag, String> {
    // Реализация
}

#[tauri::command]
pub async fn add_tag_to_file(file_id: i64, tag_id: i64) -> Result<(), String> {
    // Реализация
}

#[tauri::command]
pub async fn get_files_by_tag(tag_id: i64) -> Result<Vec<File>, String> {
    // Реализация
}
```

#### 5. UI компоненты

```svelte
<!-- src/components/tags/TagList.svelte -->
<!-- src/components/tags/TagEditor.svelte -->
<!-- src/components/tags/TagFilter.svelte -->
```

#### 6. Интеграция в FileList

```svelte
<!-- src/components/files/FileList.svelte -->
<TagFilter onTagSelect={handleTagFilter} />
<!-- В карточке файла -->
<TagList file={file} />
```

## План миграции/интеграции

### Фаза 1: Интеграция UI (Немедленно)

1. ✅ Добавить страницу "Files" в App.svelte
2. ✅ Добавить навигацию в Sidebar
3. ✅ Интегрировать FileList, BatchOperations
4. ✅ Добавить страницу "Collections"
5. ✅ Интегрировать CollectionViewer, CollectionComposer

### Фаза 2: Связь модов и файлов (Краткосрочно)

1. Добавить поле `mod_id` в таблицу `files`
2. Создать Tauri команду `import_mod_as_file(mod_id: i64)`
3. Добавить кнопку "Импортировать в файлы" в ModCard
4. Реализовать автоматическую миграцию метаданных мода в файл

### Фаза 3: Унификация интерфейса (Среднесрочно)

1. Создать единый компонент `FileModCard` для отображения и модов, и файлов
2. Добавить фильтр "Показать только файлы" / "Показать только моды"
3. Реализовать поиск по обеим сущностям

### Фаза 4: Полная миграция (Долгосрочно, опционально)

1. Мигрировать все данные из `mods` в `files`
2. Удалить таблицу `mods` (или оставить для истории)
3. Обновить все команды для работы с `files`

## Чеклист интеграции

### Backend
- [ ] Добавить поле `mod_id` в таблицу `files` (миграция)
- [ ] Создать команду `import_mod_as_file`
- [ ] Обновить команду `get_all_files` для фильтрации по типу источника
- [ ] Добавить индексы для производительности

### Frontend
- [ ] Добавить страницу "Files" в App.svelte
- [ ] Добавить страницу "Collections" в App.svelte
- [ ] Обновить Sidebar с новыми пунктами меню
- [ ] Интегрировать FileList в App.svelte
- [ ] Интегрировать BatchOperations в FileList
- [ ] Добавить кнопку "Импортировать в файлы" в ModCard
- [ ] Создать модальное окно для импорта мода в файлы

### Тестирование
- [ ] E2E тест: Импорт мода в файлы
- [ ] E2E тест: Создание файла вручную
- [ ] E2E тест: Управление зависимостями
- [ ] E2E тест: Пакетные операции
- [ ] E2E тест: Работа с коллекциями

## Документация для разработчиков

### Как добавить новый функциональный блок

1. **Создать спецификацию** в `specs/XXX-feature-name/spec.md`
2. **Создать модели данных** в `src-tauri/src/models/`
3. **Создать миграцию БД** в `src-tauri/src/database/migrations/`
4. **Создать Tauri команды** в `src-tauri/src/commands/`
5. **Зарегистрировать команды** в `src-tauri/src/main.rs`
6. **Создать TypeScript типы** в `src/types/`
7. **Создать Svelte компоненты** в `src/components/`
8. **Интегрировать в App.svelte** и Sidebar
9. **Написать тесты** (unit + E2E)

### Структура проекта

```
src-tauri/src/
├── models/          # Модели данных (Rust)
├── commands/       # Tauri команды (Rust)
├── services/        # Бизнес-логика (Rust)
├── database/       # Работа с БД (Rust)
│   └── migrations/ # Миграции SQL

src/
├── components/     # Svelte компоненты
│   ├── files/      # Компоненты для файлов
│   ├── collections/# Компоненты для коллекций
│   └── ...         # Другие компоненты
├── types/          # TypeScript типы
├── lib/            # Утилиты и helpers
└── App.svelte      # Главный компонент
```

## Заключение

**Текущая ситуация**: Моды и файлы - это концептуально одна сущность, но реализованы как две разные системы без связи.

**Решение**: 
1. Интегрировать новые UI компоненты в App.svelte
2. Добавить связь между модами и файлами через `mod_id`
3. Предоставить возможность импорта модов в файлы
4. Создать единый интерфейс для работы с обеими сущностями

**Расширяемость**: Система спроектирована для добавления новых функциональных блоков через стандартный процесс (модели → команды → компоненты → интеграция).
























