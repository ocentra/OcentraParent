import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofName = 'app-game-notification-payload-preflight-proof';
const testOutputDir = join(repoRoot, 'test-results', proofName);
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', '63-notification-payload-preflight');
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', '63-notification-payload-preflight');
const timestamp = '2026-06-05T03:18:00Z';
const initialGitStatusShort = capture('git', ['status', '--short']);

for (const path of [testOutputDir, appGameProofDir, appProofDir]) {
  await rm(path, { recursive: true, force: true });
  await mkdir(path, { recursive: true });
}

run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
run('cmd', [
  '/c',
  'npm',
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'app-game-notification-payload-preflight',
  'app-game-notification-scheduler-bridge',
  'app-game-notification-local-outbox-bridge',
]);

const importDist = (name) => import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)));
const bridge = await importDist('app-game-notification-local-outbox-bridge.js');
const scheduler = await importDist('app-game-notification-scheduler-bridge.js');
const payload = await importDist('app-game-notification-payload-preflight.js');
const intent = await importDist('app-game-notification-intent.js');
const childUx = await importDist('app-game-child-facing-ux-rules.js');
const refs = await importDist('reference-primitives.js');

const outboxReadModel = bridge.buildAppGameNotificationLocalOutboxBridgeReadModel(
  bridgeOptions(refs),
  proofIntents(intent, childUx, refs)
);
const schedulerReadModel = scheduler.buildAppGameNotificationSchedulerBridgeReadModel(
  schedulerOptions(),
  outboxReadModel
);
const payloadReadModel = payload.buildAppGameNotificationPayloadPreflightReadModel(
  payloadOptions(),
  schedulerReadModel
);
const scheduledRows = payloadReadModel.rows.filter(
  (row) => row.status === payload.AppGameNotificationPayloadPreflightStatus.MinimalPayloadRequired
);
const proof = {
  proofMode: proofName,
  generatedAt: timestamp,
  scope: {
    sourceOutboxBridgeId: outboxReadModel.bridgeId,
    sourceSchedulerBridgeId: schedulerReadModel.schedulerBridgeId,
    payloadPreflightId: payloadReadModel.payloadPreflightId,
    rowCount: payloadReadModel.rows.length,
    minimalPayloadRequiredCount: payloadReadModel.minimalPayloadRequiredCount,
    manualRequiredCount: payloadReadModel.manualRequiredCount,
    unavailableCount: payloadReadModel.unavailableCount,
    providerChannels: scheduledRows.map((row) => row.providerChannel),
    minimalPayloadFieldCountByRow: scheduledRows.map((row) => row.minimalPayloadFields.length),
    sensitiveExclusionCountByRow: scheduledRows.map((row) => row.sensitiveDetailExclusionRefs.length),
    providerTemplateRequirementCountByRow: scheduledRows.map((row) => row.providerTemplateRequirementRefs.length),
  },
  statusCounts: countBy(payloadReadModel.rows.map((row) => row.status)),
  nonClaims: {
    providerPayloadTemplateRuntimeClaimed: payloadReadModel.providerPayloadTemplateRuntimeClaimed,
    sensitiveProviderMetadataStored: payloadReadModel.sensitiveProviderMetadataStored,
    rawChildEvidenceIncluded: payloadReadModel.rawChildEvidenceIncluded,
    rawUrlOrTitleIncluded: payloadReadModel.rawUrlOrTitleIncluded,
    rawMessageTextIncluded: payloadReadModel.rawMessageTextIncluded,
    screenshotOrReportIncluded: payloadReadModel.screenshotOrReportIncluded,
    providerDeliveryRuntimeClaimed: payloadReadModel.providerDeliveryRuntimeClaimed,
    providerReceiptIngestionClaimed: payloadReadModel.providerReceiptIngestionClaimed,
    providerCredentialsClaimed: payloadReadModel.providerCredentialsClaimed,
    cloudRoutingClaimed: payloadReadModel.cloudRoutingClaimed,
    parentNotificationUiClaimed: payloadReadModel.parentNotificationUiClaimed,
    childDeliveryClaimed: payloadReadModel.childDeliveryClaimed,
    retryExecutionRuntimeClaimed: payloadReadModel.retryExecutionRuntimeClaimed,
    quietHoursTimerRuntimeClaimed: payloadReadModel.quietHoursTimerRuntimeClaimed,
    productionDurableOutboxStorageClaimed: payloadReadModel.productionDurableOutboxStorageClaimed,
    adapterDispatchClaimed: payloadReadModel.adapterDispatchClaimed,
  },
  proofPaths: {
    source: 'packages/parent-domain/src/app-game-notification-payload-preflight.ts',
    test: 'packages/parent-domain/tests/app-game-notification-payload-preflight.test.ts',
    harness: 'scripts/test/app-game-notification-payload-preflight-proof.mjs',
    readModel: 'test-results/app-game-notification-payload-preflight-proof/payload-preflight-read-model.json',
    appGameProofPack: 'output/app-game-plan-proof/63-notification-payload-preflight',
    appProofPack: 'output/app-plan-proof/63-notification-payload-preflight',
  },
  gitStatusShort: initialGitStatusShort,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeJson(join(testOutputDir, 'payload-preflight-read-model.json'), payloadReadModel);
await writeProofPack(appGameProofDir, proof, payloadReadModel);
await writeProofPack(appProofDir, proof, payloadReadModel);

console.log('app-game-notification-payload-preflight-proof-ok');
console.log(`evidence=${join('test-results', proofName, 'proof.json')}`);

function run(command, args) {
  const result = spawnSync(command, args, { cwd: repoRoot, stdio: 'inherit', shell: false });
  if (result.status !== 0) {
    throw new Error(`Command failed: ${command} ${args.join(' ')}`);
  }
}

function capture(command, args) {
  const result = spawnSync(command, args, { cwd: repoRoot, encoding: 'utf8', shell: false });
  if (result.status !== 0) {
    return result.stderr.trim();
  }
  return result.stdout.trim();
}

function payloadOptions() {
  return {
    generatedAt: timestamp,
    payloadPreflightId: 'app-game-notification-payload-preflight-proof',
  };
}

function schedulerOptions() {
  return {
    generatedAt: timestamp,
    schedulerBridgeId: 'app-game-notification-scheduler-bridge-for-payload-proof',
    schedulerArtifactRootRef: 'parent-owned-app-game-notification-scheduler-root-for-payload-proof',
    schedulerArtifactRef: 'parent-owned-app-game-notification-scheduler-jsonl-ref-for-payload-proof',
    schedulerNowAt: timestamp,
  };
}

function bridgeOptions(refs) {
  return {
    family: { familyId: 'family-app-game-payload-proof' },
    parentAction: {
      actionReferenceId: 'parent-action-app-game-payload-proof',
      actor: { actorId: 'parent-app-game-payload-proof', role: refs.ParentActorRole.Parent },
      policyVersion: 'policy-app-game-notification-payload-proof-v1',
      createdAt: timestamp,
    },
    generatedAt: timestamp,
    bridgeId: 'app-game-notification-local-outbox-bridge-for-payload-proof',
    outboxRootRef: 'parent-owned-app-game-local-outbox-root-for-payload-proof',
    outboxFileRef: 'parent-owned-app-game-local-outbox-jsonl-for-payload-proof',
    localDataPathRef: 'parent-owned-app-game-local-outbox-data-path-for-payload-proof',
  };
}

function proofIntents(intent, childUx, refs) {
  const base = {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    notificationIntentId: 'notification-intent-time-limit-payload-proof',
    intentKind: intent.AppGameNotificationIntentKind.TimeLimitReached,
    intentStatus: intent.AppGameNotificationIntentStatus.LocalOutboxEligible,
    priority: intent.AppGameNotificationPriority.Urgent,
    device: {
      deviceId: 'device-app-game-payload-proof',
      childProfileId: 'child-app-game-payload-proof',
      label: 'Study PC',
      platform: refs.ParentPlatform.Windows,
    },
    targetKind: childUx.AppGameChildUxTargetKind.NativeGame,
    targetRef: 'target-native-game-payload-proof',
    notificationReasonCode: intent.AppGameNotificationReasonCode.TimeLimit,
    providerChannelPreference: 'in-app',
    parentTitleToken: intent.AppGameNotificationParentCopyToken.TimeLimitTitle,
    parentBodyToken: intent.AppGameNotificationParentCopyToken.TimeLimitBody,
    parentActionToken: intent.AppGameNotificationParentCopyToken.OpenParentReviewAction,
    childTitleToken: childUx.AppGameChildUxCopyToken.LimitReachedTitle,
    childBodyToken: childUx.AppGameChildUxCopyToken.LimitReachedBody,
    notificationRuleRef: 'notification-rule-app-game-time-limit-payload-proof',
    notificationStatusRef: 'notification-status-app-game-time-limit-payload-proof',
    policyRefs: ['policy-ref-app-game-time-limit-payload-proof'],
    auditRefs: ['audit-ref-app-game-time-limit-payload-proof'],
    evidenceReferences: [
      {
        evidenceReferenceId: 'evidence-ref-app-game-time-limit-payload-proof',
        kind: refs.ParentEvidenceReferenceKind.PolicyDecision,
        observedAt: timestamp,
      },
    ],
    childReasonReferences: [],
    childStatusReferences: ['child-status-app-game-time-limit-payload-proof'],
    approvalActionRef: null,
    timeBudgetDecisionRef: 'time-budget-decision-app-game-payload-proof',
    unknownCandidateRef: null,
    localOutboxRecordRef: 'local-outbox-record-app-game-time-limit-payload-proof',
    providerAttemptRefs: [],
    providerReceiptRefs: [],
    manualProofRequirements: [],
    minimalPayloadFields: Object.values(intent.AppGameNotificationPayloadField),
    deliveryClaimState: intent.AppGameNotificationDeliveryClaimState.LocalOutboxOnly,
    rawChildEvidenceIncluded: false,
    rawUrlOrTitleIncluded: false,
    rawMessageTextIncluded: false,
    screenshotOrReportIncluded: false,
    providerDeliveryAttempted: false,
    providerDeliveryObserved: false,
    providerReceiptIngested: false,
    cloudRoutingClaimed: false,
    parentNotificationUiClaimed: false,
    adapterDispatchState: 'not-dispatched',
    adapterActionClaimed: false,
    createdAt: timestamp,
  };
  return [
    base,
    suspiciousUnknownIntent(base, intent, childUx),
    manualIntent(base, intent, childUx),
    unavailableIntent(base, intent, childUx),
  ];
}

function suspiciousUnknownIntent(base, intent, childUx) {
  return {
    ...base,
    notificationIntentId: 'notification-intent-suspicious-unknown-payload-proof',
    intentKind: intent.AppGameNotificationIntentKind.SuspiciousUnknown,
    priority: intent.AppGameNotificationPriority.Attention,
    targetKind: childUx.AppGameChildUxTargetKind.UnknownApp,
    notificationReasonCode: intent.AppGameNotificationReasonCode.SuspiciousUnknown,
    providerChannelPreference: 'email',
    parentTitleToken: intent.AppGameNotificationParentCopyToken.SuspiciousUnknownTitle,
    parentBodyToken: intent.AppGameNotificationParentCopyToken.SuspiciousUnknownBody,
    childTitleToken: childUx.AppGameChildUxCopyToken.NewAppTitle,
    childBodyToken: childUx.AppGameChildUxCopyToken.NewAppBody,
    localOutboxRecordRef: 'local-outbox-record-app-game-suspicious-unknown-payload-proof',
    timeBudgetDecisionRef: null,
    unknownCandidateRef: 'unknown-app-candidate-payload-proof',
  };
}

function manualIntent(base, intent, childUx) {
  return {
    ...base,
    notificationIntentId: 'notification-intent-manual-required-payload-proof',
    intentKind: intent.AppGameNotificationIntentKind.ManualRequired,
    intentStatus: intent.AppGameNotificationIntentStatus.ManualRequired,
    notificationReasonCode: intent.AppGameNotificationReasonCode.ManualReviewRequired,
    parentTitleToken: intent.AppGameNotificationParentCopyToken.ManualRequiredTitle,
    parentBodyToken: intent.AppGameNotificationParentCopyToken.ManualRequiredBody,
    parentActionToken: intent.AppGameNotificationParentCopyToken.ReviewManuallyAction,
    childTitleToken: childUx.AppGameChildUxCopyToken.ManualRequiredTitle,
    childBodyToken: childUx.AppGameChildUxCopyToken.ManualRequiredBody,
    localOutboxRecordRef: null,
    timeBudgetDecisionRef: null,
    manualProofRequirements: ['provider preference setup before app game notification payload can be claimed'],
    deliveryClaimState: intent.AppGameNotificationDeliveryClaimState.ManualRequired,
  };
}

function unavailableIntent(base, intent, childUx) {
  return {
    ...base,
    notificationIntentId: 'notification-intent-unavailable-payload-proof',
    intentKind: intent.AppGameNotificationIntentKind.CapabilityUnavailable,
    intentStatus: intent.AppGameNotificationIntentStatus.Unavailable,
    priority: intent.AppGameNotificationPriority.Info,
    notificationReasonCode: intent.AppGameNotificationReasonCode.CapabilityUnavailable,
    parentTitleToken: intent.AppGameNotificationParentCopyToken.UnavailableTitle,
    parentBodyToken: intent.AppGameNotificationParentCopyToken.UnavailableBody,
    parentActionToken: intent.AppGameNotificationParentCopyToken.ReviewManuallyAction,
    childTitleToken: childUx.AppGameChildUxCopyToken.UnavailableTitle,
    childBodyToken: childUx.AppGameChildUxCopyToken.UnavailableBody,
    localOutboxRecordRef: null,
    timeBudgetDecisionRef: null,
    manualProofRequirements: [
      'local evidence and policy readiness before unavailable notification payload can be claimed',
    ],
    deliveryClaimState: intent.AppGameNotificationDeliveryClaimState.ManualRequired,
  };
}

function assertProof(proof) {
  if (
    proof.scope.rowCount !== 4 ||
    proof.scope.minimalPayloadRequiredCount !== 2 ||
    proof.scope.manualRequiredCount !== 1 ||
    proof.scope.unavailableCount !== 1
  ) {
    throw new Error(`Unexpected payload preflight counts: ${JSON.stringify(proof.scope)}`);
  }
  if (proof.scope.minimalPayloadFieldCountByRow.some((count) => count !== 7)) {
    throw new Error(`Minimal payload fields were not complete: ${JSON.stringify(proof.scope)}`);
  }
  if (proof.scope.sensitiveExclusionCountByRow.some((count) => count !== 5)) {
    throw new Error(`Sensitive detail exclusions were not complete: ${JSON.stringify(proof.scope)}`);
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(`Payload preflight claimed unavailable runtime behavior: ${JSON.stringify(proof.nonClaims)}`);
  }
}

function countBy(values) {
  return Object.fromEntries(
    [...new Set(values)].map((value) => [value, values.filter((candidate) => candidate === value).length])
  );
}

async function writeProofPack(root, proof, readModel) {
  await writeFile(join(root, '00-source-snapshot.md'), sourceSnapshot(), 'utf8');
  await writeJson(join(root, '01-payload-preflight-proof.json'), proof);
  await writeJson(join(root, '02-payload-preflight-read-model.json'), readModel);
  await writeFile(join(root, '08-security-negative-proof.md'), securityNegativeProof(), 'utf8');
  await writeFile(join(root, '09-ui-not-applicable.md'), uiNotApplicable(), 'utf8');
  await writeFile(join(root, '10-validation-commands.log'), validationLog(), 'utf8');
  await writeFile(join(root, '11-known-gaps.md'), knownGaps(), 'utf8');
  await writeFile(join(root, '12-claim-boundary.md'), claimBoundary(), 'utf8');
}

function sourceSnapshot() {
  return [
    '# App/Game Notification Payload Preflight Source Snapshot',
    '',
    '- Source contract: packages/parent-domain/src/app-game-notification-payload-preflight.ts',
    '- Source test: packages/parent-domain/tests/app-game-notification-payload-preflight.test.ts',
    '- Harness: scripts/test/app-game-notification-payload-preflight-proof.mjs',
    '- Source scheduler bridge: packages/parent-domain/src/app-game-notification-scheduler-bridge.ts',
    '- Generated from parent-domain contracts only; no provider, UI, child delivery, or adapter runtime is executed.',
    '',
  ].join('\n');
}

function securityNegativeProof() {
  return [
    '# Security Negative Proof',
    '',
    '- Scheduled rows require minimal alert id, family/device scope, severity, reason code, evidence ref, policy ref, and parent action link refs only.',
    '- Payload preflight rows require raw child evidence, raw URL/title, raw message text, screenshot/report, and sensitive provider metadata exclusions.',
    '- Schema tests reject provider payload template runtime claims and raw URL/title inclusion.',
    '- No provider credentials, provider receipts, cloud routing, durable outbox storage, child delivery, parent UI, or adapter dispatch are claimed.',
    '',
  ].join('\n');
}

function uiNotApplicable() {
  return [
    '# UI Not Applicable',
    '',
    'WP63 is a parent-domain payload preflight contract/proof. It does not add portal, desktop, child app, notification history, or parent preference UI.',
    '',
  ].join('\n');
}

function knownGaps() {
  return [
    '# Known Gaps',
    '',
    '- Provider payload template runtime remains unimplemented.',
    '- Provider delivery, retry workers, quiet-hours timers, receipt ingestion, and credentials remain unimplemented.',
    '- Durable production outbox storage, parent notification UI, child delivery, adapter dispatch, broad blocking, and platform proof remain unclaimed.',
    '- Parent preferences and provider-specific content rendering still need separate proof.',
    '',
  ].join('\n');
}

function claimBoundary() {
  return [
    '# Claim Boundary',
    '',
    'This proof only maps existing app/game scheduler bridge rows into payload preflight rows. Scheduled source rows require minimal payload fields, provider-template proof requirements, and sensitive-detail exclusions. Manual-required and unavailable source rows remain blocked.',
    '',
  ].join('\n');
}

function validationLog() {
  return [
    'cmd /c npm run build --workspace @ocentra-parent/parent-domain: PASS',
    'cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-notification-payload-preflight app-game-notification-scheduler-bridge app-game-notification-local-outbox-bridge: PASS',
  ].join('\n');
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}
