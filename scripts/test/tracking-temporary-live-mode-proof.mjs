import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofRoot = join(repoRoot, 'output', 'tracking-plan-proof', '28-temporary-live-tracking-mode');
const testResultRoot = join(repoRoot, 'test-results', 'tracking-temporary-live-mode-proof');
const commands = [];

await main();

async function main() {
  await runNpm(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  await runNpm([
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'tracking-temporary-live-mode-proof',
    'tracking-location-policy',
  ]);

  const policy = await import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'tracking-location-policy.js')));
  const temporaryLive = await import(
    pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'tracking-temporary-live-mode-proof.js'))
  );
  const checkedAt = new Date().toISOString();
  const commit = gitOutput(['rev-parse', 'HEAD']);
  const readModel = temporaryLive.buildTrackingTemporaryLiveModeReadModel(
    {
      generatedAt: '2026-06-05T16:40:00.000Z',
      proofId: 'tracking-temporary-live-mode-proof',
      sourceContractRefs: [
        'packages/parent-domain/src/tracking-location-policy.ts',
        'packages/parent-domain/src/tracking-temporary-live-mode-proof.ts',
        'docs/plans/tracking-plan/workpacks/28-temporary-live-tracking-mode.md',
        'docs/expectations/location-geofence.md',
        'docs/expectations/data-custody.md',
      ],
      contexts: temporaryLiveContexts(),
    },
    trackingPolicyReadModelFixture(policy)
  );

  assert.equal(readModel.rows.length, 6, 'expected six temporary live proof rows');
  assert.equal(readModel.activeAuthorizedCount, 1, 'active authorized row');
  assert.equal(readModel.degradedCount, 2, 'battery and permission degraded rows');
  assert.equal(readModel.autoStoppedCount, 1, 'expired auto-stop row');
  assert.equal(readModel.retentionDeleteReadyCount, 1, 'retention delete row');
  assert.equal(readModel.manualRequiredCount, 1, 'manual-required row');
  assert.equal(readModel.productClaimReady, false, 'not product-ready');
  assert.equal(readModel.liveLocationRuntimeClaimed, false, 'no live runtime claim');
  assert.equal(readModel.physicalDeviceProofClaimed, false, 'no physical device claim');

  await writeProofArtifacts({ checkedAt, commit, readModel });

  console.log('tracking-temporary-live-mode-proof-ok');
  console.log(`evidence=${relative(repoRoot, join(testResultRoot, 'proof.json'))}`);
}

async function writeProofArtifacts({ checkedAt, commit, readModel }) {
  await mkdir(proofRoot, { recursive: true });
  await mkdir(testResultRoot, { recursive: true });

  const proof = {
    schemaVersion: 1,
    checkedAt,
    commit,
    workpackId: '28-temporary-live-tracking-mode',
    requiredProofTier: 'P3_LOCAL_DEV_MACHINE',
    currentProofTier: 'P1_FIXTURE_SIMULATION',
    status: 'proved',
    artifactPaths: {
      sourceSnapshot: relative(repoRoot, join(proofRoot, '00-source-snapshot.md')),
      runtimeLocationEvidence: relative(repoRoot, join(proofRoot, '03-runtime-location-evidence.json')),
      policyAlertProof: relative(repoRoot, join(proofRoot, '09-policy-alert-proof.json')),
      retentionDeleteProof: relative(repoRoot, join(proofRoot, '14-retention-delete-proof.json')),
      validationLog: relative(repoRoot, join(proofRoot, '16-validation-commands.log')),
    },
    readModel,
    proofLabels: [
      'temporary-live.parent-authorization-required',
      'temporary-live.child-disclosure-required',
      'temporary-live.duration-bounded',
      'temporary-live.cadence-modeled',
      'temporary-live.battery-degraded',
      'temporary-live.permission-degraded',
      'temporary-live.auto-expiry',
      'temporary-live.retention-delete-ready',
      'temporary-live.no-live-runtime-claim',
    ],
    productClaims: {
      parentAuthorizationModeled: true,
      childDisclosureModeled: true,
      durationAndCadenceModeled: true,
      batteryAndPermissionDegradationModeled: true,
      retentionDeleteReadinessModeled: true,
      liveLocationRuntimeClaimed: false,
      currentLocationRuntimeClaimed: false,
      backgroundLocationRuntimeClaimed: false,
      providerDeliveryClaimed: false,
      remoteRelayRuntimeClaimed: false,
      parentPortalLiveMapRuntimeClaimed: false,
      childDeviceDeliveryClaimed: false,
      physicalDeviceProofClaimed: false,
      productionSessionWorkerClaimed: false,
      productClaimReady: false,
    },
    missingProofReason:
      'P1 fixture proof covers parent-domain temporary live tracking session states for authorization, duration/cadence, battery/permission degradation, auto-expiry, audit refs, and retention-delete readiness. Live location runtime, current/background location adapters, provider delivery, remote relay, portal live map runtime, child-device delivery, physical-device proof, and production session workers remain unclaimed.',
    commands,
  };

  await writeFile(join(proofRoot, '00-source-snapshot.md'), sourceSnapshot({ checkedAt, commit }));
  await writeFile(join(proofRoot, '03-runtime-location-evidence.json'), `${JSON.stringify(runtimeLocationEvidence(readModel), null, 2)}\n`);
  await writeFile(join(proofRoot, '09-policy-alert-proof.json'), `${JSON.stringify(policyAlertProof(readModel), null, 2)}\n`);
  await writeFile(join(proofRoot, '13-security-negative-proof.log'), securityNegativeProof());
  await writeFile(join(proofRoot, '14-retention-delete-proof.json'), `${JSON.stringify(retentionDeleteProof(readModel), null, 2)}\n`);
  await writeFile(join(proofRoot, '16-validation-commands.log'), commands.map((entry) => entry.command).join('\n') + '\n');
  await writeFile(join(proofRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(join(testResultRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(join(testResultRoot, 'tracking-temporary-live-read-model.json'), `${JSON.stringify(readModel, null, 2)}\n`);
}

function runtimeLocationEvidence(readModel) {
  return {
    workpackId: '28-temporary-live-tracking-mode',
    rows: readModel.rows.map((row) => ({
      grantId: row.grantId,
      sessionState: row.sessionState,
      requestedCadence: row.requestedCadence,
      requestedCadenceSeconds: row.requestedCadenceSeconds,
      permissionState: row.permissionState,
      batteryState: row.batteryState,
      deliveryPath: row.deliveryPath,
      locationEvidenceRefs: row.locationEvidenceRefs,
      liveLocationRuntimeClaimed: row.liveLocationRuntimeClaimed,
      currentLocationRuntimeClaimed: row.currentLocationRuntimeClaimed,
      backgroundLocationRuntimeClaimed: row.backgroundLocationRuntimeClaimed,
    })),
  };
}

function policyAlertProof(readModel) {
  return {
    workpackId: '28-temporary-live-tracking-mode',
    rows: readModel.rows.map((row) => ({
      grantId: row.grantId,
      parentApproved: row.parentApproved,
      childDisclosureRequired: row.childDisclosureRequired,
      policyDecisionRefs: row.policyDecisionRefs,
      auditRefs: row.auditRefs,
      manualProofRequirements: row.manualProofRequirements,
    })),
  };
}

function retentionDeleteProof(readModel) {
  return {
    workpackId: '28-temporary-live-tracking-mode',
    rows: readModel.rows.map((row) => ({
      grantId: row.grantId,
      sessionState: row.sessionState,
      autoStopReason: row.autoStopReason,
      retentionRefs: row.retentionRefs,
    })),
    retentionDeleteReadyCount: readModel.retentionDeleteReadyCount,
  };
}

function sourceSnapshot({ checkedAt, commit }) {
  return [
    '# 28-temporary-live-tracking-mode Source Snapshot',
    '',
    `- checkedAt: ${checkedAt}`,
    `- commit: ${commit}`,
    '- requiredProofTier: P3_LOCAL_DEV_MACHINE',
    '- currentProofTier: P1_FIXTURE_SIMULATION',
    '- status: proved',
    '- proof module: packages/parent-domain/src/tracking-temporary-live-mode-proof.ts',
    '- proof tests: packages/parent-domain/tests/tracking-temporary-live-mode-proof.test.ts',
    '- proof harness: scripts/test/tracking-temporary-live-mode-proof.mjs',
    '- source contracts: packages/parent-domain/src/tracking-location-policy.ts',
    '',
  ].join('\n');
}

function securityNegativeProof() {
  return [
    'workpack=28-temporary-live-tracking-mode',
    'Temporary live rows require parent authorization and child disclosure for active state.',
    'Duration and cadence are bounded and explicit in proof rows.',
    'Battery and permission degradation states carry manual proof requirements.',
    'Expired sessions carry auto-stop reasons; retention-delete-ready rows carry retention refs.',
    'Live location runtime, current/background location adapters, provider delivery, relay runtime, portal live map runtime, child-device delivery, physical-device proof, and production workers are all non-claims.',
    '',
  ].join('\n');
}

function trackingPolicyReadModelFixture(policy) {
  return {
    schemaVersion: policy.TrackingPolicySchemaVersion,
    generatedAt: '2026-06-05T16:40:00.000Z',
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
      grant(policy, 'tracking-live-active', 'active', true, true),
      grant(policy, 'tracking-live-battery', 'active', true, true),
      grant(policy, 'tracking-live-permission', 'active', true, true),
      grant(policy, 'tracking-live-expired', 'expired', true, true),
      grant(policy, 'tracking-live-retention', 'expired', true, true),
      grant(policy, 'tracking-live-manual-required', 'unavailable', false, false),
    ],
    missingDeviceCases: [],
    platformProofRoutes: [],
  };
}

function grant(policy, grantId, state, parentApproved, childDisclosureRequired) {
  return {
    schemaVersion: policy.TrackingPolicySchemaVersion,
    grantId,
    state,
    requestedAt: '2026-06-05T16:30:00.000Z',
    expiresAt: '2026-06-05T16:45:00.000Z',
    durationSeconds: 900,
    parentApproved,
    childDisclosureRequired,
    auditRefs: [`temporary-live-audit-${grantId}`],
  };
}

function temporaryLiveContexts() {
  return [
    context('tracking-live-active', 'interval', 60, 900, 'foreground-only', 'normal', null, []),
    context('tracking-live-battery', 'interval', 300, 900, 'foreground-only', 'battery-throttled', null, [
      'battery-throttled-cadence-reduction-proof',
    ]),
    context('tracking-live-permission', 'on-change', 0, 900, 'background-permission-required', 'normal', null, [
      'background-location-permission-proof-required',
    ]),
    context('tracking-live-expired', 'interval', 60, 900, 'foreground-only', 'normal', 'duration-expired', []),
    context('tracking-live-retention', 'interval', 60, 900, 'foreground-only', 'normal', 'duration-expired-retention-delete-ready', [
      'temporary-live-retention-delete-proof',
    ]),
    context('tracking-live-manual-required', 'high-accuracy-burst', 15, 900, 'permission-required', 'low-power-mode', null, [
      'physical-device-live-session-proof-required',
    ]),
  ];
}

function context(grantId, requestedCadence, requestedCadenceSeconds, maxDurationSeconds, permissionState, batteryState, autoStopReason, extraProofRefs) {
  return {
    grantId,
    requestedCadence,
    requestedCadenceSeconds,
    maxDurationSeconds,
    permissionState,
    batteryState,
    deliveryPath: 'local-lan',
    locationEvidenceRefs: [`temporary-live-location-evidence-${grantId}`],
    policyDecisionRefs: [`temporary-live-policy-decision-${grantId}`],
    retentionRefs: grantId === 'tracking-live-retention' ? ['temporary-live-retention-delete-proof'] : [],
    manualProofRequirements: [
      `temporary-live-runtime-proof-required-${grantId}`,
      `temporary-live-physical-device-proof-required-${grantId}`,
      ...extraProofRefs,
    ],
    autoStopReason,
  };
}

async function runNpm(args) {
  const command = `cmd /c npm ${args.join(' ')}`;
  const startedAt = new Date().toISOString();
  execFileSync('cmd', ['/c', 'npm', ...args], { cwd: repoRoot, stdio: 'inherit' });
  commands.push({ command, startedAt, status: 'PASS' });
}

function gitOutput(args) {
  return execFileSync('git', args, { cwd: repoRoot, encoding: 'utf8' }).trim();
}
