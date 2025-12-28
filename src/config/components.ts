/**
 * Конфигурация компонентов приложения
 *
 * Позволяет включать/выключать компоненты и настраивать их поведение
 */

export interface ComponentConfig {
  enabled: boolean;
  priority?: number;
  props?: Record<string, any>;
}

export interface AppComponents {
  mods: ComponentConfig;
  sites: ComponentConfig;
  parser: ComponentConfig;
  files: ComponentConfig;
  collections: ComponentConfig;
  notifications: ComponentConfig;
  aiChat: ComponentConfig;
  sidebar: ComponentConfig;
}

// Профили конфигурации для разных режимов разработки
export const developmentProfiles = {
  // Только Parser Builder - для разработки парсера
  parserOnly: {
    mods: { enabled: false, priority: 1 },
    sites: { enabled: false, priority: 2 },
    parser: { enabled: true, priority: 3 },
    files: { enabled: false, priority: 4 },
    collections: { enabled: false, priority: 5 },
    notifications: { enabled: false, priority: 6 },
    aiChat: { enabled: false, priority: 7 },
    sidebar: { enabled: true, priority: 8 },
  },

  // Только управление сайтами (пока отключено - проблемы с импортами)
  sitesOnly: {
    mods: { enabled: false, priority: 1 },
    sites: { enabled: true, priority: 2 }, // Компонент отключен в коде
    parser: { enabled: false, priority: 3 },
    files: { enabled: false, priority: 4 },
    collections: { enabled: false, priority: 5 },
    notifications: { enabled: false, priority: 6 },
    aiChat: { enabled: false, priority: 7 },
    sidebar: { enabled: true, priority: 8 },
  },

  // Только управление файлами (пока отключено - проблемы с импортами)
  filesOnly: {
    mods: { enabled: false, priority: 1 },
    sites: { enabled: false, priority: 2 },
    parser: { enabled: false, priority: 3 },
    files: { enabled: true, priority: 4 }, // Компонент отключен в коде
    collections: { enabled: true, priority: 5 },
    notifications: { enabled: false, priority: 6 },
    aiChat: { enabled: false, priority: 7 },
    sidebar: { enabled: true, priority: 8 },
  },
};

export const defaultComponents: AppComponents = {
  mods: { enabled: true, priority: 1 },
  sites: { enabled: true, priority: 2 },
  parser: { enabled: true, priority: 3 },
  files: { enabled: true, priority: 4 },
  collections: { enabled: true, priority: 5 },
  notifications: { enabled: true, priority: 6 },
  aiChat: { enabled: true, priority: 7 },
  sidebar: { enabled: true, priority: 8 },
};

// Активная конфигурация (можно переопределить из localStorage или env)
export let componentsConfig = { ...defaultComponents };

// Функция для обновления конфигурации
export function updateComponentConfig(updates: Partial<AppComponents>) {
  componentsConfig = { ...componentsConfig, ...updates };
  // Сохранить в localStorage
  localStorage.setItem('components-config', JSON.stringify(componentsConfig));
}

// Функция для применения профиля разработки
export function applyDevelopmentProfile(profileName: keyof typeof developmentProfiles) {
  const profile = developmentProfiles[profileName];
  componentsConfig = { ...profile };
  localStorage.setItem('components-config', JSON.stringify(componentsConfig));
  localStorage.setItem('active-profile', profileName);
  console.log(`Applied development profile: ${profileName}`);
}

// Функция для сброса к полной конфигурации
export function resetToFullConfig() {
  componentsConfig = { ...defaultComponents };
  localStorage.setItem('components-config', JSON.stringify(componentsConfig));
  localStorage.removeItem('active-profile');
  console.log('Reset to full configuration');
}

// Загрузка конфигурации из localStorage
export function loadComponentConfig() {
  try {
    const saved = localStorage.getItem('components-config');
    if (saved) {
      componentsConfig = { ...defaultComponents, ...JSON.parse(saved) };
    }
  } catch (e) {
    console.warn('Failed to load component config:', e);
  }
}

// Получить отсортированный список включенных компонентов
export function getEnabledComponents(): Array<keyof AppComponents> {
  return Object.entries(componentsConfig)
    .filter(([_, config]) => config.enabled)
    .sort(([, a], [, b]) => (a.priority || 0) - (b.priority || 0))
    .map(([key]) => key as keyof AppComponents);
}
