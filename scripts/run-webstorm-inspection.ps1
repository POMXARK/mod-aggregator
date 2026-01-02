# Script to run code inspection in WebStorm for frontend
# Clean PowerShell version for Windows

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
    [switch]$Help
)

# Functions
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

# Show help
if ($Help) {
    Write-Host @"
Script to run code inspection in WebStorm for frontend projects

Usage: .\run-webstorm-inspection.ps1 [OPTIONS]

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
    -Help                  Show this help

Examples:
    .\run-webstorm-inspection.ps1
    .\run-webstorm-inspection.ps1 -Verbose -FailOnError
    .\run-webstorm-inspection.ps1 -Include "*.ts" -Format html
"@
    exit 0
}

# Setup paths
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Split-Path -Parent $ScriptDir

if (-not $ProjectPath) {
    $ProjectPath = $ProjectRoot
}

if (-not $OutputDir) {
    $OutputDir = Join-Path $ProjectRoot "inspection-results"
}

# Create output directory
if (-not (Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null
}

# Find WebStorm
if (-not $IdeaPath) {
    $IdeaPath = $env:IDEA_PATH
}

if (-not $IdeaPath) {
    # Search for WebStorm
    $possiblePaths = @(
        "C:\Program Files\JetBrains\WebStorm*\bin\webstorm64.exe",
        "$env:USERPROFILE\AppData\Local\JetBrains\Toolbox\apps\WebStorm\*\bin\webstorm64.exe",
        "C:\Program Files\JetBrains\WebStorm 2025.2.3\bin\webstorm64.exe"
    )

    foreach ($pattern in $possiblePaths) {
        $found = Get-ChildItem -Path $pattern -ErrorAction SilentlyContinue | Select-Object -First 1
        if ($found) {
            $IdeaPath = $found.FullName
            break
        }
    }
}

# Check WebStorm
if (-not $IdeaPath -or -not (Test-Path $IdeaPath)) {
    Write-ErrorMsg "WebStorm not found at: $IdeaPath"
    Write-ErrorMsg "Please install WebStorm or set IDEA_PATH variable"
    Write-ErrorMsg "Download from: https://www.jetbrains.com/webstorm/download/"
    exit 1
}

# Check inspection profile
$profileDir = Join-Path $ProjectPath ".idea\inspectionProfiles"
$profilePath = Join-Path $profileDir "$Profile.xml"

if (-not (Test-Path $profilePath)) {
    Write-Warn "Inspection profile '$Profile' not found: $profilePath"
    Write-Warn "Will use built-in profile"
    $profilePath = ""
}

# Generate output filename
$timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
$outputFile = Join-Path $OutputDir "inspection_results_$timestamp.$Format"

# Display info
Write-Info "=== Starting WebStorm Code Inspection ==="
Write-Info "IDE: $IdeaPath"
Write-Info "Project: $ProjectPath"
Write-Info "Profile: $(if ($profilePath) { $Profile } else { 'built-in' })"
Write-Info "Output: $outputFile"
Write-Info "Format: $Format"

if ($Verbose) {
    Write-Info "Include: $Include"
    if ($Exclude) {
        Write-Info "Exclude: $Exclude"
    }
}

# Build command arguments
$args = @(
    "inspect",
    $ProjectPath
)

if ($profilePath) {
    $args += $profilePath
} else {
    $args += ""
}

$args += $outputFile
$args += "-v2"

# Add filters
if ($Include -ne "*.js,*.ts,*.svelte,*.html,*.css") {
    $args += "--include=$Include"
}

if ($Exclude) {
    $args += "--exclude=$Exclude"
}

# Run inspection
Write-Info "Starting code analysis..."
$startTime = Get-Date

try {
    $argumentString = $args -join " "
    Write-Info "Command: $IdeaPath $argumentString"
    $process = Start-Process -FilePath $IdeaPath -ArgumentList $argumentString -NoNewWindow -Wait -PassThru

    if ($process.ExitCode -eq 0) {
        $endTime = Get-Date
        $duration = [math]::Round(($endTime - $startTime).TotalSeconds, 1)

        Write-Success "Analysis completed successfully in ${duration}s"
        Write-Success "Results saved to: $outputFile"

        # Create latest symlink
        $latestLink = Join-Path $OutputDir "inspection_results_latest.$Format"
        try {
            if (Test-Path $latestLink) {
                Remove-Item $latestLink -Force
            }
            # Note: Windows doesn't support symbolic links without admin rights
            Copy-Item $outputFile $latestLink -Force
            Write-Info "Latest results copy: $latestLink"
        } catch {
            # Ignore symlink errors
        }

        # Analyze XML results
        if ($Format -eq "xml" -and (Test-Path $outputFile)) {
            try {
                [xml]$xmlContent = Get-Content $outputFile -Encoding UTF8
                $problems = $xmlContent.SelectNodes("//problem")

                $problemCount = $problems.Count
                $errorCount = ($problems | Where-Object { $_.severity -eq "ERROR" }).Count
                $warningCount = ($problems | Where-Object { $_.severity -eq "WARNING" }).Count

                Write-Info "Analysis results:"
                Write-Info "  Total problems: $problemCount"
                Write-Info "  Errors: $errorCount"
                Write-Info "  Warnings: $warningCount"

                # Check limits
                if ($FailOnError -and $errorCount -gt 0) {
                    Write-ErrorMsg "Found errors ($errorCount). Failing as requested."
                    exit 1
                }

                # Show top problems
                if ($problemCount -gt 0 -and $Verbose) {
                    Write-Warn "Top problems found (first 5):"
                    $problems | Select-Object -First 5 | ForEach-Object {
                        $file = $_.file
                        $line = $_.line
                        $message = $_.description
                        $severity = $_.severity
                        Write-Warn "  ${file}:${line} - ${message} (${severity})"
                    }
                    if ($problemCount -gt 5) {
                        Write-Warn "  ... and $($problemCount - 5) more problems"
                    }
                    Write-Warn "Full results: $outputFile"
                }
            } catch {
                Write-Warn "Could not analyze XML results: $_"
            }
        }

        # Convert to JSON for Cursor
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
                            Write-Info "JSON version created: $jsonOutput"
                        }
                    }
                } catch {
                    # Ignore conversion errors
                }
            }
        }

    } else {
        Write-ErrorMsg "Analysis failed with exit code: $($process.ExitCode)"
        exit 1
    }

} catch {
    Write-ErrorMsg "Error running WebStorm: $_"
    exit 1
}

Write-Success "=== WebStorm Code Inspection Completed ==="

# Next steps
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Cyan
Write-Host "1. Check results in: $OutputDir" -ForegroundColor White
Write-Host "2. For Git integration: copy scripts\pre-commit-inspection.sh .git\hooks\pre-commit" -ForegroundColor White
Write-Host "3. For CI/CD: use this script in pipeline" -ForegroundColor White
