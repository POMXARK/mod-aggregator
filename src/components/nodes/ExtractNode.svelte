<script lang="ts">
  import { Handle, Position } from '@xyflow/svelte';
  import type { NodeProps } from '@xyflow/svelte';

  interface ExtractNodeData {
    label: string;
    attribute: string;
    selector?: string;
  }

  type Props = NodeProps<ExtractNodeData>;
  let { data, selected }: Props = $props();

  let attribute = $state(data.attribute || 'text');
  let selector = $state(data.selector || '');

  $effect(() => {
    if (data.attribute !== attribute) {
      attribute = data.attribute || 'text';
    }
    if (data.selector !== selector) {
      selector = data.selector || '';
    }
  });

  function handleAttributeChange(value: string) {
    attribute = value;
    data.attribute = value;
  }

  function handleSelectorChange(value: string) {
    selector = value;
    data.selector = value;
  }
</script>

<div class="extract-node" class:selected>
  <Handle type="target" position={Position.Top} />
  
  <div class="node-header">
    <strong>{data.label || 'Extract'}</strong>
  </div>
  
  <div class="node-content">
    <div class="config-field">
      <label>Атрибут</label>
      <select 
        value={attribute}
        onchange={(e) => handleAttributeChange(e.currentTarget.value)}
      >
        <option value="text">Текст</option>
        <option value="html">HTML</option>
        <option value="href">Ссылка (href)</option>
        <option value="src">Изображение (src)</option>
        <option value="data-*">Data атрибут</option>
      </select>
    </div>
    
    {#if attribute === 'data-*'}
      <div class="config-field">
        <label>Data атрибут</label>
        <input 
          type="text" 
          value={selector}
          oninput={(e) => handleSelectorChange(e.currentTarget.value)}
          placeholder="data-id, data-value, etc"
        />
      </div>
    {/if}
  </div>
  
  <Handle type="source" position={Position.Bottom} />
</div>

<style>
  .extract-node {
    background: #1e293b;
    border: 2px solid #334155;
    border-radius: 0.5rem;
    min-width: 200px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  }
  
  .extract-node.selected {
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


