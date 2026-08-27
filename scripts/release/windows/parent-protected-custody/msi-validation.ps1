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
        [string]$PackageCode,

        [Parameter(Mandatory)]
        [string]$PackageRoot
    )

    # WiX emits mutable SummaryInformation metadata. Set the complete
    # reproducibility contract through the Windows Installer COM API, then
    # require a repeated-build byte comparison. This helper alone is not a
    # reproducibility claim.
    Assert-SafePackageLeafPath -Path $Path -Root $PackageRoot -Description 'MSI summary target' | Out-Null
    $installer = $null
    $database = $null
    $summary = $null
    try {
        $installer = New-Object -ComObject WindowsInstaller.Installer
        $database = $installer.OpenDatabase((Resolve-Path -LiteralPath $Path).Path, 1)
        $summary = $database.SummaryInformation(20)
        $summary.Property(1) = 1252
        $summary.Property(2) = 'Installation Database'
        $summary.Property(3) = 'Ocentra Parent protected broker and owner-bound provisioner package'
        $summary.Property(4) = 'Ocentra'
        $summary.Property(5) = 'Installer'
        $summary.Property(6) = 'This installer database contains the logic and data required to install Ocentra Parent Protected Capability Custody.'
        $summary.Property(7) = 'x64;1033'
        $summary.Property(9) = $PackageCode
        $fixedDate = [DateTime]::ParseExact(
            '2000-01-01T00:00:00',
            'yyyy-MM-ddTHH:mm:ss',
            [Globalization.CultureInfo]::InvariantCulture
        )
        $summary.Property(12) = $fixedDate
        $summary.Property(13) = $fixedDate
        $summary.Property(14) = 500
        $summary.Property(15) = 3
        $summary.Property(18) = 'WiX Toolset (6.0.2.0)'
        $summary.Property(19) = 2
        Assert-SafePackageLeafPath -Path $Path -Root $PackageRoot -Description 'MSI summary target immediately before persist' | Out-Null
        $summary.Persist()
        Assert-SafePackageLeafPath -Path $Path -Root $PackageRoot -Description 'MSI summary target immediately before commit' | Out-Null
        $database.Commit()
    } catch {
        throw "Windows Installer metadata normalization is unavailable; refusing to emit a non-deterministic MSI: $($_.Exception.Message)"
    } finally {
        Release-MsiComObject -ComObject $summary
        Release-MsiComObject -ComObject $database
        Release-MsiComObject -ComObject $installer
    }
}

function Assert-MsiSummaryContract {
    param(
        [Parameter(Mandatory)]
        [object]$Database,

        [Parameter(Mandatory)]
        [string]$ExpectedPackageCode
    )

    $summary = $null
    try {
        $summary = $Database.SummaryInformation(20)
        $expected = [ordered]@{
            1 = '1252'
            2 = 'Installation Database'
            3 = 'Ocentra Parent protected broker and owner-bound provisioner package'
            4 = 'Ocentra'
            5 = 'Installer'
            6 = 'This installer database contains the logic and data required to install Ocentra Parent Protected Capability Custody.'
            7 = 'x64;1033'
            9 = $ExpectedPackageCode
            14 = '500'
            15 = '3'
            18 = 'WiX Toolset (6.0.2.0)'
            19 = '2'
        }
        foreach ($propertyId in $expected.Keys) {
            $actual = [string]$summary.Property([int]$propertyId)
            if ($actual -cne [string]$expected[$propertyId]) {
                throw "MSI SummaryInformation property $propertyId expected '$($expected[$propertyId])' but found '$actual'."
            }
        }
        $fixedDate = [DateTime]::ParseExact(
            '2000-01-01T00:00:00',
            'yyyy-MM-ddTHH:mm:ss',
            [Globalization.CultureInfo]::InvariantCulture
        )
        foreach ($propertyId in @(12, 13)) {
            $actualDate = [DateTime]$summary.Property($propertyId)
            if ($actualDate -ne $fixedDate) {
                throw "MSI SummaryInformation property $propertyId is not the fixed reproducibility timestamp."
            }
        }
        foreach ($propertyId in @(8, 10, 11, 16, 17)) {
            $unused = $summary.Property([int]$propertyId)
            if ($null -ne $unused) {
                throw "MSI SummaryInformation property $propertyId is outside the exact allowed property set."
            }
        }
    } finally {
        Release-MsiComObject -ComObject $summary
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
        [string]$Path,

        [Parameter(Mandatory)]
        [string]$PackageRoot
    )

    if (-not [System.BitConverter]::IsLittleEndian) {
        throw 'CFB normalization requires a little-endian host; refusing to mutate the MSI.'
    }
    Assert-SafePackageLeafPath -Path $Path -Root $PackageRoot -Description 'CFB normalization target' | Out-Null
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
    Assert-SafePackageLeafPath -Path $temporaryPath -Root $PackageRoot -Description 'CFB normalization temporary' | Out-Null
    Assert-SafePackageLeafPath -Path $backupPath -Root $PackageRoot -Description 'CFB normalization backup' | Out-Null
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
        Assert-SafePackageLeafPath -Path $temporaryPath -Root $PackageRoot -Description 'CFB temporary immediately before replace' | Out-Null
        Assert-SafePackageLeafPath -Path $Path -Root $PackageRoot -Description 'CFB target immediately before replace' | Out-Null
        Assert-SafePackageLeafPath -Path $backupPath -Root $PackageRoot -Description 'CFB backup immediately before replace' | Out-Null
        [System.IO.File]::Replace($temporaryPath, $Path, $backupPath, $true)
        if (Test-Path -LiteralPath $backupPath) {
            Remove-SafePackagePath -Path $backupPath -Root $PackageRoot -Description 'CFB normalization backup cleanup'
        }
    } catch {
        if (Test-Path -LiteralPath $temporaryPath) {
            Remove-SafePackagePath -Path $temporaryPath -Root $PackageRoot -Description 'CFB normalization temporary cleanup'
        }
        if (Test-Path -LiteralPath $backupPath) {
            throw "Safe CFB root FILETIME normalization stopped with preserved backup '$backupPath' for '$Path': $($_.Exception.Message)"
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

function Get-MsiAllowedTableColumns {
    # WiX emits the standard empty-table schemas even when this package has no
    # rows in those tables. Keep this contract exact so a new table/column or a
    # silently changed emitted order cannot pass merely because the used rows
    # still look correct.
    return [ordered]@{
        Property = @('Property', 'Value')
        Upgrade = @('UpgradeCode', 'VersionMin', 'VersionMax', 'Language', 'Attributes', 'Remove', 'ActionProperty')
        LaunchCondition = @('Condition', 'Description')
        Directory = @('Directory', 'Directory_Parent', 'DefaultDir')
        Feature = @('Feature', 'Feature_Parent', 'Title', 'Description', 'Display', 'Level', 'Directory_', 'Attributes')
        FeatureComponents = @('Feature_', 'Component_')
        File = @('File', 'Component_', 'FileName', 'FileSize', 'Version', 'Language', 'Attributes', 'Sequence')
        Component = @('Component', 'ComponentId', 'Directory_', 'Attributes', 'Condition', 'KeyPath')
        Wix4ServiceConfig = @('ServiceName', 'Component_', 'NewService', 'FirstFailureActionType', 'SecondFailureActionType', 'ThirdFailureActionType', 'ResetPeriodInDays', 'RestartServiceDelayInSeconds', 'ProgramCommandLine', 'RebootMessage')
        ServiceInstall = @('ServiceInstall', 'Name', 'DisplayName', 'ServiceType', 'StartType', 'ErrorControl', 'LoadOrderGroup', 'Dependencies', 'StartName', 'Password', 'Arguments', 'Component_', 'Description')
        ServiceControl = @('ServiceControl', 'Name', 'Event', 'Arguments', 'Wait', 'Component_')
        CustomAction = @('Action', 'Type', 'Source', 'Target', 'ExtendedType')
        Binary = @('Name', 'Data')
        CreateFolder = @('Directory_', 'Component_')
        RemoveFile = @('FileKey', 'Component_', 'FileName', 'DirProperty', 'InstallMode')
        Registry = @('Registry', 'Root', 'Key', 'Name', 'Value', 'Component_')
        AdminUISequence = @('Action', 'Condition', 'Sequence')
        AdminExecuteSequence = @('Action', 'Condition', 'Sequence')
        AdvtExecuteSequence = @('Action', 'Condition', 'Sequence')
        InstallUISequence = @('Action', 'Condition', 'Sequence')
        InstallExecuteSequence = @('Action', 'Condition', 'Sequence')
        Media = @('DiskId', 'LastSequence', 'DiskPrompt', 'Cabinet', 'VolumeLabel', 'Source')
        MsiFileHash = @('File_', 'Options', 'HashPart1', 'HashPart2', 'HashPart3', 'HashPart4')
        MsiLockPermissionsEx = @('MsiLockPermissionsEx', 'LockObject', 'Table', 'SDDLText', 'Condition')
        _Validation = @('Table', 'Column', 'Nullable', 'MinValue', 'MaxValue', 'KeyTable', 'KeyColumn', 'Category', 'Set', 'Description')
    }
}

function Get-MsiValidationMetadataContract {
    # Each tuple is Nullable, MinValue, MaxValue, KeyTable, KeyColumn,
    # Category, Set, Description. Empty strings are meaningful MSI metadata.
    # The values mirror the Windows Installer schema emitted by WiX 6.0.2;
    # they are checked by exact set equality below, not just spot-checked.
    $rows = [ordered]@{}
    function Add-MsiValidationMetadata {
        param(
            [string]$Table,
            [string]$Column,
            [string]$Nullable,
            [string]$MinValue,
            [string]$MaxValue,
            [string]$KeyTable,
            [string]$KeyColumn,
            [string]$Category,
            [string]$Set,
            [string]$Description
        )
        $key = "$Table`0$Column"
        if ($rows.Contains($key)) {
            throw "MSI validation metadata contract duplicates '$Table.$Column'."
        }
        $rows[$key] = [pscustomobject][ordered]@{
            Table = $Table
            Column = $Column
            Nullable = $Nullable
            MinValue = $MinValue
            MaxValue = $MaxValue
            KeyTable = $KeyTable
            KeyColumn = $KeyColumn
            Category = $Category
            Set = $Set
            Description = $Description
        }
    }

    $metadata = @{
        Property = @(
            @('Property','N','','','','','','Identifier','','Name of property, uppercase if settable by launcher or loader.'),
            @('Value','N','','','','','','Text','','String value for property.  Never null or empty.')
        )
        Upgrade = @(
            @('UpgradeCode','N','','','','','','Guid','','The UpgradeCode GUID belonging to the products in this set.'),
            @('VersionMin','Y','','','','','','Text','','The minimum ProductVersion of the products in this set.  The set may or may not include products with this particular version.'),
            @('VersionMax','Y','','','','','','Text','','The maximum ProductVersion of the products in this set.  The set may or may not include products with this particular version.'),
            @('Language','Y','','','','','','Language','','A comma-separated list of languages for either products in this set or products not in this set.'),
            @('Attributes','N','0','2147483647','','','','','','The attributes of this product set.'),
            @('Remove','Y','','','','','','Formatted','','The list of features to remove when uninstalling a product from this set.  The default is "ALL".'),
            @('ActionProperty','N','','','','','','UpperCase','','The property to set when a product in this set is found.')
        )
        LaunchCondition = @(
            @('Condition','N','','','','','','Condition','','Expression which must evaluate to TRUE in order for install to commence.'),
            @('Description','N','','','','','','Formatted','','Localizable text to display when condition fails and install must abort.')
        )
        Directory = @(
            @('Directory','N','','','','','','Identifier','','Unique identifier for directory entry, primary key. If a property by this name is defined, it contains the full path to the directory.'),
            @('Directory_Parent','Y','','','Directory','1','Identifier','','Reference to the entry in this table specifying the default parent directory. A record parented to itself or with a Null parent represents a root of the install tree.'),
            @('DefaultDir','N','','','','','','DefaultDir','','The default sub-path under parent''s path.')
        )
        Feature = @(
            @('Feature','N','','','','','','Identifier','','Primary key used to identify a particular feature record.'),
            @('Feature_Parent','Y','','','Feature','1','Identifier','','Optional key of a parent record in the same table. If the parent is not selected, then the record will not be installed. Null indicates a root item.'),
            @('Title','Y','','','','','','Text','','Short text identifying a visible feature item.'),
            @('Description','Y','','','','','','Text','','Longer descriptive text describing a visible feature item.'),
            @('Display','Y','0','32767','','','','','','Numeric sort order, used to force a specific display ordering.'),
            @('Level','N','0','32767','','','','','','The install level at which record will be initially selected. An install level of 0 will disable an item and prevent its display.'),
            @('Directory_','Y','','','Directory','1','UpperCase','','The name of the Directory that can be configured by the UI. A non-null value will enable the browse button.'),
            @('Attributes','N','','','','','','','0;1;2;4;5;6;8;9;10;16;17;18;20;21;22;24;25;26;32;33;34;36;37;38;48;49;50;52;53;54','Feature attributes')
        )
        FeatureComponents = @(
            @('Feature_','N','','','Feature','1','Identifier','','Foreign key into Feature table.'),
            @('Component_','N','','','Component','1','Identifier','','Foreign key into Component table.')
        )
        File = @(
            @('File','N','','','','','','Identifier','','Primary key, non-localized token, must match identifier in cabinet.  For uncompressed files, this field is ignored.'),
            @('Component_','N','','','Component','1','Identifier','','Foreign key referencing Component that controls the file.'),
            @('FileName','N','','','','','','Filename','','File name used for installation, may be localized.  This may contain a "short name|long name" pair.'),
            @('FileSize','N','0','2147483647','','','','','','Size of file in bytes (long integer).'),
            @('Version','Y','','','File','1','Version','','Version string for versioned files;  Blank for unversioned files.'),
            @('Language','Y','','','','','','Language','','List of decimal language Ids, comma-separated if more than one.'),
            @('Attributes','Y','0','32767','','','','','Integer containing bit flags representing file attributes (with the decimal value of each bit position in parentheses)'),
            @('Sequence','N','1','2147483647','','','','','','Sequence with respect to the media images; order must track cabinet order.')
        )
        Component = @(
            @('Component','N','','','','','','Identifier','','Primary key used to identify a particular component record.'),
            @('ComponentId','Y','','','','','','Guid','','A string GUID unique to this component, version, and language.'),
            @('Directory_','N','','','Directory','1','Identifier','','Required key of a Directory table record. This is actually a property name whose value contains the actual path, set either by the AppSearch action or with the default setting obtained from the Directory table.'),
            @('Attributes','N','','','','','','','','Remote execution option, one of irsEnum'),
            @('Condition','Y','','','','','','Condition','','A conditional statement that will disable this component if the specified condition evaluates to the ''True'' state. If a component is disabled, it will not be installed, regardless of the ''Action'' state associated with the component.'),
            @('KeyPath','Y','','','File;Registry;ODBCDataSource','1','Identifier','','Either the primary key into the File table, Registry table, or ODBCDataSource table. This extract path is stored when the component is installed, and is used to detect the presence of the component and to return the path to it.')
        )
        Wix4ServiceConfig = @(
            @('ServiceName','N','','','','','','Formatted','','Primary key, non-localized token'),
            @('Component_','N','','','Component','1','Identifier','','Foreign key, Component used to determine install state '),
            @('NewService','N','0','1','','','','','','Whether the affected service is being installed or already exists.'),
            @('FirstFailureActionType','N','','','','','','Text','','First failure action type for configured service to take.'),
            @('SecondFailureActionType','N','','','','','','Text','','Second failure action type for configured service to take.'),
            @('ThirdFailureActionType','N','','','','','','Text','','Third failure action type for configured service to take.'),
            @('ResetPeriodInDays','Y','0','','','','','Integer','','Period after which to reset the failure count for the service.'),
            @('RestartServiceDelayInSeconds','Y','0','','','','','Integer','','Period after which to restart the service after a given failure.'),
            @('ProgramCommandLine','Y','','','','','','Formatted','','Command line for program to run if failure action is RUN_COMMAND.'),
            @('RebootMessage','Y','','','','','','Text','','Message to show to users when rebooting if failure action is REBOOT.')
        )
        ServiceInstall = @(
            @('ServiceInstall','N','','','','','','Identifier','','Primary key, non-localized token.'),
            @('Name','N','','','','','','Formatted','','Internal Name of the Service'),
            @('DisplayName','Y','','','','','','Formatted','','External Name of the Service'),
            @('ServiceType','N','-2147483647','2147483647','','','','','','Type of the service'),
            @('StartType','N','0','4','','','','','','Type of the service'),
            @('ErrorControl','N','-2147483647','2147483647','','','','','','Severity of error if service fails to start'),
            @('LoadOrderGroup','Y','','','','','','Formatted','','LoadOrderGroup'),
            @('Dependencies','Y','','','','','','Formatted','','Other services this depends on to start.  Separate by [~], and end with [~][~]'),
            @('StartName','Y','','','','','','Formatted','','User or object name to run service as'),
            @('Password','Y','','','','','','Formatted','','password to run service with.  (with StartName)'),
            @('Arguments','Y','','','','','','Formatted','','Arguments to include in every start of the service, passed to WinMain'),
            @('Component_','N','','','Component','1','Identifier','','Required foreign key into the Component Table that controls the startup of the service'),
            @('Description','Y','','','','','','Text','','Description of service.')
        )
        ServiceControl = @(
            @('ServiceControl','N','','','','','','Identifier','','Primary key, non-localized token.'),
            @('Name','N','','','','','','Formatted','','Name of a service. /, \, comma and space are invalid'),
            @('Event','N','0','187','','','','','','Bit field:  Install:  0x1 = Start, 0x2 = Stop, 0x8 = Delete, Uninstall: 0x10 = Start, 0x20 = Stop, 0x80 = Delete'),
            @('Arguments','Y','','','','','','Formatted','','Arguments for the service.  Separate by [~].'),
            @('Wait','Y','0','1','','','','','','Boolean for whether to wait for the service to fully start'),
            @('Component_','N','','','Component','1','Identifier','','Required foreign key into the Component Table that controls the startup of the service')
        )
        CustomAction = @(
            @('Action','N','','','','','','Identifier','','Primary key, name of action, normally appears in sequence table unless private use.'),
            @('Type','N','1','32767','','','','','','The numeric custom action type, consisting of source location, code type, entry, option flags.'),
            @('Source','Y','','','','','','CustomSource','','The table reference of the source of the code.'),
            @('Target','Y','','','','','','Formatted','','Excecution parameter, depends on the type of custom action'),
            @('ExtendedType','Y','0','2147483647','','','','','','A numeric custom action type that extends code type or option flags of the Type column.')
        )
        Binary = @(
            @('Name','N','','','','','','Identifier','','Unique key identifying the binary data.'),
            @('Data','N','','','','','','Binary','','The unformatted binary data.')
        )
        CreateFolder = @(
            @('Directory_','N','','','Directory','1','Identifier','','Primary key, could be foreign key into the Directory table.'),
            @('Component_','N','','','Component','1','Identifier','','Foreign key into the Component table.')
        )
        RemoveFile = @(
            @('FileKey','N','','','','','','Identifier','','Primary key used to identify a particular file entry'),
            @('Component_','N','','','Component','1','Identifier','','Foreign key referencing Component that controls the file to be removed.'),
            @('FileName','Y','','','','','','WildCardFilename','','Name of the file to be removed.'),
            @('DirProperty','N','','','','','','Identifier','','Name of a property whose value is assumed to resolve to the full pathname to the folder of the file to be removed.'),
            @('InstallMode','N','','','','','','','1;2;3','Installation option, one of iimEnum.')
        )
        Registry = @(
            @('Registry','N','','','','','','Identifier','','Primary key, non-localized token.'),
            @('Root','N','-1','3','','','','','','The predefined root key for the registry value, one of rrkEnum.'),
            @('Key','N','','','','','','RegPath','','The key for the registry value.'),
            @('Name','Y','','','','','','Formatted','','The registry value name.'),
            @('Value','Y','','','','','','Formatted','','The registry value.'),
            @('Component_','N','','','Component','1','Identifier','','Foreign key into the Component table referencing component that controls the installing of the registry value.')
        )
        AdminUISequence = @(
            @('Action','N','','','','','','Identifier','','Name of action to invoke, either in the engine or the handler DLL.'),
            @('Condition','Y','','','','','','Condition','','Optional expression which skips the action if evaluates to expFalse.If the expression syntax is invalid, the engine will terminate, returning iesBadActionData.'),
            @('Sequence','Y','-4','32767','','','','','','Number that determines the sort order in which the actions are to be executed.  Leave blank to suppress action.')
        )
        AdminExecuteSequence = @(
            @('Action','N','','','','','','Identifier','','Name of action to invoke, either in the engine or the handler DLL.'),
            @('Condition','Y','','','','','','Condition','','Optional expression which skips the action if evaluates to expFalse.If the expression syntax is invalid, the engine will terminate, returning iesBadActionData.'),
            @('Sequence','Y','-4','32767','','','','','','Number that determines the sort order in which the actions are to be executed.  Leave blank to suppress action.')
        )
        AdvtExecuteSequence = @(
            @('Action','N','','','','','','Identifier','','Name of action to invoke, either in the engine or the handler DLL.'),
            @('Condition','Y','','','','','','Condition','','Optional expression which skips the action if evaluates to expFalse.If the expression syntax is invalid, the engine will terminate, returning iesBadActionData.'),
            @('Sequence','Y','-4','32767','','','','','','Number that determines the sort order in which the actions are to be executed.  Leave blank to suppress action.')
        )
        InstallUISequence = @(
            @('Action','N','','','','','','Identifier','','Name of action to invoke, either in the engine or the handler DLL.'),
            @('Condition','Y','','','','','','Condition','','Optional expression which skips the action if evaluates to expFalse. If the expression syntax is invalid, the engine will terminate, returning iesBadActionData.'),
            @('Sequence','Y','-4','32767','','','','','','Number that determines the sort order in which the actions are to be executed. Leave blank to suppress action.')
        )
        InstallExecuteSequence = @(
            @('Action','N','','','','','','Identifier','','Name of action to invoke, either in the engine or the handler DLL.'),
            @('Condition','Y','','','','','','Condition','','Optional expression which skips the action if evaluates to expFalse. If the expression syntax is invalid, the engine will terminate, returning iesBadActionData.'),
            @('Sequence','Y','-4','32767','','','','','','Number that determines the sort order in which the actions are to be executed. Leave blank to suppress action.')
        )
        Media = @(
            @('DiskId','N','1','32767','','','','','','Primary key, integer to determine sort order for table.'),
            @('LastSequence','N','0','2147483647','','','','','','File sequence number for the last file for this media.'),
            @('DiskPrompt','Y','','','','','','Text','','Disk name: the visible text actually printed on the disk.  This will be used to prompt the user when this disk needs to be inserted.'),
            @('Cabinet','Y','','','','','','Cabinet','','If some or all of the files stored on the media are compressed in a cabinet, the name of that cabinet.'),
            @('VolumeLabel','Y','','','','','','Text','','The label attributed to the volume.'),
            @('Source','Y','','','','','','Property','','The property defining the location of the cabinet file.')
        )
        MsiFileHash = @(
            @('File_','N','','','File','1','Identifier','','Primary key, foreign key into File table referencing file with this hash'),
            @('Options','N','0','32767','','','','','','Various options and attributes for this hash.'),
            @('HashPart1','N','','','','','','','Size of file in bytes (long integer).'),
            @('HashPart2','N','','','','','','','Size of file in bytes (long integer).'),
            @('HashPart3','N','','','','','','','Size of file in bytes (long integer).'),
            @('HashPart4','N','','','','','','','Size of file in bytes (long integer).')
        )
        MsiLockPermissionsEx = @(
            @('Table','N','','','','','','Identifier','CreateFolder;File;Registry;ServiceInstall','Reference to another table name'),
            @('Condition','Y','','','','','','Formatted','','Expression which must evaluate to TRUE in order for this set of permissions to be applied'),
            @('MsiLockPermissionsEx','N','','','','','','Identifier','','Primary key, non-localized token'),
            @('LockObject','N','','','','','','Identifier','','Foreign key into Registry, File, CreateFolder, or ServiceInstall table'),
            @('SDDLText','N','','','','','','FormattedSDDLText','','String to indicate permissions to be applied to the LockObject')
        )
        _Validation = @(
            @('Table','N','','','','','','Identifier','','Name of table'),
            @('Column','N','','','','','','Identifier','','Name of column'),
            @('Nullable','N','','','','','','','Y;N','Whether the column is nullable'),
            @('MinValue','Y','-2147483647','2147483647','','','','','','Minimum value allowed'),
            @('MaxValue','Y','-2147483647','2147483647','','','','','','Maximum value allowed'),
            @('KeyTable','Y','','','','','','Identifier','','For foreign key, Name of table to which data must link'),
            @('KeyColumn','Y','1','32','','','','','','Column to which foreign key connects'),
            @('Category','Y','','','','','','','Text;Formatted;Template;Condition;Guid;Path;Version;Language;Identifier;Binary;UpperCase;LowerCase;Filename;Paths;AnyPath;WildCardFilename;RegPath;CustomSource;Property;Cabinet;Shortcut;FormattedSDDLText;Integer;DoubleInteger;TimeDate;DefaultDir','String category'),
            @('Set','Y','','','','','','Text','','Set of values that are permitted'),
            @('Description','Y','','','','','','Text','','Description of column')
        )
    }
    foreach ($table in $metadata.Keys) {
        foreach ($tuple in $metadata[$table]) {
            # Rows without a key/category use one additional empty slot in
            # the readable literals above; normalize only that unambiguous
            # shape, and reject every other width/shift rather than guessing.
            if ($tuple.Count -eq 10 -and [string]$tuple[6] -ceq '') {
                $tuple = @($tuple[0..5] + $tuple[7..9])
            }
            if ($tuple.Count -ne 9) {
                throw "MSI validation metadata contract for '$table' has an invalid tuple width."
            }
            Add-MsiValidationMetadata -Table $table -Column $tuple[0] -Nullable $tuple[1] -MinValue $tuple[2] -MaxValue $tuple[3] -KeyTable $tuple[4] -KeyColumn $tuple[5] -Category $tuple[6] -Set $tuple[7] -Description $tuple[8]
        }
    }
    return $rows
}

function Assert-MsiValidationSchemaContract {
    param(
        [Parameter(Mandatory)]
        [object]$Database
    )

    $rows = @(Get-MsiRows -Database $Database -Query 'SELECT `Table`,`Column`,`Nullable`,`MinValue`,`MaxValue`,`KeyTable`,`KeyColumn`,`Category`,`Set`,`Description` FROM `_Validation`' -FieldNames @('Table', 'Column', 'Nullable', 'MinValue', 'MaxValue', 'KeyTable', 'KeyColumn', 'Category', 'Set', 'Description'))
    $expected = Get-MsiValidationMetadataContract
    $actual = [ordered]@{}
    foreach ($row in $rows) {
        $key = "$([string]$row.Table)`0$([string]$row.Column)"
        if ($actual.Contains($key)) {
            throw "Normalized MSI _Validation contains duplicate '$([string]$row.Table).$([string]$row.Column)'."
        }
        $actual[$key] = $row
    }
    if ($actual.Count -ne $expected.Count) {
        throw "Normalized MSI _Validation has $($actual.Count) rows; expected the exact $($expected.Count)-column metadata contract."
    }
    foreach ($key in $expected.Keys) {
        if (-not $actual.Contains($key)) {
            throw "Normalized MSI _Validation is missing '$($expected[$key].Table).$($expected[$key].Column)'."
        }
        $expectedRow = $expected[$key]
        $actualRow = $actual[$key]
        foreach ($field in @('Table','Column','Nullable','MinValue','MaxValue','KeyTable','KeyColumn','Category','Set','Description')) {
            if ([string]$actualRow.$field -cne [string]$expectedRow.$field) {
                throw "Normalized MSI _Validation '$($expectedRow.Table).$($expectedRow.Column)' field '$field' expected '$($expectedRow.$field)' but found '$($actualRow.$field)'."
            }
        }
    }
}

function Assert-MsiRequiredTables {
    param(
        [Parameter(Mandatory)]
        [string[]]$TableNames
    )

    $required = @((Get-MsiAllowedTableColumns).Keys)
    if (@($TableNames | Select-Object -Unique).Count -ne $TableNames.Count) {
        throw 'Normalized MSI table inventory contains duplicate table names.'
    }
    $unexpected = @($TableNames | Where-Object { $required -notcontains $_ })
    if ($unexpected.Count -gt 0) {
        throw "Normalized MSI contains unexpected tables '$($unexpected -join ', ')'; the fixed MSI table allowlist is exhaustive."
    }
    if ($TableNames.Count -ne $required.Count) {
        throw "Normalized MSI exposes $($TableNames.Count) tables; the fixed allowlist requires exactly $($required.Count)."
    }
    foreach ($tableName in $required) {
        if ($TableNames -notcontains $tableName) {
            throw "Normalized MSI is missing required fixed table '$tableName'."
        }
    }
    if ($TableNames -contains 'MsiServiceConfig') {
        throw 'Normalized MSI contains deprecated MsiServiceConfig; ServiceSid must remain an external Protected WP02 owner operation.'
    }
}

function Assert-MsiTableSchema {
    param(
        [Parameter(Mandatory)]
        [object]$Database
    )

    $expectedColumns = Get-MsiAllowedTableColumns
    $columnRows = @(Get-MsiRows -Database $Database -Query 'SELECT `Table`,`Number`,`Name` FROM `_Columns`' -FieldNames @('Table', 'Number', 'Name'))
    foreach ($tableName in $expectedColumns.Keys) {
        $actualRows = @($columnRows | Where-Object { [string]$_.Table -ceq $tableName } | Sort-Object { [int]$_.Number })
        $actual = @($actualRows | ForEach-Object { [string]$_.Name })
        $expected = @($expectedColumns[$tableName])
        $actualNumbers = @($actualRows | ForEach-Object { [int]$_.Number })
        $expectedNumbers = @(1..$expected.Count)
        if ($actual.Count -ne $expected.Count -or @($actual | Select-Object -Unique).Count -ne $actual.Count -or
            @($actualNumbers | Select-Object -Unique).Count -ne $actualNumbers.Count -or
            (($actualNumbers -join ',') -cne ($expectedNumbers -join ','))) {
            throw "Normalized MSI table '$tableName' exposes columns '$($actual -join ', ')'; expected the exhaustive fixed schema '$($expected -join ', ')'."
        }
        foreach ($columnName in $expected) {
            if ($actual -notcontains $columnName) {
                throw "Normalized MSI table '$tableName' is missing fixed column '$columnName'."
            }
        }
    }
    $knownTables = @($expectedColumns.Keys)
    $unexpectedRows = @($columnRows | Where-Object { $knownTables -notcontains [string]$_.Table })
    if ($unexpectedRows.Count -gt 0) {
        throw "Normalized MSI exposes schema rows for unexpected table '$([string]$unexpectedRows[0].Table)'."
    }
    Assert-MsiValidationSchemaContract -Database $Database
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
        [string]$ExpectedPackageCode,

        [Parameter(Mandatory)]
        [string]$ExpectedBrokerHash,

        [Parameter(Mandatory)]
        [string]$ExpectedProvisionerHash,

        [Parameter(Mandatory)]
        [string]$ExpectedRegistryId,

        [Parameter(Mandatory)]
        [string]$BrokerBinaryPath,

        [Parameter(Mandatory)]
        [string]$ProvisionerBinaryPath,

        [Parameter(Mandatory)]
        [string]$PackageRoot
    )

    Assert-SafePackageLeafPath -Path $CandidateMsiPath -Root $PackageRoot -Description 'MSI candidate validation input' | Out-Null
    Assert-PhysicalPackagePathUnderRoot -Path $CandidateIntermediatePath -Root $PackageRoot -Description 'MSI candidate validation root' | Out-Null
    Assert-SafePackageLeafPath -Path $BrokerBinaryPath -Root (Split-Path -Parent $PackageRoot) -Description 'Broker validation input' | Out-Null
    Assert-SafePackageLeafPath -Path $ProvisionerBinaryPath -Root (Split-Path -Parent $PackageRoot) -Description 'Provisioner validation input' | Out-Null

    $tableRows = @(Get-MsiRows -Database $Database -Query 'SELECT `Name` FROM `_Tables`' -FieldNames @('Name'))
    $tableNames = @($tableRows | ForEach-Object { $_.Name })
    Assert-MsiRequiredTables -TableNames $tableNames
    Assert-MsiTableSchema -Database $Database
    Assert-MsiSummaryContract -Database $Database -ExpectedPackageCode $ExpectedPackageCode
    Assert-MsiExecutableLifecycleContract -Database $Database -ExpectedVersion $ExpectedVersion -ExpectedRegistryId $ExpectedRegistryId -ExpectedBrokerHash $ExpectedBrokerHash
    Assert-MsiMediaContract -Database $Database

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

    $componentRows = @(Get-MsiRows -Database $Database -Query 'SELECT `Component`,`ComponentId`,`Directory_`,`Attributes`,`Condition`,`KeyPath` FROM `Component`' -FieldNames @('Component', 'ComponentId', 'Directory', 'Attributes', 'Condition', 'KeyPath'))
    $expectedComponents = [ordered]@{
        ProtectedBrokerService = @('{65D9B85B-1DBA-4B42-86F1-DB8F20BD4F51}', 'INSTALLFOLDER', '256', '', 'ProtectedBrokerFile')
        ProtectedCustodyDataDirectory = @('{A2D4CEFE-44C4-4E33-B96C-6C3AE5BAA6A9}', 'CUSTODYDATAFOLDER', '272', '', '')
        ProtectedInstallDirectory = @('{2B86B79F-72B4-4C7A-9B5E-14A3FC1EF1B6}', 'INSTALLFOLDER', '272', '', '')
        ProtectedProvisioner = @('{87B06C30-3A3A-44B3-B6B8-F1F6F0A4DFA2}', 'INSTALLFOLDER', '256', '', 'ProtectedProvisionerFile')
        ProtectedRegistryIdentity = @('{6D51662C-3D41-4694-8FEF-2548B0DE9DCE}', 'INSTALLFOLDER', '276', '', 'regxxeh8sZMWNeOQS6NQLf1_.sApTM')
        ProtectedRegistryRoot = @('{C7393427-4FD2-46A8-8A19-CF6ABF4E48A2}', 'INSTALLFOLDER', '276', '', 'regozpvmfX_NrEkN4q_wqmS1tt2.4I')
    }
    if ($componentRows.Count -ne $expectedComponents.Count) {
        throw "Normalized MSI Component table has $($componentRows.Count) rows; expected exactly $($expectedComponents.Count)."
    }
    $componentIds = @($componentRows | ForEach-Object { [string]$_.Component })
    if (@($componentIds | Select-Object -Unique).Count -ne $componentIds.Count -or
        (($componentIds | Sort-Object) -join "`0") -cne (($expectedComponents.Keys | Sort-Object) -join "`0")) {
        throw 'Normalized MSI Component identifiers are not an exact unique set; duplicate rows cannot replace a missing component.'
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
        Assert-MsiRowValue -Row $row -Field Condition -Expected $expected[3] -Description "Component '$componentName' condition"
        Assert-MsiRowValue -Row $row -Field KeyPath -Expected $expected[4] -Description "Component '$componentName' key path"
    }

    $brokerSize = [string](Get-Item -LiteralPath $BrokerBinaryPath).Length
    $provisionerSize = [string](Get-Item -LiteralPath $ProvisionerBinaryPath).Length
    $fileRows = @(Get-MsiRows -Database $Database -Query 'SELECT `File`,`Component_`,`FileName`,`FileSize`,`Version`,`Language`,`Attributes`,`Sequence` FROM `File`' -FieldNames @('File', 'Component', 'FileName', 'FileSize', 'Version', 'Language', 'Attributes', 'Sequence'))
    if ($fileRows.Count -ne 2) {
        throw "Normalized MSI File table has $($fileRows.Count) rows; expected exactly two fixed payload files."
    }
    $expectedFiles = @{
        ProtectedBrokerFile = @('ProtectedBrokerService', 'ocentr~1.exe|ocentra-protected-capability-custody-broker.exe', $brokerSize, '', '', '512', '1')
        ProtectedProvisionerFile = @('ProtectedProvisioner', 'ocentr~2.exe|ocentra-protected-capability-custody-provisioner.exe', $provisionerSize, '', '', '512', '2')
    }
    $fileIds = @($fileRows | ForEach-Object { [string]$_.File })
    if (@($fileIds | Select-Object -Unique).Count -ne $fileIds.Count -or
        (($fileIds | Sort-Object) -join "`0") -cne (($expectedFiles.Keys | Sort-Object) -join "`0")) {
        throw 'Normalized MSI File identifiers are not an exact unique set; duplicate rows cannot replace a missing payload.'
    }
    foreach ($row in $fileRows) {
        $fileId = [string]$row.File
        if (-not $expectedFiles.ContainsKey($fileId)) {
            throw "Normalized MSI contains unexpected payload file '$fileId'."
        }
        $expected = $expectedFiles[$fileId]
        Assert-MsiRowValue -Row $row -Field Component -Expected $expected[0] -Description "File '$fileId' component"
        Assert-MsiRowValue -Row $row -Field FileName -Expected $expected[1] -Description "File '$fileId' short/long name"
        Assert-MsiRowValue -Row $row -Field FileSize -Expected $expected[2] -Description "File '$fileId' size"
        Assert-MsiRowValue -Row $row -Field Version -Expected $expected[3] -Description "File '$fileId' version"
        Assert-MsiRowValue -Row $row -Field Language -Expected $expected[4] -Description "File '$fileId' language"
        Assert-MsiRowValue -Row $row -Field Attributes -Expected $expected[5] -Description "File '$fileId' attributes"
        Assert-MsiRowValue -Row $row -Field Sequence -Expected $expected[6] -Description "File '$fileId' sequence"
    }

    $serviceRows = @(Get-MsiRows -Database $Database -Query 'SELECT `ServiceInstall`,`Name`,`DisplayName`,`ServiceType`,`StartType`,`ErrorControl`,`LoadOrderGroup`,`Dependencies`,`StartName`,`Password`,`Arguments`,`Component_` FROM `ServiceInstall`' -FieldNames @('ServiceInstall', 'Name', 'DisplayName', 'ServiceType', 'StartType', 'ErrorControl', 'LoadOrderGroup', 'Dependencies', 'StartName', 'Password', 'Arguments', 'Component'))
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
        Password = ''
        Arguments = ''
        Component = 'ProtectedBrokerService'
    }
    foreach ($field in $serviceValues.Keys) {
        Assert-MsiRowValue -Row $service -Field $field -Expected $serviceValues[$field] -Description "Broker ServiceInstall '$field'"
    }

    $customActionRows = @(Get-MsiRows -Database $Database -Query 'SELECT `Action`,`Type`,`Source`,`Target`,`ExtendedType` FROM `CustomAction`' -FieldNames @('Action', 'Type', 'Source', 'Target', 'ExtendedType'))
    $provisionerActions = @($customActionRows | Where-Object { $_.Action -eq 'RunProtectedProvisioner' })
    if ($provisionerActions.Count -ne 1) {
        throw "Normalized MSI must contain exactly one RunProtectedProvisioner custom action."
    }
    $provisionerAction = $provisionerActions[0]
    Assert-MsiRowValue -Row $provisionerAction -Field Type -Expected '11282' -Description 'RunProtectedProvisioner custom action type'
    Assert-MsiRowValue -Row $provisionerAction -Field Source -Expected 'ProtectedProvisionerFile' -Description 'RunProtectedProvisioner custom action source'
    Assert-MsiRowValue -Row $provisionerAction -Field Target -Expected '' -Description 'RunProtectedProvisioner custom action target'
    Assert-MsiRowValue -Row $provisionerAction -Field ExtendedType -Expected '' -Description 'RunProtectedProvisioner custom action extended type'

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
    $expectedRegistryRows = @(
        "2|Software\Ocentra\ProtectedCapabilityCustody|+|",
        "2|$identityKey|+|",
        "2|Software\Ocentra\ProtectedCapabilityCustody|package-boundary|parent-protected-custody-v1",
        "2|$identityKey|package-boundary|parent-protected-custody-v1"
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

    $wixServiceRows = @(Get-MsiRows -Database $Database -Query 'SELECT `ServiceName`,`Component_`,`NewService`,`FirstFailureActionType`,`SecondFailureActionType`,`ThirdFailureActionType`,`ResetPeriodInDays`,`RestartServiceDelayInSeconds`,`ProgramCommandLine`,`RebootMessage` FROM `Wix4ServiceConfig`' -FieldNames @('ServiceName', 'Component', 'NewService', 'FirstFailureActionType', 'SecondFailureActionType', 'ThirdFailureActionType', 'ResetPeriodInDays', 'RestartServiceDelayInSeconds', 'ProgramCommandLine', 'RebootMessage'))
    if ($wixServiceRows.Count -ne 1) {
        throw "Normalized MSI Wix4ServiceConfig table has $($wixServiceRows.Count) rows; expected one supported failure-action row."
    }
    $wixService = $wixServiceRows[0]
    $wixServiceValues = [ordered]@{
        ServiceName = 'OcentraProtectedCapabilityCustodyBroker'
        Component = 'ProtectedBrokerService'
        NewService = '1'
        FirstFailureActionType = 'restart'
        SecondFailureActionType = 'restart'
        ThirdFailureActionType = 'restart'
        ResetPeriodInDays = '1'
        RestartServiceDelayInSeconds = '10'
        ProgramCommandLine = ''
        RebootMessage = ''
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
    New-SafePackageDirectory -Path $decompileRoot -Root $PackageRoot -Description 'MSI payload validation root' | Out-Null
    Assert-SafePackageLeafPath -Path $decompiledSourcePath -Root $PackageRoot -Description 'MSI decompile source output' | Out-Null
    Assert-NoPackageReparseChain -Path $decompileRoot -Description 'MSI payload validation root immediately before decompile'
    Assert-SafePackageLeafPath -Path $CandidateMsiPath -Root $PackageRoot -Description 'MSI candidate immediately before decompile' | Out-Null
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
    Assert-SafePackageLeafPath -Path $extractedBrokerPath -Root $PackageRoot -Description 'Extracted broker payload' | Out-Null
    Assert-SafePackageLeafPath -Path $extractedProvisionerPath -Root $PackageRoot -Description 'Extracted provisioner payload' | Out-Null
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
        [string]$ExpectedPackageCode,

        [Parameter(Mandatory)]
        [string]$ExpectedBrokerHash,

        [Parameter(Mandatory)]
        [string]$ExpectedProvisionerHash,

        [Parameter(Mandatory)]
        [string]$ExpectedRegistryId,

        [Parameter(Mandatory)]
        [string]$BrokerBinaryPath,

        [Parameter(Mandatory)]
        [string]$ProvisionerBinaryPath,

        [Parameter(Mandatory)]
        [string]$PackageRoot
    )

    Assert-SafePackageLeafPath -Path $CandidateMsiPath -Root $PackageRoot -Description 'MSI candidate input' | Out-Null
    Assert-PhysicalPackagePathUnderRoot -Path $CandidateIntermediatePath -Root $PackageRoot -Description 'MSI candidate intermediate root' | Out-Null
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
            ExpectedPackageCode = $ExpectedPackageCode
            PackageRoot = $PackageRoot
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
