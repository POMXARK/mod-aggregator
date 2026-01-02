# Тестовый скрипт для проверки правила derived stores

$content = @'
<script lang="ts">
  import { writable } from 'svelte/store';

  const store = writable('test');
  const derivedStore = $derived(store);

  // ❌ Неправильно - вызов derived store как функции
  const badResult = $derived(derivedStore());

  // ✅ Правильно - использование derived store напрямую
  const goodResult = $derived(derivedStore);

  console.log(badResult, goodResult);
</script>
'@

Write-Host "Testing derived store function call detection..." -ForegroundColor Cyan

if ($content -match '\$derived\s*\(\s*\w+\s*\(\s*\)\s*\)') {
    Write-Host "Pattern found!" -ForegroundColor Green
    $matches = [regex]::Matches($content, '\$derived\s*\(\s*(\w+)\s*\(\s*\)\s*\)')
    foreach ($match in $matches) {
        $functionName = $match.Groups[1].Value
        Write-Host "Found: $functionName is called as function in $derived()" -ForegroundColor Yellow
    }
} else {
    Write-Host "Pattern not found" -ForegroundColor Red
}

