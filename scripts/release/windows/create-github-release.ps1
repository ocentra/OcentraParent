param(
  [string] $Version,
  [string] $Owner = 'ocentra',
  [string] $Repository = 'OcentraParent',
  [string] $PackageRoot
)

$ErrorActionPreference = 'Stop'

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot '..\..\..')

Push-Location $RepoRoot
try {
  if ([string]::IsNullOrWhiteSpace($Version)) {
    $Version = (& node scripts/release/validate-version.mjs --print-version).Trim()
  } else {
    & node scripts/release/validate-version.mjs | Out-Host
  }

  if ([string]::IsNullOrWhiteSpace($PackageRoot)) {
    $PackageRoot = Join-Path $RepoRoot 'target\release-packages'
  }

  $Tag = "v$Version"
  $ArtifactPath = Join-Path $PackageRoot "ocentra-child-agent-windows-x64-v$Version.msi"
  $LatestArtifactPath = Join-Path $PackageRoot 'ocentra-child-agent-windows-x64-latest.msi'
  $ChecksumPath = "$ArtifactPath.sha256"
  $LatestChecksumPath = "$LatestArtifactPath.sha256"
  $ManifestPath = Join-Path $PackageRoot 'latest-windows.json'
  $BootstrapPath = Join-Path $PackageRoot 'install-ocentra-child-agent-windows.ps1'
  $NotesPath = Join-Path $PackageRoot "release-notes-v$Version.md"

  foreach ($Path in @($ArtifactPath, $ChecksumPath, $LatestArtifactPath, $LatestChecksumPath, $ManifestPath, $BootstrapPath)) {
    if (-not (Test-Path -LiteralPath $Path)) {
      throw "Release asset missing: $Path"
    }
  }

  git fetch --tags origin
  $existingTag = git tag --list $Tag
  if (-not [string]::IsNullOrWhiteSpace($existingTag)) {
    throw "Release tag already exists: $Tag. Bump the version before publishing from production."
  }

  git config user.name 'github-actions[bot]'
  git config user.email '41898282+github-actions[bot]@users.noreply.github.com'
  git tag -a $Tag -m "Ocentra Child Agent $Version"
  git push origin $Tag

  Set-Content -LiteralPath $NotesPath -Encoding utf8 -Value @"
Ocentra Child Agent $Version

Windows MSI:

````powershell
msiexec /i ocentra-child-agent-windows-x64-v$Version.msi
````

Stable latest MSI download:

https://github.com/$Owner/$Repository/releases/latest/download/ocentra-child-agent-windows-x64-latest.msi

Windows latest-release bootstrap:

````powershell
powershell -NoProfile -ExecutionPolicy Bypass -Command "irm https://github.com/$Owner/$Repository/releases/latest/download/install-ocentra-child-agent-windows.ps1 | iex"
````

This release contains the headless Windows agent MSI, checksum, bootstrap installer, and update manifest.
"@

  gh release create $Tag $ArtifactPath $ChecksumPath $LatestArtifactPath $LatestChecksumPath $ManifestPath $BootstrapPath --repo "$Owner/$Repository" --title "Ocentra Child Agent $Version" --notes-file $NotesPath
  if ($LASTEXITCODE -ne 0) {
    throw "GitHub release creation failed for $Tag"
  }
} finally {
  Pop-Location
}
