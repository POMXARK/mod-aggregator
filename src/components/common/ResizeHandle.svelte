<script lang="ts">
  import { onDestroy } from 'svelte';

  type ResizeDirection = 'vertical' | 'horizontal';
  type ResizeMode = 'left' | 'right' | 'top' | 'bottom';

  interface Props {
    direction: ResizeDirection;
    mode?: ResizeMode;
    minValue?: number;
    maxValue?: number;
    getCurrentValue: () => number;
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
    getCurrentValue,
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
      stopResizing();
      return;
    }

    // Проверяем pointerId, если он установлен
    if (pointerId !== null && e.pointerId !== pointerId) {
      return;
    }

    let newValue = startValue;

    if (direction === 'vertical') {
      const delta = e.clientX - startPosition;
      if (mode === 'right') {
        newValue = startValue - delta;
      } else {
        newValue = startValue + delta;
      }
    } else {
      const delta = e.clientY - startPosition;
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

    onResize(newValue);
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

    // Получаем текущее значение через callback
    startValue = getCurrentValue();

    // Сохраняем начальную позицию курсора
    if (direction === 'vertical') {
      startPosition = e.clientX;
    } else {
      startPosition = e.clientY;
    }

    isResizing = true;
    pointerId = e.pointerId;

    // Захватываем pointer для работы даже за пределами элемента
    try {
      handleElement.setPointerCapture(e.pointerId);
    } catch (err) {
      console.warn('Failed to set pointer capture:', err);
      stopResizing();
      return;
    }

    // Добавляем стили для body
    document.body.classList.add('resizing-panel');
    document.body.style.userSelect = 'none';
    document.body.style.cursor = direction === 'vertical' ? 'col-resize' : 'row-resize';

    // При использовании setPointerCapture все события pointer идут через элемент,
    // который захватил pointer, поэтому добавляем обработчики на него
    handleElement.addEventListener('pointermove', handlePointerMove, { passive: false });
    handleElement.addEventListener('pointerup', handlePointerUp, { passive: false });
    handleElement.addEventListener('pointercancel', handlePointerCancel, { passive: false });

    // Также добавляем на window/document для надежности на случай, если capture не сработает
    window.addEventListener('pointermove', handlePointerMove, { passive: false });
    window.addEventListener('pointerup', handlePointerUp, { passive: false });
    window.addEventListener('pointercancel', handlePointerCancel, { passive: false });

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
    const savedPointerId = pointerId;
    pointerId = null;

    // Удаляем обработчики с элемента
    if (handleElement) {
      handleElement.removeEventListener('pointermove', handlePointerMove);
      handleElement.removeEventListener('pointerup', handlePointerUp);
      handleElement.removeEventListener('pointercancel', handlePointerCancel);

      // Освобождаем capture
      if (savedPointerId !== null) {
        try {
          handleElement.releasePointerCapture(savedPointerId);
        } catch {
          // Игнорируем ошибки
        }
      }
    }

    // Удаляем обработчики с window
    window.removeEventListener('pointermove', handlePointerMove);
    window.removeEventListener('pointerup', handlePointerUp);
    window.removeEventListener('pointercancel', handlePointerCancel);

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
    z-index: 10;
    touch-action: none;
  }

  .resize-handle-vertical {
    width: 4px;
    cursor: col-resize;
  }

  .resize-handle-horizontal {
    height: 4px;
    cursor: row-resize;
    width: 100%;
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
    left: -2px;
    right: -2px;
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
