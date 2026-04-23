param(
    [switch]$CleanHotReloadCopies
)

$ErrorActionPreference = "Stop"

$ScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path (Join-Path $ScriptRoot "..")
Set-Location $ProjectRoot

if ($CleanHotReloadCopies) {
    $HotReloadDir = Join-Path $ProjectRoot "target\hot-reload"
    if (Test-Path $HotReloadDir) {
        Remove-Item $HotReloadDir -Recurse -Force
    }
}

Write-Host "Building project..."
cargo build -p game

Write-Host ""
Write-Host "Starting project with hot reload."
Write-Host "Edit Rust files under rust\\crates\\game\\src and save to reload."
Write-Host "Press Ctrl+C to stop."
Write-Host ""

cargo run -p game-shell -- --hot-reload
