param(
    [Parameter(Mandatory = $true)]
    [string] $MsiPath
)

$ErrorActionPreference = 'Stop'
$resolvedMsi = (Resolve-Path -LiteralPath $MsiPath).Path
$serviceNames = @('OcentraParentAgent', 'OcentraParentUpdater')
$smokeLogRoot = Join-Path (Split-Path -Parent $resolvedMsi) 'smoke'
$installLogPath = Join-Path $smokeLogRoot 'windows-msi-install.log'
$uninstallLogPath = Join-Path $smokeLogRoot 'windows-msi-uninstall.log'
$installed = $false

New-Item -ItemType Directory -Path $smokeLogRoot -Force | Out-Null

function Invoke-MsiExec {
    param(
        [string[]] $Arguments,
        [string] $LogPath
    )

    $process = Start-Process -FilePath 'msiexec.exe' -ArgumentList ($Arguments + @('/L*v', $LogPath)) -Wait -PassThru -WindowStyle Hidden
    if (($process.ExitCode -ne 0) -and ($process.ExitCode -ne 3010)) {
        if (Test-Path -LiteralPath $LogPath) {
            Get-Content -LiteralPath $LogPath -Tail 200 | Write-Host
        }
        throw "msiexec failed with exit code $($process.ExitCode): $($Arguments -join ' ') log=$LogPath"
    }
}

try {
    Invoke-MsiExec -Arguments @('/i', $resolvedMsi, '/qn', '/norestart') -LogPath $installLogPath
    $installed = $true

    foreach ($serviceName in $serviceNames) {
        $service = Get-Service -Name $serviceName -ErrorAction Stop
        if ($service.Name -ne $serviceName) {
            throw "Unexpected service identity for $serviceName"
        }
        if ($service.Status -ne 'Running') {
            throw "Service did not start after install: $serviceName status=$($service.Status)"
        }
    }
} finally {
    if ($installed) {
        Invoke-MsiExec -Arguments @('/x', $resolvedMsi, '/qn', '/norestart') -LogPath $uninstallLogPath
    }
}

foreach ($serviceName in $serviceNames) {
    $service = Get-Service -Name $serviceName -ErrorAction SilentlyContinue
    if ($null -ne $service) {
        throw "Service remained after uninstall: $serviceName"
    }
}

Write-Host "windows-msi-smoke-ok:$resolvedMsi"
