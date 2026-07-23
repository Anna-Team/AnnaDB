# AnnaDB launcher for opencode (Windows PowerShell)
# Starts AnnaDB as a background process and cleans up on exit.

$ErrorActionPreference = "Stop"

$AnnaDBDir = "$env:USERPROFILE\.opencode\annadb"
$AnnaDBBin = "$AnnaDBDir\anna_db.exe"
$AnnaDBData = "$AnnaDBDir\warehouse"
$AnnaDBPort = if ($env:ANNADB_PORT) { $env:ANNADB_PORT } else { "10001" }

function Download-Binary {
    $arch = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "x86" }
    $url = "https://github.com/Anna-Team/AnnaDB/releases/latest/download/anna_db-windows-${arch}.exe"

    New-Item -ItemType Directory -Force -Path $AnnaDBDir | Out-Null
    Write-Host "Downloading AnnaDB from $url..."
    Invoke-WebRequest -Uri $url -OutFile $AnnaDBBin
    Write-Host "AnnaDB installed to $AnnaDBBin"
}

if (-not (Test-Path $AnnaDBBin)) {
    Download-Binary
}

Write-Host "Starting AnnaDB on port $AnnaDBPort..."

$env:EMBEDDING_PROVIDER = if ($env:ANNADB_EMBEDDING_PROVIDER) { $env:ANNADB_EMBEDDING_PROVIDER } else { "" }
$env:EMBEDDING_MODEL = if ($env:ANNADB_EMBEDDING_MODEL) { $env:ANNADB_EMBEDDING_MODEL } else { "" }

$process = Start-Process -FilePath $AnnaDBBin `
    -ArgumentList "--port", $AnnaDBPort, "--wh-path", $AnnaDBData `
    -PassThru -WindowStyle Hidden

# Register cleanup
Register-EngineEvent -SourceIdentifier PowerShell.Exiting -Action {
    if ($process -and !$process.HasExited) {
        $process.Kill()
        Write-Host "AnnaDB stopped"
    }
} | Out-Null

Start-Sleep -Seconds 1
Write-Host "AnnaDB running (PID $($process.Id))"
