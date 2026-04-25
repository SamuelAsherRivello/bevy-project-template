$ErrorActionPreference = "Stop"

$ScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path (Join-Path $ScriptRoot "..\..")
& (Join-Path $ScriptRoot "StopGame.ps1")
Set-Location $ProjectRoot
$env:WGPU_BACKEND = "dx12"

Write-Host "Building game..."
cargo build -p game

Write-Host ""
Write-Host "Starting Windows game build."
Write-Host "Press Ctrl+C to stop."
Write-Host ""

cargo run -p game
