$ErrorActionPreference = "Stop"

$ScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path (Join-Path $ScriptRoot "..\..")
& (Join-Path $ScriptRoot "StopGame.ps1")
Set-Location $ProjectRoot

Write-Host "Building shared crate..."
cargo build -p shared
