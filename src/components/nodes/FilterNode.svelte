<script lang="ts">
  import { Handle, Position } from '@xyflow/svelte';
  import type { NodeProps } from '@xyflow/svelte';

  interface FilterNodeData {
    label: string;
    condition: string;
    operator: string;
  }

  type Props = NodeProps<FilterNodeData>;
  let { data, selected }: Props = $props();

  // Initialize state from data prop
  let condition = $state(data.condition || '');
  let operator = $state(data.operator || 'contains');

  // Sync state from data prop when it changes externally
  $effect(() => {
    condition = data.condition || '';
    operator = data.operator || 'contains';
  });

  function handleConditionChange(value: string) {
    condition = value;
    data.condition = value;
  }

  function handleOperatorChange(value: string) {
    operator = value;
    data.operator = value;
  }
</script>

<div class="filter-node" class:selected>
  <Handle type="target" position={Position.Top} />
  
  <div class="node-header">
    <strong>{data.label || 'Filter'}</strong>
  </div>
  
  <div class="node-content">
    <div class="config-field">
      <label for="filter-operator-{data.label || 'default'}">Оператор</label>
      <select 
        id="filter-operator-{data.label || 'default'}"
        value={operator}
        onchange={(e) => handleOperatorChange(e.currentTarget.value)}
      >
        <option value="contains">Содержит</option>
        <option value="equals">Равно</option>
        <option value="starts_with">Начинается с</option>
        <option value="ends_with">Заканчивается на</option>
        <option value="regex">Регулярное выражение</option>
      </select>
    </div>
    
    <div class="config-field">
      <label for="filter-condition-{data.label || 'default'}">Условие</label>
      <input 
        id="filter-condition-{data.label || 'default'}"
        type="text" 
        value={condition}
        oninput={(e) => handleConditionChange(e.currentTarget.value)}
        placeholder="Введите условие"
      />
    </div>
  </div>
  
  <Handle type="source" position={Position.Bottom} />
</div>

<style>
  .filter-node {
    background: #1e293b;
    border: 2px solid #334155;
    border-radius: 0.5rem;
    min-width: 200px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  }
  
  .filter-node.selected {
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
  
  .config-field input,
  .config-field select {
    width: 100%;
    padding: 0.5rem;
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.375rem;
    color: #e2e8f0;
    font-size: 0.875rem;
  }
  
  .config-field input:focus,
  .config-field select:focus {
    outline: none;
    border-color: #0ea5e9;
  }
</style>

