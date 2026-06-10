[CmdletBinding()]
param(
  [string]$MachineId = $env:COMPUTERNAME,
  [string]$Remote = "origin",
  [switch]$Commit,
  [switch]$Push,
  [switch]$AllowMainPush
)

$ErrorActionPreference = "Stop"

if ($Commit -or $Push -or $AllowMainPush) {
  throw "hub:state:sync no longer commits or pushes live hub state through OcentraParent. Use OcentraHub sync, or the configured legacy external hub root in .hub/hub.config.json during migration."
}

Write-Host "hub-state-sync-deprecated: product repo Git is no longer the live hub transport."
Write-Host "hub-state-sync-deprecated: use npm run hub:status and npm run lanes:status to inspect the configured external hub."
Write-Host "hub-state-sync-deprecated: target command after OcentraHub lands is ocentra-hub sync --hub ocentra-parent."
