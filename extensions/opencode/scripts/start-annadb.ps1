# AnnaDB launcher for opencode (Windows PowerShell)
# Starts AnnaDB as a background process, launches opencode, and cleans up on exit.

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

$annaProcess = Start-Process -FilePath $AnnaDBBin `
    -ArgumentList "--port", $AnnaDBPort, "--wh-path", $AnnaDBData `
    -PassThru -WindowStyle Hidden

Start-Sleep -Seconds 1
Write-Host "AnnaDB running (PID $($annaProcess.Id))"

# Verify health before launching opencode
try {
    $null = Invoke-WebRequest -Uri "http://localhost:$AnnaDBPort/health" -TimeoutSec 5
    Write-Host "AnnaDB health check passed"
} catch {
    Write-Host "WARNING: AnnaDB health check failed: $_"
}

# Launch opencode (passes through any extra arguments)
$opencodeArgs = $args -join " "
Write-Host "Launching opencode..."
$opencodeExit = 0
try {
    & opencode @args
    $opencodeExit = $LASTEXITCODE
} finally {
    # Cleanup: kill AnnaDB when opencode exits
    if ($annaProcess -and !$annaProcess.HasExited) {
        $annaProcess.Kill()
        Write-Host "AnnaDB stopped"
    }
}

exit $opencodeExit
