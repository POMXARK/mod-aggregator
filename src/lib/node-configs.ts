/**
 * Конфигурация полей для различных типов нод
 *
 * Используется для создания универсальных компонентов нод
 * без дублирования кода, как в CRM дашбордах
 */

export type FieldType = 'text' | 'select' | 'number' | 'checkbox' | 'multiselect' | 'textarea';

export interface FieldOption {
  value: string;
  label: string;
}

export interface FieldConfig {
  /** Ключ поля в data объекте */
  key: string;
  /** Тип поля */
  type: FieldType;
  /** Лейбл поля */
  label: string;
  /** Placeholder для input полей */
  placeholder?: string;
  /** Значение по умолчанию */
  defaultValue?: any;
  /** Опции для select полей */
  options?: FieldOption[];
  /** Условное отображение поля (показывать только если другое поле имеет определенное значение) */
  showIf?: {
    field: string;
    value: any;
  };
  /** Валидация */
  validation?: {
    required?: boolean;
    min?: number;
    max?: number;
    pattern?: string;
  };
}

export interface NodeTypeConfig {
  /** Тип ноды */
  type: string;
  /** Лейбл ноды */
  label: string;
  /** Цвет ноды */
  color?: string;
  /** Конфигурация полей */
  fields: FieldConfig[];
  /** Начальные данные */
  defaultData?: Record<string, any>;
}

/**
 * Конфигурации для всех типов нод
 */
export const nodeConfigs: Record<string, NodeTypeConfig> = {
  selector: {
    type: 'selector',
    label: 'Selector',
    color: '#0ea5e9',
    fields: [
      {
        key: 'selector',
        type: 'text',
        label: 'CSS Селектор',
        placeholder: '.mod-item, #content, etc',
        defaultValue: '',
        validation: {
          required: true,
        },
      },
    ],
    defaultData: {
      selector: '',
    },
  },
  extract: {
    type: 'extract',
    label: 'Extract',
    color: '#0ea5e9',
    fields: [
      {
        key: 'attribute',
        type: 'select',
        label: 'Атрибут',
        defaultValue: 'text',
        options: [
          { value: 'text', label: 'Текст' },
          { value: 'html', label: 'HTML' },
          { value: 'href', label: 'Ссылка (href)' },
          { value: 'src', label: 'Изображение (src)' },
          { value: 'data-*', label: 'Data атрибут' },
        ],
      },
      {
        key: 'dataAttribute',
        type: 'text',
        label: 'Data атрибут',
        placeholder: 'data-id, data-value, etc',
        showIf: {
          field: 'attribute',
          value: 'data-*',
        },
      },
      {
        key: 'selector',
        type: 'text',
        label: 'CSS Селектор (опционально)',
        placeholder: '.mod-title',
        defaultValue: '',
      },
    ],
    defaultData: {
      attribute: 'text',
      selector: '',
    },
  },
  filter: {
    type: 'filter',
    label: 'Filter',
    color: '#0ea5e9',
    fields: [
      {
        key: 'operator',
        type: 'select',
        label: 'Оператор',
        defaultValue: 'contains',
        options: [
          { value: 'contains', label: 'Содержит' },
          { value: 'equals', label: 'Равно' },
          { value: 'starts_with', label: 'Начинается с' },
          { value: 'ends_with', label: 'Заканчивается на' },
          { value: 'regex', label: 'Регулярное выражение' },
        ],
      },
      {
        key: 'condition',
        type: 'text',
        label: 'Условие',
        placeholder: 'Введите условие',
        defaultValue: '',
        validation: {
          required: true,
        },
      },
    ],
    defaultData: {
      operator: 'contains',
      condition: '',
    },
  },
  transform: {
    type: 'transform',
    label: 'Transform',
    color: '#0ea5e9',
    fields: [
      {
        key: 'function',
        type: 'select',
        label: 'Функция',
        defaultValue: 'trim',
        options: [
          { value: 'trim', label: 'Обрезать пробелы' },
          { value: 'uppercase', label: 'Верхний регистр' },
          { value: 'lowercase', label: 'Нижний регистр' },
          { value: 'replace', label: 'Заменить' },
          { value: 'extract_number', label: 'Извлечь число' },
          { value: 'extract_date', label: 'Извлечь дату' },
        ],
      },
      {
        key: 'replaceFrom',
        type: 'text',
        label: 'Заменить с',
        placeholder: 'старое значение',
        showIf: {
          field: 'function',
          value: 'replace',
        },
      },
      {
        key: 'replaceTo',
        type: 'text',
        label: 'Заменить на',
        placeholder: 'новое значение',
        showIf: {
          field: 'function',
          value: 'replace',
        },
      },
    ],
    defaultData: {
      function: 'trim',
    },
  },
  output: {
    type: 'output',
    label: 'Output',
    color: '#10b981',
    fields: [
      {
        key: 'fields',
        type: 'textarea',
        label: 'Поля вывода (через запятую)',
        placeholder: 'title, url, version',
        defaultValue: 'title, url',
      },
    ],
    defaultData: {
      fields: ['title', 'url'],
    },
  },
};

/**
 * Получить конфигурацию для типа ноды
 */
export function getNodeConfig(type: string): NodeTypeConfig | undefined {
  return nodeConfigs[type];
}

/**
 * Создать начальные данные для ноды на основе конфигурации
 */
export function createNodeData(type: string): Record<string, any> {
  const config = getNodeConfig(type);
  if (!config) {
    return {};
  }

  const data: Record<string, any> = {
    label: config.label,
    ...config.defaultData,
  };

  // Устанавливаем значения по умолчанию для всех полей
  config.fields.forEach(field => {
    if (field.defaultValue !== undefined && data[field.key] === undefined) {
      data[field.key] = field.defaultValue;
    }
  });

  return data;
}












