param(
  [string] $Version,
  [string] $Owner = 'SujanMishra',
  [string] $Repository = 'OcentraParent',
  [string] $OutputRoot
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

  $extensionList = dotnet wix extension list
  if ($extensionList -notmatch 'WixToolset\.Util\.wixext\s+6\.0\.2') {
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

Push-Location $RepoRoot
try {
  if ([string]::IsNullOrWhiteSpace($Version)) {
    $Version = (& node scripts/release/validate-version.mjs --print-version).Trim()
  } else {
    & node scripts/release/validate-version.mjs | Out-Host
  }

  & cargo build --release -p ocentra-parent-agent-service
  Assert-Success 'Cargo release build failed.'

  if ([string]::IsNullOrWhiteSpace($OutputRoot)) {
    $OutputRoot = Join-Path $RepoRoot 'target\release-packages'
  }

  $ArtifactName = "ocentra-parent-agent-windows-x64-v$Version.msi"
  $MsiPath = Join-Path $OutputRoot $ArtifactName
  $ManifestPath = Join-Path $OutputRoot 'latest-windows.json'
  $ChecksumPath = "$MsiPath.sha256"
  $BootstrapPath = Join-Path $OutputRoot 'install-ocentra-parent-agent-windows.ps1'
  $WinSwCacheRoot = Join-Path $OutputRoot 'tool-cache\winsw'
  $WixIntermediateRoot = Join-Path $OutputRoot 'wix-obj'
  $WixSourcePath = Join-Path $RepoRoot 'scripts\release\windows\OcentraParentAgent.wxs'
  $AgentBinaryPath = Join-Path $RepoRoot 'target\release\ocentra-parent-agent-service.exe'
  $ServiceConfigPath = Join-Path $RepoRoot 'scripts\release\windows\OcentraParentAgentService.xml'
  $ServiceWrapperPath = Resolve-WinSwWrapper -CacheRoot $WinSwCacheRoot

  New-Item -ItemType Directory -Path $OutputRoot -Force | Out-Null
  Remove-Item -LiteralPath $MsiPath -Force -ErrorAction SilentlyContinue
  Remove-Item -LiteralPath (Join-Path $OutputRoot "ocentra-parent-agent-windows-x64-v$Version.zip") -Force -ErrorAction SilentlyContinue
  Remove-Item -LiteralPath (Join-Path $OutputRoot "ocentra-parent-agent-windows-x64-v$Version.zip.sha256") -Force -ErrorAction SilentlyContinue
  Remove-Item -LiteralPath (Join-Path $OutputRoot "ocentra-parent-agent-windows-x64-v$Version") -Recurse -Force -ErrorAction SilentlyContinue
  Remove-Item -LiteralPath $WixIntermediateRoot -Recurse -Force -ErrorAction SilentlyContinue

  Install-WixTooling

  dotnet wix build -ext WixToolset.Util.wixext $WixSourcePath `
    -arch x64 `
    -d "ProductVersion=$Version" `
    -d "AgentBinaryPath=$AgentBinaryPath" `
    -d "ServiceWrapperPath=$ServiceWrapperPath" `
    -d "ServiceConfigPath=$ServiceConfigPath" `
    -intermediatefolder $WixIntermediateRoot `
    -out $MsiPath
  Assert-Success 'WiX MSI build failed.'

  $ArtifactHash = (Get-FileHash -Algorithm SHA256 -Path $MsiPath).Hash.ToUpperInvariant()
  Set-Content -LiteralPath $ChecksumPath -Encoding utf8 -Value "$ArtifactHash  $ArtifactName"
  Copy-Item -LiteralPath 'scripts\release\windows\install-latest-windows.ps1' -Destination $BootstrapPath -Force

  $Manifest = [ordered]@{
    schemaVersion = 1
    product = 'Ocentra Parent'
    package = 'ocentra-parent-agent'
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
      id = 'OcentraParentAgent'
      name = 'Ocentra Parent Agent'
      wrapper = 'WinSW'
      wrapperVersion = $WinSwVersion
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
    Add-Content -LiteralPath $env:GITHUB_OUTPUT -Value "artifact_path=$MsiPath"
    Add-Content -LiteralPath $env:GITHUB_OUTPUT -Value "checksum_path=$ChecksumPath"
    Add-Content -LiteralPath $env:GITHUB_OUTPUT -Value "manifest_path=$ManifestPath"
    Add-Content -LiteralPath $env:GITHUB_OUTPUT -Value "bootstrap_path=$BootstrapPath"
  }

  Write-Host "Built $MsiPath"
  Write-Host "Built $ManifestPath"
  Write-Host "Built $BootstrapPath"
} finally {
  Pop-Location
}
