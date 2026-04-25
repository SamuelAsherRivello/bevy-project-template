$ErrorActionPreference = "Stop"

$ScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path (Join-Path $ScriptRoot "..\..")
Set-Location $ProjectRoot

function Add-CargoToPathIfPresent {
    $CargoBin = Join-Path $env:USERPROFILE ".cargo\bin"
    if ((Test-Path $CargoBin) -and (-not (($env:Path -split ";") -contains $CargoBin))) {
        $env:Path = "$CargoBin;$env:Path"
    }
}

function Ensure-RustToolchain {
    Add-CargoToPathIfPresent

    if (Get-Command cargo -ErrorAction SilentlyContinue) {
        Write-Host "Rust is already installed."
        return
    }

    if (-not (Get-Command winget -ErrorAction SilentlyContinue)) {
        throw "Rust is not installed and winget is unavailable. Install rustup from https://rustup.rs/ and rerun this script."
    }

    Write-Host "Installing Rust via rustup..."
    winget install --id Rustlang.Rustup -e --accept-package-agreements --accept-source-agreements

    Add-CargoToPathIfPresent

    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        throw "Rust installation finished, but cargo is not available in this PowerShell session yet. Open a new terminal and rerun this script."
    }
}

Ensure-RustToolchain

Write-Host "Building game..."
cargo build -p game

Write-Host ""
Write-Host "Game install complete."
