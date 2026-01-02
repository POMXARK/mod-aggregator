<script lang="ts">
  import { writable } from 'svelte/store';

  const store = writable('test');
  const derivedStore = $derived(store);

  // ❌ Неправильно - вызов derived store как функции
  const badResult = $derived(derivedStore);

  // ✅ Правильно - использование derived store напрямую
  const goodResult = $derived(derivedStore);

  // ❌ Еще один неправильный паттерн
  const anotherBad = $derived(someFunction);

  console.log(badResult, goodResult);
</script>

<div>
  <h1>Test derived store function call detection</h1>
  <p>Bad: {badResult}</p>
  <p>Good: {goodResult}</p>
</div>

