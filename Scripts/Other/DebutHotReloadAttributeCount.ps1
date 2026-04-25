$ErrorActionPreference = "Stop"

$ScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path (Join-Path $ScriptRoot "..\..")
Set-Location $ProjectRoot

# Ignore list: any file path containing one of these fragments will be skipped.
$ignorePathFragments = @(
    "Bevy\Crates\HotReload\"
)

# Match #[hot] and #[hot(...)] attributes.
$hotAttributePattern = '#\[\s*hot\s*(\]|\()'

$rustFiles = Get-ChildItem -Path "Bevy" -Recurse -File -Filter "*.rs"
$matchesByFile = @{}
$totalCount = 0

foreach ($file in $rustFiles) {
    $relativePath = $file.FullName.Replace($ProjectRoot.Path + [System.IO.Path]::DirectorySeparatorChar, "")

    $isIgnored = $false
    foreach ($fragment in $ignorePathFragments) {
        if ($relativePath.Contains($fragment)) {
            $isIgnored = $true
            break
        }
    }

    if ($isIgnored) {
        continue
    }

    $matches = Select-String -Path $file.FullName -Pattern $hotAttributePattern -AllMatches
    if (-not $matches) {
        continue
    }

    $fileCount = 0
    foreach ($lineMatch in $matches) {
        $fileCount += $lineMatch.Matches.Count
    }

    if ($fileCount -gt 0) {
        $matchesByFile[$relativePath] = $fileCount
        $totalCount += $fileCount
    }
}

Write-Host "Total hot reload attributes: $totalCount"
Write-Host ""
Write-Host "Files:"

foreach ($path in ($matchesByFile.Keys | Sort-Object)) {
    Write-Host $path
}
