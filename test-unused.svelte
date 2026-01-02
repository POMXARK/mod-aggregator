<script lang="ts">
  // Этот файл должен показать проблемы неиспользуемых переменных
  import { writable } from 'svelte/store';

  // Неиспользуемая переменная
  const unusedVar = 'test';

  // Неиспользуемая деструктурированная переменная
  const { used, unused } = { used: 1, unused: 2 };

  console.log(used); // used используется
  // unused не используется
</script>

<div>
  <h1>Test unused variables</h1>
</div>
