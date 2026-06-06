import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const testOutputDir = join(repoRoot, 'test-results', 'tracking-notification-preference-preflight-proof');
const wp26Dir = join(repoRoot, 'output', 'tracking-plan-proof', '26-alert-severity-and-notification-model');
const wp33Dir = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const proofDir = join(repoRoot, 'output', 'tracking-plan-proof', 'tracking-notification-preference-preflight-proof');
const timestamp = '2026-06-06T08:02:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(testOutputDir, { recursive: true, force: true });
await rm(proofDir, { recursive: true, force: true });
await mkdir(testOutputDir, { recursive: true });
await mkdir(wp26Dir, { recursive: true });
await mkdir(wp33Dir, { recursive: true });
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
  'tracking-notification-preference-preflight-proof',
  'tracking-provider-notification-proof',
  'v3-notification-rule-provider-retry-contract',
]);

const tracking = await importDist('tracking-location-policy.js');
const providerModule = await importDist('tracking-provider-notification-proof.js');
const preferenceModule = await importDist('tracking-notification-preference-preflight-proof.js');
const sourceReadModel = tracking.TrackingLocationPolicyReadModelSchema.parse(sourceTrackingReadModel(tracking));
const providerReadModel = providerModule.buildTrackingProviderNotificationProofReadModel(
  providerOptions(),
  sourceReadModel
);
const readModel = preferenceModule.buildTrackingNotificationPreferencePreflightReadModel(
  preferenceOptions(),
  providerReadModel
);
const proof = {
  proofMode: 'tracking-notification-preference-preflight-proof',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: summarize(readModel),
  nonClaims: nonClaims(readModel),
  proofPaths: proofPaths(),
  sourceProviderNotificationProof: providerReadModel.proofId,
  readModel,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'tracking-notification-preference-preflight-read-model.json'), readModel);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(proofDir, proof);
await writeJson(join(wp26Dir, '24-notification-preference-preflight-proof.json'), proof);
await writeJson(join(wp33Dir, '24-notification-preference-preflight-proof.json'), proof);

console.log('tracking-notification-preference-preflight-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-notification-preference-preflight-proof', 'proof.json')}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function providerOptions() {
  return {
    generatedAt: timestamp,
    proofId: 'tracking-provider-notification-proof-for-preference-preflight',
    familyId: 'family-tracking-preference-preflight',
    sourceTrackingReadModelRef: 'tracking-location-policy-read-model-preference-preflight',
    sourceContractRefs: ['tracking-location-policy', 'v0-8-notification-provider-status-boundary'],
  };
}

function preferenceOptions() {
  return {
    generatedAt: timestamp,
    preferencePreflightId: 'tracking-notification-preference-preflight-proof',
    sourceContractRefs: [
      'tracking-provider-notification-proof',
      'v3-notification-rule-provider-retry-contract',
      'notification-parent-preference-boundary',
      'notification-quiet-hours-policy-boundary',
    ],
  };
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
      alert(tracking, 'tracking-alert-home-arrival', 'info', ['tracking-notification-intent-home-arrival']),
      alert(tracking, 'tracking-alert-left-expected-place', 'urgent', ['tracking-notification-intent-left-school']),
      alert(tracking, 'tracking-alert-provider-unavailable', 'warning', []),
    ],
    escalations: [],
    temporaryLiveGrants: [],
    missingDeviceCases: [],
    platformProofRoutes: [],
  };
}

function alert(tracking, alertId, severity, notificationStatusRefs) {
  return {
    schemaVersion: tracking.TrackingPolicySchemaVersion,
    alertId,
    severity,
    sensitiveDetailMode: severity === 'urgent' ? 'authenticated-drill-in-only' : 'minimal-provider-body',
    policyDecisionId: `tracking-decision-${alertId.replace('tracking-alert-', '')}`,
    notificationStatusRefs,
    reasonCodes: [`reason-${alertId}`],
    createdAt: timestamp,
    evidenceReferences: [
      {
        evidenceReferenceId: 'location-evidence-geofence-entry',
        kind: 'journal-event',
        observedAt: '2026-06-06T08:00:00.000Z',
      },
    ],
    acknowledgementId: null,
  };
}

function summarize(readModel) {
  return {
    rows: readModel.rows.length,
    parentPreferenceRequiredCount: readModel.parentPreferenceRequiredCount,
    sourceManualRequiredCount: readModel.sourceManualRequiredCount,
    sourceUnavailableCount: readModel.sourceUnavailableCount,
    statuses: countBy(readModel.rows.map((row) => row.status)),
    providerStatusKinds: countBy(readModel.rows.map((row) => row.providerStatusKind)),
  };
}

function nonClaims(readModel) {
  return {
    parentNotificationPreferenceUiClaimed: readModel.parentNotificationPreferenceUiClaimed,
    parentNotificationHistoryUiClaimed: readModel.parentNotificationHistoryUiClaimed,
    parentFrequencyControlUiClaimed: readModel.parentFrequencyControlUiClaimed,
    quietHoursTimerRuntimeClaimed: readModel.quietHoursTimerRuntimeClaimed,
    providerDeliveryRuntimeClaimed: readModel.providerDeliveryRuntimeClaimed,
    providerReceiptIngestionRuntimeClaimed: readModel.providerReceiptIngestionRuntimeClaimed,
    providerCredentialsClaimed: readModel.providerCredentialsClaimed,
    cloudRoutingClaimed: readModel.cloudRoutingClaimed,
    childDeviceDeliveryClaimed: readModel.childDeviceDeliveryClaimed,
    mobilePhysicalDeviceProofClaimed: readModel.mobilePhysicalDeviceProofClaimed,
    retryExecutionRuntimeClaimed: readModel.retryExecutionRuntimeClaimed,
    productionDurableOutboxStorageClaimed: readModel.productionDurableOutboxStorageClaimed,
    adapterDispatchClaimed: readModel.adapterDispatchClaimed,
  };
}

function proofPaths() {
  return {
    source: 'packages/parent-domain/src/tracking-notification-preference-preflight-proof.ts',
    test: 'packages/parent-domain/tests/tracking-notification-preference-preflight-proof.test.ts',
    harness: 'scripts/test/tracking-notification-preference-preflight-proof.mjs',
    evidence: 'test-results/tracking-notification-preference-preflight-proof/proof.json',
    wp26: 'output/tracking-plan-proof/26-alert-severity-and-notification-model/24-notification-preference-preflight-proof.json',
    wp33: 'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/24-notification-preference-preflight-proof.json',
  };
}

function assertProof(proof) {
  if (
    proof.summary.rows !== 3 ||
    proof.summary.parentPreferenceRequiredCount !== 1 ||
    proof.summary.sourceManualRequiredCount !== 1 ||
    proof.summary.sourceUnavailableCount !== 1
  ) {
    throw new Error(`Unexpected tracking preference preflight summary: ${JSON.stringify(proof.summary)}`);
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(`Tracking preference preflight overclaimed runtime behavior: ${JSON.stringify(proof.nonClaims)}`);
  }
}

async function writeProofPack(path, proof) {
  await writeFile(
    join(path, '00-source-snapshot.md'),
    [
      '# Tracking Notification Preference Preflight Source Snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Source provider proof: ${proof.sourceProviderNotificationProof}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(join(path, '01-contract-proof.log'), `${proof.commands.join('\n')}\n`, 'utf8');
  await writeFile(
    join(path, '13-security-negative-proof.log'),
    'Parent preference UI, notification history UI, quiet-hours timers, provider delivery, receipts, credentials, child delivery, physical devices, durable outbox storage, and adapter dispatch remain unclaimed.\n',
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
