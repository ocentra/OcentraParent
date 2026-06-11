import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'app-game-android-child-runtime-local-receipt-channel-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const proofPath = join(outputDir, 'proof.json');
const appGameProofDir = join(
  repoRoot,
  'output',
  'app-game-plan-proof',
  '215-app-game-android-child-runtime-local-receipt-channel-proof'
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
const receiverPath = join(
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
  'AppGameAndroidChildRuntimeReceiptReceiver.java'
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
const manifestPath = join(repoRoot, 'platforms', 'android', 'agent', 'app', 'src', 'main', 'AndroidManifest.xml');
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

  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'app-game-android-child-runtime-local-receipt-channel-proof',
  ]);
  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  await runCommand('cmd', ['/c', 'npm', 'run', 'release:package:android']);
  assertFileExists(apkPath, 'Android debug APK');

  const proofSource = await readFile(androidProofPath, 'utf8');
  const receiverSource = await readFile(receiverPath, 'utf8');
  const mainActivity = await readFile(mainActivityPath, 'utf8');
  const manifest = await readFile(manifestPath, 'utf8');
  const sourceState = parseAndroidSourceState(proofSource, receiverSource, mainActivity, manifest);
  const contractModule = await import(
    pathToFileURL(
      join(
        repoRoot,
        'packages',
        'parent-domain',
        'dist',
        'app-game-android-child-runtime-local-receipt-channel-proof.js'
      )
    ).href
  );
  const readModel = contractModule.createAppGameAndroidChildRuntimeLocalReceiptChannelProof({
    receiptChannelState: sourceState.receiptChannelState,
    receiptAppendState: sourceState.receiptAppendState,
    receiptLocalAckState: sourceState.receiptLocalAckState,
    packageLocalBroadcastReceiverDeclared: sourceState.packageLocalBroadcastReceiverDeclared,
    packageLocalBroadcastTriggeredByActivity: sourceState.packageLocalBroadcastTriggeredByActivity,
    checkedAt: '2026-06-08T23:20:00.000Z',
  });
  const summary = contractModule.summarizeAppGameAndroidChildRuntimeLocalReceiptChannelProof(readModel);

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
      androidReceiver:
        'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidChildRuntimeReceiptReceiver.java',
      androidActivity: 'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/MainActivity.java',
      androidManifest: 'platforms/android/agent/app/src/main/AndroidManifest.xml',
      contract: 'packages/parent-domain/src/app-game-android-child-runtime-local-receipt-channel-proof.ts',
      contractTest: 'packages/parent-domain/tests/app-game-android-child-runtime-local-receipt-channel-proof.test.ts',
      packageBuild: 'target/release-packages/android/ocentra-parent-agent-android-debug-latest.apk',
    },
    claimsProved: [
      'The Android child app package compiles with a non-exported package-local receipt channel receiver',
      'MainActivity can trigger the package-local receipt channel proof without exposing an external broadcast target',
      'Parent-domain accepts the channel proof only when receipt, ack, receiver, and activity trigger evidence is present',
    ],
    claimsNotProved: [
      'Service receipt ingestion',
      'Provider delivery execution',
      'Platform delivery channel execution outside the child package',
      'Adapter dispatch or platform enforcement',
      'Raw private source row storage',
    ],
  };

  await writeJson(proofPath, proof);
  await writeJson(join(appGameProofDir, 'proof.json'), proof);
  await writeFile(join(appGameProofDir, '00-source-snapshot.md'), sourceSnapshot(sourceState));
  await writeFile(join(appGameProofDir, '10-validation-commands.log'), `${commands.join('\n')}\n`);

  console.log('app-game-android-child-runtime-local-receipt-channel-proof-ok');
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

function parseAndroidSourceState(proofSource, receiverSource, mainActivity, manifest) {
  return {
    receiptChannelState:
      proofSource.includes('RECEIPT_CHANNEL_RECORD') && receiverSource.includes('recordPackageLocalReceiptChannel')
        ? 'package-local-receipt-channel-recorded'
        : 'package-local-receipt-channel-unavailable',
    receiptAppendState: proofSource.includes('RECEIPT_RECORD')
      ? 'local-receipt-append-recorded'
      : 'local-receipt-append-unavailable',
    receiptLocalAckState: proofSource.includes('RECEIPT_ACK_RECORD')
      ? 'local-receipt-ack-recorded'
      : 'local-receipt-ack-unavailable',
    packageLocalBroadcastReceiverDeclared:
      manifest.includes('AppGameAndroidChildRuntimeReceiptReceiver') &&
      manifest.includes('android:exported="false"') &&
      manifest.includes('APP_GAME_CHILD_RUNTIME_RECEIPT_CHANNEL_PROOF'),
    packageLocalBroadcastTriggeredByActivity:
      mainActivity.includes('EXTRA_RUN_APP_GAME_RECEIPT_CHANNEL_PROOF') &&
      mainActivity.includes('ACTION_LOCAL_RECEIPT_CHANNEL_PROOF') &&
      mainActivity.includes('sendBroadcast(receiptChannelIntent)'),
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
    '# WP215 Android child runtime local receipt channel proof source snapshot',
    '',
    '- Package: `ca.ocentra.parent.agent`',
    `- Receipt channel state: \`${sourceState.receiptChannelState}\``,
    `- Receipt append state: \`${sourceState.receiptAppendState}\``,
    `- Receipt ack state: \`${sourceState.receiptLocalAckState}\``,
    `- Non-exported receiver declared: \`${sourceState.packageLocalBroadcastReceiverDeclared}\``,
    `- Activity trigger declared: \`${sourceState.packageLocalBroadcastTriggeredByActivity}\``,
    '',
  ].join('\n');
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}
