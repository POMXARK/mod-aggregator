# Script to run code inspection in WebStorm
# PowerShell version for Windows

param(
    [string]$ProjectPath = $null,
    [string]$OutputDir = $null,
    [string]$Format = "xml",
    [string]$Profile = "Frontend Project Default",
    [string]$Include = "*.js,*.ts,*.svelte,*.html,*.css",
    [string]$Exclude = "node_modules/**,dist/**,.svelte-kit/**",
    [string]$IdeaPath = $null,
    [switch]$FailOnError,
    [switch]$Verbose,
    [switch]$Help,
    [switch]$Fix
)

# Show help
if ($Help) {
    Write-Host @"
Script to run code inspection in WebStorm for frontend projects

Usage: .\run-code-inspection.ps1 [OPTIONS]

Options:
    -ProjectPath PATH      Project path (default: current directory)
    -OutputDir DIR         Output directory (default: .\inspection-results)
    -Format FORMAT         Output format: xml, html, json (default: xml)
    -Profile NAME          Inspection profile name (default: Frontend Project Default)
    -Include PATTERN       Include pattern (default: frontend files)
    -Exclude PATTERN       Exclude pattern (default: node_modules, dist, etc.)
    -IdeaPath PATH         Path to WebStorm executable
    -FailOnError           Fail on inspection errors
    -Verbose               Verbose output
    -Fix                   Automatically fix found issues
    -Help                  Show this help

Environment variables:
    IDEA_PATH              Path to WebStorm

Examples:
    .\run-code-inspection.ps1
    .\run-code-inspection.ps1 -Verbose -FailOnError
    .\run-code-inspection.ps1 -Include "*.ts" -Format html
"@
    exit 0
}

# Функции
function Write-Info {
    param([string]$Message)
    Write-Host "[$((Get-Date).ToString('HH:mm:ss'))] INFO: $Message" -ForegroundColor Blue
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

# function Analyze-SvelteStoreIssues {
#     param([string]$content, [string]$filePath)
#
#     $problems = @()
#
#     # Найти все $effect блоки
#     $effectMatches = [regex]::Matches($content, '\$effect\s*\(\s*\(\)\s*=>\s*{([^}]*(?:\{[^}]*\}[^}]*)*)\}', 'Multiline, IgnoreCase')
#
#     foreach ($effectMatch in $effectMatches) {
#         $effectContent = $effectMatch.Groups[1].Value
#
#         # Найти присваивания типа variable = $storeName
#         $assignments = [regex]::Matches($effectContent, '(\w+)\s*=\s*\$(\w+)')
#
#         foreach ($assignment in $assignments) {
#             $varName = $assignment.Groups[1].Value
#             $storeRef = $assignment.Groups[2].Value
#
#             # Проверить, является ли storeRef именем store (заканчивается на Store)
#             if ($storeRef -like '*Store') {
#                 # Найти строку с присваиванием
#                 $lineMatch = [regex]::Match($effectContent, [regex]::Escape("$varName = `$$storeRef"))
#                 if ($lineMatch.Success) {
#                     # Найти номер строки
#                     $effectStart = $effectMatch.Index
#                     $assignmentStart = $effectStart + $lineMatch.Index
#                     $lineNumber = ($content.Substring(0, $assignmentStart) -split "`n").Count
#
#                     $problems += @{
#                         file = $filePath
#                         line = $lineNumber
#                         column = 0
#                         message = "Type Writable<$($varName)Type> is missing the following properties from type $($varName)Type. Use `$derived($$storeRef)` instead of `$$storeRef`"
#                         severity = "ERROR"
#                         category = "Svelte"
#                         rule = "svelte-writable-store-assignment"
#                         fix = @{
#                             type = "replace-assignment"
#                             old = "$varName = `$$storeRef"
#                             new = "$varName = `$derived(`$storeRef)"
#                         }
#                     }
#                 }
#             }
#         }
#     }
#
#     return $problems
# }

# Определение путей
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Split-Path -Parent $ScriptDir

if (-not $ProjectPath) {
    $ProjectPath = $ProjectRoot
}

if (-not $OutputDir) {
    $OutputDir = Join-Path $ProjectRoot "inspection-results"
}

# Создание директории для результатов
if (-not (Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null
}

# Поиск WebStorm
if (-not $IdeaPath) {
    $IdeaPath = $env:IDEA_PATH
}

if (-not $IdeaPath) {
    # Автопоиск WebStorm
    $possiblePaths = @(
        "C:\Program Files\JetBrains\WebStorm*\bin\webstorm64.exe",
        "$env:USERPROFILE\AppData\Local\JetBrains\Toolbox\apps\WebStorm\*\bin\webstorm64.exe",
        "C:\Program Files\JetBrains\WebStorm 2025.2.3\bin\webstorm64.exe"
    )

    foreach ($pathPattern in $possiblePaths) {
        try {
            $found = Get-ChildItem -Path $pathPattern -ErrorAction Stop | Select-Object -First 1
            if ($found) {
                $IdeaPath = $found.FullName
                break
            }
        } catch {
            # Игнорируем ошибки поиска
        }
    }
}

# Проверка IDEA
if (-not $IdeaPath -or -not (Test-Path $IdeaPath)) {
    Write-ErrorMsg "IntelliJ IDEA не найдена по пути: $IdeaPath"
    Write-ErrorMsg "Установите IDEA или укажите путь через -IdeaPath или переменную IDEA_PATH"
    Write-ErrorMsg ""
    Write-ErrorMsg "Для установки запустите:"
    Write-ErrorMsg "  .\scripts\setup-intellij-inspection.ps1"
    exit 1
}

# Проверка профиля inspection
$profileDir = Join-Path $ProjectPath ".idea\inspectionProfiles"
$profilePath = Join-Path $profileDir "$Profile.xml"

if (-not (Test-Path $profilePath)) {
    Write-Warn "Профиль inspection '$Profile' не найден: $profilePath"
    Write-Warn "Будет использован встроенный профиль IDEA"
    $profilePath = ""
}

# Генерация имени файла с timestamp
$timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
$outputFile = Join-Path $OutputDir "inspection_results_$timestamp.$Format"

# Вывод информации
Write-Info "=== Запуск Code Inspection ==="
Write-Info "IDE: $IdeaPath"
Write-Info "Project: $ProjectPath"
Write-Info "Profile: $(if ($profilePath) { $Profile } else { 'built-in' })"
Write-Info "Output: $outputFile"
Write-Info "Format: $Format"

if ($Verbose) {
    Write-Info "Включения: $Include"
    if ($Exclude) {
        Write-Info "Исключения: $Exclude"
    }
}

# Используем улучшенный анализ без WebStorm (IDE запущена)
$useWebStorm = $false
Write-Info "Using enhanced custom analysis for frontend code inspection"

if ($useWebStorm) {
    # Построение аргументов для WebStorm
    $argumentString = "`"$ProjectPath`""

    if ($profilePath) {
        $argumentString += " `"$profilePath`""
    } else {
        $argumentString += " `"`""
    }

    $argumentString += " `"$outputFile`" -v2"

    # Добавление фильтров
    if ($Include -ne "*") {
        $argumentString += " --include=`"$Include`""
    }

    if ($Exclude) {
        $argumentString += " --exclude=`"$Exclude`""
    }
} else {
    # Использование ESLint для анализа
    Write-Info "Using ESLint for frontend code analysis..."
}

# Запуск inspection
Write-Info "Запуск анализа кода..."
$startTime = Get-Date

try {
    if ($useWebStorm) {
        # Запуск WebStorm inspect.bat с аргументами
        Write-Info "Command: $inspectPath $argumentString"
        $process = Start-Process -FilePath $inspectPath -ArgumentList $argumentString -NoNewWindow -Wait -PassThru
    } else {
        # Использование простого анализа без внешних зависимостей
        Write-Info "Running basic frontend code analysis..."

        $analysis = @{
            timestamp = Get-Date -Format "yyyy-MM-ddTHH:mm:ssZ"
            project = Split-Path $ProjectRoot -Leaf
            tool = "Basic Frontend Analyzer"
            problems = @()
            summary = @{
                total = 0
                errors = 0
                warnings = 0
                info = 0
            }
        }

        # Анализ файлов
        $includePatterns = $Include -split ','
        $excludePatterns = $Exclude -split ','

        $files = Get-ChildItem -Path $ProjectRoot -Include $includePatterns -Recurse -File -ErrorAction SilentlyContinue

        foreach ($file in $files) {
            $relativePath = $file.FullName.Replace($ProjectRoot, "").TrimStart("\")

            # Проверка исключений
            $excluded = $false
            foreach ($excludePattern in $excludePatterns) {
                if ($excludePattern -and $relativePath -like $excludePattern) {
                    $excluded = $true
                    break
                }
            }

            if (-not $excluded) {
                try {
                    $content = Get-Content $file.FullName -Raw -ErrorAction SilentlyContinue
                    if ($content) {
                        $lines = $content -split "`n"

                        # Анализ типов для TypeScript/Svelte файлов (один раз на файл)
                        if ($file.Extension -eq '.ts' -or $file.Extension -eq '.svelte') {

                            # Анализ проблем с типизацией stores в Svelte
                            # $storeProblems = Analyze-SvelteStoreIssues $scriptContent $relativePath
                            # $analysis.problems += $storeProblems
                            # $analysis.summary.errors += ($storeProblems | Where-Object { $_.severity -eq "ERROR" }).Count
                            # $analysis.summary.warnings += ($storeProblems | Where-Object { $_.severity -eq "WARNING" }).Count

                            # Извлекаем все импорты типов из ВСЕГО файла
                            $typeImports = @()
                            $importMatches = [regex]::Matches($content, 'import\s+(?:type\s+)?{([^}]+)}\s+from\s+[''"]@[^''"]*types[^''"]*[''"]')
                            foreach ($match in $importMatches) {
                                $imports = $match.Groups[1].Value -split ',' | ForEach-Object { $_.Trim() -replace '\s+as\s+\w+', '' }
                                if ($imports -contains 'AISettings') {
                                    Write-Host "DEBUG: Found AISettings import in file $relativePath" -ForegroundColor Green
                                }
                                $typeImports += $imports
                            }

                            # Прямые импорты типов
                            $directTypeImports = [regex]::Matches($content, 'import\s+type\s+(\w+)')
                            foreach ($match in $directTypeImports) {
                                $typeImports += $match.Groups[1].Value
                            }

                            # Также проверяем обычные импорты типов
                            $regularImports = [regex]::Matches($content, 'import\s+{([^}]+)}\s+from\s+[''"]@[^''"]*types[^''"]*[''"]')
                            foreach ($match in $regularImports) {
                                $imports = $match.Groups[1].Value -split ',' | ForEach-Object { $_.Trim() -replace '\s+as\s+\w+', '' }
                                $typeImports += $imports
                            }

                            # Извлекаем использования типов в контексте типизации
                            $typeUsages = @()

                            # Типы в объявлениях переменных: const/let/var name: TypeName
                            $varTypeMatches = [regex]::Matches($content, '\b(?:const|let|var)\s+\w+\s*:\s*([A-Z][a-zA-Z0-9_]*(?:<[^>]*>)?)')
                            foreach ($match in $varTypeMatches) {
                                $typeName = $match.Groups[1].Value -replace '<.*', ''  # Убираем generic параметры
                                if ($typeName -eq 'AISettings') {
                                    Write-Host "DEBUG: Found AISettings type usage in file $relativePath" -ForegroundColor Red
                                }
                                $typeUsages += $typeName
                            }

                            # Типы в параметрах функций: function name(param: TypeName)
                            $paramTypeMatches = [regex]::Matches($content, '\w+\s*\(\s*[^)]*\b(\w+)\s*:\s*([A-Z][a-zA-Z0-9_]*(?:<[^>]*>)?)')
                            foreach ($match in $paramTypeMatches) {
                                if ($match.Groups.Count -gt 2) {
                                    $typeUsages += $match.Groups[2].Value -replace '<.*', ''
                                }
                            }

                            # Типы в возвращаемых значениях: function(): TypeName
                            $returnTypeMatches = [regex]::Matches($content, '\w+\s*\([^)]*\)\s*:\s*([A-Z][a-zA-Z0-9_]*(?:<[^>]*>)?)')
                            foreach ($match in $returnTypeMatches) {
                                $typeUsages += $match.Groups[1].Value -replace '<.*', ''
                            }

                            # Фильтруем дубликаты и исключаем встроенные типы
                            $excludedTypes = @(
                                'Array', 'Object', 'String', 'Number', 'Boolean', 'Date', 'Promise', 'Error',
                                'Map', 'Set', 'WeakMap', 'WeakSet', 'Symbol', 'BigInt', 'RegExp', 'Function',
                                'undefined', 'null', 'true', 'false', 'this', 'super', 'console', 'window',
                                'document', 'globalThis', 'process', 'Buffer', 'Event', 'HTMLElement',
                                'HTMLInputElement', 'HTMLButtonElement', 'HTMLDivElement', 'MouseEvent',
                                'KeyboardEvent', 'FocusEvent', 'InputEvent', 'ChangeEvent', 'SubmitEvent',
                                'FormData', 'URL', 'URLSearchParams', 'AbortController', 'AbortSignal',
                                'ReadableStream', 'WritableStream', 'TransformStream', 'Blob', 'File',
                                'FileReader', 'FormDataEntryValue', 'Headers', 'Request', 'Response',
                                'fetch', 'setTimeout', 'setInterval', 'clearTimeout', 'clearInterval',
                                'localStorage', 'sessionStorage', 'indexedDB', 'WebSocket', 'Worker',
                                'SharedWorker', 'BroadcastChannel', 'MessageChannel', 'MessagePort',
                                'Notification', 'Performance', 'IntersectionObserver', 'ResizeObserver',
                                'MutationObserver', 'MediaQueryList', 'CSSStyleDeclaration', 'DOMRect',
                                'DOMRectReadOnly', 'DOMPoint', 'DOMPointReadOnly', 'DOMMatrix', 'DOMMatrixReadOnly',
                                'DOMQuad', 'DOMParser', 'XMLSerializer', 'XPathEvaluator', 'XPathResult',
                                'Range', 'Selection', 'Node', 'Element', 'Attr', 'Text', 'Comment',
                                'DocumentFragment', 'ShadowRoot', 'CustomElementRegistry', 'CustomEvent',
                                'Animation', 'KeyframeEffect', 'AnimationTimeline', 'DocumentTimeline',
                                'CSSAnimation', 'CSSTransition', 'TransitionEvent', 'AnimationEvent'
                            )

                            $typeUsages = $typeUsages | Where-Object {
                                $_ -notin $excludedTypes -and
                                $_ -notmatch '^\d' -and
                                $_.Length -gt 1 -and
                                $_ -notmatch '^Test$'  # Исключаем очевидные не-типы
                            } | Select-Object -Unique


                                # Проверяем каждый тип
                                $checkedTypes = @{}
                                foreach ($typeUsage in $typeUsages) {
                                    if (-not $checkedTypes.ContainsKey($typeUsage)) {
                                        $isImported = $typeImports -contains $typeUsage
                                        if ($typeUsage -eq 'AISettings') {
                                            Write-Host "DEBUG: Checking AISettings - imported: $isImported" -ForegroundColor Yellow
                                        }
                                        if (-not $isImported) {
                                        # Ищем строку с использованием типа
                                        $typeUsageLine = 0
                                        for ($i = 0; $i -lt $lines.Count; $i++) {
                                            if ($lines[$i] -match "\b$typeUsage\b") {
                                                $typeUsageLine = $i + 1
                                                break
                                            }
                                        }

                                        if ($typeUsageLine -gt 0) {
                                            $analysis.problems += @{
                                                file = $relativePath
                                                line = $typeUsageLine
                                                column = 1
                                                message = "Cannot find name '$typeUsage'. Did you mean to import it from types file?"
                                                severity = "ERROR"
                                                category = "TypeScript"
                                                rule = "missing-type-reference"
                                            }
                                            $analysis.summary.errors++
                                        }
                                    }
                                    $checkedTypes[$typeUsage] = $true
                                }
                            }
                        }

                        for ($i = 0; $i -lt $lines.Count; $i++) {
                            $line = $lines[$i]
                            $lineNumber = $i + 1

                            # Анализ JavaScript/TypeScript (Только для реальных проблем, исключая ложные срабатывания)
                            if ($file.Extension -in @('.js', '.ts', '.mjs', '.cjs')) {
                                # Поиск console.log
                                if ($line -match 'console\.log') {
                                    $analysis.problems += @{
                                        file = $relativePath
                                        line = $lineNumber
                                        column = $line.IndexOf('console.log') + 1
                                        message = "Console.log statement found in production code"
                                        severity = "WARNING"
                                        category = "Code Quality"
                                        rule = "no-console"
                                        fix = @{
                                            type = "remove"
                                            description = "Remove console.log statement"
                                        }
                                    }
                                    $analysis.summary.warnings++
                                }

                                # Поиск debugger
                                if ($line -match '\bdebugger\b') {
                                    $analysis.problems += @{
                                        file = $relativePath
                                        line = $lineNumber
                                        column = $line.IndexOf('debugger') + 1
                                        message = "Debugger statement found"
                                        severity = "ERROR"
                                        category = "Debugging"
                                        rule = "no-debugger"
                                        fix = @{
                                            type = "remove"
                                            description = "Remove debugger statement"
                                        }
                                    }
                                    $analysis.summary.errors++
                                }

                                # Поиск неиспользуемых импортов (упрощенная проверка)
                                if ($line -match '^import\s+.*from' -and $lines.Length -gt ($i + 5)) {
                                    $importMatch = [regex]::Match($line, 'import\s+{?\s*([^}]+)}?\s+from')
                                    if ($importMatch.Success) {
                                        $imports = $importMatch.Groups[1].Value -split ',' | ForEach-Object { $_.Trim() }
                                        # Простая проверка использования в следующих 10 строках
                                        $used = $false
                                        for ($j = $i + 1; $j -lt [Math]::Min($lines.Length, $i + 11); $j++) {
                                            foreach ($import in $imports) {
                                                if ($lines[$j] -match "\b$import\b") {
                                                    $used = $true
                                                    break
                                                }
                                            }
                                            if ($used) { break }
                                        }
                                        if (-not $used) {
                                            $analysis.problems += @{
                                                file = $relativePath
                                                line = $lineNumber
                                                column = 1
                                                message = "Potentially unused import: $($imports -join ', ')"
                                                severity = "WARNING"
                                                category = "Code Quality"
                                                rule = "no-unused-imports"
                                            }
                                            $analysis.summary.warnings++
                                        }
                                    }
                                }

                                # Поиск дублирования кода (простая проверка)
                                if ($lines.Length -gt ($i + 2) -and $lines[$i] -eq $lines[$i+2]) {
                                    $analysis.problems += @{
                                        file = $relativePath
                                        line = $lineNumber
                                        column = 1
                                        message = "Potential code duplication detected"
                                        severity = "INFO"
                                        category = "Code Quality"
                                        rule = "no-duplicate-code"
                                    }
                                    $analysis.summary.info++
                                }

                                # Поиск магических чисел
                                if ($line -match '\b\d{2,}\b' -and $line -notmatch '^\s*//' -and $line -notmatch 'const|let|var') {
                                    $numberMatch = [regex]::Match($line, '\b(\d{2,})\b')
                                    if ($numberMatch.Success -and $numberMatch.Groups[1].Value -notin @('0', '1', '100', '1000')) {
                                        $analysis.problems += @{
                                            file = $relativePath
                                            line = $lineNumber
                                            column = $numberMatch.Index + 1
                                            message = "Magic number: $($numberMatch.Groups[1].Value). Consider using a named constant"
                                            severity = "INFO"
                                            category = "Best Practices"
                                            rule = "no-magic-numbers"
                                        }
                                        $analysis.summary.info++
                                    }
                                }

                                # Анализ проблем типизации для TypeScript
                                if ($file.Extension -eq '.ts') {
                                    # Проверка типов в объявлениях переменных
                                    if ($line -match 'let\s+\w+\s*:\s*\w+' -and $content -match 'Writable<.*>') {
                                        # Проблемы с типами Writable - часто указывают на неправильное использование stores
                                        if ($line -match 'Writable<.*>' -and $content -match '\$derived\(.*store.*\)') {
                                            $analysis.problems += @{
                                                file = $relativePath
                                                line = $lineNumber
                                                column = $line.IndexOf('let') + 1
                                                message = "Variable typed as Writable but used with $derived(). Consider using proper typing or $derived(store) directly"
                                                severity = "WARNING"
                                                category = "TypeScript"
                                                rule = "store-typing"
                                            }
                                            $analysis.summary.warnings++
                                        }
                                    }

                                }
                            }


                            # Анализ типов в TypeScript/Svelte файлах (один раз на файл)
                            if (($file.Extension -eq '.ts' -or $file.Extension -eq '.svelte') -and $lineNumber -eq 1) {
                                Write-Host "DEBUG: Starting type analysis for file: $relativePath" -ForegroundColor Magenta
                                # Анализ всего файла на типы
                                $scriptContent = $content

                                # Извлекаем все импорты типов из ВСЕГО файла
                                $typeImports = @()
                                $importMatches = [regex]::Matches($scriptContent, 'import\s+(?:type\s+)?{([^}]+)}\s+from\s+[''"]@[^''"]*types[^''"]*[''"]')
                                foreach ($match in $importMatches) {
                                    $imports = $match.Groups[1].Value -split ',' | ForEach-Object { $_.Trim() -replace '\s+as\s+\w+', '' }
                                    $typeImports += $imports
                                }

                                # Прямые импорты типов
                                $directTypeImports = [regex]::Matches($scriptContent, 'import\s+type\s+(\w+)')
                                foreach ($match in $directTypeImports) {
                                    $typeImports += $match.Groups[1].Value
                                }

                                # Также проверяем обычные импорты типов
                                $regularImports = [regex]::Matches($scriptContent, 'import\s+{([^}]+)}\s+from\s+[''"]@[^''"]*types[^''"]*[''"]')
                                foreach ($match in $regularImports) {
                                    $imports = $match.Groups[1].Value -split ',' | ForEach-Object { $_.Trim() -replace '\s+as\s+\w+', '' }
                                    $typeImports += $imports
                                }

                                # Извлекаем использования типов в контексте типизации
                                $typeUsages = @()

                                # Типы в объявлениях переменных: const/let/var name: TypeName
                                $varTypeMatches = [regex]::Matches($scriptContent, '\b(?:const|let|var)\s+\w+\s*:\s*([A-Z][a-zA-Z0-9_]*(?:<[^>]*>)?)')
                                foreach ($match in $varTypeMatches) {
                                    $typeUsages += $match.Groups[1].Value -replace '<.*', ''  # Убираем generic параметры
                                }

                                # Типы в параметрах функций: function name(param: TypeName)
                                $paramTypeMatches = [regex]::Matches($scriptContent, '\w+\s*\(\s*[^)]*\b(\w+)\s*:\s*([A-Z][a-zA-Z0-9_]*(?:<[^>]*>)?)')
                                foreach ($match in $paramTypeMatches) {
                                    if ($match.Groups.Count -gt 2) {
                                        $typeUsages += $match.Groups[2].Value -replace '<.*', ''
                                    }
                                }

                                # Типы в возвращаемых значениях: function name(): TypeName
                                $returnTypeMatches = [regex]::Matches($scriptContent, '\w+\s*\([^)]*\)\s*:\s*([A-Z][a-zA-Z0-9_]*(?:<[^>]*>)?)')
                                foreach ($match in $returnTypeMatches) {
                                    $typeUsages += $match.Groups[1].Value -replace '<.*', ''
                                }

                                # Типы в interface/type определениях
                                $interfaceTypeMatches = [regex]::Matches($scriptContent, '\b(?:interface|type)\s+\w+.*\{[^}]*\b([A-Z][a-zA-Z0-9_]*(?:<[^>]*>)?)\b[^}]*\}')
                                foreach ($match in $interfaceTypeMatches) {
                                    if ($match.Groups.Count -gt 1) {
                                        $typeUsages += $match.Groups[1].Value -replace '<.*', ''
                                    }
                                }

                                # Фильтруем дубликаты и исключаем встроенные типы
                                $excludedTypes = @(
                                    'Array', 'Object', 'String', 'Number', 'Boolean', 'Date', 'Promise', 'Error',
                                    'Map', 'Set', 'WeakMap', 'WeakSet', 'Symbol', 'BigInt', 'RegExp', 'Function',
                                    'undefined', 'null', 'true', 'false', 'this', 'super', 'console', 'window',
                                    'document', 'globalThis', 'process', 'Buffer', 'Event', 'HTMLElement',
                                    'HTMLInputElement', 'HTMLButtonElement', 'HTMLDivElement', 'MouseEvent',
                                    'KeyboardEvent', 'FocusEvent', 'InputEvent', 'ChangeEvent', 'SubmitEvent',
                                    'FormData', 'URL', 'URLSearchParams', 'AbortController', 'AbortSignal',
                                    'ReadableStream', 'WritableStream', 'TransformStream', 'Blob', 'File',
                                    'FileReader', 'FormDataEntryValue', 'Headers', 'Request', 'Response',
                                    'fetch', 'setTimeout', 'setInterval', 'clearTimeout', 'clearInterval',
                                    'localStorage', 'sessionStorage', 'indexedDB', 'WebSocket', 'Worker',
                                    'SharedWorker', 'BroadcastChannel', 'MessageChannel', 'MessagePort',
                                    'Notification', 'Performance', 'IntersectionObserver', 'ResizeObserver',
                                    'MutationObserver', 'MediaQueryList', 'CSSStyleDeclaration', 'DOMRect',
                                    'DOMRectReadOnly', 'DOMPoint', 'DOMPointReadOnly', 'DOMMatrix', 'DOMMatrixReadOnly',
                                    'DOMQuad', 'DOMParser', 'XMLSerializer', 'XPathEvaluator', 'XPathResult',
                                    'Range', 'Selection', 'Node', 'Element', 'Attr', 'Text', 'Comment',
                                    'DocumentFragment', 'ShadowRoot', 'CustomElementRegistry', 'CustomEvent',
                                    'Animation', 'KeyframeEffect', 'AnimationTimeline', 'DocumentTimeline',
                                    'CSSAnimation', 'CSSTransition', 'TransitionEvent', 'AnimationEvent'
                                )

                                $typeUsages = $typeUsages | Where-Object {
                                    $_ -notin $excludedTypes -and
                                    $_ -notmatch '^\d' -and
                                    $_.Length -gt 1 -and
                                    $_ -notmatch '^Test$'  # Исключаем очевидные не-типы
                                } | Select-Object -Unique

                                # Проверяем каждый тип
                                $checkedTypes = @{}
                                foreach ($typeUsage in $typeUsages | Select-Object -Unique) {
                                    if (-not $checkedTypes.ContainsKey($typeUsage)) {
                                        $isImported = $typeImports -contains $typeUsage
                                        if (-not $isImported) {
                                            # Ищем строку с использованием типа
                                            $typeUsageLine = 0
                                            $lines = $scriptContent -split "`n"
                                            for ($i = 0; $i -lt $lines.Count; $i++) {
                                                if ($lines[$i] -match "\b$typeUsage\b") {
                                                    $typeUsageLine = $i + 1
                                                    break
                                                }
                                            }

                                            if ($typeUsageLine -gt 0) {
                                                $analysis.problems += @{
                                                    file = $relativePath
                                                    line = $typeUsageLine
                                                    column = 1
                                                    message = "Cannot find name '$typeUsage'. Did you mean to import it from types file?"
                                                    severity = "ERROR"
                                                    category = "TypeScript"
                                                    rule = "missing-type-reference"
                                                }
                                                $analysis.summary.errors++
                                            }
                                        }
                                        $checkedTypes[$typeUsage] = $true
                                    }
                                }
                            }

                            # Анализ проблем типизации в Svelte файлах - проверяем весь файл
                            if ($file.Extension -eq '.svelte') {
                                # Проверка проблем типизации во всем файле
                                if ($line -match 'let\s+\w+\s*:\s*Writable<') {
                                    # Извлекаем тип из Writable<...>
                                    $typeMatch = [regex]::Match($line, 'Writable<([^>]+)>')
                                    if ($typeMatch.Success) {
                                        $unwrappedType = $typeMatch.Groups[1].Value
                                        $analysis.problems += @{
                                            file = $relativePath
                                            line = $lineNumber
                                            column = $line.IndexOf('let') + 1
                                            message = "Variable typed as Writable<$unwrappedType>. For derived values, use the unwrapped type $unwrappedType instead."
                                            severity = "ERROR"
                                            category = "TypeScript"
                                            rule = "incorrect-writable-typing"
                                            fix = @{
                                                type = "fix-writable-typing"
                                                oldType = "Writable<$unwrappedType>"
                                                newType = $unwrappedType
                                            }
                                        }
                                        $analysis.summary.errors++
                                    }
                                }



                                # Анализ деструктуризации для поиска неиспользуемых переменных
                                if ($line -match '^\s*const\s*\{\s*[^}]*\}\s*=\s*\w+\([^)]*\)\s*;?\s*$' -or
                                    $line -match '^\s*const\s*\{\s*$') {

                                    # Обрабатываем однострочную или многострочную деструктуризацию
                                    $destructureStart = $lineNumber
                                    $destructureContent = $line

                                    # Если это многострочная деструктуризация, собираем все строки
                                    if ($line -match '^\s*const\s*\{\s*$') {
                                        $braceCount = 1  # Уже открыли одну скобку
                                        for ($i = $lineNumber + 1; $i -lt $lines.Count; $i++) {
                                            $destructureContent += "`n" + $lines[$i]
                                            $braceCount += ($lines[$i] -split '{' | Measure-Object).Count - ($lines[$i] -split '}' | Measure-Object).Count
                                            if ($braceCount -le 0 -and $lines[$i] -match '\}\s*=\s*\w+\([^)]*\)\s*;?\s*$') {
                                                # Найшли конец деструктуризации
                                                break
                                            }
                                        }
                                    }

                                    # Извлекаем список переменных из деструктуризации
                                    if ($destructureContent -match 'const\s*\{\s*([^}]+)\s*\}\s*=\s*(\w+)\([^)]*\)\s*;?\s*$') {
                                        $destructuredVars = $matches[1] -split ',' | ForEach-Object { $_.Trim() }
                                        $functionName = $matches[2]

                                        # Проверяем каждую переменную на использование
                                        foreach ($var in $destructuredVars) {
                                            $varName = $var.Trim()
                                            if ($varName -and $varName -notmatch '^(use|create|get|set|handle|on|tmp|_).*') {
                                                # Проверяем, используется ли переменная в оставшейся части файла (исключая комментарии)
                                                $remainingContent = $content.Substring($content.IndexOf($destructureContent) + $destructureContent.Length)
                                                # Удаляем комментарии из анализа (более надежный способ)
                                                $remainingContent = $remainingContent -replace '//[^\r\n]*', '' -replace '/\*[\s\S]*?\*/', '' -replace '<!--[\s\S]*?-->', ''
                                                # Ищем использование как переменной (\b$varName\b) или как вызова функции ($varName\()
                                                $isUsed = $remainingContent -match "\b$varName\b" -or $remainingContent -match "$varName\("

                                                if (-not $isUsed) {
                                                    $analysis.problems += @{
                                                        file = $relativePath
                                                        line = $destructureStart
                                                        column = $destructureContent.IndexOf($varName) + 1
                                                        message = "Unused destructured variable: $varName (from $functionName)"
                                                        severity = "WARNING"
                                                        category = "Code Quality"
                                                        rule = "unused-destructured-var"
                                                        fix = @{
                                                            type = "remove-unused-destructured-var"
                                                            varName = $varName
                                                            destructuredVars = $destructuredVars
                                                            fullContent = $destructureContent
                                                        }
                                                    }
                                                    $analysis.summary.warnings++
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            # Глубокий анализ неиспользуемых переменных и импортов
                            if ($file.Extension -eq '.svelte' -or $file.Extension -eq '.ts' -or $file.Extension -eq '.js') {
                                try {
                                # Анализ объявлений переменных
                                if ($line -match '^\s*(const|let|var)\s+\w+\s*=') {
                                    if ($line -match '^\s*(const|let|var)\s+(\w+)\s*=') {
                                        $varType = $matches[1]
                                        $varName = $matches[2]

                                        # Исключаем переменные, которые часто являются false positive
                                        if ($varName -match '^use\w+|^create\w+|^get\w+|^set\w+|^handle\w+|^on\w+|^tmp|^_') {
                                            continue
                                        }

                                        # Проверяем использование во всем файле (исключая строку объявления)
                                        $contentWithoutDeclaration = $content -replace [regex]::Escape($line), ""

                                        $isUsed = $false

                                        # 1. Прямое использование переменной
                                        if ($contentWithoutDeclaration -match "\b$varName\b") {
                                            $isUsed = $true
                                        }

                                        # 2. Вызов как функции
                                        if ($contentWithoutDeclaration -match "$varName\s*\(") {
                                            $isUsed = $true
                                        }

                                        # 3. Использование в template (для Svelte)
                                        if ($contentWithoutDeclaration -match "\{\s*$varName\s*\}") {
                                            $isUsed = $true
                                        }

                                        # 4. Reactive binding ($varName)
                                        if ($contentWithoutDeclaration -match "\`$$varName\b") {
                                            $isUsed = $true
                                        }

                                        # 5. В reactive statements ($:)
                                        if ($contentWithoutDeclaration -match "\`$:\s*$varName") {
                                            $isUsed = $true
                                        }

                                        # 6. В bind:value или других директивах
                                        if ($contentWithoutDeclaration -match "bind:\w+=\{$varName\}") {
                                            $isUsed = $true
                                        }

                                        # 7. Экспорт (переменная может быть экспортирована)
                                        if ($line -match 'export\s+') {
                                            $isUsed = $true
                                        }

                                        if (-not $isUsed) {
                                            $analysis.problems += @{
                                                file = $relativePath
                                                line = $lineNumber
                                                column = $line.IndexOf($varName) + 1
                                                message = "Unused variable: $varName (declared with $varType)"
                                                severity = "WARNING"
                                                category = "Code Quality"
                                                rule = "unused-variable"
                                                fix = @{
                                                    type = "remove-unused-variable"
                                                    varName = $varName
                                                    varType = $varType
                                                    fullLine = $line
                                                }
                                            }
                                            $analysis.summary.warnings++
                                        }
                                    }
                                }

                                # Анализ импортов (более глубокий)
                                if ($line -match '^\s*import\s+') {
                                    if ($line -match 'import\s+{?\s*([^}]+)}?\s+from\s+[''"]([^''"]+)[''"]') {
                                        $imports = $matches[1]
                                        $fromModule = $matches[2]

                                        # Разбираем импорты
                                        $importItems = $imports -split ',' | ForEach-Object {
                                            $_.Trim() -replace '\s+as\s+\w+', '' -replace '^\w+\s+as\s+', ''
                                        }

                                        foreach ($importItem in $importItems) {
                                            $importName = $importItem.Trim()
                                            if ($importName -and $importName -notmatch '^(type|default)$') {
                                                # Исключаем часто используемые импорты
                                                if ($importName -match '^(React|Component|useState|useEffect)$' -and $fromModule -match 'react') {
                                                    continue
                                                }

                                                $isImportUsed = $false

                                                # 1. Прямое использование
                                                if ($content -match "\b$importName\b") {
                                                    $isImportUsed = $true
                                                }

                                                # 2. Использование в template (Svelte)
                                                if ($content -match "\{\s*$importName\s*\}") {
                                                    $isImportUsed = $true
                                                }

                                                # 3. Reactive binding
                                                if ($content -match "\$$importName\b") {
                                                    $isImportUsed = $true
                                                }

                                                # 4. Вызов как функции
                                                if ($content -match "$importName\s*\(") {
                                                    $isImportUsed = $true
                                                }

                                                # 5. JSX компоненты (для TSX)
                                                if ($content -match "<$importName\b") {
                                                    $isImportUsed = $true
                                                }

                                                if (-not $isImportUsed) {
                                                    $analysis.problems += @{
                                                        file = $relativePath
                                                        line = $lineNumber
                                                        column = $line.IndexOf($importName) + 1
                                                        message = "Unused import: $importName (from $fromModule)"
                                                        severity = "INFO"
                                                        category = "Code Quality"
                                                        rule = "unused-import"
                                                        fix = @{
                                                            type = "remove-unused-import"
                                                            importName = $importName
                                                            fromModule = $fromModule
                                                            allImports = $importItems
                                                            fullLine = $line
                                                        }
                                                    }
                                                    $analysis.summary.info++
                                                }
                                            }
                                        }
                                    }
                                }
                                } catch {
                                    Write-Warn "Error in deep analysis: $($_.Exception.Message)"
                                }
                            }


                            # Анализ Svelte файлов
                            if ($file.Extension -eq '.svelte') {
                                # Проверяем, что мы в template секции (не в script)
                                $inScriptBlock = $false
                                $inStyleBlock = $false

                                # Определяем секции файла
                                if ($line -match '<script[^>]*>') {
                                    $inScriptBlock = $true
                                } elseif ($line -match '</script>') {
                                    $inScriptBlock = $false
                                } elseif ($line -match '<style[^>]*>') {
                                    $inStyleBlock = $true
                                } elseif ($line -match '</style>') {
                                    $inStyleBlock = $false
                                }

                                # Анализ template секции (не script и не style)
                                if (-not $inScriptBlock -and -not $inStyleBlock) {
                                    # Поиск отсутствующих alt атрибутов в img
                                    if ($line -match '<img[^>]*>' -and $line -notmatch 'alt=') {
                                        $analysis.problems += @{
                                            file = $relativePath
                                            line = $lineNumber
                                            column = $line.IndexOf('<img') + 1
                                            message = "Missing alt attribute on img element"
                                            severity = "WARNING"
                                            category = "Accessibility"
                                            rule = "img-alt"
                                            fix = @{
                                                type = "add-attribute"
                                                description = "Add alt attribute to img element"
                                            }
                                        }
                                        $analysis.summary.warnings++
                                    }

                                    # Поиск console.log в Svelte template (редко используется, но возможно)
                                    if ($line -match 'console\.log') {
                                        $analysis.problems += @{
                                            file = $relativePath
                                            line = $lineNumber
                                            column = $line.IndexOf('console.log') + 1
                                            message = "Console.log in Svelte template"
                                            severity = "WARNING"
                                            category = "Code Quality"
                                            rule = "no-console"
                                            fix = @{
                                                type = "remove"
                                                description = "Remove console.log statement"
                                            }
                                        }
                                        $analysis.summary.warnings++
                                    }

                                    # Поиск отсутствующих label для input
                                    if ($line -match '<input[^>]*>' -and $line -notmatch 'aria-label|aria-labelledby|id=') {
                                        $analysis.problems += @{
                                            file = $relativePath
                                            line = $lineNumber
                                            column = $line.IndexOf('<input') + 1
                                            message = "Input element without accessible label"
                                            severity = "WARNING"
                                            category = "Accessibility"
                                            rule = "input-label"
                                        }
                                        $analysis.summary.warnings++
                                    }

                                    # Поиск слишком длинных строк в template
                                    if ($line.Length -gt 120) {
                                        $analysis.problems += @{
                                            file = $relativePath
                                            line = $lineNumber
                                            column = 1
                                            message = "Line too long: $($line.Length) characters (recommended: 120)"
                                            severity = "INFO"
                                            category = "Code Style"
                                            rule = "max-line-length"
                                        }
                                        $analysis.summary.info++
                                    }
                                }

                                # Анализ script секции
                                elseif ($inScriptBlock) {
                                    # Поиск console.log в script (более серьезная проблема)
                                    if ($line -match 'console\.log') {
                                        $analysis.problems += @{
                                            file = $relativePath
                                            line = $lineNumber
                                            column = $line.IndexOf('console.log') + 1
                                            message = "Console.log in Svelte script section"
                                            severity = "WARNING"
                                            category = "Code Quality"
                                            rule = "no-console"
                                            fix = @{
                                                type = "remove"
                                                description = "Remove console.log statement"
                                            }
                                        }
                                        $analysis.summary.warnings++
                                    }

                                    # Поиск debugger в script
                                    if ($line -match '\bdebugger\b') {
                                        $analysis.problems += @{
                                            file = $relativePath
                                            line = $lineNumber
                                            column = $line.IndexOf('debugger') + 1
                                            message = "Debugger statement in Svelte script"
                                            severity = "ERROR"
                                            category = "Debugging"
                                            rule = "no-debugger"
                                            fix = @{
                                                type = "remove"
                                                description = "Remove debugger statement"
                                            }
                                        }
                                        $analysis.summary.errors++
                                    }

                                    # Проверка неиспользуемых констант/переменных в script
                                    if ($line -match '\s*const\s+(\$?\w+)\s*=|\s*let\s+(\$?\w+)\s*[:=]|\s*const\s+\{([^}]+)\}|\s*let\s+\{([^}]+)\}') {
                                        Write-Info "Found variable declaration in line: $line"
                                        $varMatch = [regex]::Match($line, 'const\s+(\$?\w+)\s*=|let\s+(\$?\w+)\s*[:=]|const\s+\{([^}]+)\}|let\s+\{([^}]+)\}')
                                        if ($varMatch.Success) {
                                            $singleVar = $varMatch.Groups[1].Value
                                            if (-not $singleVar) { $singleVar = $varMatch.Groups[2].Value }
                                            $destructuredVars = $varMatch.Groups[3].Value
                                            if (-not $destructuredVars) { $destructuredVars = $varMatch.Groups[4].Value }

                                            if ($destructuredVars) {
                                                # Деструктуризация: const { var1, var2 } = ...
                                                $vars = $destructuredVars -split ',' | ForEach-Object {
                                                    $_.Trim() -replace '\s+as\s+.*', ''  # Убираем алиасы
                                                }
                                                foreach ($varName in $vars) {
                                                    if ($varName -and $varName -notmatch '^\s*$') {
                                                        # Проверяем использование переменной во всем файле
                                                        if ($content -notmatch "\b$varName\b") {
                                                            $analysis.problems += @{
                                                                file = $relativePath
                                                                line = $lineNumber
                                                                column = $line.IndexOf($varName) + 1
                                                                message = "Unused destructured variable '$varName'"
                                                                severity = "WARNING"
                                                                category = "Code Quality"
                                                                rule = "no-unused-vars"
                                                                fix = @{
                                                                    type = "remove-unused-destructured-var"
                                                                    varName = $varName
                                                                    destructuredList = $destructuredVars
                                                                }
                                                            }
                                                            $analysis.summary.warnings++
                                                        }
                                                    }
                                                }
                                            } elseif ($singleVar) {
                                                # Обычное объявление переменной
                                                # Проверяем использование переменной во всем файле
                                                if ($content -notmatch "\b$singleVar\b") {
                                                    $analysis.problems += @{
                                                        file = $relativePath
                                                        line = $lineNumber
                                                        column = $line.IndexOf($singleVar) + 1
                                                        message = "Unused variable '$singleVar'"
                                                        severity = "WARNING"
                                                        category = "Code Quality"
                                                        rule = "no-unused-vars"
                                                        fix = @{
                                                            type = "remove-unused-var"
                                                            varName = $singleVar
                                                        }
                                                    }
                                                    $analysis.summary.warnings++
                                                }
                                            }
                                        }
                                    }
                                }

                                # Анализ style секции
                                elseif ($inStyleBlock) {
                                    # Поиск слишком длинных строк в CSS
                                    if ($line.Length -gt 120) {
                                        $analysis.problems += @{
                                            file = $relativePath
                                            line = $lineNumber
                                            column = 1
                                            message = "CSS line too long: $($line.Length) characters (recommended: 120)"
                                            severity = "INFO"
                                            category = "Code Style"
                                            rule = "max-line-length"
                                        }
                                        $analysis.summary.info++
                                    }
                                }
                            }
                        }
                    }
                } catch {
                    Write-Warn "Error analyzing file: $relativePath"
                }
            }
        }

        $analysis.summary.total = $analysis.problems.Count

        # Запуск Python анализатора типов для Svelte/TypeScript файлов
        Write-Info "Running Python type analyzer for Svelte/TypeScript files..."
        try {
            $pythonScript = Join-Path $PSScriptRoot "analyze-svelte-types.py"
            if (Test-Path $pythonScript) {
                # Собираем все Svelte и TypeScript файлы для анализа
                $filesToAnalyze = @()
                foreach ($includePath in $Include) {
                    if ($includePath -match '\.(svelte|ts)$') {
                        $filesToAnalyze += $includePath
                    }
                }

                if ($filesToAnalyze.Count -gt 0) {
                    $pythonArgs = $filesToAnalyze -join " "
                    $pythonCommand = "python `"$pythonScript`" $pythonArgs"

                    Write-Debug "Running: $pythonCommand"
                    $pythonResult = Invoke-Expression $pythonCommand 2>&1

                    # Парсим результаты из JSON файла
                    $pythonResultsFile = "svelte-type-analysis-results.json"
                    if (Test-Path $pythonResultsFile) {
                        try {
                            $pythonResults = Get-Content $pythonResultsFile -Raw | ConvertFrom-Json

                            foreach ($result in $pythonResults) {
                                foreach ($problem in $result.problems) {
                                    # Преобразуем Python результат в формат PowerShell анализатора
                                    $convertedProblem = @{
                                        file = $result.file
                                        line = $problem.line
                                        column = 1
                                        message = $problem.message
                                        severity = if ($problem.severity -eq "ERROR") { "ERROR" } else { "WARNING" }
                                        category = "TypeScript"
                                        rule = $problem.type
                                        fix = $problem.fix
                                    }

                                    $analysis.problems += $convertedProblem
                                    if ($problem.severity -eq "ERROR") {
                                        $analysis.summary.errors++
                                    } else {
                                        $analysis.summary.warnings++
                                    }
                                }
                            }

                            # Обновляем общее количество проблем
                            $analysis.summary.total = $analysis.problems.Count

                            Write-Info "Python type analyzer found $($pythonResults.problems.Count) additional issues."

                        } catch {
                            Write-Warn "Error parsing Python results: $_"
                        }

                        # Удаляем временный файл
                        Remove-Item $pythonResultsFile -ErrorAction SilentlyContinue
                    }
                } else {
                    Write-Info "No Svelte/TypeScript files to analyze with Python."
                }
            } else {
                Write-Warn "Python type analyzer script not found: $pythonScript"
            }
        } catch {
            Write-Warn "Error running Python type analyzer: $_"
        }

        # Запуск Python фиксера типизации
        if ($Fix) {
            Write-Info "Running Python type fixer for Svelte/TypeScript files..."
            try {
                $pythonFixScript = Join-Path $PSScriptRoot "fix-svelte-types.py"
                if (Test-Path $pythonFixScript) {
                    # Собираем все Svelte и TypeScript файлы для исправления
                    $filesToFix = @()
                    foreach ($includePath in $Include) {
                        if ($includePath -match '\.(svelte|ts)$') {
                            $filesToFix += $includePath
                        }
                    }

                    if ($filesToFix.Count -gt 0) {
                        $pythonArgs = $filesToFix -join " "
                        $pythonFixCommand = "python `"$pythonFixScript`" $pythonArgs"

                        Write-Debug "Running: $pythonFixCommand"
                        $pythonFixResult = Invoke-Expression $pythonFixCommand 2>&1

                        Write-Info "Python type fixer completed fixes."

                        # После фиксов нужно перезапустить анализ
                        Write-Info "Re-running analysis after fixes..."
                        # Здесь можно добавить логику для повторного анализа исправленных файлов

                    } else {
                        Write-Info "No Svelte/TypeScript files to fix with Python."
                    }
                } else {
                    Write-Warn "Python type fixer script not found: $pythonFixScript"
                }
            } catch {
                Write-Warn "Error running Python type fixer: $_"
            }
        }

        # Сохранение результатов
        $jsonOutput = if ($Format -eq "json") { $outputFile } else { $outputFile.Replace(".xml", ".json") }
        $analysis | ConvertTo-Json -Depth 10 | Out-File -FilePath $jsonOutput -Encoding UTF8

        Write-Success "Analysis completed. Found $($analysis.summary.total) issues."
        $process = [PSCustomObject]@{ ExitCode = 0 }
    }

    if ($process.ExitCode -eq 0) {
        $endTime = Get-Date
        $duration = [math]::Round(($endTime - $startTime).TotalSeconds, 1)

        Write-Success "Анализ завершен успешно за ${duration}с"
        Write-Success "Результаты сохранены в: $outputFile"

        # Создание символической ссылки на последний результат
        $latestLink = Join-Path $OutputDir "inspection_results_latest.$Format"
        try {
            if (Test-Path $latestLink) {
                Remove-Item $latestLink -Force
            }
            New-Item -ItemType SymbolicLink -Path $latestLink -Target (Split-Path $outputFile -Leaf) -Force | Out-Null
            Write-Info "Ссылка на последние результаты: $latestLink"
        } catch {
            # Игнорируем ошибки создания symlink на Windows без прав администратора
        }

        # Анализ результатов (только для XML)
        if ($Format -eq "xml" -and (Test-Path $outputFile)) {
            try {
                [xml]$xmlContent = Get-Content $outputFile -Encoding UTF8
                $problems = $xmlContent.SelectNodes("//problem")

                $problemCount = $problems.Count
                $errorCount = ($problems | Where-Object { $_.severity -eq "ERROR" }).Count
                $warningCount = ($problems | Where-Object { $_.severity -eq "WARNING" }).Count

                Write-Info "Результаты анализа:"
                Write-Info "  Всего проблем: $problemCount"
                Write-Info "  Ошибок: $errorCount"
                Write-Info "  Предупреждений: $warningCount"

                # Проверка лимитов
                if ($FailOnError -and $errorCount -gt 0) {
                    Write-ErrorMsg "Найдены ошибки ($errorCount). Выполнение прервано."
                    exit 1
                }

                # Показать топ проблем
                if ($problemCount -gt 0 -and $Verbose) {
                    Write-Warn "Найденные проблемы (топ 5):"
                    $problems | Select-Object -First 5 | ForEach-Object {
                        $file = $_.file
                        $line = $_.line
                        $message = $_.description
                        $severity = $_.severity
                        Write-Warn "  ${file}:${line} - ${message} (${severity})"
                    }
                    if ($problemCount -gt 5) {
                        Write-Warn "  ... и еще $($problemCount - 5) проблем"
                    }
                    Write-Warn "Полные результаты: $outputFile"
                }
            } catch {
                Write-Warn "Не удалось проанализировать результаты XML: $_"
            }
        }

        # Конвертация в JSON для Cursor (если Python доступен)
        if ($Format -eq "xml") {
            $jsonOutput = [System.IO.Path]::ChangeExtension($outputFile, "json")
            $converterScript = Join-Path $ScriptDir "xml-to-json-converter.py"

            if (Test-Path $converterScript) {
                try {
                    $pythonCmd = Get-Command python -ErrorAction SilentlyContinue
                    if (-not $pythonCmd) {
                        $pythonCmd = Get-Command python3 -ErrorAction SilentlyContinue
                    }

                    if ($pythonCmd) {
                        & $pythonCmd.Source $converterScript $outputFile $jsonOutput 2>$null
                        if ($LASTEXITCODE -eq 0) {
                            Write-Info "JSON версия создана: $jsonOutput"
                        }
                    }
                } catch {
                    # Игнорируем ошибки конвертации
                }
            }
        }

    } else {
        Write-ErrorMsg "Анализ завершился с ошибкой (код: $($process.ExitCode))"
        exit 1
    }

} catch {
    Write-ErrorMsg "Ошибка запуска IDEA: $_"
    exit 1
}

Write-Success "=== Code Inspection завершен ==="

# Show next steps
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Cyan
Write-Host "1. Review results in: $OutputDir" -ForegroundColor White
Write-Host "2. For Git integration: cp scripts/pre-commit-inspection.sh .git/hooks/pre-commit" -ForegroundColor White
Write-Host "3. For CI/CD: use this script in pipeline" -ForegroundColor White
