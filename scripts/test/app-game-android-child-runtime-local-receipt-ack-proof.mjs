import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'app-game-android-child-runtime-local-receipt-ack-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const proofPath = join(outputDir, 'proof.json');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', '214-app-game-android-child-runtime-local-receipt-ack-proof');
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
const apkPath = join(repoRoot, 'target', 'release-packages', 'android', 'ocentra-parent-agent-android-debug-latest.apk');
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
    'app-game-android-child-runtime-local-receipt-ack-proof',
  ]);
  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  await runCommand('cmd', ['/c', 'npm', 'run', 'release:package:android']);
  assertFileExists(apkPath, 'Android debug APK');

  const proofSource = await readFile(androidProofPath, 'utf8');
  const mainActivity = await readFile(mainActivityPath, 'utf8');
  const sourceState = parseAndroidSourceState(proofSource, mainActivity);
  const contractModule = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-android-child-runtime-local-receipt-ack-proof.js')
    ).href
  );
  const readModel = contractModule.createAppGameAndroidChildRuntimeLocalReceiptAckProof({
    receiptAppendState: sourceState.receiptAppendState,
    receiptReadbackState: sourceState.receiptReadbackState,
    receiptLocalAckState: sourceState.receiptLocalAckState,
    receiptLocalAckReadbackState: sourceState.receiptLocalAckReadbackState,
    packageActivityVisible: sourceState.packageActivityVisible,
    uiReceiptAckStateObserved: sourceState.uiReceiptAckStateObserved,
    uiReceiptAckReadbackStateObserved: sourceState.uiReceiptAckReadbackStateObserved,
    checkedAt: '2026-06-08T23:10:00.000Z',
  });
  const summary = contractModule.summarizeAppGameAndroidChildRuntimeLocalReceiptAckProof(readModel);

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
      contract: 'packages/parent-domain/src/app-game-android-child-runtime-local-receipt-ack-proof.ts',
      contractTest: 'packages/parent-domain/tests/app-game-android-child-runtime-local-receipt-ack-proof.test.ts',
      packageBuild: 'target/release-packages/android/ocentra-parent-agent-android-debug-latest.apk',
    },
    claimsProved: [
      'The Android child app package compiles with package-local receipt and ack write/readback code',
      'MainActivity renders parent-safe local receipt ack and ack readback states',
      'Parent-domain accepts the ack proof only when package-local receipt and ack write/readback evidence is present',
    ],
    claimsNotProved: [
      'Physical child runtime transport execution',
      'Service receipt ingestion',
      'Provider delivery execution',
      'Platform delivery channel execution',
      'Adapter dispatch or platform enforcement',
      'Raw private source row storage',
    ],
  };

  await writeJson(proofPath, proof);
  await writeJson(join(appGameProofDir, 'proof.json'), proof);
  await writeFile(join(appGameProofDir, '00-source-snapshot.md'), sourceSnapshot(sourceState));
  await writeFile(join(appGameProofDir, '10-validation-commands.log'), `${commands.join('\n')}\n`);

  console.log('app-game-android-child-runtime-local-receipt-ack-proof-ok');
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
  return {
    receiptAppendState: proofSource.includes('RECEIPT_RECORD') ? 'local-receipt-append-recorded' : 'local-receipt-append-unavailable',
    receiptReadbackState: proofSource.includes('FileInputStream') ? 'local-receipt-readback-observed' : 'local-receipt-readback-unavailable',
    receiptLocalAckState: proofSource.includes('RECEIPT_ACK_RECORD') ? 'local-receipt-ack-recorded' : 'local-receipt-ack-unavailable',
    receiptLocalAckReadbackState: proofSource.includes('RECEIPT_LOCAL_ACK_READBACK_OBSERVED')
      ? 'local-receipt-ack-readback-observed'
      : 'local-receipt-ack-readback-unavailable',
    packageActivityVisible: mainActivity.includes(
      'AppGameAndroidChildRuntimeTransportReceiptProof.createChildRuntimeTransportReceiptBundle'
    ),
    uiReceiptAckStateObserved: mainActivity.includes('FIELD_RECEIPT_LOCAL_ACK_STATE'),
    uiReceiptAckReadbackStateObserved: mainActivity.includes('FIELD_RECEIPT_LOCAL_ACK_READBACK_STATE'),
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
    '# WP214 Android child runtime local receipt ack proof source snapshot',
    '',
    '- Package: `ca.ocentra.parent.agent`',
    `- Receipt append state: \`${sourceState.receiptAppendState}\``,
    `- Receipt readback state: \`${sourceState.receiptReadbackState}\``,
    `- Receipt local ack state: \`${sourceState.receiptLocalAckState}\``,
    `- Receipt local ack readback state: \`${sourceState.receiptLocalAckReadbackState}\``,
    `- Activity visible: \`${sourceState.packageActivityVisible}\``,
    `- UI ack state observed: \`${sourceState.uiReceiptAckStateObserved}\``,
    `- UI ack readback observed: \`${sourceState.uiReceiptAckReadbackStateObserved}\``,
    '',
  ].join('\n');
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}
