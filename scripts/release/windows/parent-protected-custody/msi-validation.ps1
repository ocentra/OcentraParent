. (Join-Path $PSScriptRoot 'msi-contract.ps1')

function Release-MsiComObject {
    param(
        [Parameter()]
        [AllowNull()]
        [object]$ComObject
    )

    if ($null -ne $ComObject) {
        try {
            [System.Runtime.InteropServices.Marshal]::FinalReleaseComObject($ComObject) | Out-Null
        } catch {
            # COM cleanup must not hide the validation failure that caused the
            # caller to leave its database/view scope.
        }
    }
}

function Set-DeterministicMsiSummary {
    param(
        [Parameter(Mandatory)]
        [string]$Path,

        [Parameter(Mandatory)]
        [string]$PackageCode
    )

    # WiX emits mutable SummaryInformation.PackageCode and current
    # Created/LastSaved metadata. Normalize only the supported fields through
    # the Windows Installer COM API, then require a repeated-build byte
    # comparison. This helper alone is not a reproducibility claim.
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
        Release-MsiComObject -ComObject $summary
        Release-MsiComObject -ComObject $database
        Release-MsiComObject -ComObject $installer
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

function Read-CfbUInt16Le {
    param(
        [Parameter(Mandatory)]
        [byte[]]$Bytes,

        [Parameter(Mandatory)]
        [int]$Offset
    )

    if ($Offset -lt 0 -or $Offset -gt ($Bytes.Length - 2)) {
        throw "CFB UInt16 read at offset $Offset is outside the MSI byte bounds."
    }
    $value = [uint32]$Bytes[$Offset]
    $value = $value -bor (([uint32]$Bytes[$Offset + 1]) -shl 8)
    return [uint16]$value
}

function Read-CfbUInt32Le {
    param(
        [Parameter(Mandatory)]
        [byte[]]$Bytes,

        [Parameter(Mandatory)]
        [int]$Offset
    )

    if ($Offset -lt 0 -or $Offset -gt ($Bytes.Length - 4)) {
        throw "CFB UInt32 read at offset $Offset is outside the MSI byte bounds."
    }
    [uint64]$value = $Bytes[$Offset]
    $value = $value -bor (([uint64]$Bytes[$Offset + 1]) -shl 8)
    $value = $value -bor (([uint64]$Bytes[$Offset + 2]) -shl 16)
    $value = $value -bor (([uint64]$Bytes[$Offset + 3]) -shl 24)
    return [uint32]$value
}

function Normalize-CfbRootModifiedFileTime {
    param(
        [Parameter(Mandatory)]
        [string]$Path
    )

    if (-not [System.BitConverter]::IsLittleEndian) {
        throw 'CFB normalization requires a little-endian host; refusing to mutate the MSI.'
    }
    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "Cannot normalize missing MSI '$Path'."
    }

    $bytes = [System.IO.File]::ReadAllBytes($Path)
    if ($bytes.Length -lt 512) {
        throw "MSI '$Path' is shorter than the CFB header; refusing to mutate it."
    }

    $signature = [byte[]](0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1)
    for ($index = 0; $index -lt $signature.Length; $index++) {
        if ($bytes[$index] -ne $signature[$index]) {
            throw "MSI '$Path' does not have the expected CFB signature; refusing to mutate it."
        }
    }

    $byteOrder = Read-CfbUInt16Le -Bytes $bytes -Offset 28
    if ($byteOrder -ne [uint16]0xFFFE) {
        throw "MSI '$Path' has an invalid CFB byte order 0x$($byteOrder.ToString('X4')); refusing to mutate it."
    }

    $majorVersion = Read-CfbUInt16Le -Bytes $bytes -Offset 26
    $sectorShift = Read-CfbUInt16Le -Bytes $bytes -Offset 30
    if (($majorVersion -eq 3 -and $sectorShift -ne 9) -or ($majorVersion -eq 4 -and $sectorShift -ne 12)) {
        throw "MSI '$Path' has an invalid CFB version/sector shift ($majorVersion/$sectorShift); refusing to mutate it."
    }
    if ($majorVersion -ne 3 -and $majorVersion -ne 4) {
        throw "MSI '$Path' has unsupported CFB major version $majorVersion; refusing to mutate it."
    }
    $miniSectorShift = Read-CfbUInt16Le -Bytes $bytes -Offset 32
    if ($miniSectorShift -ne 6) {
        throw "MSI '$Path' has an invalid CFB mini-sector shift $miniSectorShift; refusing to mutate it."
    }

    [uint64]$sectorSize = [uint64]1 -shl $sectorShift
    [uint32]$firstDirectorySector = Read-CfbUInt32Le -Bytes $bytes -Offset 48
    [uint32]$firstReservedSector = 4294967292
    if ($firstDirectorySector -ge $firstReservedSector) {
        throw "MSI '$Path' has no valid first CFB directory sector; refusing to mutate it."
    }

    [uint64]$fileLength = $bytes.LongLength
    [uint64]$directoryOffset = ([uint64]$firstDirectorySector + 1) * $sectorSize
    if ($directoryOffset -lt [uint64]$firstDirectorySector -or $directoryOffset -gt $fileLength) {
        throw "MSI '$Path' has an out-of-bounds CFB directory offset; refusing to mutate it."
    }
    [uint64]$directoryEnd = $directoryOffset + 128
    if ($directoryEnd -lt $directoryOffset -or $directoryEnd -gt $fileLength -or $directoryOffset -gt [uint64][int]::MaxValue) {
        throw "MSI '$Path' has an out-of-bounds CFB root directory entry; refusing to mutate it."
    }
    $directoryEntryOffset = [int]$directoryOffset

    $nameLength = Read-CfbUInt16Le -Bytes $bytes -Offset ($directoryEntryOffset + 64)
    if ($nameLength -lt 2 -or $nameLength -gt 64 -or ($nameLength % 2) -ne 0) {
        throw "MSI '$Path' has an invalid CFB root directory name length $nameLength; refusing to mutate it."
    }
    if ($bytes[$directoryEntryOffset + $nameLength - 2] -ne 0 -or $bytes[$directoryEntryOffset + $nameLength - 1] -ne 0) {
        throw "MSI '$Path' has an unterminated CFB root directory name; refusing to mutate it."
    }
    $rootName = [System.Text.Encoding]::Unicode.GetString($bytes, $directoryEntryOffset, $nameLength - 2)
    if ($rootName -cne 'Root Entry') {
        throw "MSI '$Path' root CFB directory entry is '$rootName', not 'Root Entry'; refusing to mutate it."
    }
    if ($bytes[$directoryEntryOffset + 66] -ne 5) {
        throw "MSI '$Path' root CFB directory entry has an invalid object type; refusing to mutate it."
    }

    [uint64]$modifiedFileTimeOffset = $directoryOffset + 108
    if ($modifiedFileTimeOffset -lt $directoryOffset -or $modifiedFileTimeOffset + 8 -gt $fileLength) {
        throw "MSI '$Path' has an out-of-bounds root modified FILETIME; refusing to mutate it."
    }
    $modifiedFileTimeOffset = [int]$modifiedFileTimeOffset
    $changed = $false
    for ($index = 0; $index -lt 8; $index++) {
        if ($bytes[$modifiedFileTimeOffset + $index] -ne 0) {
            $bytes[$modifiedFileTimeOffset + $index] = 0
            $changed = $true
        }
    }
    if (-not $changed) {
        return
    }

    $temporaryPath = "$Path.cfb-normalized.tmp"
    $backupPath = "$Path.cfb-normalized.bak"
    if (Test-Path -LiteralPath $temporaryPath) {
        throw "CFB normalization temporary path '$temporaryPath' already exists; refusing to overwrite it."
    }
    if (Test-Path -LiteralPath $backupPath) {
        throw "CFB normalization backup path '$backupPath' already exists; refusing to overwrite it."
    }
    try {
        $stream = $null
        try {
            $stream = [System.IO.FileStream]::new(
                $temporaryPath,
                [System.IO.FileMode]::CreateNew,
                [System.IO.FileAccess]::Write,
                [System.IO.FileShare]::None
            )
            $stream.Write($bytes, 0, $bytes.Length)
            $stream.Flush($true)
        } finally {
            if ($null -ne $stream) {
                $stream.Dispose()
            }
        }
        [System.IO.File]::Replace($temporaryPath, $Path, $backupPath, $true)
        if (Test-Path -LiteralPath $backupPath) {
            Remove-Item -LiteralPath $backupPath -Force
        }
    } catch {
        if (Test-Path -LiteralPath $temporaryPath) {
            Remove-Item -LiteralPath $temporaryPath -Force
        }
        if (Test-Path -LiteralPath $backupPath) {
            Remove-Item -LiteralPath $backupPath -Force
        }
        throw "Safe CFB root FILETIME normalization failed for '$Path': $($_.Exception.Message)"
    }

    $normalizedBytes = [System.IO.File]::ReadAllBytes($Path)
    for ($index = 0; $index -lt 8; $index++) {
        if ($normalizedBytes[$modifiedFileTimeOffset + $index] -ne 0) {
            throw "CFB root modified FILETIME was not fully normalized in '$Path'."
        }
    }
}

function Get-MsiRows {
    param(
        [Parameter(Mandatory)]
        [object]$Database,

        [Parameter(Mandatory)]
        [string]$Query,

        [Parameter(Mandatory)]
        [string[]]$FieldNames
    )

    $view = $null
    try {
        $view = $Database.OpenView($Query)
        if ($null -eq $view) {
            throw "Windows Installer returned no view for '$Query'."
        }
        $view.Execute() | Out-Null
        $rows = [System.Collections.Generic.List[object]]::new()
        while ($true) {
            $record = $view.Fetch()
            if ($null -eq $record) {
                break
            }
            try {
                $row = [ordered]@{}
                for ($index = 0; $index -lt $FieldNames.Count; $index++) {
                    $value = $record.StringData($index + 1)
                    if ($null -eq $value) {
                        $value = ''
                    }
                    $row[$FieldNames[$index]] = [string]$value
                }
                $rows.Add([pscustomobject]$row)
            } finally {
                Release-MsiComObject -ComObject $record
            }
        }
        return @($rows)
    } finally {
        Release-MsiComObject -ComObject $view
    }
}

function Assert-MsiRowValue {
    param(
        [Parameter(Mandatory)]
        [object]$Row,

        [Parameter(Mandatory)]
        [string]$Field,

        [Parameter(Mandatory)]
        [AllowEmptyString()]
        [string]$Expected,

        [Parameter(Mandatory)]
        [string]$Description
    )

    $actual = [string]$Row.$Field
    if ($actual -cne $Expected) {
        throw "$Description expected '$Expected' but found '$actual'."
    }
}

function Assert-MsiRequiredTables {
    param(
        [Parameter(Mandatory)]
        [string[]]$TableNames
    )

    $required = @(
        'Component',
        'CreateFolder',
        'CustomAction',
        'Directory',
        'Feature',
        'FeatureComponents',
        'File',
        'InstallExecuteSequence',
        'LaunchCondition',
        'MsiLockPermissionsEx',
        'Property',
        'Registry',
        'ServiceControl',
        'ServiceInstall',
        'Upgrade',
        'Wix4ServiceConfig'
    )
    foreach ($tableName in $required) {
        if ($TableNames -notcontains $tableName) {
            throw "Normalized MSI is missing required fixed table '$tableName'."
        }
    }
    if ($TableNames -contains 'MsiServiceConfig') {
        throw 'Normalized MSI contains deprecated MsiServiceConfig; ServiceSid must remain an external Protected WP02 owner operation.'
    }
}

function Assert-MsiFixedContent {
    param(
        [Parameter(Mandatory)]
        [object]$Database,

        [Parameter(Mandatory)]
        [string]$CandidateMsiPath,

        [Parameter(Mandatory)]
        [string]$CandidateIntermediatePath,

        [Parameter(Mandatory)]
        [string]$DotnetCommand,

        [Parameter(Mandatory)]
        [string]$ExpectedVersion,

        [Parameter(Mandatory)]
        [string]$ExpectedProductCode,

        [Parameter(Mandatory)]
        [string]$ExpectedBrokerHash,

        [Parameter(Mandatory)]
        [string]$ExpectedProvisionerHash,

        [Parameter(Mandatory)]
        [string]$ExpectedRegistryId,

        [Parameter(Mandatory)]
        [string]$BrokerBinaryPath,

        [Parameter(Mandatory)]
        [string]$ProvisionerBinaryPath
    )

    $tableRows = @(Get-MsiRows -Database $Database -Query 'SELECT `Name` FROM `_Tables`' -FieldNames @('Name'))
    $tableNames = @($tableRows | ForEach-Object { $_.Name })
    Assert-MsiRequiredTables -TableNames $tableNames
    Assert-MsiExecutableLifecycleContract -Database $Database -ExpectedVersion $ExpectedVersion -ExpectedRegistryId $ExpectedRegistryId -ExpectedBrokerHash $ExpectedBrokerHash

    $propertyRows = @(Get-MsiRows -Database $Database -Query 'SELECT `Property`,`Value` FROM `Property`' -FieldNames @('Property', 'Value'))
    $expectedProperties = [ordered]@{
        ALLUSERS = '1'
        Manufacturer = 'Ocentra'
        ProductCode = $ExpectedProductCode
        ProductLanguage = '1033'
        ProductName = 'Ocentra Parent Protected Capability Custody'
        ProductVersion = $ExpectedVersion
        UpgradeCode = '{A1BA5AA2-F5DB-4B97-9889-4BB4DBF52B3C}'
        SecureCustomProperties = 'WIX_DOWNGRADE_DETECTED;WIX_UPGRADE_DETECTED'
        MsiHiddenProperties = 'RunProtectedProvisioner'
    }
    if ($propertyRows.Count -ne $expectedProperties.Count) {
        throw "Normalized MSI Property table has $($propertyRows.Count) rows; expected exactly $($expectedProperties.Count)."
    }
    $properties = @{}
    foreach ($row in $propertyRows) {
        if ($properties.ContainsKey([string]$row.Property)) {
            throw "Normalized MSI Property table contains duplicate '$($row.Property)'."
        }
        $properties[[string]$row.Property] = [string]$row.Value
    }
    foreach ($propertyName in $expectedProperties.Keys) {
        if (-not $properties.ContainsKey($propertyName)) {
            throw "Normalized MSI Property table is missing fixed property '$propertyName'."
        }
        if ($properties[$propertyName] -cne $expectedProperties[$propertyName]) {
            throw "Normalized MSI property '$propertyName' expected '$($expectedProperties[$propertyName])' but found '$($properties[$propertyName])'."
        }
    }

    $componentRows = @(Get-MsiRows -Database $Database -Query 'SELECT `Component`,`ComponentId`,`Directory_`,`Attributes`,`KeyPath` FROM `Component`' -FieldNames @('Component', 'ComponentId', 'Directory', 'Attributes', 'KeyPath'))
    $expectedComponents = [ordered]@{
        ProtectedBrokerService = @('{65D9B85B-1DBA-4B42-86F1-DB8F20BD4F51}', 'INSTALLFOLDER', '256', 'ProtectedBrokerFile')
        ProtectedCustodyDataDirectory = @('{A2D4CEFE-44C4-4E33-B96C-6C3AE5BAA6A9}', 'CUSTODYDATAFOLDER', '272', '')
        ProtectedInstallDirectory = @('{2B86B79F-72B4-4C7A-9B5E-14A3FC1EF1B6}', 'INSTALLFOLDER', '272', '')
        ProtectedProvisioner = @('{87B06C30-3A3A-44B3-B6B8-F1F6F0A4DFA2}', 'INSTALLFOLDER', '256', 'ProtectedProvisionerFile')
        ProtectedRegistryIdentity = @('{6D51662C-3D41-4694-8FEF-2548B0DE9DCE}', 'INSTALLFOLDER', '276', '')
        ProtectedRegistryRoot = @('{C7393427-4FD2-46A8-8A19-CF6ABF4E48A2}', 'INSTALLFOLDER', '276', '')
        ProtectedRegistryRuntime = @('{C5C0C36B-3DAB-4C6C-8C67-BB5297F14F25}', 'INSTALLFOLDER', '276', '')
    }
    if ($componentRows.Count -ne $expectedComponents.Count) {
        throw "Normalized MSI Component table has $($componentRows.Count) rows; expected exactly $($expectedComponents.Count)."
    }
    foreach ($row in $componentRows) {
        $componentName = [string]$row.Component
        if (-not $expectedComponents.Contains($componentName)) {
            throw "Normalized MSI contains unexpected component '$componentName'."
        }
        $expected = $expectedComponents[$componentName]
        Assert-MsiRowValue -Row $row -Field ComponentId -Expected $expected[0] -Description "Component '$componentName' identity"
        Assert-MsiRowValue -Row $row -Field Directory -Expected $expected[1] -Description "Component '$componentName' directory"
        Assert-MsiRowValue -Row $row -Field Attributes -Expected $expected[2] -Description "Component '$componentName' attributes"
        if ([string]$expected[3] -ne '' -and [string]$row.KeyPath -cne [string]$expected[3]) {
            throw "Component '$componentName' has key path '$($row.KeyPath)', not '$($expected[3])'."
        }
        if ([string]$expected[3] -eq '' -and $componentName.StartsWith('ProtectedRegistry', [System.StringComparison]::Ordinal) -and [string]$row.KeyPath -notmatch '^reg') {
            throw "Registry component '$componentName' does not have a generated registry key path."
        }
    }

    $brokerSize = [string](Get-Item -LiteralPath $BrokerBinaryPath).Length
    $provisionerSize = [string](Get-Item -LiteralPath $ProvisionerBinaryPath).Length
    $fileRows = @(Get-MsiRows -Database $Database -Query 'SELECT `File`,`Component_`,`FileName`,`FileSize`,`Sequence` FROM `File`' -FieldNames @('File', 'Component', 'FileName', 'FileSize', 'Sequence'))
    if ($fileRows.Count -ne 2) {
        throw "Normalized MSI File table has $($fileRows.Count) rows; expected exactly two fixed payload files."
    }
    $expectedFiles = @{
        ProtectedBrokerFile = @('ProtectedBrokerService', 'ocentra-protected-capability-custody-broker.exe', $brokerSize, '1', $ExpectedBrokerHash)
        ProtectedProvisionerFile = @('ProtectedProvisioner', 'ocentra-protected-capability-custody-provisioner.exe', $provisionerSize, '2', $ExpectedProvisionerHash)
    }
    foreach ($row in $fileRows) {
        $fileId = [string]$row.File
        if (-not $expectedFiles.ContainsKey($fileId)) {
            throw "Normalized MSI contains unexpected payload file '$fileId'."
        }
        $expected = $expectedFiles[$fileId]
        Assert-MsiRowValue -Row $row -Field Component -Expected $expected[0] -Description "File '$fileId' component"
        $fileNameParts = ([string]$row.FileName) -split '\|', 2
        if ($fileNameParts.Count -ne 2 -or $fileNameParts[1] -cne $expected[1]) {
            throw "File '$fileId' has unexpected short/long name '$($row.FileName)'."
        }
        Assert-MsiRowValue -Row $row -Field FileSize -Expected $expected[2] -Description "File '$fileId' size"
        Assert-MsiRowValue -Row $row -Field Sequence -Expected $expected[3] -Description "File '$fileId' sequence"
    }

    $serviceRows = @(Get-MsiRows -Database $Database -Query 'SELECT `ServiceInstall`,`Name`,`DisplayName`,`ServiceType`,`StartType`,`ErrorControl`,`LoadOrderGroup`,`Dependencies`,`StartName`,`Arguments`,`Component_` FROM `ServiceInstall`' -FieldNames @('ServiceInstall', 'Name', 'DisplayName', 'ServiceType', 'StartType', 'ErrorControl', 'LoadOrderGroup', 'Dependencies', 'StartName', 'Arguments', 'Component'))
    if ($serviceRows.Count -ne 1) {
        throw "Normalized MSI ServiceInstall table has $($serviceRows.Count) rows; expected exactly one broker service."
    }
    $service = $serviceRows[0]
    $serviceValues = [ordered]@{
        ServiceInstall = 'ProtectedBrokerServiceInstall'
        Name = 'OcentraProtectedCapabilityCustodyBroker'
        DisplayName = 'Ocentra Protected Capability Custody Broker'
        ServiceType = '16'
        StartType = '2'
        ErrorControl = '32771'
        LoadOrderGroup = ''
        Dependencies = ''
        StartName = 'LocalSystem'
        Arguments = ''
        Component = 'ProtectedBrokerService'
    }
    foreach ($field in $serviceValues.Keys) {
        Assert-MsiRowValue -Row $service -Field $field -Expected $serviceValues[$field] -Description "Broker ServiceInstall '$field'"
    }

    $customActionRows = @(Get-MsiRows -Database $Database -Query 'SELECT `Action`,`Type`,`Source`,`Target` FROM `CustomAction`' -FieldNames @('Action', 'Type', 'Source', 'Target'))
    $provisionerActions = @($customActionRows | Where-Object { $_.Action -eq 'RunProtectedProvisioner' })
    if ($provisionerActions.Count -ne 1) {
        throw "Normalized MSI must contain exactly one RunProtectedProvisioner custom action."
    }
    $provisionerAction = $provisionerActions[0]
    Assert-MsiRowValue -Row $provisionerAction -Field Type -Expected '11282' -Description 'RunProtectedProvisioner custom action type'
    Assert-MsiRowValue -Row $provisionerAction -Field Source -Expected 'ProtectedProvisionerFile' -Description 'RunProtectedProvisioner custom action source'
    Assert-MsiRowValue -Row $provisionerAction -Field Target -Expected '' -Description 'RunProtectedProvisioner custom action target'

    $sequenceRows = @(Get-MsiRows -Database $Database -Query 'SELECT `Action`,`Condition`,`Sequence` FROM `InstallExecuteSequence`' -FieldNames @('Action', 'Condition', 'Sequence'))
    $runSequenceRows = @($sequenceRows | Where-Object { $_.Action -eq 'RunProtectedProvisioner' })
    $startSequenceRows = @($sequenceRows | Where-Object { $_.Action -eq 'StartServices' })
    if ($runSequenceRows.Count -ne 1 -or $startSequenceRows.Count -ne 1) {
        throw 'Normalized MSI must schedule RunProtectedProvisioner and StartServices exactly once.'
    }
    Assert-MsiRowValue -Row $runSequenceRows[0] -Field Condition -Expected 'NOT REMOVE~="ALL"' -Description 'RunProtectedProvisioner schedule condition'
    if ([int]$runSequenceRows[0].Sequence -ge [int]$startSequenceRows[0].Sequence) {
        throw 'RunProtectedProvisioner must execute before StartServices.'
    }

    $registryRows = @(Get-MsiRows -Database $Database -Query 'SELECT `Root`,`Key`,`Name`,`Value` FROM `Registry`' -FieldNames @('Root', 'Key', 'Name', 'Value'))
    $identityKey = "Software\Ocentra\ProtectedCapabilityCustody\$ExpectedRegistryId"
    $runtimeKey = "$identityKey\Runtime"
    $expectedRegistryRows = @(
        "2|Software\Ocentra\ProtectedCapabilityCustody|+|",
        "2|$identityKey|+|",
        "2|$runtimeKey|+|",
        "2|Software\Ocentra\ProtectedCapabilityCustody|package-boundary|parent-protected-custody-v1",
        "2|$identityKey|package-boundary|parent-protected-custody-v1",
        "2|$runtimeKey|broker-image-sha256|#x$ExpectedBrokerHash"
    )
    $actualRegistryRows = @($registryRows | ForEach-Object { "$($_.Root)|$($_.Key)|$($_.Name)|$($_.Value)" })
    if ($actualRegistryRows.Count -ne $expectedRegistryRows.Count) {
        throw "Normalized MSI Registry table has $($actualRegistryRows.Count) rows; expected exactly $($expectedRegistryRows.Count)."
    }
    foreach ($expectedRegistryRow in $expectedRegistryRows) {
        if ($actualRegistryRows -notcontains $expectedRegistryRow) {
            throw "Normalized MSI is missing fixed registry row '$expectedRegistryRow'."
        }
    }
    foreach ($registryRow in $registryRows) {
        if (([string]$registryRow.Key) -match '(?i)Enrollment|authority-v1' -or ([string]$registryRow.Name) -match '(?i)Enrollment|authority-v1') {
            throw 'Normalized MSI attempts to author protected Enrollment authority; package ownership must remain external WP02.'
        }
    }

    $wixServiceRows = @(Get-MsiRows -Database $Database -Query 'SELECT `ServiceName`,`Component_`,`FirstFailureActionType`,`SecondFailureActionType`,`ThirdFailureActionType`,`ResetPeriodInDays`,`RestartServiceDelayInSeconds` FROM `Wix4ServiceConfig`' -FieldNames @('ServiceName', 'Component', 'FirstFailureActionType', 'SecondFailureActionType', 'ThirdFailureActionType', 'ResetPeriodInDays', 'RestartServiceDelayInSeconds'))
    if ($wixServiceRows.Count -ne 1) {
        throw "Normalized MSI Wix4ServiceConfig table has $($wixServiceRows.Count) rows; expected one supported failure-action row."
    }
    $wixService = $wixServiceRows[0]
    $wixServiceValues = [ordered]@{
        ServiceName = 'OcentraProtectedCapabilityCustodyBroker'
        Component = 'ProtectedBrokerService'
        FirstFailureActionType = 'restart'
        SecondFailureActionType = 'restart'
        ThirdFailureActionType = 'restart'
        ResetPeriodInDays = '1'
        RestartServiceDelayInSeconds = '10'
    }
    foreach ($field in $wixServiceValues.Keys) {
        Assert-MsiRowValue -Row $wixService -Field $field -Expected $wixServiceValues[$field] -Description "Wix4ServiceConfig '$field'"
    }

    $decompileRoot = Join-Path $CandidateIntermediatePath 'payload-validation'
    $decompiledSourcePath = Join-Path $CandidateIntermediatePath 'normalized-validation.wxs'
    foreach ($validationOutput in @($decompileRoot, $decompiledSourcePath)) {
        if (Test-Path -LiteralPath $validationOutput) {
            throw "Unique MSI validation output '$validationOutput' already exists; refusing to overwrite or reuse it."
        }
    }
    New-Item -ItemType Directory -Path $decompileRoot | Out-Null
    Invoke-CheckedCommand -Command $DotnetCommand -ArgumentList @(
        'wix',
        'msi',
        'decompile',
        $CandidateMsiPath,
        '-x',
        $decompileRoot,
        '-out',
        $decompiledSourcePath
    ) -FailureMessage 'Normalized MSI decompile and payload inspection failed'
    Assert-NonEmptyFile -Path $decompiledSourcePath -Description 'Normalized MSI decompile source'
    $extractedBrokerPath = Join-Path $decompileRoot 'File\ProtectedBrokerFile'
    $extractedProvisionerPath = Join-Path $decompileRoot 'File\ProtectedProvisionerFile'
    Assert-NonEmptyFile -Path $extractedBrokerPath -Description 'Extracted broker payload'
    Assert-NonEmptyFile -Path $extractedProvisionerPath -Description 'Extracted provisioner payload'
    $extractedBrokerHash = Get-Sha256Hex -Path $extractedBrokerPath
    $extractedProvisionerHash = Get-Sha256Hex -Path $extractedProvisionerPath
    if ($extractedBrokerHash -cne $ExpectedBrokerHash -or $extractedBrokerHash -cne (Get-Sha256Hex -Path $BrokerBinaryPath)) {
        throw "Extracted broker payload hash '$extractedBrokerHash' does not match the fixed source hash '$ExpectedBrokerHash'."
    }
    if ($extractedProvisionerHash -cne $ExpectedProvisionerHash -or $extractedProvisionerHash -cne (Get-Sha256Hex -Path $ProvisionerBinaryPath)) {
        throw "Extracted provisioner payload hash '$extractedProvisionerHash' does not match the fixed source hash '$ExpectedProvisionerHash'."
    }

    Invoke-CheckedCommand -Command $DotnetCommand -ArgumentList @(
        'wix',
        'msi',
        'validate',
        $CandidateMsiPath
    ) -FailureMessage 'Normalized MSI table/content validation failed'
}

function Assert-MsiCandidate {
    param(
        [Parameter(Mandatory)]
        [string]$CandidateMsiPath,

        [Parameter(Mandatory)]
        [string]$CandidateIntermediatePath,

        [Parameter(Mandatory)]
        [string]$DotnetCommand,

        [Parameter(Mandatory)]
        [string]$ExpectedVersion,

        [Parameter(Mandatory)]
        [string]$ExpectedProductCode,

        [Parameter(Mandatory)]
        [string]$ExpectedBrokerHash,

        [Parameter(Mandatory)]
        [string]$ExpectedProvisionerHash,

        [Parameter(Mandatory)]
        [string]$ExpectedRegistryId,

        [Parameter(Mandatory)]
        [string]$BrokerBinaryPath,

        [Parameter(Mandatory)]
        [string]$ProvisionerBinaryPath
    )

    $installer = $null
    $database = $null
    try {
        $installer = New-Object -ComObject WindowsInstaller.Installer
        $database = $installer.OpenDatabase((Resolve-Path -LiteralPath $CandidateMsiPath).Path, 0)
        if ($null -eq $database) {
            throw 'Windows Installer returned no database handle.'
        }
        $fixedContentArguments = @{
            Database = $database
            CandidateMsiPath = $CandidateMsiPath
            CandidateIntermediatePath = $CandidateIntermediatePath
            DotnetCommand = $DotnetCommand
            ExpectedVersion = $ExpectedVersion
            ExpectedProductCode = $ExpectedProductCode
            ExpectedBrokerHash = $ExpectedBrokerHash
            ExpectedProvisionerHash = $ExpectedProvisionerHash
            ExpectedRegistryId = $ExpectedRegistryId
            BrokerBinaryPath = $BrokerBinaryPath
            ProvisionerBinaryPath = $ProvisionerBinaryPath
        }
        Assert-MsiFixedContent @fixedContentArguments
    } catch {
        throw "Windows Installer could not inspect fixed normalized MSI '$CandidateMsiPath': $($_.Exception.Message)"
    } finally {
        Release-MsiComObject -ComObject $database
        Release-MsiComObject -ComObject $installer
    }
}
