function New-MsiContractSignature {
    param(
        [Parameter(Mandatory)]
        [AllowEmptyCollection()]
        [object[]]$Values
    )

    return (($Values | ForEach-Object { [string]$_ }) -join [char]0x1f)
}

function Assert-ExactMsiRowSet {
    param(
        [Parameter(Mandatory)]
        [AllowEmptyCollection()]
        [object[]]$Rows,

        [Parameter(Mandatory)]
        [AllowEmptyCollection()]
        [string[]]$Fields,

        [Parameter(Mandatory)]
        [AllowEmptyCollection()]
        [string[]]$ExpectedSignatures,

        [Parameter(Mandatory)]
        [string]$Description
    )

    $actualCounts = [System.Collections.Generic.Dictionary[string, int]]::new([System.StringComparer]::Ordinal)
    foreach ($row in $Rows) {
        $values = @($Fields | ForEach-Object { [string]$row.$_ })
        $signature = New-MsiContractSignature -Values $values
        if ($actualCounts.ContainsKey($signature)) {
            $actualCounts[$signature]++
        } else {
            $actualCounts.Add($signature, 1)
        }
    }
    $expectedCounts = [System.Collections.Generic.Dictionary[string, int]]::new([System.StringComparer]::Ordinal)
    foreach ($signature in $ExpectedSignatures) {
        if ($expectedCounts.ContainsKey($signature)) {
            $expectedCounts[$signature]++
        } else {
            $expectedCounts.Add($signature, 1)
        }
    }

    if ($Rows.Count -ne $ExpectedSignatures.Count) {
        throw "$Description has $($Rows.Count) rows; expected exactly $($ExpectedSignatures.Count)."
    }
    foreach ($signature in $actualCounts.Keys) {
        if (-not $expectedCounts.ContainsKey($signature) -or $actualCounts[$signature] -ne $expectedCounts[$signature]) {
            $display = $signature.Replace([char]0x1f, '|')
            throw "$Description contains unexpected or duplicate row '$display'."
        }
    }
    foreach ($signature in $expectedCounts.Keys) {
        if (-not $actualCounts.ContainsKey($signature) -or $actualCounts[$signature] -ne $expectedCounts[$signature]) {
            $display = $signature.Replace([char]0x1f, '|')
            throw "$Description is missing exact row '$display'."
        }
    }
}

function Assert-MsiAllowlistedTableContent {
    param(
        [Parameter(Mandatory)]
        [object]$Database,

        [Parameter(Mandatory)]
        [string]$ExpectedVersion,

        [Parameter(Mandatory)]
        [string]$ExpectedProductCode,

        [Parameter(Mandatory)]
        [string]$ExpectedRegistryId,

        [Parameter(Mandatory)]
        [string]$BrokerBinaryPath,

        [Parameter(Mandatory)]
        [string]$ProvisionerBinaryPath
    )

    # The table allowlist is an exact content contract, not only a schema
    # contract. Every allowlisted table is queried here, including the tables
    # WiX emits empty for this package. This prevents a later authoring change
    # from silently adding a row to an otherwise unused MSI table.
    $propertyFields = @('Property', 'Value')
    $propertyRows = @(Get-MsiRows -Database $Database -Query 'SELECT `Property`,`Value` FROM `Property`' -FieldNames $propertyFields)
    $expectedProperties = @(
        New-MsiContractSignature @('ALLUSERS', '1')
        New-MsiContractSignature @('Manufacturer', 'Ocentra')
        New-MsiContractSignature @('ProductCode', $ExpectedProductCode)
        New-MsiContractSignature @('ProductLanguage', '1033')
        New-MsiContractSignature @('ProductName', 'Ocentra Parent Protected Capability Custody')
        New-MsiContractSignature @('ProductVersion', $ExpectedVersion)
        New-MsiContractSignature @('UpgradeCode', '{A1BA5AA2-F5DB-4B97-9889-4BB4DBF52B3C}')
        New-MsiContractSignature @('SecureCustomProperties', 'WIX_DOWNGRADE_DETECTED;WIX_UPGRADE_DETECTED')
        New-MsiContractSignature @('MsiHiddenProperties', 'RunProtectedProvisioner')
    )
    Assert-ExactMsiRowSet -Rows $propertyRows -Fields $propertyFields -ExpectedSignatures $expectedProperties -Description 'Normalized MSI Property table'

    $upgradeFields = @('UpgradeCode', 'VersionMin', 'VersionMax', 'Language', 'Attributes', 'Remove', 'ActionProperty')
    $upgradeRows = @(Get-MsiRows -Database $Database -Query 'SELECT `UpgradeCode`,`VersionMin`,`VersionMax`,`Language`,`Attributes`,`Remove`,`ActionProperty` FROM `Upgrade`' -FieldNames $upgradeFields)
    $expectedUpgrades = @(
        New-MsiContractSignature @('{A1BA5AA2-F5DB-4B97-9889-4BB4DBF52B3C}', '', $ExpectedVersion, '', '1', '', 'WIX_UPGRADE_DETECTED')
        New-MsiContractSignature @('{A1BA5AA2-F5DB-4B97-9889-4BB4DBF52B3C}', $ExpectedVersion, '', '', '2', '', 'WIX_DOWNGRADE_DETECTED')
    )
    Assert-ExactMsiRowSet -Rows $upgradeRows -Fields $upgradeFields -ExpectedSignatures $expectedUpgrades -Description 'Normalized MSI Upgrade table'

    $launchFields = @('Condition', 'Description')
    $launchRows = @(Get-MsiRows -Database $Database -Query 'SELECT `Condition`,`Description` FROM `LaunchCondition`' -FieldNames $launchFields)
    Assert-ExactMsiRowSet -Rows $launchRows -Fields $launchFields -ExpectedSignatures @(
        New-MsiContractSignature @('NOT WIX_DOWNGRADE_DETECTED', 'A newer Ocentra Parent protected custody package is already installed.')
    ) -Description 'Normalized MSI LaunchCondition table'

    $directoryFields = @('Directory', 'DirectoryParent', 'DefaultDir')
    $directoryRows = @(Get-MsiRows -Database $Database -Query 'SELECT `Directory`,`Directory_Parent`,`DefaultDir` FROM `Directory`' -FieldNames $directoryFields)
    Assert-ExactMsiRowSet -Rows $directoryRows -Fields $directoryFields -ExpectedSignatures @(
        New-MsiContractSignature @('INSTALLFOLDER', 'OcentraProgramFilesFolder', 'qhfblgok|OcentraParent')
        New-MsiContractSignature @('OcentraProgramFilesFolder', 'ProgramFiles64Folder', 'Ocentra')
        New-MsiContractSignature @('ProgramFiles64Folder', 'TARGETDIR', 'PFiles64')
        New-MsiContractSignature @('CUSTODYDATAFOLDER', 'PARENTDATAROOT', 'ghz6zqz1|protected-capability-custody')
        New-MsiContractSignature @('PARENTDATAROOT', 'OcentraProgramDataFolder', 'sk162dvc|OcentraParent')
        New-MsiContractSignature @('OcentraProgramDataFolder', 'CommonAppDataFolder', 'Ocentra')
        New-MsiContractSignature @('CommonAppDataFolder', 'TARGETDIR', 'CommApp')
        New-MsiContractSignature @('TARGETDIR', '', 'SourceDir')
    ) -Description 'Normalized MSI Directory table'

    $createFolderFields = @('Directory', 'Component')
    $createFolderRows = @(Get-MsiRows -Database $Database -Query 'SELECT `Directory_`,`Component_` FROM `CreateFolder`' -FieldNames $createFolderFields)
    Assert-ExactMsiRowSet -Rows $createFolderRows -Fields $createFolderFields -ExpectedSignatures @(
        New-MsiContractSignature @('INSTALLFOLDER', 'ProtectedInstallDirectory')
        New-MsiContractSignature @('CUSTODYDATAFOLDER', 'ProtectedCustodyDataDirectory')
    ) -Description 'Normalized MSI CreateFolder table'

    $featureFields = @('Feature', 'FeatureParent', 'Title', 'Description', 'Display', 'Level', 'Directory', 'Attributes')
    $featureRows = @(Get-MsiRows -Database $Database -Query 'SELECT `Feature`,`Feature_Parent`,`Title`,`Description`,`Display`,`Level`,`Directory_`,`Attributes` FROM `Feature`' -FieldNames $featureFields)
    Assert-ExactMsiRowSet -Rows $featureRows -Fields $featureFields -ExpectedSignatures @(
        New-MsiContractSignature @('ProtectedCustodyFeature', '', 'Ocentra Parent protected custody', '', '2', '1', '', '0')
    ) -Description 'Normalized MSI Feature table'

    $featureComponentFields = @('Feature', 'Component')
    $featureComponentRows = @(Get-MsiRows -Database $Database -Query 'SELECT `Feature_`,`Component_` FROM `FeatureComponents`' -FieldNames $featureComponentFields)
    Assert-ExactMsiRowSet -Rows $featureComponentRows -Fields $featureComponentFields -ExpectedSignatures @(
        New-MsiContractSignature @('ProtectedCustodyFeature', 'ProtectedBrokerService')
        New-MsiContractSignature @('ProtectedCustodyFeature', 'ProtectedCustodyDataDirectory')
        New-MsiContractSignature @('ProtectedCustodyFeature', 'ProtectedInstallDirectory')
        New-MsiContractSignature @('ProtectedCustodyFeature', 'ProtectedProvisioner')
        New-MsiContractSignature @('ProtectedCustodyFeature', 'ProtectedRegistryIdentity')
        New-MsiContractSignature @('ProtectedCustodyFeature', 'ProtectedRegistryRoot')
    ) -Description 'Normalized MSI FeatureComponents table'

    $componentFields = @('Component', 'ComponentId', 'Directory', 'Attributes', 'Condition', 'KeyPath')
    $componentRows = @(Get-MsiRows -Database $Database -Query 'SELECT `Component`,`ComponentId`,`Directory_`,`Attributes`,`Condition`,`KeyPath` FROM `Component`' -FieldNames $componentFields)
    Assert-ExactMsiRowSet -Rows $componentRows -Fields $componentFields -ExpectedSignatures @(
        New-MsiContractSignature @('ProtectedBrokerService', '{65D9B85B-1DBA-4B42-86F1-DB8F20BD4F51}', 'INSTALLFOLDER', '256', '', 'ProtectedBrokerFile')
        New-MsiContractSignature @('ProtectedCustodyDataDirectory', '{A2D4CEFE-44C4-4E33-B96C-6C3AE5BAA6A9}', 'CUSTODYDATAFOLDER', '272', '', '')
        New-MsiContractSignature @('ProtectedInstallDirectory', '{2B86B79F-72B4-4C7A-9B5E-14A3FC1EF1B6}', 'INSTALLFOLDER', '272', '', '')
        New-MsiContractSignature @('ProtectedProvisioner', '{87B06C30-3A3A-44B3-B6B8-F1F6F0A4DFA2}', 'INSTALLFOLDER', '256', '', 'ProtectedProvisionerFile')
        New-MsiContractSignature @('ProtectedRegistryIdentity', '{6D51662C-3D41-4694-8FEF-2548B0DE9DCE}', 'INSTALLFOLDER', '276', '', 'regxxeh8sZMWNeOQS6NQLf1_.sApTM')
        New-MsiContractSignature @('ProtectedRegistryRoot', '{C7393427-4FD2-46A8-8A19-CF6ABF4E48A2}', 'INSTALLFOLDER', '276', '', 'regozpvmfX_NrEkN4q_wqmS1tt2.4I')
    ) -Description 'Normalized MSI Component table'

    $brokerSize = [string](Get-Item -LiteralPath $BrokerBinaryPath).Length
    $provisionerSize = [string](Get-Item -LiteralPath $ProvisionerBinaryPath).Length
    $fileFields = @('File', 'Component', 'FileName', 'FileSize', 'Version', 'Language', 'Attributes', 'Sequence')
    $fileRows = @(Get-MsiRows -Database $Database -Query 'SELECT `File`,`Component_`,`FileName`,`FileSize`,`Version`,`Language`,`Attributes`,`Sequence` FROM `File`' -FieldNames $fileFields)
    Assert-ExactMsiRowSet -Rows $fileRows -Fields $fileFields -ExpectedSignatures @(
        New-MsiContractSignature @('ProtectedBrokerFile', 'ProtectedBrokerService', 'ocentr~1.exe|ocentra-protected-capability-custody-broker.exe', $brokerSize, '', '', '512', '1')
        New-MsiContractSignature @('ProtectedProvisionerFile', 'ProtectedProvisioner', 'ocentr~2.exe|ocentra-protected-capability-custody-provisioner.exe', $provisionerSize, '', '', '512', '2')
    ) -Description 'Normalized MSI File table'

    $serviceInstallFields = @('ServiceInstall', 'Name', 'DisplayName', 'ServiceType', 'StartType', 'ErrorControl', 'LoadOrderGroup', 'Dependencies', 'StartName', 'Password', 'Arguments', 'Component', 'Description')
    $serviceInstallRows = @(Get-MsiRows -Database $Database -Query 'SELECT `ServiceInstall`,`Name`,`DisplayName`,`ServiceType`,`StartType`,`ErrorControl`,`LoadOrderGroup`,`Dependencies`,`StartName`,`Password`,`Arguments`,`Component_`,`Description` FROM `ServiceInstall`' -FieldNames $serviceInstallFields)
    Assert-ExactMsiRowSet -Rows $serviceInstallRows -Fields $serviceInstallFields -ExpectedSignatures @(
        New-MsiContractSignature @('ProtectedBrokerServiceInstall', 'OcentraProtectedCapabilityCustodyBroker', 'Ocentra Protected Capability Custody Broker', '16', '2', '32771', '', '', 'LocalSystem', '', '', 'ProtectedBrokerService', 'Local protected custody broker for Ocentra Parent.')
    ) -Description 'Normalized MSI ServiceInstall table'

    $serviceControlFields = @('ServiceControl', 'Name', 'Event', 'Arguments', 'Wait', 'Component')
    $serviceControlRows = @(Get-MsiRows -Database $Database -Query 'SELECT `ServiceControl`,`Name`,`Event`,`Arguments`,`Wait`,`Component_` FROM `ServiceControl`' -FieldNames $serviceControlFields)
    Assert-ExactMsiRowSet -Rows $serviceControlRows -Fields $serviceControlFields -ExpectedSignatures @(
        New-MsiContractSignature @('ProtectedBrokerServiceControl', 'OcentraProtectedCapabilityCustodyBroker', '163', '', '1', 'ProtectedBrokerService')
    ) -Description 'Normalized MSI ServiceControl table'

    $customActionFields = @('Action', 'Type', 'Source', 'Target', 'ExtendedType')
    $customActionRows = @(Get-MsiRows -Database $Database -Query 'SELECT `Action`,`Type`,`Source`,`Target`,`ExtendedType` FROM `CustomAction`' -FieldNames $customActionFields)
    Assert-ExactMsiRowSet -Rows $customActionRows -Fields $customActionFields -ExpectedSignatures @(
        New-MsiContractSignature @('RunProtectedProvisioner', '11282', 'ProtectedProvisionerFile', '', '')
        New-MsiContractSignature @('Wix4SchedServiceConfig_X64', '1', 'Wix4UtilCA_X64', 'SchedServiceConfig', '')
        New-MsiContractSignature @('Wix4ExecServiceConfig_X64', '3073', 'Wix4UtilCA_X64', 'ExecServiceConfig', '')
        New-MsiContractSignature @('Wix4RollbackServiceConfig_X64', '3329', 'Wix4UtilCA_X64', 'RollbackServiceConfig', '')
    ) -Description 'Normalized MSI CustomAction table'

    $wixServiceFields = @('ServiceName', 'Component', 'NewService', 'First', 'Second', 'Third', 'ResetDays', 'RestartSeconds', 'ProgramCommandLine', 'RebootMessage')
    $wixServiceRows = @(Get-MsiRows -Database $Database -Query 'SELECT `ServiceName`,`Component_`,`NewService`,`FirstFailureActionType`,`SecondFailureActionType`,`ThirdFailureActionType`,`ResetPeriodInDays`,`RestartServiceDelayInSeconds`,`ProgramCommandLine`,`RebootMessage` FROM `Wix4ServiceConfig`' -FieldNames $wixServiceFields)
    Assert-ExactMsiRowSet -Rows $wixServiceRows -Fields $wixServiceFields -ExpectedSignatures @(
        New-MsiContractSignature @('OcentraProtectedCapabilityCustodyBroker', 'ProtectedBrokerService', '1', 'restart', 'restart', 'restart', '1', '10', '', '')
    ) -Description 'Normalized MSI Wix4ServiceConfig table'

    $registryIdentityKey = "Software\Ocentra\ProtectedCapabilityCustody\$ExpectedRegistryId"
    $registryFields = @('Registry', 'Root', 'Key', 'Name', 'Value', 'Component')
    $registryRows = @(Get-MsiRows -Database $Database -Query 'SELECT `Registry`,`Root`,`Key`,`Name`,`Value`,`Component_` FROM `Registry`' -FieldNames $registryFields)
    Assert-ExactMsiRowSet -Rows $registryRows -Fields $registryFields -ExpectedSignatures @(
        New-MsiContractSignature @('regozpvmfX_NrEkN4q_wqmS1tt2.4I', '2', 'Software\Ocentra\ProtectedCapabilityCustody', '+', '', 'ProtectedRegistryRoot')
        New-MsiContractSignature @('regxxeh8sZMWNeOQS6NQLf1_.sApTM', '2', $registryIdentityKey, '+', '', 'ProtectedRegistryIdentity')
        New-MsiContractSignature @('regkP_Jbt438.5ExKeM2XCSMBVllIw', '2', 'Software\Ocentra\ProtectedCapabilityCustody', 'package-boundary', 'parent-protected-custody-v1', 'ProtectedRegistryRoot')
        New-MsiContractSignature @('regtTGIb_gK5eVPxxvq3qt45QHEHyE', '2', $registryIdentityKey, 'package-boundary', 'parent-protected-custody-v1', 'ProtectedRegistryIdentity')
    ) -Description 'Normalized MSI Registry table'

    $installSequenceFields = @('Action', 'Condition', 'Sequence')
    $installSequenceRows = @(Get-MsiRows -Database $Database -Query 'SELECT `Action`,`Condition`,`Sequence` FROM `InstallExecuteSequence`' -FieldNames $installSequenceFields)
    Assert-ExactMsiRowSet -Rows $installSequenceRows -Fields $installSequenceFields -ExpectedSignatures @(
        New-MsiContractSignature @('RunProtectedProvisioner', 'NOT REMOVE~="ALL"', '5899')
        New-MsiContractSignature @('Wix4SchedServiceConfig_X64', 'NOT REMOVE~="ALL" AND VersionNT > 400', '5801')
        New-MsiContractSignature @('CostInitialize', '', '800')
        New-MsiContractSignature @('FileCost', '', '900')
        New-MsiContractSignature @('CostFinalize', '', '1000')
        New-MsiContractSignature @('InstallValidate', '', '1400')
        New-MsiContractSignature @('InstallInitialize', '', '1500')
        New-MsiContractSignature @('InstallFiles', '', '4000')
        New-MsiContractSignature @('InstallFinalize', '', '6600')
        New-MsiContractSignature @('PublishFeatures', '', '6300')
        New-MsiContractSignature @('PublishProduct', '', '6400')
        New-MsiContractSignature @('FindRelatedProducts', '', '25')
        New-MsiContractSignature @('LaunchConditions', '', '100')
        New-MsiContractSignature @('ValidateProductID', '', '700')
        New-MsiContractSignature @('MigrateFeatureStates', '', '1200')
        New-MsiContractSignature @('ProcessComponents', '', '1600')
        New-MsiContractSignature @('UnpublishFeatures', '', '1800')
        New-MsiContractSignature @('StopServices', 'VersionNT', '1900')
        New-MsiContractSignature @('DeleteServices', 'VersionNT', '2000')
        New-MsiContractSignature @('RemoveRegistryValues', '', '2600')
        New-MsiContractSignature @('RemoveFiles', '', '3500')
        New-MsiContractSignature @('RemoveFolders', '', '3600')
        New-MsiContractSignature @('CreateFolders', '', '3700')
        New-MsiContractSignature @('WriteRegistryValues', '', '5000')
        New-MsiContractSignature @('InstallServices', 'VersionNT', '5800')
        New-MsiContractSignature @('StartServices', 'VersionNT', '5900')
        New-MsiContractSignature @('RegisterUser', '', '6000')
        New-MsiContractSignature @('RegisterProduct', '', '6100')
        New-MsiContractSignature @('RemoveExistingProducts', '', '1401')
    ) -Description 'Normalized MSI InstallExecuteSequence table'

    $mediaFields = @('DiskId', 'LastSequence', 'DiskPrompt', 'Cabinet', 'VolumeLabel', 'Source')
    $mediaRows = @(Get-MsiRows -Database $Database -Query 'SELECT `DiskId`,`LastSequence`,`DiskPrompt`,`Cabinet`,`VolumeLabel`,`Source` FROM `Media`' -FieldNames $mediaFields)
    Assert-ExactMsiRowSet -Rows $mediaRows -Fields $mediaFields -ExpectedSignatures @(
        New-MsiContractSignature @('1', '2', '', '#cab1.cab', '', '')
    ) -Description 'Normalized MSI Media table'

    $lockFields = @('Id', 'LockObject', 'Table', 'Sddl', 'Condition')
    $lockRows = @(Get-MsiRows -Database $Database -Query 'SELECT `MsiLockPermissionsEx`,`LockObject`,`Table`,`SDDLText`,`Condition` FROM `MsiLockPermissionsEx`' -FieldNames $lockFields)
    $trustedInstallerSid = 'S-1-5-80-956008885-3418522649-1831038044-1853292631-2271478464'
    $serviceSddl = 'O:S-1-5-18G:S-1-5-18D:P(A;;CCDCLCSWRPWPDTLOCRRC;;;S-1-5-18)'
    $installFolderSddl = "O:$trustedInstallerSid`G:$trustedInstallerSid`D:P(A;;FA;;;S-1-5-18)(A;;FA;;;$trustedInstallerSid)"
    $dataFolderSddl = 'O:S-1-5-18G:S-1-5-18D:P(A;;FA;;;S-1-5-18)'
    $registryOwnerSddl = "O:$trustedInstallerSid`G:$trustedInstallerSid`D:P(A;;0x20019;;;S-1-5-18)(A;;0xF003F;;;$trustedInstallerSid)"
    Assert-ExactMsiRowSet -Rows $lockRows -Fields $lockFields -ExpectedSignatures @(
        New-MsiContractSignature @('pmeE8Qv5NyzAhksL_QQs7tOVAydpCE', 'ProtectedBrokerServiceInstall', 'ServiceInstall', $serviceSddl, '')
        New-MsiContractSignature @('pmeOjXv8hfnxrhOlGpzKfCz9tJ4LGw', 'INSTALLFOLDER', 'CreateFolder', $installFolderSddl, '')
        New-MsiContractSignature @('pmek06rvwVULLlt2_wt0e0Towrs9W0', 'CUSTODYDATAFOLDER', 'CreateFolder', $dataFolderSddl, '')
        New-MsiContractSignature @('pmei5sGgYqPUxBwNKyfjwd9A2bexvM', 'regozpvmfX_NrEkN4q_wqmS1tt2.4I', 'Registry', $registryOwnerSddl, '')
        New-MsiContractSignature @('pmebk5tH1BaoWD_7hzuR7yzdJyl5MU', 'regxxeh8sZMWNeOQS6NQLf1_.sApTM', 'Registry', $registryOwnerSddl, '')
    ) -Description 'Normalized MSI MsiLockPermissionsEx table'

    # The WiX Util ServiceConfig extension owns one fixed Binary row for its
    # scheduled custom actions. Keep that row exact here; the decompile gate
    # separately requires that its Data stream is a non-empty payload. The
    # remaining tables below are intentionally empty and reject any future
    # extension or authoring drift.
    $binaryFields = @('Name')
    $binaryRows = @(Get-MsiRows -Database $Database -Query 'SELECT `Name` FROM `Binary`' -FieldNames $binaryFields)
    Assert-ExactMsiRowSet -Rows $binaryRows -Fields $binaryFields -ExpectedSignatures @(
        New-MsiContractSignature @('Wix4UtilCA_X64')
    ) -Description 'Normalized MSI Binary table identity'

    foreach ($emptyTable in @(
            @('RemoveFile', @('FileKey', 'Component', 'FileName', 'DirProperty', 'InstallMode'), 'SELECT `FileKey`,`Component_`,`FileName`,`DirProperty`,`InstallMode` FROM `RemoveFile`'),
            @('MsiFileHash', @('File', 'Options', 'HashPart1', 'HashPart2', 'HashPart3', 'HashPart4'), 'SELECT `File_`,`Options`,`HashPart1`,`HashPart2`,`HashPart3`,`HashPart4` FROM `MsiFileHash`'),
            @('AdminUISequence', @('Action', 'Condition', 'Sequence'), 'SELECT `Action`,`Condition`,`Sequence` FROM `AdminUISequence`'),
            @('AdminExecuteSequence', @('Action', 'Condition', 'Sequence'), 'SELECT `Action`,`Condition`,`Sequence` FROM `AdminExecuteSequence`'),
            @('AdvtExecuteSequence', @('Action', 'Condition', 'Sequence'), 'SELECT `Action`,`Condition`,`Sequence` FROM `AdvtExecuteSequence`'),
            @('InstallUISequence', @('Action', 'Condition', 'Sequence'), 'SELECT `Action`,`Condition`,`Sequence` FROM `InstallUISequence`')
        )) {
        $emptyRows = @(Get-MsiRows -Database $Database -Query $emptyTable[2] -FieldNames $emptyTable[1])
        Assert-ExactMsiRowSet -Rows $emptyRows -Fields $emptyTable[1] -ExpectedSignatures @() -Description "Normalized MSI $($emptyTable[0]) table (expected-empty)"
    }

    # _Validation is the one large table whose exact row contract is defined
    # from the Windows Installer schema metadata tuples; keep that contract in
    # one place and require it for the same allowlisted-table content gate.
    Assert-MsiValidationSchemaContract -Database $Database
}

function Assert-MsiExecutableLifecycleContract {
    param(
        [Parameter(Mandatory)]
        [object]$Database,

        [Parameter(Mandatory)]
        [string]$ExpectedVersion,

        [Parameter(Mandatory)]
        [string]$ExpectedRegistryId,

        [Parameter(Mandatory)]
        [string]$ExpectedBrokerHash
    )

    $directoryFields = @('Directory', 'DirectoryParent', 'DefaultDir')
    $directoryRows = @(Get-MsiRows -Database $Database -Query 'SELECT `Directory`,`Directory_Parent`,`DefaultDir` FROM `Directory`' -FieldNames $directoryFields)
    $expectedDirectories = @(
        New-MsiContractSignature @('INSTALLFOLDER', 'OcentraProgramFilesFolder', 'qhfblgok|OcentraParent')
        New-MsiContractSignature @('OcentraProgramFilesFolder', 'ProgramFiles64Folder', 'Ocentra')
        New-MsiContractSignature @('ProgramFiles64Folder', 'TARGETDIR', 'PFiles64')
        New-MsiContractSignature @('CUSTODYDATAFOLDER', 'PARENTDATAROOT', 'ghz6zqz1|protected-capability-custody')
        New-MsiContractSignature @('PARENTDATAROOT', 'OcentraProgramDataFolder', 'sk162dvc|OcentraParent')
        New-MsiContractSignature @('OcentraProgramDataFolder', 'CommonAppDataFolder', 'Ocentra')
        New-MsiContractSignature @('CommonAppDataFolder', 'TARGETDIR', 'CommApp')
        New-MsiContractSignature @('TARGETDIR', '', 'SourceDir')
    )
    Assert-ExactMsiRowSet -Rows $directoryRows -Fields $directoryFields -ExpectedSignatures $expectedDirectories -Description 'Normalized MSI Directory table'

    $createFolderFields = @('Directory', 'Component')
    $createFolderRows = @(Get-MsiRows -Database $Database -Query 'SELECT `Directory_`,`Component_` FROM `CreateFolder`' -FieldNames $createFolderFields)
    $expectedCreateFolders = @(
        New-MsiContractSignature @('INSTALLFOLDER', 'ProtectedInstallDirectory')
        New-MsiContractSignature @('CUSTODYDATAFOLDER', 'ProtectedCustodyDataDirectory')
    )
    Assert-ExactMsiRowSet -Rows $createFolderRows -Fields $createFolderFields -ExpectedSignatures $expectedCreateFolders -Description 'Normalized MSI CreateFolder table'

    $featureFields = @('Feature', 'FeatureParent', 'Title', 'Description', 'Display', 'Level', 'Directory', 'Attributes')
    $featureRows = @(Get-MsiRows -Database $Database -Query 'SELECT `Feature`,`Feature_Parent`,`Title`,`Description`,`Display`,`Level`,`Directory_`,`Attributes` FROM `Feature`' -FieldNames $featureFields)
    $expectedFeatures = @(
        New-MsiContractSignature @('ProtectedCustodyFeature', '', 'Ocentra Parent protected custody', '', '2', '1', '', '0')
    )
    Assert-ExactMsiRowSet -Rows $featureRows -Fields $featureFields -ExpectedSignatures $expectedFeatures -Description 'Normalized MSI Feature table'

    $featureComponentFields = @('Feature', 'Component')
    $featureComponentRows = @(Get-MsiRows -Database $Database -Query 'SELECT `Feature_`,`Component_` FROM `FeatureComponents`' -FieldNames $featureComponentFields)
    $expectedFeatureComponents = @(
        New-MsiContractSignature @('ProtectedCustodyFeature', 'ProtectedBrokerService')
        New-MsiContractSignature @('ProtectedCustodyFeature', 'ProtectedCustodyDataDirectory')
        New-MsiContractSignature @('ProtectedCustodyFeature', 'ProtectedInstallDirectory')
        New-MsiContractSignature @('ProtectedCustodyFeature', 'ProtectedProvisioner')
        New-MsiContractSignature @('ProtectedCustodyFeature', 'ProtectedRegistryIdentity')
        New-MsiContractSignature @('ProtectedCustodyFeature', 'ProtectedRegistryRoot')
    )
    Assert-ExactMsiRowSet -Rows $featureComponentRows -Fields $featureComponentFields -ExpectedSignatures $expectedFeatureComponents -Description 'Normalized MSI FeatureComponents table'

    $customActionFields = @('Action', 'Type', 'Source', 'Target', 'ExtendedType')
    $customActionRows = @(Get-MsiRows -Database $Database -Query 'SELECT `Action`,`Type`,`Source`,`Target`,`ExtendedType` FROM `CustomAction`' -FieldNames $customActionFields)
    $expectedCustomActions = @(
        New-MsiContractSignature @('RunProtectedProvisioner', '11282', 'ProtectedProvisionerFile', '', '')
        New-MsiContractSignature @('Wix4SchedServiceConfig_X64', '1', 'Wix4UtilCA_X64', 'SchedServiceConfig', '')
        New-MsiContractSignature @('Wix4ExecServiceConfig_X64', '3073', 'Wix4UtilCA_X64', 'ExecServiceConfig', '')
        New-MsiContractSignature @('Wix4RollbackServiceConfig_X64', '3329', 'Wix4UtilCA_X64', 'RollbackServiceConfig', '')
    )
    Assert-ExactMsiRowSet -Rows $customActionRows -Fields $customActionFields -ExpectedSignatures $expectedCustomActions -Description 'Normalized MSI CustomAction table'

    $sequenceFields = @('Action', 'Condition', 'Sequence')
    $sequenceRows = @(Get-MsiRows -Database $Database -Query 'SELECT `Action`,`Condition`,`Sequence` FROM `InstallExecuteSequence`' -FieldNames $sequenceFields)
    $expectedSequenceRows = @(
        New-MsiContractSignature @('RunProtectedProvisioner', 'NOT REMOVE~="ALL"', '5899')
        New-MsiContractSignature @('Wix4SchedServiceConfig_X64', 'NOT REMOVE~="ALL" AND VersionNT > 400', '5801')
        New-MsiContractSignature @('CostInitialize', '', '800')
        New-MsiContractSignature @('FileCost', '', '900')
        New-MsiContractSignature @('CostFinalize', '', '1000')
        New-MsiContractSignature @('InstallValidate', '', '1400')
        New-MsiContractSignature @('InstallInitialize', '', '1500')
        New-MsiContractSignature @('InstallFiles', '', '4000')
        New-MsiContractSignature @('InstallFinalize', '', '6600')
        New-MsiContractSignature @('PublishFeatures', '', '6300')
        New-MsiContractSignature @('PublishProduct', '', '6400')
        New-MsiContractSignature @('FindRelatedProducts', '', '25')
        New-MsiContractSignature @('LaunchConditions', '', '100')
        New-MsiContractSignature @('ValidateProductID', '', '700')
        New-MsiContractSignature @('MigrateFeatureStates', '', '1200')
        New-MsiContractSignature @('ProcessComponents', '', '1600')
        New-MsiContractSignature @('UnpublishFeatures', '', '1800')
        New-MsiContractSignature @('StopServices', 'VersionNT', '1900')
        New-MsiContractSignature @('DeleteServices', 'VersionNT', '2000')
        New-MsiContractSignature @('RemoveRegistryValues', '', '2600')
        New-MsiContractSignature @('RemoveFiles', '', '3500')
        New-MsiContractSignature @('RemoveFolders', '', '3600')
        New-MsiContractSignature @('CreateFolders', '', '3700')
        New-MsiContractSignature @('WriteRegistryValues', '', '5000')
        New-MsiContractSignature @('InstallServices', 'VersionNT', '5800')
        New-MsiContractSignature @('StartServices', 'VersionNT', '5900')
        New-MsiContractSignature @('RegisterUser', '', '6000')
        New-MsiContractSignature @('RegisterProduct', '', '6100')
        New-MsiContractSignature @('RemoveExistingProducts', '', '1401')
    )
    Assert-ExactMsiRowSet -Rows $sequenceRows -Fields $sequenceFields -ExpectedSignatures $expectedSequenceRows -Description 'Normalized MSI InstallExecuteSequence table'

    $serviceControlFields = @('ServiceControl', 'Name', 'Event', 'Arguments', 'Wait', 'Component')
    $serviceControlRows = @(Get-MsiRows -Database $Database -Query 'SELECT `ServiceControl`,`Name`,`Event`,`Arguments`,`Wait`,`Component_` FROM `ServiceControl`' -FieldNames $serviceControlFields)
    $expectedServiceControls = @(
        New-MsiContractSignature @('ProtectedBrokerServiceControl', 'OcentraProtectedCapabilityCustodyBroker', '163', '', '1', 'ProtectedBrokerService')
    )
    Assert-ExactMsiRowSet -Rows $serviceControlRows -Fields $serviceControlFields -ExpectedSignatures $expectedServiceControls -Description 'Normalized MSI ServiceControl table'

    $serviceInstallFields = @('ServiceInstall', 'Name', 'DisplayName', 'ServiceType', 'StartType', 'ErrorControl', 'LoadOrderGroup', 'Dependencies', 'StartName', 'Password', 'Arguments', 'Component')
    $serviceInstallRows = @(Get-MsiRows -Database $Database -Query 'SELECT `ServiceInstall`,`Name`,`DisplayName`,`ServiceType`,`StartType`,`ErrorControl`,`LoadOrderGroup`,`Dependencies`,`StartName`,`Password`,`Arguments`,`Component_` FROM `ServiceInstall`' -FieldNames $serviceInstallFields)
    $expectedServiceInstalls = @(
        New-MsiContractSignature @('ProtectedBrokerServiceInstall', 'OcentraProtectedCapabilityCustodyBroker', 'Ocentra Protected Capability Custody Broker', '16', '2', '32771', '', '', 'LocalSystem', '', '', 'ProtectedBrokerService')
    )
    Assert-ExactMsiRowSet -Rows $serviceInstallRows -Fields $serviceInstallFields -ExpectedSignatures $expectedServiceInstalls -Description 'Normalized MSI ServiceInstall table'

    $lockFields = @('Id', 'LockObject', 'Table', 'Sddl', 'Condition')
    $lockRows = @(Get-MsiRows -Database $Database -Query 'SELECT `MsiLockPermissionsEx`,`LockObject`,`Table`,`SDDLText`,`Condition` FROM `MsiLockPermissionsEx`' -FieldNames $lockFields)
    $trustedInstallerSid = 'S-1-5-80-956008885-3418522649-1831038044-1853292631-2271478464'
    $serviceSddl = 'O:S-1-5-18G:S-1-5-18D:P(A;;CCDCLCSWRPWPDTLOCRRC;;;S-1-5-18)'
    $installFolderSddl = "O:$trustedInstallerSid`G:$trustedInstallerSid`D:P(A;;FA;;;S-1-5-18)(A;;FA;;;$trustedInstallerSid)"
    $dataFolderSddl = 'O:S-1-5-18G:S-1-5-18D:P(A;;FA;;;S-1-5-18)'
    $registryOwnerSddl = "O:$trustedInstallerSid`G:$trustedInstallerSid`D:P(A;;0x20019;;;S-1-5-18)(A;;0xF003F;;;$trustedInstallerSid)"
    $expectedLocks = @(
        New-MsiContractSignature @('pmeE8Qv5NyzAhksL_QQs7tOVAydpCE', 'ProtectedBrokerServiceInstall', 'ServiceInstall', $serviceSddl, '')
        New-MsiContractSignature @('pmeOjXv8hfnxrhOlGpzKfCz9tJ4LGw', 'INSTALLFOLDER', 'CreateFolder', $installFolderSddl, '')
        New-MsiContractSignature @('pmek06rvwVULLlt2_wt0e0Towrs9W0', 'CUSTODYDATAFOLDER', 'CreateFolder', $dataFolderSddl, '')
        New-MsiContractSignature @('pmei5sGgYqPUxBwNKyfjwd9A2bexvM', 'regozpvmfX_NrEkN4q_wqmS1tt2.4I', 'Registry', $registryOwnerSddl, '')
        New-MsiContractSignature @('pmebk5tH1BaoWD_7hzuR7yzdJyl5MU', 'regxxeh8sZMWNeOQS6NQLf1_.sApTM', 'Registry', $registryOwnerSddl, '')
    )
    Assert-ExactMsiRowSet -Rows $lockRows -Fields $lockFields -ExpectedSignatures $expectedLocks -Description 'Normalized MSI MsiLockPermissionsEx table'

    $identityKey = "Software\Ocentra\ProtectedCapabilityCustody\$ExpectedRegistryId"
    $registryFields = @('Registry', 'Root', 'Key', 'Name', 'Value', 'Component')
    $registryRows = @(Get-MsiRows -Database $Database -Query 'SELECT `Registry`,`Root`,`Key`,`Name`,`Value`,`Component_` FROM `Registry`' -FieldNames $registryFields)
    $expectedRegistryRows = @(
        New-MsiContractSignature @('regozpvmfX_NrEkN4q_wqmS1tt2.4I', '2', 'Software\Ocentra\ProtectedCapabilityCustody', '+', '', 'ProtectedRegistryRoot')
        New-MsiContractSignature @('regxxeh8sZMWNeOQS6NQLf1_.sApTM', '2', $identityKey, '+', '', 'ProtectedRegistryIdentity')
        New-MsiContractSignature @('regkP_Jbt438.5ExKeM2XCSMBVllIw', '2', 'Software\Ocentra\ProtectedCapabilityCustody', 'package-boundary', 'parent-protected-custody-v1', 'ProtectedRegistryRoot')
        New-MsiContractSignature @('regtTGIb_gK5eVPxxvq3qt45QHEHyE', '2', $identityKey, 'package-boundary', 'parent-protected-custody-v1', 'ProtectedRegistryIdentity')
    )
    Assert-ExactMsiRowSet -Rows $registryRows -Fields $registryFields -ExpectedSignatures $expectedRegistryRows -Description 'Normalized MSI Registry table'

    $wixServiceFields = @('ServiceName', 'Component', 'NewService', 'First', 'Second', 'Third', 'ResetDays', 'RestartSeconds', 'ProgramCommandLine', 'RebootMessage')
    $wixServiceRows = @(Get-MsiRows -Database $Database -Query 'SELECT `ServiceName`,`Component_`,`NewService`,`FirstFailureActionType`,`SecondFailureActionType`,`ThirdFailureActionType`,`ResetPeriodInDays`,`RestartServiceDelayInSeconds`,`ProgramCommandLine`,`RebootMessage` FROM `Wix4ServiceConfig`' -FieldNames $wixServiceFields)
    $expectedWixServiceRows = @(
        New-MsiContractSignature @('OcentraProtectedCapabilityCustodyBroker', 'ProtectedBrokerService', '1', 'restart', 'restart', 'restart', '1', '10', '', '')
    )
    Assert-ExactMsiRowSet -Rows $wixServiceRows -Fields $wixServiceFields -ExpectedSignatures $expectedWixServiceRows -Description 'Normalized MSI Wix4ServiceConfig table'

    $upgradeFields = @('UpgradeCode', 'VersionMin', 'VersionMax', 'Language', 'Attributes', 'Remove', 'ActionProperty')
    $upgradeRows = @(Get-MsiRows -Database $Database -Query 'SELECT `UpgradeCode`,`VersionMin`,`VersionMax`,`Language`,`Attributes`,`Remove`,`ActionProperty` FROM `Upgrade`' -FieldNames $upgradeFields)
    $expectedUpgrades = @(
        New-MsiContractSignature @('{A1BA5AA2-F5DB-4B97-9889-4BB4DBF52B3C}', '', $ExpectedVersion, '', '1', '', 'WIX_UPGRADE_DETECTED')
        New-MsiContractSignature @('{A1BA5AA2-F5DB-4B97-9889-4BB4DBF52B3C}', $ExpectedVersion, '', '', '2', '', 'WIX_DOWNGRADE_DETECTED')
    )
    Assert-ExactMsiRowSet -Rows $upgradeRows -Fields $upgradeFields -ExpectedSignatures $expectedUpgrades -Description 'Normalized MSI Upgrade table'

    $launchFields = @('Condition', 'Description')
    $launchRows = @(Get-MsiRows -Database $Database -Query 'SELECT `Condition`,`Description` FROM `LaunchCondition`' -FieldNames $launchFields)
    $expectedLaunchRows = @(
        New-MsiContractSignature @('NOT WIX_DOWNGRADE_DETECTED', 'A newer Ocentra Parent protected custody package is already installed.')
    )
    Assert-ExactMsiRowSet -Rows $launchRows -Fields $launchFields -ExpectedSignatures $expectedLaunchRows -Description 'Normalized MSI LaunchCondition table'
}

function Assert-MsiMediaContract {
    param(
        [Parameter(Mandatory)]
        [object]$Database
    )

    $mediaFields = @('DiskId', 'LastSequence', 'DiskPrompt', 'Cabinet', 'VolumeLabel', 'Source')
    $mediaRows = @(Get-MsiRows -Database $Database -Query 'SELECT DiskId,LastSequence,DiskPrompt,Cabinet,VolumeLabel,Source FROM Media' -FieldNames $mediaFields)
    $expectedMedia = @(
        New-MsiContractSignature @('1', '2', '', '#cab1.cab', '', '')
    )
    Assert-ExactMsiRowSet -Rows $mediaRows -Fields $mediaFields -ExpectedSignatures $expectedMedia -Description 'Normalized MSI Media table'
}
