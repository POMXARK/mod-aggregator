# PowerShell HTTP Proxy для API ErkaPharm
# Проксирует localhost:8080 -> localhost:9443 с правильным Host header

param(
    [int]$Port = 8080,
    [string]$TargetHost = "localhost",
    [int]$TargetPort = 9443,
    [string]$ApiHost = "api.erkapharm.com"
)

Write-Host "=== API Proxy Server ===" -ForegroundColor Green
Write-Host "Proxy: http://localhost:$Port -> https://$TargetHost:$TargetPort" -ForegroundColor Cyan
Write-Host "Host header: $ApiHost" -ForegroundColor Cyan
Write-Host "Press Ctrl+C to stop" -ForegroundColor Yellow
Write-Host ""

try {
    # Создаем HTTP listener
    $listener = New-Object System.Net.HttpListener
    $listener.Prefixes.Add("http://localhost:$Port/")

    $listener.Start()
    Write-Host "Proxy server listening on http://localhost:$Port" -ForegroundColor Green

    while ($listener.IsListening) {
        $context = $listener.GetContext()

        $request = $context.Request
        $response = $context.Response

        try {
            Write-Host "$(Get-Date -Format 'HH:mm:ss') - $($request.HttpMethod) $($request.RawUrl)" -ForegroundColor Gray

            # Создаем запрос к целевому серверу
            $webRequest = [System.Net.WebRequest]::Create("https://$TargetHost:$TargetPort$($request.RawUrl)")
            $webRequest.Method = $request.HttpMethod
            $webRequest.Host = $ApiHost

            # Игнорируем SSL сертификаты
            [System.Net.ServicePointManager]::ServerCertificateValidationCallback = {$true}

            # Копируем headers (кроме host)
            foreach ($header in $request.Headers.Keys) {
                if ($header -ne "Host" -and $header -ne "Connection" -and $header -ne "Keep-Alive" -and $header -ne "Proxy-Connection") {
                    try {
                        $webRequest.Headers.Add($header, $request.Headers[$header])
                    } catch {
                        # Некоторые headers нельзя добавить
                    }
                }
            }

            # Копируем тело запроса если есть
            if ($request.HasEntityBody) {
                $stream = $request.InputStream
                $reader = New-Object System.IO.StreamReader($stream)
                $body = $reader.ReadToEnd()
                $reader.Close()

                $bytes = [System.Text.Encoding]::UTF8.GetBytes($body)
                $webRequest.ContentLength = $bytes.Length
                $webRequest.ContentType = $request.ContentType

                $requestStream = $webRequest.GetRequestStream()
                $requestStream.Write($bytes, 0, $bytes.Length)
                $requestStream.Close()
            }

            # Отправляем запрос и получаем ответ
            $webResponse = $webRequest.GetResponse()
            $responseStream = $webResponse.GetResponseStream()
            $reader = New-Object System.IO.StreamReader($responseStream)
            $responseContent = $reader.ReadToEnd()
            $reader.Close()

            # Копируем статус и headers
            $response.StatusCode = [int]$webResponse.StatusCode
            $response.StatusDescription = $webResponse.StatusDescription

            foreach ($header in $webResponse.Headers.Keys) {
                try {
                    $response.AddHeader($header, $webResponse.Headers[$header])
                } catch {
                    # Некоторые headers нельзя добавить
                }
            }

            # Отправляем контент
            $buffer = [System.Text.Encoding]::UTF8.GetBytes($responseContent)
            $response.ContentLength64 = $buffer.Length
            $response.OutputStream.Write($buffer, 0, $buffer.Length)

            Write-Host " -> $($webResponse.StatusCode) ($($buffer.Length) bytes)" -ForegroundColor Green

        } catch {
            Write-Host " -> Error: $($_.Exception.Message)" -ForegroundColor Red

            # Отправляем ошибку
            $response.StatusCode = 500
            $errorMessage = "Proxy Error: $($_.Exception.Message)"
            $buffer = [System.Text.Encoding]::UTF8.GetBytes($errorMessage)
            $response.ContentLength64 = $buffer.Length
            $response.OutputStream.Write($buffer, 0, $buffer.Length)
        }

        $response.OutputStream.Close()
    }

} catch {
    Write-Host "Error starting proxy: $($_.Exception.Message)" -ForegroundColor Red
} finally {
    if ($listener) {
        $listener.Stop()
        Write-Host "Proxy server stopped" -ForegroundColor Yellow
    }
}
























