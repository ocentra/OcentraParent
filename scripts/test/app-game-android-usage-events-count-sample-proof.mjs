import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'app-game-android-usage-events-count-sample-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const appGameProofDir = join(
  repoRoot,
  'output',
  'app-game-plan-proof',
  '200-app-game-android-usage-events-count-sample'
);
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
      'app-game-android-usage-events-count-sample',
    ])
  );
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/schema-domain']));
  await runCommand(...npmCommand(['run', 'release:package:android']));

  const sourceProof = await assertAndroidSourceProof();
  const contractModule = await import(
    pathToFileURL(join(repoRoot, 'packages', 'schema-domain', 'dist', 'app-game-android-usage-events-count-sample.js'))
      .href
  );
  const readModel = contractModule.createAppGameAndroidUsageEventsCountSampleReadModel({
    sampleState: 'sample-observed',
    sampleLookbackMillis: 900000,
    sampleEventCount: 9,
    foregroundEventCount: 3,
    checkedAt: '2026-06-08T19:50:00.000Z',
  });
  const summary = contractModule.summarizeAppGameAndroidUsageEventsCountSampleReadModel(readModel);

  assertEqual(summary.sampleState, 'sample-observed', 'sample state');
  assertEqual(summary.sampleEventCount, 9, 'sample count');
  assertEqual(summary.foregroundEventCount, 3, 'foreground count');
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
      contract: 'packages/schema-domain/src/app-game-android-usage-events-count-sample.ts',
      contractTest: 'packages/app-game-domain/tests/unit/app-game-android-usage-events-count-sample.test.ts',
    },
    claimsProved: [
      'Android package source can query UsageEvents and reduce results to counts only',
      'The centralized count-only sample contract accepts UsageEvents samples without raw row or package-name custody',
      'Android debug package compiles with the count-only sampler wired into MainActivity',
    ],
    claimsNotProved: [
      'Physical child-device UsageStats settings grant',
      'Live device sample observation from the current package install',
      'Raw UsageEvents rows, package names, class names, or activity custody',
      'Adapter dispatch, platform enforcement, provider delivery, or child-device delivery',
    ],
  };

  await writeJson(proofPath, proof);
  await writeJson(join(appGameProofDir, 'proof.json'), proof);
  await writeFile(join(appGameProofDir, '00-source-snapshot.md'), sourceSnapshot(sourceProof));
  await writeFile(join(appGameProofDir, '10-validation-commands.log'), `${commands.join('\n')}\n`);

  console.log('app-game-android-usage-events-count-sample-proof-ok');
  console.log(`evidence=${relativePath(proofPath)}`);
}

async function assertAndroidSourceProof() {
  const preflightPath =
    'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidUsageEventsRuntimePreflight.java';
  const activityPath = 'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/MainActivity.java';
  const preflight = await readRepoFile(preflightPath);
  const activity = await readRepoFile(activityPath);

  assertIncludes(preflight, 'UsageEvents usageEvents = usageStatsManager.queryEvents', 'UsageEvents query');
  assertIncludes(preflight, 'usageEvents.getNextEvent(event)', 'UsageEvents iteration');
  assertIncludes(preflight, 'totalEventCount += 1', 'total count only');
  assertIncludes(preflight, 'foregroundEventCount += 1', 'foreground count only');
  assertIncludes(preflight, 'status.putBoolean("rawUsageEventsStored", false)', 'raw UsageEvents non-claim');
  assertIncludes(preflight, 'status.putBoolean("packageNamesStored", false)', 'package-name non-claim');
  assertIncludes(preflight, 'status.putBoolean("rawActivityRowsStored", false)', 'raw activity non-claim');
  assertIncludes(preflight, 'status.putBoolean("runtimeCollectionClaimed", false)', 'collection non-claim');
  assertNotIncludes(preflight, 'event.getPackageName()', 'sample package-name custody');
  assertIncludes(
    activity,
    'AppGameAndroidUsageEventsRuntimePreflight.FIELD_SAMPLE_STATE',
    'activity sample state wiring'
  );

  return {
    preflight: preflightPath,
    activity: activityPath,
    sampleCustody: 'count-only',
    rawUsageEventsStored: false,
    packageNamesStored: false,
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
    '# WP200 Android UsageEvents count sample source snapshot',
    '',
    `- Preflight: \`${sourceProof.preflight}\``,
    `- Activity wiring: \`${sourceProof.activity}\``,
    `- Sample custody: \`${sourceProof.sampleCustody}\``,
    `- Raw UsageEvents stored: \`${sourceProof.rawUsageEventsStored}\``,
    `- Package names stored: \`${sourceProof.packageNamesStored}\``,
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

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
