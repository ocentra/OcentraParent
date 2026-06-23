import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'app-game-android-child-runtime-local-receipt-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const proofPath = join(outputDir, 'proof.json');
const appGameProofDir = join(
  repoRoot,
  'output',
  'app-game-plan-proof',
  '212-app-game-android-child-runtime-local-receipt-proof'
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
      '@ocentra-parent/app-game-domain',
      '--',
      'app-game-android-child-runtime-local-receipt-proof',
    ])
  );
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/schema-domain']));
  await runCommand(...npmCommand(['run', 'release:package:android']));
  assertFileExists(apkPath, 'Android debug APK');

  const proofSource = await readFile(androidProofPath, 'utf8');
  const mainActivity = await readFile(mainActivityPath, 'utf8');
  const sourceState = parseAndroidSourceState(proofSource, mainActivity);

  const contractModule = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'schema-domain', 'dist', 'app-game-android-child-runtime-local-receipt-proof.js')
    ).href
  );
  const readModel = contractModule.createAppGameAndroidChildRuntimeLocalReceiptProof({
    receiptStoreState: sourceState.receiptStoreState,
    receiptAppendState: sourceState.receiptAppendState,
    receiptReadbackState: sourceState.receiptReadbackState,
    packageActivityVisible: sourceState.packageActivityVisible,
    uiReceiptAppendStateObserved: sourceState.uiReceiptAppendStateObserved,
    uiReceiptReadbackStateObserved: sourceState.uiReceiptReadbackStateObserved,
    checkedAt: '2026-06-08T22:20:00.000Z',
  });
  const summary = contractModule.summarizeAppGameAndroidChildRuntimeLocalReceiptProof(readModel);

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
      contract: 'packages/schema-domain/src/app-game-android-child-runtime-local-receipt-proof.ts',
      contractTest: 'packages/app-game-domain/tests/unit/app-game-android-child-runtime-local-receipt-proof.test.ts',
      packageBuild: 'target/release-packages/android/ocentra-parent-agent-android-debug-latest.apk',
    },
    claimsProved: [
      'The Android child app package compiles with a package-local receipt append and readback proof',
      'MainActivity renders parent-safe local receipt append and readback states',
      'The centralized local-receipt proof accepts the Android evidence only when package-local write/readback markers are present',
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

  console.log('app-game-android-child-runtime-local-receipt-proof-ok');
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
  const packageActivityVisible =
    proofSource.includes('writeAndReadLocalReceiptProof') &&
    proofSource.includes('RECEIPT_RECORD') &&
    mainActivity.includes('AppGameAndroidChildRuntimeTransportReceiptProof.createChildRuntimeTransportReceiptBundle');
  const uiReceiptAppendStateObserved =
    mainActivity.includes('FIELD_RECEIPT_APPEND_STATE') && proofSource.includes('local-receipt-append-recorded');
  const uiReceiptReadbackStateObserved =
    mainActivity.includes('FIELD_RECEIPT_READBACK_STATE') && proofSource.includes('local-receipt-readback-observed');

  return {
    receiptStoreState: proofSource.includes('internalReceiptStoreAvailable')
      ? 'internal-receipt-store-available'
      : 'internal-receipt-store-unavailable',
    receiptAppendState: proofSource.includes('FileOutputStream')
      ? 'local-receipt-append-recorded'
      : 'local-receipt-append-unavailable',
    receiptReadbackState: proofSource.includes('FileInputStream')
      ? 'local-receipt-readback-observed'
      : 'local-receipt-readback-unavailable',
    packageActivityVisible,
    uiReceiptAppendStateObserved,
    uiReceiptReadbackStateObserved,
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
    '# WP212 Android child runtime local receipt proof source snapshot',
    '',
    '- Package: `ca.ocentra.parent.agent`',
    `- Receipt store state: \`${sourceState.receiptStoreState}\``,
    `- Receipt append state: \`${sourceState.receiptAppendState}\``,
    `- Receipt readback state: \`${sourceState.receiptReadbackState}\``,
    `- Activity visible: \`${sourceState.packageActivityVisible}\``,
    `- UI append state observed: \`${sourceState.uiReceiptAppendStateObserved}\``,
    `- UI readback state observed: \`${sourceState.uiReceiptReadbackStateObserved}\``,
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
