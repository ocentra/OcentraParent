function Get-WixUtilExtensionVersions {
    param(
        [Parameter(Mandatory)]
        [string]$DotnetCommand
    )

    Assert-TrustedCommandIntegrity -Command $DotnetCommand
    $extensionOutput = @(& $DotnetCommand wix extension list 2>&1)
    $extensionExitCode = $LASTEXITCODE
    $outputLines = @($extensionOutput | ForEach-Object { [string]$_ })
    $contentLines = @($outputLines | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })

    # WiX 6.0.2 returns exit 2 with no output for a clean local extension
    # cache. That exact observed shape means zero installed extensions. Any
    # diagnostic text or other nonzero exit remains a hard failure.
    if ($extensionExitCode -eq 2 -and $outputLines.Count -eq 0) {
        return @()
    }
    if ($extensionExitCode -ne 0) {
        $diagnostic = if ($contentLines.Count -eq 0) { '<no output>' } else { $contentLines -join ' | ' }
        throw "Pinned WiX extension listing failed with exit code $extensionExitCode and output '$diagnostic'; refusing to package."
    }

    $utilVersions = [System.Collections.Generic.List[string]]::new()
    foreach ($line in $contentLines) {
        if ($line -notmatch '^\s*(?<id>[A-Za-z0-9_.-]+)\s+(?<version>\d+\.\d+\.\d+)\s*$') {
            throw "Pinned WiX extension listing has an unparseable row '$line'; refusing to accept a prefix or unanchored version."
        }
        if ($Matches['id'] -ceq 'WixToolset.Util.wixext') {
            $utilVersions.Add($Matches['version'])
        }
    }
    return @($utilVersions)
}

function Install-ExactWixUtilExtension {
    param(
        [Parameter(Mandatory)]
        [string]$DotnetCommand
    )

    $requiredVersion = '6.0.2'
    $extensionVersions = @(Get-WixUtilExtensionVersions -DotnetCommand $DotnetCommand)
    if ($extensionVersions.Count -eq 1 -and $extensionVersions[0] -ceq $requiredVersion) {
        return
    }
    if ($extensionVersions.Count -gt 0) {
        throw "Pinned WiX Util extension versions '$($extensionVersions -join ', ')' do not equal the required exact version $requiredVersion; refusing to continue."
    }

    Invoke-CheckedCommand -Command $DotnetCommand -ArgumentList @(
        'wix',
        'extension',
        'add',
        "WixToolset.Util.wixext/$requiredVersion"
    ) -FailureMessage 'WiX Util extension installation failed'
    $postAddVersions = @(Get-WixUtilExtensionVersions -DotnetCommand $DotnetCommand)
    if ($postAddVersions.Count -ne 1 -or $postAddVersions[0] -cne $requiredVersion) {
        throw "Post-add WiX Util extension verification found '$($postAddVersions -join ', ')', not exactly $requiredVersion; refusing to package."
    }
}
