import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, readdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'v0-8-os-adapter-proof-hardening');
const proofPath = join(outputDir, 'proof.json');
const proofCommand = 'node scripts/test/v0-8-os-adapter-proof-hardening.mjs';
const commands = [];
const proofLabels = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });

  await runCommand(...npmCommand(['run', 'build:contracts']));
  await runCommand('cmd', ['/c', 'node', 'scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs']);
  const appTimeLimit = await latestJson(join(repoRoot, 'test-results', 'v0-8-windows-app-time-limit-adapter-mvp'));
  assertAppTimeLimit(appTimeLimit.data);

  await runCommand('cmd', ['/c', 'node', 'scripts/test/v0-8-production-enforcement-hardening.mjs']);
  const productionHardening = await latestJson(join(repoRoot, 'test-results', 'v0-8-production-enforcement-hardening'));
  assertProductionHardening(productionHardening.data);

  await runCommand('cmd', ['/c', 'node', 'scripts/test/windows-managed-unmanaged-browser-enforcement-proof.mjs']);
  const browserBoundary = await latestJson(
    join(repoRoot, 'test-results', 'windows-managed-unmanaged-browser-enforcement-proof')
  );
  assertBrowserBoundary(browserBoundary.data);

  const capabilities = await platformCapabilities();
  assertWindowsCapabilities(capabilities);
  const matrix = await readJson(join(repoRoot, 'docs', 'expectations', 'pre-ai-proof-matrix.json'));
  assertProofMatrix(matrix);

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    commands,
    proofLabels,
    evidence: {
      appTimeLimit: relative(repoRoot, appTimeLimit.path),
      productionHardening: relative(repoRoot, productionHardening.path),
      browserBoundary: relative(repoRoot, browserBoundary.path),
      proofMatrix: relative(repoRoot, join(repoRoot, 'docs', 'expectations', 'pre-ai-proof-matrix.json')),
    },
    osAdapterTruth: {
      ownedProcessTerminate: assertionById(productionHardening.data, 'process-terminate-owned-process'),
      appTimeLimitLifecycle: appTimeLimit.data.assertions,
      broadAdapterStates: productionHardening.data.assertions.filter((assertion) =>
        ['app-block-process-control', 'domain-block-network-control', 'site-block-managed-browser-control'].includes(
          assertion.id
        )
      ),
      browserBoundary: browserBoundary.data.states,
      unsupportedClaims: [
        'broad app blocking is manual-required or unavailable outside owned-process proof',
        'network/domain blocking is manual-required or unavailable until an OS-approved adapter proves it',
        'managed-browser service commands do not prove exact URL enforcement by themselves',
        'unmanaged-browser process control does not prove exact URL, tab, title, download source, or page content',
      ],
    },
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`v0-8-os-adapter-proof-hardening-ok:${proofLabels.join(',')}`);
  console.log(`evidence=${proofPath}`);
}

function assertAppTimeLimit(evidence) {
  assertEqual(evidence.serviceScope?.timeLimitCreateRecoverCancelExpireProven, true, 'time-limit lifecycle');
  assertEqual(evidence.serviceScope?.expiryAdapterReachedThroughService, true, 'expiry adapter service path');
  assertEqual(evidence.assertions?.recover?.recoveredAfterRestart, true, 'restart recovery');
  assertEqual(evidence.assertions?.cancel?.auditEventKind, 'cancelled', 'parent cancel audit');
  assertEqual(evidence.assertions?.cancel?.stateCleared, true, 'parent cancel clears state');
  assertEqual(evidence.assertions?.expire?.stateCleared, true, 'expiry clears state');
  assertOneOf(evidence.assertions?.expire?.status, ['expired', 'unavailable'], 'expiry status');
  proofLabels.push('v0.8.app-time-limit.restart-cancel-expire-audit-proof');
}

function assertProductionHardening(evidence) {
  assertEqual(evidence.serviceScope?.manualRequiredStatesProvenThroughService, true, 'manual-required service states');
  assertEqual(evidence.serviceScope?.unsupportedBlockingClaimsRejected, true, 'unsupported claims rejected');
  assertEqual(evidence.serviceScope?.auditStoragePathProven, true, 'audit storage path');
  assertOneOf(
    evidence.serviceScope?.processTerminateServiceProof,
    ['actually-enforced', 'unsupported-platform'],
    'process terminate service proof'
  );
  const expected = [
    'process-terminate-owned-process',
    'app-block-process-control',
    'domain-block-network-control',
    'site-block-managed-browser-control',
  ];
  for (const id of expected) {
    assertionById(evidence, id);
  }
  for (const assertion of evidence.assertions) {
    if (assertion.id === 'process-terminate-owned-process') {
      assertOneOf(assertion.status, ['actually-enforced', 'unavailable'], 'process terminate status');
    } else {
      assertEqual(assertion.status, 'unavailable', `${assertion.id} status`);
      assertOneOf(assertion.capabilityState, ['manual-required', 'unavailable'], `${assertion.id} capability`);
    }
  }
  proofLabels.push('v0.8.production-hardening.manual-required-service-boundaries');
}

function assertBrowserBoundary(evidence) {
  const states = evidence.states ?? {};
  assertEqual(states.processIdRequiredRejection, 'rejected', 'process id required rejection');
  assertEqual(states.windowsProcessAdapterGuard, 'rejected-without-termination', 'pid/name guard');
  assertOneOf(states.windowsProcessAdapterRuntime, ['terminated', 'already-exited'], 'owned process runtime');
  assertEqual(states.broadAppBlockingCapability, 'manual-required', 'broad app blocking boundary');
  assertEqual(states.managedBrowserServiceCommand, 'manual-required', 'managed browser service command');
  assertEqual(states.exactUnmanagedUrlClaim, 'not-claimed', 'unmanaged exact URL non-claim');
  assertEqual(
    states.exactManagedBrowserServiceCommandUrlClaim,
    'not-claimed-service-command-manual-required',
    'managed service command exact URL non-claim'
  );
  const unmanaged = assertionById(evidence, 'unmanaged-browser-terminate');
  assertOneOf(unmanaged.state, ['terminated', 'manual-required'], 'unmanaged browser terminate boundary');
  assertEqual(unmanaged.exactUrlClaimState, 'not-claimed', 'unmanaged exact URL claim');
  assertEqual(assertionById(evidence, 'unmanaged-browser-warn').state, 'warned', 'unmanaged warn boundary');
  proofLabels.push('v0.8.browser-boundary.pid-name-unmanaged-managed-nonclaim-proof');
}

function assertWindowsCapabilities(capabilities) {
  assertCapability(capabilities, 'windows', 'owned-process-terminate', 'implemented');
  assertCapability(capabilities, 'windows', 'app-time-limit', 'implemented');
  assertCapability(capabilities, 'windows', 'app-blocking', 'manual-required');
  assertCapability(capabilities, 'windows', 'network-domain-blocking', 'manual-required');
  assertCapability(capabilities, 'windows', 'managed-browser-control', 'implemented');
  assertCapability(capabilities, 'windows', 'unmanaged-browser-detection', 'implemented');
  proofLabels.push('v0.8.windows-capability-specific-os-adapter-states');
}

function assertProofMatrix(matrix) {
  if (!matrix.requiredCompletedClaimIds.includes('v0-8-os-adapter-proof-hardening')) {
    throw new Error('Proof matrix required claims are missing v0-8-os-adapter-proof-hardening.');
  }
  const claim = matrix.claims.find((candidate) => candidate.id === 'v0-8-os-adapter-proof-hardening');
  if (!claim) {
    throw new Error('Proof matrix is missing v0-8-os-adapter-proof-hardening claim.');
  }
  assertEqual(claim.platformCoverage.windows, 'real-local-windows-proof', 'V0.8 hardening Windows coverage');
  assertArrayIncludes(claim.ciProof.commands, proofCommand, 'V0.8 hardening claim command');
  const scenario = matrix.checkpointScenarios.find((candidate) => candidate.id === 'v0-8-os-adapter-proof-hardening');
  if (!scenario) {
    throw new Error('Proof matrix is missing v0-8-os-adapter-proof-hardening checkpoint scenario.');
  }
  assertArrayIncludes(scenario.ciCommands, proofCommand, 'V0.8 hardening scenario command');
  proofLabels.push('proof-matrix.v0-8-os-adapter-proof-hardening');
}

function assertionById(evidence, id) {
  const assertion = evidence.assertions?.find((candidate) => candidate.id === id);
  if (!assertion) {
    throw new Error(`Missing assertion: ${id}`);
  }
  return assertion;
}

function assertCapability(capabilities, platform, capabilityName, expectedStatus) {
  const entry = capabilities.find((candidate) => candidate.platform === platform);
  const capability = entry?.capabilities.find((candidate) => candidate.capability === capabilityName);
  assertEqual(capability?.status, expectedStatus, `${platform}.${capabilityName}`);
}

async function platformCapabilities() {
  const modulePath = join(repoRoot, 'packages', 'parent-domain', 'dist', 'capabilities.js');
  if (!existsSync(modulePath)) {
    throw new Error(`Missing built capabilities module: ${modulePath}`);
  }
  const module = await import(`file:///${modulePath.replaceAll('\\', '/')}`);
  return module.ParentControlPlatformCapabilities;
}

async function runCommand(command, args) {
  commands.push([command, ...args].join(' '));
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) =>
      code === 0 ? resolve() : reject(new Error(`${command} ${args.join(' ')} exited with ${code}`))
    );
    child.once('error', reject);
  });
}

async function latestJson(directory) {
  const entries = await readdir(directory, { withFileTypes: true });
  const jsonFiles = [];
  for (const entry of entries) {
    if (entry.isFile() && entry.name.endsWith('.json')) {
      const path = join(directory, entry.name);
      jsonFiles.push({ path, data: JSON.parse(await readFile(path, 'utf8')) });
    }
  }
  if (jsonFiles.length === 0) {
    throw new Error(`No JSON evidence files found in ${directory}`);
  }
  jsonFiles.sort((left, right) => left.path.localeCompare(right.path));
  return jsonFiles.at(-1);
}

async function readJson(path) {
  if (!existsSync(path)) {
    throw new Error(`Missing proof artifact: ${path}`);
  }
  return JSON.parse(await readFile(path, 'utf8'));
}

async function gitHead() {
  const chunks = [];
  await new Promise((resolve, reject) => {
    const child = spawn('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, stdio: ['ignore', 'pipe', 'pipe'] });
    child.stdout.on('data', (chunk) => chunks.push(String(chunk)));
    child.once('exit', (code) => (code === 0 ? resolve() : reject(new Error('git rev-parse HEAD failed'))));
    child.once('error', reject);
  });
  return chunks.join('').trim();
}

function assertArrayIncludes(values, expected, label) {
  if (!Array.isArray(values) || !values.includes(expected)) {
    throw new Error(`${label}: missing ${expected}`);
  }
}

function assertEqual(actual, expected, label) {
  if (actual !== expected) {
    throw new Error(`${label}: expected ${expected}, received ${actual}`);
  }
}

function assertOneOf(actual, expectedValues, label) {
  if (!expectedValues.includes(actual)) {
    throw new Error(`${label}: expected one of ${expectedValues.join(', ')}, received ${actual}`);
  }
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
