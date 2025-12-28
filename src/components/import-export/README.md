# Import/Export Components

Компоненты для импорта и экспорта файлов и коллекций в JSON формате.

## Компоненты

- **ImportDialog.svelte** - Диалог импорта файлов/коллекций из JSON
- **ExportDialog.svelte** - Диалог экспорта файлов/коллекций в JSON

## Архитектура

Компоненты используют Svelte 5 runes для реактивности. Коммуникация с backend через Tauri команды из `src-tauri/src/commands/import_export.rs`.

## Утилиты

- `src/lib/import-export/json-serializer.ts` - Утилиты для сериализации/десериализации JSON

## Формат JSON

```json
{
  "version": "1.0",
  "type": "file" | "collection" | "build",
  "data": {
    "file": {...},
    "dependencies": [...]
  }
}
```

## Backend команды

- `export_file()` - экспортировать файл
- `import_file()` - импортировать файл
- `validate_import_json()` - валидировать JSON перед импортом
