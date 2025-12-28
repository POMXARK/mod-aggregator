/**
 * Реестр конфигураций нод
 *
 * Здесь регистрируются все типы нод для конструктора
 */

import { registerNodeType } from './node-config';

/**
 * Регистрирует все стандартные типы нод
 */
export function registerStandardNodeTypes(): void {
  // Selector Node
  registerNodeType({
    type: 'selector',
    label: 'Selector',
    description: 'Выбор элементов по CSS селектору',
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
          minLength: 1,
        },
      },
    ],
    defaultData: {
      selector: '',
    },
    handles: {
      source: true,
      target: false,
    },
  });

  // Extract Node
  registerNodeType({
    type: 'extract',
    label: 'Extract',
    description: 'Извлечение данных из элементов',
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
          operator: 'equals',
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
    handles: {
      source: true,
      target: true,
    },
  });

  // Filter Node
  registerNodeType({
    type: 'filter',
    label: 'Filter',
    description: 'Фильтрация элементов по условию',
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
          minLength: 1,
        },
      },
    ],
    defaultData: {
      operator: 'contains',
      condition: '',
    },
    handles: {
      source: true,
      target: true,
    },
  });

  // Transform Node
  registerNodeType({
    type: 'transform',
    label: 'Transform',
    description: 'Трансформация данных',
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
          operator: 'equals',
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
          operator: 'equals',
          value: 'replace',
        },
      },
    ],
    defaultData: {
      function: 'trim',
    },
    handles: {
      source: true,
      target: true,
    },
  });

  // Output Node
  registerNodeType({
    type: 'output',
    label: 'Output',
    description: 'Выходные поля результата',
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
    handles: {
      source: false,
      target: true,
    },
  });
}
