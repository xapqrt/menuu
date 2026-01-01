$listener = New-Object System.Net.HttpListener
$listener.Prefixes.Add('http://localhost:8080/')
$listener.Start()
Write-Host 'Server running at http://localhost:8080/'
Write-Host 'Press Ctrl+C to stop'
while ($listener.IsListening) {
    $context = $listener.GetContext()
    $request = $context.Request
    $response = $context.Response
    $path = if ($request.Url.LocalPath -eq '/') { '/index.html' } else { $request.Url.LocalPath }
    $file = Join-Path $PWD $path.TrimStart('/')
    if (Test-Path $file) {
        $content = [System.IO.File]::ReadAllBytes($file)
        $ext = [System.IO.Path]::GetExtension($file)
        $mimeTypes = @{
            '.html' = 'text/html'
            '.css' = 'text/css'
            '.js' = 'application/javascript'
            '.wasm' = 'application/wasm'
            '.json' = 'application/json'
        }
        $response.ContentType = if ($mimeTypes[$ext]) { $mimeTypes[$ext] } else { 'application/octet-stream' }
        $response.ContentLength64 = $content.Length
        $response.OutputStream.Write($content, 0, $content.Length)
    } else {
        $response.StatusCode = 404
    }
    $response.Close()
}
