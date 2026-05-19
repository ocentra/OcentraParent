param(
  [string] $Version,
  [string] $Owner = 'SujanMishra',
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
  $ArtifactPath = Join-Path $PackageRoot "ocentra-parent-agent-windows-x64-v$Version.msi"
  $ChecksumPath = "$ArtifactPath.sha256"
  $ManifestPath = Join-Path $PackageRoot 'latest-windows.json'
  $BootstrapPath = Join-Path $PackageRoot 'install-ocentra-parent-agent-windows.ps1'
  $NotesPath = Join-Path $PackageRoot "release-notes-v$Version.md"

  foreach ($Path in @($ArtifactPath, $ChecksumPath, $ManifestPath, $BootstrapPath)) {
    if (-not (Test-Path -LiteralPath $Path)) {
      throw "Release asset missing: $Path"
    }
  }

  git fetch --tags origin
  $existingTag = git tag --list $Tag
  if (-not [string]::IsNullOrWhiteSpace($existingTag)) {
    throw "Release tag already exists: $Tag. Bump the version before merging to main."
  }

  git config user.name 'github-actions[bot]'
  git config user.email '41898282+github-actions[bot]@users.noreply.github.com'
  git tag -a $Tag -m "Ocentra Parent $Version"
  git push origin $Tag

  Set-Content -LiteralPath $NotesPath -Encoding utf8 -Value @"
Ocentra Parent $Version

Windows MSI:

````powershell
msiexec /i ocentra-parent-agent-windows-x64-v$Version.msi
````

Windows latest-release bootstrap:

````powershell
powershell -NoProfile -ExecutionPolicy Bypass -Command "irm https://github.com/$Owner/$Repository/releases/latest/download/install-ocentra-parent-agent-windows.ps1 | iex"
````

This release contains the headless Windows agent MSI, checksum, bootstrap installer, and update manifest.
"@

  gh release create $Tag $ArtifactPath $ChecksumPath $ManifestPath $BootstrapPath --repo "$Owner/$Repository" --title "Ocentra Parent $Version" --notes-file $NotesPath
  if ($LASTEXITCODE -ne 0) {
    throw "GitHub release creation failed for $Tag"
  }
} finally {
  Pop-Location
}
