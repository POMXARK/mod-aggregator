<script lang="ts">
  import { Handle, Position } from '@xyflow/svelte';
  import type { NodeProps } from '@xyflow/svelte';

  interface SelectorNodeData {
    label: string;
    selector: string;
  }

  type Props = NodeProps<SelectorNodeData>;
  let { data, selected }: Props = $props();

  let selector = $state(data.selector || '');

  $effect(() => {
    if (data.selector !== selector) {
      selector = data.selector || '';
    }
  });

  function handleSelectorChange(value: string) {
    selector = value;
    data.selector = value;
  }
</script>

<div class="selector-node" class:selected>
  <Handle type="target" position={Position.Top} />
  
  <div class="node-header">
    <strong>{data.label || 'Selector'}</strong>
  </div>
  
  <div class="node-content">
    <div class="config-field">
      <label>CSS Селектор</label>
      <input 
        type="text" 
        value={selector}
        oninput={(e) => handleSelectorChange(e.currentTarget.value)}
        placeholder=".mod-item, #content, etc"
      />
    </div>
  </div>
  
  <Handle type="source" position={Position.Bottom} />
</div>

<style>
  .selector-node {
    background: #1e293b;
    border: 2px solid #334155;
    border-radius: 0.5rem;
    min-width: 200px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  }
  
  .selector-node.selected {
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
  
  .config-field input {
    width: 100%;
    padding: 0.5rem;
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.375rem;
    color: #e2e8f0;
    font-size: 0.875rem;
  }
  
  .config-field input:focus {
    outline: none;
    border-color: #0ea5e9;
  }
</style>








