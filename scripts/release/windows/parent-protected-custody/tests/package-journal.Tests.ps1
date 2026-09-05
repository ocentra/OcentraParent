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
        throw "$Description did not reject the invalid journal state."
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
    return [ordered]@{
        brokerBinarySha256 = $hash
        provisionerBinarySha256 = $hash
        sourceHashes = [ordered]@{ 'parent-protected-custody.wxs' = $hash }
        anchoredInputHashes = [ordered]@{ 'Cargo.lock' = $hash }
        commandFingerprints = [ordered]@{ dotnet = 'C:\toolchain\dotnet.exe||C:\toolchain\dotnet.exe|' + $hash }
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

$temporaryRoot = Join-Path ([System.IO.Path]::GetTempPath()) ('ocentra-parent-wp12-journal-' + [Guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path $temporaryRoot | Out-Null

try {
    $packageFilesystemRoot = Join-Path $temporaryRoot 'package-root'
    $outputParent = Join-Path $packageFilesystemRoot 'published'
    $outputName = '1.2.3'
    $operationId = '0123456789abcdef0123456789abcdef'
    $outputRoot = Join-Path $outputParent $outputName
    $stagingRoot = Join-Path $outputParent "$outputName.staging.$operationId"
    $backupRoot = Join-Path $outputParent "$outputName.backup.$operationId"
    New-Item -ItemType Directory -Path $packageFilesystemRoot | Out-Null
    New-Item -ItemType Directory -Path $outputParent | Out-Null
    New-Item -ItemType Directory -Path $stagingRoot | Out-Null

    $inputContract = Get-PackagePublicationInputContract -InputContract (New-TestInputContract)
    $msiFileName = 'package.msi'
    $checksumFileName = 'package.msi.sha256'
    $manifestFileName = 'package.manifest.json'
    $artifactNames = @($msiFileName, $checksumFileName, $manifestFileName)
    $msiPath = Join-Path $stagingRoot $msiFileName
    [System.IO.File]::WriteAllText($msiPath, 'real staged package payload fixture', [System.Text.UTF8Encoding]::new($false))
    $msiHash = Get-Sha256Hex -Path $msiPath
    [System.IO.File]::WriteAllText(
        (Join-Path $stagingRoot $checksumFileName),
        $msiHash + ' *' + $msiFileName + [Environment]::NewLine,
        [System.Text.UTF8Encoding]::new($false)
    )
    $manifest = New-TestManifest -MsiName $msiFileName -ChecksumName $checksumFileName -MsiHash $msiHash -InputContract $inputContract
    Write-DeterministicJson -Path (Join-Path $stagingRoot $manifestFileName) -Value $manifest -Root $packageFilesystemRoot

    $stagedHashes = Get-PackageDirectoryHashes -Directory $stagingRoot -PackageRoot $packageFilesystemRoot -ExpectedNames $artifactNames -Description 'journal staged package fixture'
    $stagedSemanticHash = Get-PackageManifestSemanticHashFromFile -Path (Join-Path $stagingRoot $manifestFileName) -PackageRoot $packageFilesystemRoot
    $state = [ordered]@{
        phase = 'prepared'
        operationId = $operationId
        outputRoot = $outputRoot
        stagingRoot = $stagingRoot
        backupRoot = $backupRoot
        artifactNames = $artifactNames
        hadPrevious = $false
        stagedHashes = $stagedHashes
        previousHashes = [ordered]@{}
        stagedManifestSemanticSha256 = $stagedSemanticHash
        previousManifestSemanticSha256 = ''
        inputContract = $inputContract
    }

    $names = Get-PackagePublicationNames -OutputRoot $outputRoot
    $lockStream = Acquire-PackagePublicationLock -PackageRoot $packageFilesystemRoot -LockPath $names.LockPath
    try {
        Write-PackagePublicationJournal -PackageRoot $packageFilesystemRoot -JournalPath $names.JournalPath -State $state -LockPath $names.LockPath -LockStream $lockStream
        $records = @(Read-PackagePublicationJournalRecords -PackageRoot $packageFilesystemRoot -JournalPath $names.JournalPath)
        Assert-Equal $records.Count 1 'initial journal record count'
        Assert-Equal $records[0].schema 4 'journal schema v4'
        Assert-Equal $records[0].phase 'prepared' 'initial journal phase'
        Assert-Equal $records[0].sequence 1 'initial journal sequence'
        $journalFields = @($records[0].PSObject.Properties.Name | Sort-Object) -join '|'
        $expectedFields = @(
            'artifactNames',
            'backupRoot',
            'hadPrevious',
            'inputContract',
            'operationId',
            'outputRoot',
            'phase',
            'previousHashes',
            'previousManifestSemanticSha256',
            'previousRecordHash',
            'recordHash',
            'schema',
            'sequence',
            'stagedHashes',
            'stagedManifestSemanticSha256',
            'stagingRoot'
        ) -join '|'
        Assert-Equal $journalFields $expectedFields 'journal exact schema-v4 fields'

        $journalText = [System.IO.File]::ReadAllText($names.JournalPath, [System.Text.UTF8Encoding]::new($false))
        $mutatedJournalText = $journalText.Replace('"phase":"prepared"', '"phase":"prepared-mutated"')
        if ($mutatedJournalText -ceq $journalText) {
            throw 'journal mutation fixture did not change the durable phase bytes.'
        }
        [System.IO.File]::WriteAllText($names.JournalPath, $mutatedJournalText, [System.Text.UTF8Encoding]::new($false))
        Assert-Throws {
            Read-PackagePublicationJournalRecords -PackageRoot $packageFilesystemRoot -JournalPath $names.JournalPath
        } 'journal record mutation' 'record digest|unsupported'
        if (-not (Test-Path -LiteralPath $names.JournalPath -PathType Leaf)) {
            throw 'journal mutation rejection removed durable bytes.'
        }
        [System.IO.File]::WriteAllText($names.JournalPath, $journalText, [System.Text.UTF8Encoding]::new($false))

        $badSchema = $records[0] | ConvertTo-Json -Depth 32 -Compress | ConvertFrom-Json
        $badSchema.schema = 3
        Assert-Throws {
            Assert-PackagePublicationJournalShape -PackageRoot $packageFilesystemRoot -Journal $badSchema
        } 'journal schema downgrade' 'schema.*unsupported'

        $state.phase = 'staging-moved'
        Write-PackagePublicationJournal -PackageRoot $packageFilesystemRoot -JournalPath $names.JournalPath -State $state -LockPath $names.LockPath -LockStream $lockStream
        $records = @(Read-PackagePublicationJournalRecords -PackageRoot $packageFilesystemRoot -JournalPath $names.JournalPath)
        Assert-Equal $records.Count 2 'append-only journal record count'
        Assert-Equal $records[1].sequence 2 'append-only journal sequence'
        Assert-Equal $records[1].previousRecordHash $records[0].recordHash 'journal previous-record digest link'
        Assert-PackagePublicationJournalChain -PackageRoot $packageFilesystemRoot -Records $records | Out-Null

        Recover-PackagePublication -PackageRoot $packageFilesystemRoot -OutputRoot $outputRoot -LockPath $names.LockPath -LockStream $lockStream
        if (-not (Test-Path -LiteralPath $outputRoot -PathType Container)) {
            throw 'recovery did not promote the validated staged package directory.'
        }
        if (Test-Path -LiteralPath $stagingRoot) {
            throw 'recovery left the staged package directory after promotion.'
        }
        if (Test-Path -LiteralPath $names.JournalPath) {
            throw 'recovery left a completed publication journal.'
        }
        Assert-PackageDirectoryHashes -Directory $outputRoot -PackageRoot $packageFilesystemRoot -ExpectedNames $artifactNames -ExpectedHashes $stagedHashes -ExpectedManifestSemanticHash $stagedSemanticHash -Description 'recovered package fixture'
    } finally {
        $lockStream.Dispose()
    }
} finally {
    if (Test-Path -LiteralPath $temporaryRoot) {
        [System.IO.Directory]::Delete($temporaryRoot, $true)
    }
}

Write-Output 'PASS: Parent WP12 schema-v4 journal chaining, mutation rejection, and staged-package recovery.'
