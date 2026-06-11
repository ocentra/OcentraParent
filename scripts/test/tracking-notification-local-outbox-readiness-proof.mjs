import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { runNpmCommand } from './run-npm-command.mjs';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const testOutputDir = join(repoRoot, 'test-results', 'tracking-notification-local-outbox-readiness-proof');
const wp26ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '26-alert-severity-and-notification-model');
const wp33ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const focusedProofDir = join(
  repoRoot,
  'output',
  'tracking-plan-proof',
  'tracking-notification-local-outbox-readiness-proof'
);
const timestamp = '2026-06-07T15:03:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(testOutputDir, { recursive: true, force: true });
await rm(focusedProofDir, { recursive: true, force: true });
await mkdir(testOutputDir, { recursive: true });
await mkdir(wp26ProofDir, { recursive: true });
await mkdir(wp33ProofDir, { recursive: true });
await mkdir(focusedProofDir, { recursive: true });

runNpmCommand(run, ['run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
run('cmd', [
  '/c',
  'npm',
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'tracking-notification-local-outbox-readiness-proof',
  'tracking-notification-receipt-boundary-proof',
  'notification-local-outbox-adapter-proof',
  'notification-local-outbox-scheduler-proof',
]);

const tracking = await importDist('tracking-location-policy.js');
const providerProofModule = await importDist('tracking-provider-notification-proof.js');
const receiptProofModule = await importDist('tracking-notification-receipt-boundary-proof.js');
const localOutboxModule = await importDist('notification-local-outbox-adapter-proof.js');
const schedulerModule = await importDist('notification-local-outbox-scheduler-proof.js');
const readinessProofModule = await importDist('tracking-notification-local-outbox-readiness-proof.js');
const sourceReadModel = tracking.TrackingLocationPolicyReadModelSchema.parse(sourceTrackingReadModel(tracking));
const sourceProviderProof = providerProofModule.buildTrackingProviderNotificationProofReadModel(
  {
    generatedAt: timestamp,
    proofId: 'tracking-provider-notification-proof',
    familyId: 'family-tracking-notification-local-outbox',
    sourceTrackingReadModelRef: 'tracking-location-policy-read-model-provider-notification',
    sourceContractRefs: [
      'tracking-location-policy',
      'v0-8-notification-provider-status-boundary',
      'notification-local-outbox-adapter-proof',
      'notification-local-outbox-scheduler-proof',
      'location-geofence-device-status',
      'reports-notifications-sync',
    ],
  },
  sourceReadModel
);
const sourceReceiptProof = receiptProofModule.buildTrackingNotificationReceiptBoundaryReadModel(
  {
    generatedAt: timestamp,
    proofId: 'tracking-notification-receipt-boundary-proof',
    familyId: 'family-tracking-notification-local-outbox',
    sourceProviderNotificationProofRef: 'tracking-provider-notification-proof',
    sourceContractRefs: [
      'tracking-provider-notification-proof',
      'v0-8-notification-provider-status-boundary',
      'notification-local-outbox-adapter-proof',
      'notification-local-outbox-scheduler-proof',
      'notifications-expectations',
      'location-geofence-device-status',
      'reports-notifications-sync',
    ],
  },
  sourceProviderProof
);
const readModel = readinessProofModule.buildTrackingNotificationLocalOutboxReadinessReadModel(
  {
    generatedAt: timestamp,
    proofId: 'tracking-notification-local-outbox-readiness-proof',
    sourceContractRefs: [
      'tracking-notification-receipt-boundary-proof',
      'notification-local-outbox-adapter-proof',
      'notification-local-outbox-scheduler-proof',
      'notifications-expectations',
      'location-geofence-device-status',
      'reports-notifications-sync',
    ],
  },
  sourceReceiptProof,
  localOutboxModule.NotificationLocalOutboxAdapterProofReadModel,
  schedulerModule.NotificationLocalOutboxSchedulerProofReadModel
);

const proof = {
  proofMode: 'tracking-notification-local-outbox-readiness-proof',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: summarize(readModel),
  nonClaims: nonClaims(readModel),
  proofPaths: {
    source: 'packages/parent-domain/src/tracking-notification-local-outbox-readiness-proof.ts',
    test: 'packages/parent-domain/tests/tracking-notification-local-outbox-readiness-proof.test.ts',
    harness: 'scripts/test/tracking-notification-local-outbox-readiness-proof.mjs',
    evidence: 'test-results/tracking-notification-local-outbox-readiness-proof/proof.json',
    focusedProofRoot: 'output/tracking-plan-proof/tracking-notification-local-outbox-readiness-proof',
    wp26Proof:
      'output/tracking-plan-proof/26-alert-severity-and-notification-model/28-notification-local-outbox-readiness-proof.json',
    wp33Proof:
      'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/42-notification-local-outbox-readiness-proof.json',
  },
  readModel,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'tracking-notification-local-outbox-readiness-read-model.json'), readModel);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeFocusedProofPack(focusedProofDir, proof);
await writeJson(join(wp26ProofDir, '28-notification-local-outbox-readiness-proof.json'), proof);
await writeJson(join(wp33ProofDir, '42-notification-local-outbox-readiness-proof.json'), proof);

console.log('tracking-notification-local-outbox-readiness-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-notification-local-outbox-readiness-proof', 'proof.json')}`);

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
        observedAt: '2026-06-07T15:00:00.000Z',
      },
    ],
    acknowledgementId: null,
    ...input,
  };
}

function summarize(readModel) {
  return {
    rows: readModel.rows.length,
    receiptRequiredCount: readModel.receiptRequiredCount,
    manualRequiredCount: readModel.manualRequiredCount,
    providerUnavailableCount: readModel.providerUnavailableCount,
    readinessStates: countBy(readModel.rows.map((row) => row.readinessState)),
    localOutboxStates: countBy(readModel.rows.map((row) => row.localOutboxStateRef)),
    schedulerStates: countBy(readModel.rows.map((row) => row.schedulerStateRef)),
    sourceLocalOutboxAdapterProofRef: readModel.sourceLocalOutboxAdapterProofRef,
    sourceLocalOutboxSchedulerProofRef: readModel.sourceLocalOutboxSchedulerProofRef,
  };
}

function nonClaims(readModel) {
  return {
    providerDeliveryRuntimeClaimed: readModel.providerDeliveryRuntimeClaimed,
    providerReceiptIngestionRuntimeClaimed: readModel.providerReceiptIngestionRuntimeClaimed,
    providerCredentialsClaimed: readModel.providerCredentialsClaimed,
    cloudRoutingClaimed: readModel.cloudRoutingClaimed,
    parentNotificationUiClaimed: readModel.parentNotificationUiClaimed,
    retryExecutionRuntimeClaimed: readModel.retryExecutionRuntimeClaimed,
    quietHoursTimerRuntimeClaimed: readModel.quietHoursTimerRuntimeClaimed,
    productionDurableOutboxStorageClaimed: readModel.productionDurableOutboxStorageClaimed,
    childDeviceDeliveryClaimed: readModel.childDeviceDeliveryClaimed,
    mobilePhysicalDeviceProofClaimed: readModel.mobilePhysicalDeviceProofClaimed,
    authorityProofClaimed: readModel.authorityProofClaimed,
  };
}

function assertProof(proof) {
  if (
    proof.summary.rows !== 3 ||
    proof.summary.receiptRequiredCount !== 1 ||
    proof.summary.manualRequiredCount !== 1 ||
    proof.summary.providerUnavailableCount !== 1 ||
    proof.summary.sourceLocalOutboxAdapterProofRef !== 'notification-local-outbox-adapter-proof' ||
    proof.summary.sourceLocalOutboxSchedulerProofRef !== 'notification-local-outbox-scheduler-proof'
  ) {
    throw new Error(`Unexpected tracking notification local outbox summary: ${JSON.stringify(proof.summary)}`);
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(
      `Tracking notification local outbox proof overclaimed behavior: ${JSON.stringify(proof.nonClaims)}`
    );
  }
}

async function writeFocusedProofPack(path, proof) {
  await writeFile(
    join(path, '00-source-snapshot.md'),
    [
      '# Tracking Notification Local Outbox Readiness Source Snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
      '- Scope: tracking notification receipt rows mapped to existing local outbox and scheduler proof artifacts.',
      '- Source inspected: tracking notification receipt boundary proof, notification local outbox adapter proof, notification local outbox scheduler proof, notification expectations, location/geofence feature doc, and WP26/WP33 tracking workpacks.',
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
      '- cmd /c npm run test --workspace @ocentra-parent/parent-domain -- tracking-notification-local-outbox-readiness-proof tracking-notification-receipt-boundary-proof notification-local-outbox-adapter-proof notification-local-outbox-scheduler-proof: PASS',
      '- Tracking receipt-required rows map to the existing local receipt-required outbox and scheduler records.',
      '- Tracking manual-required rows map to the existing local manual-required outbox and scheduler records.',
      '- Tracking provider-unavailable rows map to the existing local dead-letter/manual-review scheduler path.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '13-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- Provider delivery, provider receipt ingestion runtime, credentials, cloud routing, parent notification UI, retry worker runtime, quiet-hours timer runtime, production durable outbox storage, child-device delivery, physical-device proof, and authority proof remain false.',
      '- The proof references parent-owned local outbox/scheduler artifact refs only; it does not store raw child evidence, provider secrets, private paths, or delivered receipt payloads.',
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
