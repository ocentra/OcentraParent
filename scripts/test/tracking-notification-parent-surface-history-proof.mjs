import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { runNpmCommand } from './run-npm-command.mjs';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const testOutputDir = join(repoRoot, 'test-results', 'tracking-notification-parent-surface-history-proof');
const wp26Dir = join(repoRoot, 'output', 'tracking-plan-proof', '26-alert-severity-and-notification-model');
const wp33Dir = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const proofDir = join(repoRoot, 'output', 'tracking-plan-proof', 'tracking-notification-parent-surface-history-proof');
const timestamp = '2026-06-06T16:16:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(testOutputDir, { recursive: true, force: true });
await rm(proofDir, { recursive: true, force: true });
await mkdir(testOutputDir, { recursive: true });
await mkdir(wp26Dir, { recursive: true });
await mkdir(wp33Dir, { recursive: true });
await mkdir(proofDir, { recursive: true });

runNpmCommand(run, ['run', 'build', '--workspace', '@ocentra-parent/schema-domain']);
runNpmCommand(run, ['run', 'build', '--workspace', '@ocentra-parent/tracking-domain']);
run('cmd', [
  '/c',
  'npm',
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/tracking-domain',
  '--',
  'tracking-notification-parent-surface-history-proof',
  'tracking-notification-receipt-boundary-proof',
  'tracking-notification-preference-preflight-proof',
  'tracking-provider-notification-proof',
]);

const tracking = await importSchemaDist('tracking-location-policy.js');
const preferenceModule = await importSchemaDist('tracking-notification-preference-preflight-proof.js');
const providerModule = await import(
  pathToFileURL(join(repoRoot, 'packages', 'schema-domain', 'dist', 'tracking-provider-notification-proof.js')).href
);
const receiptModule = await import(
  pathToFileURL(
    join(repoRoot, 'packages', 'schema-domain', 'dist', 'tracking-notification-receipt-boundary-proof.js')
  ).href
);
const historyModule = await import(
  pathToFileURL(
    join(repoRoot, 'packages', 'schema-domain', 'dist', 'tracking-notification-parent-surface-history-proof.js')
  ).href
);
const sourceReadModel = tracking.TrackingLocationPolicyReadModelSchema.parse(sourceTrackingReadModel(tracking));
const providerReadModel = providerModule.buildTrackingProviderNotificationProofReadModel(
  providerOptions(),
  sourceReadModel
);
const receiptReadModel = receiptModule.buildTrackingNotificationReceiptBoundaryReadModel(
  receiptOptions(),
  providerReadModel
);
const preferenceReadModel = preferenceModule.buildTrackingNotificationPreferencePreflightReadModel(
  preferenceOptions(),
  providerReadModel
);
const readModel = historyModule.buildTrackingNotificationParentSurfaceHistoryReadModel(
  historyOptions(),
  providerReadModel,
  receiptReadModel,
  preferenceReadModel
);
const proof = {
  proofMode: 'tracking-notification-parent-surface-history-proof',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: summarize(readModel),
  nonClaims: nonClaims(readModel),
  proofPaths: proofPaths(),
  sourceProviderNotificationProof: providerReadModel.proofId,
  sourceReceiptBoundaryProof: receiptReadModel.proofId,
  sourcePreferencePreflightProof: preferenceReadModel.preferencePreflightId,
  readModel,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'tracking-notification-parent-surface-history-read-model.json'), readModel);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(proofDir, proof);
await writeJson(join(wp26Dir, '26-notification-parent-surface-history-proof.json'), proof);
await writeJson(join(wp33Dir, '26-notification-parent-surface-history-proof.json'), proof);

console.log('tracking-notification-parent-surface-history-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-notification-parent-surface-history-proof', 'proof.json')}`);

function importSchemaDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'schema-domain', 'dist', name)).href);
}

function providerOptions() {
  return {
    generatedAt: timestamp,
    proofId: 'tracking-provider-notification-proof-for-parent-surface-history',
    familyId: 'family-tracking-notification-history',
    sourceTrackingReadModelRef: 'tracking-location-policy-read-model-parent-surface-history',
    sourceContractRefs: ['tracking-location-policy', 'v0-8-notification-provider-status-boundary'],
  };
}

function receiptOptions() {
  return {
    generatedAt: timestamp,
    proofId: 'tracking-notification-receipt-boundary-proof-for-parent-surface-history',
    familyId: 'family-tracking-notification-history',
    sourceProviderNotificationProofRef: 'tracking-provider-notification-proof-for-parent-surface-history',
    sourceContractRefs: ['tracking-provider-notification-proof', 'notification-receipt-boundary'],
  };
}

function preferenceOptions() {
  return {
    generatedAt: timestamp,
    preferencePreflightId: 'tracking-notification-preference-preflight-proof-for-parent-surface-history',
    sourceContractRefs: [
      'tracking-provider-notification-proof',
      'v3-notification-rule-provider-retry-contract',
      'notification-parent-preference-boundary',
      'notification-quiet-hours-policy-boundary',
    ],
  };
}

function historyOptions() {
  return {
    generatedAt: timestamp,
    proofId: 'tracking-notification-parent-surface-history-proof',
    sourceContractRefs: [
      'tracking-provider-notification-proof',
      'tracking-notification-receipt-boundary-proof',
      'tracking-notification-preference-preflight-proof',
      'notification-audit-history-contract-proof',
      'notifications-expectations',
      'location-geofence-device-status',
      'reports-notifications-sync',
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
        observedAt: '2026-06-06T16:15:00.000Z',
      },
    ],
    acknowledgementId: null,
  };
}

function summarize(readModel) {
  return {
    rows: readModel.rows.length,
    historyIntentReadyCount: readModel.historyIntentReadyCount,
    manualActionRequiredCount: readModel.manualActionRequiredCount,
    providerUnavailableCount: readModel.providerUnavailableCount,
    statuses: countBy(readModel.rows.map((row) => row.status)),
    sourceProviderNotificationProofRef: readModel.sourceProviderNotificationProofRef,
    sourceReceiptBoundaryProofRef: readModel.sourceReceiptBoundaryProofRef,
    sourcePreferencePreflightProofRef: readModel.sourcePreferencePreflightProofRef,
  };
}

function nonClaims(readModel) {
  return {
    renderedParentNotificationUiClaimed: readModel.renderedParentNotificationUiClaimed,
    parentPreferenceMutationRuntimeClaimed: readModel.parentPreferenceMutationRuntimeClaimed,
    parentFrequencyControlUiClaimed: readModel.parentFrequencyControlUiClaimed,
    quietHoursTimerRuntimeClaimed: readModel.quietHoursTimerRuntimeClaimed,
    providerDeliveryRuntimeClaimed: readModel.providerDeliveryRuntimeClaimed,
    providerReceiptIngestionRuntimeClaimed: readModel.providerReceiptIngestionRuntimeClaimed,
    providerCredentialsClaimed: readModel.providerCredentialsClaimed,
    cloudRoutingClaimed: readModel.cloudRoutingClaimed,
    childDeviceDeliveryClaimed: readModel.childDeviceDeliveryClaimed,
    mobilePhysicalDeviceProofClaimed: readModel.mobilePhysicalDeviceProofClaimed,
    authorityProofClaimed: readModel.authorityProofClaimed,
    retryExecutionRuntimeClaimed: readModel.retryExecutionRuntimeClaimed,
    productionDurableHistoryStorageClaimed: readModel.productionDurableHistoryStorageClaimed,
    productionDurableOutboxStorageClaimed: readModel.productionDurableOutboxStorageClaimed,
    adapterDispatchClaimed: readModel.adapterDispatchClaimed,
  };
}

function proofPaths() {
  return {
    source: 'packages/schema-domain/src/tracking-notification-parent-surface-history-proof.ts',
    test: 'packages/tracking-domain/tests/contract/tracking-notification-parent-surface-history-proof.test.ts',
    harness: 'scripts/test/tracking-notification-parent-surface-history-proof.mjs',
    evidence: 'test-results/tracking-notification-parent-surface-history-proof/proof.json',
    focusedProofRoot: 'output/tracking-plan-proof/tracking-notification-parent-surface-history-proof',
    wp26: 'output/tracking-plan-proof/26-alert-severity-and-notification-model/26-notification-parent-surface-history-proof.json',
    wp33: 'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/26-notification-parent-surface-history-proof.json',
  };
}

function assertProof(proof) {
  if (
    proof.summary.rows !== 3 ||
    proof.summary.historyIntentReadyCount !== 1 ||
    proof.summary.manualActionRequiredCount !== 1 ||
    proof.summary.providerUnavailableCount !== 1
  ) {
    throw new Error(`Unexpected tracking notification parent-surface summary: ${JSON.stringify(proof.summary)}`);
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(
      `Tracking notification parent-surface proof overclaimed behavior: ${JSON.stringify(proof.nonClaims)}`
    );
  }
}

async function writeProofPack(path, proof) {
  await writeFile(
    join(path, '00-source-snapshot.md'),
    [
      '# Tracking Notification Parent-Surface History Source Snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
      '- Scope: tracking notification parent-surface history/preference intent readiness derived from provider, receipt, and preference preflight proof rows.',
      '- Source refs carried by this proof: tracking provider-notification proof, receipt boundary proof, preference preflight proof, notification expectations, tracking feature doc, reports/notifications feature doc, and WP26/WP33 tracking workpacks.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '01-contract-proof.log'),
    [
      'Contract proof:',
      '',
      '- cmd /c npm run build --workspace @ocentra-parent/schema-domain: PASS',
      '- cmd /c npm run build --workspace @ocentra-parent/tracking-domain: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/tracking-domain -- tracking-notification-parent-surface-history-proof tracking-notification-receipt-boundary-proof tracking-notification-preference-preflight-proof tracking-provider-notification-proof: PASS',
      '- Parent-surface history rows preserve source provider, receipt, preference, evidence, policy decision, notification status, reason, audit, drill-in, quiet-hours, and manual proof refs.',
      '- Rows are parent-surface intent/readiness rows only; no rendered notification UI, preference mutation, provider delivery, or receipt runtime is claimed.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '13-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- Rendered parent notification UI, parent preference mutation runtime, frequency-control UI, quiet-hours timer runtime, provider delivery, receipt ingestion, credentials, cloud routing, child delivery, physical-device proof, authority proof, retry runtime, durable production history/outbox storage, and adapter dispatch remain false.',
      '- Parent summary refs are redacted intent refs; raw child evidence remains behind evidence refs and authenticated drill-in refs.',
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
