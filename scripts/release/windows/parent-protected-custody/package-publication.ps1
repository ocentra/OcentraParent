$script:PackagePublicationJournalSchema = 4
$script:PackagePublicationJournalPhases = @('prepared', 'previous-moved', 'staging-moved', 'final-validated', 'committed')
$script:PackagePublicationJournalFields = @(
    'schema',
    'phase',
    'sequence',
    'operationId',
    'outputRoot',
    'stagingRoot',
    'backupRoot',
    'artifactNames',
    'hadPrevious',
    'stagedHashes',
    'previousHashes',
    'stagedManifestSemanticSha256',
    'previousManifestSemanticSha256',
    'inputContract',
    'previousRecordHash',
    'recordHash'
)

function Get-PackagePublicationMapEntries {
    param(
        [Parameter(Mandatory)]
        [object]$Value,

        [Parameter(Mandatory)]
        [string]$Description
    )

    if ($null -eq $Value) {
        throw "$Description is null."
    }
    $entries = [ordered]@{}
    if ($Value -is [System.Collections.IDictionary]) {
        foreach ($key in $Value.Keys) {
            $name = [string]$key
            if ([string]::IsNullOrWhiteSpace($name) -or $entries.Contains($name)) {
                throw "$Description contains a missing or duplicate key."
            }
            $entries[$name] = $Value[$key]
        }
    } else {
        foreach ($property in @($Value.PSObject.Properties)) {
            $name = [string]$property.Name
            if ([string]::IsNullOrWhiteSpace($name) -or $entries.Contains($name)) {
                throw "$Description contains a missing or duplicate key."
            }
            $entries[$name] = $property.Value
        }
    }
    $canonical = [ordered]@{}
    foreach ($name in @($entries.Keys | Sort-Object)) {
        $canonical[$name] = $entries[$name]
    }
    return $canonical
}

function ConvertTo-PackagePublicationCanonicalJson {
    param(
        [Parameter(Mandatory)]
        [object]$Value
    )

    return $Value | ConvertTo-Json -Depth 32 -Compress
}

function ConvertTo-PackageManifestCanonicalValue {
    param(
        [Parameter()]
        [AllowNull()]
        [object]$Value
    )

    if ($null -eq $Value) {
        return $null
    }
    if ($Value -is [string] -or $Value -is [bool] -or $Value -is [byte] -or
        $Value -is [int16] -or $Value -is [int32] -or $Value -is [int64] -or
        $Value -is [uint16] -or $Value -is [uint32] -or $Value -is [uint64] -or
        $Value -is [decimal] -or $Value -is [double] -or $Value -is [single]) {
        return $Value
    }
    if ($Value -is [System.Collections.IDictionary]) {
        $canonical = [ordered]@{}
        foreach ($key in @($Value.Keys | ForEach-Object { [string]$_ } | Sort-Object)) {
            $canonical[$key] = ConvertTo-PackageManifestCanonicalValue -Value $Value[$key]
        }
        return ,$canonical
    }
    if ($Value -is [System.Collections.IEnumerable]) {
        $canonicalItems = [System.Collections.Generic.List[object]]::new()
        foreach ($item in $Value) {
            $canonicalItems.Add((ConvertTo-PackageManifestCanonicalValue -Value $item))
        }
        return ,$canonicalItems.ToArray()
    }
    $properties = @($Value.PSObject.Properties)
    if ($properties.Count -gt 0) {
        $canonical = [ordered]@{}
        foreach ($propertyName in @($properties.Name | ForEach-Object { [string]$_ } | Sort-Object)) {
            $canonical[$propertyName] = ConvertTo-PackageManifestCanonicalValue -Value $Value.$propertyName
        }
        return ,$canonical
    }
    if ($Value.PSObject.TypeNames -contains 'System.Management.Automation.PSCustomObject') {
        # ConvertFrom-Json represents an empty JSON object as an empty
        # PSCustomObject. Preserve that object shape instead of falling
        # through to an empty string, so semantic hashes are stable across
        # in-memory and serialized manifests.
        return ,([ordered]@{})
    }
    return [string]$Value
}

function Get-PackageManifestSemanticHash {
    param(
        [Parameter(Mandatory)]
        [object]$Manifest
    )

    $canonical = ConvertTo-PackageManifestCanonicalValue -Value $Manifest
    if ($canonical -isnot [System.Collections.IDictionary]) {
        throw 'Package manifest root is not a JSON object; refusing semantic binding.'
    }
    $requiredRootFields = @(
        'schema',
        'packageId',
        'productVersion',
        'architecture',
        'artifact',
        'reproducibility',
        'publicationIntegrity',
        'inputs',
        'inputIntegrity',
        'digestBinding',
        'fixedIdentity',
        'lifecycle',
        'protectedBoundary',
        'source',
        'manifestIntegrity'
    )
    $actualRootFields = @($canonical.Keys | ForEach-Object { [string]$_ })
    if ($actualRootFields.Count -ne $requiredRootFields.Count -or
        @($actualRootFields | Where-Object { $requiredRootFields -notcontains $_ }).Count -gt 0 -or
        @($requiredRootFields | Where-Object { $actualRootFields -notcontains $_ }).Count -gt 0) {
        throw 'Package manifest root fields are not the exact protected-custody contract; refusing semantic binding.'
    }
    $integrity = $canonical['manifestIntegrity']
    if ($integrity -isnot [System.Collections.IDictionary]) {
        throw 'Package manifest manifestIntegrity is not an object; refusing semantic binding.'
    }
    $integrityFields = @($integrity.Keys | ForEach-Object { [string]$_ })
    if ($integrityFields.Count -ne 2 -or $integrityFields -notcontains 'schema' -or $integrityFields -notcontains 'semanticSha256') {
        throw 'Package manifest manifestIntegrity fields are not exact; refusing semantic binding.'
    }
    if ([string]$integrity['schema'] -cne 'canonical-json-v1') {
        throw "Package manifest semantic schema '$($integrity['schema'])' is unsupported; refusing semantic binding."
    }
    $declaredHash = [string]$integrity['semanticSha256']
    if (-not [string]::IsNullOrEmpty($declaredHash) -and
        ($declaredHash -notmatch '^[0-9a-f]{64}$' -or $declaredHash -cne $declaredHash.ToLowerInvariant())) {
        throw 'Package manifest semanticSha256 is not a lowercase SHA-256 or the permitted empty preimage value.'
    }
    $integrity['semanticSha256'] = ''
    $json = $canonical | ConvertTo-Json -Depth 64 -Compress
    $bytes = [System.Text.UTF8Encoding]::new($false).GetBytes($json)
    $computedHash = ([System.Security.Cryptography.SHA256]::HashData($bytes) | ForEach-Object { $_.ToString('x2') }) -join ''
    if (-not [string]::IsNullOrEmpty($declaredHash) -and $declaredHash -cne $computedHash) {
        throw 'Package manifest declared semanticSha256 does not match its canonical full-manifest preimage.'
    }
    return $computedHash
}

function Get-PackageManifestSemanticHashFromFile {
    param(
        [Parameter(Mandatory)]
        [string]$Path,

        [Parameter(Mandatory)]
        [string]$PackageRoot
    )

    $safePath = Assert-SafePackageLeafPath -Path $Path -Root $PackageRoot -Description 'Package manifest semantic input'
    $text = [System.IO.File]::ReadAllText($safePath, [System.Text.UTF8Encoding]::new($false))
    if ([string]::IsNullOrWhiteSpace($text)) {
        throw "Package manifest '$safePath' is empty; refusing semantic binding."
    }
    try {
        $manifest = $text | ConvertFrom-Json
        return Get-PackageManifestSemanticHash -Manifest $manifest
    } catch {
        throw "Package manifest '$safePath' is not a canonical semantic object; refusing semantic binding: $($_.Exception.Message)"
    }
}

function Get-PackagePublicationToolchainProvenance {
    param(
        [Parameter(Mandatory)]
        [object]$Provenance
    )

    if ($null -eq $Provenance) {
        throw 'Package publication toolchain provenance is null.'
    }
    $actualFields = if ($Provenance -is [System.Collections.IDictionary]) {
        @($Provenance.Keys | ForEach-Object { [string]$_ })
    } else {
        @($Provenance.PSObject.Properties.Name)
    }
    $expectedFields = @('wixTool', 'wixUtilExtension')
    if ($actualFields.Count -ne $expectedFields.Count -or
        @($actualFields | Where-Object { $expectedFields -notcontains $_ }).Count -gt 0 -or
        @($expectedFields | Where-Object { $actualFields -notcontains $_ }).Count -gt 0) {
        throw 'Package publication toolchain provenance fields are not exact.'
    }

    function Normalize-PackagePublicationPayload {
        param(
            [Parameter(Mandatory)]
            [object]$Payload,

            [Parameter(Mandatory)]
            [string[]]$Fields,

            [Parameter(Mandatory)]
            [string]$Description
        )

        if ($null -eq $Payload) {
            throw "$Description is null."
        }
        $payloadFields = if ($Payload -is [System.Collections.IDictionary]) {
            @($Payload.Keys | ForEach-Object { [string]$_ })
        } else {
            @($Payload.PSObject.Properties.Name)
        }
        if ($payloadFields.Count -ne $Fields.Count -or
            @($payloadFields | Where-Object { $Fields -notcontains $_ }).Count -gt 0 -or
            @($Fields | Where-Object { $payloadFields -notcontains $_ }).Count -gt 0) {
            throw "$Description fields are not exact."
        }
        $pathField = if ($Description -ceq 'WiX tool payload provenance') { 'packageRoot' } else { 'payloadRoot' }
        $idField = if ($Description -ceq 'WiX tool payload provenance') { 'packageId' } else { 'extensionId' }
        $expectedId = if ($Description -ceq 'WiX tool payload provenance') { 'wix' } else { 'WixToolset.Util.wixext' }
        if ([string]$Payload.$idField -cne $expectedId -or [string]$Payload.version -cne '6.0.2') {
            throw "$Description identity/version is not the pinned WiX 6.0.2 payload."
        }
        $root = [string]$Payload.$pathField
        if (-not [System.IO.Path]::IsPathRooted($root) -or [string]::IsNullOrWhiteSpace($root)) {
            throw "$Description root is not an absolute path."
        }
        $fileHashes = Get-PackagePublicationMapEntries -Value $Payload.fileHashes -Description "$Description file hash contract"
        if ($fileHashes.Count -eq 0) {
            throw "$Description file hash contract is empty."
        }
        foreach ($relativeName in $fileHashes.Keys) {
            $normalizedRelativeName = $relativeName.Replace('/', '\')
            $relativeSegments = @($normalizedRelativeName.Split('\'))
            if ([System.IO.Path]::IsPathRooted($normalizedRelativeName) -or
                @($relativeSegments | Where-Object { $_ -ceq '..' }).Count -gt 0 -or
                $normalizedRelativeName.Contains(':')) {
                throw "$Description contains an unsafe payload-relative path '$relativeName'."
            }
            $hash = [string]$fileHashes[$relativeName]
            if ($hash -notmatch '^[0-9a-f]{64}$' -or $hash -cne $hash.ToLowerInvariant()) {
                throw "$Description hash for '$relativeName' is not a lowercase SHA-256."
            }
        }
        $contentSha256 = [string]$Payload.contentSha256
        if ($contentSha256 -notmatch '^[0-9a-f]{64}$' -or $contentSha256 -cne $contentSha256.ToLowerInvariant()) {
            throw "$Description contentSha256 is not a lowercase SHA-256."
        }
        $fileHashJson = ConvertTo-PackagePublicationCanonicalJson -Value $fileHashes
        $fileHashBytes = [System.Text.UTF8Encoding]::new($false).GetBytes($fileHashJson)
        $expectedContentSha256 = ([System.Security.Cryptography.SHA256]::HashData($fileHashBytes) | ForEach-Object { $_.ToString('x2') }) -join ''
        if ($contentSha256 -cne $expectedContentSha256) {
            throw "$Description contentSha256 does not bind its exact payload file hash map."
        }
        return [ordered]@{
            $idField = $expectedId
            version = '6.0.2'
            $pathField = $root
            fileHashes = $fileHashes
            contentSha256 = $contentSha256
        }
    }

    return [ordered]@{
        wixTool = Normalize-PackagePublicationPayload -Payload $Provenance.wixTool -Fields @('packageId', 'version', 'packageRoot', 'fileHashes', 'contentSha256') -Description 'WiX tool payload provenance'
        wixUtilExtension = Normalize-PackagePublicationPayload -Payload $Provenance.wixUtilExtension -Fields @('extensionId', 'version', 'payloadRoot', 'fileHashes', 'contentSha256') -Description 'WiX Util extension payload provenance'
    }
}

function Get-PackagePublicationInputContract {
    param(
        [Parameter(Mandatory)]
        [object]$InputContract
    )

    $allowed = @('brokerBinarySha256', 'provisionerBinarySha256', 'sourceHashes', 'anchoredInputHashes', 'commandFingerprints', 'toolchainProvenance')
    $actual = if ($InputContract -is [System.Collections.IDictionary]) {
        @($InputContract.Keys | ForEach-Object { [string]$_ })
    } else {
        @($InputContract.PSObject.Properties.Name)
    }
    if ($actual.Count -ne $allowed.Count -or @($actual | Where-Object { $allowed -notcontains $_ }).Count -gt 0 -or
        @($allowed | Where-Object { $actual -notcontains $_ }).Count -gt 0) {
        throw 'Package publication input contract has an unexpected or missing field.'
    }

    $brokerHash = [string]$InputContract.brokerBinarySha256
    $provisionerHash = [string]$InputContract.provisionerBinarySha256
    foreach ($entry in @(@('brokerBinarySha256', $brokerHash), @('provisionerBinarySha256', $provisionerHash))) {
        if ($entry[1] -notmatch '^[0-9a-f]{64}$' -or $entry[1] -cne $entry[1].ToLowerInvariant()) {
            throw "Package publication input contract '$($entry[0])' is not a lowercase SHA-256."
        }
    }

    $sourceHashes = Get-PackagePublicationMapEntries -Value $InputContract.sourceHashes -Description 'Package publication source hash contract'
    if ($sourceHashes.Count -eq 0) {
        throw 'Package publication source hash contract is empty.'
    }
    foreach ($name in $sourceHashes.Keys) {
        $hash = [string]$sourceHashes[$name]
        if ($hash -notmatch '^[0-9a-f]{64}$' -or $hash -cne $hash.ToLowerInvariant()) {
            throw "Package publication source hash for '$name' is not a lowercase SHA-256."
        }
    }

    $anchoredInputHashes = Get-PackagePublicationMapEntries -Value $InputContract.anchoredInputHashes -Description 'Package publication anchored input hash contract'
    if ($anchoredInputHashes.Count -eq 0) {
        throw 'Package publication anchored input hash contract is empty.'
    }
    foreach ($name in $anchoredInputHashes.Keys) {
        $hash = [string]$anchoredInputHashes[$name]
        if ($hash -notmatch '^[0-9a-f]{64}$' -or $hash -cne $hash.ToLowerInvariant()) {
            throw "Package publication anchored input hash for '$name' is not a lowercase SHA-256."
        }
    }

    $commandFingerprints = Get-PackagePublicationMapEntries -Value $InputContract.commandFingerprints -Description 'Package publication command fingerprint contract'
    if ($commandFingerprints.Count -eq 0) {
        throw 'Package publication command fingerprint contract is empty.'
    }
    foreach ($name in $commandFingerprints.Keys) {
        $fingerprint = [string]$commandFingerprints[$name]
        $parts = $fingerprint -split '\|', 4
        if ($parts.Count -ne 4 -or [string]::IsNullOrWhiteSpace($parts[0]) -or
            [string]::IsNullOrWhiteSpace($parts[2]) -or $parts[3] -notmatch '^[0-9a-f]{64}$' -or
            $parts[3] -cne $parts[3].ToLowerInvariant()) {
            throw "Package publication command fingerprint for '$name' is malformed."
        }
    }

    $toolchainProvenance = Get-PackagePublicationToolchainProvenance -Provenance $InputContract.toolchainProvenance

    return [ordered]@{
        brokerBinarySha256 = $brokerHash
        provisionerBinarySha256 = $provisionerHash
        sourceHashes = $sourceHashes
        anchoredInputHashes = $anchoredInputHashes
        commandFingerprints = $commandFingerprints
        toolchainProvenance = $toolchainProvenance
    }
}

function Get-PackagePublicationCanonicalRecord {
    param(
        [Parameter(Mandatory)]
        [object]$Record
    )

    return [ordered]@{
        schema = [int]$Record.schema
        phase = [string]$Record.phase
        sequence = [int]$Record.sequence
        operationId = [string]$Record.operationId
        outputRoot = [string]$Record.outputRoot
        stagingRoot = [string]$Record.stagingRoot
        backupRoot = [string]$Record.backupRoot
        artifactNames = @($Record.artifactNames | ForEach-Object { [string]$_ })
        hadPrevious = [bool]$Record.hadPrevious
        stagedHashes = Get-PackagePublicationMapEntries -Value $Record.stagedHashes -Description 'Package publication staged hash contract'
        previousHashes = Get-PackagePublicationMapEntries -Value $Record.previousHashes -Description 'Package publication previous hash contract'
        stagedManifestSemanticSha256 = [string]$Record.stagedManifestSemanticSha256
        previousManifestSemanticSha256 = [string]$Record.previousManifestSemanticSha256
        inputContract = Get-PackagePublicationInputContract -InputContract $Record.inputContract
        previousRecordHash = [string]$Record.previousRecordHash
    }
}

function Get-PackagePublicationRecordHash {
    param(
        [Parameter(Mandatory)]
        [object]$Record
    )

    $json = ConvertTo-PackagePublicationCanonicalJson -Value (Get-PackagePublicationCanonicalRecord -Record $Record)
    $bytes = [System.Text.UTF8Encoding]::new($false).GetBytes($json)
    return ([System.Security.Cryptography.SHA256]::HashData($bytes) | ForEach-Object { $_.ToString('x2') }) -join ''
}

function Assert-PackagePublicationPathImmediatelyBeforeAccess {
    param(
        [Parameter(Mandatory)]
        [string]$PackageRoot,

        [Parameter(Mandatory)]
        [string]$Path,

        [Parameter(Mandatory)]
        [string]$Description
    )

    # Keep the journal/lock path check adjacent to the filesystem operation.
    # The safe-path helper rejects reparse components and verifies the nearest
    # physical ancestor, while this second check makes that boundary explicit
    # at every journal read/write and lock probe/open site.
    $safePath = Assert-SafePackageLeafPath -Path $Path -Root $PackageRoot -Description "$Description path"
    $parentPath = [System.IO.Path]::GetDirectoryName($safePath)
    if ([string]::IsNullOrWhiteSpace($parentPath)) {
        throw "$Description path '$safePath' has no parent filesystem path."
    }
    Assert-PhysicalPackagePathUnderRoot -Path $parentPath -Root $PackageRoot -Description "$Description parent immediately before access" | Out-Null
    Assert-NoPackageReparseChain -Path $parentPath -Description "$Description parent immediately before access"
    Assert-NoPackageReparseChain -Path $safePath -Description "$Description immediately before access"
    if (Test-Path -LiteralPath $safePath) {
        $item = Get-Item -LiteralPath $safePath -Force
        if ($item.PSIsContainer -or ($item.Attributes -band [System.IO.FileAttributes]::ReparsePoint) -ne 0) {
            throw "$Description path '$safePath' is not a regular non-reparse file."
        }
    }
    return $safePath
}

function Assert-HeldPackagePublicationLock {
    param(
        [Parameter(Mandatory)]
        [string]$PackageRoot,

        [Parameter(Mandatory)]
        [string]$LockPath,

        [Parameter(Mandatory)]
        [System.IO.FileStream]$LockStream,

        [Parameter()]
        [string]$Description = 'Package publication lock'
    )

    if ($null -eq $LockStream) {
        throw "$Description is absent; refusing publication or recovery."
    }
    try {
        if ($LockStream.SafeFileHandle.IsClosed -or $LockStream.SafeFileHandle.IsInvalid -or
            -not $LockStream.CanRead -or -not $LockStream.CanWrite) {
            throw "$Description is not a live read/write file handle."
        }
        $safeLockPath = Assert-PackagePublicationPathImmediatelyBeforeAccess -PackageRoot $PackageRoot -Path $LockPath -Description $Description
        $streamName = Get-NormalizedPackagePath -Path ([string]$LockStream.Name)
        if (-not $streamName.Equals($safeLockPath, [System.StringComparison]::OrdinalIgnoreCase)) {
            throw "$Description handle '$streamName' is not the exact safe lock path '$safeLockPath'."
        }
        $item = Get-Item -LiteralPath $safeLockPath -Force
        if ($item.PSIsContainer -or ($item.Attributes -band [System.IO.FileAttributes]::ReparsePoint) -ne 0) {
            throw "$Description '$safeLockPath' is not a regular non-reparse file."
        }
        Assert-PhysicalPackagePathUnderRoot -Path $safeLockPath -Root $PackageRoot -Description $Description | Out-Null

        # FileStream does not expose its FileShare flags. Acquire-* below uses
        # FileShare.None, and this probe must therefore fail while this exact
        # handle is live. A successful probe means the caller supplied an
        # arbitrary readable stream or a non-exclusive handle.
        $probe = $null
        $sharingViolationObserved = $false
        try {
            $safeLockPath = Assert-PackagePublicationPathImmediatelyBeforeAccess -PackageRoot $PackageRoot -Path $safeLockPath -Description "$Description probe"
            $probe = [System.IO.FileStream]::new(
                $safeLockPath,
                [System.IO.FileMode]::Open,
                [System.IO.FileAccess]::Read,
                [System.IO.FileShare]::ReadWrite
            )
            throw "$Description '$safeLockPath' is not exclusively held by the supplied handle."
        } catch [System.IO.IOException] {
            # ERROR_SHARING_VIOLATION is the only admissible proof that the
            # live FileShare.None handle denies this probe. Other IO failures
            # (missing path, access denied, bad device, and so on) must not be
            # misclassified as evidence of an exclusive lock.
            if (([int]$_.Exception.HResult -band 0xffff) -ne 32) {
                throw "$Description lock probe failed with non-sharing IOException HResult $($_.Exception.HResult); refusing to treat it as exclusivity proof."
            }
            $sharingViolationObserved = $true
        } finally {
            if ($null -ne $probe) {
                $probe.Dispose()
            }
        }
        if (-not $sharingViolationObserved) {
            throw "$Description '$safeLockPath' did not produce the required sharing-violation lock proof."
        }
        return $safeLockPath
    } catch {
        throw "$Description is not the live exclusive lock: $($_.Exception.Message)"
    }
}

function Get-PackagePublicationNames {
    param(
        [Parameter(Mandatory)]
        [string]$OutputRoot
    )

    $outputName = [System.IO.Path]::GetFileName($OutputRoot)
    if ([string]::IsNullOrWhiteSpace($outputName)) {
        throw "OutputRoot '$OutputRoot' has no package-specific directory name."
    }
    $outputParent = [System.IO.Path]::GetDirectoryName($OutputRoot)
    return [pscustomobject]@{
        OutputName = $outputName
        OutputParent = $outputParent
        JournalPath = Join-Path $outputParent "$outputName.publication.json"
        LockPath = Join-Path $outputParent "$outputName.publication.lock"
    }
}

function Assert-ExactPackageDirectoryFiles {
    param(
        [Parameter(Mandatory)]
        [string]$Directory,

        [Parameter(Mandatory)]
        [string]$PackageRoot,

        [Parameter(Mandatory)]
        [string[]]$ExpectedNames,

        [Parameter(Mandatory)]
        [string]$Description
    )

    Assert-PhysicalPackagePathUnderRoot -Path $Directory -Root $PackageRoot -Description $Description | Out-Null
    if (-not (Test-Path -LiteralPath $Directory -PathType Container)) {
        throw "$Description '$Directory' is absent."
    }
    if (@($ExpectedNames | Select-Object -Unique).Count -ne $ExpectedNames.Count) {
        throw "$Description expected-file contract contains duplicate names."
    }
    $items = @(Get-ChildItem -LiteralPath $Directory -Force)
    if ($items.Count -ne $ExpectedNames.Count) {
        throw "$Description '$Directory' contains $($items.Count) entries; expected exactly $($ExpectedNames.Count)."
    }
    foreach ($item in $items) {
        if (($item.Attributes -band [System.IO.FileAttributes]::ReparsePoint) -ne 0 -or $item.PSIsContainer) {
            throw "$Description contains non-regular or reparse entry '$($item.FullName)'."
        }
        if ($ExpectedNames -cnotcontains $item.Name) {
            throw "$Description contains unexpected entry '$($item.Name)'."
        }
        Assert-SafePackageLeafPath -Path $item.FullName -Root $PackageRoot -Description "$Description entry" | Out-Null
    }
    foreach ($expectedName in $ExpectedNames) {
        $expectedPath = Join-Path $Directory $expectedName
        Assert-NonEmptyFile -Path $expectedPath -Description "$Description expected file"
    }
}

function Get-PackageDirectoryHashes {
    param(
        [Parameter(Mandatory)]
        [string]$Directory,

        [Parameter(Mandatory)]
        [string]$PackageRoot,

        [Parameter(Mandatory)]
        [string[]]$ExpectedNames,

        [Parameter(Mandatory)]
        [string]$Description
    )

    Assert-ExactPackageDirectoryFiles -Directory $Directory -PackageRoot $PackageRoot -ExpectedNames $ExpectedNames -Description $Description
    $hashes = [ordered]@{}
    foreach ($artifactName in $ExpectedNames) {
        $artifactPath = Join-Path $Directory $artifactName
        $hashes[$artifactName] = Get-Sha256Hex -Path $artifactPath
    }
    return $hashes
}

function Assert-PackageManifestBinding {
    param(
        [Parameter(Mandatory)]
        [string]$Directory,

        [Parameter(Mandatory)]
        [string[]]$ExpectedNames,

        [Parameter(Mandatory)]
        [object]$Hashes,

        [Parameter(Mandatory)]
        [string]$ExpectedManifestSemanticHash,

        [Parameter(Mandatory)]
        [string]$Description
    )

    $manifestNames = @($ExpectedNames | Where-Object { $_ -like '*.manifest.json' })
    $msiNames = @($ExpectedNames | Where-Object { $_ -like '*.msi' })
    $checksumNames = @($ExpectedNames | Where-Object { $_ -like '*.msi.sha256' })
    if ($manifestNames.Count -ne 1 -or $msiNames.Count -ne 1 -or $checksumNames.Count -ne 1) {
        throw "$Description does not have exactly one MSI, checksum, and manifest artifact; refusing publication."
    }
    try {
        $manifestPath = Join-Path $Directory $manifestNames[0]
        $manifest = [System.IO.File]::ReadAllText($manifestPath, [System.Text.UTF8Encoding]::new($false)) | ConvertFrom-Json
        $actualManifestSemanticHash = Get-PackageManifestSemanticHash -Manifest $manifest
        if ($ExpectedManifestSemanticHash -notmatch '^[0-9a-f]{64}$' -or
            $ExpectedManifestSemanticHash -cne $ExpectedManifestSemanticHash.ToLowerInvariant() -or
            $actualManifestSemanticHash -cne $ExpectedManifestSemanticHash) {
            throw 'manifest canonical semantic digest does not match the journaled full-manifest binding.'
        }
        if ([string]$manifest.artifact.file -cne $msiNames[0] -or
            [string]$manifest.artifact.sha256 -cne ([string]$Hashes[$msiNames[0]]) -or
            [string]$manifest.artifact.checksumFile -cne $checksumNames[0]) {
            throw 'manifest artifact identity or digest does not match the staged artifact set.'
        }
        $checksumPath = Join-Path $Directory $checksumNames[0]
        $expectedChecksum = ([string]$Hashes[$msiNames[0]]) + ' *' + $msiNames[0] + [Environment]::NewLine
        if ([System.IO.File]::ReadAllText($checksumPath, [System.Text.UTF8Encoding]::new($false)) -cne $expectedChecksum) {
            throw 'checksum content does not match the staged MSI digest.'
        }
    } catch {
        throw "$Description manifest/checksum binding is invalid; refusing publication: $($_.Exception.Message)"
    }
}

function Assert-PackageManifestInputContract {
    param(
        [Parameter(Mandatory)]
        [string]$Directory,

        [Parameter(Mandatory)]
        [string[]]$ExpectedNames,

        [Parameter(Mandatory)]
        [object]$ExpectedInputContract,

        [Parameter(Mandatory)]
        [string]$Description
    )

    $manifestNames = @($ExpectedNames | Where-Object { $_ -like '*.manifest.json' })
    if ($manifestNames.Count -ne 1) {
        throw "$Description has no unique manifest for input-integrity binding."
    }
    try {
        $manifestPath = Join-Path $Directory $manifestNames[0]
        $manifest = [System.IO.File]::ReadAllText($manifestPath, [System.Text.UTF8Encoding]::new($false)) | ConvertFrom-Json
        if ($null -eq $manifest.PSObject.Properties['inputIntegrity']) {
            throw 'manifest has no inputIntegrity contract.'
        }
        $actual = Get-PackagePublicationInputContract -InputContract $manifest.inputIntegrity
        $expected = Get-PackagePublicationInputContract -InputContract $ExpectedInputContract
        if ((ConvertTo-PackagePublicationCanonicalJson -Value $actual) -cne (ConvertTo-PackagePublicationCanonicalJson -Value $expected)) {
            throw 'manifest inputIntegrity does not exactly match the publication input contract.'
        }
    } catch {
        throw "$Description manifest input-integrity binding is invalid; refusing publication: $($_.Exception.Message)"
    }
}

function Assert-PackageDirectoryHashes {
    param(
        [Parameter(Mandatory)]
        [string]$Directory,

        [Parameter(Mandatory)]
        [string]$PackageRoot,

        [Parameter(Mandatory)]
        [string[]]$ExpectedNames,

        [Parameter(Mandatory)]
        [object]$ExpectedHashes,

        [Parameter(Mandatory)]
        [string]$ExpectedManifestSemanticHash,

        [Parameter(Mandatory)]
        [string]$Description
    )

    $actualHashes = Get-PackageDirectoryHashes -Directory $Directory -PackageRoot $PackageRoot -ExpectedNames $ExpectedNames -Description $Description
    foreach ($artifactName in $ExpectedNames) {
        $expectedHash = [string]$ExpectedHashes.$artifactName
        if ([string]::IsNullOrWhiteSpace($expectedHash) -or $actualHashes[$artifactName] -cne $expectedHash.ToLowerInvariant()) {
            throw "$Description artifact '$artifactName' does not match its journaled SHA-256."
        }
    }
    Assert-PackageManifestBinding -Directory $Directory -ExpectedNames $ExpectedNames -Hashes $actualHashes -ExpectedManifestSemanticHash $ExpectedManifestSemanticHash -Description $Description
}

function Write-PackagePublicationJournal {
    param(
        [Parameter(Mandatory)]
        [string]$PackageRoot,

        [Parameter(Mandatory)]
        [string]$JournalPath,

        [Parameter(Mandatory)]
        [object]$State,

        [Parameter(Mandatory)]
        [string]$LockPath,

        [Parameter(Mandatory)]
        [System.IO.FileStream]$LockStream
    )

    Assert-HeldPackagePublicationLock -PackageRoot $PackageRoot -LockPath $LockPath -LockStream $LockStream | Out-Null
    $safeJournalPath = Assert-PackagePublicationPathImmediatelyBeforeAccess -PackageRoot $PackageRoot -Path $JournalPath -Description 'Package publication journal'
    $journalParent = [System.IO.Path]::GetDirectoryName($safeJournalPath)
    Assert-PhysicalPackagePathUnderRoot -Path $journalParent -Root $PackageRoot -Description 'Package publication journal parent' | Out-Null

    $existingRecords = @()
    if (Test-Path -LiteralPath $safeJournalPath -PathType Leaf) {
        $existingRecords = @(Read-PackagePublicationJournalRecords -PackageRoot $PackageRoot -JournalPath $safeJournalPath)
    }
    $record = [ordered]@{
        schema = $script:PackagePublicationJournalSchema
        phase = [string]$State.phase
        sequence = $existingRecords.Count + 1
        operationId = [string]$State.operationId
        outputRoot = [string]$State.outputRoot
        stagingRoot = [string]$State.stagingRoot
        backupRoot = [string]$State.backupRoot
        artifactNames = @($State.artifactNames | ForEach-Object { [string]$_ })
        hadPrevious = [bool]$State.hadPrevious
        stagedHashes = Get-PackagePublicationMapEntries -Value $State.stagedHashes -Description 'Package publication staged hash contract'
        previousHashes = Get-PackagePublicationMapEntries -Value $State.previousHashes -Description 'Package publication previous hash contract'
        stagedManifestSemanticSha256 = [string]$State.stagedManifestSemanticSha256
        previousManifestSemanticSha256 = [string]$State.previousManifestSemanticSha256
        inputContract = Get-PackagePublicationInputContract -InputContract $State.inputContract
        previousRecordHash = if ($existingRecords.Count -eq 0) { '' } else { [string]$existingRecords[-1].recordHash }
        recordHash = ''
    }
    $record.recordHash = Get-PackagePublicationRecordHash -Record ([pscustomobject]$record)
    $candidate = ($record | ConvertTo-Json -Depth 32 -Compress | ConvertFrom-Json)
    $allRecords = @($existingRecords + @($candidate))
    Assert-PackagePublicationJournalChain -PackageRoot $PackageRoot -Records $allRecords | Out-Null
    $json = $candidate | ConvertTo-Json -Depth 32 -Compress
    $stream = $null
    try {
        # The held FileStream excludes cooperating writers. The journal is
        # append-only and chain-validated; a torn or partially written record
        # is deliberately fatal, so recovery preserves every byte instead of
        # guessing which earlier phase was durable. The path check immediately
        # before opening is not a hostile-filesystem no-follow guarantee.
        $safeJournalPath = Assert-PackagePublicationPathImmediatelyBeforeAccess -PackageRoot $PackageRoot -Path $safeJournalPath -Description 'Package publication journal immediately before write'
        $mode = if (Test-Path -LiteralPath $safeJournalPath -PathType Leaf) {
            [System.IO.FileMode]::Append
        } else {
            [System.IO.FileMode]::CreateNew
        }
        $stream = [System.IO.FileStream]::new(
            $safeJournalPath,
            $mode,
            [System.IO.FileAccess]::Write,
            [System.IO.FileShare]::None
        )
        $bytes = [System.Text.UTF8Encoding]::new($false).GetBytes($json + [Environment]::NewLine)
        $stream.Write($bytes, 0, $bytes.Length)
        $stream.Flush($true)
    } finally {
        if ($null -ne $stream) {
            $stream.Dispose()
        }
    }
}

function Read-PackagePublicationJournal {
    param(
        [Parameter(Mandatory)]
        [string]$PackageRoot,

        [Parameter(Mandatory)]
        [string]$JournalPath
    )

    $records = @(Read-PackagePublicationJournalRecords -PackageRoot $PackageRoot -JournalPath $JournalPath)
    return $records[$records.Count - 1]
}

function Read-PackagePublicationJournalRecords {
    param(
        [Parameter(Mandatory)]
        [string]$PackageRoot,

        [Parameter(Mandatory)]
        [string]$JournalPath
    )

    $safeJournalPath = Assert-PackagePublicationPathImmediatelyBeforeAccess -PackageRoot $PackageRoot -Path $JournalPath -Description 'Package publication journal immediately before read'
    try {
        $text = [System.IO.File]::ReadAllText($safeJournalPath, [System.Text.UTF8Encoding]::new($false))
        if ([string]::IsNullOrEmpty($text) -or -not $text.EndsWith("`n", [System.StringComparison]::Ordinal)) {
            throw 'journal does not end in a complete newline-delimited record.'
        }
        $rawLines = @($text -split "`n")
        if ($rawLines.Count -lt 2 -or $rawLines[-1] -cne '') {
            throw 'journal newline framing is incomplete.'
        }
        $records = [System.Collections.Generic.List[object]]::new()
        for ($index = 0; $index -lt ($rawLines.Count - 1); $index++) {
            $line = $rawLines[$index].TrimEnd("`r")
            if ([string]::IsNullOrWhiteSpace($line)) {
                throw "journal record $index is blank or torn."
            }
            try {
                $records.Add(($line | ConvertFrom-Json))
            } catch {
                throw "journal record $index is corrupt."
            }
        }
        if ($records.Count -eq 0) {
            throw 'journal contains no complete durable phase record.'
        }
        Assert-PackagePublicationJournalChain -PackageRoot $PackageRoot -Records $records.ToArray() | Out-Null
        return $records.ToArray()
    } catch {
        throw "Package publication journal '$safeJournalPath' is unreadable, torn, or provenance-invalid; preserving all package bytes and refusing recovery: $($_.Exception.Message)"
    }
}

function Acquire-PackagePublicationLock {
    param(
        [Parameter(Mandatory)]
        [string]$PackageRoot,

        [Parameter(Mandatory)]
        [string]$LockPath
    )

    $safeLockPath = Assert-PackagePublicationPathImmediatelyBeforeAccess -PackageRoot $PackageRoot -Path $LockPath -Description 'Package publication lock immediately before open'
    $lockStream = $null
    try {
        $lockStream = [System.IO.FileStream]::new(
            $safeLockPath,
            [System.IO.FileMode]::OpenOrCreate,
            [System.IO.FileAccess]::ReadWrite,
            [System.IO.FileShare]::None
        )
        Assert-HeldPackagePublicationLock -PackageRoot $PackageRoot -LockPath $safeLockPath -LockStream $lockStream | Out-Null
        return $lockStream
    } catch {
        if ($null -ne $lockStream) {
            $lockStream.Dispose()
        }
        throw "Could not acquire exclusive package publication lock '$safeLockPath'; refusing concurrent or unsafe publication: $($_.Exception.Message)"
    }
}

function Assert-PackagePublicationJournalShape {
    param(
        [Parameter(Mandatory)]
        [string]$PackageRoot,

        [Parameter()]
        [AllowEmptyString()]
        [string]$ExpectedOutputRoot,

        [Parameter(Mandatory)]
        [object]$Journal
    )

    $expectedFields = @($script:PackagePublicationJournalFields)
    $actualFields = @($Journal.PSObject.Properties.Name)
    $missingFields = @($expectedFields | Where-Object { $actualFields -notcontains $_ })
    $unexpectedFields = @($actualFields | Where-Object { $expectedFields -notcontains $_ })
    if ($missingFields.Count -gt 0 -or $unexpectedFields.Count -gt 0) {
        throw "Package publication journal fields are not exact (missing: '$($missingFields -join ', ')'; unexpected: '$($unexpectedFields -join ', ')'); preserving bytes and refusing recovery."
    }
    if ($Journal.schema -is [string] -or [int64]$Journal.schema -ne $script:PackagePublicationJournalSchema) {
        throw "Package publication journal schema '$($Journal.schema)' is unsupported; preserving bytes and refusing recovery."
    }
    if ([string]$Journal.phase -notin $script:PackagePublicationJournalPhases) {
        throw "Package publication journal phase '$($Journal.phase)' is unsupported; preserving bytes and refusing recovery."
    }
    if ($Journal.sequence -is [string] -or [int64]$Journal.sequence -lt 1 -or [int64]$Journal.sequence -gt [int64][int]::MaxValue) {
        throw 'Package publication journal sequence is not a positive bounded integer; preserving bytes and refusing recovery.'
    }
    $journalOutput = Assert-PhysicalPackagePathUnderRoot -Path ([string]$Journal.outputRoot) -Root $PackageRoot -Description 'Journaled final package directory'
    $journalStage = Assert-PhysicalPackagePathUnderRoot -Path ([string]$Journal.stagingRoot) -Root $PackageRoot -Description 'Journaled staged package directory'
    $journalBackup = Assert-PhysicalPackagePathUnderRoot -Path ([string]$Journal.backupRoot) -Root $PackageRoot -Description 'Journaled package backup directory'
    if (-not [string]::IsNullOrWhiteSpace($ExpectedOutputRoot) -and -not $journalOutput.Equals($ExpectedOutputRoot, [System.StringComparison]::OrdinalIgnoreCase)) {
        throw "Package publication journal targets '$journalOutput', not the requested final '$ExpectedOutputRoot'; preserving bytes and refusing recovery."
    }
    $outputName = [System.IO.Path]::GetFileName($journalOutput)
    $operationId = [string]$Journal.operationId
    if ($operationId -notmatch '^[0-9a-f]{32}$') {
        throw "Package publication journal operation id '$operationId' is not a canonical GUID token; preserving bytes and refusing recovery."
    }
    if (-not ([System.IO.Path]::GetFileName($journalStage)).Equals("$outputName.staging.$operationId", [System.StringComparison]::OrdinalIgnoreCase) -or
        -not ([System.IO.Path]::GetFileName($journalBackup)).Equals("$outputName.backup.$operationId", [System.StringComparison]::OrdinalIgnoreCase)) {
        throw 'Package publication journal staging/backup names do not bind to its output and operation id; preserving bytes and refusing recovery.'
    }
    $outputParent = [System.IO.Path]::GetDirectoryName($journalOutput)
    foreach ($candidate in @($journalStage, $journalBackup)) {
        if (-not ([System.IO.Path]::GetDirectoryName($candidate)).Equals($outputParent, [System.StringComparison]::OrdinalIgnoreCase)) {
            throw 'Package publication journal paths are not physical siblings; preserving bytes and refusing recovery.'
        }
    }
    if ($Journal.artifactNames -isnot [System.Array]) {
        throw 'Package publication journal artifactNames is not an array; preserving bytes and refusing recovery.'
    }
    $artifactNames = @($Journal.artifactNames | ForEach-Object { [string]$_ })
    if ($artifactNames.Count -eq 0 -or @($artifactNames | Select-Object -Unique).Count -ne $artifactNames.Count) {
        throw 'Package publication journal contains an empty or duplicate artifact contract; preserving bytes and refusing recovery.'
    }
    foreach ($artifactName in $artifactNames) {
        if ([string]::IsNullOrWhiteSpace($artifactName) -or
            [System.IO.Path]::IsPathRooted($artifactName) -or
            -not ([System.IO.Path]::GetFileName($artifactName)).Equals($artifactName, [System.StringComparison]::Ordinal)) {
            throw "Package publication journal contains unsafe artifact name '$artifactName'; preserving bytes and refusing recovery."
        }
    }
    $stagedHashes = Get-PackagePublicationMapEntries -Value $Journal.stagedHashes -Description 'Package publication staged hash contract'
    if ($stagedHashes.Count -ne $artifactNames.Count) {
        throw 'Package publication journal staged hash count does not match its artifact contract; preserving bytes and refusing recovery.'
    }
    foreach ($artifactName in $artifactNames) {
        if (-not $stagedHashes.Contains($artifactName) -or [string]$stagedHashes[$artifactName] -notmatch '^[0-9a-f]{64}$') {
            throw "Package publication journal has no canonical staged SHA-256 for '$artifactName'; preserving bytes and refusing recovery."
        }
    }
    if ($Journal.hadPrevious -isnot [bool]) {
        throw 'Package publication journal hadPrevious is not a JSON boolean; preserving bytes and refusing recovery.'
    }
    $hadPrevious = [bool]$Journal.hadPrevious
    $previousHashes = Get-PackagePublicationMapEntries -Value $Journal.previousHashes -Description 'Package publication previous hash contract'
    if (($hadPrevious -and $previousHashes.Count -ne $artifactNames.Count) -or
        (-not $hadPrevious -and $previousHashes.Count -ne 0)) {
        throw 'Package publication journal previous hash contract is inconsistent; preserving bytes and refusing recovery.'
    }
    foreach ($artifactName in $artifactNames) {
        if ($hadPrevious -and (-not $previousHashes.Contains($artifactName) -or [string]$previousHashes[$artifactName] -notmatch '^[0-9a-f]{64}$')) {
            throw "Package publication journal has no canonical prior SHA-256 for '$artifactName'; preserving bytes and refusing recovery."
        }
    }
    if ($Journal.operationId -isnot [string] -or $Journal.outputRoot -isnot [string] -or
        $Journal.stagingRoot -isnot [string] -or $Journal.backupRoot -isnot [string]) {
        throw 'Package publication journal identity/path fields have non-string JSON types; preserving bytes and refusing recovery.'
    }
    if ($Journal.previousRecordHash -isnot [string] -or $Journal.recordHash -isnot [string]) {
        throw 'Package publication journal chain digest fields have non-string JSON types; preserving bytes and refusing recovery.'
    }
    $sequence = [int]$Journal.sequence
    $previousRecordHash = [string]$Journal.previousRecordHash
    if (($sequence -eq 1 -and $previousRecordHash -cne '') -or
        ($sequence -gt 1 -and $previousRecordHash -notmatch '^[0-9a-f]{64}$') -or
        [string]$Journal.recordHash -notmatch '^[0-9a-f]{64}$' -or
        [string]$Journal.recordHash -cne ([string]$Journal.recordHash).ToLowerInvariant()) {
        throw 'Package publication journal chain digest contract is invalid; preserving bytes and refusing recovery.'
    }
    $inputContract = Get-PackagePublicationInputContract -InputContract $Journal.inputContract
    $stagedManifestSemanticSha256 = [string]$Journal.stagedManifestSemanticSha256
    $previousManifestSemanticSha256 = [string]$Journal.previousManifestSemanticSha256
    if ($stagedManifestSemanticSha256 -notmatch '^[0-9a-f]{64}$' -or
        $stagedManifestSemanticSha256 -cne $stagedManifestSemanticSha256.ToLowerInvariant() -or
        ($hadPrevious -and ($previousManifestSemanticSha256 -notmatch '^[0-9a-f]{64}$' -or
                $previousManifestSemanticSha256 -cne $previousManifestSemanticSha256.ToLowerInvariant())) -or
        (-not $hadPrevious -and -not [string]::IsNullOrEmpty($previousManifestSemanticSha256))) {
        throw 'Package publication journal full-manifest semantic digest contract is invalid; preserving bytes and refusing recovery.'
    }
    $expectedRecordHash = Get-PackagePublicationRecordHash -Record $Journal
    if ([string]$Journal.recordHash -cne $expectedRecordHash) {
        throw 'Package publication journal record digest does not match its immutable bytes; preserving bytes and refusing recovery.'
    }
    return [pscustomobject]@{
        Schema = [int]$Journal.schema
        Sequence = $sequence
        OutputRoot = $journalOutput
        StagingRoot = $journalStage
        BackupRoot = $journalBackup
        ArtifactNames = $artifactNames
        StagedHashes = $stagedHashes
        PreviousHashes = $previousHashes
        StagedManifestSemanticSha256 = $stagedManifestSemanticSha256
        PreviousManifestSemanticSha256 = $previousManifestSemanticSha256
        HadPrevious = $hadPrevious
        Phase = [string]$Journal.phase
        OperationId = [string]$Journal.operationId
        InputContract = $inputContract
        PreviousRecordHash = $previousRecordHash
        RecordHash = [string]$Journal.recordHash
    }
}

function Get-PackagePublicationImmutableRecord {
    param(
        [Parameter(Mandatory)]
        [object]$Record
    )

    return [ordered]@{
        schema = [int]$Record.schema
        operationId = [string]$Record.operationId
        outputRoot = [string]$Record.outputRoot
        stagingRoot = [string]$Record.stagingRoot
        backupRoot = [string]$Record.backupRoot
        artifactNames = @($Record.artifactNames | ForEach-Object { [string]$_ })
        hadPrevious = [bool]$Record.hadPrevious
        stagedHashes = Get-PackagePublicationMapEntries -Value $Record.stagedHashes -Description 'Package publication staged hash contract'
        previousHashes = Get-PackagePublicationMapEntries -Value $Record.previousHashes -Description 'Package publication previous hash contract'
        inputContract = Get-PackagePublicationInputContract -InputContract $Record.inputContract
        stagedManifestSemanticSha256 = [string]$Record.stagedManifestSemanticSha256
        previousManifestSemanticSha256 = [string]$Record.previousManifestSemanticSha256
    }
}

function Assert-PackagePublicationJournalChain {
    param(
        [Parameter(Mandatory)]
        [string]$PackageRoot,

        [Parameter(Mandatory)]
        [object[]]$Records
    )

    if ($Records.Count -eq 0) {
        throw 'Package publication journal chain is empty; preserving bytes and refusing recovery.'
    }
    $normalized = [System.Collections.Generic.List[object]]::new()
    $expectedOutput = ''
    $immutableJson = ''
    $previousHash = ''
    $hadPrevious = $false
    $phaseSequence = $null
    for ($index = 0; $index -lt $Records.Count; $index++) {
        $record = $Records[$index]
        $shape = Assert-PackagePublicationJournalShape -PackageRoot $PackageRoot -ExpectedOutputRoot $expectedOutput -Journal $record
        if ($index -eq 0) {
            $expectedOutput = $shape.OutputRoot
            $hadPrevious = $shape.HadPrevious
            $phaseSequence = if ($hadPrevious) {
                @('prepared', 'previous-moved', 'staging-moved', 'final-validated', 'committed')
            } else {
                @('prepared', 'staging-moved', 'final-validated', 'committed')
            }
            $immutableJson = ConvertTo-PackagePublicationCanonicalJson -Value (Get-PackagePublicationImmutableRecord -Record $record)
        } else {
            $candidateImmutable = ConvertTo-PackagePublicationCanonicalJson -Value (Get-PackagePublicationImmutableRecord -Record $record)
            if ($candidateImmutable -cne $immutableJson) {
                throw "Package publication journal record $index changes the immutable operation/output/input contract; preserving bytes and refusing recovery."
            }
            if ($shape.PreviousRecordHash -cne $previousHash) {
                throw "Package publication journal record $index does not link to the prior record digest; preserving bytes and refusing recovery."
            }
        }
        if ($shape.Sequence -ne ($index + 1)) {
            throw "Package publication journal record $index has sequence $($shape.Sequence), expected $($index + 1); preserving bytes and refusing recovery."
        }
        if ($shape.Phase -cne $phaseSequence[$index]) {
            throw "Package publication journal phase '$($shape.Phase)' is not the legal phase $($index + 1) for this operation; preserving bytes and refusing recovery."
        }
        if ($shape.HadPrevious -ne $hadPrevious) {
            throw 'Package publication journal changes hadPrevious across records; preserving bytes and refusing recovery.'
        }
        $normalized.Add($shape)
        $previousHash = $shape.RecordHash
    }
    if ($normalized[-1].Phase -eq 'committed' -and $Records.Count -ne $phaseSequence.Count) {
        throw 'Package publication journal contains records after committed; preserving bytes and refusing recovery.'
    }
    return ,$normalized.ToArray()
}

function Recover-PackagePublication {
    param(
        [Parameter(Mandatory)]
        [string]$PackageRoot,

        [Parameter(Mandatory)]
        [string]$OutputRoot,

        [Parameter(Mandatory)]
        [string]$LockPath,

        [Parameter(Mandatory)]
        [System.IO.FileStream]$LockStream
    )

    Assert-HeldPackagePublicationLock -PackageRoot $PackageRoot -LockPath $LockPath -LockStream $LockStream | Out-Null
    $names = Get-PackagePublicationNames -OutputRoot $OutputRoot
    $safeJournalPath = Assert-PackagePublicationPathImmediatelyBeforeAccess -PackageRoot $PackageRoot -Path $names.JournalPath -Description 'Package publication journal immediately before recovery read'
    if (-not (Test-Path -LiteralPath $safeJournalPath -PathType Leaf)) {
        $orphanEntries = @(Get-ChildItem -LiteralPath $names.OutputParent -Force | Where-Object {
                $_.Name.StartsWith("$($names.OutputName).staging.", [System.StringComparison]::OrdinalIgnoreCase) -or
                $_.Name.StartsWith("$($names.OutputName).backup.", [System.StringComparison]::OrdinalIgnoreCase)
            })
        if ($orphanEntries.Count -gt 0) {
            throw "Package publication found orphan staging/backup entries without a durable journal under '$($names.OutputParent)'; preserving them and refusing ambiguous cleanup."
        }
        return
    }

    $journalRecords = @(Read-PackagePublicationJournalRecords -PackageRoot $PackageRoot -JournalPath $safeJournalPath)
    $journal = $journalRecords[-1]
    $requestedOutput = Assert-PhysicalPackagePathUnderRoot -Path $OutputRoot -Root $PackageRoot -Description 'Requested final package directory'
    $record = Assert-PackagePublicationJournalShape -PackageRoot $PackageRoot -ExpectedOutputRoot $requestedOutput -Journal $journal
    $finalPresent = Test-Path -LiteralPath $record.OutputRoot
    $stagePresent = Test-Path -LiteralPath $record.StagingRoot
    $backupPresent = Test-Path -LiteralPath $record.BackupRoot
    $finalNew = $false
    $finalOld = $false
    $stageNew = $false
    $backupOld = $false

    if ($finalPresent) {
        try {
            Assert-PackageDirectoryHashes -Directory $record.OutputRoot -PackageRoot $PackageRoot -ExpectedNames $record.ArtifactNames -ExpectedHashes $record.StagedHashes -ExpectedManifestSemanticHash $record.StagedManifestSemanticSha256 -Description 'Journaled final package set'
            $finalNew = $true
        } catch {
            if ($record.HadPrevious) {
                try {
                    Assert-PackageDirectoryHashes -Directory $record.OutputRoot -PackageRoot $PackageRoot -ExpectedNames $record.ArtifactNames -ExpectedHashes $record.PreviousHashes -ExpectedManifestSemanticHash $record.PreviousManifestSemanticSha256 -Description 'Journaled prior final package set'
                    $finalOld = $true
                } catch {
                    throw "Journaled final '$($record.OutputRoot)' is neither the expected new nor prior package set; preserving bytes and refusing recovery: $($_.Exception.Message)"
                }
            } else {
                throw "Journaled final '$($record.OutputRoot)' is not the expected new package set; preserving bytes and refusing recovery: $($_.Exception.Message)"
            }
        }
    }
    if ($stagePresent) {
        Assert-PackageDirectoryHashes -Directory $record.StagingRoot -PackageRoot $PackageRoot -ExpectedNames $record.ArtifactNames -ExpectedHashes $record.StagedHashes -ExpectedManifestSemanticHash $record.StagedManifestSemanticSha256 -Description 'Journaled staged package set'
        $stageNew = $true
    }
    if ($backupPresent) {
        if (-not $record.HadPrevious) {
            throw "Journaled backup '$($record.BackupRoot)' exists although no prior final was recorded; preserving bytes and refusing recovery."
        }
        Assert-PackageDirectoryHashes -Directory $record.BackupRoot -PackageRoot $PackageRoot -ExpectedNames $record.ArtifactNames -ExpectedHashes $record.PreviousHashes -ExpectedManifestSemanticHash $record.PreviousManifestSemanticSha256 -Description 'Journaled rollback package set'
        $backupOld = $true
    }

    if ($finalNew) {
        Assert-HeldPackagePublicationLock -PackageRoot $PackageRoot -LockPath $LockPath -LockStream $LockStream | Out-Null
        if ($backupPresent -and -not $backupOld) {
            throw 'A valid new final is paired with an invalid rollback backup; preserving both and refusing recovery.'
        }
        if ($stagePresent -and -not $stageNew) {
            throw 'A valid new final is paired with invalid staged bytes; preserving both and refusing recovery.'
        }
        if ($stageNew) {
            Remove-SafePackagePath -Path $record.StagingRoot -Root $PackageRoot -Description 'Recovered superseded package staging'
        }
        if ($backupOld) {
            Remove-SafePackagePath -Path $record.BackupRoot -Root $PackageRoot -Description 'Recovered superseded package backup'
        }
        Remove-SafePackagePath -Path $safeJournalPath -Root $PackageRoot -Description 'Committed package publication journal'
        return
    }

    if ($finalOld) {
        if (-not $backupPresent -and $record.Phase -eq 'prepared') {
            Assert-HeldPackagePublicationLock -PackageRoot $PackageRoot -LockPath $LockPath -LockStream $LockStream | Out-Null
            if ($stageNew) {
                Remove-SafePackagePath -Path $record.StagingRoot -Root $PackageRoot -Description 'Discard unstarted package staging'
            }
            Remove-SafePackagePath -Path $safeJournalPath -Root $PackageRoot -Description 'Unstarted package publication journal'
            return
        }
        throw 'Journaled publication has a prior final plus an incomplete/ambiguous transition; preserving all bytes and refusing recovery.'
    }

    if ($backupOld) {
        Assert-HeldPackagePublicationLock -PackageRoot $PackageRoot -LockPath $LockPath -LockStream $LockStream | Out-Null
        Move-SafePackageDirectory -SourcePath $record.BackupRoot -DestinationPath $record.OutputRoot -Root $PackageRoot -Description 'Recover prior package final'
        if ($stageNew) {
            Remove-SafePackagePath -Path $record.StagingRoot -Root $PackageRoot -Description 'Discard aborted package staging after prior restore'
        }
        Assert-PackageDirectoryHashes -Directory $record.OutputRoot -PackageRoot $PackageRoot -ExpectedNames $record.ArtifactNames -ExpectedHashes $record.PreviousHashes -ExpectedManifestSemanticHash $record.PreviousManifestSemanticSha256 -Description 'Restored prior package set'
        Remove-SafePackagePath -Path $safeJournalPath -Root $PackageRoot -Description 'Recovered rollback publication journal'
        return
    }

    if ($stageNew -and -not $record.HadPrevious) {
        Assert-HeldPackagePublicationLock -PackageRoot $PackageRoot -LockPath $LockPath -LockStream $LockStream | Out-Null
        Move-SafePackageDirectory -SourcePath $record.StagingRoot -DestinationPath $record.OutputRoot -Root $PackageRoot -Description 'Complete new package publication'
        Assert-PackageDirectoryHashes -Directory $record.OutputRoot -PackageRoot $PackageRoot -ExpectedNames $record.ArtifactNames -ExpectedHashes $record.StagedHashes -ExpectedManifestSemanticHash $record.StagedManifestSemanticSha256 -Description 'Completed new package set'
        Remove-SafePackagePath -Path $safeJournalPath -Root $PackageRoot -Description 'Completed new publication journal'
        return
    }

    if (-not $record.HadPrevious -and -not $stagePresent -and -not $backupPresent -and $record.Phase -eq 'prepared') {
        Assert-HeldPackagePublicationLock -PackageRoot $PackageRoot -LockPath $LockPath -LockStream $LockStream | Out-Null
        Remove-SafePackagePath -Path $safeJournalPath -Root $PackageRoot -Description 'Empty package publication journal'
        return
    }
    throw 'Journaled package publication has no safely recoverable final, staging, or backup state; preserving bytes and refusing recovery.'
}

function New-PackagePublicationLayout {
    param(
        [Parameter(Mandatory)]
        [string]$PackageRoot,

        [Parameter(Mandatory)]
        [string]$OutputRoot
    )

    $safeOutputRoot = Assert-PhysicalPackagePathUnderRoot -Path $OutputRoot -Root $PackageRoot -Description 'OutputRoot'
    $names = Get-PackagePublicationNames -OutputRoot $safeOutputRoot
    $outputParent = Assert-PhysicalPackagePathUnderRoot -Path $names.OutputParent -Root $PackageRoot -Description 'OutputRoot parent'
    New-SafePackageDirectory -Path $outputParent -Root $PackageRoot -Description 'OutputRoot parent' | Out-Null
    $lockStream = Acquire-PackagePublicationLock -PackageRoot $PackageRoot -LockPath $names.LockPath
    try {
        Recover-PackagePublication -PackageRoot $PackageRoot -OutputRoot $safeOutputRoot -LockPath $names.LockPath -LockStream $lockStream
        $operationId = [Guid]::NewGuid().ToString('N')
        $stagingRoot = Join-Path $outputParent "$($names.OutputName).staging.$operationId"
        $backupRoot = Join-Path $outputParent "$($names.OutputName).backup.$operationId"
        foreach ($uniquePath in @($stagingRoot, $backupRoot)) {
            if (Test-Path -LiteralPath $uniquePath) {
                throw "Unique package operation path '$uniquePath' already exists; refusing to reuse it."
            }
        }
        New-SafePackageDirectory -Path $stagingRoot -Root $PackageRoot -Description 'Package staging root' | Out-Null
        Assert-PhysicalPackagePathUnderRoot -Path $backupRoot -Root $PackageRoot -Description 'Package rollback backup' | Out-Null
        return [pscustomobject]@{
            OperationId = $operationId
            OutputRoot = $safeOutputRoot
            StagingRoot = $stagingRoot
            BackupRoot = $backupRoot
            JournalPath = $names.JournalPath
            LockPath = $names.LockPath
            LockStream = $lockStream
        }
    } catch {
        $lockStream.Dispose()
        throw
    }
}

function Publish-StagedPackageDirectory {
    param(
        [Parameter(Mandatory)]
        [string]$PackageRoot,

        [Parameter(Mandatory)]
        [string]$OutputRoot,

        [Parameter(Mandatory)]
        [string]$StagingRoot,

        [Parameter(Mandatory)]
        [string]$BackupRoot,

        [Parameter(Mandatory)]
        [string[]]$ArtifactNames,

        [Parameter(Mandatory)]
        [System.IO.FileStream]$LockStream,

        [Parameter(Mandatory)]
        [object]$InputContract,

        [Parameter(Mandatory)]
        [string]$ExpectedManifestSemanticHash
    )

    $names = Get-PackagePublicationNames -OutputRoot $OutputRoot
    Assert-HeldPackagePublicationLock -PackageRoot $PackageRoot -LockPath $names.LockPath -LockStream $LockStream | Out-Null
    $normalizedInputContract = Get-PackagePublicationInputContract -InputContract $InputContract
    $safeOutputRoot = Assert-PhysicalPackagePathUnderRoot -Path $OutputRoot -Root $PackageRoot -Description 'Final package directory'
    $safeStagingRoot = Assert-PhysicalPackagePathUnderRoot -Path $StagingRoot -Root $PackageRoot -Description 'Staged package directory'
    $safeBackupRoot = Assert-PhysicalPackagePathUnderRoot -Path $BackupRoot -Root $PackageRoot -Description 'Package rollback backup'
    $names = Get-PackagePublicationNames -OutputRoot $safeOutputRoot
    $outputParent = [System.IO.Path]::GetDirectoryName($safeOutputRoot)
    if (-not ([System.IO.Path]::GetDirectoryName($safeStagingRoot)).Equals($outputParent, [System.StringComparison]::OrdinalIgnoreCase) -or
        -not ([System.IO.Path]::GetDirectoryName($safeBackupRoot)).Equals($outputParent, [System.StringComparison]::OrdinalIgnoreCase)) {
        throw 'Package staging, rollback backup, and final directory must be physical siblings for bounded publication.'
    }

    $stagedHashes = Get-PackageDirectoryHashes -Directory $safeStagingRoot -PackageRoot $PackageRoot -ExpectedNames $ArtifactNames -Description 'Validated staged package set'
    $manifestNames = @($ArtifactNames | Where-Object { $_ -like '*.manifest.json' })
    if ($manifestNames.Count -ne 1) {
        throw 'Validated staged package set does not contain exactly one manifest for semantic publication binding.'
    }
    $stagedManifestSemanticHash = Get-PackageManifestSemanticHashFromFile -Path (Join-Path $safeStagingRoot $manifestNames[0]) -PackageRoot $PackageRoot
    if ($ExpectedManifestSemanticHash -notmatch '^[0-9a-f]{64}$' -or
        $ExpectedManifestSemanticHash -cne $ExpectedManifestSemanticHash.ToLowerInvariant() -or
        $stagedManifestSemanticHash -cne $ExpectedManifestSemanticHash) {
        throw 'Validated staged package set manifest semantic digest does not match the pre-publication binding.'
    }
    Assert-PackageManifestBinding -Directory $safeStagingRoot -ExpectedNames $ArtifactNames -Hashes $stagedHashes -ExpectedManifestSemanticHash $stagedManifestSemanticHash -Description 'Validated staged package set'
    Assert-PackageManifestInputContract -Directory $safeStagingRoot -ExpectedNames $ArtifactNames -ExpectedInputContract $normalizedInputContract -Description 'Validated staged package set'
    $hadPrevious = Test-Path -LiteralPath $safeOutputRoot
    $previousHashes = [ordered]@{}
    $previousManifestSemanticHash = ''
    if ($hadPrevious) {
        $previousHashes = Get-PackageDirectoryHashes -Directory $safeOutputRoot -PackageRoot $PackageRoot -ExpectedNames $ArtifactNames -Description 'Previously published package set'
        $previousManifestSemanticHash = Get-PackageManifestSemanticHashFromFile -Path (Join-Path $safeOutputRoot $manifestNames[0]) -PackageRoot $PackageRoot
        Assert-PackageManifestBinding -Directory $safeOutputRoot -ExpectedNames $ArtifactNames -Hashes $previousHashes -ExpectedManifestSemanticHash $previousManifestSemanticHash -Description 'Previously published package set'
    }
    if (Test-Path -LiteralPath $safeBackupRoot) {
        throw "Unique package rollback backup '$safeBackupRoot' already exists; refusing publication."
    }

    $journal = [ordered]@{
        schema = $script:PackagePublicationJournalSchema
        phase = 'prepared'
        operationId = [System.IO.Path]::GetFileName($safeStagingRoot).Substring($names.OutputName.Length + 9)
        outputRoot = $safeOutputRoot
        stagingRoot = $safeStagingRoot
        backupRoot = $safeBackupRoot
        artifactNames = @($ArtifactNames)
        hadPrevious = $hadPrevious
        stagedHashes = $stagedHashes
        previousHashes = $previousHashes
        stagedManifestSemanticSha256 = $stagedManifestSemanticHash
        previousManifestSemanticSha256 = $previousManifestSemanticHash
        inputContract = $normalizedInputContract
    }

    try {
        Assert-HeldPackagePublicationLock -PackageRoot $PackageRoot -LockPath $names.LockPath -LockStream $LockStream | Out-Null
        Write-PackagePublicationJournal -PackageRoot $PackageRoot -JournalPath $names.JournalPath -State $journal -LockPath $names.LockPath -LockStream $LockStream
        if ($hadPrevious) {
            Assert-HeldPackagePublicationLock -PackageRoot $PackageRoot -LockPath $names.LockPath -LockStream $LockStream | Out-Null
            Move-SafePackageDirectory -SourcePath $safeOutputRoot -DestinationPath $safeBackupRoot -Root $PackageRoot -Description 'Move prior package to rollback backup'
            $journal.phase = 'previous-moved'
            Write-PackagePublicationJournal -PackageRoot $PackageRoot -JournalPath $names.JournalPath -State $journal -LockPath $names.LockPath -LockStream $LockStream
        }
        Assert-HeldPackagePublicationLock -PackageRoot $PackageRoot -LockPath $names.LockPath -LockStream $LockStream | Out-Null
        Move-SafePackageDirectory -SourcePath $safeStagingRoot -DestinationPath $safeOutputRoot -Root $PackageRoot -Description 'Publish staged package'
        $journal.phase = 'staging-moved'
        Write-PackagePublicationJournal -PackageRoot $PackageRoot -JournalPath $names.JournalPath -State $journal -LockPath $names.LockPath -LockStream $LockStream
        Assert-PackageDirectoryHashes -Directory $safeOutputRoot -PackageRoot $PackageRoot -ExpectedNames $ArtifactNames -ExpectedHashes $stagedHashes -ExpectedManifestSemanticHash $stagedManifestSemanticHash -Description 'Published package set'
        $journal.phase = 'final-validated'
        Write-PackagePublicationJournal -PackageRoot $PackageRoot -JournalPath $names.JournalPath -State $journal -LockPath $names.LockPath -LockStream $LockStream
        if ($hadPrevious) {
            Assert-HeldPackagePublicationLock -PackageRoot $PackageRoot -LockPath $names.LockPath -LockStream $LockStream | Out-Null
            Remove-SafePackagePath -Path $safeBackupRoot -Root $PackageRoot -Description 'Superseded package rollback backup'
        }
        $journal.phase = 'committed'
        Write-PackagePublicationJournal -PackageRoot $PackageRoot -JournalPath $names.JournalPath -State $journal -LockPath $names.LockPath -LockStream $LockStream
        Assert-HeldPackagePublicationLock -PackageRoot $PackageRoot -LockPath $names.LockPath -LockStream $LockStream | Out-Null
        Remove-SafePackagePath -Path $names.JournalPath -Root $PackageRoot -Description 'Committed package publication journal'
    } catch {
        throw "Package publication stopped with durable journal '$($names.JournalPath)'; rerunning will validate and recover without discarding ambiguous bytes: $($_.Exception.Message)"
    }
    return $safeOutputRoot
}
