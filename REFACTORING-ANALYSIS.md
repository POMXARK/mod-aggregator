# 📊 Анализ кандидатов на рефакторинг

**Дата генерации:** 03.01.2026

## 📋 СРЕДНИЙ ПРИОРИТЕТ (500-1000 строк)

Рассмотреть рефакторинг при следующей модификации:

1. **tauri-plugin-mcp\guest-js\index.ts** - 986 строк
2. **src\lib\tauri-mock.ts** - 948 строк
3. **src\components\Parser\UI\constructor\GeneratedCodePanel.svelte** - 942 строк
4. **src\components\AIChat.svelte** - 936 строк
5. **src\components\Sidebar.svelte** - 853 строк
6. **src-tauri\src\ai_parser.rs** - 794 строк
7. **src-tauri\src\database\modules\collections.rs** - 768 строк
8. **src-tauri\src\commands\batch_operations.rs** - 762 строк
9. **src\components\collections\CollectionLogicBuilder.svelte** - 761 строк
10. **src\components\collections\UI\collections\CollectionLogicBuilder.svelte** - 761 строк
11. **src\lib\utils\selection-script.ts** - 724 строк
12. **src\components\files\FileForm.svelte** - 657 строк
13. **src\components\files\UI\files\FileForm.svelte** - 657 строк
14. **src-tauri\src\handlers\resources.rs** - 648 строк
15. **src\components\files\BatchOperations.svelte** - 632 строк

**Рекомендации:**
- Мониторить рост файла
- Рассмотреть извлечение утилитарных функций
- Проверить single responsibility principle

## 📈 Статистика по языкам

### Svelte
- **Файлов:** 134
- **Всего строк:** 26,242
- **Средний размер:** 196 строк
- **Крупных файлов (>500 строк):** 15

**Крупные файлы:**
- src\components\Parser\UI\constructor\GeneratedCodePanel.svelte (942 строк)
- src\components\AIChat.svelte (936 строк)
- src\components\Sidebar.svelte (853 строк)
- src\components\collections\CollectionLogicBuilder.svelte (761 строк)
- src\components\collections\UI\collections\CollectionLogicBuilder.svelte (761 строк)

### Rust
- **Файлов:** 85
- **Всего строк:** 18,442
- **Средний размер:** 217 строк
- **Крупных файлов (>500 строк):** 11

**Крупные файлы:**
- src-tauri\src\ai_parser.rs (794 строк)
- src-tauri\src\database\modules\collections.rs (768 строк)
- src-tauri\src\commands\batch_operations.rs (762 строк)
- src-tauri\src\handlers\resources.rs (648 строк)
- src-tauri\src\commands\import_export.rs (618 строк)

### TypeScript
- **Файлов:** 110
- **Всего строк:** 16,976
- **Средний размер:** 154 строк
- **Крупных файлов (>500 строк):** 6

**Крупные файлы:**
- tauri-plugin-mcp\guest-js\index.ts (986 строк)
- src\lib\tauri-mock.ts (948 строк)
- src\lib\utils\selection-script.ts (724 строк)
- src\components\Parser\UI\ParserBuilder\composables\useUIState.ts (560 строк)
- src\lib\collections\__tests__\collection-logic.property.test.ts (532 строк)

### JavaScript
- **Файлов:** 40
- **Всего строк:** 3,885
- **Средний размер:** 97 строк
- **Крупных файлов (>500 строк):** 0

### TSX
- **Файлов:** 1
- **Всего строк:** 112
- **Средний размер:** 112 строк
- **Крупных файлов (>500 строк):** 0

