---
sidebar_position: 2
---

# PageViewer

Компонент для просмотра HTML страниц и выбора элементов.

## Описание

PageViewer отображает HTML страницу в iframe и позволяет выбирать элементы для создания парсера.

## Использование

```svelte
<PageViewer 
  url={currentUrl}
  on:element-selected={(e) => handleSelection(e.detail)}
/>
```

## API

### Props

- `url: string` - URL страницы для загрузки
- `siteId?: number` - ID сайта для привязки кэша

### Events

- `element-selected` - вызывается при выборе элемента, detail содержит селектор
























