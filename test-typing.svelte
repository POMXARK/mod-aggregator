<script lang="ts">
  // Тестовый файл с проблемами типизации
  import { writable } from 'svelte/store';
  import type { AISettings } from './src/components/Parser/UI/ParserBuilder/types/parser-builder.types';

  // Неправильная типизация - должна быть AISettings, а не Writable<AISettings>
  let wrongType: Writable<AISettings> = writable({
    modelType: 'ollama',
    modelName: 'llama3.2:3b',
    apiKey: '',
    ollamaUrl: 'http://localhost:11434',
    description: 'test'
  });

  // Правильная типизация
  let correctType: AISettings = {
    modelType: 'ollama',
    modelName: 'llama3.2:3b',
    apiKey: '',
    ollamaUrl: 'http://localhost:11434',
    description: 'test'
  };
</script>

<div>
  <h1>Test file with typing issues</h1>
  <p>This file should show typing problems</p>
</div>

