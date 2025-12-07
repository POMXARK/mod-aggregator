<script lang="ts">
  interface Props {
    class?: string;
  }
  let { class: className = "" }: Props = $props();
</script>

<!-- Улучшенная иконка сайтов - серверы/облако -->
<svg class={className} fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2" style="width: 100%; height: 100%;">
  <path stroke-linecap="round" stroke-linejoin="round" d="M21 12a2.25 2.25 0 00-2.25-2.25H15a2.25 2.25 0 00-2.25 2.25v6a2.25 2.25 0 002.25 2.25h3.75A2.25 2.25 0 0021 18v-6zM3.75 12a2.25 2.25 0 012.25-2.25h3.75A2.25 2.25 0 0112 12v6a2.25 2.25 0 01-2.25 2.25H6A2.25 2.25 0 013.75 18v-6zM13.5 6.75a2.25 2.25 0 012.25-2.25h3.75a2.25 2.25 0 012.25 2.25v6a2.25 2.25 0 01-2.25 2.25h-3.75a2.25 2.25 0 01-2.25-2.25v-6z" />
</svg>

