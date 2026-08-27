#requires -Version 7.2

[CmdletBinding()]
param(
    [Parameter()]
    [string]$Version,

    [Parameter(Mandatory)]
    [string]$OutputRoot,

    [Parameter(Mandatory)]
    [string]$RepoRoot,

    [Parameter(Mandatory)]
    [string]$OrchestratorPath
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$helperRoot = (Resolve-Path -LiteralPath $PSScriptRoot).Path
$repoRootForTrust = [System.IO.Path]::GetFullPath($RepoRoot).TrimEnd('\')
$orchestratorPathForTrust = [System.IO.Path]::GetFullPath($OrchestratorPath)
$expectedRepoRootForTrust = [System.IO.Path]::GetFullPath((Join-Path $helperRoot '..\..\..\..')).TrimEnd('\')
$expectedOrchestratorPathForTrust = [System.IO.Path]::GetFullPath((Join-Path $helperRoot '..\build-parent-protected-custody-package.ps1'))
if (-not $repoRootForTrust.Equals($expectedRepoRootForTrust, [System.StringComparison]::OrdinalIgnoreCase)) {
    throw "Package helper repo root '$repoRootForTrust' is not the source repository root '$expectedRepoRootForTrust'; refusing an unanchored package invocation."
}
if (-not $orchestratorPathForTrust.Equals($expectedOrchestratorPathForTrust, [System.StringComparison]::OrdinalIgnoreCase)) {
    throw "Package orchestrator '$orchestratorPathForTrust' is not the trusted protected-custody launcher '$expectedOrchestratorPathForTrust'; refusing an unanchored package invocation."
}
$wixSourcePathForTrust = Join-Path $repoRootForTrust 'scripts\release\windows\parent-protected-custody.wxs'
$trustedSourcePaths = @(
    $orchestratorPathForTrust,
    $wixSourcePathForTrust,
    (Join-Path $helperRoot 'build-package.ps1'),
    (Join-Path $helperRoot 'msi-validation.ps1'),
    (Join-Path $helperRoot 'msi-contract.ps1'),
    (Join-Path $helperRoot 'package-inputs.ps1'),
    (Join-Path $helperRoot 'package-path-safety.ps1'),
    (Join-Path $helperRoot 'package-publication.ps1'),
    (Join-Path $helperRoot 'wix-extension.ps1')
)

function Get-TrustedSourceHash {
    param(
        [Parameter(Mandatory)]
        [string]$Path
    )

    return (Get-FileHash -LiteralPath $Path -Algorithm SHA256).Hash.ToLowerInvariant()
}

function Assert-TrustedSourcePath {
    param(
        [Parameter(Mandatory)]
        [string]$Path,

        [Parameter(Mandatory)]
        [string]$Root,

        [Parameter(Mandatory)]
        [string]$Description
    )

    $fullPath = [System.IO.Path]::GetFullPath($Path)
    $fullRoot = [System.IO.Path]::GetFullPath($Root).TrimEnd('\')
    if (-not $fullPath.Equals($fullRoot, [System.StringComparison]::OrdinalIgnoreCase) -and
        -not $fullPath.StartsWith("$fullRoot\", [System.StringComparison]::OrdinalIgnoreCase)) {
        throw "$Description '$fullPath' is outside trusted root '$fullRoot'."
    }
    if (-not (Test-Path -LiteralPath $fullPath -PathType Leaf)) {
        throw "$Description '$fullPath' is absent; refusing to load an unanchored source."
    }
    $item = Get-Item -LiteralPath $fullPath -Force
    if (($item.Attributes -band [System.IO.FileAttributes]::ReparsePoint) -ne 0 -or $item.PSIsContainer) {
        throw "$Description '$fullPath' is not a regular non-reparse source file."
    }
    $current = [System.IO.Path]::GetDirectoryName($fullPath)
    while (-not $current.Equals($fullRoot, [System.StringComparison]::OrdinalIgnoreCase)) {
        if (-not (Test-Path -LiteralPath $current -PathType Container)) {
            throw "$Description '$fullPath' has a missing trusted ancestor '$current'."
        }
        $ancestor = Get-Item -LiteralPath $current -Force
        if (($ancestor.Attributes -band [System.IO.FileAttributes]::ReparsePoint) -ne 0) {
            throw "$Description '$fullPath' crosses reparse ancestor '$current'."
        }
        $parent = [System.IO.Path]::GetDirectoryName($current)
        if ([string]::IsNullOrWhiteSpace($parent) -or $parent.Equals($current, [System.StringComparison]::OrdinalIgnoreCase)) {
            throw "$Description '$fullPath' cannot be anchored to '$fullRoot'."
        }
        $current = $parent
    }
    return $fullPath
}

foreach ($trustedSourcePath in $trustedSourcePaths) {
    $trustedRoot = if ($trustedSourcePath.Equals($orchestratorPathForTrust, [System.StringComparison]::OrdinalIgnoreCase) -or
        $trustedSourcePath.Equals($wixSourcePathForTrust, [System.StringComparison]::OrdinalIgnoreCase)) {
        $repoRootForTrust
    } else {
        $helperRoot
    }
    Assert-TrustedSourcePath -Path $trustedSourcePath -Root $trustedRoot -Description 'Trusted package source' | Out-Null
}
$trustedSourceSnapshot = [ordered]@{}
foreach ($trustedSourcePath in $trustedSourcePaths) {
    $trustedSourceSnapshot[$trustedSourcePath] = Get-TrustedSourceHash -Path $trustedSourcePath
}

function Assert-TrustedSourceSnapshot {
    foreach ($trustedSourcePath in $trustedSourceSnapshot.Keys) {
        $trustedRoot = if ($trustedSourcePath.Equals($orchestratorPathForTrust, [System.StringComparison]::OrdinalIgnoreCase) -or
            $trustedSourcePath.Equals($wixSourcePathForTrust, [System.StringComparison]::OrdinalIgnoreCase)) {
            $repoRootForTrust
        } else {
            $helperRoot
        }
        Assert-TrustedSourcePath -Path $trustedSourcePath -Root $trustedRoot -Description 'Trusted package source' | Out-Null
        $currentHash = Get-TrustedSourceHash -Path $trustedSourcePath
        if ($currentHash -cne $trustedSourceSnapshot[$trustedSourcePath]) {
            throw "Trusted package source '$trustedSourcePath' changed after the anchored snapshot; refusing to continue."
        }
    }
}

function Assert-TrustedInputSnapshot {
    param(
        [Parameter(Mandatory)]
        [object]$ExpectedSnapshot,

        [Parameter(Mandatory)]
        [string[]]$Paths
    )

    $current = [ordered]@{}
    foreach ($path in $Paths) {
        Assert-TrustedSourcePath -Path $path -Root $repoRootForTrust -Description 'Anchored package input' | Out-Null
        $current[$path] = Get-TrustedSourceHash -Path $path
    }
    $expectedJson = $ExpectedSnapshot | ConvertTo-Json -Depth 8 -Compress
    $currentJson = $current | ConvertTo-Json -Depth 8 -Compress
    if ($currentJson -cne $expectedJson) {
        throw 'A repository-anchored package input changed after its snapshot; refusing to publish a drifted package.'
    }
}

. (Join-Path $helperRoot 'package-inputs.ps1')
. (Join-Path $helperRoot 'package-path-safety.ps1')
. (Join-Path $helperRoot 'package-publication.ps1')
. (Join-Path $helperRoot 'wix-extension.ps1')
. (Join-Path $helperRoot 'msi-validation.ps1')
Assert-TrustedSourceSnapshot

$repoRoot = [System.IO.Path]::GetFullPath($RepoRoot).TrimEnd('\')
$orchestratorPath = [System.IO.Path]::GetFullPath($OrchestratorPath)
$packageRoot = Join-Path $repoRoot 'target\release-packages'
$targetRoot = Join-Path $repoRoot 'target\release'
$manifestPath = $null
$msiPath = $null
$checksumPath = $null
$wixIntermediateRoot = $null
$stagingRoot = $null
$backupRoot = $null
$publicationLayout = $null

try {
    if (-not (Test-Path -LiteralPath $repoRoot -PathType Container)) {
        throw "Repository root '$repoRoot' is not a directory; refusing to package."
    }
    Assert-NoPackageReparseChain -Path $repoRoot -Description 'Repository root'
    if (-not (Test-Path -LiteralPath $orchestratorPath -PathType Leaf)) {
        throw "Package orchestrator '$orchestratorPath' is absent; refusing to package."
    }
    New-SafePackageDirectory -Path $packageRoot -Root $repoRoot -Description 'Repository package root' | Out-Null
    $outputRoot = [System.IO.Path]::GetFullPath($OutputRoot).TrimEnd('\')
    $outputRoot = Assert-PhysicalPackagePathUnderRoot -Path $outputRoot -Root $packageRoot -Description 'OutputRoot parent'
    if ($outputRoot.Equals([System.IO.Path]::GetFullPath($packageRoot).TrimEnd('\'), [System.StringComparison]::OrdinalIgnoreCase)) {
        throw "OutputRoot must identify a package-specific child under '$packageRoot'."
    }

    $wixSourcePath = Join-Path $repoRoot 'scripts\release\windows\parent-protected-custody.wxs'
    $validationHelperPath = Join-Path $helperRoot 'msi-validation.ps1'
    $inputsHelperPath = Join-Path $helperRoot 'package-inputs.ps1'
    $pathSafetyHelperPath = Join-Path $helperRoot 'package-path-safety.ps1'
    $publicationHelperPath = Join-Path $helperRoot 'package-publication.ps1'
    $wixExtensionHelperPath = Join-Path $helperRoot 'wix-extension.ps1'
    $msiContractHelperPath = Join-Path $helperRoot 'msi-contract.ps1'
    $helperPath = Join-Path $helperRoot 'build-package.ps1'
    $packageSourcePaths = @(
        $wixSourcePath,
        $orchestratorPath,
        $helperPath,
        $validationHelperPath,
        $inputsHelperPath,
        $pathSafetyHelperPath,
        $publicationHelperPath,
        $wixExtensionHelperPath,
        $msiContractHelperPath
    )
    foreach ($sourcePath in $packageSourcePaths) {
        if (-not (Test-Path -LiteralPath $sourcePath -PathType Leaf)) {
            throw "Required package source '$sourcePath' is absent; refusing to package."
        }
        Assert-PhysicalPackagePathUnderRoot -Path $sourcePath -Root $repoRoot -Description 'Package source' | Out-Null
    }

    if (-not [string]::IsNullOrWhiteSpace($Version) -and $Version -notmatch '^\d+\.\d+\.\d+$') {
        throw "Package version '$Version' is not a strict semantic version; refusing to package."
    }

    $cargoCommand = Register-TrustedCommand -Command 'cargo'
    $dotnetCommand = Register-TrustedCommand -Command 'dotnet'
    $nodeCommand = Register-TrustedCommand -Command 'node'
    Assert-TrustedSourceSnapshot
    $cargoManifestPath = Join-Path $repoRoot 'Cargo.toml'
    $dotnetToolManifestPath = Join-Path $repoRoot '.config\dotnet-tools.json'
    $versionScriptPath = Join-Path $repoRoot 'scripts\release\validate-version.mjs'
    $anchoredInputPaths = @($cargoManifestPath, $dotnetToolManifestPath, $versionScriptPath)
    foreach ($anchoredInput in $anchoredInputPaths) {
        if (-not (Test-Path -LiteralPath $anchoredInput -PathType Leaf)) {
            throw "Required repository-anchored input '$anchoredInput' is absent; refusing to package."
        }
        Assert-PhysicalPackagePathUnderRoot -Path $anchoredInput -Root $repoRoot -Description 'Build input' | Out-Null
    }
    $trustedInputSnapshot = [ordered]@{}
    foreach ($anchoredInput in $anchoredInputPaths) {
        Assert-TrustedSourcePath -Path $anchoredInput -Root $repoRootForTrust -Description 'Anchored package input' | Out-Null
        $trustedInputSnapshot[$anchoredInput] = Get-TrustedSourceHash -Path $anchoredInput
    }

    Push-Location $repoRoot
    try {
        $policyVersionOutput = Invoke-CheckedCommand -Command $nodeCommand -ArgumentList @(
            $versionScriptPath,
            '--print-version'
        ) -FailureMessage 'Release version policy validation failed'
        $policyVersion = ($policyVersionOutput | Select-Object -Last 1).ToString().Trim()
        if ([string]::IsNullOrWhiteSpace($policyVersion) -or $policyVersion -notmatch '^\d+\.\d+\.\d+$') {
            throw "Release version policy returned an invalid MSI version '$policyVersion'."
        }
        if ([string]::IsNullOrWhiteSpace($Version)) {
            $Version = $policyVersion
        } elseif ($Version -ne $policyVersion) {
            throw "Requested package version '$Version' does not match the repository release version '$policyVersion'."
        }
    } finally {
        Pop-Location
    }

    # Keep each published version in its own immutable-name directory. A
    # v1 final is never mistaken for a v2 artifact set during recovery.
    $outputRoot = Join-Path $outputRoot $Version
    $outputRoot = Assert-PhysicalPackagePathUnderRoot -Path $outputRoot -Root $packageRoot -Description 'Versioned OutputRoot'

    $brokerBinaryPath = Join-Path $targetRoot 'ocentra-protected-capability-custody-broker.exe'
    $provisionerBinaryPath = Join-Path $targetRoot 'ocentra-protected-capability-custody-provisioner.exe'
    New-SafePackageDirectory -Path $targetRoot -Root $repoRoot -Description 'Cargo release target root' | Out-Null
    Assert-NoPackageReparseChain -Path $targetRoot -Description 'Cargo release target root immediately before build'
    $brokerInstallPath = 'C:\Program Files\Ocentra\OcentraParent\ocentra-protected-capability-custody-broker.exe'
    $brokerDatabaseIdentityPath = 'C:\ProgramData\Ocentra\OcentraParent\protected-capability-custody\custody.sqlite'
    $expectedRegistryId = '2cc753a30323ee51ee0301439996c5e4077fe49d3a31250ee75b32b6ecd1baf7'
    $registryId = Get-RegistryId -CanonicalDatabasePath $brokerDatabaseIdentityPath
    if ($registryId -cne $expectedRegistryId) {
        throw "Canonical database identity registry id '$registryId' does not match the owner-approved core/provisioner identity '$expectedRegistryId'; refusing to package."
    }
    $productCode = Get-DeterministicGuid -Seed "ocentra-parent-protected-custody/msi/$Version"

    $publicationLayout = New-PackagePublicationLayout -PackageRoot $packageRoot -OutputRoot $outputRoot
    $stagingRoot = $publicationLayout.StagingRoot
    $backupRoot = $publicationLayout.BackupRoot
    $msiFileName = "ocentra-parent-protected-custody-$Version-x64.msi"
    $checksumFileName = "$msiFileName.sha256"
    $manifestFileName = "ocentra-parent-protected-custody-$Version-x64.manifest.json"
    $artifactNames = @($msiFileName, $checksumFileName, $manifestFileName)
    $wixIntermediateRoot = Join-Path $stagingRoot 'wix-obj'
    $msiPath = Join-Path $stagingRoot $msiFileName
    $checksumPath = Join-Path $stagingRoot $checksumFileName
    $manifestPath = Join-Path $stagingRoot $manifestFileName
    $firstMsiPath = Join-Path $wixIntermediateRoot 'candidate-a.msi'
    $secondMsiPath = Join-Path $wixIntermediateRoot 'candidate-b.msi'
    $firstIntermediatePath = Join-Path $wixIntermediateRoot 'candidate-a'
    $secondIntermediatePath = Join-Path $wixIntermediateRoot 'candidate-b'
    $finalValidationPath = Join-Path $wixIntermediateRoot 'final'
    New-SafePackageDirectory -Path $wixIntermediateRoot -Root $packageRoot -Description 'WiX intermediate root' | Out-Null
    foreach ($stagedLeafPath in @($msiPath, $checksumPath, $manifestPath, $firstMsiPath, $secondMsiPath)) {
        Assert-SafePackageLeafPath -Path $stagedLeafPath -Root $packageRoot -Description 'Staged package file' | Out-Null
    }

    Push-Location $repoRoot
    try {
        Assert-TrustedSourceSnapshot
        Assert-NoPackageReparseChain -Path $targetRoot -Description 'Cargo release target root immediately before build'
        Invoke-CheckedCommand -Command $cargoCommand -ArgumentList @(
            'build',
            '--manifest-path',
            $cargoManifestPath,
            '--release',
            '--locked',
            '-p',
            'ocentra-protected-capability-custody-broker',
            '-p',
            'ocentra-protected-capability-custody-provisioner'
        ) -FailureMessage 'Protected broker/provisioner release build failed'
        Assert-TrustedSourceSnapshot

        foreach ($requiredBinary in @($brokerBinaryPath, $provisionerBinaryPath)) {
            Assert-NonEmptyFile -Path $requiredBinary -Root $repoRoot -Description 'Required protected custody binary'
        }

        $brokerHash = Get-Sha256Hex -Path $brokerBinaryPath
        $provisionerHash = Get-Sha256Hex -Path $provisionerBinaryPath
        $wixSourceHash = Get-Sha256Hex -Path $wixSourcePath
        $orchestratorHash = Get-Sha256Hex -Path $orchestratorPath
        $helperHash = Get-Sha256Hex -Path $helperPath
        $validationHelperHash = Get-Sha256Hex -Path $validationHelperPath
        $inputsHelperHash = Get-Sha256Hex -Path $inputsHelperPath
        $pathSafetyHelperHash = Get-Sha256Hex -Path $pathSafetyHelperPath
        $publicationHelperHash = Get-Sha256Hex -Path $publicationHelperPath
        $wixExtensionHelperHash = Get-Sha256Hex -Path $wixExtensionHelperPath
        $msiContractHelperHash = Get-Sha256Hex -Path $msiContractHelperPath
        $packageCode = Get-DeterministicGuid -Seed "ocentra-parent-protected-custody/package/$Version/$brokerHash/$provisionerHash/$wixSourceHash"

        Assert-TrustedSourceSnapshot
        Invoke-CheckedCommand -Command $dotnetCommand -ArgumentList @(
            'tool',
            'restore',
            '--tool-manifest',
            $dotnetToolManifestPath
        ) -FailureMessage 'Pinned WiX dotnet tool restore failed'

        Install-ExactWixUtilExtension -DotnetCommand $dotnetCommand

        function Invoke-WixCandidateBuild {
            param(
                [Parameter(Mandatory)]
                [string]$CandidateMsiPath,

                [Parameter(Mandatory)]
                [string]$CandidateIntermediatePath
            )

            New-SafePackageDirectory -Path $CandidateIntermediatePath -Root $packageRoot -Description 'WiX candidate intermediate' | Out-Null
            Assert-SafePackageLeafPath -Path $CandidateMsiPath -Root $packageRoot -Description 'WiX candidate MSI' | Out-Null
            Assert-TrustedSourceSnapshot
            Assert-NoPackageReparseChain -Path $CandidateIntermediatePath -Description 'WiX candidate intermediate immediately before build'
            Assert-SafePackageLeafPath -Path $CandidateMsiPath -Root $packageRoot -Description 'WiX candidate MSI immediately before build' | Out-Null
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
                "RegistryId=$registryId",
                '-intermediatefolder',
                $CandidateIntermediatePath,
                '-out',
                $CandidateMsiPath,
                '-pdbtype',
                'none'
            ) -FailureMessage 'WiX protected custody package build failed'
            Assert-NonEmptyFile -Path $CandidateMsiPath -Root $packageRoot -Description 'WiX candidate MSI'
            Set-DeterministicMsiSummary -Path $CandidateMsiPath -PackageCode $packageCode -PackageRoot $packageRoot
            Normalize-CfbRootModifiedFileTime -Path $CandidateMsiPath -PackageRoot $packageRoot
            $candidateValidationArguments = @{
                CandidateMsiPath = $CandidateMsiPath
                CandidateIntermediatePath = $CandidateIntermediatePath
                DotnetCommand = $dotnetCommand
                ExpectedVersion = $Version
                ExpectedProductCode = $productCode
                ExpectedPackageCode = $packageCode
                ExpectedBrokerHash = $brokerHash
                ExpectedProvisionerHash = $provisionerHash
                ExpectedRegistryId = $registryId
                BrokerBinaryPath = $brokerBinaryPath
                ProvisionerBinaryPath = $provisionerBinaryPath
                PackageRoot = $packageRoot
            }
            Assert-MsiCandidate @candidateValidationArguments
        }

        # Build twice with isolated intermediates and compare every byte. MSI
        # success is not package readiness; all fixed table, boundary, and
        # payload checks must pass before publication.
        Invoke-WixCandidateBuild -CandidateMsiPath $firstMsiPath -CandidateIntermediatePath $firstIntermediatePath
        Invoke-WixCandidateBuild -CandidateMsiPath $secondMsiPath -CandidateIntermediatePath $secondIntermediatePath
        Assert-ByteIdentical -LeftPath $firstMsiPath -RightPath $secondMsiPath
        $firstMsiHash = Get-Sha256Hex -Path $firstMsiPath
        $secondMsiHash = Get-Sha256Hex -Path $secondMsiPath
        if ($firstMsiHash -cne $secondMsiHash) {
            throw "WiX repeated-build SHA-256 values differ ($firstMsiHash vs $secondMsiHash); refusing to emit a non-deterministic MSI."
        }
        Copy-SafePackageFile -SourcePath $firstMsiPath -DestinationPath $msiPath -Root $packageRoot -Description 'Validated MSI copy' | Out-Null
        Assert-NonEmptyFile -Path $msiPath -Root $packageRoot -Description 'Deterministic final MSI'
        $finalValidationArguments = @{
            CandidateMsiPath = $msiPath
            CandidateIntermediatePath = $finalValidationPath
            DotnetCommand = $dotnetCommand
            ExpectedVersion = $Version
            ExpectedProductCode = $productCode
            ExpectedPackageCode = $packageCode
            ExpectedBrokerHash = $brokerHash
            ExpectedProvisionerHash = $provisionerHash
            ExpectedRegistryId = $registryId
            BrokerBinaryPath = $brokerBinaryPath
            ProvisionerBinaryPath = $provisionerBinaryPath
            PackageRoot = $packageRoot
        }
        Assert-MsiCandidate @finalValidationArguments
        $msiHash = Get-Sha256Hex -Path $msiPath
        if ($msiHash -cne $firstMsiHash) {
            throw "Published MSI SHA-256 '$msiHash' differs from the validated repeat-build SHA-256 '$firstMsiHash'."
        }
        Write-Utf8NoBom -Path $checksumPath -Content ("$msiHash *$([System.IO.Path]::GetFileName($msiPath))" + [Environment]::NewLine) -Root $packageRoot

        Assert-TrustedSourceSnapshot
        Assert-TrustedInputSnapshot -ExpectedSnapshot $trustedInputSnapshot -Paths $anchoredInputPaths
        $trustedCommandSnapshot = Get-TrustedCommandSnapshot
        $publicationInputContract = Get-PackagePublicationInputContract -InputContract ([ordered]@{
                brokerBinarySha256 = $brokerHash
                provisionerBinarySha256 = $provisionerHash
                sourceHashes = $trustedSourceSnapshot
                anchoredInputHashes = $trustedInputSnapshot
                commandFingerprints = $trustedCommandSnapshot
            })

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
                verification = 'two independent WiX builds opened, fixed tables and extracted payload hashes checked, MSI validated, normalized for MSI metadata and CFB root FILETIME, then compared byte-for-byte and by SHA-256'
                byteIdenticalRepeatBuilds = $true
                repeatSha256 = $firstMsiHash
            }
            inputs = @(
                [ordered]@{ role = 'broker'; file = [System.IO.Path]::GetFileName($brokerBinaryPath); sha256 = $brokerHash },
                [ordered]@{ role = 'provisioner'; file = [System.IO.Path]::GetFileName($provisionerBinaryPath); sha256 = $provisionerHash }
            )
            inputIntegrity = $publicationInputContract
            digestBinding = 'broker sha256 binds only the compiled payload; live Enrollment\\authority-v1 broker digest remains external WP02 owner state and is never authored by MSI'
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
                enrollment = 'External WP02 owner ceremony; package does not create or overwrite Enrollment\authority-v1.'
                permanentEnrollmentRepair = 'manual-required: permanent TrustedInstaller-owned Enrollment\authority-v1 remains external WP02 owner state.'
                serviceSidConfiguration = 'External Protected WP02 provisioner/SCM owner ceremony; MSI does not author MsiServiceConfig ServiceSid.'
                serviceSidRequirement = 'Broker start remains fail-closed until SCM observation reports SERVICE_SID_TYPE_UNRESTRICTED and the expected service identity.'
                permanentServiceSidRepair = 'manual-required: Protected WP02 owns SCM service-SID provisioning and readback; package failure-actions remain supported util:ServiceConfig only.'
                tpmOwnerCeremony = 'External OEM/firmware/MDM owner ceremony; not packaged.'
                tpmAuthorizationSecret = 'Never packaged, registered, logged, or passed on the command line.'
                readiness = 'Not claimed by MSI success, checksum, signing status, service registration, or package publication.'
                signing = 'manual-required'
            }
            source = [ordered]@{
                wix = 'scripts/release/windows/parent-protected-custody.wxs'
                wixSha256 = $wixSourceHash
                orchestrator = 'scripts/release/windows/build-parent-protected-custody-package.ps1'
                orchestratorSha256 = $orchestratorHash
                helper = 'scripts/release/windows/parent-protected-custody/build-package.ps1'
                helperSha256 = $helperHash
                validationHelper = 'scripts/release/windows/parent-protected-custody/msi-validation.ps1'
                validationHelperSha256 = $validationHelperHash
                inputsHelper = 'scripts/release/windows/parent-protected-custody/package-inputs.ps1'
                inputsHelperSha256 = $inputsHelperHash
                pathSafetyHelper = 'scripts/release/windows/parent-protected-custody/package-path-safety.ps1'
                pathSafetyHelperSha256 = $pathSafetyHelperHash
                publicationHelper = 'scripts/release/windows/parent-protected-custody/package-publication.ps1'
                publicationHelperSha256 = $publicationHelperHash
                wixExtensionHelper = 'scripts/release/windows/parent-protected-custody/wix-extension.ps1'
                wixExtensionHelperSha256 = $wixExtensionHelperHash
                msiContractHelper = 'scripts/release/windows/parent-protected-custody/msi-contract.ps1'
                msiContractHelperSha256 = $msiContractHelperHash
            }
        }
        Write-DeterministicJson -Path $manifestPath -Value $manifest -Root $packageRoot
        Assert-NonEmptyFile -Path $manifestPath -Root $packageRoot -Description 'Generated package manifest'
        $manifestText = [System.IO.File]::ReadAllText($manifestPath)
        foreach ($requiredBoundaryText in @(
                'manual-required',
                'External WP02 owner ceremony',
                'MsiServiceConfig ServiceSid',
                'SERVICE_SID_TYPE_UNRESTRICTED'
            )) {
            if ($manifestText -notmatch [System.Text.RegularExpressions.Regex]::Escape($requiredBoundaryText)) {
                throw "Generated package manifest lost required boundary text '$requiredBoundaryText'."
            }
        }
        $checksumText = [System.IO.File]::ReadAllText($checksumPath)
        $expectedChecksumText = "$msiHash *$msiFileName`n"
        if ($checksumText -cne $expectedChecksumText) {
            throw 'Generated package checksum does not exactly bind the validated staged MSI.'
        }
        $manifestRecord = $manifestText | ConvertFrom-Json
        if ([string]$manifestRecord.artifact.file -cne $msiFileName -or
            [string]$manifestRecord.artifact.sha256 -cne $msiHash -or
            [string]$manifestRecord.artifact.checksumFile -cne $checksumFileName) {
            throw 'Generated package manifest does not exactly bind the staged MSI/checksum set.'
        }

        Remove-SafePackagePath -Path $wixIntermediateRoot -Root $packageRoot -Description 'Validated WiX intermediate tree'
        Assert-ExactPackageDirectoryFiles -Directory $stagingRoot -PackageRoot $packageRoot -ExpectedNames $artifactNames -Description 'Ready-to-publish package set'
        Assert-TrustedSourceSnapshot
        Assert-TrustedInputSnapshot -ExpectedSnapshot $trustedInputSnapshot -Paths $anchoredInputPaths
        Assert-TrustedCommandSnapshot -ExpectedSnapshot $trustedCommandSnapshot
        if ((Get-Sha256Hex -Path $brokerBinaryPath) -cne $brokerHash -or
            (Get-Sha256Hex -Path $provisionerBinaryPath) -cne $provisionerHash) {
            throw 'A protected custody release binary changed after validation; refusing publication.'
        }
        $currentPublicationInputContract = Get-PackagePublicationInputContract -InputContract ([ordered]@{
                brokerBinarySha256 = Get-Sha256Hex -Path $brokerBinaryPath
                provisionerBinarySha256 = Get-Sha256Hex -Path $provisionerBinaryPath
                sourceHashes = $trustedSourceSnapshot
                anchoredInputHashes = $trustedInputSnapshot
                commandFingerprints = Get-TrustedCommandSnapshot
            })
        $expectedContractJson = $publicationInputContract | ConvertTo-Json -Depth 32 -Compress
        $currentContractJson = $currentPublicationInputContract | ConvertTo-Json -Depth 32 -Compress
        if ($expectedContractJson -cne $currentContractJson) {
            throw 'Package inputs, helper sources, or tool fingerprints changed immediately before publication; refusing to publish drifted bytes.'
        }
        Publish-StagedPackageDirectory -PackageRoot $packageRoot -OutputRoot $outputRoot -StagingRoot $stagingRoot -BackupRoot $backupRoot -ArtifactNames $artifactNames -LockStream $publicationLayout.LockStream -InputContract $publicationInputContract | Out-Null
        Assert-TrustedSourceSnapshot
        $msiPath = Join-Path $outputRoot $msiFileName
        $checksumPath = Join-Path $outputRoot $checksumFileName
        $manifestPath = Join-Path $outputRoot $manifestFileName
        Write-Output "Built $msiPath"
        Write-Output "SHA256 $msiHash"
        Write-Output "Manifest $manifestPath"
    } finally {
        Pop-Location
    }
} finally {
    $cleanupFailures = [System.Collections.Generic.List[string]]::new()
    $durableJournalPresent = $false
    if ($null -ne $publicationLayout -and -not [string]::IsNullOrWhiteSpace($publicationLayout.JournalPath)) {
        $durableJournalPresent = Test-Path -LiteralPath $publicationLayout.JournalPath -PathType Leaf
    }
    if (-not $durableJournalPresent -and -not [string]::IsNullOrWhiteSpace($stagingRoot) -and (Test-Path -LiteralPath $stagingRoot)) {
        try {
            Remove-SafePackagePath -Path $stagingRoot -Root $packageRoot -Description 'Unpublished package staging cleanup'
        } catch {
            $cleanupFailures.Add("${stagingRoot}: $($_.Exception.Message)")
        }
    }
    if ($null -ne $publicationLayout -and $null -ne $publicationLayout.LockStream) {
        try {
            $publicationLayout.LockStream.Dispose()
        } catch {
            $cleanupFailures.Add('Package publication lock disposal failed: ' + $_.Exception.Message)
        }
    }
    if ($cleanupFailures.Count -gt 0) {
        throw "Package staging cleanup failed; prior published package state was not targeted: $($cleanupFailures -join '; ')"
    }
}
