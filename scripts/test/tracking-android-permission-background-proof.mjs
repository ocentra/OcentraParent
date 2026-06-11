import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const testOutputDir = join(repoRoot, 'test-results', 'tracking-android-permission-background-proof');
const wp08ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '08-android-foreground-location-adapter');
const wp09ProofDir = join(
  repoRoot,
  'output',
  'tracking-plan-proof',
  '09-android-background-location-and-geofence-adapter'
);
const timestamp = '2026-06-05T23:46:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(testOutputDir, { recursive: true, force: true });
await mkdir(testOutputDir, { recursive: true });
await mkdir(wp08ProofDir, { recursive: true });
await mkdir(wp09ProofDir, { recursive: true });

runNpm(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
runNpm([
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'tracking-android-permission-background-proof',
]);

const proofModule = await importDist('tracking-android-permission-background-proof.js');
const readModel = proofModule.buildTrackingAndroidPermissionBackgroundProofReadModel(
  {
    generatedAt: timestamp,
    proofId: 'tracking-android-permission-background-proof',
    familyId: 'family-tracking-android-permission-background',
    childProfileId: 'child-profile-aarav',
    deviceId: 'device-aarav-android',
    deviceLabel: 'Aarav Android emulator',
    sourceProofRefs: [
      'output/tracking-plan-proof/08-android-foreground-location-adapter/03-runtime-location-evidence.json',
      'output/tracking-plan-proof/09-android-background-location-and-geofence-adapter/05-geofence-transition-proof.json',
      'docs/plans/tracking-plan/workpacks/08-android-foreground-location-adapter.md',
      'docs/plans/tracking-plan/workpacks/09-android-background-location-and-geofence-adapter.md',
    ],
  },
  permissionRows()
);

const proof = {
  proofMode: 'tracking-android-permission-background-proof',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: summarize(readModel),
  nonClaims: nonClaims(readModel),
  proofPaths: {
    source: 'packages/parent-domain/src/tracking-android-permission-background-proof.ts',
    test: 'packages/parent-domain/tests/tracking-android-permission-background-proof.test.ts',
    harness: 'scripts/test/tracking-android-permission-background-proof.mjs',
    evidence: 'test-results/tracking-android-permission-background-proof/proof.json',
    foregroundProofPack: 'output/tracking-plan-proof/08-android-foreground-location-adapter',
    backgroundProofPack: 'output/tracking-plan-proof/09-android-background-location-and-geofence-adapter',
  },
  readModel,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'tracking-android-permission-background-read-model.json'), readModel);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPacks(proof);

console.log('tracking-android-permission-background-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-android-permission-background-proof', 'proof.json')}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function permissionRows() {
  return [
    {
      rowId: 'tracking-android-foreground-permission',
      caseKind: 'foreground-permission-manual-required',
      source: 'android-emulator-foreground-proof',
      observedAt: timestamp,
      packageLaunchObserved: true,
      foregroundServiceObserved: true,
      foregroundPermissionRequested: false,
      foregroundLocationSampleCaptured: false,
      backgroundPermissionRequested: false,
      geofenceTransitionCount: 0,
      evidenceRefs: ['wp08-runtime-location-evidence-manual-required'],
      manualProofRefs: ['android-studio-foreground-permission-proof-plan'],
    },
    {
      rowId: 'tracking-android-foreground-sample',
      caseKind: 'foreground-sample-manual-required',
      source: 'physical-device-manual-plan',
      observedAt: timestamp,
      packageLaunchObserved: true,
      foregroundServiceObserved: true,
      foregroundPermissionRequested: false,
      foregroundLocationSampleCaptured: false,
      backgroundPermissionRequested: false,
      geofenceTransitionCount: 0,
      evidenceRefs: ['wp08-foreground-location-sample-absent'],
      manualProofRefs: ['physical-device-foreground-location-proof-plan'],
    },
    {
      rowId: 'tracking-android-background-permission',
      caseKind: 'background-permission-manual-required',
      source: 'android-emulator-background-proof',
      observedAt: timestamp,
      packageLaunchObserved: true,
      foregroundServiceObserved: true,
      foregroundPermissionRequested: false,
      foregroundLocationSampleCaptured: false,
      backgroundPermissionRequested: false,
      geofenceTransitionCount: 0,
      evidenceRefs: ['wp09-background-permission-manual-required'],
      manualProofRefs: ['android-settings-background-permission-proof-plan'],
    },
    {
      rowId: 'tracking-android-geofence-transition',
      caseKind: 'geofence-transition-manual-required',
      source: 'physical-device-manual-plan',
      observedAt: timestamp,
      packageLaunchObserved: true,
      foregroundServiceObserved: true,
      foregroundPermissionRequested: false,
      foregroundLocationSampleCaptured: false,
      backgroundPermissionRequested: false,
      geofenceTransitionCount: 0,
      evidenceRefs: ['wp09-geofence-transition-count-zero'],
      manualProofRefs: ['physical-device-geofence-transition-proof-plan'],
    },
  ];
}

function summarize(readModel) {
  return {
    rows: readModel.rows.length,
    foregroundPermissionManualRequiredCount: readModel.foregroundPermissionManualRequiredCount,
    foregroundSampleManualRequiredCount: readModel.foregroundSampleManualRequiredCount,
    backgroundPermissionManualRequiredCount: readModel.backgroundPermissionManualRequiredCount,
    geofenceTransitionManualRequiredCount: readModel.geofenceTransitionManualRequiredCount,
    runtimeEvidenceRefs: readModel.runtimeEvidenceRefs.length,
    claimStates: countBy(readModel.rows.map((row) => row.claimState)),
    caseKinds: countBy(readModel.rows.map((row) => row.caseKind)),
  };
}

function nonClaims(readModel) {
  return {
    foregroundPermissionClaimed: readModel.foregroundPermissionClaimed,
    foregroundLocationSampleClaimed: readModel.foregroundLocationSampleClaimed,
    backgroundPermissionClaimed: readModel.backgroundPermissionClaimed,
    backgroundLocationRuntimeClaimed: readModel.backgroundLocationRuntimeClaimed,
    geofenceRuntimeClaimed: readModel.geofenceRuntimeClaimed,
    physicalDeviceProofClaimed: readModel.physicalDeviceProofClaimed,
    deviceOwnerAuthorityClaimed: readModel.deviceOwnerAuthorityClaimed,
    notificationDeliveryClaimed: readModel.notificationDeliveryClaimed,
    providerDeliveryClaimed: readModel.providerDeliveryClaimed,
    productReadyAndroidTrackingClaimed: readModel.productReadyAndroidTrackingClaimed,
  };
}

function assertProof(proof) {
  if (
    proof.summary.rows !== 4 ||
    proof.summary.foregroundPermissionManualRequiredCount !== 1 ||
    proof.summary.foregroundSampleManualRequiredCount !== 1 ||
    proof.summary.backgroundPermissionManualRequiredCount !== 1 ||
    proof.summary.geofenceTransitionManualRequiredCount !== 1
  ) {
    throw new Error(`Unexpected Android permission/background summary: ${JSON.stringify(proof.summary)}`);
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(`Android permission/background proof overclaimed behavior: ${JSON.stringify(proof.nonClaims)}`);
  }
}

async function writeProofPacks(proof) {
  const foregroundRows = proof.readModel.rows.filter((row) =>
    ['foreground-permission-manual-required', 'foreground-sample-manual-required'].includes(row.caseKind)
  );
  const backgroundRows = proof.readModel.rows.filter((row) =>
    ['background-permission-manual-required', 'geofence-transition-manual-required'].includes(row.caseKind)
  );

  await writeFile(
    join(wp08ProofDir, '00-source-snapshot.md'),
    sourceSnapshot(proof, 'WP08 Android Foreground Location Adapter', 'foreground permission and sample gaps'),
    'utf8'
  );
  await writeFile(
    join(wp09ProofDir, '00-source-snapshot.md'),
    sourceSnapshot(
      proof,
      'WP09 Android Background Location And Geofence Adapter',
      'background permission and geofence gaps'
    ),
    'utf8'
  );
  await writeFile(
    join(wp08ProofDir, '02-platform-permission-proof.md'),
    permissionProofMarkdown('WP08 foreground location permission', foregroundRows),
    'utf8'
  );
  await writeFile(
    join(wp09ProofDir, '02-platform-permission-proof.md'),
    permissionProofMarkdown('WP09 background location permission', backgroundRows),
    'utf8'
  );
  await writeJson(join(wp08ProofDir, '03-runtime-location-evidence.json'), {
    proofMode: proof.proofMode,
    generatedAt: proof.generatedAt,
    rows: foregroundRows,
    nonClaims: proof.nonClaims,
  });
  await writeJson(join(wp09ProofDir, '05-geofence-transition-proof.json'), {
    proofMode: proof.proofMode,
    generatedAt: proof.generatedAt,
    rows: backgroundRows,
    geofenceTransitionCount: 0,
    nonClaims: proof.nonClaims,
  });
  await writeFile(
    join(wp08ProofDir, '15-manual-platform-proof.md'),
    manualProofMarkdown('WP08', foregroundRows),
    'utf8'
  );
  await writeFile(
    join(wp09ProofDir, '15-manual-platform-proof.md'),
    manualProofMarkdown('WP09', backgroundRows),
    'utf8'
  );
  await writeFile(join(wp08ProofDir, '16-validation-commands.log'), validationLog(proof), 'utf8');
  await writeFile(join(wp09ProofDir, '16-validation-commands.log'), validationLog(proof), 'utf8');
}

function sourceSnapshot(proof, title, scope) {
  return [
    `# ${title} Source Snapshot`,
    '',
    `- Branch: ${proof.branch}`,
    `- Base commit at generation: ${proof.baseCommitAtGeneration}`,
    '- Git status at proof generation:',
    '',
    '```text',
    proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
    '```',
    '',
    `- Scope: parent-domain Android ${scope} read model against existing emulator scaffold/manual proof plans.`,
    '- Source inspected: location/geofence feature doc, location/geofence expectations, platform expectations, WP08 workpack, and WP09 workpack.',
    '- Boundary: this proof keeps Android foreground permission, foreground sample, background permission, and geofence transitions manual-required until device/runtime artifacts exist.',
    '',
  ].join('\n');
}

function permissionProofMarkdown(title, rows) {
  return [
    `# ${title}`,
    '',
    ...rows.flatMap((row) => [
      `- ${row.rowId}: ${row.parentVisibleStatusToken}`,
      `  - Evidence refs: ${row.evidenceRefs.join(', ')}`,
      `  - Manual proof refs: ${row.manualProofRefs.join(', ')}`,
      `  - Missing proof refs: ${row.missingProofReasonRefs.join(', ')}`,
    ]),
    '',
  ].join('\n');
}

function manualProofMarkdown(workpack, rows) {
  return [
    `# ${workpack} Manual Platform Proof`,
    '',
    'Current status: manual_required.',
    '',
    ...rows.map((row) => `- ${row.caseKind}: ${row.missingProofReasonRefs.join(', ')}`),
    '',
    'Required before product claim: real Android device or emulator evidence that captures permission grant state, runtime location samples or geofence transitions, OS version, app build, logs, screenshots, and adb commands.',
    '',
  ].join('\n');
}

function validationLog(proof) {
  return proof.commands
    .map((command) =>
      [`$ ${command.command}`, command.stdout.trim(), command.stderr.trim()]
        .filter((line) => line.length > 0)
        .join('\n')
    )
    .join('\n\n');
}

function run(command, args) {
  const result = spawnSync(command, args, { cwd: repoRoot, encoding: 'utf8', shell: false });
  commands.push({
    command: [command, ...args].join(' '),
    status: result.status,
    stdout: result.stdout,
    stderr: result.stderr,
  });
  if (result.status !== 0) {
    throw new Error(
      `Command failed: ${[command, ...args].join(' ')}\nstdout:\n${result.stdout}\nstderr:\n${result.stderr}`
    );
  }
}

function gitOutput(args) {
  const result = spawnSync('git', args, { cwd: repoRoot, encoding: 'utf8', shell: false });
  if (result.status !== 0) {
    throw new Error(`git ${args.join(' ')} failed: ${result.stderr}`);
  }
  return result.stdout.trim();
}

async function writeJson(path, value) {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}

function countBy(values) {
  return values.reduce((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}

function runNpm(args, ...rest) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return run(command, commandArgs, ...rest);
}
