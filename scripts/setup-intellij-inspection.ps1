# WebStorm setup script for Windows

Write-Host "=== WebStorm Setup for Frontend Code Inspection ==="

# Find existing WebStorm installation
Write-Host "Searching for WebStorm..."

$ideaPaths = @(
    "C:\Program Files\JetBrains\WebStorm*\bin\webstorm64.exe",
    "$env:USERPROFILE\AppData\Local\JetBrains\Toolbox\apps\WebStorm\*\bin\webstorm64.exe",
    "C:\Program Files\JetBrains\WebStorm 2025.2.3\bin\webstorm64.exe"
)

$foundIdea = $null
foreach ($pattern in $ideaPaths) {
    $found = Get-ChildItem -Path $pattern -ErrorAction SilentlyContinue | Select-Object -First 1
    if ($found) {
        $foundIdea = $found.FullName
        break
    }
}

if ($foundIdea) {
    Write-Host "Found WebStorm: $foundIdea"

    # Set environment variable
    $env:IDEA_PATH = $foundIdea
    [Environment]::SetEnvironmentVariable("IDEA_PATH", $foundIdea, "User")
    Write-Host "IDEA_PATH environment variable set"
} else {
    Write-Host "WebStorm not found"
    Write-Host ""
    Write-Host "Please install WebStorm:"
    Write-Host "1. Download from: https://www.jetbrains.com/webstorm/download/"
    Write-Host "2. Install WebStorm"
    Write-Host "3. Run this script again"
    Write-Host ""
    Write-Host "Or set IDEA_PATH manually:"
    Write-Host "  `$env:IDEA_PATH = `"C:\Program Files\JetBrains\WebStorm 2025.2.3\bin\webstorm64.exe`""
    exit 1
}

# Create results directory
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$projectRoot = Split-Path -Parent $scriptDir
$resultsDir = Join-Path $projectRoot "inspection-results"

if (-not (Test-Path $resultsDir)) {
    New-Item -ItemType Directory -Path $resultsDir -Force | Out-Null
    Write-Host "Created results directory: $resultsDir"
}

# Check inspection profile
$profilePath = Join-Path $projectRoot ".idea\inspectionProfiles\Project_Default.xml"
if (Test-Path $profilePath) {
    Write-Host "Inspection profile found: $profilePath"
} else {
    Write-Host "Inspection profile not found (will be created automatically)"
}

# Test IDEA
Write-Host "Testing IDEA..."
if (Test-Path $env:IDEA_PATH) {
    Write-Host "IDEA executable found"
} else {
    Write-Host "IDEA executable not accessible"
}

Write-Host ""
Write-Host "=== Setup Complete! ==="
Write-Host ""
Write-Host "To run code inspection:"
Write-Host "  .\scripts\run-code-inspection.ps1"
Write-Host ""
Write-Host "Results will be saved in: inspection-results"
