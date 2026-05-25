import { execFileSync, spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdirSync } from 'node:fs';
import { join, resolve } from 'node:path';

import {
  PackageLifecycleProofError,
  WINDOWS_AGENT_SERVICE_ID,
  WINDOWS_UPDATER_SERVICE_ID,
} from './package-lifecycle-artifacts.mjs';

export const DEFAULT_HEALTH_URL = 'http://127.0.0.1:4477/health';

export function readElevationState() {
  if (process.platform !== 'win32') {
    return {
      isElevated: false,
      status: 'unsupported-platform',
    };
  }
  const script = [
    '$identity = [Security.Principal.WindowsIdentity]::GetCurrent()',
    '$principal = [Security.Principal.WindowsPrincipal]::new($identity)',
    '$isAdmin = $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)',
    '[Console]::Out.Write(($isAdmin).ToString().ToLowerInvariant())',
  ].join('; ');
  const output = runPowerShell(['-NoProfile', '-ExecutionPolicy', 'Bypass', '-Command', script], {
    encoding: 'utf8',
  }).trim();
  return {
    isElevated: output === 'true',
    status: output === 'true' ? 'elevated' : 'not-elevated',
  };
}

export function readMsiMetadata(msiPath) {
  if (process.platform !== 'win32') {
    return {
      properties: {},
      status: 'unsupported-platform',
    };
  }
  const script = String.raw`
$installer = New-Object -ComObject WindowsInstaller.Installer
$database = $installer.GetType().InvokeMember('OpenDatabase', 'InvokeMethod', $null, $installer, @($env:OCENTRA_PARENT_MSI_PATH, 0))
$properties = @('ProductName', 'ProductVersion', 'Manufacturer', 'ProductCode', 'UpgradeCode')
$result = [ordered]@{}
foreach ($property in $properties) {
  $query = "SELECT Value FROM Property WHERE Property='$property'"
  $view = $database.GetType().InvokeMember('OpenView', 'InvokeMethod', $null, $database, @($query))
  $view.GetType().InvokeMember('Execute', 'InvokeMethod', $null, $view, $null) | Out-Null
  $record = $view.GetType().InvokeMember('Fetch', 'InvokeMethod', $null, $view, $null)
  $value = if ($null -eq $record) { $null } else { $record.GetType().InvokeMember('StringData', 'GetProperty', $null, $record, @(1)) }
  $result[$property] = $value
}
$result | ConvertTo-Json -Compress
`;
  const output = runPowerShell(['-NoProfile', '-ExecutionPolicy', 'Bypass', '-Command', script], {
    encoding: 'utf8',
    env: {
      ...process.env,
      OCENTRA_PARENT_MSI_PATH: resolve(msiPath),
    },
  });
  return {
    properties: JSON.parse(output),
    status: 'read',
  };
}

export function runInstallLifecycle({ healthUrl = DEFAULT_HEALTH_URL, msiPath, outputDirectory }) {
  const outputRoot = resolve(outputDirectory);
  mkdirSync(outputRoot, { recursive: true });
  const installLogPath = join(outputRoot, 'windows-msi-install.log');
  const uninstallLogPath = join(outputRoot, 'windows-msi-uninstall.log');
  const lifecycle = {
    health: { status: 'not-run', url: healthUrl },
    install: { attempted: false, logPath: installLogPath, status: 'not-run' },
    processCleanup: { status: 'not-run' },
    reboot: { attempted: false, status: 'not-run' },
    services: { afterInstall: [], afterUninstall: [], status: 'not-run' },
    uninstall: { attempted: false, logPath: uninstallLogPath, status: 'not-run' },
  };
  let installed = false;
  try {
    lifecycle.install.attempted = true;
    lifecycle.install.exitCode = runMsiExec(['/i', resolve(msiPath), '/qn', '/norestart', '/L*v', installLogPath]);
    lifecycle.install.status = 'installed';
    installed = true;

    lifecycle.services.afterInstall = ensureServicesRunning();
    lifecycle.services.status = 'running';
    lifecycle.health = readHealth(healthUrl);
  } finally {
    if (installed) {
      lifecycle.uninstall.attempted = true;
      lifecycle.uninstall.exitCode = runMsiExec([
        '/x',
        resolve(msiPath),
        '/qn',
        '/norestart',
        '/L*v',
        uninstallLogPath,
      ]);
      lifecycle.uninstall.status = 'uninstalled';
      lifecycle.services.afterUninstall = readServices();
      lifecycle.processCleanup = readProcessCleanup();
    }
  }
  assertCleanup(lifecycle);
  return lifecycle;
}

function runMsiExec(argumentsList) {
  const result = spawnSync('msiexec.exe', argumentsList, {
    encoding: 'utf8',
    stdio: ['ignore', 'pipe', 'pipe'],
    windowsHide: true,
  });
  if (result.error) {
    throw result.error;
  }
  if (result.status !== 0 && result.status !== 3010) {
    throw new PackageLifecycleProofError('msiexec-failed', `msiexec exited ${result.status}.`);
  }
  return result.status;
}

function ensureServicesRunning() {
  const services = readServices();
  const foundNames = new Set(services.map((service) => service.name));
  for (const serviceName of [WINDOWS_AGENT_SERVICE_ID, WINDOWS_UPDATER_SERVICE_ID]) {
    if (!foundNames.has(serviceName)) {
      throw new PackageLifecycleProofError('service-not-registered', `Service was not registered: ${serviceName}`);
    }
    const service = services.find((entry) => entry.name === serviceName);
    if (service.status !== 'Running') {
      runPowerShell(['-NoProfile', '-ExecutionPolicy', 'Bypass', '-Command', `Start-Service -Name '${serviceName}'`]);
    }
  }
  const runningServices = readServices();
  for (const serviceName of [WINDOWS_AGENT_SERVICE_ID, WINDOWS_UPDATER_SERVICE_ID]) {
    const service = runningServices.find((entry) => entry.name === serviceName);
    if (service?.status !== 'Running') {
      throw new PackageLifecycleProofError('service-not-running', `Service did not reach Running: ${serviceName}`);
    }
  }
  return runningServices;
}

function readServices() {
  const serviceList = [WINDOWS_AGENT_SERVICE_ID, WINDOWS_UPDATER_SERVICE_ID].join("','");
  const script = `Get-Service -Name '${serviceList}' -ErrorAction SilentlyContinue | Select-Object Name,Status,StartType | ConvertTo-Json -Compress`;
  const output = runPowerShell(['-NoProfile', '-ExecutionPolicy', 'Bypass', '-Command', script], {
    encoding: 'utf8',
  }).trim();
  if (output === '') {
    return [];
  }
  const parsed = JSON.parse(output);
  return (Array.isArray(parsed) ? parsed : [parsed]).map((service) => ({
    name: service.Name,
    startType: service.StartType,
    status: service.Status,
  }));
}

function readHealth(healthUrl) {
  let lastError = '';
  for (let attempt = 1; attempt <= 10; attempt += 1) {
    try {
      const output = execFileSync('curl.exe', ['--silent', '--show-error', '--fail', healthUrl], {
        encoding: 'utf8',
        timeout: 15_000,
      });
      return {
        attempts: attempt,
        bodySha256: createTextSha(output),
        status: 'healthy',
        url: healthUrl,
      };
    } catch (error) {
      lastError = error.message;
      execFileSync('powershell', ['-NoProfile', '-Command', 'Start-Sleep -Milliseconds 500']);
    }
  }
  throw new PackageLifecycleProofError('service-health-unavailable', lastError);
}

function readProcessCleanup() {
  const script =
    "Get-Process -Name 'ocentra-parent-agent-service','ocentra-parent-agent-updater' -ErrorAction SilentlyContinue | Select-Object ProcessName,Id | ConvertTo-Json -Compress";
  const output = runPowerShell(['-NoProfile', '-ExecutionPolicy', 'Bypass', '-Command', script], {
    encoding: 'utf8',
  }).trim();
  const remaining = output === '' ? [] : JSON.parse(output);
  return {
    remaining: Array.isArray(remaining) ? remaining : [remaining],
    status: 'checked',
  };
}

function assertCleanup(lifecycle) {
  if (lifecycle.services.afterUninstall.length !== 0) {
    throw new PackageLifecycleProofError(
      'service-remained-after-uninstall',
      'Ocentra Parent services remained after uninstall.'
    );
  }
  if (lifecycle.processCleanup.remaining.length !== 0) {
    throw new PackageLifecycleProofError(
      'process-remained-after-uninstall',
      'Ocentra Parent service processes remained after uninstall.'
    );
  }
}

function runPowerShell(args, options = {}) {
  const result = spawnSync('powershell', args, {
    encoding: options.encoding ?? 'utf8',
    env: options.env ?? process.env,
    stdio: ['ignore', 'pipe', 'pipe'],
    windowsHide: true,
  });
  if (result.error) {
    throw result.error;
  }
  if (result.status !== 0) {
    throw new PackageLifecycleProofError('powershell-failed', (result.stderr || result.stdout).trim());
  }
  return result.stdout;
}

function createTextSha(text) {
  return createHash('sha256').update(text).digest('hex').toUpperCase();
}
