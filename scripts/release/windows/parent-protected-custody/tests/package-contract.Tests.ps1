#requires -Version 7.2

[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$packageTestRoot = (Resolve-Path $PSScriptRoot).Path
$packageRoot = (Resolve-Path (Join-Path $packageTestRoot '..')).Path
$repoRoot = (Resolve-Path (Join-Path $packageRoot '..\..\..\..')).Path

. (Join-Path $packageRoot 'package-inputs.ps1')
. (Join-Path $packageRoot 'package-path-safety.ps1')
. (Join-Path $packageRoot 'wix-extension.ps1')
. (Join-Path $packageRoot 'package-publication.ps1')
. (Join-Path $packageRoot 'msi-validation.ps1')

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

function Assert-Matches {
    param(
        [Parameter(Mandatory)]
        [string]$Text,

        [Parameter(Mandatory)]
        [string]$Pattern,

        [Parameter(Mandatory)]
        [string]$Description
    )

    if ($Text -notmatch $Pattern) {
        throw "$Description did not match '$Pattern'."
    }
}

function Assert-DoesNotMatch {
    param(
        [Parameter(Mandatory)]
        [string]$Text,

        [Parameter(Mandatory)]
        [string]$Pattern,

        [Parameter(Mandatory)]
        [string]$Description
    )

    if ($Text -match $Pattern) {
        throw "$Description unexpectedly matched '$Pattern'."
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
        throw "$Description did not reject the invalid contract."
    }
    if (-not [string]::IsNullOrWhiteSpace($MessagePattern) -and
        ([string]$caught.Exception.Message -notmatch $MessagePattern)) {
        throw "$Description rejected with '$($caught.Exception.Message)', not '$MessagePattern'."
    }
}

function New-TestToolchainProvenance {
    $hash = (('ab' * 32) -join '')
    $toolHashes = [ordered]@{ 'wix.dll' = $hash }
    $extensionHashes = [ordered]@{ 'WixToolset.Util.wixext.dll' = $hash }
    return [ordered]@{
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

$wixSourcePath = Join-Path $repoRoot 'scripts\release\windows\parent-protected-custody.wxs'
$wixSource = [System.IO.File]::ReadAllText($wixSourcePath)
$buildSource = [System.IO.File]::ReadAllText((Join-Path $packageRoot 'build-package.ps1'))
$wrapperSource = [System.IO.File]::ReadAllText((Join-Path $repoRoot 'scripts\release\windows\build-parent-protected-custody-package.ps1'))

# Parse and exercise the same exact row-set comparator used by the MSI
# validator. The fixture is intentionally a normalized row, not an MSI
# database or a claim that MSI execution has been proven here.
$rowFields = @('Action', 'Condition', 'Sequence')
$row = [pscustomobject]@{
    Action = 'RunProtectedProvisioner'
    Condition = 'NOT REMOVE~="ALL"'
    Sequence = '5899'
}
$expectedRow = New-MsiContractSignature -Values @('RunProtectedProvisioner', 'NOT REMOVE~="ALL"', '5899')
Assert-ExactMsiRowSet -Rows @($row) -Fields $rowFields -ExpectedSignatures @($expectedRow) -Description 'RunProtectedProvisioner sequence fixture'
$row.Condition = 'REMOVE~="ALL"'
Assert-Throws {
    Assert-ExactMsiRowSet -Rows @($row) -Fields $rowFields -ExpectedSignatures @($expectedRow) -Description 'mutated MSI sequence fixture'
} 'MSI sequence mutation' 'unexpected or duplicate row'

$wix = [xml]$wixSource
$package = $wix.Wix.Package
Assert-Equal $package.Name 'Ocentra Parent Protected Capability Custody' 'MSI package name'
Assert-Equal $package.Manufacturer 'Ocentra' 'MSI manufacturer'
Assert-Equal $package.UpgradeCode 'A1BA5AA2-F5DB-4B97-9889-4BB4DBF52B3C' 'MSI upgrade code'
Assert-Equal $package.Scope 'perMachine' 'MSI scope'
Assert-Equal $package.InstallerVersion '500' 'MSI installer version'
Assert-Equal $package.SummaryInformation.Description 'Ocentra Parent protected broker and owner-bound provisioner package' 'MSI summary description'

$customAction = @($package.CustomAction | Where-Object { $_.Id -ceq 'RunProtectedProvisioner' })
Assert-Equal $customAction.Count 1 'protected provisioner custom action count'
Assert-Equal $customAction[0].FileRef 'ProtectedProvisionerFile' 'protected provisioner custom action file binding'
Assert-Equal $customAction[0].ExeCommand '' 'protected provisioner custom action command'
Assert-Equal $customAction[0].Execute 'deferred' 'protected provisioner custom action execution mode'
Assert-Equal $customAction[0].Impersonate 'no' 'protected provisioner custom action identity'
Assert-Equal $customAction[0].Return 'check' 'protected provisioner custom action failure policy'
Assert-Equal $customAction[0].HideTarget 'yes' 'protected provisioner custom action target hiding'

$sequenceAction = @($package.InstallExecuteSequence.Custom | Where-Object { $_.Action -ceq 'RunProtectedProvisioner' })
Assert-Equal $sequenceAction.Count 1 'protected provisioner sequence count'
Assert-Equal $sequenceAction[0].Before 'StartServices' 'protected provisioner service ordering'
Assert-Equal $sequenceAction[0].Condition 'NOT REMOVE~="ALL"' 'protected provisioner uninstall condition'

$components = @($wix.Wix.Fragment.ComponentGroup.Component) + @($wix.Wix.Fragment.Component)
$brokerComponent = @($components | Where-Object { $_.Id -ceq 'ProtectedBrokerService' })[0]
$provisionerComponent = @($components | Where-Object { $_.Id -ceq 'ProtectedProvisioner' })[0]
$brokerService = @($brokerComponent.ServiceInstall)[0]
$brokerControl = @($brokerComponent.ServiceControl)[0]
Assert-Equal $brokerService.Name 'OcentraProtectedCapabilityCustodyBroker' 'broker service name'
Assert-Equal $brokerService.Account 'LocalSystem' 'broker service account'
Assert-Equal $brokerService.Start 'auto' 'broker service start mode'
Assert-Equal $brokerService.ErrorControl 'critical' 'broker service error policy'
Assert-Equal $brokerControl.Remove 'uninstall' 'broker service uninstall control'
Assert-Equal $provisionerComponent.File.Name 'ocentra-protected-capability-custody-provisioner.exe' 'provisioner payload name'

$registryValues = @($components | Where-Object { $null -ne $_.RegistryKey } | ForEach-Object { $_.RegistryKey.RegistryValue } | Where-Object { $_.Name -ceq 'package-boundary' })
Assert-Equal $registryValues.Count 2 'package-boundary registry value count'
foreach ($registryValue in $registryValues) {
    Assert-Equal $registryValue.Value 'parent-protected-custody-v1' 'package-boundary registry value'
}

Assert-DoesNotMatch $wixSource '(?i)authValue' 'WiX source raw authValue boundary'
Assert-DoesNotMatch $wixSource '(?i)<RegistryValue[^>]+authority-v1' 'WiX source protected authority mutation'
Assert-Matches $wixSource 'Enrollment\\authority-v1' 'WiX source external enrollment boundary documentation'
Assert-Matches $buildSource "zero-argument owner-approved provisioner" 'build manifest zero-argument provisioner boundary'
Assert-Matches $buildSource "deprovisioning = 'Manual-required external WP02 owner path; uninstall never invokes deprovisioning implicitly" 'build manifest deprovisioning ownership'
Assert-Matches $buildSource "signing = 'manual-required'" 'build manifest signing boundary'
Assert-Matches $buildSource "'manual-required'" 'build manifest manual-required state'
Assert-DoesNotMatch $buildSource '(?i)authValue\s*=' 'build source raw authValue transport'
Assert-Matches $wrapperSource 'build-package\.ps1' 'top-level package wrapper delegation'
Assert-DoesNotMatch $wrapperSource '(?i)authValue|Enrollment\\authority-v1' 'top-level package wrapper authority transport'

$canonicalDatabasePath = 'C:\ProgramData\Ocentra\OcentraParent\protected-capability-custody\custody.sqlite'
Assert-Equal (Get-RegistryId -CanonicalDatabasePath $canonicalDatabasePath) '2cc753a30323ee51ee0301439996c5e4077fe49d3a31250ee75b32b6ecd1baf7' 'owner-approved fixed registry identity'

$normalizedProvenance = Get-PackagePublicationToolchainProvenance -Provenance (New-TestToolchainProvenance)
Assert-Equal $normalizedProvenance.wixTool.version '6.0.2' 'normalized WiX tool version'
Assert-Equal $normalizedProvenance.wixUtilExtension.version '6.0.2' 'normalized WiX Util extension version'
Assert-Equal $normalizedProvenance.wixUtilExtension.extensionId 'WixToolset.Util.wixext' 'normalized WiX Util extension identity'
$badProvenance = New-TestToolchainProvenance
$badProvenance.wixUtilExtension.version = '6.0.3'
Assert-Throws {
    Get-PackagePublicationToolchainProvenance -Provenance $badProvenance
} 'WiX extension version mutation' 'pinned WiX 6\.0\.2'
$badProvenance = New-TestToolchainProvenance
$badProvenance.wixUtilExtension.fileHashes['..\escape.dll'] = (('cd' * 32) -join '')
$badProvenance.wixUtilExtension.contentSha256 = Get-WixPayloadDigest -Hashes $badProvenance.wixUtilExtension.fileHashes
Assert-Throws {
    Get-PackagePublicationToolchainProvenance -Provenance $badProvenance
} 'WiX extension relative path escape' 'unsafe payload-relative path'

Write-Output 'PASS: Parent WP12 package identity, MSI lifecycle boundary, pinned provenance, and no-authority contracts.'
