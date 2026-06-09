import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'app-game-platform-runtime-readiness-batch';
const outputDir = join(repoRoot, 'test-results', proofMode);
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', '190-191-platform-runtime-readiness-batch');
const proofPath = join(outputDir, 'proof.json');
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });
  await mkdir(appGameProofDir, { recursive: true });

  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'app-game-android-usage-events-child-runtime-replay',
    'app-game-linux-foreground-source-preflight',
    'app-game-android-usage-events-replay',
    'app-game-linux-foreground-capture-readiness',
  ]);
  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);

  const androidProof = await readJson(
    join(repoRoot, 'test-results', 'app-game-android-physical-device-proof', 'proof.json')
  );
  const linuxProof = await readJson(join(repoRoot, 'test-results', 'app-game-linux-wsl-runtime-proof', 'proof.json'));
  const androidReplayModule = await import(
    pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-android-usage-events-replay.js')).href
  );
  const androidRuntimeModule = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-android-usage-events-child-runtime-replay.js')
    ).href
  );
  const linuxReadinessModule = await import(
    pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-linux-foreground-capture-readiness.js'))
      .href
  );
  const linuxPreflightModule = await import(
    pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-linux-foreground-source-preflight.js'))
      .href
  );

  const androidUsageReplay = androidReplayModule.createAppGameAndroidUsageEventsReplayReadModel({
    androidProof: androidProof.readModel,
    generatedAt: '2026-06-08T17:46:00.000Z',
  });
  const androidChildRuntimeReplay =
    androidRuntimeModule.createAppGameAndroidUsageEventsChildRuntimeReplayReadModel({
      replayReadModel: androidUsageReplay,
      generatedAt: '2026-06-08T17:47:00.000Z',
    });
  const linuxForegroundReadiness = linuxReadinessModule.createAppGameLinuxForegroundCaptureReadiness({
    linuxProof: linuxProof.readModel,
    generatedAt: '2026-06-08T17:46:00.000Z',
  });
  const linuxForegroundPreflight = linuxPreflightModule.createAppGameLinuxForegroundSourcePreflightReadModel({
    readiness: linuxForegroundReadiness,
    generatedAt: '2026-06-08T17:47:00.000Z',
  });

  const androidSummary =
    androidRuntimeModule.summarizeAppGameAndroidUsageEventsChildRuntimeReplayReadModel(androidChildRuntimeReplay);
  const linuxSummary =
    linuxPreflightModule.summarizeAppGameLinuxForegroundSourcePreflightReadModel(linuxForegroundPreflight);

  assertEqual(androidSummary.childRuntimeReplayConsumerAttached, true, 'Android child runtime replay consumer');
  assertPositive(androidSummary.replayedForegroundEventCount, 'Android replayed foreground event count');
  assertEqual(linuxSummary.displayProofAttached, true, 'Linux WSLg display proof');
  assertEqual(linuxSummary.foregroundSourcePreflightReady, false, 'Linux foreground source preflight ready on host');
  assertEqual(linuxSummary.preflightState, 'foreground-tool-install-required', 'Linux preflight state');

  const proof = {
    schemaVersion: 1,
    proofMode,
    checkedAt: 'deterministic-proof-artifact',
    commit: await gitHead(),
    commands,
    androidChildRuntimeReplay,
    androidSummary,
    linuxForegroundPreflight,
    linuxSummary,
    evidence: {
      androidContract: 'packages/parent-domain/src/app-game-android-usage-events-child-runtime-replay.ts',
      androidContractTest: 'packages/parent-domain/tests/app-game-android-usage-events-child-runtime-replay.test.ts',
      linuxContract: 'packages/parent-domain/src/app-game-linux-foreground-source-preflight.ts',
      linuxContractTest: 'packages/parent-domain/tests/app-game-linux-foreground-source-preflight.test.ts',
      androidPhysicalProof: 'test-results/app-game-android-physical-device-proof/proof.json',
      linuxWslProof: 'test-results/app-game-linux-wsl-runtime-proof/proof.json',
    },
    claimsProved: [
      'Android UsageEvents replay can feed a child-runtime replay consumer as redacted counters only',
      'Android child runtime replay consumer gap is removed without claiming child-device delivery',
      'Linux WSLg display/socket readiness feeds a foreground source preflight',
      'Linux host currently requires an active-window tool before foreground capture can be claimed',
    ],
    claimsNotProved: [
      'Android raw UsageEvents rows or package names',
      'Android child-device delivery, hide, suspend, lock task, managed configuration, or Device Owner/Profile Owner authority',
      'Linux raw window title capture',
      'Linux foreground capture, AppArmor/SELinux/package manager enforcement, rollback, audit, or child-device delivery',
      'macOS or iOS native runtime execution on this Windows host',
    ],
  };

  await writeJson(proofPath, proof);
  await writeJson(join(appGameProofDir, 'proof.json'), proof);
  await writeFile(join(appGameProofDir, '10-validation-commands.log'), `${commands.join('\n')}\n`);

  console.log('app-game-platform-runtime-readiness-batch-ok');
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
