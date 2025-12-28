# Создание простого HTTP прокси для API с правильным Host header
Write-Host "=== Создание прокси для API ErkaPharm ===" -ForegroundColor Green

# Проверяем .NET
$dotnetVersion = Get-ChildItem 'HKLM:\SOFTWARE\Microsoft\NET Framework Setup\NDP\v4\Full' | Get-ItemPropertyValue -Name Version -ErrorAction SilentlyContinue
if (-not $dotnetVersion) {
    Write-Host "❌ .NET Framework не найден" -ForegroundColor Red
    Write-Host "Установите .NET Framework 4.5 или выше" -ForegroundColor Yellow
    exit 1
}

Write-Host "✓ .NET Framework найден: $dotnetVersion" -ForegroundColor Green

# Создаем простой C# прокси
$proxyCode = @"
using System;
using System.Net;
using System.Net.Sockets;
using System.Threading;
using System.Text;

class ApiProxy
{
    static void Main(string[] args)
    {
        Console.WriteLine("=== API Proxy Server ===");
        Console.WriteLine("Proxy: http://localhost:3000 -> https://localhost:9443 (with Host: api.erkapharm.com)");

        var listener = new TcpListener(IPAddress.Loopback, 3000);
        listener.Start();
        Console.WriteLine("Proxy listening on http://localhost:3000");

        while (true)
        {
            var client = listener.AcceptTcpClient();
            ThreadPool.QueueUserWorkItem(HandleRequest, client);
        }
    }

    static void HandleRequest(object obj)
    {
        var client = (TcpClient)obj;
        try
        {
            using (var clientStream = client.GetStream())
            {
                var buffer = new byte[4096];
                var bytesRead = clientStream.Read(buffer, 0, buffer.Length);
                var request = Encoding.UTF8.GetString(buffer, 0, bytesRead);

                // Проверяем что это GET запрос
                if (request.StartsWith("GET"))
                {
                    // Создаем запрос к реальному API
                    var webRequest = (HttpWebRequest)WebRequest.Create("https://localhost:9443");
                    webRequest.Method = "GET";
                    webRequest.Host = "api.erkapharm.com";
                    webRequest.ServerCertificateValidationCallback = delegate { return true; };

                    try
                    {
                        var response = (HttpWebResponse)webRequest.GetResponse();
                        var responseStream = response.GetResponseStream();

                        // Читаем ответ
                        var responseBuffer = new byte[4096];
                        var responseBytesRead = responseStream.Read(responseBuffer, 0, responseBuffer.Length);

                        // Отправляем ответ клиенту
                        var responseText = Encoding.UTF8.GetString(responseBuffer, 0, responseBytesRead);
                        var responseBytes = Encoding.UTF8.GetBytes(responseText);

                        clientStream.Write(responseBytes, 0, responseBytes.Length);
                    }
                    catch (Exception ex)
                    {
                        var errorResponse = $"HTTP/1.1 500 Error\r\nContent-Type: text/plain\r\n\r\nError: {ex.Message}";
                        var errorBytes = Encoding.UTF8.GetBytes(errorResponse);
                        clientStream.Write(errorBytes, 0, errorBytes.Length);
                    }
                }
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine($"Error handling request: {ex.Message}");
        }
        finally
        {
            client.Close();
        }
    }
}
"@

# Сохраняем код в файл
$proxyCode | Out-File -FilePath "ApiProxy.cs" -Encoding UTF8

Write-Host "✓ Код прокси создан: ApiProxy.cs" -ForegroundColor Green

# Компилируем
Write-Host "Компиляция прокси..." -ForegroundColor Cyan
$cscPath = "C:\Windows\Microsoft.NET\Framework\v4.0.30319\csc.exe"
if (Test-Path $cscPath) {
    & $cscPath /out:ApiProxy.exe ApiProxy.cs
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✓ Прокси скомпилирован: ApiProxy.exe" -ForegroundColor Green
    } else {
        Write-Host "❌ Ошибка компиляции" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "❌ C# компилятор не найден" -ForegroundColor Red
    Write-Host "Попробуйте другой подход" -ForegroundColor Yellow
    exit 1
}

Write-Host ""
Write-Host "=== ЗАПУСК ПРОКСИ ===" -ForegroundColor Green
Write-Host "Запустите: .\ApiProxy.exe"
Write-Host "Затем откройте: http://localhost:3000"
Write-Host ""
Write-Host "Остановка: Ctrl+C"
Write-Host ""

# Запускаем прокси
Write-Host "Запуск прокси..." -ForegroundColor Cyan
Start-Process -FilePath ".\ApiProxy.exe" -NoNewWindow
























