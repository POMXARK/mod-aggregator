<script lang="ts">
  let definedVar = "hello";


  // Test shorthand properties in console.log
  console.log({definedVar,  // Это работает
    undefinedVar: undefined // Это должно вызвать ошибку
  });
</script>

<div>
  <h1>Test shorthand properties</h1>
  <!-- Это должно работать -->
  <p>{{ definedVar }}</p>
  <!-- Это должно вызвать ошибку shorthand-property-undefined -->
  <p>{{ undefinedVar }}</p>
</div>
