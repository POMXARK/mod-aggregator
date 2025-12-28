# Скрипт для массового обновления импортов после упрощения структуры

$files = Get-ChildItem -Path "src/components" -Recurse -Include "*.svelte", "*.ts" -File

foreach ($file in $files) {
    $content = Get-Content $file.FullName -Raw

    # Обновляем импорты из Services/composables
    $content = $content -replace "from '\.\./Services/composables/([^']*)'", "from '@/lib/composables/`$1'"
    $content = $content -replace "from '\.\./\.\./Services/composables/([^']*)'", "from '@/lib/composables/`$1'"
    $content = $content -replace "from '\.\./\.\./\.\./Services/composables/([^']*)'", "from '@/lib/composables/`$1'"

    # Обновляем импорты из lib
    $content = $content -replace "from '\.\./\.\./lib/([^']*)'", "from '@/lib/`$1'"
    $content = $content -replace "from '\.\./\.\./\.\./lib/([^']*)'", "from '@/lib/`$1'"
    $content = $content -replace "from '\.\./\.\./\.\./\.\./lib/([^']*)'", "from '@/lib/`$1'"

    # Обновляем импорты типов
    $content = $content -replace "from '\.\./\.\./Data/Models/types/([^']*)'", "from '@/types/`$1'"
    $content = $content -replace "from '\.\./\.\./\.\./Data/Models/types/([^']*)'", "from '@/types/`$1'"
    $content = $content -replace "from '\.\./\.\./\.\./\.\./Data/Models/types/([^']*)'", "from '@/types/`$1'"

    # Обновляем импорты компонентов
    $content = $content -replace "from '\.\./([^']*)'", "from '@/components/`$1'"
    $content = $content -replace "from '\.\./\.\./([^']*)'", "from '@/components/`$1'"

    # Убираем .js расширения
    $content = $content -replace '\.js([''"])', '$1'
    $content = $content -replace '\.svelte([''"])', '$1'

    # Сохраняем файл
    Set-Content -Path $file.FullName -Value $content -NoNewline
}

Write-Host "Обновление импортов завершено!"
