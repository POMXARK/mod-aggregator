#!/usr/bin/env powershell

<#
.SYNOPSIS
    Проверка вызова derived stores как функций в Svelte файлах

.DESCRIPTION
    Этот скрипт анализирует Svelte файлы на предмет неправильного использования
    derived stores - когда они вызываются как функции внутри $derived().

    Неправильный паттерн: $derived(store())
    Правильный паттерн:   $derived(store)

.PARAMETER Path
    Путь к файлу или директории для анализа

.PARAMETER Fix
    Автоматически исправить найденные проблемы

.PARAMETER DryRun
    Показать, какие исправления будут сделаны, но не применять их

.EXAMPLE
    # Анализ одного файла
    .\check-derived-stores.ps1 -Path "src/components/MyComponent.svelte"

    # Анализ директории
    .\check-derived-stores.ps1 -Path "src"

    # Анализ с автоматическим исправлением
    .\check-derived-stores.ps1 -Path "src" -Fix

    # Предварительный просмотр исправлений
    .\check-derived-stores.ps1 -Path "src" -DryRun
#>

param(
    [Parameter(Mandatory = $true)]
    [string]$Path,

    [switch]$Fix,

    [switch]$DryRun
)

# Функция для красивого вывода
function Write-Header {
    param([string]$Text)
    Write-Host ""
    Write-Host "==================================================" -ForegroundColor Cyan
    Write-Host " $Text" -ForegroundColor Cyan
    Write-Host "==================================================" -ForegroundColor Cyan
}

function Write-Success {
    param([string]$Text)
    Write-Host "✅ $Text" -ForegroundColor Green
}

function Write-Warning {
    param([string]$Text)
    Write-Host "⚠️  $Text" -ForegroundColor Yellow
}

function Write-Error {
    param([string]$Text)
    Write-Host "❌ $Text" -ForegroundColor Red
}

function Write-Info {
    param([string]$Text)
    Write-Host "ℹ️  $Text" -ForegroundColor Blue
}

# Основная функция анализа
function Test-DerivedStoreUsage {
    param(
        [string]$FilePath,
        [ref]$ProblemsFound,
        [ref]$FixesApplied
    )

    try {
        $content = Get-Content $FilePath -Raw -ErrorAction Stop
        if (-not $content) {
            return
        }

        $lines = $content -split "`n"
        $fileProblems = @()

        for ($i = 0; $i -lt $lines.Count; $i++) {
            $line = $lines[$i]
            $lineNumber = $i + 1

            # Ищем паттерн: $derived(store())
            if ($line -match '\$derived\s*\(\s*(\w+)\s*\(\s*\)\s*\)') {
                $match = [regex]::Match($line, '\$derived\s*\(\s*(\w+)\s*\(\s*\)\s*\)')
                if ($match.Success) {
                    $functionName = $match.Groups[1].Value

                    $problem = @{
                        File = $FilePath
                        Line = $lineNumber
                        Column = $line.IndexOf('$derived') + 1
                        FunctionName = $functionName
                        FullMatch = $match.Value
                        LineContent = $line.Trim()
                    }

                    $fileProblems += $problem
                    $ProblemsFound.Value++
                }
            }
        }

        # Выводим найденные проблемы
        if ($fileProblems.Count -gt 0) {
            Write-Warning "Найдено $($fileProblems.Count) проблем в файле: $FilePath"
            foreach ($problem in $fileProblems) {
                Write-Host "  Строка $($problem.Line): $($problem.LineContent)" -ForegroundColor Gray
                Write-Host "  Проблема: Вызов derived store '$($problem.FunctionName)' как функции" -ForegroundColor Yellow
                Write-Host "  Исправление: `$derived($($problem.FunctionName))` вместо `$derived($($problem.FunctionName)())`" -ForegroundColor Cyan
                Write-Host ""
            }

            # Применяем исправления если запрошено
            if ($Fix -or $DryRun) {
                Apply-Fixes -FilePath $FilePath -Problems $fileProblems -FixesApplied $FixesApplied
            }
        }

    } catch {
        Write-Error "Ошибка при анализе файла $FilePath : $($_.Exception.Message)"
    }
}

# Функция для применения исправлений
function Apply-Fixes {
    param(
        [string]$FilePath,
        [array]$Problems,
        [ref]$FixesApplied
    )

    try {
        $content = Get-Content $FilePath -Raw -ErrorAction Stop
        $originalContent = $content

        foreach ($problem in $Problems) {
            if ($DryRun) {
                Write-Host "DRY RUN: В файле $FilePath заменил бы:" -ForegroundColor Magenta
                Write-Host "  '$($problem.FullMatch)' → '`$derived($($problem.FunctionName))'" -ForegroundColor Magenta
            } else {
                # Заменяем вызов функции на прямое использование
                $newMatch = "`$derived($($problem.FunctionName))"
                $content = $content -replace [regex]::Escape($problem.FullMatch), $newMatch
                Write-Success "Исправлено в файле $FilePath : $($problem.FullMatch) → $newMatch"
                $FixesApplied.Value++
            }
        }

        # Сохраняем файл если не dry run
        if (-not $DryRun -and $content -ne $originalContent) {
            $content | Out-File $FilePath -Encoding UTF8 -NoNewline
            Write-Success "Файл $FilePath сохранен"
        }

    } catch {
        Write-Error "Ошибка при исправлении файла $FilePath : $($_.Exception.Message)"
    }
}

function Write-Success {
    param([string]$Text)
    Write-Host "✅ $Text" -ForegroundColor Green
}

function Write-Warning {
    param([string]$Text)
    Write-Host "⚠️  $Text" -ForegroundColor Yellow
}

function Write-Error {
    param([string]$Text)
    Write-Host "❌ $Text" -ForegroundColor Red
}

# Основная функция анализа
function Test-DerivedStoreUsage {
    param(
        [string]$FilePath,
        [ref]$ProblemsFound,
        [ref]$FixesApplied
    )

    try {
        $content = Get-Content $FilePath -Raw -ErrorAction Stop
        if (-not $content) {
            return
        }

        $lines = $content -split "`n"
        $fileProblems = @()

        for ($i = 0; $i -lt $lines.Count; $i++) {
            $line = $lines[$i]
            $lineNumber = $i + 1

            # Ищем паттерн: $derived(store())
            if ($line -match '\$derived\s*\(\s*(\w+)\s*\(\s*\)\s*\)') {
                $match = [regex]::Match($line, '\$derived\s*\(\s*(\w+)\s*\(\s*\)\s*\)')
                if ($match.Success) {
                    $functionName = $match.Groups[1].Value

                    $problem = @{
                        File = $FilePath
                        Line = $lineNumber
                        Column = $line.IndexOf('$derived') + 1
                        FunctionName = $functionName
                        FullMatch = $match.Value
                        LineContent = $line.Trim()
                    }

                    $fileProblems += $problem
                    $ProblemsFound.Value++
                }
            }
        }

        # Выводим найденные проблемы
        if ($fileProblems.Count -gt 0) {
            Write-Warning "Найдено $($fileProblems.Count) проблем в файле: $FilePath"
            foreach ($problem in $fileProblems) {
                Write-Host "  Строка $($problem.Line): $($problem.LineContent)" -ForegroundColor Gray
                Write-Host "  Проблема: Вызов derived store '$($problem.FunctionName)' как функции" -ForegroundColor Yellow
                Write-Host "  Исправление: `$derived($($problem.FunctionName))` вместо `$derived($($problem.FunctionName)())`" -ForegroundColor Cyan
                Write-Host ""
            }

            # Применяем исправления если запрошено
            if ($Fix -or $DryRun) {
                Apply-Fixes -FilePath $FilePath -Problems $fileProblems -FixesApplied $FixesApplied
            }
        }

    } catch {
        Write-Error "Ошибка при анализе файла $FilePath : $($_.Exception.Message)"
    }
}

# Функция для применения исправлений
function Apply-Fixes {
    param(
        [string]$FilePath,
        [array]$Problems,
        [ref]$FixesApplied
    )

    try {
        $content = Get-Content $FilePath -Raw -ErrorAction Stop
        $originalContent = $content

        foreach ($problem in $Problems) {
            if ($DryRun) {
                Write-Host "DRY RUN: В файле $FilePath заменил бы:" -ForegroundColor Magenta
                Write-Host "  '$($problem.FullMatch)' → '`$derived($($problem.FunctionName))'" -ForegroundColor Magenta
            } else {
                # Заменяем вызов функции на прямое использование
                $newMatch = "`$derived($($problem.FunctionName))"
                $content = $content -replace [regex]::Escape($problem.FullMatch), $newMatch
                Write-Success "Исправлено в файле $FilePath : $($problem.FullMatch) → $newMatch"
                $FixesApplied.Value++
            }
        }

        # Сохраняем файл если не dry run
        if (-not $DryRun -and $content -ne $originalContent) {
            $content | Out-File $FilePath -Encoding UTF8 -NoNewline
            Write-Success "Файл $FilePath сохранен"
        }

    } catch {
        Write-Error "Ошибка при исправлении файла $FilePath : $($_.Exception.Message)"
    }
}

# Основная логика скрипта
function Main {
    Write-Header "Проверка Derived Stores в Svelte файлах"

    if ($Fix -and $DryRun) {
        Write-Error "Нельзя использовать -Fix и -DryRun одновременно"
        exit 1
    }

    # Проверяем существование пути
    if (-not (Test-Path $Path)) {
        Write-Error "Путь не существует: $Path"
        exit 1
    }

    # Получаем список файлов для анализа
    if (Test-Path $Path -PathType Leaf) {
        # Это файл
        $files = @($Path)
    } else {
        # Это директория - ищем все .svelte файлы
        Write-Info "Поиск Svelte файлов в директории: $Path"
        $files = Get-ChildItem -Path $Path -Include "*.svelte" -Recurse -File -ErrorAction SilentlyContinue
        if ($files.Count -eq 0) {
            Write-Warning "Svelte файлы не найдены в указанной директории"
            exit 0
        }
        Write-Info "Найдено $($files.Count) Svelte файлов"
    }

    # Анализируем файлы
    $problemsFound = 0
    $fixesApplied = 0

    Write-Info "Начинаем анализ..."
    Write-Host ""

    foreach ($file in $files) {
        $filePath = $file.FullName
        Test-DerivedStoreUsage -FilePath $filePath -ProblemsFound ([ref]$problemsFound) -FixesApplied ([ref]$fixesApplied)
    }

    # Выводим итоги
    Write-Host ""
    Write-Header "ИТОГИ АНАЛИЗА"

    if ($problemsFound -eq 0) {
        Write-Success "Проблем не найдено! Все derived stores используются правильно."
    } else {
        Write-Warning "Найдено проблем: $problemsFound"

        if ($Fix -and $fixesApplied -gt 0) {
            Write-Success "Исправлено проблем: $fixesApplied"
        } elseif ($DryRun) {
            Write-Info "Это был DRY RUN - никакие файлы не были изменены"
        } elseif (-not $Fix) {
            Write-Info "Для автоматического исправления используйте параметр -Fix"
        }
    }

    Write-Host ""
    Write-Info "Анализ завершен"
}

# Запускаем основную функцию
Main
