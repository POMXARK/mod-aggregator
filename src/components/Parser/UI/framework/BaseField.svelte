<script lang="ts">
  import type { FieldConfig } from '@/lib/framework/field-types';
  import { validateField } from '@/lib/framework/field-types';

  interface Props {
    field: FieldConfig;
    value: unknown;
    onValueChange: (key: string, value: unknown) => void;
    nodeId?: string;
    error?: string | null;
  }

  const { field, value, onValueChange, nodeId, error: externalError }: Props = $props();

  let internalError = $state<string | null>(null);
  let touched = $state(false);

  const fieldId = `field-${field.key}-${nodeId || 'default'}`;
  const displayError = $derived(externalError || (touched ? internalError : null));

  function handleChange(newValue: unknown) {
    touched = true;
    const validationError = validateField(field, newValue);
    internalError = validationError;
    onValueChange(field.key, newValue);
  }

  function handleBlur() {
    touched = true;
    const validationError = validateField(field, value);
    internalError = validationError;
  }

  // Обработка специальных типов
  function handleTextareaChange(e: Event) {
    const target = e.currentTarget as HTMLTextAreaElement;
    if (field.key === 'fields') {
      // Для полей типа fields (массив через запятую)
      const fields = target.value
        .split(',')
        .map(f => f.trim())
        .filter(f => f);
      handleChange(fields);
    } else {
      handleChange(target.value);
    }
  }

  function handleMultiselectChange(e: Event) {
    const target = e.currentTarget as HTMLSelectElement;
    const selected = Array.from(target.selectedOptions, option => option.value);
    handleChange(selected);
  }
</script>

<div class="base-field" class:has-error={displayError}>
  <label for={fieldId}>
    {field.label}
    {#if field.validation?.required}
      <span class="required">*</span>
    {/if}
  </label>

  {#if field.description}
    <div class="description">{field.description}</div>
  {/if}

  {#if field.type === 'text' || field.type === 'number' || field.type === 'color' || field.type === 'date' || field.type === 'time'}
    <input
      id={fieldId}
      type={field.type}
      value={value ?? ''}
      oninput={e =>
        handleChange(
          field.type === 'number' ? Number(e.currentTarget.value) : e.currentTarget.value
        )}
      onblur={handleBlur}
      placeholder={field.placeholder}
      required={field.validation?.required}
      min={field.validation?.min}
      max={field.validation?.max}
      minlength={field.validation?.minLength}
      maxlength={field.validation?.maxLength}
      pattern={field.validation?.pattern}
      {...field.attrs || {}}
    />
  {:else if field.type === 'select'}
    <select
      id={fieldId}
      value={value ?? field.defaultValue ?? ''}
      onchange={e => handleChange(e.currentTarget.value)}
      onblur={handleBlur}
      {...field.attrs || {}}
    >
      {#if field.options}
        {#each field.options as option (option.value)}
          <option value={option.value} disabled={option.disabled}>
            {option.label}
          </option>
        {/each}
      {/if}
    </select>
  {:else if field.type === 'multiselect'}
    <select
      id={fieldId}
      multiple
      onchange={handleMultiselectChange}
      onblur={handleBlur}
      {...field.attrs || {}}
    >
      {#if field.options}
        {#each field.options as option (option.value)}
          <option
            value={option.value}
            selected={Array.isArray(value) && value.includes(String(option.value))}
            disabled={option.disabled}
          >
            {option.label}
          </option>
        {/each}
      {/if}
    </select>
  {:else if field.type === 'textarea'}
    <textarea
      id={fieldId}
      value={typeof value === 'string' ? value : Array.isArray(value) ? value.join(', ') : ''}
      oninput={handleTextareaChange}
      onblur={handleBlur}
      placeholder={field.placeholder}
      required={field.validation?.required}
      minlength={field.validation?.minLength}
      maxlength={field.validation?.maxLength}
      rows={3}
      {...field.attrs || {}}
    ></textarea>
  {:else if field.type === 'checkbox'}
    <input
      id={fieldId}
      type="checkbox"
      checked={value ?? false}
      onchange={e => handleChange(e.currentTarget.checked)}
      onblur={handleBlur}
      {...field.attrs || {}}
    />
  {/if}

  {#if displayError}
    <span class="error">{displayError}</span>
  {/if}
</div>

<style>
  .base-field {
    margin-bottom: 0.75rem;
  }

  .base-field label {
    display: block;
    font-size: 0.75rem;
    color: #94a3b8;
    margin-bottom: 0.25rem;
    font-weight: 500;
  }

  .base-field .required {
    color: #ef4444;
    margin-left: 0.25rem;
  }

  .base-field .description {
    font-size: 0.7rem;
    color: #64748b;
    margin-bottom: 0.25rem;
    font-style: italic;
  }

  .base-field input,
  .base-field select,
  .base-field textarea {
    width: 100%;
    padding: 0.5rem;
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.375rem;
    color: #e2e8f0;
    font-size: 0.875rem;
    font-family: inherit;
    transition:
      border-color 0.2s,
      box-shadow 0.2s;
  }

  .base-field.has-error input,
  .base-field.has-error select,
  .base-field.has-error textarea {
    border-color: #ef4444;
  }

  .base-field input:focus,
  .base-field select:focus,
  .base-field textarea:focus {
    outline: none;
    border-color: #0ea5e9;
    box-shadow: 0 0 0 2px rgba(14, 165, 233, 0.1);
  }

  .base-field.has-error input:focus,
  .base-field.has-error select:focus,
  .base-field.has-error textarea:focus {
    border-color: #ef4444;
    box-shadow: 0 0 0 2px rgba(239, 68, 68, 0.1);
  }

  .base-field textarea {
    resize: vertical;
    min-height: 60px;
  }

  .base-field select[multiple] {
    min-height: 100px;
  }

  .base-field .error {
    display: block;
    font-size: 0.7rem;
    color: #ef4444;
    margin-top: 0.25rem;
  }
</style>
