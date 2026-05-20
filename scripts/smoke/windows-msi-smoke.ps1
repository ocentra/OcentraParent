param(
    [Parameter(Mandatory = $true)]
    [string] $MsiPath
)

$ErrorActionPreference = 'Stop'
$resolvedMsi = (Resolve-Path -LiteralPath $MsiPath).Path
$serviceNames = @('OcentraParentAgent', 'OcentraParentUpdater')
$installed = $false

function Invoke-MsiExec {
    param([string[]] $Arguments)

    $process = Start-Process -FilePath 'msiexec.exe' -ArgumentList $Arguments -Wait -PassThru -WindowStyle Hidden
    if (($process.ExitCode -ne 0) -and ($process.ExitCode -ne 3010)) {
        throw "msiexec failed with exit code $($process.ExitCode): $($Arguments -join ' ')"
    }
}

try {
    Invoke-MsiExec -Arguments @('/i', $resolvedMsi, '/qn', '/norestart')
    $installed = $true

    foreach ($serviceName in $serviceNames) {
        $service = Get-Service -Name $serviceName -ErrorAction Stop
        if ($service.Name -ne $serviceName) {
            throw "Unexpected service identity for $serviceName"
        }
    }
} finally {
    if ($installed) {
        Invoke-MsiExec -Arguments @('/x', $resolvedMsi, '/qn', '/norestart')
    }
}

foreach ($serviceName in $serviceNames) {
    $service = Get-Service -Name $serviceName -ErrorAction SilentlyContinue
    if ($null -ne $service) {
        throw "Service remained after uninstall: $serviceName"
    }
}

Write-Host "windows-msi-smoke-ok:$resolvedMsi"
