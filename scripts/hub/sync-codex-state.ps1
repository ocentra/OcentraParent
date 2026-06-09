[CmdletBinding()]
param(
  [string]$MachineId = $env:COMPUTERNAME,
  [string]$Remote = "origin",
  [switch]$Commit,
  [switch]$Push,
  [switch]$AllowMainPush
)

$ErrorActionPreference = "Stop"

function Invoke-Git {
  param([Parameter(Mandatory = $true)][string[]]$Args)
  & git @Args
  if ($LASTEXITCODE -ne 0) {
    throw "git $($Args -join ' ') failed"
  }
}

function Get-GitOutput {
  param([Parameter(Mandatory = $true)][string[]]$Args)
  $output = & git @Args 2>$null
  if ($LASTEXITCODE -ne 0) {
    return $null
  }
  return $output
}

function Normalize-MachineId {
  param([string]$Value)
  if ([string]::IsNullOrWhiteSpace($Value)) {
    return "unknown-pc"
  }
  return ($Value.ToLowerInvariant() -replace "[^a-z0-9_.-]", "-")
}

$repoRoot = (Get-GitOutput @("rev-parse", "--show-toplevel") | Select-Object -First 1)
if ([string]::IsNullOrWhiteSpace($repoRoot)) {
  throw "Not inside a Git checkout."
}
Set-Location $repoRoot

$branch = (Get-GitOutput @("rev-parse", "--abbrev-ref", "HEAD") | Select-Object -First 1)
$machine = Normalize-MachineId $MachineId
$machineDir = Join-Path $repoRoot ".hub/state/machines"
New-Item -ItemType Directory -Force $machineDir | Out-Null

$status = @(Get-GitOutput @("status", "--short"))
$head = (Get-GitOutput @("rev-parse", "HEAD") | Select-Object -First 1)
$machineState = [ordered]@{
  schema = "https://ocentra.ca/schemas/ocentra-parent-hub-machine-state.v1.json"
  version = 1
  machineId = $machine
  updatedAt = (Get-Date).ToUniversalTime().ToString("o")
  repoRoot = $repoRoot
  branch = $branch
  head = $head
  user = $env:USERNAME
  dirtySummary = $status
}
$machineState | ConvertTo-Json -Depth 6 | Set-Content -LiteralPath (Join-Path $machineDir "$machine.json") -Encoding utf8

Invoke-Git @("fetch", $Remote, "--prune")

$remoteBranch = "$Remote/$branch"
$counts = Get-GitOutput @("rev-list", "--left-right", "--count", "$remoteBranch...HEAD")
if ($counts -match "^\s*(\d+)\s+(\d+)\s*$") {
  $behind = [int]$Matches[1]
  $ahead = [int]$Matches[2]
  if ($behind -gt 0) {
    $dirty = @(Get-GitOutput @("status", "--short"))
    if ($dirty.Count -gt 0) {
      Write-Host "hub-state-sync: branch is behind $remoteBranch and checkout is dirty; pull/rebase manually after reviewing local changes."
    } else {
      Invoke-Git @("pull", "--rebase", $Remote, $branch)
    }
  }
  Write-Host "hub-state-sync: branch=$branch behind=$behind ahead=$ahead"
} else {
  Write-Host "hub-state-sync: no remote tracking comparison for $remoteBranch"
}

if ($Commit) {
  $paths = @(
    ".hub/lane-ledger.json",
    ".hub/codex-rules.md",
    ".hub/state",
    "docs/hub",
    "docs/architecture/worktree-lanes.md",
    "AGENTS.md",
    "scripts/hub",
    "scripts/dev/hub-heartbeat-lib.mjs",
    "scripts/dev/hub-mailbox-lib.mjs",
    "scripts/dev/worktree-lanes-lib.mjs",
    "package.json"
  )
  $addArgs = @("add", "--") + $paths
  Invoke-Git $addArgs
  $staged = Get-GitOutput @("diff", "--cached", "--name-only")
  if ($staged -and $staged.Count -gt 0) {
    Invoke-Git @("commit", "-m", "chore(hub): sync codex state from $machine")
  } else {
    Write-Host "hub-state-sync: no staged hub state changes to commit"
  }
}

if ($Push) {
  if ($branch -eq "main" -and -not $AllowMainPush) {
    throw "Refusing to push main. Use a codex branch or pass -AllowMainPush only for an explicit user-approved exception."
  }
  Invoke-Git @("push", $Remote, $branch)
}

Write-Host "hub-state-sync-ok: machine=$machine branch=$branch"
