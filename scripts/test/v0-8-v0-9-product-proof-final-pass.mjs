import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, readdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'v0-8-v0-9-product-proof-final-pass');
const proofPath = join(outputDir, 'proof.json');

const commands = [];
const proofLabels = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });

  await runCommand('cmd', ['/c', 'npm', 'run', 'build:contracts']);
  await runCommand('cargo', [
    'test',
    '-p',
    'ocentra-parent-agent-service',
    'lan_pairing_status_reports_stale_and_offline_selected_device_state',
  ]);
  await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-service', 'controller_lease']);

  await runCommand('cmd', ['/c', 'node', 'scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs']);
  const v08Evidence = await latestJson(join(repoRoot, 'test-results', 'v0-8-windows-app-time-limit-adapter-mvp'));
  assertV08Evidence(v08Evidence.data);

  await runCommand('cargo', ['build', '-p', 'ocentra-parent-agent-service']);
  await runCommand('cmd', ['/c', 'node', 'scripts/test/v0-9-lan-pairing-control-mvp.mjs']);
  const v09Evidence = await readJson(join(repoRoot, 'test-results', 'v0-9-lan-pairing-control-mvp', 'proof.json'));
  assertV09Evidence(v09Evidence);

  await runCommand('cmd', ['/c', 'node', 'scripts/test/platform-roles-lan-ai-provider-pool.mjs']);
  const platformEvidence = await readJson(
    join(repoRoot, 'test-results', 'platform-roles-lan-ai-provider-pool', 'proof.json')
  );
  assertPlatformEvidence(platformEvidence);

  const matrix = await readJson(join(repoRoot, 'docs', 'expectations', 'pre-ai-proof-matrix.json'));
  assertProofMatrix(matrix);

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    commands,
    proofLabels,
    evidence: {
      v08AppTimeLimit: relative(repoRoot, v08Evidence.path),
      v09LanPairingControl: relative(
        repoRoot,
        join(repoRoot, 'test-results', 'v0-9-lan-pairing-control-mvp', 'proof.json')
      ),
      platformRolesLanAiProviderPool: relative(
        repoRoot,
        join(repoRoot, 'test-results', 'platform-roles-lan-ai-provider-pool', 'proof.json')
      ),
    },
    honestBoundaries: [
      'V0.8 proves the owned local process time-limit adapter path and unavailable states; it does not claim global OS blocking.',
      'V0.9 proves direct WebSocket multi-service behavior; it does not claim production discovery or household router proof.',
      'Parent mobile, Android child, iOS child, signing, store distribution, device-owner, and Family Controls remain manual-required or unavailable.',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`v0-8-v0-9-product-proof-final-pass-ok:${proofLabels.join(',')}`);
  console.log(`evidence=${proofPath}`);
}

async function runCommand(command, args) {
  commands.push([command, ...args].join(' '));
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) => {
      if (code === 0) {
        resolve();
        return;
      }
      reject(new Error(`${command} ${args.join(' ')} exited with ${code}`));
    });
    child.once('error', reject);
  });
}

async function latestJson(directory) {
  const entries = await readdir(directory, { withFileTypes: true });
  const jsonFiles = [];
  for (const entry of entries) {
    if (entry.isFile() && entry.name.endsWith('.json')) {
      const path = join(directory, entry.name);
      const text = await readFile(path, 'utf8');
      jsonFiles.push({ path, data: JSON.parse(text) });
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

function assertV08Evidence(evidence) {
  assertEqual(evidence.serviceScope?.timeLimitCreateRecoverCancelExpireProven, true, 'V0.8 full timer lifecycle');
  assertEqual(evidence.serviceScope?.expiryAdapterReachedThroughService, true, 'V0.8 expiry adapter service path');
  assertEqual(evidence.assertions?.execute?.timerEventKind, 'created', 'V0.8 execute timer kind');
  assertEqual(evidence.assertions?.execute?.status, 'no-op', 'V0.8 execute status');
  assertEqual(evidence.assertions?.recover?.timerEventKind, 'restart-recovered', 'V0.8 restart recovery');
  assertEqual(evidence.assertions?.recover?.recoveredAfterRestart, true, 'V0.8 restart flag');
  assertEqual(evidence.assertions?.cancel?.auditEventKind, 'cancelled', 'V0.8 parent cancel audit');
  assertEqual(evidence.assertions?.cancel?.stateCleared, true, 'V0.8 cancel clears state');
  assertEqual(
    evidence.assertions?.unavailable?.reason,
    'enforcement-active-timer-state-required',
    'V0.8 missing timer unavailable reason'
  );
  assertEqual(evidence.assertions?.expire?.stateCleared, true, 'V0.8 expire clears state');
  assertOneOf(evidence.assertions?.expire?.status, ['expired', 'unavailable'], 'V0.8 expire status');
  if (evidence.assertions.expire.status === 'expired') {
    assertOneOf(
      evidence.assertions.expire.adapterResultCode,
      ['process-terminated', 'process-already-exited'],
      'V0.8 Windows adapter result'
    );
    proofLabels.push('v0.8.enforcement.owned-process-expiry-proven');
  } else {
    assertEqual(
      evidence.assertions.expire.adapterResultCode,
      'unsupported-platform',
      'V0.8 non-Windows unavailable adapter result'
    );
    proofLabels.push('v0.8.enforcement.non-windows-unavailable-proven');
  }
  proofLabels.push('v0.8.enforcement.restart-recovery-proven');
  proofLabels.push('v0.8.enforcement.parent-cancel-override-proven');
  proofLabels.push('v0.8.enforcement.audit-and-storage-proven');
}

function assertV09Evidence(evidence) {
  const assertions = new Set(evidence.assertions);
  for (const label of [
    'wrong-origin-websocket-rejected-before-upgrade',
    'wrong-agent-port-rejected-as-wrong-device',
    'first-child-agent:controller-lease-takeover-denied',
    'second-child-agent:controller-lease-takeover-accepted',
    'second-child-agent:restart-restores-trusted-unselected',
    'second-child-agent:restart-unselected-control-rejected',
    'second-child-agent:restart-route-reselected',
    'first-child-agent:stale-control-rejected',
    'first-child-agent:wrong-controller-rejected',
    'first-child-agent:lan-ai-provider-advertised',
    'first-child-agent:lan-ai-job-degraded',
    'first-child-agent:observer-lan-ai-job-rejected',
  ]) {
    assertSetHas(assertions, label, 'V0.9 LAN control proof label');
  }
  proofLabels.push('v0.9.lan.controller-conflict-and-takeover-proven');
  proofLabels.push('v0.9.lan.registry-restart-persistence-proven');
  proofLabels.push('v0.9.lan.wrong-origin-and-wrong-device-rejection-proven');
  proofLabels.push('v0.9.lan.degraded-provider-state-proven');
}

function assertPlatformEvidence(evidence) {
  const assertions = new Set(evidence.assertions);
  for (const label of [
    'parent-desktop-controller-ai-provider:provider-advertised-available',
    'parent-desktop-controller-ai-provider:controller-job-completed-observer-job-rejected',
    'parent-desktop-controller-ai-provider:unsupported-capability-rejected',
    'parent-mobile-observer-scaffold:provider-unavailable',
    'parent-mobile-observer-scaffold:controller-job-degraded-with-provider-unavailable',
    'parent-mobile-observer-scaffold:observer-job-rejected',
  ]) {
    assertSetHas(assertions, label, 'Platform role LAN AI proof label');
  }
  proofLabels.push('v0.9.lan.provider-selection-available-rejected-degraded-proven');
  proofLabels.push('platform.parent-mobile-scaffold-unavailable-state-proven');
}

function assertProofMatrix(matrix) {
  const claim = matrix.claims.find((candidate) => candidate.id === 'v08-v09-product-proof-final-pass');
  if (!claim) {
    throw new Error('Proof matrix is missing v08-v09-product-proof-final-pass claim.');
  }
  assertEqual(
    claim.platformCoverage.windows,
    'real-local-windows-proof',
    'Final pass Windows coverage keeps local proof explicit'
  );
  assertEqual(claim.platformCoverage.android, 'manual-required', 'Final pass Android coverage stays manual');
  assertEqual(claim.platformCoverage.ios, 'manual-required', 'Final pass iOS coverage stays manual');
  const scenario = matrix.checkpointScenarios.find((candidate) => candidate.id === 'v08-v09-product-proof-final-pass');
  if (!scenario) {
    throw new Error('Proof matrix is missing v08-v09-product-proof-final-pass checkpoint scenario.');
  }
  assertSetHas(
    new Set(scenario.ciCommands),
    'node scripts/test/v0-8-v0-9-product-proof-final-pass.mjs',
    'Final pass proof command is matrix-listed'
  );
  proofLabels.push('proof-matrix.final-pass-honest-platform-states-proven');
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

function assertSetHas(set, value, label) {
  if (!set.has(value)) {
    throw new Error(`${label}: missing ${value}`);
  }
}
