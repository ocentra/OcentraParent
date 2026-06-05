import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { spawnSync } from 'node:child_process';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const testOutputDir = join(repoRoot, 'test-results', 'app-game-notification-scheduler-bridge-proof');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', '59-notification-scheduler-bridge');
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', '59-notification-scheduler-bridge');
const timestamp = '2026-06-05T01:02:00Z';

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
  'app-game-notification-scheduler-bridge',
  'app-game-notification-local-outbox-bridge',
  'notification-local-outbox-scheduler-proof',
]);

const importDist = (name) => import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)));
const bridge = await importDist('app-game-notification-local-outbox-bridge.js');
const scheduler = await importDist('app-game-notification-scheduler-bridge.js');
const intent = await importDist('app-game-notification-intent.js');
const childUx = await importDist('app-game-child-facing-ux-rules.js');
const refs = await importDist('reference-primitives.js');

const source = bridge.buildAppGameNotificationLocalOutboxBridgeReadModel(
  bridgeOptions(),
  proofIntents(intent, childUx, refs)
);
const readModel = scheduler.buildAppGameNotificationSchedulerBridgeReadModel(schedulerOptions(), source);
const jsonl = scheduler.serializeAppGameNotificationSchedulerJsonl(readModel);
const schedulerRecords = scheduler.parseAppGameNotificationSchedulerJsonl(jsonl);
const proof = {
  proofMode: 'app-game-notification-scheduler-bridge',
  generatedAt: timestamp,
  scope: {
    bridge: 'App/game local outbox records to existing notification scheduler JSONL rows',
    sourceReadModel: source.bridgeId,
    schedulerReadModel: readModel.schedulerBridgeId,
    scheduledRecordCount: readModel.scheduledRecordCount,
    parsedSchedulerRecordCount: schedulerRecords.length,
    unscheduledManualRequiredCount: readModel.unscheduledManualRequiredCount,
    unscheduledUnavailableCount: readModel.unscheduledUnavailableCount,
  },
  nonClaims: {
    providerDeliveryRuntimeClaimed: readModel.providerDeliveryRuntimeClaimed,
    providerReceiptIngestionClaimed: readModel.providerReceiptIngestionClaimed,
    providerCredentialsClaimed: readModel.providerCredentialsClaimed,
    retryExecutionRuntimeClaimed: readModel.retryExecutionRuntimeClaimed,
    quietHoursTimerRuntimeClaimed: readModel.quietHoursTimerRuntimeClaimed,
    productionDurableOutboxStorageClaimed: readModel.productionDurableOutboxStorageClaimed,
    parentNotificationUiClaimed: readModel.parentNotificationUiClaimed,
    childDeliveryClaimed: readModel.childDeliveryClaimed,
    adapterDispatchClaimed: readModel.adapterDispatchClaimed,
  },
  proofPaths: {
    source: 'packages/parent-domain/src/app-game-notification-scheduler-bridge.ts',
    test: 'packages/parent-domain/tests/app-game-notification-scheduler-bridge.test.ts',
    harness: 'scripts/test/app-game-notification-scheduler-bridge-proof.mjs',
    schedulerJsonl: 'test-results/app-game-notification-scheduler-bridge-proof/scheduler-records.jsonl',
    appGameProofPack: 'output/app-game-plan-proof/59-notification-scheduler-bridge',
    appProofPack: 'output/app-plan-proof/59-notification-scheduler-bridge',
  },
};

assertProof(proof);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeFile(join(testOutputDir, 'scheduler-records.jsonl'), jsonl, 'utf8');
await writeJson(join(appGameProofDir, '01-scheduler-bridge-proof.json'), proof);
await writeJson(join(appProofDir, '01-scheduler-bridge-proof.json'), proof);
await writeFile(join(appGameProofDir, '10-validation-commands.log'), validationLog(), 'utf8');
await writeFile(join(appProofDir, '10-validation-commands.log'), validationLog(), 'utf8');

console.log('app-game-notification-scheduler-bridge-proof-ok');
console.log(`evidence=${join('test-results', 'app-game-notification-scheduler-bridge-proof', 'proof.json')}`);

function run(command, args) {
  const result = spawnSync(command, args, { cwd: repoRoot, stdio: 'inherit', shell: false });
  if (result.status !== 0) {
    throw new Error(`Command failed: ${command} ${args.join(' ')}`);
  }
}

function schedulerOptions() {
  return {
    generatedAt: timestamp,
    schedulerBridgeId: 'app-game-notification-scheduler-bridge-proof',
    schedulerArtifactRootRef: 'parent-owned-app-game-notification-scheduler-root',
    schedulerArtifactRef: 'parent-owned-app-game-notification-scheduler-jsonl-ref',
    schedulerNowAt: timestamp,
  };
}

function bridgeOptions() {
  return {
    family: { familyId: 'family-app-game-scheduler-proof' },
    parentAction: {
      actionReferenceId: 'parent-action-app-game-scheduler-proof',
      actor: { actorId: 'parent-app-game-scheduler-proof', role: refs.ParentActorRole.Parent },
      policyVersion: 'policy-app-game-notification-scheduler-proof-v1',
      createdAt: timestamp,
    },
    generatedAt: timestamp,
    bridgeId: 'app-game-notification-local-outbox-bridge-for-scheduler-proof',
    outboxRootRef: 'parent-owned-app-game-local-outbox-root-for-scheduler-proof',
    outboxFileRef: 'parent-owned-app-game-local-outbox-jsonl-for-scheduler-proof',
    localDataPathRef: 'parent-owned-app-game-local-outbox-data-path-for-scheduler-proof',
  };
}

function proofIntents(intent, childUx, refs) {
  const base = {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    notificationIntentId: 'notification-intent-time-limit-scheduler-proof',
    intentKind: intent.AppGameNotificationIntentKind.TimeLimitReached,
    intentStatus: intent.AppGameNotificationIntentStatus.LocalOutboxEligible,
    priority: intent.AppGameNotificationPriority.Urgent,
    device: {
      deviceId: 'device-app-game-scheduler-proof',
      childProfileId: 'child-app-game-scheduler-proof',
      label: 'Study PC',
      platform: refs.ParentPlatform.Windows,
    },
    targetKind: childUx.AppGameChildUxTargetKind.NativeGame,
    targetRef: 'target-native-game-scheduler-proof',
    notificationReasonCode: intent.AppGameNotificationReasonCode.TimeLimit,
    providerChannelPreference: 'in-app',
    parentTitleToken: intent.AppGameNotificationParentCopyToken.TimeLimitTitle,
    parentBodyToken: intent.AppGameNotificationParentCopyToken.TimeLimitBody,
    parentActionToken: intent.AppGameNotificationParentCopyToken.OpenParentReviewAction,
    childTitleToken: childUx.AppGameChildUxCopyToken.LimitReachedTitle,
    childBodyToken: childUx.AppGameChildUxCopyToken.LimitReachedBody,
    notificationRuleRef: 'notification-rule-app-game-time-limit-scheduler-proof',
    notificationStatusRef: 'notification-status-app-game-time-limit-scheduler-proof',
    policyRefs: ['policy-ref-app-game-time-limit-scheduler-proof'],
    auditRefs: ['audit-ref-app-game-time-limit-scheduler-proof'],
    evidenceReferences: [
      {
        evidenceReferenceId: 'evidence-ref-app-game-time-limit-scheduler-proof',
        kind: refs.ParentEvidenceReferenceKind.PolicyDecision,
        observedAt: timestamp,
      },
    ],
    childReasonReferences: [],
    childStatusReferences: ['child-status-app-game-time-limit-scheduler-proof'],
    approvalActionRef: null,
    timeBudgetDecisionRef: 'time-budget-decision-app-game-scheduler-proof',
    unknownCandidateRef: null,
    localOutboxRecordRef: 'local-outbox-record-app-game-time-limit-scheduler-proof',
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
    {
      ...base,
      notificationIntentId: 'notification-intent-suspicious-unknown-scheduler-proof',
      intentKind: intent.AppGameNotificationIntentKind.SuspiciousUnknown,
      priority: intent.AppGameNotificationPriority.Attention,
      targetKind: childUx.AppGameChildUxTargetKind.UnknownApp,
      notificationReasonCode: intent.AppGameNotificationReasonCode.SuspiciousUnknown,
      providerChannelPreference: 'email',
      parentTitleToken: intent.AppGameNotificationParentCopyToken.SuspiciousUnknownTitle,
      parentBodyToken: intent.AppGameNotificationParentCopyToken.SuspiciousUnknownBody,
      childTitleToken: childUx.AppGameChildUxCopyToken.NewAppTitle,
      childBodyToken: childUx.AppGameChildUxCopyToken.NewAppBody,
      localOutboxRecordRef: 'local-outbox-record-app-game-suspicious-unknown-scheduler-proof',
      timeBudgetDecisionRef: null,
      unknownCandidateRef: 'unknown-app-candidate-scheduler-proof',
    },
    manualIntent(base, intent, childUx),
    unavailableIntent(base, intent, childUx),
  ];
}

function manualIntent(base, intent, childUx) {
  return {
    ...base,
    notificationIntentId: 'notification-intent-manual-required-scheduler-proof',
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
    manualProofRequirements: ['provider preference setup before app game notification can be scheduled'],
    deliveryClaimState: intent.AppGameNotificationDeliveryClaimState.ManualRequired,
  };
}

function unavailableIntent(base, intent, childUx) {
  return {
    ...base,
    notificationIntentId: 'notification-intent-unavailable-scheduler-proof',
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
    manualProofRequirements: ['local evidence and policy readiness before unavailable notification can be scheduled'],
    deliveryClaimState: intent.AppGameNotificationDeliveryClaimState.ManualRequired,
  };
}

function assertProof(proof) {
  if (proof.scope.scheduledRecordCount !== 2 || proof.scope.unscheduledManualRequiredCount !== 1) {
    throw new Error(`Unexpected scheduler bridge counts: ${JSON.stringify(proof.scope)}`);
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(`Scheduler bridge claimed unavailable runtime behavior: ${JSON.stringify(proof.nonClaims)}`);
  }
}

function validationLog() {
  return [
    'cmd /c npm run build --workspace @ocentra-parent/parent-domain: PASS',
    'cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-notification-scheduler-bridge app-game-notification-local-outbox-bridge notification-local-outbox-scheduler-proof: PASS',
  ].join('\n');
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}
