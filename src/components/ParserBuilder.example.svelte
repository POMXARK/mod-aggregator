<!-- 
  ПРИМЕР: Обновленный ParserBuilder с использованием универсальной системы нод
  
  Этот файл показывает, как упростить ParserBuilder, используя:
  1. UniversalNode вместо отдельных компонентов
  2. node-factory для создания нод
  3. node-configs для конфигурации
-->

<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '../lib/tauri-wrapper';
  import { SvelteFlow, Background, Controls, MiniMap } from '@xyflow/svelte';
  import '@xyflow/svelte/dist/style.css';
  import type { Node, Edge, Connection, NodeTypes } from '@xyflow/svelte';
  
  // Вместо импорта отдельных компонентов нод
  // import SelectorNode from './nodes/SelectorNode.svelte';
  // import ExtractNode from './nodes/ExtractNode.svelte';
  // ...
  
  // Используем один универсальный компонент
  import UniversalNode from './nodes/UniversalNode.svelte';
  import { createNode } from '../lib/node-factory';
  
  // Остальные импорты...
  import PageViewer from './PageViewer.svelte';
  import ContextMenu from './ContextMenu.svelte';
  import ElementSelector from './ElementSelector.svelte';

  // Один компонент для всех типов нод
  const nodeTypes: NodeTypes = {
    universal: UniversalNode,
  };

  let nodes = $state<Node[]>([]);
  let edges = $state<Edge[]>([]);
  // ... остальные состояния

  /**
   * Упрощенная функция добавления ноды
   * 
   * Вместо большого switch/case используем фабрику
   */
  function handleAddNode(type: string) {
    const position = { 
      x: Math.random() * 400 + 100, 
      y: Math.random() * 400 + 100 
    };
    
    const newNode = createNode(type, position);
    
    if (newNode) {
      nodes = [...nodes, newNode];
      contextMenu = null;
    }
  }

  // Остальные функции остаются без изменений...
</script>

<!-- Остальной код компонента -->

