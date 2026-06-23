import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'app-game-android-usage-events-capability-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const appGameProofDir = join(
  repoRoot,
  'output',
  'app-game-plan-proof',
  '198-app-game-android-usage-events-capability-proof'
);
const proofPath = join(outputDir, 'proof.json');
const commands = [];
const appGameUsageContractsPath = 'packages/schema-domain/src/app-game-android-usage-events-contracts.ts';

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
      'app-game-android-usage-events-capability-proof',
    ])
  );
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/schema-domain']));
  await runCommand(...npmCommand(['run', 'release:package:android']));

  const sourceProof = await assertAndroidSourceProof();
  const contractModule = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'schema-domain', 'dist', 'app-game-android-usage-events-capability-proof.js')
    ).href
  );
  const readModel = contractModule.createAppGameAndroidUsageEventsCapabilityReadModel({
    checkedAt: '2026-06-08T19:20:00.000Z',
  });
  const summary = contractModule.summarizeAppGameAndroidUsageEventsCapabilityReadModel(readModel);

  assertEqual(summary.usageEventsBridgeState, 'package-local-scaffold', 'UsageEvents bridge state');
  assertEqual(summary.permissionState, 'settings-grant-required', 'UsageStats permission state');
  assertEqual(summary.eventCollectionState, 'runtime-grant-not-proved', 'UsageEvents collection state');
  assertEqual(summary.adapterDispatchClaimed, false, 'adapter dispatch claim');
  assertEqual(summary.platformEnforcementClaimed, false, 'platform enforcement claim');
  assertEqual(summary.childDeviceDeliveryClaimed, false, 'child delivery claim');

  const proof = {
    schemaVersion: 1,
    proofMode,
    checkedAt: 'deterministic-proof-artifact',
    commit: await gitHead(),
    commands,
    readModel,
    summary,
    sourceProof,
    evidence: {
      androidBridge:
        'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidUsageEventsCapabilityProof.java',
      androidActivity: 'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/MainActivity.java',
      androidManifest: 'platforms/android/agent/app/src/main/AndroidManifest.xml',
      contract: 'packages/schema-domain/src/app-game-android-usage-events-capability-proof.ts',
      contractTest: 'packages/app-game-domain/tests/unit/app-game-android-usage-events-capability-proof.test.ts',
    },
    claimsProved: [
      'Android package-local app/game UsageEvents capability bridge compiles into the debug package',
      'The centralized capability contract keeps UsageStats settings grant, runtime collection, adapter dispatch, platform enforcement, and child delivery unclaimed',
      'Android MainActivity exposes the package-local bridge state without storing raw UsageEvents rows or package names',
    ],
    claimsNotProved: [
      'UsageStats settings grant on a child device',
      'Runtime UsageEvents collection from the Android package',
      'Raw UsageEvents rows or package-name custody',
      'Android child runtime delivery, adapter dispatch, or platform enforcement',
      'Device Owner/Profile Owner authority or Play policy proof',
    ],
  };

  await writeJson(proofPath, proof);
  await writeJson(join(appGameProofDir, 'proof.json'), proof);
  await writeFile(join(appGameProofDir, '00-source-snapshot.md'), sourceSnapshot(sourceProof));
  await writeFile(join(appGameProofDir, '10-validation-commands.log'), `${commands.join('\n')}\n`);

  console.log('app-game-android-usage-events-capability-proof-ok');
  console.log(`evidence=${relativePath(proofPath)}`);
}

async function assertAndroidSourceProof() {
  const bridgePath =
    'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidUsageEventsCapabilityProof.java';
  const activityPath = 'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/MainActivity.java';
  const manifestPath = 'platforms/android/agent/app/src/main/AndroidManifest.xml';
  const bridge = await readRepoFile(bridgePath);
  const activity = await readRepoFile(activityPath);
  const manifest = await readRepoFile(manifestPath);
  const appGameUsageContracts = readAppGameUsageContracts(await readRepoFile(appGameUsageContractsPath));

  assertIncludes(bridge, appGameUsageContracts.CapabilityGet, 'capability command');
  assertIncludes(bridge, appGameUsageContracts.ReplayBoundaryGet, 'replay boundary command');
  assertIncludes(bridge, appGameUsageContracts.CapabilityReported, 'capability event');
  assertIncludes(bridge, appGameUsageContracts.ReplayBoundaryReported, 'replay boundary event');
  assertIncludes(bridge, 'android-usage-events-capability-bridge-ref', 'bridge proof ref');
  assertIncludes(bridge, 'android-package-local-usage-events-proof-ref', 'package-local proof ref');
  assertIncludes(bridge, 'android-usage-stats-settings-grant-not-proved', 'settings grant gap');
  assertIncludes(bridge, 'android-usage-events-runtime-collection-not-proved', 'runtime collection gap');
  assertIncludes(bridge, 'android-child-runtime-delivery-not-proved', 'child runtime gap');
  assertIncludes(bridge, 'android-platform-enforcement-not-proved', 'platform enforcement gap');
  assertIncludes(bridge, 'status.putBoolean("rawUsageEventsStored", false)', 'raw UsageEvents non-claim');
  assertIncludes(bridge, 'status.putBoolean("packageNamesStored", false)', 'package-name non-claim');
  assertIncludes(bridge, 'status.putBoolean("adapterDispatchClaimed", false)', 'adapter dispatch non-claim');
  assertIncludes(bridge, 'status.putBoolean("platformEnforcementClaimed", false)', 'platform enforcement non-claim');
  assertIncludes(bridge, 'status.putBoolean("childDeviceDeliveryClaimed", false)', 'child delivery non-claim');
  assertIncludes(
    activity,
    'AppGameAndroidUsageEventsCapabilityProof.createUsageEventsCapabilityBundle()',
    'activity bridge wiring'
  );
  assertIncludes(
    activity,
    'AppGameAndroidUsageEventsCapabilityProof.FIELD_USAGE_EVENTS_BRIDGE_STATE',
    'activity status wiring'
  );
  assertNotIncludes(manifest, 'android.permission.PACKAGE_USAGE_STATS', 'UsageStats privileged permission');

  return {
    bridge: bridgePath,
    activity: activityPath,
    manifest: manifestPath,
    usageStatsManifestState: 'not-declared-by-design',
    bridgeState: 'package-local-scaffold',
  };
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

async function readRepoFile(path) {
  return readFile(join(repoRoot, path), 'utf8');
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

function sourceSnapshot(sourceProof) {
  return [
    '# WP198 Android UsageEvents capability proof source snapshot',
    '',
    `- Bridge: \`${sourceProof.bridge}\``,
    `- Activity wiring: \`${sourceProof.activity}\``,
    `- Manifest: \`${sourceProof.manifest}\``,
    `- UsageStats manifest state: \`${sourceProof.usageStatsManifestState}\``,
    `- Bridge state: \`${sourceProof.bridgeState}\``,
    '',
  ].join('\n');
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}

function readAppGameUsageContracts(source) {
  return {
    CapabilityGet: readLiteralObjectEntry(source, 'CapabilityGet'),
    ReplayBoundaryGet: readLiteralObjectEntry(source, 'ReplayBoundaryGet'),
    CapabilityReported: readLiteralObjectEntry(source, 'CapabilityReported'),
    ReplayBoundaryReported: readLiteralObjectEntry(source, 'ReplayBoundaryReported'),
  };
}

function readLiteralObjectEntry(source, propertyName) {
  const match = source.match(new RegExp(`${propertyName}:\\s*'([^']+)'`, 'u'));
  if (match === null) {
    throw new Error(`missing ${propertyName} contract literal in ${appGameUsageContractsPath}`);
  }
  return match[1];
}

function assertIncludes(value, expected, label) {
  if (!value.includes(expected)) {
    throw new Error(`${label}: missing ${expected}`);
  }
}

function assertNotIncludes(value, expected, label) {
  if (value.includes(expected)) {
    throw new Error(`${label}: unexpectedly contains ${expected}`);
  }
}

function assertEqual(actual, expected, label) {
  if (actual !== expected) {
    throw new Error(`${label}: expected ${expected}, received ${actual}`);
  }
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
