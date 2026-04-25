param(
    [switch]$CleanHotReloadCopies
)

$ErrorActionPreference = "Stop"
Write-Host "This script no longer provides hot reload; it will run the normal game app instead."

$ScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path (Join-Path $ScriptRoot "..")
Set-Location $ProjectRoot
$env:WGPU_BACKEND = "dx12"

if ($CleanHotReloadCopies) {
    $HotReloadDir = Join-Path $ProjectRoot "target\hot-reload"
    if (Test-Path $HotReloadDir) {
        Remove-Item $HotReloadDir -Recurse -Force
    }
}

Write-Host "Building project..."
cargo build -p game

Write-Host ""
Write-Host "Native hot reload is disabled for the typical Bevy game template."
Write-Host "Starting project normally."
Write-Host "Press Ctrl+C to stop."
Write-Host ""

cargo run -p game
