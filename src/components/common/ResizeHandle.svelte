<script lang="ts">
  // Force reload: 2026-01-02 01:55 UTC - VERSION 1.0.20
  import { onDestroy } from 'svelte';
  import { get } from 'svelte/store';

  type ResizeDirection = 'vertical' | 'horizontal';
  type ResizeMode = 'left' | 'right' | 'top' | 'bottom';

  interface Props {
    direction: ResizeDirection;
    mode?: ResizeMode;
    minValue?: number;
    maxValue?: number;
    currentValue: number;
    onResize: (newValue: number) => void;
    onResizeStart?: () => void;
    onResizeEnd?: () => void;
    className?: string;
    title?: string;
  }

  const {
    direction,
    mode = 'left',
    minValue = 100,
    maxValue = Infinity,
    currentValue,
    onResize,
    onResizeStart,
    onResizeEnd,
    className = '',
    title = 'Перетащите для изменения размера',
  }: Props = $props();

  let handleElement: HTMLDivElement | null = $state(null);
  let isResizing = $state(false);
  let pointerId: number | null = $state(null);
  let startPosition = $state(0);
  let startValue = $state(0);

  function handlePointerMove(e: PointerEvent) {
    if (!isResizing) {
      return;
    }

    // Проверяем pointerId, если он установлен
    if (pointerId !== null && e.pointerId !== pointerId) {
      return;
    }


    let newValue = startValue;
    let delta;

    if (direction === 'vertical') {
      delta = e.clientX - startPosition;
      if (mode === 'right') {
        newValue = startValue - delta;
      } else {
        newValue = startValue + delta;
      }
    } else {
      delta = e.clientY - startPosition;
      if (mode === 'bottom') {
        newValue = startValue - delta;
      } else {
        newValue = startValue + delta;
      }
    }


    // Применяем ограничения
    if (newValue < minValue) {
      newValue = minValue;
    }
    if (maxValue !== Infinity && newValue > maxValue) {
      newValue = maxValue;
    }

    try {
      onResize(newValue);
    } catch (error) {
      console.error('ResizeHandle: error calling onResize:', error);
    }
    e.preventDefault();
    e.stopPropagation();
  }

  function handlePointerUp(e: PointerEvent) {
    // Обрабатываем только события нашего pointer
    if (pointerId !== null && e.pointerId !== pointerId) {
      return;
    }

    stopResizing();
    e.preventDefault();
    e.stopPropagation();
  }

  function handlePointerCancel(e: PointerEvent) {
    // Обрабатываем только события нашего pointer
    if (pointerId !== null && e.pointerId !== pointerId) {
      return;
    }

    stopResizing();
  }

  // Дополнительный обработчик для случаев, когда pointer уходит за пределы документа
  function handlePointerLeave(e: PointerEvent) {
    // Если pointer уходит за пределы документа во время resizing, продолжаем обработку
    // Это предотвратит прерывание resizing при быстрых движениях
    if (isResizing && pointerId !== null && e.pointerId === pointerId) {
      // Pointer ушел за пределы, но мы продолжаем отслеживать его
      // через глобальные обработчики
    }
  }

  function startResizing(e: PointerEvent) {
    if (e.button !== 0) {
      return;
    } // Только левая кнопка мыши
    if (isResizing) {
      return;
    }

    if (!handleElement) {
      return;
    }

    // Получаем текущее значение
    // currentValue может быть числом, $state proxy или Svelte store
    let rawValue = currentValue;

    // Проверяем тип значения
    if (typeof rawValue === 'object' && rawValue !== null && 'subscribe' in rawValue) {
      // Это Svelte store, получаем значение через get()
      rawValue = get(rawValue as any);
    }

    startValue = Number(rawValue);

    // Сохраняем начальную позицию курсора
    if (direction === 'vertical') {
      startPosition = e.clientX;
    } else {
      startPosition = e.clientY;
    }

    isResizing = true;
    pointerId = e.pointerId;

    // Добавляем стили для body
    document.body.classList.add('resizing-panel');
    document.body.style.userSelect = 'none';
    document.body.style.cursor = direction === 'vertical' ? 'col-resize' : 'row-resize';

    // Добавляем обработчики на document для работы даже за пределами элемента
    // Используем capture фазу для надежной обработки
    document.addEventListener('pointermove', handlePointerMove, { passive: false, capture: true });
    document.addEventListener('pointerup', handlePointerUp, { passive: false, capture: true });
    document.addEventListener('pointercancel', handlePointerCancel, { passive: false, capture: true });
    document.addEventListener('pointerleave', handlePointerLeave, { passive: false, capture: true });

    if (onResizeStart) {
      onResizeStart();
    }
    e.preventDefault();
    e.stopPropagation();
  }

  function stopResizing() {
    if (!isResizing) {
      return;
    }

    isResizing = false;
    pointerId = null;

    // Удаляем обработчики с document
    document.removeEventListener('pointermove', handlePointerMove, { capture: true });
    document.removeEventListener('pointerup', handlePointerUp, { capture: true });
    document.removeEventListener('pointercancel', handlePointerCancel, { capture: true });
    document.removeEventListener('pointerleave', handlePointerLeave, { capture: true });

    // Восстанавливаем стили body
    document.body.classList.remove('resizing-panel');
    document.body.style.userSelect = '';
    document.body.style.cursor = '';

    if (onResizeEnd) {
      onResizeEnd();
    }
  }

  // Очистка при размонтировании компонента
  onDestroy(() => {
    stopResizing();
  });
</script>

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

<style>
  .resize-handle {
    flex-shrink: 0;
    background: transparent;
    transition: background-color 0.2s ease;
    position: relative;
    touch-action: none;
  }

  .resize-handle-vertical {
    width: 6px;
    cursor: col-resize;
    margin: 0 0; /* Минимальная область клика */
  }

  .resize-handle-horizontal {
    height: 6px;
    cursor: row-resize;
    width: 100%;
    margin: 0 0; /* Без расширения для горизонтального */
  }

  .resize-handle:hover {
    background: rgba(14, 165, 233, 0.3);
  }

  .resize-handle:active {
    background: rgba(14, 165, 233, 0.5);
  }

  /* Улучшенная видимость при наведении */
  .resize-handle-vertical:hover::before {
    content: '';
    position: absolute;
    left: -1px;
    right: -1px;
    top: 0;
    bottom: 0;
    background: rgba(14, 165, 233, 0.3);
  }

  .resize-handle-horizontal:hover::before {
    content: '';
    position: absolute;
    left: 0;
    right: 0;
    top: -2px;
    bottom: -2px;
    background: rgba(14, 165, 233, 0.3);
  }
</style>
