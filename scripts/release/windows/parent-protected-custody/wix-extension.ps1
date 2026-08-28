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

function Get-WixPayloadFileHashes {
    param(
        [Parameter(Mandatory)]
        [string]$Root,

        [Parameter(Mandatory)]
        [string]$Description
    )

    $fullRoot = [System.IO.Path]::GetFullPath($Root).TrimEnd('\')
    if (-not (Test-Path -LiteralPath $fullRoot -PathType Container)) {
        throw "$Description root '$fullRoot' is absent; refusing unverifiable WiX provenance."
    }
    Assert-NoPackageReparseChain -Path $fullRoot -Description $Description
    $files = @(Get-ChildItem -LiteralPath $fullRoot -File -Recurse -Force | Sort-Object FullName)
    if ($files.Count -eq 0) {
        throw "$Description root '$fullRoot' contains no payload files; refusing unverifiable WiX provenance."
    }
    $hashes = [ordered]@{}
    foreach ($file in $files) {
        if (($file.Attributes -band [System.IO.FileAttributes]::ReparsePoint) -ne 0) {
            throw "$Description payload '$($file.FullName)' is a reparse point; refusing unverifiable WiX provenance."
        }
        Assert-NoPackageReparseChain -Path $file.FullName -Description $Description
        $relative = [System.IO.Path]::GetRelativePath($fullRoot, $file.FullName)
        if ([System.IO.Path]::IsPathRooted($relative) -or $relative.StartsWith('..\', [System.StringComparison]::Ordinal) -or
            $relative.Equals('..', [System.StringComparison]::Ordinal)) {
            throw "$Description payload '$($file.FullName)' escaped its payload root; refusing unverifiable WiX provenance."
        }
        $hashes[$relative] = (Get-FileHash -LiteralPath $file.FullName -Algorithm SHA256).Hash.ToLowerInvariant()
    }
    return $hashes
}

function Get-WixPayloadDigest {
    param(
        [Parameter(Mandatory)]
        [object]$Hashes
    )

    $canonical = [ordered]@{}
    foreach ($name in @($Hashes.Keys | ForEach-Object { [string]$_ } | Sort-Object)) {
        $canonical[$name] = [string]$Hashes[$name]
    }
    $json = $canonical | ConvertTo-Json -Depth 16 -Compress
    $bytes = [System.Text.UTF8Encoding]::new($false).GetBytes($json)
    return ([System.Security.Cryptography.SHA256]::HashData($bytes) | ForEach-Object { $_.ToString('x2') }) -join ''
}

function Get-WixGlobalPackagesRoot {
    param(
        [Parameter(Mandatory)]
        [string]$DotnetCommand
    )

    Assert-TrustedCommandIntegrity -Command $DotnetCommand
    $output = @(Invoke-CheckedCommand -Command $DotnetCommand -ArgumentList @(
            'nuget',
            'locals',
            'global-packages',
            '--list'
        ) -FailureMessage 'WiX tool global package root discovery failed')
    $roots = @($output | ForEach-Object {
            $line = [string]$_
            if ($line -match '^\s*global-packages:\s*(?<root>.+?)\s*$') {
                [System.IO.Path]::GetFullPath($Matches['root'].Trim()).TrimEnd('\')
            }
        })
    if ($roots.Count -ne 1 -or [string]::IsNullOrWhiteSpace($roots[0])) {
        throw 'WiX tool global package root discovery returned an unexpected output shape; refusing unverifiable tool provenance.'
    }
    return $roots[0]
}

function Get-WixToolchainProvenance {
    param(
        [Parameter(Mandatory)]
        [string]$DotnetCommand,

        [Parameter(Mandatory)]
        [string]$RepoRoot
    )

    $requiredVersion = '6.0.2'
    $extensionVersions = @(Get-WixUtilExtensionVersions -DotnetCommand $DotnetCommand)
    if ($extensionVersions.Count -ne 1 -or $extensionVersions[0] -cne $requiredVersion) {
        throw "WiX Util extension provenance requires exactly version $requiredVersion, found '$($extensionVersions -join ', ')'."
    }
    $globalPackagesRoot = Get-WixGlobalPackagesRoot -DotnetCommand $DotnetCommand
    if (-not (Test-Path -LiteralPath $globalPackagesRoot -PathType Container)) {
        throw "WiX global package root '$globalPackagesRoot' is absent; refusing unverifiable tool provenance."
    }
    Assert-NoPackageReparseChain -Path $globalPackagesRoot -Description 'WiX global package root'
    $wixToolRoot = Join-Path $globalPackagesRoot 'wix\6.0.2'
    $extensionRoot = Join-Path ([System.IO.Path]::GetFullPath($RepoRoot).TrimEnd('\')) '.wix\extensions\WixToolset.Util.wixext\6.0.2'
    $toolHashes = Get-WixPayloadFileHashes -Root $wixToolRoot -Description 'Restored WiX tool payload'
    $extensionHashes = Get-WixPayloadFileHashes -Root $extensionRoot -Description 'Restored WiX Util extension payload'
    return [ordered]@{
        wixTool = [ordered]@{
            packageId = 'wix'
            version = $requiredVersion
            packageRoot = $wixToolRoot
            fileHashes = $toolHashes
            contentSha256 = Get-WixPayloadDigest -Hashes $toolHashes
        }
        wixUtilExtension = [ordered]@{
            extensionId = 'WixToolset.Util.wixext'
            version = $requiredVersion
            payloadRoot = $extensionRoot
            fileHashes = $extensionHashes
            contentSha256 = Get-WixPayloadDigest -Hashes $extensionHashes
        }
    }
}

function Assert-WixToolchainProvenanceSnapshot {
    param(
        [Parameter(Mandatory)]
        [object]$Expected,

        [Parameter(Mandatory)]
        [string]$DotnetCommand,

        [Parameter(Mandatory)]
        [string]$RepoRoot
    )

    $actual = Get-WixToolchainProvenance -DotnetCommand $DotnetCommand -RepoRoot $RepoRoot
    $expectedJson = $Expected | ConvertTo-Json -Depth 32 -Compress
    $actualJson = $actual | ConvertTo-Json -Depth 32 -Compress
    if ($expectedJson -cne $actualJson) {
        throw 'Restored WiX tool or extension payload changed after its provenance snapshot; refusing to publish drifted package bytes.'
    }
}
