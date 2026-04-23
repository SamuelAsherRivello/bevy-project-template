param(
    [switch]$All
)

$ErrorActionPreference = "Stop"

$ScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path (Join-Path $ScriptRoot "..")
$ShellExe = Join-Path $ProjectRoot "target\debug\game-shell.exe"

$Processes = Get-Process game-shell -ErrorAction SilentlyContinue |
    Where-Object { $_.Path -eq $ShellExe }

if (-not $Processes) {
    Write-Host "No project process is running for this project."
    exit 0
}

if (-not $All -and $Processes.Count -gt 1) {
    Write-Host "Found multiple project processes:"
    $Processes | Select-Object Id, ProcessName, StartTime, Path | Format-Table
    Write-Host "Run .\scripts\StopProject.ps1 -All to stop all of them."
    exit 1
}

foreach ($Process in $Processes) {
    Write-Host "Stopping project PID $($Process.Id)..."
    Stop-Process -Id $Process.Id
}

Write-Host "Stopped project."
