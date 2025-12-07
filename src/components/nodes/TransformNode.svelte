<script lang="ts">
  import { Handle, Position } from '@xyflow/svelte';
  import type { NodeProps } from '@xyflow/svelte';

  interface TransformNodeData {
    label: string;
    function: string;
  }

  type Props = NodeProps<TransformNodeData>;
  let { data, selected }: Props = $props();

  let transformFunction = $state(data.function || 'trim');

  $effect(() => {
    if (data.function !== transformFunction) {
      transformFunction = data.function || 'trim';
    }
  });

  function handleFunctionChange(value: string) {
    transformFunction = value;
    data.function = value;
  }
</script>

<div class="transform-node" class:selected>
  <Handle type="target" position={Position.Top} />
  
  <div class="node-header">
    <strong>{data.label || 'Transform'}</strong>
  </div>
  
  <div class="node-content">
    <div class="config-field">
      <label>Функция</label>
      <select 
        value={transformFunction}
        onchange={(e) => handleFunctionChange(e.currentTarget.value)}
      >
        <option value="trim">Обрезать пробелы</option>
        <option value="uppercase">Верхний регистр</option>
        <option value="lowercase">Нижний регистр</option>
        <option value="replace">Заменить</option>
        <option value="extract_number">Извлечь число</option>
        <option value="extract_date">Извлечь дату</option>
      </select>
    </div>
  </div>
  
  <Handle type="source" position={Position.Bottom} />
</div>

<style>
  .transform-node {
    background: #1e293b;
    border: 2px solid #334155;
    border-radius: 0.5rem;
    min-width: 200px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  }
  
  .transform-node.selected {
    border-color: #0ea5e9;
    box-shadow: 0 0 0 2px rgba(14, 165, 233, 0.2);
  }
  
  .node-header {
    padding: 0.75rem;
    background: #0f172a;
    border-bottom: 1px solid #334155;
    border-radius: 0.5rem 0.5rem 0 0;
  }
  
  .node-header strong {
    color: #0ea5e9;
    font-size: 0.9rem;
  }
  
  .node-content {
    padding: 0.75rem;
  }
  
  .config-field {
    margin-bottom: 0.5rem;
  }
  
  .config-field label {
    display: block;
    font-size: 0.75rem;
    color: #94a3b8;
    margin-bottom: 0.25rem;
  }
  
  .config-field select {
    width: 100%;
    padding: 0.5rem;
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.375rem;
    color: #e2e8f0;
    font-size: 0.875rem;
  }
  
  .config-field select:focus {
    outline: none;
    border-color: #0ea5e9;
  }
</style>








