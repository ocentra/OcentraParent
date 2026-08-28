#requires -Version 7.2

[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$packageTestRoot = (Resolve-Path $PSScriptRoot).Path
$packageRoot = (Resolve-Path (Join-Path $packageTestRoot '..')).Path

. (Join-Path $packageRoot 'package-inputs.ps1')
. (Join-Path $packageRoot 'package-path-safety.ps1')
. (Join-Path $packageRoot 'wix-extension.ps1')
. (Join-Path $packageRoot 'package-publication.ps1')

function Assert-Equal {
    param(
        [Parameter(Mandatory)]
        [object]$Actual,

        [Parameter(Mandatory)]
        [object]$Expected,

        [Parameter(Mandatory)]
        [string]$Description
    )

    if ($Actual -cne $Expected) {
        throw "$Description was '$Actual'; expected '$Expected'."
    }
}

function Assert-Throws {
    param(
        [Parameter(Mandatory)]
        [scriptblock]$Action,

        [Parameter(Mandatory)]
        [string]$Description,

        [Parameter()]
        [string]$MessagePattern
    )

    $caught = $null
    try {
        & $Action | Out-Null
    } catch {
        $caught = $_
    }
    if ($null -eq $caught) {
        throw "$Description did not reject the invalid state."
    }
    if (-not [string]::IsNullOrWhiteSpace($MessagePattern) -and
        ([string]$caught.Exception.Message -notmatch $MessagePattern)) {
        throw "$Description rejected with '$($caught.Exception.Message)', not '$MessagePattern'."
    }
}

function New-TestInputContract {
    $hash = (('ab' * 32) -join '')
    $toolHashes = [ordered]@{ 'wix.dll' = $hash }
    $extensionHashes = [ordered]@{ 'WixToolset.Util.wixext.dll' = $hash }
    $fingerprint = 'C:\toolchain\dotnet.exe||C:\toolchain\dotnet.exe|' + $hash
    return [ordered]@{
        brokerBinarySha256 = $hash
        provisionerBinarySha256 = $hash
        sourceHashes = [ordered]@{ 'parent-protected-custody.wxs' = $hash }
        anchoredInputHashes = [ordered]@{ 'Cargo.lock' = $hash }
        commandFingerprints = [ordered]@{ dotnet = $fingerprint }
        toolchainProvenance = [ordered]@{
            wixTool = [ordered]@{
                packageId = 'wix'
                version = '6.0.2'
                packageRoot = 'C:\Users\test\.nuget\packages\wix\6.0.2'
                fileHashes = $toolHashes
                contentSha256 = Get-WixPayloadDigest -Hashes $toolHashes
            }
            wixUtilExtension = [ordered]@{
                extensionId = 'WixToolset.Util.wixext'
                version = '6.0.2'
                payloadRoot = 'C:\repo\.wix\extensions\WixToolset.Util.wixext\6.0.2'
                fileHashes = $extensionHashes
                contentSha256 = Get-WixPayloadDigest -Hashes $extensionHashes
            }
        }
    }
}

function New-TestManifest {
    param(
        [Parameter(Mandatory)]
        [string]$MsiName,

        [Parameter(Mandatory)]
        [string]$ChecksumName,

        [Parameter(Mandatory)]
        [string]$MsiHash,

        [Parameter(Mandatory)]
        [object]$InputContract
    )

    $manifest = [ordered]@{
        schema = 'ocentra-parent-protected-custody-package/v1'
        packageId = 'ocentra-parent-protected-custody'
        productVersion = '1.2.3'
        architecture = 'x64'
        artifact = [ordered]@{
            file = $MsiName
            sha256 = $MsiHash
            checksumFile = $ChecksumName
        }
        reproducibility = [ordered]@{ mode = 'deterministic' }
        publicationIntegrity = [ordered]@{ journalSchema = 'append-only-v4' }
        inputs = [ordered]@{ owner = 'package-builder' }
        inputIntegrity = $InputContract
        digestBinding = [ordered]@{ artifact = 'sha256'; authority = 'external-wp02' }
        fixedIdentity = [ordered]@{ upgradeCode = 'A1BA5AA2-F5DB-4B97-9889-4BB4DBF52B3C' }
        lifecycle = [ordered]@{
            install = 'elevated package transaction'
            repair = 'repeat fixed zero-argument provisioner boundary'
            upgrade = 'bounded major upgrade'
            rollback = 'preserve ambiguous bytes'
            uninstall = 'package-owned service and binaries only'
            deprovisioning = 'manual-required external owner'
        }
        protectedBoundary = [ordered]@{ enrollment = 'external WP02 owner ceremony' }
        source = [ordered]@{ production = 'fixed Rust package closure' }
        manifestIntegrity = [ordered]@{
            schema = 'canonical-json-v1'
            semanticSha256 = ''
        }
    }
    $semanticHash = Get-PackageManifestSemanticHash -Manifest $manifest
    $manifest.manifestIntegrity.semanticSha256 = $semanticHash
    return $manifest
}

$temporaryRoot = Join-Path ([System.IO.Path]::GetTempPath()) ('ocentra-parent-wp12-publication-' + [Guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path $temporaryRoot | Out-Null

try {
    $packageFilesystemRoot = Join-Path $temporaryRoot 'package-root'
    $outputParent = Join-Path $packageFilesystemRoot 'published'
    $artifactDirectory = Join-Path $packageFilesystemRoot 'staging'
    New-Item -ItemType Directory -Path $packageFilesystemRoot | Out-Null
    New-Item -ItemType Directory -Path $outputParent | Out-Null
    New-Item -ItemType Directory -Path $artifactDirectory | Out-Null

    $inputContract = New-TestInputContract
    $normalizedInputContract = Get-PackagePublicationInputContract -InputContract $inputContract
    Assert-Equal $normalizedInputContract.brokerBinarySha256 (('ab' * 32) -join '') 'normalized broker input hash'
    Assert-Equal $normalizedInputContract.toolchainProvenance.wixTool.version '6.0.2' 'normalized pinned WiX tool version'

    $manifestFileName = 'package.manifest.json'
    $msiFileName = 'package.msi'
    $checksumFileName = 'package.msi.sha256'
    $artifactNames = @($msiFileName, $checksumFileName, $manifestFileName)
    $msiPath = Join-Path $artifactDirectory $msiFileName
    [System.IO.File]::WriteAllText($msiPath, 'real publication payload fixture', [System.Text.UTF8Encoding]::new($false))
    $msiHash = Get-Sha256Hex -Path $msiPath
    $checksumPath = Join-Path $artifactDirectory $checksumFileName
    [System.IO.File]::WriteAllText(
        $checksumPath,
        $msiHash + ' *' + $msiFileName + [Environment]::NewLine,
        [System.Text.UTF8Encoding]::new($false)
    )
    $manifest = New-TestManifest -MsiName $msiFileName -ChecksumName $checksumFileName -MsiHash $msiHash -InputContract $normalizedInputContract
    $manifestPath = Join-Path $artifactDirectory $manifestFileName
    Write-DeterministicJson -Path $manifestPath -Value $manifest -Root $packageFilesystemRoot

    $stagedHashes = Get-PackageDirectoryHashes -Directory $artifactDirectory -PackageRoot $packageFilesystemRoot -ExpectedNames $artifactNames -Description 'publication fixture'
    $semanticHash = Get-PackageManifestSemanticHashFromFile -Path $manifestPath -PackageRoot $packageFilesystemRoot
    Assert-PackageManifestBinding -Directory $artifactDirectory -ExpectedNames $artifactNames -Hashes $stagedHashes -ExpectedManifestSemanticHash $semanticHash -Description 'publication fixture'
    Assert-PackageManifestInputContract -Directory $artifactDirectory -ExpectedNames $artifactNames -ExpectedInputContract $normalizedInputContract -Description 'publication fixture'

    $mutatedManifest = [System.IO.File]::ReadAllText($manifestPath, [System.Text.UTF8Encoding]::new($false)) | ConvertFrom-Json
    $mutatedManifest.lifecycle.install = 'caller-supplied authority'
    [System.IO.File]::WriteAllText(
        $manifestPath,
        ($mutatedManifest | ConvertTo-Json -Depth 32 -Compress),
        [System.Text.UTF8Encoding]::new($false)
    )
    Assert-Throws {
        Get-PackageManifestSemanticHashFromFile -Path $manifestPath -PackageRoot $packageFilesystemRoot
    } 'semantic manifest mutation' 'declared semanticSha256 does not match'
    [System.IO.File]::WriteAllText(
        $manifestPath,
        (($manifest | ConvertTo-Json -Depth 32) + "`n"),
        [System.Text.UTF8Encoding]::new($false)
    )

    $mutatedInputManifest = [System.IO.File]::ReadAllText($manifestPath, [System.Text.UTF8Encoding]::new($false)) | ConvertFrom-Json
    $mutatedInputManifest.inputIntegrity.brokerBinarySha256 = (('cd' * 32) -join '')
    [System.IO.File]::WriteAllText(
        $manifestPath,
        (($mutatedInputManifest | ConvertTo-Json -Depth 32) + "`n"),
        [System.Text.UTF8Encoding]::new($false)
    )
    Assert-Throws {
        Assert-PackageManifestInputContract -Directory $artifactDirectory -ExpectedNames $artifactNames -ExpectedInputContract $normalizedInputContract -Description 'mutated publication fixture'
    } 'manifest input-integrity mutation' 'does not exactly match the publication input contract'

    $names = Get-PackagePublicationNames -OutputRoot (Join-Path $outputParent '1.2.3')
    $nonExclusiveStream = [System.IO.FileStream]::new(
        $names.LockPath,
        [System.IO.FileMode]::OpenOrCreate,
        [System.IO.FileAccess]::ReadWrite,
        [System.IO.FileShare]::ReadWrite
    )
    try {
        Assert-Throws {
            Assert-HeldPackagePublicationLock -PackageRoot $packageFilesystemRoot -LockPath $names.LockPath -LockStream $nonExclusiveStream
        } 'non-exclusive publication lock' 'not exclusively held'
    } finally {
        $nonExclusiveStream.Dispose()
    }

    $lockStream = Acquire-PackagePublicationLock -PackageRoot $packageFilesystemRoot -LockPath $names.LockPath
    try {
        $safeLockPath = Assert-HeldPackagePublicationLock -PackageRoot $packageFilesystemRoot -LockPath $names.LockPath -LockStream $lockStream
        Assert-Equal $safeLockPath (Get-NormalizedPackagePath -Path $names.LockPath) 'live exclusive publication lock path'

        $otherPath = Join-Path $outputParent 'other.lock'
        [System.IO.File]::WriteAllText($otherPath, 'unrelated stream', [System.Text.UTF8Encoding]::new($false))
        $wrongStream = [System.IO.FileStream]::new(
            $otherPath,
            [System.IO.FileMode]::Open,
            [System.IO.FileAccess]::ReadWrite,
            [System.IO.FileShare]::ReadWrite
        )
        try {
            Assert-Throws {
                Assert-HeldPackagePublicationLock -PackageRoot $packageFilesystemRoot -LockPath $names.LockPath -LockStream $wrongStream
            } 'arbitrary readable lock stream' 'exact safe lock path'
        } finally {
            $wrongStream.Dispose()
        }
    } finally {
        $lockStream.Dispose()
    }
} finally {
    if (Test-Path -LiteralPath $temporaryRoot) {
        [System.IO.Directory]::Delete($temporaryRoot, $true)
    }
}

Write-Output 'PASS: Parent WP12 manifest binding, publication input integrity, and exclusive lock contracts.'
