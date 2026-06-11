import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'app-game-android-child-runtime-transport-receipt-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const proofPath = join(outputDir, 'proof.json');
const appGameProofDir = join(
  repoRoot,
  'output',
  'app-game-plan-proof',
  '211-app-game-android-child-runtime-transport-receipt-proof'
);
const androidProofPath = join(
  repoRoot,
  'platforms',
  'android',
  'agent',
  'app',
  'src',
  'main',
  'java',
  'ca',
  'ocentra',
  'parent',
  'agent',
  'AppGameAndroidChildRuntimeTransportReceiptProof.java'
);
const mainActivityPath = join(
  repoRoot,
  'platforms',
  'android',
  'agent',
  'app',
  'src',
  'main',
  'java',
  'ca',
  'ocentra',
  'parent',
  'agent',
  'MainActivity.java'
);
const apkPath = join(
  repoRoot,
  'target',
  'release-packages',
  'android',
  'ocentra-parent-agent-android-debug-latest.apk'
);
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
      '@ocentra-parent/parent-domain',
      '--',
      'app-game-android-child-runtime-transport-receipt-proof',
    ])
  );
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']));
  await runCommand(...npmCommand(['run', 'release:package:android']));
  assertFileExists(apkPath, 'Android debug APK');

  const proofSource = await readFile(androidProofPath, 'utf8');
  const mainActivity = await readFile(mainActivityPath, 'utf8');
  const sourceState = parseAndroidSourceState(proofSource, mainActivity);

  const contractModule = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-android-child-runtime-transport-receipt-proof.js')
    ).href
  );
  const readModel = contractModule.createAppGameAndroidChildRuntimeTransportReceiptProof({
    transportChannelState: sourceState.transportChannelState,
    receiptStoreState: sourceState.receiptStoreState,
    receiptAckState: sourceState.receiptAckState,
    packageActivityVisible: sourceState.packageActivityVisible,
    uiTransportStateObserved: sourceState.uiTransportStateObserved,
    uiReceiptStateObserved: sourceState.uiReceiptStateObserved,
    checkedAt: '2026-06-08T21:45:00.000Z',
  });
  const summary = contractModule.summarizeAppGameAndroidChildRuntimeTransportReceiptProof(readModel);

  const proof = {
    schemaVersion: 1,
    proofMode,
    checkedAt: 'deterministic-proof-artifact',
    commit: await gitHead(),
    commands,
    sourceState,
    readModel,
    summary,
    evidence: {
      androidProofSource:
        'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidChildRuntimeTransportReceiptProof.java',
      androidActivity: 'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/MainActivity.java',
      contract: 'packages/parent-domain/src/app-game-android-child-runtime-transport-receipt-proof.ts',
      contractTest: 'packages/parent-domain/tests/app-game-android-child-runtime-transport-receipt-proof.test.ts',
      packageBuild: 'target/release-packages/android/ocentra-parent-agent-android-debug-latest.apk',
    },
    claimsProved: [
      'The Android child app package compiles with a child runtime transport receipt status bundle',
      'MainActivity renders parent-safe transport-channel, receipt-store, and receipt-ack states',
      'Parent-domain accepts the Android child runtime transport receipt proof only when activity UI and receipt-store evidence are present',
      'Runtime transport execution, receipt ingestion, provider delivery, platform delivery channel, adapter dispatch, platform enforcement, and raw private source rows remain unclaimed',
    ],
    claimsNotProved: [
      'Physical Android child runtime transport execution',
      'Runtime receipt ingestion',
      'Provider delivery execution',
      'Platform delivery channel execution',
      'Adapter dispatch or platform enforcement',
    ],
  };

  await writeJson(proofPath, proof);
  await writeJson(join(appGameProofDir, 'proof.json'), proof);
  await writeFile(join(appGameProofDir, '00-source-snapshot.md'), sourceSnapshot(sourceState));
  await writeFile(join(appGameProofDir, '10-validation-commands.log'), `${commands.join('\n')}\n`);

  console.log('app-game-android-child-runtime-transport-receipt-proof-ok');
  console.log(`evidence=${relativePath(proofPath)}`);
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

function parseAndroidSourceState(proofSource, mainActivity) {
  const hasBundleFactory = proofSource.includes('createChildRuntimeTransportReceiptBundle');
  const packageActivityVisible =
    hasBundleFactory &&
    proofSource.includes('TRANSPORT_CHANNEL_ACTIVITY_VISIBLE') &&
    mainActivity.includes('AppGameAndroidChildRuntimeTransportReceiptProof.createChildRuntimeTransportReceiptBundle');
  const uiTransportStateObserved =
    mainActivity.includes('FIELD_TRANSPORT_CHANNEL_STATE') &&
    proofSource.includes('activity-visible-transport-channel');
  const uiReceiptStateObserved =
    mainActivity.includes('FIELD_RECEIPT_STORE_STATE') && proofSource.includes('internal-receipt-store-available');

  return {
    transportChannelState: packageActivityVisible
      ? 'activity-visible-transport-channel'
      : 'activity-unavailable-transport-channel',
    receiptStoreState: proofSource.includes('internalReceiptStoreAvailable')
      ? 'internal-receipt-store-available'
      : 'internal-receipt-store-unavailable',
    receiptAckState: 'receipt-ack-waiting-for-runtime',
    packageActivityVisible,
    uiTransportStateObserved,
    uiReceiptStateObserved,
  };
}

function assertFileExists(path, label) {
  if (!existsSync(path)) {
    throw new Error(`${label} missing at ${relativePath(path)}`);
  }
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}

function sourceSnapshot(sourceState) {
  return [
    '# WP211 Android child runtime transport receipt proof source snapshot',
    '',
    '- Package: `ca.ocentra.parent.agent`',
    `- Transport channel state: \`${sourceState.transportChannelState}\``,
    `- Receipt store state: \`${sourceState.receiptStoreState}\``,
    `- Receipt ack state: \`${sourceState.receiptAckState}\``,
    `- Activity visible: \`${sourceState.packageActivityVisible}\``,
    `- UI transport state observed: \`${sourceState.uiTransportStateObserved}\``,
    `- UI receipt state observed: \`${sourceState.uiReceiptStateObserved}\``,
    '',
  ].join('\n');
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
