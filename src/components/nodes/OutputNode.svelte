<script lang="ts">
  import { Handle, Position } from '@xyflow/svelte';
  import type { NodeProps } from '@xyflow/svelte';

  interface OutputNodeData {
    label: string;
    fields: string[];
  }

  type Props = NodeProps<OutputNodeData>;
  let { data, selected }: Props = $props();

  let fields = $state(data.fields || ['title', 'url']);

  $effect(() => {
    if (data.fields && JSON.stringify(data.fields) !== JSON.stringify(fields)) {
      fields = [...(data.fields || [])];
    }
  });

  function handleFieldsChange(value: string) {
    fields = value.split(',').map(f => f.trim()).filter(f => f);
    data.fields = fields;
  }
</script>

<div class="output-node" class:selected>
  <Handle type="target" position={Position.Top} />
  
  <div class="node-header">
    <strong>{data.label || 'Output'}</strong>
  </div>
  
  <div class="node-content">
    <div class="config-field">
      <label>Поля вывода (через запятую)</label>
      <input 
        type="text" 
        value={fields.join(', ')}
        oninput={(e) => handleFieldsChange(e.currentTarget.value)}
        placeholder="title, url, version"
      />
    </div>
  </div>
</div>

<style>
  .output-node {
    background: #1e293b;
    border: 2px solid #334155;
    border-radius: 0.5rem;
    min-width: 200px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  }
  
  .output-node.selected {
    border-color: #10b981;
    box-shadow: 0 0 0 2px rgba(16, 185, 129, 0.2);
  }
  
  .node-header {
    padding: 0.75rem;
    background: #0f172a;
    border-bottom: 1px solid #334155;
    border-radius: 0.5rem 0.5rem 0 0;
  }
  
  .node-header strong {
    color: #10b981;
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
    border-color: #10b981;
  }
</style>








