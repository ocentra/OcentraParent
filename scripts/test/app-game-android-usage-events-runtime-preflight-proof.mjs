import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'app-game-android-usage-events-runtime-preflight-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const appGameProofDir = join(
  repoRoot,
  'output',
  'app-game-plan-proof',
  '199-app-game-android-usage-events-runtime-preflight'
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
    'app-game-android-usage-events-runtime-preflight',
  ]);
  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  await runCommand('cmd', ['/c', 'npm', 'run', 'release:package:android']);

  const sourceProof = await assertAndroidSourceProof();
  const contractModule = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-android-usage-events-runtime-preflight.js')
    ).href
  );
  const readModel = contractModule.createAppGameAndroidUsageEventsRuntimePreflightReadModel({
    permissionCheckState: 'settings-grant-required',
    usageStatsServiceState: 'service-visible',
    checkedAt: '2026-06-08T19:35:00.000Z',
  });
  const summary = contractModule.summarizeAppGameAndroidUsageEventsRuntimePreflightReadModel(readModel);

  assertEqual(summary.permissionCheckState, 'settings-grant-required', 'UsageStats permission preflight');
  assertEqual(summary.runtimeCollectionState, 'collection-blocked-before-runtime-proof', 'runtime collection state');
  assertEqual(summary.runtimeCollectionClaimed, false, 'runtime collection claim');
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
      androidPreflight:
        'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidUsageEventsRuntimePreflight.java',
      androidActivity: 'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/MainActivity.java',
      androidManifest: 'platforms/android/agent/app/src/main/AndroidManifest.xml',
      contract: 'packages/parent-domain/src/app-game-android-usage-events-runtime-preflight.ts',
      contractTest: 'packages/parent-domain/tests/app-game-android-usage-events-runtime-preflight.test.ts',
    },
    claimsProved: [
      'Android package source checks UsageStats AppOps readiness through a package-local preflight',
      'MainActivity surfaces the UsageStats permission preflight state without raw UsageEvents rows',
      'Parent-domain blocks runtime collection until a real runtime sample proof exists',
    ],
    claimsNotProved: [
      'UsageStats settings grant on the child device',
      'Runtime UsageEvents sample collection from the package',
      'Raw UsageEvents rows or package-name custody',
      'Android adapter dispatch, platform enforcement, child delivery, or provider delivery',
      'Device Owner/Profile Owner authority or Play policy proof',
    ],
  };

  await writeJson(proofPath, proof);
  await writeJson(join(appGameProofDir, 'proof.json'), proof);
  await writeFile(join(appGameProofDir, '00-source-snapshot.md'), sourceSnapshot(sourceProof));
  await writeFile(join(appGameProofDir, '10-validation-commands.log'), `${commands.join('\n')}\n`);

  console.log('app-game-android-usage-events-runtime-preflight-proof-ok');
  console.log(`evidence=${relativePath(proofPath)}`);
}

async function assertAndroidSourceProof() {
  const preflightPath =
    'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidUsageEventsRuntimePreflight.java';
  const activityPath = 'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/MainActivity.java';
  const manifestPath = 'platforms/android/agent/app/src/main/AndroidManifest.xml';
  const preflight = await readRepoFile(preflightPath);
  const activity = await readRepoFile(activityPath);
  const manifest = await readRepoFile(manifestPath);

  assertIncludes(preflight, 'AppOpsManager.OPSTR_GET_USAGE_STATS', 'UsageStats AppOps check');
  assertIncludes(preflight, 'Context.USAGE_STATS_SERVICE', 'UsageStats service check');
  assertIncludes(preflight, 'app-game.android.usage-events.runtime-preflight.get', 'runtime preflight command');
  assertIncludes(preflight, 'app-game.android.usage-events.runtime-preflight.reported', 'runtime preflight event');
  assertIncludes(preflight, 'android-usage-events-runtime-preflight-ref', 'runtime preflight proof ref');
  assertIncludes(preflight, 'android-usage-stats-appops-preflight-ref', 'AppOps proof ref');
  assertIncludes(preflight, 'android-usage-events-runtime-sample-not-proved', 'runtime sample gap');
  assertIncludes(preflight, 'status.putBoolean("rawUsageEventsStored", false)', 'raw UsageEvents non-claim');
  assertIncludes(preflight, 'status.putBoolean("runtimeCollectionClaimed", false)', 'runtime collection non-claim');
  assertIncludes(preflight, 'status.putBoolean("platformEnforcementClaimed", false)', 'enforcement non-claim');
  assertIncludes(
    activity,
    'AppGameAndroidUsageEventsRuntimePreflight.createRuntimePreflightBundle(this)',
    'activity runtime preflight wiring'
  );
  assertIncludes(
    activity,
    'AppGameAndroidUsageEventsRuntimePreflight.FIELD_PERMISSION_CHECK_STATE',
    'activity preflight status wiring'
  );
  assertNotIncludes(manifest, 'android.permission.PACKAGE_USAGE_STATS', 'UsageStats privileged permission');

  return {
    preflight: preflightPath,
    activity: activityPath,
    manifest: manifestPath,
    usageStatsManifestState: 'not-declared-by-design',
    preflightState: 'settings-grant-required',
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
    '# WP199 Android UsageEvents runtime preflight source snapshot',
    '',
    `- Preflight: \`${sourceProof.preflight}\``,
    `- Activity wiring: \`${sourceProof.activity}\``,
    `- Manifest: \`${sourceProof.manifest}\``,
    `- UsageStats manifest state: \`${sourceProof.usageStatsManifestState}\``,
    `- Preflight state: \`${sourceProof.preflightState}\``,
    '',
  ].join('\n');
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
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
