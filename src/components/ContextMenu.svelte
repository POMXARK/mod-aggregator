<script lang="ts">
  interface Props {
    x: number;
    y: number;
    onClose: () => void;
    onAddNode: (type: string) => void;
  }

  let { x, y, onClose, onAddNode }: Props = $props();

  const nodeTypes = [
    { type: 'selector', label: 'Selector', icon: '🔍' },
    { type: 'extract', label: 'Extract', icon: '📤' },
    { type: 'filter', label: 'Filter', icon: '🔽' },
    { type: 'transform', label: 'Transform', icon: '🔄' },
    { type: 'output', label: 'Output', icon: '📋' },
  ];

  function handleAddNode(type: string) {
    onAddNode(type);
    onClose();
  }

  function handleClickOutside(e: MouseEvent) {
    const target = e.target as HTMLElement;
    if (!target.closest('.context-menu')) {
      onClose();
    }
  }

  $effect(() => {
    document.addEventListener('click', handleClickOutside);
    return () => {
      document.removeEventListener('click', handleClickOutside);
    };
  });
</script>

<div 
  class="context-menu" 
  style="left: {x}px; top: {y}px;"
  role="menu"
  tabindex="-1"
  onkeydown={(e) => {
    if (e.key === 'Escape') {
      onClose();
    }
  }}
>
  <div class="menu-header">Добавить ноду</div>
  <div class="menu-items">
    {#each nodeTypes as nodeType}
      <button
        class="menu-item"
        onclick={() => handleAddNode(nodeType.type)}
      >
        <span class="menu-icon">{nodeType.icon}</span>
        <span class="menu-label">{nodeType.label}</span>
      </button>
    {/each}
  </div>
</div>

<style>
  .context-menu {
    position: fixed;
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 0.5rem;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
    z-index: 1000;
    min-width: 200px;
    overflow: hidden;
  }

  .menu-header {
    padding: 0.75rem 1rem;
    background: #0f172a;
    border-bottom: 1px solid #334155;
    font-weight: 600;
    color: #e2e8f0;
    font-size: 0.875rem;
  }

  .menu-items {
    padding: 0.5rem;
  }

  .menu-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    width: 100%;
    padding: 0.75rem 1rem;
    background: transparent;
    border: none;
    color: #e2e8f0;
    cursor: pointer;
    border-radius: 0.375rem;
    text-align: left;
    transition: background 0.2s;
  }

  .menu-item:hover {
    background: #334155;
  }

  .menu-icon {
    font-size: 1.2rem;
  }

  .menu-label {
    font-size: 0.9rem;
  }
</style>

