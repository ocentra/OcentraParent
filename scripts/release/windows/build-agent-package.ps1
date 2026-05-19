param(
  [string] $Version,
  [string] $Owner = 'SujanMishra',
  [string] $Repository = 'OcentraParent',
  [string] $OutputRoot
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

  & cargo build --release -p ocentra-parent-agent-service
  if ($LASTEXITCODE -ne 0) {
    throw 'Cargo release build failed.'
  }

  if ([string]::IsNullOrWhiteSpace($OutputRoot)) {
    $OutputRoot = Join-Path $RepoRoot 'target\release-packages'
  }

  $ArtifactName = "ocentra-parent-agent-windows-x64-v$Version.zip"
  $PackageName = "ocentra-parent-agent-windows-x64-v$Version"
  $StagingRoot = Join-Path $OutputRoot $PackageName
  $InstallerRoot = Join-Path $StagingRoot 'install\windows'
  $ZipPath = Join-Path $OutputRoot $ArtifactName
  $ManifestPath = Join-Path $OutputRoot 'latest-windows.json'
  $ChecksumPath = "$ZipPath.sha256"
  $BootstrapPath = Join-Path $OutputRoot 'install-ocentra-parent-agent-windows.ps1'

  Remove-Item -LiteralPath $StagingRoot -Recurse -Force -ErrorAction SilentlyContinue
  New-Item -ItemType Directory -Path $InstallerRoot -Force | Out-Null
  New-Item -ItemType Directory -Path $OutputRoot -Force | Out-Null

  Copy-Item -LiteralPath 'target\release\ocentra-parent-agent-service.exe' -Destination $StagingRoot -Force
  Copy-Item -LiteralPath 'scripts\release\windows\OcentraParentAgentService.xml' -Destination $InstallerRoot -Force
  Copy-Item -LiteralPath 'scripts\release\windows\install-service.ps1' -Destination $InstallerRoot -Force
  Copy-Item -LiteralPath 'scripts\release\windows\uninstall-service.ps1' -Destination $InstallerRoot -Force
  Copy-Item -LiteralPath 'scripts\release\windows\install-latest-windows.ps1' -Destination $BootstrapPath -Force

  Set-Content -LiteralPath (Join-Path $StagingRoot 'README.txt') -Encoding utf8 -Value @"
Ocentra Parent Agent $Version

Run install\windows\install-service.ps1 from an elevated PowerShell session to install the headless agent service.
Run install\windows\uninstall-service.ps1 from an elevated PowerShell session to remove it.
"@

  Remove-Item -LiteralPath $ZipPath -Force -ErrorAction SilentlyContinue
  Compress-Archive -Path (Join-Path $StagingRoot '*') -DestinationPath $ZipPath -Force

  $ArtifactHash = (Get-FileHash -Algorithm SHA256 -Path $ZipPath).Hash.ToUpperInvariant()
  Set-Content -LiteralPath $ChecksumPath -Encoding utf8 -Value "$ArtifactHash  $ArtifactName"

  $Manifest = [ordered]@{
    schemaVersion = 1
    product = 'Ocentra Parent'
    package = 'ocentra-parent-agent'
    version = $Version
    channel = 'stable'
    target = 'windows-x64'
    service = [ordered]@{
      id = 'OcentraParentAgent'
      name = 'Ocentra Parent Agent'
    }
    artifact = [ordered]@{
      name = $ArtifactName
      sha256 = $ArtifactHash
      downloadUrl = "https://github.com/$Owner/$Repository/releases/download/v$Version/$ArtifactName"
    }
    generatedAt = (Get-Date).ToUniversalTime().ToString('o')
  }
  $Manifest | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath $ManifestPath -Encoding utf8

  if ($env:GITHUB_OUTPUT) {
    Add-Content -LiteralPath $env:GITHUB_OUTPUT -Value "version=$Version"
    Add-Content -LiteralPath $env:GITHUB_OUTPUT -Value "artifact_path=$ZipPath"
    Add-Content -LiteralPath $env:GITHUB_OUTPUT -Value "checksum_path=$ChecksumPath"
    Add-Content -LiteralPath $env:GITHUB_OUTPUT -Value "manifest_path=$ManifestPath"
    Add-Content -LiteralPath $env:GITHUB_OUTPUT -Value "bootstrap_path=$BootstrapPath"
  }

  Write-Host "Built $ZipPath"
  Write-Host "Built $ManifestPath"
  Write-Host "Built $BootstrapPath"
} finally {
  Pop-Location
}
