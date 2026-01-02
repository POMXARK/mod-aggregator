<script lang="ts">
  import type { AISettings } from './src/components/Parser/UI/ParserBuilder/types/parser-builder.types';
  import { writable } from 'svelte/store';

  // Эта ошибка должна воспроизвестись:
  // Type Writable<AISettings> is missing the following properties from type AISettings:
  // modelType, modelName, apiKey, ollamaUrl, description

  // Создаем store
  const testStore = writable<AISettings>({
    modelType: 'ollama',
    modelName: 'llama3.2:3b',
    apiKey: '',
    ollamaUrl: 'http://localhost:11434',
    description: 'test'
  });

  // Эта переменная имеет тип Writable<AISettings>
  let wrongVar: Writable<AISettings> = testStore;

  // Теперь пытаемся присвоить ей обычное значение типа AISettings
  // Это должно вызвать ошибку
  $effect(() => {
    wrongVar = {
      modelType: 'openai',
      modelName: 'gpt-4',
      apiKey: 'sk-...',
      ollamaUrl: '',
      description: 'updated'
    };
  });
</script>

<div>
  <h1>Error Reproduction Test</h1>
  <p>This should show the Writable<AISettings> error</p>
</div>

