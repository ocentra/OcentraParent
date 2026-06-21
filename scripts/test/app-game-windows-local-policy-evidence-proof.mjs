import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'app-game-windows-local-policy-evidence-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const appGameProofDir = join(
  repoRoot,
  'output',
  'app-game-plan-proof',
  '203-app-game-windows-local-policy-evidence-proof'
);
const proofPath = join(outputDir, 'proof.json');
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });
  await mkdir(appGameProofDir, { recursive: true });

  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/app-game-domain',
      '--',
      'app-game-windows-local-policy-evidence-proof',
    ])
  );
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/app-game-domain']));

  const sourceState = await readWindowsPolicyState();
  const contractModule = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'app-game-domain', 'dist', 'app-game-windows-local-policy-evidence-proof.js')
    ).href
  );
  const readModel = contractModule.createAppGameWindowsLocalPolicyEvidenceProof({
    ...sourceState,
    checkedAt: '2026-06-08T21:35:00.000Z',
  });
  const summary = contractModule.summarizeAppGameWindowsLocalPolicyEvidenceProof(readModel);

  const proof = {
    schemaVersion: 1,
    proofMode,
    checkedAt: 'deterministic-proof-artifact',
    commit: await gitHead(),
    commands: commands.map(redactCommandRecord),
    sourceState,
    readModel,
    summary,
    evidence: {
      contract: 'packages/app-game-domain/src/app-game-windows-local-policy-evidence-proof.ts',
      contractTest: 'packages/app-game-domain/tests/unit/app-game-windows-local-policy-evidence-proof.test.ts',
      appLockerService: 'Get-Service AppIDSvc state reduced to parent-safe service state',
      appLockerPolicy:
        'Get-AppLockerPolicy -Local sampled when available; raw XML/rules/executable paths are not stored',
      appControlPolicy: 'Win32_DeviceGuard sampled when available; raw device policy details are not stored',
    },
    claimsProved: [
      'Windows local AppLocker/App Control policy evidence can be sampled as parent-safe counts and booleans',
      'Raw AppLocker policy XML, executable paths, publisher rules, and private policy details are not stored',
      'Broad app/game blocking remains blocked until enforce proof, allowlist, rollback, audit custody, adapter dispatch, and child delivery proof exist',
    ],
    claimsNotProved: [
      'Windows broad installed-app launch blocking execution',
      'System-app allowlist execution',
      'Rollback execution or audit custody',
      'Adapter dispatch, platform enforcement, provider delivery, or child-device delivery',
      'Raw executable path custody or raw policy XML custody',
    ],
  };

  await writeJson(proofPath, proof);
  await writeJson(join(appGameProofDir, 'proof.json'), proof);
  await writeFile(join(appGameProofDir, '00-source-snapshot.md'), sourceSnapshot(sourceState));
  await writeFile(
    join(appGameProofDir, '10-validation-commands.log'),
    `${commands.map(redactCommandRecord).join('\n\n')}\n`
  );

  console.log('app-game-windows-local-policy-evidence-proof-ok');
  console.log(`evidence=${relativePath(proofPath)}`);
}

async function readWindowsPolicyState() {
  const script = String.raw`
$ErrorActionPreference = "SilentlyContinue"
$service = Get-Service -Name AppIDSvc
$serviceState = "appidsvc-unavailable"
if ($null -ne $service) {
  if ($service.Status -eq "Running") {
    $serviceState = "appidsvc-running"
  } else {
    $serviceState = "appidsvc-stopped"
  }
}

$policyReadable = $false
$appLockerPolicyState = "policy-unavailable"
$appLockerRuleCount = 0
$enforceModeObserved = $false
$auditModeObserved = $false
if ($null -ne (Get-Command Get-AppLockerPolicy -ErrorAction SilentlyContinue)) {
  $xml = Get-AppLockerPolicy -Local -Xml
  if ($null -ne $xml -and $xml.Length -gt 0) {
    $policyReadable = $true
    $appLockerRuleCount = ([regex]::Matches($xml, "<File[A-Za-z]+Rule\\b")).Count
    $enforceModeObserved = $xml -match 'EnforcementMode="Enabled"'
    $auditModeObserved = $xml -match 'EnforcementMode="AuditOnly"'
    if ($appLockerRuleCount -gt 0 -or $enforceModeObserved -or $auditModeObserved) {
      $appLockerPolicyState = "policy-readable"
    } else {
      $appLockerPolicyState = "policy-empty"
    }
  } else {
    $appLockerPolicyState = "policy-empty"
  }
}

$appControlPolicyState = "app-control-unavailable"
$appControlPolicyCount = 0
$appControlEnforcementObserved = $false
$deviceGuard = Get-CimInstance -Namespace root\Microsoft\Windows\DeviceGuard -ClassName Win32_DeviceGuard
if ($null -ne $deviceGuard) {
  $configured = @($deviceGuard.SecurityServicesConfigured)
  $running = @($deviceGuard.SecurityServicesRunning)
  $appControlPolicyCount = $configured.Count + $running.Count
  $appControlEnforcementObserved = $deviceGuard.CodeIntegrityPolicyEnforcementStatus -eq 2
  if ($appControlPolicyCount -gt 0 -or $appControlEnforcementObserved) {
    $appControlPolicyState = "app-control-present"
  } else {
    $appControlPolicyState = "app-control-not-present"
  }
}

[pscustomobject]@{
  serviceState = $serviceState
  appLockerPolicyState = $appLockerPolicyState
  appControlPolicyState = $appControlPolicyState
  appLockerRuleCount = $appLockerRuleCount
  appControlPolicyCount = $appControlPolicyCount
  policyReadable = $policyReadable
  enforceModeObserved = $enforceModeObserved
  auditModeObserved = $auditModeObserved
  appControlEnforcementObserved = $appControlEnforcementObserved
} | ConvertTo-Json -Compress
`;
  const result = await runCommand('powershell', ['-NoProfile', '-ExecutionPolicy', 'Bypass', '-Command', script]);
  return JSON.parse(result.stdout.trim());
}

async function runCommand(command, args, options = {}) {
  const commandLine = [command, ...args].join(' ');
  const result = await new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: ['ignore', 'pipe', 'pipe'], windowsHide: true });
    const stdout = [];
    const stderr = [];
    child.stdout.on('data', (chunk) => stdout.push(String(chunk)));
    child.stderr.on('data', (chunk) => stderr.push(String(chunk)));
    child.once('exit', (code) =>
      resolve({ commandLine, status: code ?? 1, stdout: stdout.join(''), stderr: stderr.join('') })
    );
    child.once('error', reject);
  });
  commands.push(result);
  if (result.status !== 0 && !options.allowFailure) {
    throw new Error(`${commandLine} exited with ${result.status}: ${result.stderr}`);
  }
  return result;
}

async function gitHead() {
  const result = await runCommand('git', ['rev-parse', 'HEAD']);
  return result.stdout.trim();
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}

function sourceSnapshot(sourceState) {
  return [
    '# WP203 Windows local policy evidence proof source snapshot',
    '',
    `- AppIDSvc state: \`${sourceState.serviceState}\``,
    `- AppLocker policy state: \`${sourceState.appLockerPolicyState}\``,
    `- AppLocker rule count: \`${sourceState.appLockerRuleCount}\``,
    `- AppLocker enforce mode observed: \`${sourceState.enforceModeObserved}\``,
    `- AppLocker audit mode observed: \`${sourceState.auditModeObserved}\``,
    `- App Control policy state: \`${sourceState.appControlPolicyState}\``,
    `- App Control policy count: \`${sourceState.appControlPolicyCount}\``,
    `- App Control enforcement observed: \`${sourceState.appControlEnforcementObserved}\``,
    '',
  ].join('\n');
}

function redactCommandRecord(record) {
  const command = record.commandLine.startsWith('powershell ')
    ? 'powershell -NoProfile -ExecutionPolicy Bypass -Command <windows-policy-evidence-script-redacted>'
    : record.commandLine;
  return [command, `exit=${record.status}`, redactOutput(record.stdout), redactOutput(record.stderr)]
    .filter(Boolean)
    .join('\n');
}

function redactOutput(output) {
  return output
    .split(repoRoot)
    .join('<repo-root>')
    .replace(/<File[A-Za-z]+Rule\b[^>]*>/g, '<windows-policy-rule-redacted>');
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
