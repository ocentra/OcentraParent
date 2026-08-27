$script:TrustedPackageCommandHashes = @{}

function Invoke-CheckedCommand {
    param(
        [Parameter(Mandatory)]
        [string]$Command,

        [Parameter()]
        [string[]]$ArgumentList = @(),

        [Parameter(Mandatory)]
        [string]$FailureMessage
    )

    Assert-TrustedCommandIntegrity -Command $Command
    & $Command @ArgumentList
    if ($LASTEXITCODE -ne 0) {
        throw "$FailureMessage (exit code $LASTEXITCODE)."
    }
}

function Register-TrustedCommand {
    param(
        [Parameter(Mandatory)]
        [string]$Command
    )

    $resolved = Resolve-RequiredCommand -Name $Command
    $script:TrustedPackageCommandHashes[$resolved] = Get-TrustedCommandFingerprint -Path $resolved
    return $resolved
}

function Get-TrustedCommandFingerprint {
    param(
        [Parameter(Mandatory)]
        [string]$Path
    )

    $fullPath = [System.IO.Path]::GetFullPath($Path)
    $item = Get-Item -LiteralPath $fullPath -Force
    $targetPath = $fullPath
    $linkTarget = ''
    if (($item.Attributes -band [System.IO.FileAttributes]::ReparsePoint) -ne 0) {
        if ($item.LinkType -cne 'SymbolicLink' -or @($item.Target).Count -ne 1) {
            throw "Pinned build tool '$fullPath' uses an unsupported reparse form; refusing to invoke it."
        }
        $linkTarget = [string]$item.Target
        $targetPath = [System.IO.Path]::GetFullPath((Join-Path ([System.IO.Path]::GetDirectoryName($fullPath)) $linkTarget))
        if (-not [System.IO.Path]::IsPathRooted($targetPath) -or [System.IO.Path]::GetExtension($targetPath) -cne '.exe') {
            throw "Pinned build tool '$fullPath' points to a non-absolute executable target '$targetPath'; refusing to invoke it."
        }
    }
    if (-not (Test-Path -LiteralPath $targetPath -PathType Leaf)) {
        throw "Pinned build tool target '$targetPath' is missing; refusing to invoke it."
    }
    $targetItem = Get-Item -LiteralPath $targetPath -Force
    if (($targetItem.Attributes -band [System.IO.FileAttributes]::ReparsePoint) -ne 0 -or $targetItem.PSIsContainer) {
        throw "Pinned build tool target '$targetPath' is not a regular non-reparse executable."
    }
    $targetHash = (Get-FileHash -LiteralPath $targetPath -Algorithm SHA256).Hash.ToLowerInvariant()
    return "$fullPath|$linkTarget|$targetPath|$targetHash"
}

function Assert-TrustedCommandIntegrity {
    param(
        [Parameter(Mandatory)]
        [string]$Command
    )

    $fullPath = [System.IO.Path]::GetFullPath($Command)
    if (-not [System.IO.Path]::IsPathRooted($fullPath) -or [System.IO.Path]::GetExtension($fullPath) -cne '.exe') {
        throw "Build tool '$Command' is not an absolute executable path; refusing to invoke an unpinned command."
    }
    if (-not $script:TrustedPackageCommandHashes.ContainsKey($fullPath)) {
        throw "Build tool '$fullPath' was not registered before invocation; refusing an unanchored executable."
    }
    $actualFingerprint = Get-TrustedCommandFingerprint -Path $fullPath
    if ($actualFingerprint -cne $script:TrustedPackageCommandHashes[$fullPath]) {
        throw "Pinned build tool '$fullPath' changed after registration; refusing to invoke a drifted executable."
    }
}

function Resolve-RequiredCommand {
    param(
        [Parameter(Mandatory)]
        [string]$Name
    )

    $command = Get-Command -Name $Name -CommandType Application -ErrorAction SilentlyContinue |
        Where-Object { $_.CommandType -eq 'Application' } |
        Select-Object -First 1
    if ($null -eq $command) {
        throw "Required build tool '$Name' is unavailable; refusing to produce a package."
    }
    $source = [string]$command.Source
    if ([string]::IsNullOrWhiteSpace($source) -or -not [System.IO.Path]::IsPathRooted($source) -or
        [System.IO.Path]::GetExtension($source) -cne '.exe') {
        throw "Required build tool '$Name' did not resolve to an absolute .exe application; refusing to produce a package."
    }
    $fullPath = [System.IO.Path]::GetFullPath($source)
    if (-not (Test-Path -LiteralPath $fullPath -PathType Leaf)) {
        throw "Required build tool '$Name' resolved to missing executable '$fullPath'; refusing to produce a package."
    }
    $item = Get-Item -LiteralPath $fullPath -Force
    if (($item.Attributes -band [System.IO.FileAttributes]::ReparsePoint) -ne 0) {
        # Rustup installs cargo/rustc as a symbolic link. It is admissible only
        # when the link target is a single absolute .exe whose bytes are
        # fingerprinted before every invocation; junctions and other reparse
        # forms remain rejected by Get-TrustedCommandFingerprint.
        Get-TrustedCommandFingerprint -Path $fullPath | Out-Null
    } elseif ($item.PSIsContainer) {
        throw "Required build tool '$Name' resolved to a non-regular path '$fullPath'; refusing to produce a package."
    }
    return $fullPath
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

    # This mirrors protected-capability-custody-core's fixed database identity:
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
    # UUID version 5 / RFC 4122 variant keeps the value valid while making the
    # MSI ProductCode stable for an exact package identity and version.
    $bytes[6] = ($bytes[6] -band 0x0f) -bor 0x50
    $bytes[8] = ($bytes[8] -band 0x3f) -bor 0x80
    return ([Guid]::new([byte[]]$bytes[0..15])).ToString('B').ToUpperInvariant()
}

function Write-Utf8NoBom {
    param(
        [Parameter(Mandatory)]
        [string]$Path,

        [Parameter(Mandatory)]
        [string]$Content,

        [Parameter()]
        [string]$Root
    )

    if (-not [string]::IsNullOrWhiteSpace($Root)) {
        Assert-SafePackageLeafPath -Path $Path -Root $Root -Description 'Package text output' | Out-Null
    }
    $parentPath = [System.IO.Path]::GetDirectoryName([System.IO.Path]::GetFullPath($Path))
    if (-not [string]::IsNullOrWhiteSpace($Root)) {
        Assert-PhysicalPackagePathUnderRoot -Path $parentPath -Root $Root -Description 'Package text output parent' | Out-Null
    }
    $encoding = [System.Text.UTF8Encoding]::new($false)
    $bytes = $encoding.GetBytes($Content)
    if (-not [string]::IsNullOrWhiteSpace($Root)) {
        Assert-SafePackageLeafPath -Path $Path -Root $Root -Description 'Package text output immediately before write' | Out-Null
    }
    $stream = $null
    try {
        $stream = [System.IO.FileStream]::new(
            $Path,
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
}

function Write-DeterministicJson {
    param(
        [Parameter(Mandatory)]
        [string]$Path,

        [Parameter(Mandatory)]
        [object]$Value,

        [Parameter()]
        [string]$Root
    )

    $json = $Value | ConvertTo-Json -Depth 16
    Write-Utf8NoBom -Path $Path -Content ($json + "`n") -Root $Root
}

function Assert-NonEmptyFile {
    param(
        [Parameter(Mandatory)]
        [string]$Path,

        [Parameter(Mandatory)]
        [string]$Description,

        [Parameter()]
        [string]$Root
    )

    if (-not [string]::IsNullOrWhiteSpace($Root)) {
        Assert-SafePackageLeafPath -Path $Path -Root $Root -Description $Description | Out-Null
    }
    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "$Description '$Path' is absent; refusing to produce a package."
    }
    if ((Get-Item -LiteralPath $Path).Length -le 0) {
        throw "$Description '$Path' is empty; refusing to produce a package."
    }
}
