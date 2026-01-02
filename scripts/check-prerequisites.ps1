# Скрипт для проверки предварительных требований для SpecKit (PowerShell)

Write-Host "Проверка предварительных требований для SpecKit..." -ForegroundColor Cyan

$errors = @()

# Проверка Node.js
try {
    $nodeVersion = node --version
    Write-Host "✅ Node.js установлен: $nodeVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ Node.js не установлен" -ForegroundColor Red
    $errors += "Node.js"
}

# Проверка npm
try {
    $npmVersion = npm --version
    Write-Host "✅ npm установлен: $npmVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ npm не установлен" -ForegroundColor Red
    $errors += "npm"
}

# Проверка Rust
try {
    $rustVersion = rustc --version
    Write-Host "✅ Rust установлен: $rustVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ Rust не установлен" -ForegroundColor Red
    $errors += "Rust"
}

# Проверка структуры директорий
$directories = @("specs", "memory", "templates")
foreach ($dir in $directories) {
    if (Test-Path $dir) {
        Write-Host "✅ Директория $dir/ существует" -ForegroundColor Green
    } else {
        Write-Host "❌ Директория $dir/ не найдена" -ForegroundColor Red
        $errors += "Директория $dir"
    }
}

# Проверка конституции
if (Test-Path "memory/constitution.md") {
    Write-Host "✅ Файл memory/constitution.md существует" -ForegroundColor Green
} else {
    Write-Host "❌ Файл memory/constitution.md не найден" -ForegroundColor Red
    $errors += "memory/constitution.md"
}

Write-Host ""

if ($errors.Count -eq 0) {
    Write-Host "✅ Все предварительные требования выполнены!" -ForegroundColor Green
    exit 0
} else {
    Write-Host "❌ Обнаружены ошибки:" -ForegroundColor Red
    foreach ($error in $errors) {
        Write-Host "  - $error" -ForegroundColor Red
    }
    exit 1
}



































