[CmdletBinding()]
param(
  [string[]]$SearchRoots = @("E:\", "D:\", "C:\Users\$env:USERNAME"),
  [string]$Repo = "ocentra/OcentraParent",
  [switch]$Json
)

$ErrorActionPreference = "Continue"

function Invoke-Git {
  param(
    [Parameter(Mandatory = $true)][string]$Path,
    [Parameter(Mandatory = $true)][string[]]$Args
  )

  $output = & git -C $Path @Args 2>$null
  if ($LASTEXITCODE -ne 0) {
    return $null
  }

  return $output
}

function Get-FirstLine {
  param([object]$Value)

  if ($null -eq $Value) {
    return $null
  }

  if ($Value -is [array]) {
    return ($Value | Select-Object -First 1)
  }

  return [string]$Value
}

function Test-GhAvailable {
  $command = Get-Command gh -ErrorAction SilentlyContinue
  return $null -ne $command
}

function Get-PrForBranch {
  param(
    [string]$Branch,
    [bool]$GhAvailable
  )

  if (-not $GhAvailable -or [string]::IsNullOrWhiteSpace($Branch)) {
    return $null
  }

  $jsonText = & gh pr list --repo $Repo --head $Branch --state open --json number,title,url,mergeStateStatus 2>$null
  if ($LASTEXITCODE -ne 0 -or [string]::IsNullOrWhiteSpace($jsonText)) {
    return $null
  }

  try {
    $items = $jsonText | ConvertFrom-Json
    return $items | Select-Object -First 1
  } catch {
    return $null
  }
}

function Get-LaneStatus {
  param(
    [string[]]$ShortStatus,
    [string]$Upstream,
    [int]$Ahead,
    [object]$Pr
  )

  if ($ShortStatus -and $ShortStatus.Count -gt 0) {
    return "dirty-local"
  }

  if ($Ahead -gt 0 -and [string]::IsNullOrWhiteSpace($Upstream)) {
    return "local-unpushed"
  }

  if ($Pr) {
    return "open-pr"
  }

  if ($Ahead -gt 0) {
    return "remote-branch-no-pr"
  }

  return "stale-preserve"
}

$ghAvailable = Test-GhAvailable
$results = New-Object System.Collections.Generic.List[object]

foreach ($root in $SearchRoots) {
  if (-not (Test-Path -LiteralPath $root)) {
    continue
  }

  $rootItem = Get-Item -LiteralPath $root -ErrorAction SilentlyContinue
  $childDirs = Get-ChildItem -LiteralPath $root -Directory -Recurse -ErrorAction SilentlyContinue
  $candidateDirs = @($rootItem) + @($childDirs) |
    Where-Object { $null -ne $_ -and $_.PSIsContainer -and (Test-Path -LiteralPath (Join-Path $_.FullName ".git")) }

  foreach ($dir in $candidateDirs) {
    $path = $dir.FullName
    $topLevel = Get-FirstLine (Invoke-Git -Path $path -Args @("rev-parse", "--show-toplevel"))
    if ([string]::IsNullOrWhiteSpace($topLevel)) {
      continue
    }

    $remotes = Invoke-Git -Path $path -Args @("remote", "-v")
    $remoteText = if ($remotes) { ($remotes -join "`n") } else { "" }
    if ($remoteText -notmatch [regex]::Escape($Repo)) {
      continue
    }

    $branch = Get-FirstLine (Invoke-Git -Path $path -Args @("branch", "--show-current"))
    $lastCommit = Get-FirstLine (Invoke-Git -Path $path -Args @("log", "-1", "--format=%H %s"))
    $shortStatus = @(Invoke-Git -Path $path -Args @("status", "--short"))
    $upstream = Get-FirstLine (Invoke-Git -Path $path -Args @("rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}"))
    $counts = Get-FirstLine (Invoke-Git -Path $path -Args @("rev-list", "--left-right", "--count", "origin/main...HEAD"))
    $ahead = 0
    $behind = 0
    if ($counts -match "^\s*(\d+)\s+(\d+)\s*$") {
      $behind = [int]$Matches[1]
      $ahead = [int]$Matches[2]
    }

    $commitsNotInMain = @(Invoke-Git -Path $path -Args @("log", "--oneline", "origin/main..HEAD", "-10"))
    $pr = Get-PrForBranch -Branch $branch -GhAvailable $ghAvailable
    $classification = Get-LaneStatus -ShortStatus $shortStatus -Upstream $upstream -Ahead $ahead -Pr $pr

    $results.Add([pscustomobject]@{
      path = $topLevel
      branch = $branch
      upstream = $upstream
      classification = $classification
      pr = if ($pr) { [pscustomobject]@{ number = $pr.number; title = $pr.title; url = $pr.url; mergeStateStatus = $pr.mergeStateStatus } } else { $null }
      lastCommit = $lastCommit
      aheadOfOriginMain = $ahead
      behindOriginMain = $behind
      dirtyLocalFiles = $shortStatus
      commitsNotInMain = $commitsNotInMain
    }) | Out-Null
  }
}

$uniqueResults = $results |
  Sort-Object path -Unique |
  Sort-Object classification, branch, path

if ($Json) {
  $uniqueResults | ConvertTo-Json -Depth 8
  exit 0
}

foreach ($item in $uniqueResults) {
  Write-Host ""
  Write-Host "==============================="
  Write-Host $item.path
  Write-Host "==============================="
  Write-Host "branch: $($item.branch)"
  Write-Host "upstream: $($item.upstream)"
  Write-Host "classification: $($item.classification)"
  if ($item.pr) {
    Write-Host "pr: #$($item.pr.number) $($item.pr.title) [$($item.pr.mergeStateStatus)]"
    Write-Host "pr_url: $($item.pr.url)"
  } else {
    Write-Host "pr: none detected"
  }
  Write-Host "last_commit: $($item.lastCommit)"
  Write-Host "origin_main: behind=$($item.behindOriginMain) ahead=$($item.aheadOfOriginMain)"

  Write-Host "dirty_local_files:"
  if ($item.dirtyLocalFiles -and $item.dirtyLocalFiles.Count -gt 0) {
    $item.dirtyLocalFiles | ForEach-Object { Write-Host "  $_" }
  } else {
    Write-Host "  none"
  }

  Write-Host "commits_not_in_main:"
  if ($item.commitsNotInMain -and $item.commitsNotInMain.Count -gt 0) {
    $item.commitsNotInMain | ForEach-Object { Write-Host "  $_" }
  } else {
    Write-Host "  none"
  }
}
