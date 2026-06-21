import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'app-game-android-child-runtime-local-delivery-intake-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const proofPath = join(outputDir, 'proof.json');
const appGameProofDir = join(
  repoRoot,
  'output',
  'app-game-plan-proof',
  '216-app-game-android-child-runtime-local-delivery-intake-proof'
);
const androidDeliveryPath = join(
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
  'AppGameAndroidChildRuntimeDeliveryProof.java'
);
const deliveryReceiverPath = join(
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
  'AppGameAndroidChildRuntimeDeliveryReceiver.java'
);
const receiptProofPath = join(
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

  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/app-game-domain',
      '--',
      'app-game-android-child-runtime-local-delivery-intake-proof',
    ])
  );
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/app-game-domain']));
  await runCommand(...npmCommand(['run', 'release:package:android']));
  assertFileExists(apkPath, 'Android debug APK');

  const deliverySource = await readFile(androidDeliveryPath, 'utf8');
  const deliveryReceiver = await readFile(deliveryReceiverPath, 'utf8');
  const receiptSource = await readFile(receiptProofPath, 'utf8');
  const mainActivity = await readFile(mainActivityPath, 'utf8');
  const manifest = await readFile(manifestPath, 'utf8');
  const sourceState = parseAndroidSourceState(deliverySource, deliveryReceiver, receiptSource, mainActivity, manifest);
  const contractModule = await import(
    pathToFileURL(
      join(
        repoRoot,
        'packages',
        'app-game-domain',
        'dist',
        'app-game-android-child-runtime-local-delivery-intake-proof.js'
      )
    ).href
  );
  const readModel = contractModule.createAppGameAndroidChildRuntimeLocalDeliveryIntakeProof({
    deliveryIntakeState: sourceState.deliveryIntakeState,
    deliveryReadbackState: sourceState.deliveryReadbackState,
    receiptChannelState: sourceState.receiptChannelState,
    receiptAppendState: sourceState.receiptAppendState,
    receiptLocalAckState: sourceState.receiptLocalAckState,
    packageLocalDeliveryReceiverDeclared: sourceState.packageLocalDeliveryReceiverDeclared,
    packageLocalDeliveryTriggeredByActivity: sourceState.packageLocalDeliveryTriggeredByActivity,
    checkedAt: '2026-06-08T23:30:00.000Z',
  });
  const summary = contractModule.summarizeAppGameAndroidChildRuntimeLocalDeliveryIntakeProof(readModel);

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
      androidDeliverySource:
        'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidChildRuntimeDeliveryProof.java',
      androidDeliveryReceiver:
        'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidChildRuntimeDeliveryReceiver.java',
      androidReceiptSource:
        'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidChildRuntimeTransportReceiptProof.java',
      androidActivity: 'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/MainActivity.java',
      androidManifest: 'platforms/android/agent/app/src/main/AndroidManifest.xml',
      contract: 'packages/app-game-domain/src/app-game-android-child-runtime-local-delivery-intake-proof.ts',
      contractTest: 'packages/app-game-domain/tests/unit/app-game-android-child-runtime-local-delivery-intake-proof.test.ts',
      packageBuild: 'target/release-packages/android/ocentra-parent-agent-android-debug-latest.apk',
    },
    claimsProved: [
      'The Android child app package compiles with a non-exported package-local delivery receiver',
      'MainActivity can trigger package-local delivery intake proof without exposing an external broadcast target',
      'The delivery intake path records local delivery, receipt channel, receipt, and receipt-ack marker custody',
    ],
    claimsNotProved: [
      'Service delivery or receipt ingestion',
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

  console.log('app-game-android-child-runtime-local-delivery-intake-proof-ok');
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

function parseAndroidSourceState(deliverySource, deliveryReceiver, receiptSource, mainActivity, manifest) {
  return {
    deliveryIntakeState:
      deliverySource.includes('DELIVERY_RECORD') && deliveryReceiver.includes('recordPackageLocalDeliveryIntake')
        ? 'package-local-delivery-intake-recorded'
        : 'package-local-delivery-intake-unavailable',
    deliveryReadbackState: deliverySource.includes('readDeliveryFile')
      ? 'package-local-delivery-readback-observed'
      : 'package-local-delivery-readback-unavailable',
    receiptChannelState: receiptSource.includes('RECEIPT_CHANNEL_RECORD')
      ? 'package-local-receipt-channel-recorded'
      : 'package-local-receipt-channel-unavailable',
    receiptAppendState: receiptSource.includes('RECEIPT_RECORD')
      ? 'local-receipt-append-recorded'
      : 'local-receipt-append-unavailable',
    receiptLocalAckState: receiptSource.includes('RECEIPT_ACK_RECORD')
      ? 'local-receipt-ack-recorded'
      : 'local-receipt-ack-unavailable',
    packageLocalDeliveryReceiverDeclared:
      manifest.includes('AppGameAndroidChildRuntimeDeliveryReceiver') &&
      manifest.includes('android:exported="false"') &&
      manifest.includes('APP_GAME_CHILD_RUNTIME_DELIVERY_INTAKE_PROOF'),
    packageLocalDeliveryTriggeredByActivity:
      mainActivity.includes('EXTRA_RUN_APP_GAME_DELIVERY_INTAKE_PROOF') &&
      mainActivity.includes('ACTION_LOCAL_DELIVERY_INTAKE_PROOF') &&
      mainActivity.includes('sendBroadcast(deliveryIntakeIntent)'),
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
    '# WP216 Android child runtime local delivery intake proof source snapshot',
    '',
    '- Package: `ca.ocentra.parent.agent`',
    `- Delivery intake state: \`${sourceState.deliveryIntakeState}\``,
    `- Delivery readback state: \`${sourceState.deliveryReadbackState}\``,
    `- Receipt channel state: \`${sourceState.receiptChannelState}\``,
    `- Receipt append state: \`${sourceState.receiptAppendState}\``,
    `- Receipt ack state: \`${sourceState.receiptLocalAckState}\``,
    `- Non-exported delivery receiver declared: \`${sourceState.packageLocalDeliveryReceiverDeclared}\``,
    `- Activity delivery trigger declared: \`${sourceState.packageLocalDeliveryTriggeredByActivity}\``,
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
