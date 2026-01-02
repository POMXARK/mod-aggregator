---
name: resize-handle-component
description: Создание переиспользуемых компонентов ResizeHandle для изменения размеров панелей в Svelte 5 приложениях. Использовать когда нужно добавить функциональность изменения размеров между панелями, разделителями или областями интерфейса.
version: 1.0.0
---

# Создание компонента ResizeHandle в Svelte 5

## Основные принципы

### Используй pointer events вместо mouse events
Pointer events поддерживают touch, mouse и stylus одновременно:
```typescript
// ✅ Правильно
onpointerdown={startResizing}

// ❌ Неправильно
onmousedown={startResizing}
```

### Всегда используй глобальные обработчики событий
Локальные обработчики теряют события при быстром движении курсора:
```typescript
function startResizing(e: PointerEvent) {
  // Добавляем глобальные обработчики
  document.addEventListener('pointermove', handlePointerMove, { capture: true });
  document.addEventListener('pointerup', handlePointerUp, { capture: true });
}
```

### Проверяй pointerId для поддержки множественных указателей
```typescript
function handlePointerMove(e: PointerEvent) {
  if (pointerId !== null && e.pointerId !== pointerId) return;
  // Обработка только нашего указателя
}
```

## Типичные ошибки и исправления

### Ошибка: Использование старого reactive синтаксиса
```svelte
<!-- ❌ Неправильно в Svelte 5 -->
$: doubled = count * 2;

<!-- ✅ Правильно -->
let doubled = $derived(count * 2);
```

### Ошибка: Неправильная обработка разных типов state
```typescript
// ✅ Универсальная обработка
function getCurrentValue(): number {
  let rawValue = currentValue;
  if (typeof rawValue === 'object' && rawValue !== null && 'subscribe' in rawValue) {
    rawValue = get(rawValue as any); // Для Svelte stores
  }
  return Number(rawValue); // Для обычных чисел и $state
}
```

### Ошибка: Отсутствие ограничений min/max
```typescript
// ✅ Всегда добавляй ограничения
newValue = Math.max(minValue, Math.min(maxValue, newValue));
```

## Структура компонента

### Props интерфейс
```typescript
interface Props {
  direction: 'vertical' | 'horizontal';
  mode?: 'left' | 'right' | 'top' | 'bottom';
  minValue?: number;
  maxValue?: number;
  currentValue: number | Store<number>;
  onResize: (newValue: number) => void;
  onResizeStart?: () => void;
  onResizeEnd?: () => void;
  className?: string;
  title?: string;
}
```

### Базовый template
```svelte
<div
  bind:this={handleElement}
  class="resize-handle {className}"
  class:resize-handle-vertical={direction === 'vertical'}
  class:resize-handle-horizontal={direction === 'horizontal'}
  onpointerdown={startResizing}
  {title}
  role="separator"
  aria-orientation={direction === 'vertical' ? 'vertical' : 'horizontal'}
></div>
```

## Лучшие практики

1. **Используй CSS flexbox** для правильного layout панелей
2. **Добавляй визуальную обратную связь** при resizing
3. **Очищай ресурсы** в onDestroy
4. **Тестируй на разных устройствах** (mouse, touch, stylus)
5. **Обеспечивай accessibility** с role и aria атрибутами

## Примеры использования

### Базовое использование
```svelte
<script>
  let panelWidth = $state(400);
</script>

<ResizeHandle
  direction="vertical"
  mode="left"
  currentValue={panelWidth}
  onResize={(width) => panelWidth = width}
  minValue={200}
  maxValue={800}
/>
```

### С Svelte stores
```svelte
<script>
  const panelWidth = writable(400);
</script>

<ResizeHandle
  direction="vertical"
  currentValue={panelWidth}
  onResize={(width) => panelWidth.set(width)}
/>
```