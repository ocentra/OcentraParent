#Requires -RunAsAdministrator

param(
  [string] $InstallDirectory = "$env:ProgramFiles\Ocentra Parent\Agent",
  [switch] $RemoveFiles
)

$ErrorActionPreference = 'Stop'

$ServiceId = 'OcentraParentAgent'
$WrapperPath = Join-Path $InstallDirectory 'OcentraParentAgentService.exe'
$service = Get-Service -Name $ServiceId -ErrorAction SilentlyContinue

if ($null -ne $service) {
  if ($service.Status -ne 'Stopped') {
    Stop-Service -Name $ServiceId -Force -ErrorAction Stop
    $service.WaitForStatus('Stopped', [TimeSpan]::FromSeconds(30))
  }

  if (Test-Path -LiteralPath $WrapperPath) {
    & $WrapperPath uninstall
    if ($LASTEXITCODE -ne 0) {
      throw "WinSW command failed: uninstall"
    }
  } else {
    & sc.exe delete $ServiceId | Out-Null
  }
}

if ($RemoveFiles -and (Test-Path -LiteralPath $InstallDirectory)) {
  Remove-Item -LiteralPath $InstallDirectory -Recurse -Force
}

Write-Host "Uninstalled $ServiceId"
