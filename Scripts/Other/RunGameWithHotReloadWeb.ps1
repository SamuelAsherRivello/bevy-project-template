$ErrorActionPreference = "Stop"
Write-Host "This script uses local bevy_simple_subsecond_system hot reload for browser/wasm."

$ScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path (Join-Path $ScriptRoot "..\..")
& (Join-Path $ScriptRoot "StopGame.ps1")
Set-Location $ProjectRoot
$env:BEVY_ASSET_ROOT = "$ProjectRoot"

if (-not (Get-Command dx -ErrorAction SilentlyContinue)) {
    throw "Dioxus CLI is required for subsecond hot reload. Install it with: cargo install dioxus-cli@0.7.0-rc.1"
}

Write-Host ""
Write-Host "Starting game wasm/browser subsecond hot reload."
Write-Host "Edit hot-annotated systems under Bevy\Crates\Game\Runtime and save."
Write-Host "Press Ctrl+C to stop."
Write-Host ""

$WebAddress = "127.0.0.1"
$WebPort = "8080"

# Ensure the interactive dashboard has enough columns to render the full address.
try {
    $MinimumColumns = 140
    $RawUi = $Host.UI.RawUI
    if ($RawUi.BufferSize.Width -lt $MinimumColumns) {
        $BufferSize = $RawUi.BufferSize
        $BufferSize.Width = $MinimumColumns
        $RawUi.BufferSize = $BufferSize
    }
    if ($RawUi.WindowSize.Width -lt $MinimumColumns) {
        $WindowSize = $RawUi.WindowSize
        $WindowSize.Width = $MinimumColumns
        $RawUi.WindowSize = $WindowSize
    }
} catch {
    # Some hosts do not allow resizing; continue without failing.
}

# Use the web platform so the app is built for wasm and served in the browser.
dx serve --hot-patch --platform web --addr $WebAddress --port $WebPort --open true --package game --bin game
