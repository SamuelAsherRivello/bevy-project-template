param(
    [int]$Port = 8080
)

$ErrorActionPreference = "Stop"
Write-Host "This script no longer works with the current typical Bevy game workflow."
Write-Host "Use scripts\RunProject.ps1 to run the active game app."
exit 1

$ScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path (Join-Path $ScriptRoot "..")
Set-Location $ProjectRoot

function Add-CargoToPathIfPresent {
    $CargoBin = Join-Path $env:USERPROFILE ".cargo\bin"
    if ((Test-Path $CargoBin) -and (-not (($env:Path -split ";") -contains $CargoBin))) {
        $env:Path = "$CargoBin;$env:Path"
    }
}

function Ensure-Command {
    param(
        [string]$Name,
        [string]$InstallCommand
    )

    if (Get-Command $Name -ErrorAction SilentlyContinue) {
        return
    }

    throw "$Name is required. Install it with: $InstallCommand"
}

function Ensure-WasmTarget {
    $Targets = rustup target list --installed
    if ($Targets -contains "wasm32-unknown-unknown") {
        return
    }

    Write-Host "Installing wasm target..."
    rustup target add wasm32-unknown-unknown
}

function Test-PortAvailable {
    param(
        [int]$Port
    )

    try {
        $listener = [System.Net.Sockets.TcpListener]::new([System.Net.IPAddress]::Loopback, $Port)
        $listener.Start()
        $listener.Stop()
        return $true
    }
    catch {
        return $false
    }
}

function Resolve-AvailablePort {
    param(
        [int]$PreferredPort
    )

    if (Test-PortAvailable -Port $PreferredPort) {
        return $PreferredPort
    }

    foreach ($Candidate in ($PreferredPort + 1)..($PreferredPort + 20)) {
        if (Test-PortAvailable -Port $Candidate) {
            Write-Warning "Port $PreferredPort is busy. Using port $Candidate instead."
            return $Candidate
        }
    }

    throw "Could not find an available port in the range $PreferredPort-$($PreferredPort + 20)."
}

function Get-WatchedFiles {
    param(
        [string]$ProjectRoot
    )

    $paths = @(
        (Join-Path $ProjectRoot "Cargo.toml"),
        (Join-Path $ProjectRoot "Cargo.lock"),
        (Join-Path $ProjectRoot "rust\crates\game\Cargo.toml"),
        (Join-Path $ProjectRoot "rust\crates\game_api\Cargo.toml"),
        (Join-Path $ProjectRoot "rust\crates\game_shell\Cargo.toml"),
        (Join-Path $ProjectRoot "rust\crates\game_shell\index.html")
    )

    $paths += Get-ChildItem (Join-Path $ProjectRoot "rust\crates\game\src") -Recurse -File -Filter *.rs | ForEach-Object FullName
    $paths += Get-ChildItem (Join-Path $ProjectRoot "rust\crates\game_api\src") -Recurse -File -Filter *.rs | ForEach-Object FullName
    $paths += Get-ChildItem (Join-Path $ProjectRoot "rust\crates\game_shell\src") -Recurse -File -Filter *.rs | ForEach-Object FullName

    $paths | Where-Object { Test-Path $_ } | Sort-Object -Unique
}

Add-CargoToPathIfPresent
Ensure-Command -Name "cargo" -InstallCommand "rustup default stable"
Ensure-Command -Name "rustup" -InstallCommand "https://rustup.rs/"
Ensure-Command -Name "trunk" -InstallCommand "cargo install trunk"
Ensure-WasmTarget

Write-Host "Starting wasm dev server with live reload."
Write-Host "This is browser live reload, not native DLL hot swap."
Write-Host "Edit Rust files and trunk will rebuild and refresh the page."
Write-Host "Press Ctrl+C to stop."
Write-Host ""

$GameShellDir = Join-Path $ProjectRoot "rust\crates\game_shell"
$ReloadTriggerPath = Join-Path $GameShellDir "reload-trigger.txt"
Set-Location $GameShellDir

$SelectedPort = Resolve-AvailablePort -PreferredPort $Port
Write-Host "Serving wasm app on http://127.0.0.1:$SelectedPort/"

$watcherJob = Start-Job -ArgumentList $ProjectRoot, $ReloadTriggerPath -ScriptBlock {
    param(
        [string]$ProjectRoot,
        [string]$ReloadTriggerPath
    )

    function Get-WatchedFiles {
        param(
            [string]$ProjectRoot
        )

        $paths = @(
            (Join-Path $ProjectRoot "Cargo.toml"),
            (Join-Path $ProjectRoot "Cargo.lock"),
            (Join-Path $ProjectRoot "rust\crates\game\Cargo.toml"),
            (Join-Path $ProjectRoot "rust\crates\game_api\Cargo.toml"),
            (Join-Path $ProjectRoot "rust\crates\game_shell\Cargo.toml"),
            (Join-Path $ProjectRoot "rust\crates\game_shell\index.html")
        )

        $paths += Get-ChildItem (Join-Path $ProjectRoot "rust\crates\game\src") -Recurse -File -Filter *.rs | ForEach-Object FullName
        $paths += Get-ChildItem (Join-Path $ProjectRoot "rust\crates\game_api\src") -Recurse -File -Filter *.rs | ForEach-Object FullName
        $paths += Get-ChildItem (Join-Path $ProjectRoot "rust\crates\game_shell\src") -Recurse -File -Filter *.rs | ForEach-Object FullName

        $paths | Where-Object { Test-Path $_ } | Sort-Object -Unique
    }

    $lastSignature = ""

    while ($true) {
        $signature = (Get-WatchedFiles -ProjectRoot $ProjectRoot | ForEach-Object {
            $item = Get-Item $_
            "{0}|{1}" -f $item.FullName, $item.LastWriteTimeUtc.Ticks
        }) -join ";"

        if ($signature -ne $lastSignature) {
            Set-Content -Path $ReloadTriggerPath -Value ("reload-trigger " + [DateTime]::UtcNow.ToString("O")) -NoNewline
            $lastSignature = $signature
        }

        Start-Sleep -Milliseconds 750
    }
}

try {
    trunk serve .\index.html `
        --open `
        --port $SelectedPort `
        --address 127.0.0.1 `
        --poll `
        --poll-interval 1s `
        --watch $ReloadTriggerPath
}
finally {
    if ($watcherJob) {
        Stop-Job $watcherJob -ErrorAction SilentlyContinue
        Remove-Job $watcherJob -Force -ErrorAction SilentlyContinue
    }
}
