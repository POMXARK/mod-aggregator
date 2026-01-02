<script lang="ts">
  import type { AISettings } from './src/components/Parser/UI/ParserBuilder/types/parser-builder.types';


  // Используем только AISettings
  let settings: AISettings = {
    modelType: 'ollama',
    modelName: 'llama3.2:3b',
    apiKey: '',
    ollamaUrl: 'http://localhost:11434',
    description: 'test'
  };
</script>

<div>
  <h1>Test unused import</h1>
  <p>This file has an unused import that should be detected and removed.</p>
</div>

