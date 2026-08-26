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

function Invoke-CheckedCommand {
    param(
        [Parameter(Mandatory)]
        [string]$Command,

        [Parameter()]
        [string[]]$ArgumentList = @(),

        [Parameter(Mandatory)]
        [string]$FailureMessage
    )

    & $Command @ArgumentList
    if ($LASTEXITCODE -ne 0) {
        throw "$FailureMessage (exit code $LASTEXITCODE)."
    }
}

function Resolve-RequiredCommand {
    param(
        [Parameter(Mandatory)]
        [string]$Name
    )

    $command = Get-Command -Name $Name -ErrorAction SilentlyContinue
    if ($null -eq $command) {
        throw "Required build tool '$Name' is unavailable; refusing to produce a package."
    }
    return $command.Source
}

function Resolve-UnderRoot {
    param(
        [Parameter(Mandatory)]
        [string]$Path,

        [Parameter(Mandatory)]
        [string]$Root,

        [Parameter(Mandatory)]
        [string]$Description
    )

    $resolvedRoot = [System.IO.Path]::GetFullPath($Root).TrimEnd('\') + '\'
    $resolvedPath = [System.IO.Path]::GetFullPath($Path).TrimEnd('\')
    if (-not $resolvedPath.StartsWith($resolvedRoot, [System.StringComparison]::OrdinalIgnoreCase)) {
        throw "$Description '$resolvedPath' must remain under '$($resolvedRoot.TrimEnd('\'))'."
    }
    return $resolvedPath
}

function Get-Sha256Hex {
    param(
        [Parameter(Mandatory)]
        [string]$Path
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "Cannot hash missing file '$Path'."
    }
    return (Get-FileHash -LiteralPath $Path -Algorithm SHA256).Hash.ToLowerInvariant()
}

function Get-RegistryId {
    param(
        [Parameter(Mandatory)]
        [string]$CanonicalDatabasePath
    )

    # This mirrors protected-capability-custody-core registry_id exactly for
    # broker_admission::storage_path::fixed_database_identity_path:
    # UTF-16 code units are encoded big-endian, with a big-endian byte length
    # after the fixed domain. The path is an owner-approved package input, not
    # a caller-supplied executable or command-line value.
    $domainBytes = [System.Text.Encoding]::UTF8.GetBytes('ocentra.pcc.registry-path.v1')
    $canonicalBytes = [System.Text.Encoding]::BigEndianUnicode.GetBytes($CanonicalDatabasePath)
    $lengthBytes = [BitConverter]::GetBytes([uint32]$canonicalBytes.Length)
    [Array]::Reverse($lengthBytes)
    $payload = [byte[]]::new($domainBytes.Length + $lengthBytes.Length + $canonicalBytes.Length)
    [Array]::Copy($domainBytes, 0, $payload, 0, $domainBytes.Length)
    [Array]::Copy($lengthBytes, 0, $payload, $domainBytes.Length, $lengthBytes.Length)
    [Array]::Copy($canonicalBytes, 0, $payload, $domainBytes.Length + $lengthBytes.Length, $canonicalBytes.Length)
    return ([System.Security.Cryptography.SHA256]::HashData($payload) | ForEach-Object { $_.ToString('x2') }) -join ''
}

function Get-DeterministicGuid {
    param(
        [Parameter(Mandatory)]
        [string]$Seed
    )

    $bytes = [System.Security.Cryptography.SHA256]::HashData([System.Text.Encoding]::UTF8.GetBytes($Seed))
    # UUID version 5 / RFC 4122 variant keeps the value valid while making
    # the MSI ProductCode stable for an exact package identity and version.
    $bytes[6] = ($bytes[6] -band 0x0f) -bor 0x50
    $bytes[8] = ($bytes[8] -band 0x3f) -bor 0x80
    return ([Guid]::new([byte[]]$bytes[0..15])).ToString('B').ToUpperInvariant()
}

function Write-Utf8NoBom {
    param(
        [Parameter(Mandatory)]
        [string]$Path,

        [Parameter(Mandatory)]
        [string]$Content
    )

    [System.IO.File]::WriteAllText($Path, $Content, [System.Text.UTF8Encoding]::new($false))
}

function Write-DeterministicJson {
    param(
        [Parameter(Mandatory)]
        [string]$Path,

        [Parameter(Mandatory)]
        [object]$Value
    )

    $json = $Value | ConvertTo-Json -Depth 12
    Write-Utf8NoBom -Path $Path -Content ($json + "`n")
}

function Set-DeterministicMsiSummary {
    param(
        [Parameter(Mandatory)]
        [string]$Path,

        [Parameter(Mandatory)]
        [string]$PackageCode
    )

    # WiX 6.0.2 emits mutable SummaryInformation.PackageCode and current
    # Created/LastSaved metadata. Normalize the supported fields through the
    # Windows Installer COM API, then require a complete repeated-build byte
    # comparison below. This helper alone is not a reproducibility claim.
    $installer = $null
    $database = $null
    $summary = $null
    try {
        $installer = New-Object -ComObject WindowsInstaller.Installer
        $database = $installer.OpenDatabase((Resolve-Path -LiteralPath $Path).Path, 1)
        $summary = $database.SummaryInformation(20)
        $summary.Property(9) = $PackageCode
        $fixedDate = [DateTime]::ParseExact(
            '2000-01-01T00:00:00',
            'yyyy-MM-ddTHH:mm:ss',
            [Globalization.CultureInfo]::InvariantCulture
        )
        $summary.Property(12) = $fixedDate
        $summary.Property(13) = $fixedDate
        $summary.Persist()
        $database.Commit()
    } catch {
        throw "Windows Installer metadata normalization is unavailable; refusing to emit a non-deterministic MSI: $($_.Exception.Message)"
    } finally {
        foreach ($comObject in @($summary, $database, $installer)) {
            if ($null -ne $comObject) {
                [System.Runtime.InteropServices.Marshal]::FinalReleaseComObject($comObject) | Out-Null
            }
        }
    }
}

function Assert-ByteIdentical {
    param(
        [Parameter(Mandatory)]
        [string]$LeftPath,

        [Parameter(Mandatory)]
        [string]$RightPath
    )

    $left = [System.IO.File]::ReadAllBytes($LeftPath)
    $right = [System.IO.File]::ReadAllBytes($RightPath)
    if ($left.Length -ne $right.Length) {
        throw "WiX repeated-build outputs differ in length ($($left.Length) vs $($right.Length)); refusing to emit a non-deterministic MSI."
    }
    for ($index = 0; $index -lt $left.Length; $index++) {
        if ($left[$index] -ne $right[$index]) {
            throw "WiX repeated-build outputs differ at byte offset $index; the local toolchain cannot guarantee byte-for-byte MSI reproducibility, refusing to emit a package."
        }
    }
}

$scriptRoot = (Resolve-Path -LiteralPath $PSScriptRoot).Path
$repoRoot = (Resolve-Path -LiteralPath (Join-Path $scriptRoot '..\..\..')).Path
$wixSourcePath = Join-Path $scriptRoot 'parent-protected-custody.wxs'
$targetRoot = Join-Path $repoRoot 'target\release'
$packageRoot = Join-Path $repoRoot 'target\release-packages'

if ([string]::IsNullOrWhiteSpace($OutputRoot)) {
    $OutputRoot = Join-Path $packageRoot 'parent-protected-custody'
} elseif (-not [System.IO.Path]::IsPathRooted($OutputRoot)) {
    $OutputRoot = Join-Path $repoRoot $OutputRoot
}

$OutputRoot = [System.IO.Path]::GetFullPath($OutputRoot).TrimEnd('\')
$OutputRoot = Resolve-UnderRoot -Path $OutputRoot -Root $packageRoot -Description 'OutputRoot'
if ($OutputRoot.Equals([System.IO.Path]::GetFullPath($packageRoot).TrimEnd('\'), [System.StringComparison]::OrdinalIgnoreCase)) {
    throw "OutputRoot must identify a package-specific child under '$packageRoot'."
}

$cargoCommand = Resolve-RequiredCommand -Name 'cargo'
$dotnetCommand = Resolve-RequiredCommand -Name 'dotnet'
$nodeCommand = Resolve-RequiredCommand -Name 'node'

$policyVersionOutput = & $nodeCommand (Join-Path $repoRoot 'scripts\release\validate-version.mjs') '--print-version'
if ($LASTEXITCODE -ne 0) {
    throw 'Release version policy validation failed; refusing to build the package.'
}
$policyVersion = ($policyVersionOutput | Select-Object -Last 1).ToString().Trim()
if ([string]::IsNullOrWhiteSpace($policyVersion) -or $policyVersion -notmatch '^\d+\.\d+\.\d+$') {
    throw "Release version policy returned an invalid MSI version '$policyVersion'."
}
if ([string]::IsNullOrWhiteSpace($Version)) {
    $Version = $policyVersion
} elseif ($Version -ne $policyVersion) {
    throw "Requested package version '$Version' does not match the repository release version '$policyVersion'."
}

$brokerBinaryPath = Join-Path $targetRoot 'ocentra-protected-capability-custody-broker.exe'
$provisionerBinaryPath = Join-Path $targetRoot 'ocentra-protected-capability-custody-provisioner.exe'
$brokerInstallPath = 'C:\Program Files\Ocentra\OcentraParent\ocentra-protected-capability-custody-broker.exe'
$brokerDatabaseIdentityPath = 'C:\ProgramData\Ocentra\OcentraParent\protected-capability-custody\custody.sqlite'
$expectedRegistryId = '2cc753a30323ee51ee0301439996c5e4077fe49d3a31250ee75b32b6ecd1baf7'
$registryId = Get-RegistryId -CanonicalDatabasePath $brokerDatabaseIdentityPath
if ($registryId -cne $expectedRegistryId) {
    throw "Canonical database identity registry id '$registryId' does not match the owner-approved core/provisioner identity '$expectedRegistryId'; refusing to package."
}
$productCode = Get-DeterministicGuid -Seed "ocentra-parent-protected-custody/msi/$Version"

Invoke-CheckedCommand -Command $cargoCommand -ArgumentList @(
    'build',
    '--release',
    '--locked',
    '-p',
    'ocentra-protected-capability-custody-broker',
    '-p',
    'ocentra-protected-capability-custody-provisioner'
) -FailureMessage 'Protected broker/provisioner release build failed'

foreach ($requiredBinary in @($brokerBinaryPath, $provisionerBinaryPath)) {
    if (-not (Test-Path -LiteralPath $requiredBinary -PathType Leaf)) {
        throw "Required built binary '$requiredBinary' is absent; refusing to package."
    }
    if ((Get-Item -LiteralPath $requiredBinary).Length -le 0) {
        throw "Required built binary '$requiredBinary' is empty; refusing to package."
    }
}

$brokerHash = Get-Sha256Hex -Path $brokerBinaryPath
$provisionerHash = Get-Sha256Hex -Path $provisionerBinaryPath
$wixSourceHash = Get-Sha256Hex -Path $wixSourcePath
$buildScriptHash = Get-Sha256Hex -Path $PSCommandPath
$packageCode = Get-DeterministicGuid -Seed "ocentra-parent-protected-custody/package/$Version/$brokerHash/$provisionerHash/$wixSourceHash"

$wixIntermediateRoot = Join-Path $OutputRoot 'wix-obj'
$msiPath = Join-Path $OutputRoot "ocentra-parent-protected-custody-$Version-x64.msi"
$checksumPath = "$msiPath.sha256"
$manifestPath = Join-Path $OutputRoot "ocentra-parent-protected-custody-$Version-x64.manifest.json"

New-Item -ItemType Directory -Path $OutputRoot -Force | Out-Null
foreach ($generatedPath in @($wixIntermediateRoot)) {
    if (Test-Path -LiteralPath $generatedPath) {
        Remove-Item -LiteralPath $generatedPath -Recurse -Force
    }
    New-Item -ItemType Directory -Path $generatedPath -Force | Out-Null
}
foreach ($generatedFile in @($msiPath, $checksumPath, $manifestPath)) {
    if (Test-Path -LiteralPath $generatedFile -PathType Leaf) {
        Remove-Item -LiteralPath $generatedFile -Force
    }
}

$extensionList = (& $dotnetCommand wix extension list | Out-String)
if ($LASTEXITCODE -ne 0 -or $extensionList -notmatch 'WixToolset\.Util\.wixext\s+6\.0\.2') {
    Invoke-CheckedCommand -Command $dotnetCommand -ArgumentList @('wix', 'extension', 'add', 'WixToolset.Util.wixext/6.0.2') -FailureMessage 'WiX Util extension is unavailable'
}

$firstMsiPath = Join-Path $wixIntermediateRoot 'candidate-a.msi'
$secondMsiPath = Join-Path $wixIntermediateRoot 'candidate-b.msi'
$firstIntermediatePath = Join-Path $wixIntermediateRoot 'candidate-a'
$secondIntermediatePath = Join-Path $wixIntermediateRoot 'candidate-b'

function Invoke-WixCandidateBuild {
    param(
        [Parameter(Mandatory)]
        [string]$CandidateMsiPath,

        [Parameter(Mandatory)]
        [string]$CandidateIntermediatePath
    )

    New-Item -ItemType Directory -Path $CandidateIntermediatePath -Force | Out-Null
    Invoke-CheckedCommand -Command $dotnetCommand -ArgumentList @(
        'wix',
        'build',
        $wixSourcePath,
        '-ext',
        'WixToolset.Util.wixext',
        '-arch',
        'x64',
        '-d',
        "ProductVersion=$Version",
        '-d',
        "ProductCode=$productCode",
        '-d',
        "BrokerBinaryPath=$brokerBinaryPath",
        '-d',
        "ProvisionerBinaryPath=$provisionerBinaryPath",
        '-d',
        "BrokerDigestHex=$brokerHash",
        '-d',
        "RegistryId=$registryId",
        '-intermediatefolder',
        $CandidateIntermediatePath,
        '-out',
        $CandidateMsiPath,
        '-pdbtype',
        'none'
    ) -FailureMessage 'WiX protected custody package build failed'

    if (-not (Test-Path -LiteralPath $CandidateMsiPath -PathType Leaf) -or (Get-Item -LiteralPath $CandidateMsiPath).Length -le 0) {
        throw "WiX reported success without a non-empty MSI at '$CandidateMsiPath'."
    }

    Set-DeterministicMsiSummary -Path $CandidateMsiPath -PackageCode $packageCode
}

# WiX and the Windows Installer OLE container can carry metadata outside the
# MSI tables. Build twice with isolated intermediates and compare every byte;
# if the toolchain leaves any mutable container metadata, fail closed before a
# final MSI, checksum, or manifest is published.
Invoke-WixCandidateBuild -CandidateMsiPath $firstMsiPath -CandidateIntermediatePath $firstIntermediatePath
Invoke-WixCandidateBuild -CandidateMsiPath $secondMsiPath -CandidateIntermediatePath $secondIntermediatePath
Assert-ByteIdentical -LeftPath $firstMsiPath -RightPath $secondMsiPath
Copy-Item -LiteralPath $firstMsiPath -Destination $msiPath -Force

if (-not (Test-Path -LiteralPath $msiPath -PathType Leaf) -or (Get-Item -LiteralPath $msiPath).Length -le 0) {
    throw "Deterministic WiX comparison passed without a non-empty final MSI at '$msiPath'."
}

$msiHash = Get-Sha256Hex -Path $msiPath
Write-Utf8NoBom -Path $checksumPath -Content "$msiHash *$([System.IO.Path]::GetFileName($msiPath))`n"

$manifest = [ordered]@{
    schema = 'ocentra-parent-protected-custody-package/v1'
    packageId = 'ocentra-parent-protected-custody'
    productVersion = $Version
    architecture = 'x64'
    artifact = [ordered]@{
        file = [System.IO.Path]::GetFileName($msiPath)
        sha256 = $msiHash
        checksumFile = [System.IO.Path]::GetFileName($checksumPath)
    }
    reproducibility = [ordered]@{
        verification = 'two independent WiX builds compared byte-for-byte after MSI metadata normalization'
        byteIdenticalRepeatBuilds = $true
    }
    inputs = @(
        [ordered]@{ role = 'broker'; file = [System.IO.Path]::GetFileName($brokerBinaryPath); sha256 = $brokerHash },
        [ordered]@{ role = 'provisioner'; file = [System.IO.Path]::GetFileName($provisionerBinaryPath); sha256 = $provisionerHash }
    )
    fixedIdentity = [ordered]@{
        brokerInstallPath = $brokerInstallPath
        databaseIdentityPath = $brokerDatabaseIdentityPath
        registryIdBasis = 'protected-capability-custody-core fixed_database_identity_path and provisioner fixed registry identity'
        registryId = $registryId
        productCode = $productCode
        packageCode = $packageCode
        upgradeCode = 'A1BA5AA2-F5DB-4B97-9889-4BB4DBF52B3C'
        serviceName = 'OcentraProtectedCapabilityCustodyBroker'
        serviceAccount = 'LocalSystem'
        dataRoot = 'C:\ProgramData\Ocentra\OcentraParent\protected-capability-custody'
    }
    lifecycle = [ordered]@{
        install = 'Elevated per-machine install; zero-argument owner-approved provisioner is checked before broker service start.'
        repair = 'Re-runs the same fixed provisioner boundary without caller-supplied authority inputs.'
        upgrade = 'Major upgrade uses the fixed package identity and bounded MSI transaction; protected enrollment is not rewritten.'
        rollback = 'A failed major upgrade tears down the prior package before the new transaction; no older broker identity or protected registry/DACL/enrollment snapshot is restored.'
        uninstall = 'Stops/removes only the package-owned broker service and binaries; protected registry/data components are permanent.'
        deprovisioning = 'Manual-required external WP02 owner path; uninstall never invokes deprovisioning implicitly.'
    }
    protectedBoundary = [ordered]@{
        enrollment = 'External WP02 owner ceremony; package does not create or overwrite Enrollment\\authority-v1.'
        permanentEnrollmentRepair = 'manual-required: permanent TrustedInstaller-owned Enrollment\\authority-v1 remains external WP02 owner state.'
        tpmOwnerCeremony = 'External OEM/firmware/MDM owner ceremony; not packaged.'
        tpmAuthorizationSecret = 'Never packaged, registered, logged, or passed on the command line.'
        readiness = 'Not claimed by MSI success, checksum, signing status, or service registration.'
        signing = 'manual-required'
    }
    source = [ordered]@{
        wix = [System.IO.Path]::GetFileName($wixSourcePath)
        wixSha256 = $wixSourceHash
        buildScript = [System.IO.Path]::GetFileName($PSCommandPath)
        buildScriptSha256 = $buildScriptHash
    }
}
Write-DeterministicJson -Path $manifestPath -Value $manifest

$manifestText = [System.IO.File]::ReadAllText($manifestPath)
if ($manifestText -notmatch 'manual-required' -or $manifestText -notmatch 'External WP02 owner ceremony') {
    throw 'Generated package manifest lost the external/manual protected provisioning boundary.'
}

Write-Output "Built $msiPath"
Write-Output "SHA256 $msiHash"
Write-Output "Manifest $manifestPath"
