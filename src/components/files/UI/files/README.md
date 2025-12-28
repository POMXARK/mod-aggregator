# Files Components

Компоненты для работы с файлами/модами, включая множественный выбор и пакетные операции.

## Компоненты

- **FileForm.svelte** - Форма для добавления и редактирования файлов
- **FileList.svelte** - Список файлов с поддержкой множественного выбора (Ctrl+Click, Shift+Click, drag selection)
- **FileDragDrop.svelte** - Компонент для переупорядочивания файлов через drag & drop
- **BatchOperations.svelte** - Меню пакетных операций (удаление, перемещение, экспорт)

## Архитектура

Все компоненты используют Svelte 5 runes ($state, $derived, $effect) для реактивности. Коммуникация с backend через Tauri команды из `src-tauri/src/commands/files.rs` и `src-tauri/src/commands/batch_operations.rs`.

## Использование

### FileList с множественным выбором

```svelte
<script>
  import FileList from './components/files/FileList.svelte';
  import BatchOperations from './components/files/BatchOperations.svelte';
  
  let selectedFiles = $state<number[]>([]);
</script>

<FileList bind:selectedFiles={selectedFiles} />
<BatchOperations {selectedFiles} />
```

### FileDragDrop для переупорядочивания

```svelte
<script>
  import FileDragDrop from './components/files/FileDragDrop.svelte';
  
  let files = $state<File[]>([]);
  let fileOrder = $state<number[]>([]);
</script>

<FileDragDrop {files} bind:fileOrder={fileOrder} />
```

## Пакетные операции

Компонент `BatchOperations` поддерживает:
- Удаление файлов (с проверкой зависимостей)
- Перемещение в коллекцию
- Экспорт в JSON
- Предупреждения о зависимостях

## Backend команды

### Файлы (`src-tauri/src/commands/files.rs`):
- `create_file()` - создать файл
- `update_file()` - обновить файл
- `delete_file()` - удалить файл
- `get_all_files()` - получить все файлы

### Пакетные операции (`src-tauri/src/commands/batch_operations.rs`):
- `batch_delete_files()` - удалить несколько файлов
- `batch_move_files_to_collection()` - переместить в коллекцию
- `batch_export_files()` - экспортировать файлы
- `batch_check_dependencies()` - проверить зависимости
- `validate_batch_operation()` - валидировать операцию
