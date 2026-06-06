import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const testOutputDir = join(repoRoot, 'test-results', 'tracking-notification-receipt-boundary-proof');
const wp26ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '26-alert-severity-and-notification-model');
const wp33ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const focusedProofDir = join(repoRoot, 'output', 'tracking-plan-proof', 'tracking-notification-receipt-boundary-proof');
const timestamp = '2026-06-06T07:04:00.000Z';
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
  'tracking-notification-receipt-boundary-proof',
  'tracking-provider-notification-proof',
  'v0-8-notification-provider-status-boundary',
]);

const tracking = await importDist('tracking-location-policy.js');
const providerProofModule = await importDist('tracking-provider-notification-proof.js');
const receiptProofModule = await importDist('tracking-notification-receipt-boundary-proof.js');
const sourceReadModel = tracking.TrackingLocationPolicyReadModelSchema.parse(sourceTrackingReadModel(tracking));
const sourceProviderProof = providerProofModule.buildTrackingProviderNotificationProofReadModel(
  {
    generatedAt: timestamp,
    proofId: 'tracking-provider-notification-proof',
    familyId: 'family-tracking-notification-receipt',
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
const readModel = receiptProofModule.buildTrackingNotificationReceiptBoundaryReadModel(
  {
    generatedAt: timestamp,
    proofId: 'tracking-notification-receipt-boundary-proof',
    familyId: 'family-tracking-notification-receipt',
    sourceProviderNotificationProofRef: 'tracking-provider-notification-proof',
    sourceContractRefs: [
      'tracking-provider-notification-proof',
      'v0-8-notification-provider-status-boundary',
      'notifications-expectations',
      'location-geofence-device-status',
      'reports-notifications-sync',
    ],
  },
  sourceProviderProof
);

const proof = {
  proofMode: 'tracking-notification-receipt-boundary-proof',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: summarize(readModel),
  nonClaims: nonClaims(readModel),
  proofPaths: {
    source: 'packages/parent-domain/src/tracking-notification-receipt-boundary-proof.ts',
    test: 'packages/parent-domain/tests/tracking-notification-receipt-boundary-proof.test.ts',
    harness: 'scripts/test/tracking-notification-receipt-boundary-proof.mjs',
    evidence: 'test-results/tracking-notification-receipt-boundary-proof/proof.json',
    focusedProofRoot: 'output/tracking-plan-proof/tracking-notification-receipt-boundary-proof',
    wp26Proof:
      'output/tracking-plan-proof/26-alert-severity-and-notification-model/22-notification-receipt-boundary-proof.json',
    wp33Proof:
      'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/22-notification-receipt-boundary-proof.json',
  },
  readModel,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'tracking-notification-receipt-boundary-read-model.json'), readModel);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeFocusedProofPack(focusedProofDir, proof);
await writeJson(join(wp26ProofDir, '22-notification-receipt-boundary-proof.json'), proof);
await writeJson(join(wp33ProofDir, '22-notification-receipt-boundary-proof.json'), proof);

console.log('tracking-notification-receipt-boundary-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-notification-receipt-boundary-proof', 'proof.json')}`);

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
        observedAt: '2026-06-06T07:00:00.000Z',
      },
    ],
    acknowledgementId: null,
    ...input,
  };
}

function summarize(readModel) {
  return {
    rows: readModel.rows.length,
    receiptIngestionRequiredCount: readModel.receiptIngestionRequiredCount,
    manualReceiptRequiredCount: readModel.manualReceiptRequiredCount,
    providerUnavailableCount: readModel.providerUnavailableCount,
    receiptBoundaryStates: countBy(readModel.rows.map((row) => row.receiptBoundaryState)),
    sourceProviderProofNonClaims: readModel.sourceProviderProofNonClaims.length,
    providerReceiptRequiredCoverageRef: readModel.providerReceiptRequiredCoverageRef,
  };
}

function nonClaims(readModel) {
  return {
    providerDeliveryRuntimeClaimed: readModel.providerDeliveryRuntimeClaimed,
    webhookReceiptIngestionRuntimeClaimed: readModel.webhookReceiptIngestionRuntimeClaimed,
    providerCredentialsClaimed: readModel.providerCredentialsClaimed,
    adapterDispatchClaimed: readModel.adapterDispatchClaimed,
    retryExecutionRuntimeClaimed: readModel.retryExecutionRuntimeClaimed,
    quietHoursTimerRuntimeClaimed: readModel.quietHoursTimerRuntimeClaimed,
    parentNotificationUiClaimed: readModel.parentNotificationUiClaimed,
    childDeviceDeliveryClaimed: readModel.childDeviceDeliveryClaimed,
    mobilePhysicalDeviceProofClaimed: readModel.mobilePhysicalDeviceProofClaimed,
    authorityProofClaimed: readModel.authorityProofClaimed,
    productionDurableOutboxStorageClaimed: readModel.productionDurableOutboxStorageClaimed,
  };
}

function assertProof(proof) {
  if (
    proof.summary.rows !== 3 ||
    proof.summary.receiptIngestionRequiredCount !== 1 ||
    proof.summary.manualReceiptRequiredCount !== 1 ||
    proof.summary.providerUnavailableCount !== 1 ||
    proof.summary.providerReceiptRequiredCoverageRef !== 'notification-provider-delivered-receipt-required'
  ) {
    throw new Error(`Unexpected tracking notification receipt summary: ${JSON.stringify(proof.summary)}`);
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(`Tracking notification receipt proof overclaimed behavior: ${JSON.stringify(proof.nonClaims)}`);
  }
}

async function writeFocusedProofPack(path, proof) {
  await writeFile(
    join(path, '00-source-snapshot.md'),
    [
      '# Tracking Notification Receipt Boundary Source Snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
      '- Scope: tracking notification receipt ingestion boundary readiness derived from tracking provider-notification proof rows.',
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
      '- cmd /c npm run test --workspace @ocentra-parent/parent-domain -- tracking-notification-receipt-boundary-proof tracking-provider-notification-proof v0-8-notification-provider-status-boundary: PASS',
      '- Receipt boundary rows preserve provider proof refs, evidence refs, policy decision refs, notification status refs, reason refs, provider attempt refs, and audit refs.',
      '- The delivered provider status contract is cited as a future receipt-required coverage ref while actual receipt ingestion remains unclaimed.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '13-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- Provider delivery, webhook receipt ingestion, credentials, adapter dispatch, retry worker runtime, quiet-hours runtime, parent notification UI, child-device delivery, mobile physical-device proof, authority proof, and durable outbox storage remain false.',
      '- Receipt boundary rows keep provider receipt refs empty because no real provider receipt has been ingested.',
      '- Tracking evidence remains behind source evidence refs instead of provider payload metadata.',
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
