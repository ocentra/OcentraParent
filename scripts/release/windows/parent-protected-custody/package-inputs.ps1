function Invoke-CheckedCommand {
    param(
        [Parameter(Mandatory)]
        [string]$Command,

        [Parameter()]
        [string[]]$ArgumentList = @(),

        [Parameter(Mandatory)]
        [string]$FailureMessage
    )

    & $Command @ArgumentList
    if ($LASTEXITCODE -ne 0) {
        throw "$FailureMessage (exit code $LASTEXITCODE)."
    }
}

function Resolve-RequiredCommand {
    param(
        [Parameter(Mandatory)]
        [string]$Name
    )

    $command = Get-Command -Name $Name -ErrorAction SilentlyContinue
    if ($null -eq $command) {
        throw "Required build tool '$Name' is unavailable; refusing to produce a package."
    }
    return $command.Source
}

function Resolve-UnderRoot {
    param(
        [Parameter(Mandatory)]
        [string]$Path,

        [Parameter(Mandatory)]
        [string]$Root,

        [Parameter(Mandatory)]
        [string]$Description
    )

    $resolvedRoot = [System.IO.Path]::GetFullPath($Root).TrimEnd('\') + '\'
    $resolvedPath = [System.IO.Path]::GetFullPath($Path).TrimEnd('\')
    if (-not $resolvedPath.StartsWith($resolvedRoot, [System.StringComparison]::OrdinalIgnoreCase)) {
        throw "$Description '$resolvedPath' must remain under '$($resolvedRoot.TrimEnd('\'))'."
    }
    return $resolvedPath
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
        [string]$Content
    )

    [System.IO.File]::WriteAllText($Path, $Content, [System.Text.UTF8Encoding]::new($false))
}

function Write-DeterministicJson {
    param(
        [Parameter(Mandatory)]
        [string]$Path,

        [Parameter(Mandatory)]
        [object]$Value
    )

    $json = $Value | ConvertTo-Json -Depth 16
    Write-Utf8NoBom -Path $Path -Content ($json + "`n")
}

function Remove-ExactPath {
    param(
        [Parameter(Mandatory)]
        [string]$Path
    )

    if (Test-Path -LiteralPath $Path) {
        Remove-Item -LiteralPath $Path -Recurse -Force
    }
}

function Assert-NonEmptyFile {
    param(
        [Parameter(Mandatory)]
        [string]$Path,

        [Parameter(Mandatory)]
        [string]$Description
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "$Description '$Path' is absent; refusing to produce a package."
    }
    if ((Get-Item -LiteralPath $Path).Length -le 0) {
        throw "$Description '$Path' is empty; refusing to produce a package."
    }
}
