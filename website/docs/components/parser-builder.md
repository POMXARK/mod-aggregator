---
sidebar_position: 1
---

# ParserBuilder

Визуальный конструктор парсеров с графом нод.

## Описание

ParserBuilder позволяет создавать парсеры визуально, добавляя и настраивая ноды на графе.

## Компоненты нод

- **SelectorNode** - выбор элементов
- **ExtractNode** - извлечение данных
- **FilterNode** - фильтрация
- **TransformNode** - трансформация данных
- **OutputNode** - вывод результата

## Использование

```svelte
<ParserBuilder 
  site={selectedSite}
  on:save={(e) => handleSave(e.detail)}
/>
```

## API

### Props

- `site: Site | null` - текущий сайт для редактирования
- `config: ParserConfig | null` - начальная конфигурация парсера

### Events

- `save` - вызывается при сохранении парсера, detail содержит `ParserConfig`



































