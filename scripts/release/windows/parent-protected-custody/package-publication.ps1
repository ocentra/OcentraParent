function New-PackagePublicationLayout {
    param(
        [Parameter(Mandatory)]
        [string]$PackageRoot,

        [Parameter(Mandatory)]
        [string]$OutputRoot
    )

    $safeOutputRoot = Assert-PhysicalPackagePathUnderRoot -Path $OutputRoot -Root $PackageRoot -Description 'OutputRoot'
    $outputParent = [System.IO.Path]::GetDirectoryName($safeOutputRoot)
    New-SafePackageDirectory -Path $outputParent -Root $PackageRoot -Description 'OutputRoot parent' | Out-Null
    $outputName = [System.IO.Path]::GetFileName($safeOutputRoot)
    if ([string]::IsNullOrWhiteSpace($outputName)) {
        throw "OutputRoot '$safeOutputRoot' has no package-specific directory name."
    }

    $operationId = [Guid]::NewGuid().ToString('N')
    $stagingRoot = Join-Path $outputParent "$outputName.staging.$operationId"
    $backupRoot = Join-Path $outputParent "$outputName.backup.$operationId"
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
        [string[]]$ArtifactNames
    )

    $safeOutputRoot = Assert-PhysicalPackagePathUnderRoot -Path $OutputRoot -Root $PackageRoot -Description 'Final package directory'
    $safeStagingRoot = Assert-PhysicalPackagePathUnderRoot -Path $StagingRoot -Root $PackageRoot -Description 'Staged package directory'
    $safeBackupRoot = Assert-PhysicalPackagePathUnderRoot -Path $BackupRoot -Root $PackageRoot -Description 'Package rollback backup'
    $outputParent = [System.IO.Path]::GetDirectoryName($safeOutputRoot)
    if (-not ([System.IO.Path]::GetDirectoryName($safeStagingRoot)).Equals($outputParent, [System.StringComparison]::OrdinalIgnoreCase) -or
        -not ([System.IO.Path]::GetDirectoryName($safeBackupRoot)).Equals($outputParent, [System.StringComparison]::OrdinalIgnoreCase)) {
        throw 'Package staging, rollback backup, and final directory must be physical siblings for bounded publication.'
    }

    Assert-ExactPackageDirectoryFiles -Directory $safeStagingRoot -PackageRoot $PackageRoot -ExpectedNames $ArtifactNames -Description 'Validated staged package set'
    $stagedHashes = @{}
    foreach ($artifactName in $ArtifactNames) {
        $stagedHashes[$artifactName] = Get-Sha256Hex -Path (Join-Path $safeStagingRoot $artifactName)
    }

    $hadPrevious = Test-Path -LiteralPath $safeOutputRoot
    if ($hadPrevious) {
        Assert-ExactPackageDirectoryFiles -Directory $safeOutputRoot -PackageRoot $PackageRoot -ExpectedNames $ArtifactNames -Description 'Previously published package set'
    }
    if (Test-Path -LiteralPath $safeBackupRoot) {
        throw "Unique package rollback backup '$safeBackupRoot' already exists; refusing publication."
    }

    $previousMoved = $false
    $stagingMoved = $false
    try {
        if ($hadPrevious) {
            Move-Item -LiteralPath $safeOutputRoot -Destination $safeBackupRoot
            $previousMoved = $true
        }
        Move-Item -LiteralPath $safeStagingRoot -Destination $safeOutputRoot
        $stagingMoved = $true
        Assert-ExactPackageDirectoryFiles -Directory $safeOutputRoot -PackageRoot $PackageRoot -ExpectedNames $ArtifactNames -Description 'Published package set'
        foreach ($artifactName in $ArtifactNames) {
            $publishedHash = Get-Sha256Hex -Path (Join-Path $safeOutputRoot $artifactName)
            if ($publishedHash -cne $stagedHashes[$artifactName]) {
                throw "Published package file '$artifactName' hash '$publishedHash' differs from validated staged hash '$($stagedHashes[$artifactName])'."
            }
        }
    } catch {
        $publishFailure = $_.Exception.Message
        $rollbackFailures = [System.Collections.Generic.List[string]]::new()
        if ($stagingMoved -and (Test-Path -LiteralPath $safeOutputRoot)) {
            try {
                Remove-SafePackagePath -Path $safeOutputRoot -Root $PackageRoot -Description 'Unpublished failed package set'
            } catch {
                $rollbackFailures.Add("remove unpublished final: $($_.Exception.Message)")
            }
        }
        if ($previousMoved -and (Test-Path -LiteralPath $safeBackupRoot)) {
            try {
                if (Test-Path -LiteralPath $safeOutputRoot) {
                    throw "final path '$safeOutputRoot' still exists"
                }
                Move-Item -LiteralPath $safeBackupRoot -Destination $safeOutputRoot
                Assert-ExactPackageDirectoryFiles -Directory $safeOutputRoot -PackageRoot $PackageRoot -ExpectedNames $ArtifactNames -Description 'Restored prior package set'
            } catch {
                $rollbackFailures.Add("restore prior package: $($_.Exception.Message)")
            }
        }
        if ($rollbackFailures.Count -gt 0) {
            throw "Package publication failed: $publishFailure. Rollback also failed: $($rollbackFailures -join '; '). Prior bytes remain at the final or unique backup path; refusing cleanup."
        }
        throw "Package publication failed and the prior final set was restored: $publishFailure"
    }

    if ($previousMoved -and (Test-Path -LiteralPath $safeBackupRoot)) {
        Remove-SafePackagePath -Path $safeBackupRoot -Root $PackageRoot -Description 'Superseded package rollback backup'
    }
    return $safeOutputRoot
}
