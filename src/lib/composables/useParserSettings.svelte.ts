/**
 * Composable для управления настройками парсера
 *
 * Управляет настройками парсера: максимальное количество элементов,
 * таймаут выполнения, медленный режим и задержка между элементами.
 *
 * @param initialSettings - начальные настройки парсера
 * @param onSettingsChange - callback для уведомления об изменении настроек
 * @returns Объект с настройками и методами для их управления
 */
export interface ParserSettings {
  maxElements: number;
  timeoutSeconds: number;
  slowMode: boolean;
  delayPerElement: number;
}

export interface UseParserSettingsOptions {
  initialMaxElements?: number;
  initialTimeoutSeconds?: number;
  initialSlowMode?: boolean;
  initialDelayPerElement?: number;
  onSettingsChange?: (settings: ParserSettings) => void;
}

export function useParserSettings(options: UseParserSettingsOptions = {}) {
  const {
    initialMaxElements = 100,
    initialTimeoutSeconds = 60,
    initialSlowMode = false,
    initialDelayPerElement = 500,
    onSettingsChange,
  } = options;

  let maxElements = $state<number>(initialMaxElements);
  let timeoutSeconds = $state<number>(initialTimeoutSeconds);
  let slowMode = $state<boolean>(initialSlowMode);
  let delayPerElement = $state<number>(initialDelayPerElement);

  // Синхронизация с начальными значениями из props
  $effect(() => {
    if (options.initialMaxElements !== undefined && maxElements !== options.initialMaxElements) {
      maxElements = options.initialMaxElements;
    }
    if (
      options.initialTimeoutSeconds !== undefined &&
      timeoutSeconds !== options.initialTimeoutSeconds
    ) {
      timeoutSeconds = options.initialTimeoutSeconds;
    }
    if (options.initialSlowMode !== undefined && slowMode !== options.initialSlowMode) {
      slowMode = options.initialSlowMode;
    }
    if (
      options.initialDelayPerElement !== undefined &&
      delayPerElement !== options.initialDelayPerElement
    ) {
      delayPerElement = options.initialDelayPerElement;
    }
  });

  // Функция для уведомления об изменении настроек
  function notifySettingsChange() {
    if (onSettingsChange) {
      onSettingsChange({ maxElements, timeoutSeconds, slowMode, delayPerElement });
    }
  }

  // Автоматически уведомляем об изменении настроек при их изменении в UI
  let lastSettings = $state<ParserSettings | null>(null);
  $effect(() => {
    const currentSettings = { maxElements, timeoutSeconds, slowMode, delayPerElement };

    // Проверяем, изменились ли настройки (избегаем вызовов при инициализации)
    if (
      lastSettings &&
      (lastSettings.maxElements !== currentSettings.maxElements ||
        lastSettings.timeoutSeconds !== currentSettings.timeoutSeconds ||
        lastSettings.slowMode !== currentSettings.slowMode ||
        lastSettings.delayPerElement !== currentSettings.delayPerElement)
    ) {
      notifySettingsChange();
    }

    lastSettings = currentSettings;
  });

  /**
   * Сбрасывает настройки на значения по умолчанию
   */
  function resetSettings() {
    maxElements = initialMaxElements;
    timeoutSeconds = initialTimeoutSeconds;
    slowMode = initialSlowMode;
    delayPerElement = initialDelayPerElement;
    notifySettingsChange();
  }

  return {
    maxElements,
    timeoutSeconds,
    slowMode,
    delayPerElement,
    resetSettings,
    notifySettingsChange,
  };
}
