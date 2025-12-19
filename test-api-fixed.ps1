# Test API through PowerShell
Write-Host "=== Test API through PowerShell ===" -ForegroundColor Green

$url = "https://localhost:9443"

try {
    # Ignore SSL checks
    [System.Net.ServicePointManager]::ServerCertificateValidationCallback = {$true}

    # Create request with correct Host header
    $request = [System.Net.WebRequest]::Create($url)
    $request.Method = "GET"
    $request.Timeout = 10000
    $request.Host = "api.erkapharm.com"

    # Add User-Agent like browser
    $request.UserAgent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"

    $response = $request.GetResponse()
    $reader = New-Object System.IO.StreamReader($response.GetResponseStream())
    $content = $reader.ReadToEnd()
    $reader.Close()
    $response.Close()

    Write-Host "API available!" -ForegroundColor Green
    Write-Host "Status: $($response.StatusCode)" -ForegroundColor Cyan
    Write-Host "Content-Type: $($response.ContentType)" -ForegroundColor Cyan

    # Check if it's Swagger UI
    if ($content -like "*swagger*") {
        Write-Host "Swagger UI received - API works!" -ForegroundColor Green
    } elseif ($content -like "*API*") {
        Write-Host "API response received" -ForegroundColor Green
    } else {
        Write-Host "Response received but not recognized" -ForegroundColor Yellow
    }

}
catch {
    Write-Host "API access error" -ForegroundColor Red
    Write-Host "Error: $($_.Exception.Message)" -ForegroundColor Yellow

    # Detailed diagnostics
    if ($_.Exception.Message -like "*SSL*" -or $_.Exception.Message -like "*certificate*") {
        Write-Host ""
        Write-Host "SSL certificate issue" -ForegroundColor Yellow
        Write-Host "Solution: api.erkapharm.com certificate not trusted for localhost" -ForegroundColor Cyan
    } elseif ($_.Exception.Message -like "*403*") {
        Write-Host ""
        Write-Host "403 Forbidden" -ForegroundColor Yellow
        Write-Host "Solution: API blocks requests without correct Host header" -ForegroundColor Cyan
    } elseif ($_.Exception.Message -like "*connection*") {
        Write-Host ""
        Write-Host "Network error" -ForegroundColor Yellow
        Write-Host "Solution: Check SSH tunnel works (port 9443)" -ForegroundColor Cyan
    }
}

Write-Host ""
Write-Host "=== COMPARISON ===" -ForegroundColor Green
Write-Host "From WSL (works): curl -k -H 'Host: api.erkapharm.com' https://localhost:9443" -ForegroundColor Cyan
Write-Host "From PowerShell: Needs SSL ignore and correct Host header" -ForegroundColor Cyan

Write-Host ""
Write-Host "For browser: Add api.erkapharm.com -> 127.0.0.1 to hosts" -ForegroundColor Green

