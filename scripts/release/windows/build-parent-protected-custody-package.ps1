#requires -Version 7.2

[CmdletBinding()]
param(
    [Parameter()]
    [string]$Version,

    [Parameter()]
    [string]$OutputRoot
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$scriptRoot = (Resolve-Path -LiteralPath $PSScriptRoot).Path
$repoRoot = (Resolve-Path -LiteralPath (Join-Path $scriptRoot '..\..\..')).Path
$packageRoot = Join-Path $repoRoot 'target\release-packages'
if ([string]::IsNullOrWhiteSpace($OutputRoot)) {
    $OutputRoot = Join-Path $packageRoot 'parent-protected-custody'
} elseif (-not [System.IO.Path]::IsPathRooted($OutputRoot)) {
    $OutputRoot = Join-Path $repoRoot $OutputRoot
}
$OutputRoot = [System.IO.Path]::GetFullPath($OutputRoot).TrimEnd('\')

$helperPath = Join-Path $scriptRoot 'parent-protected-custody\build-package.ps1'
if (-not (Test-Path -LiteralPath $helperPath -PathType Leaf)) {
    throw "Routed package helper '$helperPath' is absent; refusing to package."
}

$helperArguments = @{
    Version = $Version
    OutputRoot = $OutputRoot
    RepoRoot = $repoRoot
    OrchestratorPath = $PSCommandPath
}
& $helperPath @helperArguments
if ($LASTEXITCODE -ne 0) {
    throw "Routed package helper failed with exit code $LASTEXITCODE."
}
