#!/usr/bin/env powershell

<#
.SYNOPSIS
    Проверка вызова derived stores как функций в Svelte файлах

.PARAMETER Path
    Путь к файлу или директории для анализа

.PARAMETER Fix
    Автоматически исправить найденные проблемы
#>

param(
    [Parameter(Mandatory = $true)]
    [string]$Path,

    [switch]$Fix
)

Write-Host "==================================================" -ForegroundColor Cyan
Write-Host " Проверка Derived Stores в Svelte файлах" -ForegroundColor Cyan
Write-Host "==================================================" -ForegroundColor Cyan

if (-not (Test-Path $Path)) {
    Write-Host "❌ Путь не существует: $Path" -ForegroundColor Red
    exit 1
}

# Получаем файлы
if (Test-Path $Path -PathType Leaf) {
    $files = @($Path)
} else {
    $files = Get-ChildItem -Path $Path -Include "*.svelte" -Recurse -File -ErrorAction SilentlyContinue
    if ($files.Count -eq 0) {
        Write-Host "⚠️  Svelte файлы не найдены" -ForegroundColor Yellow
        exit 0
    }
    Write-Host "ℹ️  Найдено $($files.Count) Svelte файлов" -ForegroundColor Blue
}

$problemsFound = 0
$fixesApplied = 0

foreach ($file in $files) {
    $filePath = $file.FullName
    try {
        $content = Get-Content $filePath -Raw -ErrorAction Stop

        # Ищем проблемы: $derived(func())
        $matches = [regex]::Matches($content, '\$derived\s*\(\s*(\w+)\s*\(\s*\)\s*\)')
        if ($matches.Count -gt 0) {
            Write-Host "⚠️  Найдено $($matches.Count) проблем в файле: $filePath" -ForegroundColor Yellow

            foreach ($match in $matches) {
                $functionName = $match.Groups[1].Value
                Write-Host "  Проблема: `$derived($functionName())` - вызов derived store как функции" -ForegroundColor Yellow
                Write-Host "  Исправление: `$derived($functionName)`" -ForegroundColor Cyan

                if ($Fix) {
                    $newMatch = "`$derived($functionName)"
                    $content = $content -replace [regex]::Escape($match.Value), $newMatch
                    Write-Host "✅ Исправлено: $($match.Value) → $newMatch" -ForegroundColor Green
                    $fixesApplied++
                }

                $problemsFound++
            }

            # Сохраняем файл если исправляли
            if ($Fix) {
                $content | Out-File $filePath -Encoding UTF8 -NoNewline
                Write-Host "✅ Файл $filePath сохранен" -ForegroundColor Green
            }
        }
    } catch {
        Write-Host "❌ Ошибка при обработке файла $filePath : $($_.Exception.Message)" -ForegroundColor Red
    }
}

Write-Host ""
Write-Host "==================================================" -ForegroundColor Cyan
Write-Host " ИТОГИ АНАЛИЗА" -ForegroundColor Cyan
Write-Host "==================================================" -ForegroundColor Cyan

if ($problemsFound -eq 0) {
    Write-Host "✅ Проблем не найдено! Все derived stores используются правильно." -ForegroundColor Green
} else {
    Write-Host "⚠️  Найдено проблем: $problemsFound" -ForegroundColor Yellow
    if ($Fix -and $fixesApplied -gt 0) {
        Write-Host "✅ Исправлено: $fixesApplied" -ForegroundColor Green
    } elseif (-not $Fix) {
        Write-Host "ℹ️  Используйте -Fix для автоматического исправления" -ForegroundColor Blue
    }
}

