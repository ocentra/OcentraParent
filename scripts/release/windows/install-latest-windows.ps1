#Requires -RunAsAdministrator

param(
  [string] $Owner = 'SujanMishra',
  [string] $Repository = 'OcentraParent',
  [string] $InstallDirectory = "$env:ProgramFiles\Ocentra Parent\Agent"
)

$ErrorActionPreference = 'Stop'

function Get-ReleaseAsset {
  param(
    [Parameter(Mandatory = $true)] [object] $Release,
    [Parameter(Mandatory = $true)] [string] $Name
  )

  $asset = $Release.assets | Where-Object { $_.name -eq $Name } | Select-Object -First 1
  if ($null -eq $asset) {
    throw "Release asset not found: $Name"
  }
  return $asset
}

function Assert-FileHash {
  param(
    [Parameter(Mandatory = $true)] [string] $Path,
    [Parameter(Mandatory = $true)] [string] $ExpectedSha256
  )

  $actual = (Get-FileHash -Algorithm SHA256 -Path $Path).Hash.ToUpperInvariant()
  if ($actual -ne $ExpectedSha256.ToUpperInvariant()) {
    throw "SHA256 mismatch for $Path. Expected $ExpectedSha256 but found $actual."
  }
}

$release = Invoke-RestMethod -Uri "https://api.github.com/repos/$Owner/$Repository/releases/latest"
$manifestAsset = Get-ReleaseAsset -Release $release -Name 'latest-windows.json'
$tempRoot = Join-Path ([System.IO.Path]::GetTempPath()) "ocentra-parent-agent-$([Guid]::NewGuid().ToString('N'))"
$manifestPath = Join-Path $tempRoot 'latest-windows.json'

New-Item -ItemType Directory -Path $tempRoot -Force | Out-Null

try {
  Invoke-WebRequest -Uri $manifestAsset.browser_download_url -OutFile $manifestPath
  $manifest = Get-Content -Raw -LiteralPath $manifestPath | ConvertFrom-Json
  $artifactAsset = Get-ReleaseAsset -Release $release -Name $manifest.artifact.name
  $artifactPath = Join-Path $tempRoot $manifest.artifact.name
  $extractPath = Join-Path $tempRoot 'package'

  Invoke-WebRequest -Uri $artifactAsset.browser_download_url -OutFile $artifactPath
  Assert-FileHash -Path $artifactPath -ExpectedSha256 $manifest.artifact.sha256

  Expand-Archive -LiteralPath $artifactPath -DestinationPath $extractPath -Force
  & (Join-Path $extractPath 'install\windows\install-service.ps1') -InstallDirectory $InstallDirectory
  if ($LASTEXITCODE -ne 0) {
    throw 'Ocentra Parent agent installer failed.'
  }
} finally {
  Remove-Item -LiteralPath $tempRoot -Recurse -Force -ErrorAction SilentlyContinue
}
