<script lang="ts">
  import type { DependencyCheckResult } from '@/components/../types/dependency';
  import { DependencyValidator } from '@/lib/dependencies/dependency-validator';

  const props = $props<{
    checkResult: DependencyCheckResult;
    allFiles: unknown[];
  }>();

  const analysis = $derived(
    DependencyValidator.analyzeCheckResult(props.checkResult, props.allFiles)
  );
</script>

{#if analysis.errors.length > 0 || analysis.warnings.length > 0}
  <div
    class="dependency-warning bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-lg p-4 space-y-3"
  >
    {#if analysis.errors.length > 0}
      <div class="errors">
        <h4 class="text-red-600 dark:text-red-400 font-semibold mb-2">Ошибки зависимостей:</h4>
        <ul class="list-disc list-inside space-y-1 text-sm text-red-700 dark:text-red-300">
          {#each analysis.errors as error, index (index)}
            <li>{error}</li>
          {/each}
        </ul>
      </div>
    {/if}

    {#if analysis.warnings.length > 0}
      <div class="warnings">
        <h4 class="text-yellow-600 dark:text-yellow-400 font-semibold mb-2">Предупреждения:</h4>
        <ul class="list-disc list-inside space-y-1 text-sm text-yellow-700 dark:text-yellow-300">
          {#each analysis.warnings as warning, index (index)}
            <li>{warning}</li>
          {/each}
        </ul>
      </div>
    {/if}

    {#if analysis.recommendations.length > 0}
      <div class="recommendations">
        <h4 class="text-blue-600 dark:text-blue-400 font-semibold mb-2">Рекомендации:</h4>
        <ul class="list-disc list-inside space-y-1 text-sm text-blue-700 dark:text-blue-300">
          {#each analysis.recommendations as recommendation, index (index)}
            <li>{recommendation}</li>
          {/each}
        </ul>
      </div>
    {/if}
  </div>
{/if}
