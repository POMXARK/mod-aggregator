# Скрипт для создания тестового файла с примерами проблем

Write-Host "Creating test file with sample problems..." -ForegroundColor Yellow

# Создание тестового Svelte файла
$testFile = @"
<script>
  let count = 0;
  console.log('Debug message'); // Console.log - should be flagged

  function increment() {
    count++;
    debugger; // Debugger - should be flagged
  }
</script>

<main>
  <h1>Counter: {count}</h1>
  <button onclick={increment}>+</button>
  <img src="image.jpg" /> <!-- Missing alt - should be flagged -->
  <input type="text" /> <!-- Missing label - should be flagged -->
</main>

<style>
  main {
    margin: 50px; /* Magic number */
    padding: 20px;
  }

  button {
    background: #007bff;
    border: none;
    padding: 10px 20px;
    color: white;
    border-radius: 5px;
    cursor: pointer;
  }

  /* Very long line that exceeds 120 characters in a CSS rule with multiple properties and values - should be flagged */
  .very-long-class-name-that-makes-this-line-exceed-the-recommended-length { margin: 0; padding: 0; border: 1px solid #000; background-color: #fff; color: #333; font-size: 16px; line-height: 1.5; }
</style>
"@

# Сохранение тестового файла
$testFilePath = "test-sample.svelte"
$testFile | Out-File -FilePath $testFilePath -Encoding UTF8

Write-Host "Created test file: $testFilePath" -ForegroundColor Green

# Создание тестового JS файла
$testJsFile = @"
// Test file with various issues
import React from 'react'; // May be unused
import { useState } from 'react';

function TestComponent() {
  const [count, setCount] = useState(0);

  console.log('Component rendered'); // Console statement - should be flagged

  const handleClick = () => {
    setCount(count + 1);
    debugger; // Debugger statement - should be flagged
  };

  return (
    <div>
      <h1>Count: {count}</h1>
      <button onClick={handleClick}>
        Increment
      </button>
      <img src="test.jpg" /> {/* Missing alt - should be flagged */}
      <input type="text" placeholder="Enter value" /> {/* Missing label - should be flagged */}
    </div>
  );
}

export default TestComponent;
"@

$testJsFilePath = "test-sample.js"
$testJsFile | Out-File -FilePath $testJsFilePath -Encoding UTF8

Write-Host "Created test JS file: $testJsFilePath" -ForegroundColor Green

Write-Host ""
Write-Host "Test files created. Run inspection to see problems:"
Write-Host ".\scripts\run-code-inspection.ps1 -Include '*.svelte,*.js' -Verbose"
