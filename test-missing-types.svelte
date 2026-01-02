<script lang="ts">
  import { writable } from 'svelte/store';

  // Этот тип не импортирован
  const settings: AISettings = {
    modelType: 'ollama',
    modelName: 'test'
  };

  // Этот тип тоже не импортирован
  const parser: ParserSettings = {
    maxElements: 100
  };

  // Этот тип импортирован правильно
  import type { ParserResult } from './types/parser-builder.types';

  const result: ParserResult = {
    success: true
  };

  console.log(settings, parser, result);
</script>

<div>
  <h1>Test missing types detection</h1>
</div>

