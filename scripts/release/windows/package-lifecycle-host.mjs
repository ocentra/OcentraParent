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
    lifecycleStates: {
      restart: { attempted: false, status: 'not-run' },
      start: { attempted: false, status: 'not-run' },
      stop: { attempted: false, status: 'not-run' },
    },
    processCleanup: { status: 'not-run' },
    reboot: { attempted: false, status: 'not-run' },
    respawn: { services: [], status: 'not-run' },
    services: { afterInstall: [], afterUninstall: [], status: 'not-run' },
    uninstallAuthorityCleanup: { status: 'not-run' },
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
    lifecycle.lifecycleStates.stop = stopServices({ healthUrl });
    lifecycle.lifecycleStates.start = startServices({ healthUrl });
    lifecycle.lifecycleStates.restart = restartServices({ healthUrl });
    lifecycle.respawn = readRespawnState();
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
      lifecycle.uninstallAuthorityCleanup = readAuthorityCleanupState();
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

function stopServices({ healthUrl }) {
  runPowerShell([
    '-NoProfile',
    '-ExecutionPolicy',
    'Bypass',
    '-Command',
    `Stop-Service -Name '${WINDOWS_AGENT_SERVICE_ID}','${WINDOWS_UPDATER_SERVICE_ID}' -Force -ErrorAction Stop`,
  ]);
  const stoppedServices = waitForServicesStatus('Stopped');
  return {
    attempted: true,
    health: readHealthUnavailable(healthUrl),
    services: stoppedServices,
    status: 'stopped',
  };
}

function startServices({ healthUrl }) {
  runPowerShell([
    '-NoProfile',
    '-ExecutionPolicy',
    'Bypass',
    '-Command',
    `Start-Service -Name '${WINDOWS_AGENT_SERVICE_ID}','${WINDOWS_UPDATER_SERVICE_ID}' -ErrorAction Stop`,
  ]);
  const startedServices = waitForServicesStatus('Running');
  return {
    attempted: true,
    health: readHealth(healthUrl),
    services: startedServices,
    status: 'running',
  };
}

function restartServices({ healthUrl }) {
  runPowerShell([
    '-NoProfile',
    '-ExecutionPolicy',
    'Bypass',
    '-Command',
    `Restart-Service -Name '${WINDOWS_AGENT_SERVICE_ID}','${WINDOWS_UPDATER_SERVICE_ID}' -Force -ErrorAction Stop`,
  ]);
  const restartedServices = waitForServicesStatus('Running');
  return {
    attempted: true,
    health: readHealth(healthUrl),
    services: restartedServices,
    status: 'running',
  };
}

function waitForServicesStatus(expectedStatus) {
  for (let attempt = 1; attempt <= 20; attempt += 1) {
    const services = readServices();
    const allMatched = services.length === 2 && services.every((service) => service.status === expectedStatus);
    if (allMatched) {
      return services;
    }
    sleepMilliseconds(500);
  }
  throw new PackageLifecycleProofError(
    'service-status-timeout',
    `Services did not reach ${expectedStatus} within the expected timeout.`
  );
}

function readRespawnState() {
  return {
    noClaim:
      'Manual stop does not prove crash recovery, and this harness does not force a crash or reboot. Respawn is proved only from installed Windows service-manager failure actions.',
    services: [WINDOWS_AGENT_SERVICE_ID, WINDOWS_UPDATER_SERVICE_ID].map(readServiceManagerRespawnProof),
    status: 'service-manager-checked',
  };
}

function readServiceManagerRespawnProof(serviceName) {
  const failureActions = parseServiceFailureActions(runSc(['qfailure', serviceName], { encoding: 'utf8' }));
  const failureFlag = parseServiceFailureFlag(runSc(['qfailureflag', serviceName], { encoding: 'utf8' }));
  const restartActions = failureActions.actions.filter((action) => action.type === 'restart');
  return {
    actions: failureActions.actions,
    failureActionsFlagEnabled: failureFlag.enabled,
    name: serviceName,
    noClaim:
      'Configured failure actions prove service-manager restart policy only. Manual stop is expected to leave the service stopped, and crash-loop execution is not exercised here.',
    resetPeriodSeconds: failureActions.resetPeriodSeconds,
    respawnState: restartActions.length > 0 ? 'proved' : 'manual-required',
    status: restartActions.length > 0 ? 'configured' : 'unconfigured',
  };
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

function readHealthUnavailable(healthUrl) {
  let lastBody = '';
  for (let attempt = 1; attempt <= 10; attempt += 1) {
    try {
      lastBody = execFileSync('curl.exe', ['--silent', '--show-error', '--fail', healthUrl], {
        encoding: 'utf8',
        timeout: 15_000,
      });
      sleepMilliseconds(500);
    } catch (error) {
      return {
        attempts: attempt,
        reason: error.message,
        status: 'unavailable',
        url: healthUrl,
      };
    }
  }
  throw new PackageLifecycleProofError(
    'service-health-remained-available',
    `Health endpoint remained available after the services were stopped. Last body SHA-256: ${createTextSha(lastBody)}`
  );
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

function readAuthorityCleanupState() {
  const script = String.raw`
$installPath = Join-Path ${'$env:ProgramFiles'} 'Ocentra\Ocentra Parent Agent'
$dataPath = Join-Path ${'$env:ProgramData'} 'Ocentra\Ocentra Parent Agent'
$registryPath = 'HKLM:\Software\Ocentra\Ocentra Parent Agent'
[ordered]@{
  installPath = [ordered]@{ path = $installPath; exists = (Test-Path -LiteralPath $installPath) }
  dataPath = [ordered]@{ path = $dataPath; exists = (Test-Path -LiteralPath $dataPath) }
  registryPath = [ordered]@{ path = $registryPath; exists = (Test-Path -LiteralPath $registryPath) }
} | ConvertTo-Json -Compress
`;
  const output = runPowerShell(['-NoProfile', '-ExecutionPolicy', 'Bypass', '-Command', script], {
    encoding: 'utf8',
  });
  const parsed = JSON.parse(output);
  return {
    dataPath: parsed.dataPath,
    installPath: parsed.installPath,
    registryPath: parsed.registryPath,
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
  if (lifecycle.uninstallAuthorityCleanup.installPath.exists) {
    throw new PackageLifecycleProofError(
      'install-directory-remained-after-uninstall',
      'The child Windows install directory remained after uninstall.'
    );
  }
  if (lifecycle.uninstallAuthorityCleanup.dataPath.exists) {
    throw new PackageLifecycleProofError(
      'programdata-remained-after-uninstall',
      'The child Windows ProgramData directory remained after uninstall.'
    );
  }
  if (lifecycle.uninstallAuthorityCleanup.registryPath.exists) {
    throw new PackageLifecycleProofError(
      'authority-registry-remained-after-uninstall',
      'The child Windows authority registry marker remained after uninstall.'
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

function runSc(args, options = {}) {
  const result = spawnSync('sc.exe', args, {
    encoding: options.encoding ?? 'utf8',
    stdio: ['ignore', 'pipe', 'pipe'],
    windowsHide: true,
  });
  if (result.error) {
    throw result.error;
  }
  if (result.status !== 0) {
    throw new PackageLifecycleProofError('sc-failed', (result.stderr || result.stdout).trim());
  }
  return result.stdout;
}

export function parseServiceFailureActions(output) {
  const text = String(output ?? '');
  const resetPeriodMatch = /RESET_PERIOD \(in seconds\)\s*:\s*([A-Z0-9]+)/u.exec(text);
  const resetValue = resetPeriodMatch?.[1] ?? '';
  const resetPeriodSeconds = /^\d+$/u.test(resetValue) ? Number(resetValue) : null;
  const actionLines = [];
  let captureActions = false;
  for (const rawLine of text.split(/\r?\n/u)) {
    const line = rawLine.trim();
    if (line.startsWith('FAILURE_ACTIONS')) {
      captureActions = true;
      const [, firstAction = ''] = line.split(':', 2);
      if (firstAction.trim().length > 0) {
        actionLines.push(firstAction.trim());
      }
      continue;
    }
    if (!captureActions) {
      continue;
    }
    if (line.length === 0 || line.includes(':')) {
      break;
    }
    actionLines.push(line);
  }
  return {
    actions: actionLines.map(parseServiceFailureActionLine),
    resetPeriodSeconds,
  };
}

export function parseServiceFailureFlag(output) {
  const match = /FAILURE_ACTIONS_FLAG\s*:\s*(\d+)/u.exec(String(output ?? ''));
  if (!match) {
    throw new PackageLifecycleProofError(
      'failure-actions-flag-missing',
      'The Windows service-manager failure-actions flag was not present in sc.exe output.'
    );
  }
  return {
    enabled: match[1] === '1',
  };
}

function parseServiceFailureActionLine(line) {
  const match = /^([A-Z_]+)\s+--\s+Delay\s*=\s*(\d+)\s+milliseconds\.?$/iu.exec(line);
  if (!match) {
    throw new PackageLifecycleProofError(
      'failure-action-line-invalid',
      `Unrecognized Windows service failure action line: ${line}`
    );
  }
  return {
    delayMilliseconds: Number(match[2]),
    type: normalizeServiceActionType(match[1]),
  };
}

function normalizeServiceActionType(value) {
  const normalized = value.toLowerCase();
  if (normalized === 'restart' || normalized === 'reboot' || normalized === 'run_command' || normalized === 'none') {
    return normalized;
  }
  return normalized;
}

function sleepMilliseconds(milliseconds) {
  execFileSync('powershell', ['-NoProfile', '-Command', `Start-Sleep -Milliseconds ${milliseconds}`], {
    stdio: 'ignore',
    windowsHide: true,
  });
}

function createTextSha(text) {
  return createHash('sha256').update(text).digest('hex').toUpperCase();
}
