import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const testOutputDir = join(repoRoot, 'test-results', 'tracking-notification-parent-surface-proof');
const wp26ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '26-alert-severity-and-notification-model');
const wp33ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const focusedProofDir = join(repoRoot, 'output', 'tracking-plan-proof', 'tracking-notification-parent-surface-proof');
const timestamp = '2026-06-06T07:22:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(testOutputDir, { recursive: true, force: true });
await rm(focusedProofDir, { recursive: true, force: true });
await mkdir(testOutputDir, { recursive: true });
await mkdir(wp26ProofDir, { recursive: true });
await mkdir(wp33ProofDir, { recursive: true });
await mkdir(focusedProofDir, { recursive: true });

run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
run('cmd', [
  '/c',
  'npm',
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'tracking-notification-parent-surface-proof',
  'tracking-provider-notification-proof',
  'v0-8-notification-provider-status-boundary',
]);

const tracking = await importDist('tracking-location-policy.js');
const providerProofModule = await importDist('tracking-provider-notification-proof.js');
const parentSurfaceModule = await importDist('tracking-notification-parent-surface-proof.js');
const sourceReadModel = tracking.TrackingLocationPolicyReadModelSchema.parse(sourceTrackingReadModel(tracking));
const sourceProviderProof = providerProofModule.buildTrackingProviderNotificationProofReadModel(
  {
    generatedAt: timestamp,
    proofId: 'tracking-provider-notification-proof',
    familyId: 'family-tracking-notification-parent-surface',
    sourceTrackingReadModelRef: 'tracking-location-policy-read-model-provider-notification',
    sourceContractRefs: [
      'tracking-location-policy',
      'v0-8-notification-provider-status-boundary',
      'notification-local-outbox-adapter-proof',
      'location-geofence-device-status',
      'reports-notifications-sync',
    ],
  },
  sourceReadModel
);
const readModel = parentSurfaceModule.buildTrackingNotificationParentSurfaceReadModel(
  {
    generatedAt: timestamp,
    proofId: 'tracking-notification-parent-surface-proof',
    sourceProviderNotificationProofRef: 'tracking-provider-notification-proof',
    sourceContractRefs: [
      'tracking-provider-notification-proof',
      'v0-8-notification-provider-status-boundary',
      'notifications-expectations-parent-surface',
      'location-geofence-device-status',
      'reports-notifications-sync',
    ],
  },
  sourceProviderProof
);

const proof = {
  proofMode: 'tracking-notification-parent-surface-proof',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: summarize(readModel),
  nonClaims: nonClaims(readModel),
  proofPaths: {
    source: 'packages/parent-domain/src/tracking-notification-parent-surface-proof.ts',
    test: 'packages/parent-domain/tests/tracking-notification-parent-surface-proof.test.ts',
    harness: 'scripts/test/tracking-notification-parent-surface-proof.mjs',
    evidence: 'test-results/tracking-notification-parent-surface-proof/proof.json',
    focusedProofRoot: 'output/tracking-plan-proof/tracking-notification-parent-surface-proof',
    wp26Proof:
      'output/tracking-plan-proof/26-alert-severity-and-notification-model/23-notification-parent-surface-proof.json',
    wp33Proof:
      'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/23-notification-parent-surface-proof.json',
  },
  readModel,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'tracking-notification-parent-surface-read-model.json'), readModel);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeFocusedProofPack(focusedProofDir, proof);
await writeJson(join(wp26ProofDir, '23-notification-parent-surface-proof.json'), proof);
await writeJson(join(wp33ProofDir, '23-notification-parent-surface-proof.json'), proof);

console.log('tracking-notification-parent-surface-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-notification-parent-surface-proof', 'proof.json')}`);

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
    alerts: [
      alert(tracking, {
        alertId: 'tracking-alert-home-arrival',
        severity: 'info',
        sensitiveDetailMode: 'minimal-provider-body',
        policyDecisionId: 'tracking-decision-home-arrival',
        notificationStatusRefs: ['tracking-notification-intent-home-arrival'],
        reasonCodes: ['home-arrival-notification'],
      }),
      alert(tracking, {
        alertId: 'tracking-alert-left-expected-place',
        severity: 'urgent',
        sensitiveDetailMode: 'authenticated-drill-in-only',
        policyDecisionId: 'tracking-decision-left-school',
        notificationStatusRefs: ['tracking-notification-intent-left-school'],
        reasonCodes: ['left-expected-place'],
      }),
      alert(tracking, {
        alertId: 'tracking-alert-provider-unavailable',
        severity: 'warning',
        sensitiveDetailMode: 'minimal-provider-body',
        policyDecisionId: 'tracking-decision-provider-unavailable',
        notificationStatusRefs: [],
        reasonCodes: ['provider-unavailable'],
      }),
    ],
    escalations: [],
    temporaryLiveGrants: [],
    missingDeviceCases: [],
    platformProofRoutes: [],
  };
}

function alert(tracking, input) {
  return {
    schemaVersion: tracking.TrackingPolicySchemaVersion,
    createdAt: timestamp,
    evidenceReferences: [
      {
        evidenceReferenceId: 'location-evidence-geofence-entry',
        kind: 'journal-event',
        observedAt: '2026-06-06T07:20:00.000Z',
      },
    ],
    acknowledgementId: null,
    ...input,
  };
}

function summarize(readModel) {
  return {
    rows: readModel.rows.length,
    historyRowReadyCount: readModel.historyRowReadyCount,
    manualActionRequiredCount: readModel.manualActionRequiredCount,
    unavailableVisibleCount: readModel.unavailableVisibleCount,
    preferenceSetupRequiredCount: readModel.preferenceSetupRequiredCount,
    parentSurfaceStatuses: countBy(readModel.rows.map((row) => row.parentSurfaceStatus)),
    historyVisibility: countBy(readModel.rows.map((row) => row.historyVisibility)),
    preferenceVisibility: countBy(readModel.rows.map((row) => row.preferenceVisibility)),
    sourceProviderProofNonClaims: readModel.sourceProviderProofNonClaims.length,
  };
}

function nonClaims(readModel) {
  return {
    parentNotificationUiRendered: readModel.parentNotificationUiRendered,
    parentPreferenceUiRendered: readModel.parentPreferenceUiRendered,
    parentHistoryUiRendered: readModel.parentHistoryUiRendered,
    parentPreferenceMutationRuntimeClaimed: readModel.parentPreferenceMutationRuntimeClaimed,
    providerDeliveryRuntimeClaimed: readModel.providerDeliveryRuntimeClaimed,
    providerReceiptIngestionClaimed: readModel.providerReceiptIngestionClaimed,
    providerCredentialsClaimed: readModel.providerCredentialsClaimed,
    adapterDispatchClaimed: readModel.adapterDispatchClaimed,
    childDeviceDeliveryClaimed: readModel.childDeviceDeliveryClaimed,
    mobilePhysicalDeviceProofClaimed: readModel.mobilePhysicalDeviceProofClaimed,
    authorityProofClaimed: readModel.authorityProofClaimed,
    productionDurableOutboxStorageClaimed: readModel.productionDurableOutboxStorageClaimed,
  };
}

function assertProof(proof) {
  if (
    proof.summary.rows !== 3 ||
    proof.summary.historyRowReadyCount !== 1 ||
    proof.summary.manualActionRequiredCount !== 1 ||
    proof.summary.unavailableVisibleCount !== 1 ||
    proof.summary.preferenceSetupRequiredCount !== 2
  ) {
    throw new Error(`Unexpected tracking notification parent surface summary: ${JSON.stringify(proof.summary)}`);
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(
      `Tracking notification parent surface proof overclaimed behavior: ${JSON.stringify(proof.nonClaims)}`
    );
  }
}

async function writeFocusedProofPack(path, proof) {
  await writeFile(
    join(path, '00-source-snapshot.md'),
    [
      '# Tracking Notification Parent Surface Source Snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
      '- Scope: tracking notification parent history/preference surface readiness derived from tracking provider-notification proof rows.',
      '- Source inspected: tracking provider-notification proof, V0.8 notification provider-status boundary, notification expectations, location/geofence feature doc, and WP26/WP33 tracking workpacks.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '01-contract-proof.log'),
    [
      'Contract proof:',
      '',
      '- cmd /c npm run build --workspace @ocentra-parent/parent-domain: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/parent-domain -- tracking-notification-parent-surface-proof tracking-provider-notification-proof v0-8-notification-provider-status-boundary: PASS',
      '- Parent surface rows preserve provider proof refs, evidence refs, policy decision refs, notification status refs, reason refs, drill-in refs, audit refs, and preference refs.',
      '- The proof exposes future history/preference surface readiness only; no rendered portal UI or parent preference mutation runtime is claimed.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '13-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- Parent notification UI, parent preference UI, parent history UI, parent preference mutation runtime, provider delivery, provider receipt ingestion, credentials, adapter dispatch, child-device delivery, mobile physical-device proof, authority proof, and durable outbox storage remain false.',
      '- Surface rows use drill-in and status refs only; sensitive tracking detail remains behind authenticated tracking detail surfaces.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(join(path, '16-validation-commands.log'), `${proof.commands.join('\n')}\n`, 'utf8');
  await writeJson(join(path, 'proof.json'), proof);
}

function run(command, args) {
  commands.push([command, ...args].join(' '));
  const result = spawnSync(command, args, { cwd: repoRoot, stdio: 'inherit', shell: false });
  if (result.status !== 0) {
    throw new Error(`Command failed: ${command} ${args.join(' ')}`);
  }
}

function gitOutput(args) {
  return spawnSync('git', args, { cwd: repoRoot, encoding: 'utf8' }).stdout.trim();
}

function countBy(values) {
  return values.reduce((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}
