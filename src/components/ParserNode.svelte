<script lang="ts">
  interface Props {
    data: any;
  }
  
  let { data }: Props = $props();
  
  let config = $state(data?.config || {});
  
  // Update config when data changes
  $effect(() => {
    if (data?.config) {
      config = { ...data.config };
    }
  });
  
  function handleConfigChange(key: string, value: string) {
    config = { ...config, [key]: value };
    data.config = config;
  }
</script>

<div class="parser-node">
  <div class="node-header">
    <strong>{data.label}</strong>
  </div>
  
  <div class="node-content">
    {#if data.nodeType === 'selector'}
      <div class="config-field">
        <label>CSS Селектор</label>
        <input 
          type="text" 
          value={config.selector || ''} 
          oninput={(e) => handleConfigChange('selector', e.currentTarget.value)}
          placeholder=".mod-item"
        />
      </div>
    {:else if data.nodeType === 'extract'}
      <div class="config-field">
        <label>Атрибут</label>
        <input 
          type="text" 
          value={config.attribute || ''} 
          oninput={(e) => handleConfigChange('attribute', e.currentTarget.value)}
          placeholder="href, src, text"
        />
      </div>
    {:else if data.nodeType === 'filter'}
      <div class="config-field">
        <label>Условие</label>
        <input 
          type="text" 
          value={config.condition || ''} 
          oninput={(e) => handleConfigChange('condition', e.currentTarget.value)}
          placeholder="contains('mod')"
        />
      </div>
    {:else if data.nodeType === 'transform'}
      <div class="config-field">
        <label>Функция</label>
        <input 
          type="text" 
          value={config.function || ''} 
          oninput={(e) => handleConfigChange('function', e.currentTarget.value)}
          placeholder="trim, uppercase, etc"
        />
      </div>
    {/if}
  </div>
</div>

<style>
  .parser-node {
    background: #1e293b;
    border: 2px solid #334155;
    border-radius: 0.5rem;
    min-width: 200px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
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

