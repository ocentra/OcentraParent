import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'app-game-broad-blocking-proof-gates');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', '23-broad-blocking-proof-gates');
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', '22-broad-blocking-proof-gates');
const commands = [];

await main();

async function main() {
  await mkdir(testOutputDir, { recursive: true });
  await mkdir(appGameProofDir, { recursive: true });
  await mkdir(appProofDir, { recursive: true });

  await runCommand(...npmCommand(['run', 'build:contracts']));
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/app-game-domain',
      '--',
      'app-game-broad-blocking-proof-gates',
      'app-game-control-platform-authority',
      'app-game-policy-target-compiler',
    ])
  );

  const { AppGameBroadBlockingGateMatrix } =
    await import('../../packages/app-game-domain/dist/app-game-broad-blocking-proof-gate-data.js');
  const summary = summarizeMatrix(AppGameBroadBlockingGateMatrix);
  assertMatrix(AppGameBroadBlockingGateMatrix, summary);

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-game-broad-blocking-proof-gates',
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    commands,
    counts: summary,
    evidence: {
      tsContract: 'packages/app-game-domain/src/app-game-broad-blocking-proof-gates.ts',
      tsContractData: 'packages/app-game-domain/src/app-game-broad-blocking-proof-gate-data.ts',
      tsContractRules: 'packages/app-game-domain/src/app-game-broad-blocking-proof-gate-rules.ts',
      tsContractTest: 'packages/app-game-domain/tests/unit/app-game-broad-blocking-proof-gates.test.ts',
      proofHarness: 'scripts/test/app-game-broad-blocking-proof-gates.mjs',
      appGameProofPack: 'output/app-game-plan-proof/23-broad-blocking-proof-gates',
      appProofPack: 'output/app-plan-proof/22-broad-blocking-proof-gates',
    },
    claimsProved: [
      'manual-required app/game broad blocking gates cannot dispatch adapters',
      'unavailable broad blocking gates cannot dispatch adapters',
      'Windows AppLocker audit-only evidence is not AppLocker enforce proof',
      'platform proof requirements name setup, authority tier, rollback, audit, and platform-specific proof',
      'Android normal mode hide/suspend and iOS process killing remain unclaimed before platform proof',
      'parent-visible manual-required reasons are explicit instead of generic unsupported copy',
    ],
    claimsNotProved: [
      'Windows AppLocker/App Control enforce support',
      'macOS MDM, Endpoint Security, or System Extension hard blocking',
      'Linux cgroup/systemd/AppArmor/SELinux/package blocking',
      'Android Device Owner/Profile Owner hide, suspend, or allowlist execution',
      'iOS FamilyControls/ManagedSettings shield execution or process control',
      'runtime platform adapters, service events, rollback execution, or portal screenshots',
    ],
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeJson(join(appGameProofDir, '03-runtime-evidence.json'), proof);
  await writeJson(join(appProofDir, '03-runtime-evidence.json'), proof);

  console.log(`app-game-broad-blocking-proof-gates-ok:${Object.keys(summary.byOutcomeState).join(',')}`);
  console.log(`evidence=${relative(repoRoot, join(testOutputDir, 'proof.json'))}`);
}

function summarizeMatrix(matrix) {
  return {
    gateCount: matrix.gates.length,
    byPlatform: countBy(matrix.gates.map((gate) => gate.platform)),
    byOutcomeState: countBy(matrix.gates.map((gate) => gate.outcomeState)),
    dispatchEligible: matrix.gates.filter((gate) => gate.adapterDispatchState === 'dispatch-eligible').length,
    adapterCallAllowed: matrix.gates.filter((gate) => gate.canCallAdapter).length,
    broadBlockingClaimed: matrix.gates.filter((gate) => gate.broadBlockingClaimed).length,
  };
}

function assertMatrix(matrix, summary) {
  assertEqual(String(matrix.matrixId), 'app-game-broad-blocking-proof-gates', 'matrix id');
  assertEqual(summary.gateCount, 7, 'gate count');
  assertEqual(summary.byOutcomeState['manual-required'], 5, 'manual-required count');
  assertEqual(summary.byOutcomeState.unavailable, 1, 'unavailable count');
  assertEqual(summary.byOutcomeState['not-claimed'], 1, 'not-claimed count');
  assertEqual(summary.dispatchEligible, 0, 'dispatch eligible count');
  assertEqual(summary.adapterCallAllowed, 0, 'adapter call allowed count');
  assertEqual(summary.broadBlockingClaimed, 0, 'broad blocking claimed count');

  const windowsGate = gateFor(matrix, 'windows-block-launch-applocker-app-control-manual-required');
  assertIncludes(windowsGate.requiredProofKinds, 'windows-applocker-proof', 'Windows AppLocker proof requirement');
  assertIncludes(windowsGate.requiredProofKinds, 'windows-app-control-proof', 'Windows App Control proof requirement');
  assertIncludes(windowsGate.requiredProofKinds, 'rollback-proof', 'rollback proof requirement');
  assertIncludes(windowsGate.requiredProofKinds, 'audit-state-proof', 'audit state proof requirement');

  const androidGate = gateFor(matrix, 'android-normal-mode-hide-suspend-manual-required');
  assertEqual(androidGate.canCallAdapter, false, 'Android normal mode adapter call');

  const iosGate = gateFor(matrix, 'ios-process-kill-not-claimed');
  assertEqual(iosGate.outcomeState, 'not-claimed', 'iOS process kill outcome');
}

function gateFor(matrix, gateId) {
  const gate = matrix.gates.find((candidate) => String(candidate.gateId) === gateId);
  if (gate === undefined) {
    throw new Error(`missing broad blocking gate ${gateId}`);
  }
  return gate;
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}

async function runCommand(command, args) {
  commands.push([command, ...args].join(' '));
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) => (code === 0 ? resolve() : reject(new Error(`${command} exited with ${code}`))));
    child.once('error', reject);
  });
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

function countBy(values) {
  return values.reduce((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}

function assertEqual(actual, expected, label) {
  if (actual !== expected) {
    throw new Error(`${label}: expected ${expected}, received ${actual}`);
  }
}

function assertIncludes(values, expected, label) {
  if (!values.includes(expected)) {
    throw new Error(`${label}: missing ${expected}`);
  }
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
