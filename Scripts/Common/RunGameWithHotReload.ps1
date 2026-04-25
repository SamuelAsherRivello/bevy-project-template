$ErrorActionPreference = "Stop"
Write-Host "This script uses local bevy_simple_subsecond_system hot reload."

$ScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path (Join-Path $ScriptRoot "..\..")
& (Join-Path $ScriptRoot "..\Other\StopGame.ps1")
Set-Location $ProjectRoot
$env:WGPU_BACKEND = "dx12"
$env:BEVY_ASSET_ROOT = "$ProjectRoot"

if (-not (Get-Command dx -ErrorAction SilentlyContinue)) {
    throw "Dioxus CLI is required for subsecond hot reload. Install it with: cargo install dioxus-cli@0.7.0-rc.1"
}

Write-Host ""
Write-Host "Starting game subsecond hot reload."
Write-Host "Edit hot-annotated systems under Bevy\Crates\Game\Runtime and save."
Write-Host "Press Ctrl+C to stop."
Write-Host ""

dx serve --hot-patch --windows --package game --bin game
