import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const testOutputDir = join(repoRoot, 'test-results', 'tracking-ios-location-manual-required-proof');
const wp11ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '11-ios-core-location-foreground-adapter');
const wp12ProofDir = join(
  repoRoot,
  'output',
  'tracking-plan-proof',
  '12-ios-background-region-significant-change-adapter'
);
const timestamp = '2026-06-06T02:30:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(testOutputDir, { recursive: true, force: true });
await mkdir(testOutputDir, { recursive: true });
await mkdir(wp11ProofDir, { recursive: true });
await mkdir(wp12ProofDir, { recursive: true });

runNpm(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
runNpm([
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'tracking-ios-location-manual-required-proof',
]);

const proofModule = await importDist('tracking-ios-location-manual-required-proof.js');
const readModel = proofModule.buildTrackingIosLocationManualRequiredProofReadModel(
  {
    generatedAt: timestamp,
    proofId: 'tracking-ios-location-manual-required-proof',
    familyId: 'family-tracking-ios-location-manual-required',
    childProfileId: 'child-profile-maya',
    deviceId: 'device-maya-ios',
    deviceLabel: 'Maya iOS simulator',
    sourceProofRefs: [
      'output/tracking-plan-proof/11-ios-core-location-foreground-adapter/18-ios-simulator-proof.json',
      'output/tracking-plan-proof/12-ios-background-region-significant-change-adapter/18-ios-simulator-proof.json',
      'docs/plans/tracking-plan/workpacks/11-ios-core-location-foreground-adapter.md',
      'docs/plans/tracking-plan/workpacks/12-ios-background-region-significant-change-adapter.md',
    ],
  },
  iosLocationRows()
);

const proof = {
  proofMode: 'tracking-ios-location-manual-required-proof',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: summarize(readModel),
  nonClaims: nonClaims(readModel),
  proofPaths: {
    source: 'packages/parent-domain/src/tracking-ios-location-manual-required-proof.ts',
    test: 'packages/parent-domain/tests/tracking-ios-location-manual-required-proof.test.ts',
    harness: 'scripts/test/tracking-ios-location-manual-required-proof.mjs',
    evidence: 'test-results/tracking-ios-location-manual-required-proof/proof.json',
    foregroundProofPack: 'output/tracking-plan-proof/11-ios-core-location-foreground-adapter',
    backgroundProofPack: 'output/tracking-plan-proof/12-ios-background-region-significant-change-adapter',
  },
  readModel,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'tracking-ios-location-manual-required-read-model.json'), readModel);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPacks(proof);

console.log('tracking-ios-location-manual-required-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-ios-location-manual-required-proof', 'proof.json')}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function iosLocationRows() {
  return [
    iosLocationRow({
      rowId: 'tracking-ios-when-in-use-authorization',
      caseKind: 'when-in-use-authorization-manual-required',
      source: 'ios-simulator-package-proof',
      evidenceRefs: ['wp11-when-in-use-authorization-manual-required'],
      manualProofRefs: ['xcode-ios-when-in-use-authorization-proof-plan'],
    }),
    iosLocationRow({
      rowId: 'tracking-ios-foreground-sample',
      caseKind: 'foreground-sample-manual-required',
      source: 'physical-device-manual-plan',
      evidenceRefs: ['wp11-foreground-location-sample-absent'],
      manualProofRefs: ['physical-device-ios-foreground-location-proof-plan'],
    }),
    iosLocationRow({
      rowId: 'tracking-ios-degraded-location-state',
      caseKind: 'denied-restricted-services-disabled-manual-required',
      source: 'ios-simulator-manual-plan',
      evidenceRefs: ['wp11-denied-restricted-services-disabled-state-absent'],
      manualProofRefs: ['ios-simulator-denied-restricted-services-disabled-proof-plan'],
    }),
    iosLocationRow({
      rowId: 'tracking-ios-always-authorization',
      caseKind: 'always-authorization-manual-required',
      source: 'apple-entitlement-manual-plan',
      evidenceRefs: ['wp12-always-authorization-manual-required'],
      manualProofRefs: ['apple-always-authorization-entitlement-proof-plan'],
    }),
    iosLocationRow({
      rowId: 'tracking-ios-region-transition',
      caseKind: 'region-transition-manual-required',
      source: 'physical-device-manual-plan',
      evidenceRefs: ['wp12-region-transition-count-zero'],
      manualProofRefs: ['physical-device-ios-region-transition-proof-plan'],
    }),
    iosLocationRow({
      rowId: 'tracking-ios-significant-change-visit',
      caseKind: 'significant-change-visit-manual-required',
      source: 'physical-device-manual-plan',
      evidenceRefs: ['wp12-significant-change-visit-count-zero'],
      manualProofRefs: ['physical-device-ios-significant-change-visit-proof-plan'],
    }),
    iosLocationRow({
      rowId: 'tracking-ios-background-terminated-relaunch',
      caseKind: 'background-terminated-relaunch-manual-required',
      source: 'physical-device-manual-plan',
      evidenceRefs: ['wp12-background-terminated-relaunch-absent'],
      manualProofRefs: ['physical-device-ios-background-terminated-relaunch-proof-plan'],
    }),
  ];
}

function iosLocationRow(input) {
  return {
    ...input,
    observedAt: timestamp,
    simulatorPackageBuilt: true,
    simulatorLaunchObserved: true,
    whenInUseAuthorizationObserved: false,
    foregroundLocationSampleCaptured: false,
    deniedRestrictedStateCaptured: false,
    locationServicesDisabledStateCaptured: false,
    alwaysAuthorizationObserved: false,
    regionTransitionCount: 0,
    significantChangeEventCount: 0,
    visitEventCount: 0,
    backgroundDeliveryObserved: false,
    terminatedRelaunchObserved: false,
    entitlementProofObserved: false,
  };
}

function summarize(readModel) {
  return {
    rows: readModel.rows.length,
    whenInUseAuthorizationManualRequiredCount: readModel.whenInUseAuthorizationManualRequiredCount,
    foregroundSampleManualRequiredCount: readModel.foregroundSampleManualRequiredCount,
    degradedStateManualRequiredCount: readModel.degradedStateManualRequiredCount,
    alwaysAuthorizationManualRequiredCount: readModel.alwaysAuthorizationManualRequiredCount,
    regionTransitionManualRequiredCount: readModel.regionTransitionManualRequiredCount,
    significantChangeVisitManualRequiredCount: readModel.significantChangeVisitManualRequiredCount,
    backgroundTerminatedRelaunchManualRequiredCount: readModel.backgroundTerminatedRelaunchManualRequiredCount,
    runtimeEvidenceRefs: readModel.runtimeEvidenceRefs.length,
    claimStates: countBy(readModel.rows.map((row) => row.claimState)),
    caseKinds: countBy(readModel.rows.map((row) => row.caseKind)),
  };
}

function nonClaims(readModel) {
  return {
    whenInUseAuthorizationClaimed: readModel.whenInUseAuthorizationClaimed,
    foregroundLocationSampleClaimed: readModel.foregroundLocationSampleClaimed,
    deniedRestrictedStateClaimed: readModel.deniedRestrictedStateClaimed,
    servicesDisabledStateClaimed: readModel.servicesDisabledStateClaimed,
    alwaysAuthorizationClaimed: readModel.alwaysAuthorizationClaimed,
    regionMonitoringClaimed: readModel.regionMonitoringClaimed,
    significantChangeClaimed: readModel.significantChangeClaimed,
    visitEventClaimed: readModel.visitEventClaimed,
    backgroundLocationDeliveryClaimed: readModel.backgroundLocationDeliveryClaimed,
    terminatedRelaunchClaimed: readModel.terminatedRelaunchClaimed,
    entitlementProofClaimed: readModel.entitlementProofClaimed,
    notificationDeliveryClaimed: readModel.notificationDeliveryClaimed,
    providerDeliveryClaimed: readModel.providerDeliveryClaimed,
    physicalDeviceProofClaimed: readModel.physicalDeviceProofClaimed,
    authorityProofClaimed: readModel.authorityProofClaimed,
    productReadyIosTrackingClaimed: readModel.productReadyIosTrackingClaimed,
  };
}

function assertProof(proof) {
  if (
    proof.summary.rows !== 7 ||
    proof.summary.whenInUseAuthorizationManualRequiredCount !== 1 ||
    proof.summary.foregroundSampleManualRequiredCount !== 1 ||
    proof.summary.degradedStateManualRequiredCount !== 1 ||
    proof.summary.alwaysAuthorizationManualRequiredCount !== 1 ||
    proof.summary.regionTransitionManualRequiredCount !== 1 ||
    proof.summary.significantChangeVisitManualRequiredCount !== 1 ||
    proof.summary.backgroundTerminatedRelaunchManualRequiredCount !== 1
  ) {
    throw new Error(`Unexpected iOS manual-required summary: ${JSON.stringify(proof.summary)}`);
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(`iOS manual-required proof overclaimed behavior: ${JSON.stringify(proof.nonClaims)}`);
  }
}

async function writeProofPacks(proof) {
  const foregroundRows = proof.readModel.rows.filter((row) =>
    [
      'when-in-use-authorization-manual-required',
      'foreground-sample-manual-required',
      'denied-restricted-services-disabled-manual-required',
    ].includes(row.caseKind)
  );
  const backgroundRows = proof.readModel.rows.filter((row) =>
    [
      'always-authorization-manual-required',
      'region-transition-manual-required',
      'significant-change-visit-manual-required',
      'background-terminated-relaunch-manual-required',
    ].includes(row.caseKind)
  );

  await writeFile(
    join(wp11ProofDir, '00-source-snapshot.md'),
    sourceSnapshot(proof, 'WP11 iOS Core Location Foreground Adapter', 'foreground authorization and sample gaps'),
    'utf8'
  );
  await writeFile(
    join(wp12ProofDir, '00-source-snapshot.md'),
    sourceSnapshot(
      proof,
      'WP12 iOS Background Region Significant-Change Adapter',
      'Always authorization, region, significant-change, visits, and background relaunch gaps'
    ),
    'utf8'
  );
  await writeFile(
    join(wp11ProofDir, '02-platform-permission-proof.md'),
    permissionProofMarkdown('WP11 When In Use and degraded location states', foregroundRows),
    'utf8'
  );
  await writeFile(
    join(wp12ProofDir, '02-platform-permission-proof.md'),
    permissionProofMarkdown('WP12 Always/background location states', backgroundRows),
    'utf8'
  );
  await writeJson(join(wp11ProofDir, '03-runtime-location-evidence.json'), {
    proofMode: proof.proofMode,
    generatedAt: proof.generatedAt,
    rows: foregroundRows,
    nonClaims: proof.nonClaims,
  });
  await writeJson(join(wp12ProofDir, '05-geofence-transition-proof.json'), {
    proofMode: proof.proofMode,
    generatedAt: proof.generatedAt,
    rows: backgroundRows,
    regionTransitionCount: 0,
    significantChangeEventCount: 0,
    visitEventCount: 0,
    nonClaims: proof.nonClaims,
  });
  await writeFile(
    join(wp11ProofDir, '15-manual-platform-proof.md'),
    manualProofMarkdown('WP11', foregroundRows),
    'utf8'
  );
  await writeFile(
    join(wp12ProofDir, '15-manual-platform-proof.md'),
    manualProofMarkdown('WP12', backgroundRows),
    'utf8'
  );
  await writeFile(join(wp11ProofDir, '16-validation-commands.log'), validationLog(proof), 'utf8');
  await writeFile(join(wp12ProofDir, '16-validation-commands.log'), validationLog(proof), 'utf8');
  await writeJson(join(wp11ProofDir, '19-ios-location-manual-required-proof.json'), proof);
  await writeJson(join(wp12ProofDir, '19-ios-location-manual-required-proof.json'), proof);
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
    `- Scope: parent-domain iOS ${scope} read model against existing simulator package/manual proof plans.`,
    '- Source inspected: location/geofence feature doc, location/geofence expectations, platform expectations, WP11 workpack, and WP12 workpack.',
    '- Boundary: this proof keeps Core Location authorization, sample, region, background, entitlement, notification, physical-device, authority, and product-ready behavior manual-required until matching artifacts exist.',
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
    'Required before product claim: real iOS simulator or physical-device evidence that captures authorization state, runtime Core Location samples or region/significant-change/visit events, OS version, app build, logs, screenshots, and xcrun/simctl or Xcode commands.',
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
