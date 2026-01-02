<script lang="ts">
  // Этот тип не импортирован - должна быть ошибка
  const settings: NonExistentType = {
    value: "test"
  };

  console.log(settings);
</script>

<div>
  <h1>Test type error detection</h1>
</div>

