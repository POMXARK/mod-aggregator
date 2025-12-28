<script lang="ts">
  import { Handle, Position } from '@xyflow/svelte';
  import type { NodeProps } from '@xyflow/svelte';
  import { getNodeConfig } from '@/lib/framework/node-config';
  import { shouldShowField } from '@/lib/framework/field-types';
  import BaseField from './BaseField.svelte';

  interface UniversalNodeData {
    nodeType: string;
    label: string;
    [key: string]: unknown;
  }

  type Props = NodeProps<UniversalNodeData>;
  const { data, selected }: Props = $props();

  // Получаем конфигурацию ноды
  const nodeType = $derived(data.nodeType || 'selector');
  const config = $derived(getNodeConfig(nodeType));

  // Синхронизация состояния с data
  $effect(() => {
    if (config) {
      // Устанавливаем значения по умолчанию для отсутствующих полей
      config.fields.forEach(field => {
        if (data[field.key] === undefined && field.defaultValue !== undefined) {
          data[field.key] = field.defaultValue;
        }
      });
    }
  });

  // Управление состоянием полей - упрощенная версия
  function getFieldValue(key: string) {
    return data[key];
  }

  function setFieldValue(key: string, value: unknown) {
    data[key] = value;
  }

  function handleFieldChange(key: string, value: unknown) {
    setFieldValue(key, value);
  }

  // Фильтруем поля по условиям отображения
  const visibleFields = $derived(
    config?.fields.filter(field => shouldShowField(field, data)) || []
  );

  const nodeColor = $derived(config?.color || '#0ea5e9');
  const hasSource = $derived(config?.handles?.source !== false);
  const hasTarget = $derived(config?.handles?.target !== false);
</script>

<div class="base-node" class:selected style="--node-color: {nodeColor};">
  {#if hasTarget}
    <Handle type="target" position={Position.Top} />
  {/if}

  <div class="node-header">
    {#if config?.icon}
      <span class="node-icon">{config.icon}</span>
    {/if}
    <strong>{data.label || config?.label || 'Node'}</strong>
  </div>

  {#if config?.description}
    <div class="node-description">{config.description}</div>
  {/if}

  <div class="node-content">
    {#if config}
      {#each visibleFields as field (field.key)}
        <BaseField
          {field}
          value={getFieldValue(field.key)}
          onValueChange={handleFieldChange}
          nodeId={data.id}
        />
      {/each}
    {:else}
      <div class="error">Неизвестный тип ноды: {nodeType}</div>
    {/if}
  </div>

  {#if hasSource}
    <Handle type="source" position={Position.Bottom} />
  {/if}
</div>

<style>
  .base-node {
    background: #1e293b;
    border: 2px solid #334155;
    border-radius: 0.5rem;
    min-width: 200px;
    max-width: 300px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
    transition: all 0.2s;
  }

  .base-node.selected {
    border-color: var(--node-color, #0ea5e9);
    box-shadow: 0 0 0 2px color-mix(in srgb, var(--node-color, #0ea5e9) 20%, transparent);
  }

  .node-header {
    padding: 0.75rem;
    background: #0f172a;
    border-bottom: 1px solid #334155;
    border-radius: 0.5rem 0.5rem 0 0;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .node-icon {
    font-size: 1rem;
  }

  .node-header strong {
    font-size: 0.9rem;
    font-weight: 600;
    color: var(--node-color, #0ea5e9);
  }

  .node-description {
    padding: 0.5rem 0.75rem;
    font-size: 0.75rem;
    color: #64748b;
    background: #0f172a;
    border-bottom: 1px solid #334155;
  }

  .node-content {
    padding: 0.75rem;
  }

  .error {
    color: #ef4444;
    font-size: 0.875rem;
    padding: 0.5rem;
    background: rgba(239, 68, 68, 0.1);
    border-radius: 0.375rem;
  }
</style>
