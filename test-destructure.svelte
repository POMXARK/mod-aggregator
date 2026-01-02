<script lang="ts">
  // Тест деструктуризации с неиспользуемой переменной
  const { usedVar, unusedVar } = someFunction();

  console.log(usedVar); // usedVar используется
  // unusedVar не используется
</script>

<div>
  <h1>Test destructuring analysis</h1>
  <p>{usedVar}</p>
</div>

