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
. (Join-Path $helperRoot 'package-inputs.ps1')
. (Join-Path $helperRoot 'msi-validation.ps1')

$repoRoot = [System.IO.Path]::GetFullPath($RepoRoot).TrimEnd('\')
$orchestratorPath = [System.IO.Path]::GetFullPath($OrchestratorPath)
$packageRoot = Join-Path $repoRoot 'target\release-packages'
$targetRoot = Join-Path $repoRoot 'target\release'
$manifestPath = $null
$msiPath = $null
$checksumPath = $null
$wixIntermediateRoot = $null
$success = $false
$cleanupPaths = [System.Collections.Generic.List[string]]::new()

try {
    if (-not (Test-Path -LiteralPath $repoRoot -PathType Container)) {
        throw "Repository root '$repoRoot' is not a directory; refusing to package."
    }
    if (-not (Test-Path -LiteralPath $orchestratorPath -PathType Leaf)) {
        throw "Package orchestrator '$orchestratorPath' is absent; refusing to package."
    }
    if (-not (Test-Path -LiteralPath $packageRoot -PathType Container)) {
        New-Item -ItemType Directory -Path $packageRoot -Force | Out-Null
    }
    $outputRoot = [System.IO.Path]::GetFullPath($OutputRoot).TrimEnd('\')
    $outputRoot = Resolve-UnderRoot -Path $outputRoot -Root $packageRoot -Description 'OutputRoot'
    if ($outputRoot.Equals([System.IO.Path]::GetFullPath($packageRoot).TrimEnd('\'), [System.StringComparison]::OrdinalIgnoreCase)) {
        throw "OutputRoot must identify a package-specific child under '$packageRoot'."
    }

    $wixSourcePath = Join-Path $repoRoot 'scripts\release\windows\parent-protected-custody.wxs'
    $validationHelperPath = Join-Path $helperRoot 'msi-validation.ps1'
    $inputsHelperPath = Join-Path $helperRoot 'package-inputs.ps1'
    $helperPath = Join-Path $helperRoot 'build-package.ps1'
    foreach ($sourcePath in @($wixSourcePath, $orchestratorPath, $helperPath, $validationHelperPath, $inputsHelperPath)) {
        if (-not (Test-Path -LiteralPath $sourcePath -PathType Leaf)) {
            throw "Required package source '$sourcePath' is absent; refusing to package."
        }
        Resolve-UnderRoot -Path $sourcePath -Root $repoRoot -Description 'Package source' | Out-Null
    }

    if (-not [string]::IsNullOrWhiteSpace($Version) -and $Version -notmatch '^\d+\.\d+\.\d+$') {
        throw "Package version '$Version' is not a strict semantic version; refusing to package."
    }

    $cargoCommand = Resolve-RequiredCommand -Name 'cargo'
    $dotnetCommand = Resolve-RequiredCommand -Name 'dotnet'
    $nodeCommand = Resolve-RequiredCommand -Name 'node'
    $cargoManifestPath = Join-Path $repoRoot 'Cargo.toml'
    $dotnetToolManifestPath = Join-Path $repoRoot '.config\dotnet-tools.json'
    $versionScriptPath = Join-Path $repoRoot 'scripts\release\validate-version.mjs'
    foreach ($anchoredInput in @($cargoManifestPath, $dotnetToolManifestPath, $versionScriptPath)) {
        if (-not (Test-Path -LiteralPath $anchoredInput -PathType Leaf)) {
            throw "Required repository-anchored input '$anchoredInput' is absent; refusing to package."
        }
        Resolve-UnderRoot -Path $anchoredInput -Root $repoRoot -Description 'Build input' | Out-Null
    }

    Push-Location $repoRoot
    try {
        $policyVersionOutput = & $nodeCommand $versionScriptPath '--print-version'
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
    } finally {
        Pop-Location
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

    $wixIntermediateRoot = Join-Path $outputRoot 'wix-obj'
    $msiPath = Join-Path $outputRoot "ocentra-parent-protected-custody-$Version-x64.msi"
    $checksumPath = "$msiPath.sha256"
    $manifestPath = Join-Path $outputRoot "ocentra-parent-protected-custody-$Version-x64.manifest.json"
    $firstMsiPath = Join-Path $wixIntermediateRoot 'candidate-a.msi'
    $secondMsiPath = Join-Path $wixIntermediateRoot 'candidate-b.msi'
    $firstIntermediatePath = Join-Path $wixIntermediateRoot 'candidate-a'
    $secondIntermediatePath = Join-Path $wixIntermediateRoot 'candidate-b'
    $finalValidationPath = Join-Path $wixIntermediateRoot 'final'
    $cleanupPaths.Add($wixIntermediateRoot)
    foreach ($knownGeneratedPath in @(
            "$firstMsiPath.cfb-normalized.tmp",
            "$firstMsiPath.cfb-normalized.bak",
            "$secondMsiPath.cfb-normalized.tmp",
            "$secondMsiPath.cfb-normalized.bak",
            "$msiPath.cfb-normalized.tmp",
            "$msiPath.cfb-normalized.bak"
        )) {
        $cleanupPaths.Add($knownGeneratedPath)
    }

    New-Item -ItemType Directory -Path $outputRoot -Force | Out-Null
    foreach ($knownGeneratedPath in @($wixIntermediateRoot, $msiPath, $checksumPath, $manifestPath)) {
        Remove-ExactPath -Path $knownGeneratedPath
    }
    New-Item -ItemType Directory -Path $wixIntermediateRoot -Force | Out-Null

    Push-Location $repoRoot
    try {
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

        foreach ($requiredBinary in @($brokerBinaryPath, $provisionerBinaryPath)) {
            Assert-NonEmptyFile -Path $requiredBinary -Description 'Required protected custody binary'
        }

        $brokerHash = Get-Sha256Hex -Path $brokerBinaryPath
        $provisionerHash = Get-Sha256Hex -Path $provisionerBinaryPath
        $wixSourceHash = Get-Sha256Hex -Path $wixSourcePath
        $orchestratorHash = Get-Sha256Hex -Path $orchestratorPath
        $helperHash = Get-Sha256Hex -Path $helperPath
        $validationHelperHash = Get-Sha256Hex -Path $validationHelperPath
        $inputsHelperHash = Get-Sha256Hex -Path $inputsHelperPath
        $packageCode = Get-DeterministicGuid -Seed "ocentra-parent-protected-custody/package/$Version/$brokerHash/$provisionerHash/$wixSourceHash"

        function Get-WixUtilExtensionVersions {
            $extensionOutput = @(& $dotnetCommand wix extension list 2>&1)
            if ($LASTEXITCODE -ne 0) {
                throw 'Pinned WiX extension listing failed; refusing to package.'
            }
            $utilLines = @($extensionOutput | ForEach-Object { [string]$_ } | Where-Object { $_ -match '(?i)WixToolset\.Util\.wixext' })
            $versions = [System.Collections.Generic.List[string]]::new()
            foreach ($utilLine in $utilLines) {
                if ($utilLine -notmatch '^\s*WixToolset\.Util\.wixext\s+(?<version>\d+\.\d+\.\d+)\s*$') {
                    throw "Pinned WiX extension listing has an unparseable Util row '$utilLine'; refusing to accept a prefix or unanchored version."
                }
                $versions.Add($Matches['version'])
            }
            return @($versions)
        }

        Invoke-CheckedCommand -Command $dotnetCommand -ArgumentList @(
            'tool',
            'restore',
            '--tool-manifest',
            $dotnetToolManifestPath
        ) -FailureMessage 'Pinned WiX dotnet tool restore failed'

        $extensionVersions = @(Get-WixUtilExtensionVersions)
        if ($extensionVersions.Count -eq 1 -and $extensionVersions[0] -ceq '6.0.2') {
            # Exact anchored match is already present.
        } elseif ($extensionVersions.Count -gt 0) {
            throw "Pinned WiX Util extension versions '$($extensionVersions -join ', ')' do not equal the required exact version 6.0.2; refusing to continue."
        } else {
            Invoke-CheckedCommand -Command $dotnetCommand -ArgumentList @(
                'wix',
                'extension',
                'add',
                'WixToolset.Util.wixext/6.0.2'
            ) -FailureMessage 'WiX Util extension installation failed'
            $postAddVersions = @(Get-WixUtilExtensionVersions)
            if ($postAddVersions.Count -ne 1 -or $postAddVersions[0] -cne '6.0.2') {
                throw "Post-add WiX Util extension verification found '$($postAddVersions -join ', ')', not exactly 6.0.2; refusing to package."
            }
        }

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
            Assert-NonEmptyFile -Path $CandidateMsiPath -Description 'WiX candidate MSI'
            Set-DeterministicMsiSummary -Path $CandidateMsiPath -PackageCode $packageCode
            Normalize-CfbRootModifiedFileTime -Path $CandidateMsiPath
            $candidateValidationArguments = @{
                CandidateMsiPath = $CandidateMsiPath
                CandidateIntermediatePath = $CandidateIntermediatePath
                DotnetCommand = $dotnetCommand
                ExpectedVersion = $Version
                ExpectedProductCode = $productCode
                ExpectedBrokerHash = $brokerHash
                ExpectedProvisionerHash = $provisionerHash
                ExpectedRegistryId = $registryId
                BrokerBinaryPath = $brokerBinaryPath
                ProvisionerBinaryPath = $provisionerBinaryPath
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
        Copy-Item -LiteralPath $firstMsiPath -Destination $msiPath -Force
        Assert-NonEmptyFile -Path $msiPath -Description 'Deterministic final MSI'
        $finalValidationArguments = @{
            CandidateMsiPath = $msiPath
            CandidateIntermediatePath = $finalValidationPath
            DotnetCommand = $dotnetCommand
            ExpectedVersion = $Version
            ExpectedProductCode = $productCode
            ExpectedBrokerHash = $brokerHash
            ExpectedProvisionerHash = $provisionerHash
            ExpectedRegistryId = $registryId
            BrokerBinaryPath = $brokerBinaryPath
            ProvisionerBinaryPath = $provisionerBinaryPath
        }
        Assert-MsiCandidate @finalValidationArguments
        $msiHash = Get-Sha256Hex -Path $msiPath
        if ($msiHash -cne $firstMsiHash) {
            throw "Published MSI SHA-256 '$msiHash' differs from the validated repeat-build SHA-256 '$firstMsiHash'."
        }
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
                verification = 'two independent WiX builds opened, fixed tables and extracted payload hashes checked, MSI validated, normalized for MSI metadata and CFB root FILETIME, then compared byte-for-byte and by SHA-256'
                byteIdenticalRepeatBuilds = $true
                repeatSha256 = $firstMsiHash
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
            }
        }
        Write-DeterministicJson -Path $manifestPath -Value $manifest
        Assert-NonEmptyFile -Path $manifestPath -Description 'Generated package manifest'
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
        $success = $true
        Write-Output "Built $msiPath"
        Write-Output "SHA256 $msiHash"
        Write-Output "Manifest $manifestPath"
    } finally {
        Pop-Location
    }
} finally {
    $cleanupFailures = [System.Collections.Generic.List[string]]::new()
    if (-not $success) {
        $cleanupPaths.Add($msiPath)
        $cleanupPaths.Add($checksumPath)
        $cleanupPaths.Add($manifestPath)
    }
    foreach ($cleanupPath in @($cleanupPaths | Select-Object -Unique)) {
        if ([string]::IsNullOrWhiteSpace($cleanupPath)) {
            continue
        }
        try {
            Remove-ExactPath -Path $cleanupPath
        } catch {
            $cleanupFailures.Add("${cleanupPath}: $($_.Exception.Message)")
        }
    }
    if ($cleanupFailures.Count -gt 0) {
        throw "Package cleanup failed; refusing to report a usable artifact: $($cleanupFailures -join '; ')"
    }
}
