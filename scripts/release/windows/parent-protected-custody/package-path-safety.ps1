function Get-NormalizedPackagePath {
    param(
        [Parameter(Mandatory)]
        [string]$Path
    )

    $fullPath = [System.IO.Path]::GetFullPath($Path)
    $pathRoot = [System.IO.Path]::GetPathRoot($fullPath)
    if ($fullPath.Equals($pathRoot, [System.StringComparison]::OrdinalIgnoreCase)) {
        return $pathRoot
    }
    return $fullPath.TrimEnd([System.IO.Path]::DirectorySeparatorChar, [System.IO.Path]::AltDirectorySeparatorChar)
}

function Test-PackagePathAtOrBelow {
    param(
        [Parameter(Mandatory)]
        [string]$Path,

        [Parameter(Mandatory)]
        [string]$Root
    )

    $normalizedPath = Get-NormalizedPackagePath -Path $Path
    $normalizedRoot = Get-NormalizedPackagePath -Path $Root
    if ($normalizedPath.Equals($normalizedRoot, [System.StringComparison]::OrdinalIgnoreCase)) {
        return $true
    }
    $rootPrefix = $normalizedRoot.TrimEnd('\') + '\'
    return $normalizedPath.StartsWith($rootPrefix, [System.StringComparison]::OrdinalIgnoreCase)
}

function Assert-NoPackageReparseChain {
    param(
        [Parameter(Mandatory)]
        [string]$Path,

        [Parameter(Mandatory)]
        [string]$Description
    )

    $fullPath = Get-NormalizedPackagePath -Path $Path
    $pathRoot = [System.IO.Path]::GetPathRoot($fullPath)
    if ([string]::IsNullOrWhiteSpace($pathRoot) -or -not (Test-Path -LiteralPath $pathRoot -PathType Container)) {
        throw "$Description '$fullPath' has no existing filesystem root."
    }

    $segments = @()
    $relativePath = [System.IO.Path]::GetRelativePath($pathRoot, $fullPath)
    if ($relativePath -ne '.') {
        $segments = @($relativePath.Split(
                [char[]]@([System.IO.Path]::DirectorySeparatorChar, [System.IO.Path]::AltDirectorySeparatorChar),
                [System.StringSplitOptions]::RemoveEmptyEntries
            ))
    }

    $currentPath = $pathRoot
    foreach ($segment in $segments) {
        $currentPath = Join-Path $currentPath $segment
        if (-not (Test-Path -LiteralPath $currentPath)) {
            break
        }
        $item = Get-Item -LiteralPath $currentPath -Force
        if (($item.Attributes -band [System.IO.FileAttributes]::ReparsePoint) -ne 0) {
            throw "$Description '$fullPath' crosses reparse point '$currentPath'; refusing package filesystem access."
        }
        $resolvedCurrent = Get-NormalizedPackagePath -Path (Resolve-Path -LiteralPath $currentPath).ProviderPath
        $expectedCurrent = Get-NormalizedPackagePath -Path $currentPath
        if (-not $resolvedCurrent.Equals($expectedCurrent, [System.StringComparison]::OrdinalIgnoreCase)) {
            throw "$Description '$fullPath' resolves existing segment '$currentPath' to '$resolvedCurrent'; refusing physical path escape."
        }
    }
}

function Get-NearestExistingPackageAncestor {
    param(
        [Parameter(Mandatory)]
        [string]$Path
    )

    $candidate = Get-NormalizedPackagePath -Path $Path
    while (-not (Test-Path -LiteralPath $candidate)) {
        $parent = [System.IO.Path]::GetDirectoryName($candidate)
        if ([string]::IsNullOrWhiteSpace($parent) -or $parent.Equals($candidate, [System.StringComparison]::OrdinalIgnoreCase)) {
            throw "Package path '$Path' has no existing ancestor."
        }
        $candidate = Get-NormalizedPackagePath -Path $parent
    }
    return $candidate
}

function Assert-PhysicalPackagePathUnderRoot {
    param(
        [Parameter(Mandatory)]
        [string]$Path,

        [Parameter(Mandatory)]
        [string]$Root,

        [Parameter(Mandatory)]
        [string]$Description
    )

    $fullPath = Get-NormalizedPackagePath -Path $Path
    $fullRoot = Get-NormalizedPackagePath -Path $Root
    if (-not (Test-Path -LiteralPath $fullRoot -PathType Container)) {
        throw "$Description root '$fullRoot' must exist before physical containment is checked."
    }
    if (-not (Test-PackagePathAtOrBelow -Path $fullPath -Root $fullRoot)) {
        throw "$Description '$fullPath' must remain under '$fullRoot'."
    }

    Assert-NoPackageReparseChain -Path $fullRoot -Description "$Description root"
    Assert-NoPackageReparseChain -Path $fullPath -Description $Description
    $physicalRoot = Get-NormalizedPackagePath -Path (Resolve-Path -LiteralPath $fullRoot).ProviderPath
    $nearestExisting = Get-NearestExistingPackageAncestor -Path $fullPath
    $physicalAncestor = Get-NormalizedPackagePath -Path (Resolve-Path -LiteralPath $nearestExisting).ProviderPath
    if (-not (Test-PackagePathAtOrBelow -Path $physicalAncestor -Root $physicalRoot)) {
        throw "$Description '$fullPath' has physical ancestor '$physicalAncestor' outside '$physicalRoot'."
    }
    return $fullPath
}

function New-SafePackageDirectory {
    param(
        [Parameter(Mandatory)]
        [string]$Path,

        [Parameter(Mandatory)]
        [string]$Root,

        [Parameter(Mandatory)]
        [string]$Description
    )

    $fullRoot = Get-NormalizedPackagePath -Path $Root
    $fullPath = Assert-PhysicalPackagePathUnderRoot -Path $Path -Root $fullRoot -Description $Description
    $relativePath = [System.IO.Path]::GetRelativePath($fullRoot, $fullPath)
    if ($relativePath -eq '.') {
        return $fullRoot
    }

    $currentPath = $fullRoot
    $segments = @($relativePath.Split(
            [char[]]@([System.IO.Path]::DirectorySeparatorChar, [System.IO.Path]::AltDirectorySeparatorChar),
            [System.StringSplitOptions]::RemoveEmptyEntries
        ))
    foreach ($segment in $segments) {
        $currentPath = Join-Path $currentPath $segment
        if (Test-Path -LiteralPath $currentPath) {
            if (-not (Test-Path -LiteralPath $currentPath -PathType Container)) {
                throw "$Description '$fullPath' crosses non-directory '$currentPath'."
            }
        } else {
            New-Item -ItemType Directory -Path $currentPath | Out-Null
        }
        Assert-NoPackageReparseChain -Path $currentPath -Description $Description
    }
    Assert-PhysicalPackagePathUnderRoot -Path $fullPath -Root $fullRoot -Description $Description | Out-Null
    return $fullPath
}

function Assert-SafePackageLeafPath {
    param(
        [Parameter(Mandatory)]
        [string]$Path,

        [Parameter(Mandatory)]
        [string]$Root,

        [Parameter(Mandatory)]
        [string]$Description
    )

    $fullPath = Assert-PhysicalPackagePathUnderRoot -Path $Path -Root $Root -Description $Description
    $parentPath = [System.IO.Path]::GetDirectoryName($fullPath)
    Assert-PhysicalPackagePathUnderRoot -Path $parentPath -Root $Root -Description "$Description parent" | Out-Null
    if (Test-Path -LiteralPath $fullPath) {
        $item = Get-Item -LiteralPath $fullPath -Force
        if (($item.Attributes -band [System.IO.FileAttributes]::ReparsePoint) -ne 0) {
            throw "$Description '$fullPath' is a reparse point."
        }
        if ($item.PSIsContainer -or -not (Test-Path -LiteralPath $fullPath -PathType Leaf)) {
            throw "$Description '$fullPath' is not a regular filesystem leaf."
        }
    }
    return $fullPath
}

function Remove-SafePackagePath {
    param(
        [Parameter(Mandatory)]
        [string]$Path,

        [Parameter(Mandatory)]
        [string]$Root,

        [Parameter(Mandatory)]
        [string]$Description
    )

    $fullPath = Assert-PhysicalPackagePathUnderRoot -Path $Path -Root $Root -Description $Description
    $fullRoot = Get-NormalizedPackagePath -Path $Root
    if ($fullPath.Equals($fullRoot, [System.StringComparison]::OrdinalIgnoreCase)) {
        throw "$Description refuses to remove package root '$fullRoot'."
    }
    if (Test-Path -LiteralPath $fullPath) {
        Assert-NoPackageReparseChain -Path $fullPath -Description $Description
        $item = Get-Item -LiteralPath $fullPath -Force
        if ($item.PSIsContainer) {
            foreach ($descendant in @(Get-ChildItem -LiteralPath $fullPath -Recurse -Force)) {
                if (($descendant.Attributes -band [System.IO.FileAttributes]::ReparsePoint) -ne 0) {
                    throw "$Description '$fullPath' contains reparse descendant '$($descendant.FullName)'; refusing recursive removal."
                }
            }
        }
        Remove-Item -LiteralPath $fullPath -Recurse -Force
    }
}
