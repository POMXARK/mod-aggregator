# Скрипт для автоматического исправления найденных проблем в коде

param(
    [string]$ResultsFile = "inspection-results\inspection_results_latest.json",
    [switch]$DryRun,
    [switch]$Verbose,
    [string]$BackupDir = "inspection-backups"
)

Write-Host "=== Code Issues Auto-Fix Script ===" -ForegroundColor Green

# Функции
function Write-Info {
    param([string]$Message)
    if ($Verbose) {
        Write-Host "[$((Get-Date).ToString('HH:mm:ss'))] INFO: $Message" -ForegroundColor Blue
    }
}

function Write-Warn {
    param([string]$Message)
    Write-Host "[$((Get-Date).ToString('HH:mm:ss'))] WARN: $Message" -ForegroundColor Yellow
}

function Write-ErrorMsg {
    param([string]$Message)
    Write-Host "[$((Get-Date).ToString('HH:mm:ss'))] ERROR: $Message" -ForegroundColor Red
}

function Write-Success {
    param([string]$Message)
    Write-Host "[$((Get-Date).ToString('HH:mm:ss'))] SUCCESS: $Message" -ForegroundColor Green
}

# Функция для чтения XML результатов WebStorm
function Read-WebStormResults {
    param([string]$XmlFile)

    if (-not (Test-Path $XmlFile)) {
        Write-Warn "WebStorm XML file not found: $XmlFile"
        return $null
    }

    try {
        [xml]$xmlContent = Get-Content $XmlFile -Encoding UTF8

        $webStormProblems = @()
        foreach ($problem in $xmlContent.problems.problem) {
            $filePath = $problem.file -replace 'file://\$PROJECT_DIR\$/', ''
            $line = [int]$problem.line
            $description = $problem.description
            $severity = $problem.problem_class.severity
            $problemClass = $problem.problem_class.id

            # Преобразуем severity WebStorm в наш формат
            $ourSeverity = switch ($severity) {
                "ERROR" { "ERROR" }
                "WARNING" { "WARNING" }
                "WEAK WARNING" { "INFO" }
                "INFO" { "INFO" }
                default { "INFO" }
            }

            $webStormProblems += @{
                file = $filePath
                line = $line
                message = $description
                severity = $ourSeverity
                category = "WebStorm"
                rule = $problemClass
                source = "WebStorm"
            }
        }

        Write-Info "Loaded $($webStormProblems.Count) problems from WebStorm XML"
        return $webStormProblems
    }
    catch {
        Write-Warn "Failed to parse WebStorm XML: $_"
        return $null
    }
}

# Проверка файла результатов
if (-not (Test-Path $ResultsFile)) {
    Write-ErrorMsg "Results file not found: $ResultsFile"
    exit 1
}

# Создание директории для бэкапов
if (-not $DryRun) {
    if (-not (Test-Path $BackupDir)) {
        New-Item -ItemType Directory -Path $BackupDir -Force | Out-Null
    }
}

# Загрузка результатов
Write-Info "Loading results from: $ResultsFile"

$problems = @()

if ($ResultsFile.EndsWith('.xml')) {
    # Загрузка XML результатов WebStorm
    $webStormProblems = Read-WebStormResults -XmlFile $ResultsFile
    if ($webStormProblems) {
        $problems = $webStormProblems
    } else {
        Write-ErrorMsg "Failed to load WebStorm XML results"
        exit 1
    }
} else {
    # Загрузка JSON результатов
    try {
        $results = Get-Content $ResultsFile -Raw | ConvertFrom-Json
        $problems = $results.problems
    } catch {
        Write-ErrorMsg "Failed to parse JSON results file: $_"
        exit 1
    }
}
$fixedCount = 0
$skippedCount = 0

Write-Host "Found $($problems.Count) problems to fix" -ForegroundColor Cyan

foreach ($problem in $problems) {
    $filePath = $problem.file
    $line = $problem.line
    $rule = $problem.rule
    $fix = $problem.fix

    Write-Info "Processing: $filePath (line $line) - $rule"

    if (-not (Test-Path $filePath)) {
        Write-Warn "File not found: $filePath"
        $skippedCount++
        continue
    }

    if (-not $fix) {
        Write-Info "No fix available for $rule"
        $skippedCount++
        continue
    }

    # Создание бэкапа
    if (-not $DryRun) {
        $timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
        $backupName = "$BackupDir\$($filePath -replace '\\', '_')_$timestamp.bak"
        Copy-Item $filePath $backupName -Force
        Write-Info "Backup created: $backupName"
    }

    # Чтение файла
    $content = Get-Content $filePath -Raw
    $lines = $content -split "`n"

    if ($line -gt $lines.Count) {
        Write-Warn "Line $line not found in $filePath"
        $skippedCount++
        continue
    }

    $targetLine = $lines[$line - 1]

    # Применение исправлений
    switch ($fix.type) {
        "remove" {
            if ($DryRun) {
                Write-Host "DRY RUN: Would remove line $line from $filePath" -ForegroundColor Magenta
            } else {
                # Удаление строки
                $newLines = $lines | Where-Object { $_ -ne $targetLine }
                $newLines | Out-File $filePath -Encoding UTF8
                Write-Success "Removed line $line from $filePath"
                $fixedCount++
            }
        }

        "add-attribute" {
            if ($rule -eq "img-alt") {
                if ($DryRun) {
                    Write-Host "DRY RUN: Would add alt attribute to img on line $line in $filePath" -ForegroundColor Magenta
                } else {
                    # Добавление alt атрибута к img
                    $newLine = $targetLine -replace '<img([^>]*)>', '<img$1 alt=""'
                    $lines[$line - 1] = $newLine
                    $lines | Out-File $filePath -Encoding UTF8
                    Write-Success "Added alt attribute to img on line $line in $filePath"
                    $fixedCount++
                }
            }
        }

        "fix-writable-typing" {
            if ($DryRun) {
                Write-Host "DRY RUN: Would change type from $($fix.oldType) to $($fix.newType) on line $line in $filePath" -ForegroundColor Magenta
            } else {
                # Исправление типизации Writable<T> на T
                $newLine = $targetLine -replace $fix.oldType, $fix.newType
                $lines[$line - 1] = $newLine
                $lines | Out-File $filePath -Encoding UTF8
                Write-Success "Changed type from $($fix.oldType) to $($fix.newType) on line $line in $filePath"
                $fixedCount++
            }
        }

        "add-type-import" {
            if ($DryRun) {
                Write-Host "DRY RUN: Would add import for type $($fix.typeName) from $($fix.importPath) in $filePath" -ForegroundColor Magenta
            } else {
                # Добавление импорта типа в начало файла
                $importLine = "import type { $($fix.typeName) } from '$($fix.importPath)';"
                $lines = @($importLine) + $lines
                $lines | Out-File $filePath -Encoding UTF8
                Write-Success "Added import for type $($fix.typeName) from $($fix.importPath) in $filePath"
                $fixedCount++
            }
        }

        "remove-unused-destructured-var" {
            if ($DryRun) {
                Write-Host "DRY RUN: Would remove unused destructured variable $($fix.varName) from $filePath" -ForegroundColor Magenta
            } else {
                # Удаление неиспользуемой переменной из деструктуризации
                $destructuredList = $fix.destructuredVars -join ', '
                $newDestructuredList = ($fix.destructuredVars | Where-Object { $_ -ne $fix.varName }) -join ', '
                $newLine = $targetLine -replace $destructuredList, $newDestructuredList
                $lines[$line - 1] = $newLine
                $lines | Out-File $filePath -Encoding UTF8
                Write-Success "Removed unused destructured variable $($fix.varName) from $filePath"
                $fixedCount++
            }
        }

        "remove-unused-variable" {
            if ($DryRun) {
                Write-Host "DRY RUN: Would remove unused $($fix.varType) variable $($fix.varName) from $filePath" -ForegroundColor Magenta
            } else {
                # Удаление неиспользуемой переменной
                $lines = $lines | Where-Object { $_ -ne $fix.fullLine }
                $lines | Out-File $filePath -Encoding UTF8
                Write-Success "Removed unused $($fix.varType) variable $($fix.varName) from $filePath"
                $fixedCount++
            }
        }

        "remove-unused-import" {
            if ($DryRun) {
                Write-Host "DRY RUN: Would remove unused import $($fix.importName) from $filePath" -ForegroundColor Magenta
            } else {
                # Удаление неиспользуемого импорта
                if ($fix.allImports.Count -eq 1) {
                    # Удаляем всю строку импорта
                    $lines = $lines | Where-Object { $_ -ne $fix.fullLine }
                } else {
                    # Удаляем только конкретный импорт из списка
                    $remainingImports = $fix.allImports | Where-Object { $_ -ne $fix.importName }
                    $newImportsList = $remainingImports -join ', '
                    $newLine = $fix.fullLine -replace [regex]::Escape(($fix.allImports -join ', ')), $newImportsList
                    $lines[$line - 1] = $newLine
                }
                $lines | Out-File $filePath -Encoding UTF8
                Write-Success "Removed unused import $($fix.importName) from $filePath"
                $fixedCount++
            }
        }


        "remove-unused-import" {
            if ($DryRun) {
                Write-Host "DRY RUN: Would remove unused import $($fix.importName) from $filePath" -ForegroundColor Magenta
            } else {
                # Удаление неиспользуемого импорта
                $newLine = $targetLine -replace "\b$($fix.importName)\b\s*,?\s*", "" -replace "\s*,\s*$", "" -replace "\s*{\s*", "{" -replace "\s*}\s*", "}"
                $lines[$line - 1] = $newLine
                $lines | Out-File $filePath -Encoding UTF8
                Write-Success "Removed unused import $($fix.importName) from $filePath"
                $fixedCount++
            }
        }

        "remove-unused-var" {
            if ($DryRun) {
                Write-Host "DRY RUN: Would remove unused variable $($fix.varName) from $filePath" -ForegroundColor Magenta
            } else {
                # Удаление неиспользуемой переменной
                # Удаляем всю строку с объявлением переменной
                $lines = $lines | Where-Object { $_ -ne $targetLine }
                $lines | Out-File $filePath -Encoding UTF8
                Write-Success "Removed unused variable $($fix.varName) from $filePath"
                $fixedCount++
            }
        }

        "remove-unused-destructured-var" {
            if ($DryRun) {
                Write-Host "DRY RUN: Would remove unused destructured variable $($fix.varName) from $filePath" -ForegroundColor Magenta
            } else {
                # Удаление неиспользуемой переменной из деструктуризации
                $newLine = $targetLine -replace "\b$($fix.varName)\b\s*,?\s*", "" -replace "\s*,\s*$", "" -replace "\s*{\s*", "{" -replace "\s*}\s*", "}"
                # Если осталась пустая деструктуризация, удаляем всю строку
                if ($newLine -match 'const\s*\{\s*\}\s*=|let\s*\{\s*\}\s*=') {
                    $lines = $lines | Where-Object { $_ -ne $targetLine }
                } else {
                    $lines[$line - 1] = $newLine
                }
                $lines | Out-File $filePath -Encoding UTF8
                Write-Success "Removed unused destructured variable $($fix.varName) from $filePath"
                $fixedCount++
            }
        }

        default {
            Write-Warn "Unknown fix type: $($fix.type)"
            $skippedCount++
        }
    }
}

# Итоговый отчет
Write-Host ""
Write-Host "=== Fix Results ===" -ForegroundColor Cyan
Write-Host "Fixed: $fixedCount problems" -ForegroundColor Green
Write-Host "Skipped: $skippedCount problems" -ForegroundColor Yellow

if ($DryRun) {
    Write-Host ""
    Write-Host "This was a DRY RUN - no files were modified" -ForegroundColor Magenta
    Write-Host "Run without -DryRun to apply fixes" -ForegroundColor Magenta
} else {
    Write-Host ""
    Write-Host "Backups saved in: $BackupDir" -ForegroundColor Blue
}

Write-Success "Code fix script completed"
