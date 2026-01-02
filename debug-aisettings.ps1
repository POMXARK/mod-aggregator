# Отладка проблемы с AISettings

$filePath = "src/components/Parser/UI/ParserBuilder/components/AISettingsModal.svelte"
$content = Get-Content $filePath -Raw -ErrorAction Stop

Write-Host "=== Анализ файла: $filePath ==="

# Поиск импортов типов
$importMatches = [regex]::Matches($content, 'import\s+(?:type\s+)?{([^}]+)}\s+from\s+[''"]@[^''"]*types[^''"]*[''"]')
Write-Host "Найдено импортов типов: $($importMatches.Count)"

$typeImports = @()
foreach ($match in $importMatches) {
    $imports = $match.Groups[1].Value -split ',' | ForEach-Object { $_.Trim() -replace '\s+as\s+\w+', '' }
    Write-Host "  Импорты: $($imports -join ', ')"
    $typeImports += $imports
}

# Поиск использований типов
$varTypeMatches = [regex]::Matches($content, '\b(?:const|let|var)\s+\w+\s*:\s*([A-Z][a-zA-Z0-9_]*(?:<[^>]*>)?)')
Write-Host "Найдено объявлений типов: $($varTypeMatches.Count)"

$typeUsages = @()
foreach ($match in $varTypeMatches) {
    $typeName = $match.Groups[1].Value -replace '<.*', ''
    Write-Host "  Тип: $typeName"
    $typeUsages += $typeName
}

Write-Host ""
Write-Host "=== Проверка AISettings ==="
$hasAISettingsUsage = $typeUsages -contains 'AISettings'
$hasAISettingsImport = $typeImports -contains 'AISettings'

Write-Host "AISettings используется: $hasAISettingsUsage"
Write-Host "AISettings импортирован: $hasAISettingsImport"

if ($hasAISettingsUsage -and -not $hasAISettingsImport) {
    Write-Host "❌ НАЙДЕНА ПРОБЛЕМА: AISettings используется, но не импортирован!"
} else {
    Write-Host "✅ Проблема с AISettings не найдена"
}

Write-Host ""
Write-Host "Все найденные типы:"
$typeUsages | Select-Object -Unique | ForEach-Object { Write-Host "  $_" }
