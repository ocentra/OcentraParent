import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const testOutputDir = join(repoRoot, 'test-results', 'tracking-temporary-live-readiness-proof');
const proofDir = join(repoRoot, 'output', 'tracking-plan-proof', '28-temporary-live-tracking-mode');
const timestamp = '2026-06-05T14:10:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(testOutputDir, { recursive: true, force: true });
await mkdir(testOutputDir, { recursive: true });
await mkdir(proofDir, { recursive: true });

run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
run('cmd', [
  '/c',
  'npm',
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'tracking-temporary-live-readiness-proof',
  'tracking-location-policy',
]);

const tracking = await importDist('tracking-location-policy.js');
const proofModule = await importDist('tracking-temporary-live-readiness-proof.js');
const sourceReadModel = tracking.TrackingLocationPolicyReadModelSchema.parse(sourceTrackingReadModel(tracking));
const readModel = proofModule.buildTrackingTemporaryLiveReadinessReadModel(
  {
    generatedAt: timestamp,
    proofId: 'tracking-temporary-live-readiness-proof',
    sourceTrackingReadModelRef: 'tracking-location-policy-temporary-live-proof',
    sourceContractRefs: ['tracking-location-policy', 'location-geofence-device-status', 'temporary-live-tracking-mode'],
  },
  sourceReadModel
);

const proof = {
  proofMode: 'tracking-temporary-live-readiness-proof',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: summarize(readModel),
  nonClaims: nonClaims(readModel),
  proofPaths: {
    source: 'packages/parent-domain/src/tracking-temporary-live-readiness-proof.ts',
    test: 'packages/parent-domain/tests/tracking-temporary-live-readiness-proof.test.ts',
    harness: 'scripts/test/tracking-temporary-live-readiness-proof.mjs',
    evidence: 'test-results/tracking-temporary-live-readiness-proof/proof.json',
    trackingProofPack: 'output/tracking-plan-proof/28-temporary-live-tracking-mode',
  },
  readModel,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'tracking-temporary-live-readiness-read-model.json'), readModel);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(proofDir, proof);

console.log('tracking-temporary-live-readiness-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-temporary-live-readiness-proof', 'proof.json')}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function sourceTrackingReadModel(tracking) {
  return {
    schemaVersion: tracking.TrackingPolicySchemaVersion,
    generatedAt: timestamp,
    rules: [],
    decisions: [],
    acknowledgements: [],
    checkInRequests: [],
    checkInResponses: [],
    aiRoutes: [],
    aiResults: [],
    alerts: [],
    escalations: [],
    temporaryLiveGrants: [
      grant(tracking, 'tracking-live-ready', 'requested', '2026-06-05T14:35:00.000Z', 1200, true, true),
      grant(tracking, 'tracking-live-active', 'active', '2026-06-05T14:20:00.000Z', 900, true, true),
      grant(tracking, 'tracking-live-expired', 'active', '2026-06-05T14:05:00.000Z', 300, true, true),
      grant(tracking, 'tracking-live-revoked', 'revoked', '2026-06-05T14:25:00.000Z', 1200, true, true),
      grant(tracking, 'tracking-live-unavailable', 'unavailable', '2026-06-05T14:25:00.000Z', 1200, true, true),
      grant(tracking, 'tracking-live-too-long', 'requested', '2026-06-05T16:10:00.000Z', 7200, true, true),
    ],
    missingDeviceCases: [],
    platformProofRoutes: [
      {
        schemaVersion: tracking.TrackingPolicySchemaVersion,
        platform: 'android',
        foregroundLocation: 'real-device-required',
        backgroundLocation: 'background-permission-required',
        geofence: 'real-device-required',
        deviceStatus: 'manual-required',
        proofArtifactRefs: [],
        manualRequiredReason: 'android-physical-device-required',
      },
    ],
  };
}

function grant(tracking, grantId, state, expiresAt, durationSeconds, parentApproved, childDisclosureRequired) {
  return {
    schemaVersion: tracking.TrackingPolicySchemaVersion,
    grantId,
    state,
    requestedAt: '2026-06-05T14:00:00.000Z',
    expiresAt,
    durationSeconds,
    parentApproved,
    childDisclosureRequired,
    auditRefs: [`${grantId}-audit`],
  };
}

function summarize(readModel) {
  return {
    rows: readModel.rows.length,
    readyToStartCount: readModel.readyToStartCount,
    activeCount: readModel.activeCount,
    expiredAutoStopCount: readModel.expiredAutoStopCount,
    manualRequiredCount: readModel.manualRequiredCount,
    revokedOrDeniedCount: readModel.revokedOrDeniedCount,
    readinessStates: countBy(readModel.rows.map((row) => row.readinessState)),
    retentionActions: countBy(readModel.rows.map((row) => row.retentionAction)),
    platformRuntimeProofStates: countBy(readModel.rows.map((row) => row.platformRuntimeProofState)),
    batteryStatusProofStates: countBy(readModel.rows.map((row) => row.batteryStatusProofState)),
  };
}

function nonClaims(readModel) {
  return {
    liveLocationRuntimeClaimed: readModel.liveLocationRuntimeClaimed,
    physicalDeviceProofClaimed: readModel.physicalDeviceProofClaimed,
    backgroundLocationClaimed: readModel.backgroundLocationClaimed,
    batteryRuntimeClaimed: readModel.batteryRuntimeClaimed,
    childDisclosureUiClaimed: readModel.childDisclosureUiClaimed,
    parentLiveUiClaimed: readModel.parentLiveUiClaimed,
    remoteSyncClaimed: readModel.remoteSyncClaimed,
    providerDeliveryClaimed: readModel.providerDeliveryClaimed,
    productClaimReady: readModel.productClaimReady,
  };
}

function assertProof(proof) {
  if (
    proof.summary.rows !== 6 ||
    proof.summary.readyToStartCount !== 1 ||
    proof.summary.activeCount !== 1 ||
    proof.summary.expiredAutoStopCount !== 1 ||
    proof.summary.manualRequiredCount !== 6 ||
    proof.summary.revokedOrDeniedCount !== 1
  ) {
    throw new Error(`Unexpected temporary live readiness summary: ${JSON.stringify(proof.summary)}`);
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(`Temporary live proof overclaimed runtime behavior: ${JSON.stringify(proof.nonClaims)}`);
  }
}

async function writeProofPack(path, proof) {
  await writeFile(
    join(path, '00-source-snapshot.md'),
    [
      '# WP28 Temporary Live Tracking Source Snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
      '- Scope: parent-domain temporary live readiness over existing tracking policy read model.',
      '- Source inspected: tracking location policy, location/geofence expectations, data-custody expectation, location/geofence feature doc, and WP28 checklist.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeJson(join(path, '03-runtime-location-evidence.json'), {
    status: 'p1-fixture-simulation',
    liveLocationRuntimeClaimed: false,
    physicalDeviceProofClaimed: false,
    backgroundLocationClaimed: false,
    rows: proof.summary.rows,
    platformRuntimeProofStates: proof.summary.platformRuntimeProofStates,
  });
  await writeJson(join(path, '09-policy-alert-proof.json'), {
    status: 'p1-fixture-simulation',
    parentAuthorizationProved: true,
    durationAndCadencePolicyProved: true,
    autoStopReasonProved: true,
    childDisclosureRequirementProved: true,
    productClaimReady: false,
    readinessStates: proof.summary.readinessStates,
  });
  await writeJson(join(path, '14-retention-delete-proof.json'), {
    status: 'p1-fixture-simulation',
    retentionActions: proof.summary.retentionActions,
    expiredAutoStopCount: proof.summary.expiredAutoStopCount,
    liveRetentionRuntimeClaimed: false,
  });
  await writeFile(
    join(path, '16-validation-commands.log'),
    [
      'Validation commands:',
      '',
      '- cmd /c npm run build --workspace @ocentra-parent/parent-domain: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/parent-domain -- tracking-temporary-live-readiness-proof tracking-location-policy: PASS',
      '',
    ].join('\n'),
    'utf8'
  );
}

async function writeJson(path, data) {
  await writeFile(path, `${JSON.stringify(data, null, 2)}\n`, 'utf8');
}

function run(command, args) {
  const printable = [command, ...args].join(' ');
  const result = spawnSync(command, args, { cwd: repoRoot, encoding: 'utf8', stdio: 'inherit' });
  commands.push({ command: printable, status: result.status ?? 1 });
  if (result.status !== 0) {
    throw new Error(`Command failed: ${printable}`);
  }
}

function gitOutput(args) {
  const result = spawnSync('git', args, { cwd: repoRoot, encoding: 'utf8' });
  if (result.status !== 0) {
    throw new Error(`git ${args.join(' ')} failed: ${result.stderr}`);
  }
  return result.stdout.trim();
}

function countBy(values) {
  return values.reduce((acc, value) => {
    acc[value] = (acc[value] ?? 0) + 1;
    return acc;
  }, {});
}
