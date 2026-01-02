<script lang="ts">
  import { PageViewer } from '@/components';

  interface Props {
    visible: boolean;
    width: number;
    url: string;
    siteId?: number | null;
    onElementSelect?: (selector: string, element: HTMLElement | null, data?: unknown) => void;
    onResize?: (newWidth: number) => void;
    onResizeStart?: () => void;
    onResizeEnd?: () => void;
    onLoadComplete: () => void;
    className?: string;
  }

  let {
    visible,
    width,
    url = $bindable(),
    siteId,
    onElementSelect,
    onLoadComplete = undefined,
    className = '',
  }: Props = $props();

  // Debug: следим за изменениями url
  $effect(() => {
    console.log('📺 BrowserPanel: url changed to:', url);
  });

</script>

{#if visible}
  <div class="browser-panel {className}" style="width: {width}px">
    <PageViewer
      bind:url
      {siteId}
      {onElementSelect}
      onLoadComplete={onLoadComplete || (() => {})}
      onRefreshCache={() => console.log('Cache refreshed')}
    />
  </div>
{/if}

<style>
  .browser-panel {
    flex-shrink: 0;
    height: 100%;
    overflow: hidden;
    border-right: 1px solid #334155;
    background: #0f172a;
  }
</style>
