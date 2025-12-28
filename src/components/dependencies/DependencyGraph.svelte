<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@/lib/tauri-wrapper';
  import { SvelteFlow, Background, Controls, MiniMap } from '@xyflow/svelte';
  import '@xyflow/svelte/dist/style.css';
  import type { Node, Edge } from '@xyflow/svelte';
  import type { DependencyGraph as DependencyGraphType } from '@/types/dependency';
  import { DependencyGraph as DependencyGraphUtil } from '@/lib/dependencies/dependency-graph';

  let graphData = $state<DependencyGraphType | null>(null);
  let nodes = $state<Node[]>([]);
  let edges = $state<Edge[]>([]);
  let loading = $state(false);
  let error = $state<string | null>(null);

  onMount(async () => {
    await loadGraph();
  });

  async function loadGraph() {
    loading = true;
    error = null;

    try {
      graphData = await invoke('get_dependency_graph');

      if (graphData) {
        const util = DependencyGraphUtil.fromData(graphData);

        // Преобразуем в формат для SvelteFlow
        nodes = util.getNodes().map((node, index) => ({
          id: node.fileId.toString(),
          type: 'default',
          position: {
            x: (index % 5) * 200,
            y: Math.floor(index / 5) * 150,
          },
          data: {
            label: `${node.name}@${node.version}`,
            hasMissing: node.hasMissingDependencies,
          },
          style: node.hasMissingDependencies
            ? {
                border: '2px solid #ef4444',
                backgroundColor: '#fee2e2',
              }
            : {},
        }));

        edges = util.getEdges().map(edge => ({
          id: `e${edge.fromFileId}-${edge.toFileId}`,
          source: edge.fromFileId.toString(),
          target: edge.toFileId.toString(),
          style: edge.satisfied
            ? { stroke: '#10b981' }
            : { stroke: '#ef4444', strokeDasharray: '5,5' },
          label: edge.dependencyType,
        }));
      }
    } catch (e: unknown) {
      error = e?.toString() ?? 'Ошибка при загрузке графа';
    } finally {
      loading = false;
    }
  }
</script>

<div class="dependency-graph h-full w-full">
  {#if loading}
    <div class="flex items-center justify-center h-full">
      <p class="text-gray-500 dark:text-gray-400">Загрузка графа...</p>
    </div>
  {:else if error}
    <div class="flex items-center justify-center h-full">
      <div
        class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-4"
      >
        <p class="text-red-700 dark:text-red-300">{error}</p>
        <button
          onclick={loadGraph}
          class="mt-2 px-3 py-1.5 bg-red-600 hover:bg-red-700 text-white rounded-lg text-sm"
        >
          Повторить
        </button>
      </div>
    </div>
  {:else}
    <div class="h-full w-full">
      <SvelteFlow {nodes} {edges}>
        <Background />
        <Controls />
        <MiniMap />
      </SvelteFlow>
    </div>
  {/if}
</div>
