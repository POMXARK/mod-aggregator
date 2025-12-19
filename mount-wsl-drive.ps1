# Монтирование WSL как сетевой диск в Windows
param(
    [string]$DriveLetter = "W",
    [string]$DistroName = "Ubuntu"
)

Write-Host "=== Монтирование WSL как сетевой диск ===" -ForegroundColor Green

# Проверяем что WSL запущен
$wslStatus = wsl -l -v | Select-String $DistroName | Select-String "Running"
if (-not $wslStatus) {
    Write-Host "❌ WSL $DistroName не запущен" -ForegroundColor Red
    Write-Host "Запустите: wsl -d $DistroName" -ForegroundColor Yellow
    exit 1
}

Write-Host "✓ WSL $DistroName запущен" -ForegroundColor Green

# Проверяем доступность диска
$drivePath = $DriveLetter + ":"
$driveExists = Get-PSDrive -Name $DriveLetter -ErrorAction SilentlyContinue

if ($driveExists) {
    Write-Host "⚠️  Диск $DriveLetter уже существует" -ForegroundColor Yellow
    $replace = Read-Host "Заменить? (y/n)"
    if ($replace -eq "y") {
        net use $drivePath /delete 2>$null | Out-Null
    } else {
        exit 0
    }
}

# Создаем сетевой диск
Write-Host "Создаем сетевой диск $DriveLetter`: для WSL..." -ForegroundColor Cyan
$uncPath = "\\wsl$\$DistroName"

try {
    net use $drivePath $uncPath /persistent:yes 2>$null
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✓ Диск $DriveLetter`: успешно создан!" -ForegroundColor Green
        Write-Host "Путь: $uncPath" -ForegroundColor Cyan
    } else {
        Write-Host "❌ Ошибка создания диска $DriveLetter`:" -ForegroundColor Red
        Write-Host "Возможно, буква диска занята" -ForegroundColor Yellow
        exit 1
    }
} catch {
    Write-Host "❌ Ошибка: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host ""
Write-Host "=== ДОСТУП К ФАЙЛАМ ===" -ForegroundColor Green
Write-Host "$DriveLetter`: - корень WSL $DistroName" -ForegroundColor Cyan
Write-Host "$DriveLetter`:\mnt\c\Users\User\mod-aggregator - ваш проект" -ForegroundColor Cyan

Write-Host ""
Write-Host "=== УПРАВЛЕНИЕ ДИСКОМ ===" -ForegroundColor Yellow
Write-Host "Отключить: net use $DriveLetter`: /delete" -ForegroundColor Cyan
Write-Host "Все диски: net use" -ForegroundColor Cyan

Write-Host ""
Write-Host "Теперь можно открывать $DriveLetter`: в проводнике!" -ForegroundColor Green
