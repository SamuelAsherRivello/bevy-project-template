$ErrorActionPreference = "Stop"

$ScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path (Join-Path $ScriptRoot "..\..")
& (Join-Path $ScriptRoot "StopGame.ps1")
Set-Location $ProjectRoot

Write-Host "Building hot reload crate..."
cargo check --manifest-path Bevy/Crates/HotReload/bevy_simple_subsecond_system/Cargo.toml
