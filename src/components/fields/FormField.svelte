<script lang="ts">
  import type { FieldConfig } from '@/lib/node-configs';

  interface Props {
    field: FieldConfig;
    value: unknown;
    onValueChange: (key: string, value: unknown) => void;
    nodeId?: string;
  }

  const { field, value, onValueChange, nodeId }: Props = $props();

  function handleChange(newValue: unknown) {
    onValueChange(field.key, newValue);
  }

  function handleTextareaChange(e: Event) {
    const target = e.currentTarget as HTMLTextAreaElement;
    // Для полей типа fields (массив через запятую)
    if (field.key === 'fields') {
      const fields = target.value
        .split(',')
        .map(f => f.trim())
        .filter(f => f);
      handleChange(fields);
    } else {
      handleChange(target.value);
    }
  }
</script>

<div class="config-field">
  <label for="field-{field.key}-{nodeId || 'default'}">
    {field.label}
    {#if field.validation?.required}
      <span class="required">*</span>
    {/if}
  </label>

  {#if field.type === 'text' || field.type === 'number'}
    <input
      id="field-{field.key}-{nodeId || 'default'}"
      type={field.type}
      value={value ?? ''}
      oninput={e =>
        handleChange(
          field.type === 'number' ? Number(e.currentTarget.value) : e.currentTarget.value
        )}
      placeholder={field.placeholder}
      required={field.validation?.required}
      min={field.validation?.min}
      max={field.validation?.max}
      pattern={field.validation?.pattern}
    />
  {:else if field.type === 'select'}
    <select
      id="field-{field.key}-{nodeId || 'default'}"
      value={value ?? field.defaultValue ?? ''}
      onchange={e => handleChange(e.currentTarget.value)}
    >
      {#if field.options}
        {#each field.options as option (option.value)}
          <option value={option.value}>{option.label}</option>
        {/each}
      {/if}
    </select>
  {:else if field.type === 'textarea'}
    <textarea
      id="field-{field.key}-{nodeId || 'default'}"
      value={typeof value === 'string' ? value : Array.isArray(value) ? value.join(', ') : ''}
      oninput={handleTextareaChange}
      placeholder={field.placeholder}
      required={field.validation?.required}
      rows={3}
    ></textarea>
  {:else if field.type === 'checkbox'}
    <input
      id="field-{field.key}-{nodeId || 'default'}"
      type="checkbox"
      checked={value ?? false}
      onchange={e => handleChange(e.currentTarget.checked)}
    />
  {/if}

  {#if field.validation?.required && !value && value !== 0}
    <span class="error">Обязательное поле</span>
  {/if}
</div>

<style>
  .config-field {
    margin-bottom: 0.75rem;
  }

  .config-field label {
    display: block;
    font-size: 0.75rem;
    color: #94a3b8;
    margin-bottom: 0.25rem;
    font-weight: 500;
  }

  .config-field .required {
    color: #ef4444;
    margin-left: 0.25rem;
  }

  .config-field input,
  .config-field select,
  .config-field textarea {
    width: 100%;
    padding: 0.5rem;
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.375rem;
    color: #e2e8f0;
    font-size: 0.875rem;
    font-family: inherit;
  }

  .config-field input:focus,
  .config-field select:focus,
  .config-field textarea:focus {
    outline: none;
    border-color: #0ea5e9;
    box-shadow: 0 0 0 2px rgba(14, 165, 233, 0.1);
  }

  .config-field textarea {
    resize: vertical;
    min-height: 60px;
  }

  .config-field .error {
    display: block;
    font-size: 0.7rem;
    color: #ef4444;
    margin-top: 0.25rem;
  }
</style>
