/**
 * Типы полей и их конфигурации
 *
 * Определяет все возможные типы полей и их поведение
 */

export type FieldType =
  | 'text'
  | 'select'
  | 'number'
  | 'checkbox'
  | 'multiselect'
  | 'textarea'
  | 'color'
  | 'date'
  | 'time';

export interface FieldOption {
  value: string | number;
  label: string;
  disabled?: boolean;
}

export interface FieldValidation {
  required?: boolean;
  min?: number;
  max?: number;
  minLength?: number;
  maxLength?: number;
  pattern?: string;
  custom?: (value: any) => string | null; // Возвращает ошибку или null
}

export interface FieldCondition {
  field: string;
  operator: 'equals' | 'not_equals' | 'contains' | 'in' | 'not_in';
  value: any;
}

export interface FieldConfig {
  /** Уникальный ключ поля */
  key: string;
  /** Тип поля */
  type: FieldType;
  /** Лейбл поля */
  label: string;
  /** Описание поля (подсказка) */
  description?: string;
  /** Placeholder */
  placeholder?: string;
  /** Значение по умолчанию */
  defaultValue?: any;
  /** Опции для select/multiselect */
  options?: FieldOption[];
  /** Условное отображение */
  showIf?: FieldCondition;
  /** Валидация */
  validation?: FieldValidation;
  /** Дополнительные атрибуты */
  attrs?: Record<string, any>;
}

/**
 * Проверяет, должно ли поле отображаться на основе условий
 */
export function shouldShowField(field: FieldConfig, data: Record<string, any>): boolean {
  if (!field.showIf) {
    return true;
  }

  const { field: dependentField, operator, value } = field.showIf;
  const dependentValue = data[dependentField];

  switch (operator) {
    case 'equals':
      return dependentValue === value;
    case 'not_equals':
      return dependentValue !== value;
    case 'contains':
      return String(dependentValue).includes(String(value));
    case 'in':
      return Array.isArray(value) && value.includes(dependentValue);
    case 'not_in':
      return Array.isArray(value) && !value.includes(dependentValue);
    default:
      return true;
  }
}

/**
 * Валидирует значение поля
 */
export function validateField(field: FieldConfig, value: any): string | null {
  if (!field.validation) {
    return null;
  }

  const { validation } = field;

  // Required
  if (validation.required) {
    if (value === undefined || value === null || value === '') {
      return `${field.label} обязательно для заполнения`;
    }
  }

  // Min/Max для чисел
  if (field.type === 'number' && typeof value === 'number') {
    if (validation.min !== undefined && value < validation.min) {
      return `Минимальное значение: ${validation.min}`;
    }
    if (validation.max !== undefined && value > validation.max) {
      return `Максимальное значение: ${validation.max}`;
    }
  }

  // MinLength/MaxLength для строк
  if (typeof value === 'string') {
    if (validation.minLength !== undefined && value.length < validation.minLength) {
      return `Минимальная длина: ${validation.minLength}`;
    }
    if (validation.maxLength !== undefined && value.length > validation.maxLength) {
      return `Максимальная длина: ${validation.maxLength}`;
    }
  }

  // Pattern
  if (validation.pattern && typeof value === 'string') {
    const regex = new RegExp(validation.pattern);
    if (!regex.test(value)) {
      return `Неверный формат`;
    }
  }

  // Custom validation
  if (validation.custom) {
    return validation.custom(value);
  }

  return null;
}

