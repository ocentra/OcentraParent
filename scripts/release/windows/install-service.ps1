#Requires -RunAsAdministrator

param(
  [string] $InstallDirectory = "$env:ProgramFiles\Ocentra Parent\Agent"
)

$ErrorActionPreference = 'Stop'

$ServiceId = 'OcentraParentAgent'
$WrapperName = 'OcentraParentAgentService.exe'
$WrapperConfigName = 'OcentraParentAgentService.xml'
$AgentBinaryName = 'ocentra-parent-agent-service.exe'
$WinSwUrl = 'https://github.com/winsw/winsw/releases/download/v2.12.0/WinSW-x64.exe'
$WinSwSha256 = '05B82D46AD331CC16BDC00DE5C6332C1EF818DF8CEEFCD49C726553209B3A0DA'

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

function Invoke-WinSw {
  param(
    [Parameter(Mandatory = $true)] [string] $WrapperPath,
    [Parameter(Mandatory = $true)] [string] $Command
  )

  & $WrapperPath $Command
  if ($LASTEXITCODE -ne 0) {
    throw "WinSW command failed: $Command"
  }
}

$packageRoot = Resolve-Path (Join-Path $PSScriptRoot '..\..')
$agentSourcePath = Join-Path $packageRoot $AgentBinaryName
$configSourcePath = Join-Path $PSScriptRoot $WrapperConfigName

if (-not (Test-Path -LiteralPath $agentSourcePath)) {
  throw "Agent binary not found: $agentSourcePath"
}

New-Item -ItemType Directory -Path $InstallDirectory -Force | Out-Null

$wrapperPath = Join-Path $InstallDirectory $WrapperName
$wrapperConfigPath = Join-Path $InstallDirectory $WrapperConfigName
$agentTargetPath = Join-Path $InstallDirectory $AgentBinaryName
$service = Get-Service -Name $ServiceId -ErrorAction SilentlyContinue

if ($null -ne $service -and $service.Status -ne 'Stopped') {
  Stop-Service -Name $ServiceId -Force -ErrorAction Stop
  $service.WaitForStatus('Stopped', [TimeSpan]::FromSeconds(30))
}

$downloadPath = Join-Path ([System.IO.Path]::GetTempPath()) $WrapperName
Invoke-WebRequest -Uri $WinSwUrl -OutFile $downloadPath
Assert-FileHash -Path $downloadPath -ExpectedSha256 $WinSwSha256

Copy-Item -LiteralPath $downloadPath -Destination $wrapperPath -Force
Copy-Item -LiteralPath $agentSourcePath -Destination $agentTargetPath -Force
Copy-Item -LiteralPath $configSourcePath -Destination $wrapperConfigPath -Force

if ($null -eq $service) {
  Invoke-WinSw -WrapperPath $wrapperPath -Command 'install'
}

Invoke-WinSw -WrapperPath $wrapperPath -Command 'start'
Write-Host "Installed and started $ServiceId at $InstallDirectory"
