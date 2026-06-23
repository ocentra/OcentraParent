import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const testOutputDir = join(repoRoot, 'test-results', 'tracking-android-status-proof');
const proofDir = join(repoRoot, 'output', 'tracking-plan-proof', '10-android-battery-connectivity-and-status-adapter');
const timestamp = '2026-06-05T22:36:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(testOutputDir, { recursive: true, force: true });
await mkdir(testOutputDir, { recursive: true });
await mkdir(proofDir, { recursive: true });

runNpm(['run', 'build', '--workspace', '@ocentra-parent/schema-domain']);
runNpm(['run', 'build', '--workspace', '@ocentra-parent/tracking-domain']);
runNpm([
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/tracking-domain',
  '--',
  'tracking-android-status-proof.test.ts',
]);

const proofModule = await importDist('tracking-android-status-proof.js');
const readModel = proofModule.buildTrackingAndroidStatusProofReadModel(
  {
    generatedAt: timestamp,
    proofId: 'tracking-android-status-proof',
    familyId: 'family-tracking-android-status',
    childProfileId: 'child-profile-aarav',
    deviceId: 'device-aarav-android',
    deviceLabel: 'Aarav Android emulator',
    sourceProofRefs: [
      'output/tracking-plan-proof/10-android-battery-connectivity-and-status-adapter/04-device-status-proof.json',
      'test-results/tracking-plan-android-emulator-proof/proof.json',
      'test-results/tracking-android-physical-device-runtime-proof/proof.json',
      'docs/plans/tracking-plan/workpacks/10-android-battery-connectivity-and-status-adapter.md',
    ],
  },
  androidStatusRows()
);

const proof = {
  proofMode: 'tracking-android-status-proof',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: summarize(readModel),
  nonClaims: nonClaims(readModel),
  proofPaths: {
    source: 'packages/schema-domain/src/tracking-android-status-proof.ts',
    test: 'packages/tracking-domain/tests/contract/tracking-android-status-proof.test.ts',
    harness: 'scripts/test/tracking-android-status-proof.mjs',
    evidence: 'test-results/tracking-android-status-proof/proof.json',
    trackingProofPack: 'output/tracking-plan-proof/10-android-battery-connectivity-and-status-adapter',
  },
  readModel,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'tracking-android-status-read-model.json'), readModel);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(proofDir, proof);

console.log('tracking-android-status-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-android-status-proof', 'proof.json')}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'schema-domain', 'dist', name)).href);
}

function androidStatusRows() {
  return [
    {
      rowId: 'tracking-android-status-low-power',
      caseKind: 'low-power-degraded',
      source: 'emulator-battery-dump',
      observedAt: timestamp,
      batteryPercent: 14,
      charging: false,
      lowPowerMode: true,
      appProcessRunning: true,
      appRestartObserved: false,
      pendingUploadCount: 0,
      evidenceRefs: ['android-battery-dumpsys-low-power'],
      auditRefs: ['tracking-android-status-audit-low-power'],
    },
    {
      rowId: 'tracking-android-status-restarted',
      caseKind: 'app-killed-restarted',
      source: 'emulator-activity-manager',
      observedAt: timestamp,
      batteryPercent: 39,
      charging: false,
      lowPowerMode: false,
      appProcessRunning: true,
      appRestartObserved: true,
      pendingUploadCount: 0,
      evidenceRefs: ['android-activity-manager-force-stop-and-restart'],
      auditRefs: ['tracking-android-status-audit-restart'],
    },
    {
      rowId: 'tracking-android-status-pending-upload',
      caseKind: 'pending-upload-auditable',
      source: 'query-store-pending-upload',
      observedAt: timestamp,
      batteryPercent: 21,
      charging: false,
      lowPowerMode: false,
      appProcessRunning: true,
      appRestartObserved: false,
      pendingUploadCount: 3,
      evidenceRefs: ['android-query-store-pending-upload-count'],
      auditRefs: ['tracking-android-status-audit-pending-upload'],
    },
    {
      rowId: 'tracking-android-status-physical-device-status',
      caseKind: 'physical-status-observed',
      source: 'physical-device-battery-connectivity-dump',
      observedAt: timestamp,
      batteryPercent: 83,
      charging: false,
      lowPowerMode: false,
      appProcessRunning: true,
      appRestartObserved: false,
      pendingUploadCount: 0,
      evidenceRefs: [
        'test-results/tracking-android-physical-device-runtime-proof/07-battery.txt',
        'test-results/tracking-android-physical-device-runtime-proof/08-connectivity.txt',
      ],
      auditRefs: ['tracking-android-status-audit-physical-status-observed'],
    },
    {
      rowId: 'tracking-android-status-manual-required',
      caseKind: 'manual-required',
      source: 'manual-platform-plan',
      observedAt: timestamp,
      batteryPercent: null,
      charging: false,
      lowPowerMode: false,
      appProcessRunning: false,
      appRestartObserved: false,
      pendingUploadCount: 0,
      evidenceRefs: ['android-manual-platform-proof-plan'],
      auditRefs: ['tracking-android-status-audit-manual-required'],
    },
  ];
}

function summarize(readModel) {
  return {
    rows: readModel.rows.length,
    lowPowerDegradedCount: readModel.lowPowerDegradedCount,
    appRestartObservedCount: readModel.appRestartObservedCount,
    pendingUploadAuditableCount: readModel.pendingUploadAuditableCount,
    physicalStatusObservedCount: readModel.physicalStatusObservedCount,
    manualRequiredCount: readModel.manualRequiredCount,
    runtimeEvidenceRefs: readModel.runtimeEvidenceRefs.length,
    physicalDeviceStatusEvidenceObserved: readModel.physicalDeviceStatusEvidenceObserved,
    claimStates: countBy(readModel.rows.map((row) => row.claimState)),
    caseKinds: countBy(readModel.rows.map((row) => row.caseKind)),
  };
}

function nonClaims(readModel) {
  return {
    foregroundLocationClaimed: readModel.foregroundLocationClaimed,
    backgroundLocationRuntimeClaimed: readModel.backgroundLocationRuntimeClaimed,
    geofenceRuntimeClaimed: readModel.geofenceRuntimeClaimed,
    physicalDeviceProofClaimed: readModel.physicalDeviceProofClaimed,
    notificationDeliveryClaimed: readModel.notificationDeliveryClaimed,
    deviceOwnerAuthorityClaimed: readModel.deviceOwnerAuthorityClaimed,
    productionUploadWorkerClaimed: readModel.productionUploadWorkerClaimed,
    productReadyAndroidTrackingClaimed: readModel.productReadyAndroidTrackingClaimed,
  };
}

function assertProof(proof) {
  if (
    proof.summary.rows !== 5 ||
    proof.summary.lowPowerDegradedCount !== 1 ||
    proof.summary.appRestartObservedCount !== 1 ||
    proof.summary.pendingUploadAuditableCount !== 1 ||
    proof.summary.physicalStatusObservedCount !== 1 ||
    proof.summary.manualRequiredCount !== 1
  ) {
    throw new Error(`Unexpected Android status summary: ${JSON.stringify(proof.summary)}`);
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(`Android status proof overclaimed runtime behavior: ${JSON.stringify(proof.nonClaims)}`);
  }
}

async function writeProofPack(path, proof) {
  await writeFile(
    join(path, '00-source-snapshot.md'),
    [
      '# WP10 Android Battery Connectivity And Status Adapter Source Snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Base commit at generation: ${proof.baseCommitAtGeneration}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
      '- Scope: tracking-domain Android status read model for low-power degradation, killed/restarted audit rows, pending-upload auditability, Samsung S9 physical battery/connectivity/status evidence, and manual-required platform gaps.',
      '- Source inspected: location/geofence feature doc, location/geofence expectations, platform expectations, tracking settings inventory, V0.5 platform deep dive, and WP10 workpack.',
      '- Boundary: this proof extends emulator/local plus Samsung S9 status evidence only; it does not claim foreground location samples, background location runtime, geofence transitions, offline radio behavior, notification delivery, device-owner authority, physical-device behavior, or product-ready Android tracking.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeJson(join(path, '04-device-status-proof.json'), {
    rows: proof.readModel.rows.map((row) => ({
      rowId: row.rowId,
      caseKind: row.caseKind,
      source: row.source,
      claimState: row.claimState,
      observedAt: row.observedAt,
      batteryPercent: row.batteryPercent,
      charging: row.charging,
      lowPowerMode: row.lowPowerMode,
      appProcessRunning: row.appProcessRunning,
      appRestartObserved: row.appRestartObserved,
      pendingUploadCount: row.pendingUploadCount,
      evidenceRefs: row.evidenceRefs,
      auditRefs: row.auditRefs,
      parentVisibleStatusToken: row.parentVisibleStatusToken,
      manualRequiredReasonRefs: row.manualRequiredReasonRefs,
    })),
    summary: proof.summary,
    nonClaims: proof.nonClaims,
  });
  await writeJson(join(path, '17-status-gap-proof.json'), {
    proofMode: proof.proofMode,
    generatedAt: proof.generatedAt,
    lowPower: proof.readModel.rows.find((row) => row.caseKind === 'low-power-degraded'),
    killedRestarted: proof.readModel.rows.find((row) => row.caseKind === 'app-killed-restarted'),
    pendingUpload: proof.readModel.rows.find((row) => row.caseKind === 'pending-upload-auditable'),
    physicalStatus: proof.readModel.rows.find((row) => row.caseKind === 'physical-status-observed'),
    manualRequired: proof.readModel.rows.find((row) => row.caseKind === 'manual-required'),
    nonClaims: proof.nonClaims,
  });
  await writeFile(
    join(path, '16-validation-commands.log'),
    proof.commands
      .map((command) =>
        [`$ ${command.command}`, command.stdout.trim(), command.stderr.trim()]
          .filter((line) => line.length > 0)
          .join('\n')
      )
      .join('\n\n'),
    'utf8'
  );
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
