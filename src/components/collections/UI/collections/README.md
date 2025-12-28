# Collections Components

Компоненты для работы с коллекциями файлов и логикой включения/выключения.

## Компоненты

- **CollectionViewer.svelte** - Просмотр файлов из одной или нескольких коллекций с поддержкой оценки логики
- **CollectionComposer.svelte** - Объединение нескольких коллекций в одну
- **CollectionLogicBuilder.svelte** - UI-конструктор правил логики для коллекций

## Архитектура

Все компоненты используют Svelte 5 runes ($state, $derived, $effect) для реактивности. Коммуникация с backend через Tauri команды из `src-tauri/src/commands/collections.rs`.

## Утилиты

- `src/lib/collections/collection-logic.ts` - Утилиты для работы с правилами логики
- `src/lib/collections/collection-composer.ts` - Утилиты для объединения коллекций

## Типы

- `src/types/collection.ts` - TypeScript типы для коллекций, правил логики и результатов оценки

## Использование

### CollectionViewer

```svelte
<script>
  import CollectionViewer from './components/collections/CollectionViewer.svelte';
  
  let collectionIds = $state([1, 2, 3]);
</script>

<CollectionViewer {collectionIds} showEvaluation={true} />
```

### CollectionComposer

```svelte
<script>
  import CollectionComposer from './components/collections/CollectionComposer.svelte';
  
  function handleCombined(collection) {
    console.log('Collection created:', collection);
  }
</script>

<CollectionComposer onCombined={handleCombined} />
```

### CollectionLogicBuilder

```svelte
<script>
  import CollectionLogicBuilder from './components/collections/CollectionLogicBuilder.svelte';
  
  let collectionId = $state(1);
</script>

<CollectionLogicBuilder {collectionId} />
```

## Backend команды

Все команды находятся в `src-tauri/src/commands/collections.rs`:
- `get_collections()` - получить все коллекции
- `create_collection()` - создать коллекцию
- `evaluate_collection_logic()` - оценить логику коллекции
- `combine_collections()` - объединить коллекции
- И другие...
