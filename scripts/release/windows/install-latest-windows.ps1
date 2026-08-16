#Requires -RunAsAdministrator

param(
  [string] $Owner = 'ocentra',
  [string] $Repository = 'OcentraParent',
  [string] $ManifestVerifierPath = $env:OCENTRA_CHILD_UPDATE_VERIFIER_PATH,
  [string] $UpdatePublicKeyBase64 = $env:OCENTRA_CHILD_UPDATE_PUBLIC_KEY_BASE64,
  [switch] $Quiet
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

  if ($ExpectedSha256 -notmatch '^[A-Fa-f0-9]{64}$') {
    throw "Invalid SHA256 value for $Path."
  }
  $actual = (Get-FileHash -Algorithm SHA256 -Path $Path).Hash.ToUpperInvariant()
  if ($actual -ne $ExpectedSha256.ToUpperInvariant()) {
    throw "SHA256 mismatch for $Path. Expected $ExpectedSha256 but found $actual."
  }
}

function Assert-ManifestSignature {
  param(
    [Parameter(Mandatory = $true)] [string] $ManifestPath
  )

  if ([string]::IsNullOrWhiteSpace($ManifestVerifierPath) -or
      -not (Test-Path -LiteralPath $ManifestVerifierPath -PathType Leaf)) {
    throw 'A trusted child update manifest verifier executable is required.'
  }
  if ([string]::IsNullOrWhiteSpace($UpdatePublicKeyBase64)) {
    throw 'OCENTRA_CHILD_UPDATE_PUBLIC_KEY_BASE64 is required to verify the release manifest.'
  }

  & $ManifestVerifierPath verify-manifest --manifest $ManifestPath --public-key-base64 $UpdatePublicKeyBase64 | Out-Host
  if ($LASTEXITCODE -ne 0) {
    throw "Release manifest signature verification failed with exit code $LASTEXITCODE."
  }
}

function Invoke-MsiInstall {
  param(
    [Parameter(Mandatory = $true)] [string] $Path,
    [Parameter(Mandatory = $true)] [bool] $Silent
  )

  $uiMode = if ($Silent) { '/qn' } else { '/passive' }
  $arguments = "/i `"$Path`" $uiMode /norestart"
  $process = Start-Process -FilePath msiexec.exe -ArgumentList $arguments -Wait -PassThru
  if ($process.ExitCode -ne 0 -and $process.ExitCode -ne 3010) {
    throw "MSI install failed with exit code $($process.ExitCode)."
  }
}

$release = Invoke-RestMethod -Uri "https://api.github.com/repos/$Owner/$Repository/releases/latest"
$manifestAsset = Get-ReleaseAsset -Release $release -Name 'latest-windows.json'
$tempRoot = Join-Path ([System.IO.Path]::GetTempPath()) "ocentra-child-agent-$([Guid]::NewGuid().ToString('N'))"
$manifestPath = Join-Path $tempRoot 'latest-windows.json'

New-Item -ItemType Directory -Path $tempRoot -Force | Out-Null

try {
  Invoke-WebRequest -Uri $manifestAsset.browser_download_url -OutFile $manifestPath
  Assert-ManifestSignature -ManifestPath $manifestPath
  $manifest = Get-Content -Raw -LiteralPath $manifestPath | ConvertFrom-Json
  if ($manifest.payload.product -ne 'Ocentra Child Agent' -or
      $manifest.payload.package -ne 'ocentra-child-agent' -or
      $manifest.payload.service.id -ne 'OcentraChildAgent' -or
      $manifest.payload.service.updaterId -ne 'OcentraChildUpdater') {
    throw 'Release manifest is not for the Ocentra Child Agent.'
  }
  if ($manifest.payload.installer.type -ne 'msi') {
    throw "Unsupported Windows installer type: $($manifest.payload.installer.type)"
  }
  if ($manifest.payload.artifact.name -notmatch '^[^\\/]+\.msi$') {
    throw 'Release manifest artifact name is not a safe MSI file name.'
  }

  $artifactAsset = Get-ReleaseAsset -Release $release -Name $manifest.payload.artifact.name
  $artifactPath = Join-Path $tempRoot $manifest.payload.artifact.name

  Invoke-WebRequest -Uri $artifactAsset.browser_download_url -OutFile $artifactPath
  Assert-FileHash -Path $artifactPath -ExpectedSha256 $manifest.payload.artifact.sha256
  Invoke-MsiInstall -Path $artifactPath -Silent $Quiet.IsPresent
} finally {
  Remove-Item -LiteralPath $tempRoot -Recurse -Force -ErrorAction SilentlyContinue
}
