import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'app-game-android-usage-events-replay-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', '188-app-game-android-usage-events-replay');
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
      'app-game-android-physical-device-proof',
      'app-game-android-usage-events-replay',
      'app-game-platform-proof-status',
    ])
  );
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/schema-domain']));
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/app-game-domain']));

  const androidProof = await readJson(
    join(repoRoot, 'test-results', 'app-game-android-physical-device-proof', 'proof.json')
  );
  const replayModule = await import(
    pathToFileURL(join(repoRoot, 'packages', 'schema-domain', 'dist', 'app-game-android-usage-events-replay.js')).href
  );
  const statusModule = await import(
    pathToFileURL(join(repoRoot, 'packages', 'app-game-domain', 'dist', 'app-game-platform-proof-status.js')).href
  );
  const linuxProof = await readJson(join(repoRoot, 'test-results', 'app-game-linux-wsl-runtime-proof', 'proof.json'));
  const androidUsageEventsReplay = replayModule.createAppGameAndroidUsageEventsReplayReadModel({
    androidProof: androidProof.readModel,
    generatedAt: '2026-06-08T17:35:00.000Z',
  });
  const replaySummary = replayModule.summarizeAppGameAndroidUsageEventsReplayReadModel(androidUsageEventsReplay);
  const platformStatus = statusModule.createAppGamePlatformProofStatusReadModel({
    androidProof: androidProof.readModel,
    androidUsageEventsReplay,
    linuxProof: linuxProof.readModel,
    generatedAt: '2026-06-08T17:36:00.000Z',
  });

  assertEqual(replaySummary.replayState, 'durable-replay-ready', 'replay state');
  assertEqual(replaySummary.runtimeVisibilityReady, true, 'runtime visibility ready');
  assertPositive(replaySummary.usageEventsSampleCount, 'usage-events sample count');
  assertPositive(replaySummary.foregroundActivityEventCount, 'foreground activity event count');
  assertAndroidStatusUsesReplay(platformStatus);

  const proof = {
    schemaVersion: 1,
    proofMode,
    checkedAt: 'deterministic-proof-artifact',
    commit: await gitHead(),
    commands,
    androidUsageEventsReplay,
    replaySummary,
    platformStatus,
    evidence: {
      contract: 'packages/schema-domain/src/app-game-android-usage-events-replay.ts',
      contractTest: 'packages/app-game-domain/tests/unit/app-game-android-usage-events-replay.test.ts',
      platformStatus: 'packages/app-game-domain/src/app-game-platform-proof-status.ts',
      platformStatusTest: 'packages/app-game-domain/tests/unit/app-game-platform-proof-status.test.ts',
      androidPhysicalProof: 'test-results/app-game-android-physical-device-proof/proof.json',
    },
    claimsProved: [
      'Android UsageEvents foreground counts are replay-ready as parent-safe redacted counts',
      'The Android platform proof status row can carry android-usage-events-replay-ref',
      'The durable usage replay gap is removed only when replay readiness is attached',
    ],
    claimsNotProved: [
      'Android raw UsageEvents rows or package names',
      'Android child runtime replay consumer',
      'Android Device Owner or Profile Owner authority',
      'Android hide, suspend, uninstall block, lock task, or managed configuration',
      'Adapter dispatch, platform enforcement, provider delivery, or child-device delivery',
    ],
  };

  await writeJson(proofPath, proof);
  await writeJson(join(appGameProofDir, 'proof.json'), proof);
  await writeFile(join(appGameProofDir, '10-validation-commands.log'), `${commands.join('\n')}\n`);

  console.log('app-game-android-usage-events-replay-proof-ok');
  console.log(`evidence=${relative(repoRoot, proofPath)}`);
}

async function runCommand(command, args) {
  const commandLine = [command, ...args].join(' ');
  commands.push(commandLine);
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) => (code === 0 ? resolve() : reject(new Error(`${commandLine} exited with ${code}`))));
    child.once('error', reject);
  });
}

async function readJson(path) {
  return JSON.parse(await readFile(path, 'utf8'));
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
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

function assertAndroidStatusUsesReplay(platformStatus) {
  const androidRow = platformStatus.rows.find((row) => row.platform === 'android');
  if (!androidRow) {
    throw new Error('Android platform status row missing');
  }
  if (!androidRow.proofRefs.includes('android-usage-events-replay-ref')) {
    throw new Error('Android platform status row missing replay proof ref');
  }
  if (androidRow.openGaps.includes('android-durable-usage-events-replay-not-proved')) {
    throw new Error('Android durable replay gap remained after replay readiness attached');
  }
}

function assertEqual(actual, expected, label) {
  if (actual !== expected) {
    throw new Error(`${label}: expected ${expected}, received ${actual}`);
  }
}

function assertPositive(actual, label) {
  if (actual <= 0) {
    throw new Error(`${label}: expected positive count, received ${actual}`);
  }
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
