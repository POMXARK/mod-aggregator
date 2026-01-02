<!-- ✅ Простой тестовый компонент для демонстрации работы Agent Skills -->
<script lang="ts">
  // Пример правильного использования Svelte 5 runes
  // согласно вашему навыку static-analysis-frontend-skill.md

  let count = $state(0);
  let doubled = $derived(count * 2);

  $effect(() => {
    console.log('Count changed:', count);
  });

  function increment() {
    count++;
  }
</script>

<main class="p-4">
  <h1>✅ Agent Skills Test</h1>
  <p>Count: {count}</p>
  <p>Doubled: {doubled}</p>
  <button onclick={increment} class="btn">Increment</button>
</main>

<style>
  main {
    max-width: 400px;
    margin: 0 auto;
  }

  .btn {
    background: #3b82f6;
    color: white;
    padding: 8px 16px;
    border-radius: 4px;
    border: none;
    cursor: pointer;
  }
</style>