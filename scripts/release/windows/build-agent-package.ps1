param(
  [string] $Version,
  [string] $Owner = 'ocentra',
  [string] $Repository = 'OcentraParent',
  [string] $OutputRoot,
  [string] $SigningKeyBase64,
  [switch] $AllowEphemeralSigningKey
)

$ErrorActionPreference = 'Stop'

$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..\..\..')).Path
$WinSwVersion = '2.12.0'
$WinSwAssetName = 'WinSW-x64.exe'
$WinSwUrl = "https://github.com/winsw/winsw/releases/download/v$WinSwVersion/$WinSwAssetName"
$WinSwSha256 = '05B82D46AD331CC16BDC00DE5C6332C1EF818DF8CEEFCD49C726553209B3A0DA'
$WixUtilExtension = 'WixToolset.Util.wixext/6.0.2'

function Assert-Success {
  param([Parameter(Mandatory = $true)] [string] $Message)

  if ($LASTEXITCODE -ne 0) {
    throw $Message
  }
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

function Install-WixTooling {
  dotnet tool restore
  Assert-Success 'WiX dotnet tool restore failed.'

  $extensionList = @(dotnet wix extension list)
  $extensionListText = $extensionList -join "`n"
  if ($extensionList.Count -eq 0 -or $extensionListText -notmatch 'WixToolset\.Util\.wixext\s+6\.0\.2') {
    dotnet wix extension add $WixUtilExtension
    Assert-Success 'WiX Util extension install failed.'
  }
}

function Resolve-WinSwWrapper {
  param([Parameter(Mandatory = $true)] [string] $CacheRoot)

  New-Item -ItemType Directory -Path $CacheRoot -Force | Out-Null
  $wrapperPath = Join-Path $CacheRoot "WinSW-x64-v$WinSwVersion.exe"
  if (Test-Path -LiteralPath $wrapperPath) {
    Assert-FileHash -Path $wrapperPath -ExpectedSha256 $WinSwSha256
    return $wrapperPath
  }

  $downloadPath = Join-Path $CacheRoot $WinSwAssetName
  Invoke-WebRequest -Uri $WinSwUrl -OutFile $downloadPath
  Assert-FileHash -Path $downloadPath -ExpectedSha256 $WinSwSha256
  Move-Item -LiteralPath $downloadPath -Destination $wrapperPath -Force
  return $wrapperPath
}

function Invoke-UpdaterTool {
  param([Parameter(Mandatory = $true)] [string[]] $Arguments)

  $output = & cargo run --quiet -p ocentra-parent-agent-maintenance --bin ocentra-parent-agent-tool -- @Arguments
  Assert-Success 'Updater release tool failed.'
  return $output
}

function Get-UpdateSigningKey {
  param([Parameter(Mandatory = $true)] [string] $KeyPath)

  if (-not [string]::IsNullOrWhiteSpace($SigningKeyBase64)) {
    return $SigningKeyBase64.Trim()
  }
  if (-not [string]::IsNullOrWhiteSpace($env:OCENTRA_CHILD_UPDATE_SIGNING_KEY_BASE64)) {
    return $env:OCENTRA_CHILD_UPDATE_SIGNING_KEY_BASE64.Trim()
  }
  if ($AllowEphemeralSigningKey -or $env:OCENTRA_CHILD_ALLOW_EPHEMERAL_UPDATE_KEY -eq 'true') {
    return New-LocalUpdateSigningKey -KeyPath $KeyPath -Reason 'Generated an ephemeral preview update signing key.'
  }
  if ($env:GITHUB_ACTIONS -eq 'true') {
    throw 'Missing OCENTRA_CHILD_UPDATE_SIGNING_KEY_BASE64. CI releases must use the production update signing key.'
  }
  if (Test-Path -LiteralPath $KeyPath) {
    return (Get-Content -Raw -LiteralPath $KeyPath).Trim()
  }

  throw 'Missing OCENTRA_CHILD_UPDATE_SIGNING_KEY_BASE64. Supply an external signing key or explicitly use -AllowEphemeralSigningKey for a non-production preview.'
}

function New-LocalUpdateSigningKey {
  param(
    [Parameter(Mandatory = $true)] [string] $KeyPath,
    [Parameter(Mandatory = $true)] [string] $Reason
  )

  $keygenOutput = Invoke-UpdaterTool -Arguments @('keygen')
  $privateLine = $keygenOutput | Where-Object { $_ -like 'privateKeyBase64=*' } | Select-Object -First 1
  if ([string]::IsNullOrWhiteSpace($privateLine)) {
    throw 'Updater key generation did not return a private key.'
  }
  $localKey = $privateLine.Substring('privateKeyBase64='.Length).Trim()
  Set-Content -LiteralPath $KeyPath -Encoding utf8 -Value $localKey
  Write-Warning "$Reason Key path: $KeyPath. Do not use this key for production releases."
  return $localKey
}

function Get-UpdatePublicKey {
  param([Parameter(Mandatory = $true)] [string] $PrivateKeyBase64)

  $publicKey = Invoke-UpdaterTool -Arguments @('derive-public-key', '--private-key-base64', $PrivateKeyBase64)
  return ($publicKey | Select-Object -Last 1).Trim()
}

Push-Location $RepoRoot
try {
  if ([string]::IsNullOrWhiteSpace($Version)) {
    $Version = (& node scripts/release/validate-version.mjs --print-version).Trim()
  } else {
    & node scripts/release/validate-version.mjs | Out-Host
  }

  if ([string]::IsNullOrWhiteSpace($OutputRoot)) {
    $OutputRoot = Join-Path $RepoRoot 'target\release-packages'
  }

  New-Item -ItemType Directory -Path $OutputRoot -Force | Out-Null
  $LocalSigningKeyPath = Join-Path $OutputRoot 'local-dev-update-signing-key.base64.txt'
  $UpdateSigningKeyBase64 = Get-UpdateSigningKey -KeyPath $LocalSigningKeyPath
  $UpdatePublicKeyBase64 = Get-UpdatePublicKey -PrivateKeyBase64 $UpdateSigningKeyBase64
  $previousPublicKey = $env:OCENTRA_CHILD_UPDATE_PUBLIC_KEY_BASE64
  $env:OCENTRA_CHILD_UPDATE_PUBLIC_KEY_BASE64 = $UpdatePublicKeyBase64

  & cargo build --release -p ocentra-child-runtime --bin ocentra-child-agent-service
  Assert-Success 'Cargo child-agent service release build failed.'
  & cargo build --release -p ocentra-parent-agent-maintenance --bin ocentra-parent-agent-updater
  Assert-Success 'Cargo updater release build failed.'

  $ArtifactName = "ocentra-child-agent-windows-x64-v$Version.msi"
  $LatestArtifactName = 'ocentra-child-agent-windows-x64-latest.msi'
  $MsiPath = Join-Path $OutputRoot $ArtifactName
  $LatestMsiPath = Join-Path $OutputRoot $LatestArtifactName
  $ManifestPath = Join-Path $OutputRoot 'latest-windows.json'
  $ManifestPayloadPath = Join-Path $OutputRoot 'latest-windows.payload.json'
  $ChecksumPath = "$MsiPath.sha256"
  $LatestChecksumPath = "$LatestMsiPath.sha256"
  $BootstrapPath = Join-Path $OutputRoot 'install-ocentra-child-agent-windows.ps1'
  $WinSwCacheRoot = Join-Path $OutputRoot 'tool-cache\winsw'
  $WixIntermediateRoot = Join-Path $OutputRoot 'wix-obj'
  $WixSourcePath = Join-Path $RepoRoot 'scripts\release\windows\OcentraParentAgent.wxs'
  $AgentBinaryPath = Join-Path $RepoRoot 'target\release\ocentra-child-agent-service.exe'
  $UpdaterBinaryPath = Join-Path $RepoRoot 'target\release\ocentra-parent-agent-updater.exe'
  $ServiceConfigPath = Join-Path $RepoRoot 'scripts\release\windows\OcentraParentAgentService.xml'
  $UpdaterConfigPath = Join-Path $RepoRoot 'scripts\release\windows\OcentraParentUpdaterService.xml'
  $ServiceWrapperPath = Resolve-WinSwWrapper -CacheRoot $WinSwCacheRoot

  Remove-Item -LiteralPath $MsiPath -Force -ErrorAction SilentlyContinue
  Remove-Item -LiteralPath $LatestMsiPath -Force -ErrorAction SilentlyContinue
  Remove-Item -LiteralPath $LatestChecksumPath -Force -ErrorAction SilentlyContinue
  Remove-Item -LiteralPath (Join-Path $OutputRoot "ocentra-child-agent-windows-x64-v$Version.zip") -Force -ErrorAction SilentlyContinue
  Remove-Item -LiteralPath (Join-Path $OutputRoot "ocentra-child-agent-windows-x64-v$Version.zip.sha256") -Force -ErrorAction SilentlyContinue
  Remove-Item -LiteralPath (Join-Path $OutputRoot "ocentra-child-agent-windows-x64-v$Version") -Recurse -Force -ErrorAction SilentlyContinue
  Remove-Item -LiteralPath $WixIntermediateRoot -Recurse -Force -ErrorAction SilentlyContinue

  Install-WixTooling

  dotnet wix build -ext WixToolset.Util.wixext $WixSourcePath `
    -arch x64 `
    -d "ProductVersion=$Version" `
    -d "AgentBinaryPath=$AgentBinaryPath" `
    -d "UpdaterBinaryPath=$UpdaterBinaryPath" `
    -d "ServiceWrapperPath=$ServiceWrapperPath" `
    -d "ServiceConfigPath=$ServiceConfigPath" `
    -d "UpdaterConfigPath=$UpdaterConfigPath" `
    -intermediatefolder $WixIntermediateRoot `
    -out $MsiPath
  Assert-Success 'WiX MSI build failed.'

  $ArtifactHash = (Get-FileHash -Algorithm SHA256 -Path $MsiPath).Hash.ToUpperInvariant()
  Set-Content -LiteralPath $ChecksumPath -Encoding utf8 -Value "$ArtifactHash  $ArtifactName"
  Copy-Item -LiteralPath $MsiPath -Destination $LatestMsiPath -Force
  Set-Content -LiteralPath $LatestChecksumPath -Encoding utf8 -Value "$ArtifactHash  $LatestArtifactName"
  Copy-Item -LiteralPath 'scripts\release\windows\install-latest-windows.ps1' -Destination $BootstrapPath -Force

  $ManifestPayload = [ordered]@{
    schemaVersion = 1
    product = 'Ocentra Child Agent'
    package = 'ocentra-child-agent'
    version = $Version
    channel = 'stable'
    target = 'windows-x64'
    installer = [ordered]@{
      type = 'msi'
      scope = 'per-machine'
      silentArgs = '/qn /norestart'
      passiveArgs = '/passive /norestart'
    }
    service = [ordered]@{
      id = 'OcentraChildAgent'
      name = 'Ocentra Child Agent'
      wrapper = 'WinSW'
      wrapperVersion = $WinSwVersion
      updaterId = 'OcentraChildUpdater'
      updaterName = 'Ocentra Child Updater'
    }
    artifact = [ordered]@{
      name = $ArtifactName
      sha256 = $ArtifactHash
      downloadUrl = "https://github.com/$Owner/$Repository/releases/download/v$Version/$ArtifactName"
    }
    generatedAt = (Get-Date).ToUniversalTime().ToString('o')
  }
  $ManifestPayload | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath $ManifestPayloadPath -Encoding utf8
  Invoke-UpdaterTool -Arguments @(
    'sign-manifest',
    '--payload',
    $ManifestPayloadPath,
    '--out',
    $ManifestPath,
    '--private-key-base64',
    $UpdateSigningKeyBase64
  ) | Out-Host

  if ($env:GITHUB_OUTPUT) {
    Add-Content -LiteralPath $env:GITHUB_OUTPUT -Value "version=$Version"
    Add-Content -LiteralPath $env:GITHUB_OUTPUT -Value "artifact_path=$MsiPath"
    Add-Content -LiteralPath $env:GITHUB_OUTPUT -Value "checksum_path=$ChecksumPath"
    Add-Content -LiteralPath $env:GITHUB_OUTPUT -Value "latest_artifact_path=$LatestMsiPath"
    Add-Content -LiteralPath $env:GITHUB_OUTPUT -Value "latest_checksum_path=$LatestChecksumPath"
    Add-Content -LiteralPath $env:GITHUB_OUTPUT -Value "manifest_path=$ManifestPath"
    Add-Content -LiteralPath $env:GITHUB_OUTPUT -Value "bootstrap_path=$BootstrapPath"
  }

  Write-Host "Built $MsiPath"
  Write-Host "Built $LatestMsiPath"
  Write-Host "Built $ManifestPath"
  Write-Host "Built $BootstrapPath"
} finally {
  if ($null -eq $previousPublicKey) {
    Remove-Item Env:\OCENTRA_CHILD_UPDATE_PUBLIC_KEY_BASE64 -ErrorAction SilentlyContinue
  } else {
    $env:OCENTRA_CHILD_UPDATE_PUBLIC_KEY_BASE64 = $previousPublicKey
  }
  Pop-Location
}
