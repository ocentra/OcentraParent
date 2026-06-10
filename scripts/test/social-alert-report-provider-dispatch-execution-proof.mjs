import { execFileSync } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const generatedAt = '2026-06-08T22:47:00Z';
const outputDirectory = join(
  repoRoot,
  'output',
  'browser-plan-proof',
  'social-alert-report-provider-dispatch-execution-proof'
);
const resultDirectory = join(repoRoot, 'test-results', 'social-alert-report-provider-dispatch-execution-proof');
const proofPath = join(resultDirectory, 'proof.json');
const manifestPath = join(outputDirectory, '01-social-alert-report-provider-dispatch-execution-proof.md');
const commands = [];

await main();

async function main() {
  await mkdir(outputDirectory, { recursive: true });
  await mkdir(resultDirectory, { recursive: true });

  run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  run('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'social-alert-report-provider-dispatch-execution.test.ts',
  ]);

  const dispatch = await importDist('social-alert-report-provider-dispatch-execution.js');
  const localOutbox = await importDist('social-alert-report-local-outbox-bridge.js');
  const preflight = await importDist('social-alert-report-provider-preflight-proof.js');
  const status = await importDist('social-alert-report-provider-status-handoff-proof.js');
  const receipt = await importDist('social-alert-report-provider-receipt-boundary-proof.js');

  const intents = proofIntents();
  const outboxBridge = localOutbox.buildSocialAlertReportLocalOutboxBridgeReadModel(bridgeOptions(), intents);
  const localRecords = outboxBridge.rows.flatMap((row) => (row.outboxRecord === null ? [] : [row.outboxRecord]));
  const preflightReadModel = preflight.buildSocialAlertReportProviderPreflightReadModel(
    {
      generatedAt,
      providerPreflightId: 'social-alert-report-provider-preflight-for-dispatch-proof',
      sourceContractRefs: ['social-alert-report-intent', 'social-alert-report-provider-dispatch-execution'],
    },
    intents
  );
  const statusReadModel = status.buildSocialAlertReportProviderStatusHandoffReadModel(
    {
      generatedAt,
      handoffId: 'social-alert-report-provider-status-handoff-for-dispatch-proof',
      sourceContractRefs: [
        'social-alert-report-provider-preflight-proof',
        'v0-8-notification-provider-status-boundary',
      ],
    },
    preflightReadModel
  );
  const receiptReadModel = receipt.buildSocialAlertReportProviderReceiptBoundaryReadModel(
    {
      generatedAt,
      receiptBoundaryId: 'social-alert-report-provider-receipt-boundary-for-dispatch-proof',
      sourceContractRefs: [
        'social-alert-report-provider-status-handoff-proof',
        'v0-8-notification-provider-status-boundary',
      ],
    },
    statusReadModel
  );
  const dispatchReadModel = dispatch.buildSocialAlertReportProviderDispatchExecutionReadModel(
    {
      generatedAt,
      dispatchExecutionId: 'social-alert-report-provider-dispatch-execution-proof',
    },
    receiptReadModel,
    localRecords
  );
  const summary = dispatch.summarizeSocialAlertReportProviderDispatchExecution(dispatchReadModel);

  const proof = {
    schemaVersion: 1,
    proofMode: 'social-alert-report-provider-dispatch-execution-proof',
    generatedAt: new Date().toISOString(),
    branch: git(['branch', '--show-current']),
    commit: git(['rev-parse', 'HEAD']),
    baseCommit: git(['rev-parse', 'origin/main']),
    commands,
    summary,
    sourceChain: {
      localOutboxLinkedRecords: outboxBridge.linkedRecordCount,
      receiptBoundaryDispatchRequiredRows: receiptReadModel.providerDispatchRequiredCount,
      localDispatchPacketReadyRows: dispatchReadModel.localDispatchPacketReadyCount,
      manualRequiredRows: dispatchReadModel.manualRequiredCount,
      providerUnavailableRows: dispatchReadModel.providerUnavailableCount,
    },
    noClaimBoundaries: {
      providerDeliveryAttempted: dispatchReadModel.providerDeliveryAttempted,
      providerDeliveryObserved: dispatchReadModel.providerDeliveryObserved,
      providerReceiptIngested: dispatchReadModel.providerReceiptIngested,
      providerWebhookRuntimeClaimed: dispatchReadModel.providerWebhookRuntimeClaimed,
      providerCredentialsClaimed: dispatchReadModel.providerCredentialsClaimed,
      cloudRoutingClaimed: dispatchReadModel.cloudRoutingClaimed,
      parentNotificationUiDeliveryClaimed: dispatchReadModel.parentNotificationUiDeliveryClaimed,
      reportDeliveryExecutionClaimed: dispatchReadModel.reportDeliveryExecutionClaimed,
      finalPolicyExecutionClaimed: dispatchReadModel.finalPolicyExecutionClaimed,
      connectorNativeRuntimeClaimed: dispatchReadModel.connectorNativeRuntimeClaimed,
      enforcementClaimed: dispatchReadModel.enforcementClaimed,
    },
    dispatchReadModel,
    evidence: {
      source: 'packages/parent-domain/src/social-alert-report-provider-dispatch-execution.ts',
      test: 'packages/parent-domain/tests/social-alert-report-provider-dispatch-execution.test.ts',
      proof: 'test-results/social-alert-report-provider-dispatch-execution-proof/proof.json',
      manifest:
        'output/browser-plan-proof/social-alert-report-provider-dispatch-execution-proof/01-social-alert-report-provider-dispatch-execution-proof.md',
    },
  };

  const failures = Object.entries({
    oneDispatchPacketReady: proof.summary.localDispatchPacketReadyCount === 1,
    localOutboxSourceUsed: proof.sourceChain.localOutboxLinkedRecords === 1,
    noProviderDeliveryAttempted: proof.noClaimBoundaries.providerDeliveryAttempted === false,
    noProviderReceiptIngested: proof.noClaimBoundaries.providerReceiptIngested === false,
    noFinalPolicyExecution: proof.noClaimBoundaries.finalPolicyExecutionClaimed === false,
    noEnforcement: proof.noClaimBoundaries.enforcementClaimed === false,
  })
    .filter(([, pass]) => !pass)
    .map(([name]) => name);

  if (failures.length > 0) {
    throw new Error(`Social provider dispatch execution proof failed: ${failures.join(', ')}`);
  }

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(manifestPath, `${markdownFor(proof)}\n`);
  console.log('social-alert-report-provider-dispatch-execution-proof-ok=true');
  console.log(`proof=${relativePath(proofPath)}`);
  console.log(`manifest=${relativePath(manifestPath)}`);
}

function run(command, args) {
  const commandLine = [command, ...args].join(' ');
  execFileSync(command, args, { cwd: repoRoot, stdio: 'inherit' });
  commands.push({ command: commandLine, exitCode: 0 });
}

function bridgeOptions() {
  return {
    family: { familyId: 'family-social-provider-dispatch-proof' },
    parentAction: {
      actionReferenceId: 'parent-action-social-provider-dispatch-proof',
      actor: { actorId: 'parent-social-provider-dispatch-proof', role: 'parent' },
      policyVersion: 'policy-social-provider-dispatch-proof-v1',
      createdAt: generatedAt,
    },
    generatedAt,
    bridgeId: 'social-alert-report-local-outbox-bridge-dispatch-proof',
    outboxRootRef: 'parent-owned-social-provider-dispatch-outbox-root',
    outboxFileRef: 'parent-owned-social-provider-dispatch-outbox-jsonl-ref',
    localDataPathRef: 'parent-owned-social-provider-dispatch-local-data-path-ref',
  };
}

function proofIntents() {
  const base = {
    schemaVersion: 'v0.6',
    alertReportIntentId: 'social-provider-dispatch-high-risk-proof',
    intentKind: 'high-risk-signal',
    intentStatus: 'local-outbox-eligible',
    priority: 'urgent',
    severity: 'critical',
    device: {
      deviceId: 'device-social-provider-dispatch-proof',
      childProfileId: 'child-social-provider-dispatch-proof',
      label: 'Study Phone',
      platform: 'android',
    },
    notificationReasonCode: 'social-high-risk-signal',
    providerChannelPreference: 'in-app',
    parentTitleToken: 'social.alert.highRisk.title',
    parentBodyToken: 'social.alert.highRisk.body',
    parentActionToken: 'social.alert.action.openParentReview',
    dashboardPanelRefs: ['panel-feed-video-gates'],
    explanationSnapshotRef: 'social-explanation-snapshot-provider-dispatch-proof',
    explanationEventRefs: ['social-explanation-event-provider-dispatch-proof'],
    evidenceReferences: [
      {
        evidenceReferenceId: 'evidence-social-provider-dispatch-proof',
        kind: 'policy-decision',
        observedAt: generatedAt,
      },
    ],
    policyRefs: ['policy-ref-social-provider-dispatch-proof'],
    auditRefs: ['audit-ref-social-provider-dispatch-proof'],
    parentReportRef: null,
    parentActionRef: null,
    localOutboxRecordRef: 'local-outbox-social-provider-dispatch-proof',
    providerAttemptRefs: [],
    providerReceiptRefs: [],
    manualProofRequirements: [],
    minimalPayloadFields: [
      'alert-id',
      'family-device-scope',
      'severity',
      'reason-code',
      'evidence-ref',
      'policy-ref',
      'explanation-ref',
      'parent-action-link-ref',
    ],
    deliveryClaimState: 'local-outbox-only',
    rawAccountDataIncluded: false,
    rawVideoContentIncluded: false,
    rawMessageContentIncluded: false,
    screenshotIncluded: false,
    providerDeliveryAttempted: false,
    providerDeliveryObserved: false,
    providerReceiptIngested: false,
    cloudRoutingClaimed: false,
    parentNotificationUiClaimed: false,
    reportDeliveryClaimed: false,
    finalPolicyDecisionClaimed: false,
    enforcementClaimed: false,
    adapterDispatchState: 'not-dispatched',
    adapterActionClaimed: false,
    createdAt: generatedAt,
  };

  return [
    base,
    {
      ...base,
      alertReportIntentId: 'social-provider-dispatch-manual-required-proof',
      intentKind: 'manual-required',
      intentStatus: 'manual-required',
      priority: 'attention',
      severity: 'warning',
      notificationReasonCode: 'social-manual-review-required',
      parentTitleToken: 'social.alert.manualRequired.title',
      parentBodyToken: 'social.alert.manualRequired.body',
      parentActionToken: 'social.alert.action.reviewManually',
      localOutboxRecordRef: null,
      deliveryClaimState: 'manual-required',
      manualProofRequirements: ['manual-proof-social-provider-dispatch-required'],
    },
    {
      ...base,
      alertReportIntentId: 'social-provider-dispatch-unavailable-proof',
      intentKind: 'capability-unavailable',
      intentStatus: 'unavailable',
      priority: 'info',
      severity: 'info',
      notificationReasonCode: 'social-capability-unavailable',
      parentTitleToken: 'social.alert.unavailable.title',
      parentBodyToken: 'social.alert.unavailable.body',
      parentActionToken: 'social.alert.action.reviewManually',
      localOutboxRecordRef: null,
      deliveryClaimState: 'manual-required',
      manualProofRequirements: ['manual-proof-social-provider-dispatch-unavailable'],
    },
  ];
}

function markdownFor(proof) {
  return [
    '# Social Alert Report Provider Dispatch Execution Proof',
    '',
    `Generated: ${proof.generatedAt}`,
    `Branch: ${proof.branch}`,
    `Commit: ${proof.commit}`,
    '',
    `Local dispatch packet ready rows: ${proof.summary.localDispatchPacketReadyCount}`,
    `Manual-required rows: ${proof.summary.manualRequiredCount}`,
    `Provider-unavailable rows: ${proof.summary.providerUnavailableCount}`,
    '',
    'This proof consumes the existing social alert/report local outbox bridge,',
    'provider preflight, provider status handoff, and provider receipt boundary',
    'contracts. It prepares a local redaction-safe dispatch packet only for the',
    'row that has both a provider-dispatch-required receipt boundary and a parsed',
    'local outbox record.',
    '',
    'It does not claim provider delivery observed, provider receipt ingestion,',
    'provider webhook runtime, provider credentials, cloud routing, parent',
    'notification UI delivery, report delivery execution, final policy execution,',
    'connector/native runtime, or enforcement.',
  ].join('\n');
}

function importDist(fileName) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', fileName)).href);
}

function git(args) {
  return execFileSync('git', args, { cwd: repoRoot, encoding: 'utf8' }).trim();
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}
