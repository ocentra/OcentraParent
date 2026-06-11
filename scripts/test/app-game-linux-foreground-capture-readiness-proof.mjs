import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'app-game-linux-foreground-capture-readiness-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const appGameProofDir = join(
  repoRoot,
  'output',
  'app-game-plan-proof',
  '189-app-game-linux-foreground-capture-readiness'
);
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
    'app-game-linux-wsl-runtime-proof',
    'app-game-linux-foreground-capture-readiness',
    'app-game-platform-proof-status',
  ]);
  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);

  const linuxProof = await readJson(join(repoRoot, 'test-results', 'app-game-linux-wsl-runtime-proof', 'proof.json'));
  const readinessModule = await import(
    pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-linux-foreground-capture-readiness.js'))
      .href
  );
  const statusModule = await import(
    pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-platform-proof-status.js')).href
  );
  const androidProof = await readJson(
    join(repoRoot, 'test-results', 'app-game-android-physical-device-proof', 'proof.json')
  );
  const replayModule = await import(
    pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-android-usage-events-replay.js')).href
  );
  const linuxForegroundCaptureReadiness = readinessModule.createAppGameLinuxForegroundCaptureReadiness({
    linuxProof: linuxProof.readModel,
    generatedAt: '2026-06-08T17:45:00.000Z',
  });
  const readinessSummary = readinessModule.summarizeAppGameLinuxForegroundCaptureReadiness(
    linuxForegroundCaptureReadiness
  );
  const androidUsageEventsReplay = replayModule.createAppGameAndroidUsageEventsReplayReadModel({
    androidProof: androidProof.readModel,
    generatedAt: '2026-06-08T17:44:00.000Z',
  });
  const platformStatus = statusModule.createAppGamePlatformProofStatusReadModel({
    androidProof: androidProof.readModel,
    androidUsageEventsReplay,
    linuxProof: linuxProof.readModel,
    linuxForegroundCaptureReadiness,
    generatedAt: '2026-06-08T17:46:00.000Z',
  });

  assertEqual(readinessSummary.readinessState, 'display-ready-capture-tool-missing', 'readiness state');
  assertEqual(readinessSummary.foregroundCaptureReady, false, 'foreground capture ready');
  assertLinuxStatusCarriesReadiness(platformStatus);

  const proof = {
    schemaVersion: 1,
    proofMode,
    checkedAt: 'deterministic-proof-artifact',
    commit: await gitHead(),
    commands,
    linuxForegroundCaptureReadiness,
    readinessSummary,
    platformStatus,
    evidence: {
      contract: 'packages/parent-domain/src/app-game-linux-foreground-capture-readiness.ts',
      contractTest: 'packages/parent-domain/tests/app-game-linux-foreground-capture-readiness.test.ts',
      platformStatus: 'packages/parent-domain/src/app-game-platform-proof-status.ts',
      platformStatusTest: 'packages/parent-domain/tests/app-game-platform-proof-status.test.ts',
      linuxWslProof: 'test-results/app-game-linux-wsl-runtime-proof/proof.json',
    },
    claimsProved: [
      'Linux WSLg display and X11/Wayland socket readiness are represented as foreground-capture preflight evidence',
      'The Linux platform proof status row can carry linux-foreground-capture-readiness-ref',
      'Foreground capture remains unproved when no active-window tool/source is attached',
    ],
    claimsNotProved: [
      'Linux active foreground capture',
      'Raw active-window title custody',
      'Linux AppArmor, SELinux, package, Flatpak, Snap, rollback, or audit enforcement',
      'Adapter dispatch, platform enforcement, provider delivery, or child-device delivery',
    ],
  };

  await writeJson(proofPath, proof);
  await writeJson(join(appGameProofDir, 'proof.json'), proof);
  await writeFile(join(appGameProofDir, '10-validation-commands.log'), `${commands.join('\n')}\n`);

  console.log('app-game-linux-foreground-capture-readiness-proof-ok');
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

function assertLinuxStatusCarriesReadiness(platformStatus) {
  const linuxRow = platformStatus.rows.find((row) => row.platform === 'linux');
  if (!linuxRow) {
    throw new Error('Linux platform status row missing');
  }
  if (!linuxRow.proofRefs.includes('linux-foreground-capture-readiness-ref')) {
    throw new Error('Linux platform status row missing foreground readiness ref');
  }
  if (!linuxRow.openGaps.includes('linux-foreground-capture-not-proved')) {
    throw new Error('Linux foreground capture gap must remain open');
  }
}

function assertEqual(actual, expected, label) {
  if (actual !== expected) {
    throw new Error(`${label}: expected ${expected}, received ${actual}`);
  }
}
