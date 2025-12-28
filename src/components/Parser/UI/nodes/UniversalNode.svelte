<!-- 
  Универсальный компонент ноды
  Использует BaseNode из фреймворка
-->
<script lang="ts">
  import BaseNode from '@/components/framework/BaseNode';
  import type { NodeProps } from '@xyflow/svelte';

  type Props = NodeProps<unknown>;
  const props: Props = $props();
</script>

<BaseNode {...props} />
