# План интеграции UI компонентов

**Дата**: 2025-12-19  
**Статус**: Готов к реализации

## Проблема

Новые UI компоненты созданы, но не интегрированы в приложение:
- ✅ `FileList.svelte` - создан, но не используется
- ✅ `BatchOperations.svelte` - создан, но не используется  
- ✅ `CollectionViewer.svelte` - создан, но не используется
- ✅ `DependencyGraph.svelte` - создан, но не используется

**App.svelte** все еще использует только старые компоненты (`ModsList.svelte`).

## Быстрое решение (5 минут)

### 1. Добавить новые страницы в App.svelte

```svelte
// В App.svelte, изменить тип Page:
type Page = 'mods' | 'sites' | 'parser' | 'notifications' | 'files' | 'collections';

// Добавить импорты:
import FileList from './components/files/FileList.svelte';
import CollectionViewer from './components/collections/CollectionViewer.svelte';

// Добавить рендеринг новых страниц:
{#if currentPage === 'files'}
  <FileList />
{/if}

{#if currentPage === 'collections'}
  <CollectionViewer />
{/if}
```

### 2. Обновить Sidebar.svelte

```svelte
// Добавить кнопки навигации:
<button onclick={() => handlePageChange('files')}>
  📁 Файлы
</button>
<button onclick={() => handlePageChange('collections')}>
  📦 Коллекции
</button>
```

### 3. Проверить работу

Запустить `npm run tauri:dev` и проверить:
- Открывается страница "Файлы"
- Открывается страница "Коллекции"
- Компоненты загружают данные

## Полная интеграция (30 минут)

См. подробную спецификацию: `integration-spec.md`

## Связь модов и файлов

**Важно**: Моды и файлы - это одна и та же сущность, но реализованы как две разные системы.

**Решение**: 
1. Добавить поле `mod_id` в таблицу `files` для связи
2. Создать команду `import_mod_as_file` для преобразования мода в файл
3. Добавить кнопку "Импортировать в файлы" в `ModCard.svelte`

Подробности в `integration-spec.md`, раздел "Связь модов и файлов".
























