import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const generatedAt = '2026-06-07T08:18:00Z';
const outputDirectory = join(
  repoRoot,
  'output',
  'browser-plan-proof',
  'social-alert-report-audit-history-bridge-proof'
);
const resultDirectory = join(repoRoot, 'test-results', 'social-alert-report-audit-history-bridge-proof');
const commands = [];

await main();

async function main() {
  await mkdir(outputDirectory, { recursive: true });
  await mkdir(resultDirectory, { recursive: true });

  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/logging-domain']);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'social-alert-report-local-outbox-bridge',
    'social-alert-report-intent',
  ]);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/logging-domain',
    '--',
    'notification-audit-history',
    'notification-audit-history-handoff',
  ]);

  const localOutboxBridge = await importDist('parent-domain', 'social-alert-report-local-outbox-bridge.js');
  const auditHandoff = await importDist('logging-domain', 'notification-audit-history-handoff.js');
  const localOutboxReadModel = localOutboxBridge.buildSocialAlertReportLocalOutboxBridgeReadModel(
    bridgeOptions(),
    proofIntents()
  );
  const sourceRows = localOutboxReadModel.rows.map((row) => socialOutboxRowToAuditHandoffRow(auditHandoff, row));
  const readModel = auditHandoff.buildNotificationAuditHistoryHandoffReadModel(
    {
      handoffReadModelId: 'social-alert-report-audit-history-bridge-proof',
      generatedAt,
      sourceReadModelRef: localOutboxReadModel.bridgeId,
      sourceContractRefs: [
        'social-alert-report-local-outbox-bridge',
        'notification-audit-history-handoff',
        'notification-audit-history-contract',
        'social-video-control-feature-doc',
        'notification-expectations',
      ],
    },
    sourceRows
  );

  const source = await readText('packages/logging-domain/src/notification-audit-history-handoff.ts');
  const socialFeature = await readText('docs/features/social-video-control.md');
  const socialExpectation = await readText('docs/expectations/social-video-control.md');
  const notificationExpectation = await readText('docs/expectations/notifications.md');
  const workpackReadme = await readText('docs/plans/browser-plan/social-platform-account-feed/readme.md');
  const checks = [
    checkFile('scripts/test/social-alert-report-audit-history-bridge-proof.mjs'),
    checkFile('packages/logging-domain/src/notification-audit-history-handoff.ts'),
    checkFile('packages/logging-domain/tests/notification-audit-history-handoff.test.ts'),
    checkIncludes(source, 'providerDeliveryRuntimeClaimed: Schema.Literal(false)', 'provider delivery non-claim guard'),
    checkIncludes(source, 'parentNotificationUiClaimed: Schema.Literal(false)', 'parent UI non-claim guard'),
    checkIncludes(source, 'childDeliveryClaimed: Schema.Literal(false)', 'child delivery non-claim guard'),
    checkIncludes(socialFeature, 'social-alert-report-audit-history-bridge-proof', 'social feature audit bridge note'),
    checkIncludes(
      socialExpectation,
      'Social alert/report audit-history bridge',
      'social expectation audit bridge note'
    ),
    checkIncludes(notificationExpectation, 'social alert/report audit-history bridge', 'notification expectation note'),
    checkIncludes(workpackReadme, 'social-alert-report-audit-history-bridge-proof', 'social README audit bridge note'),
  ];
  const failures = checks.filter((check) => !check.pass).map((check) => check.label);

  assertNoUnsafeClaims(readModel);
  const proof = {
    schemaVersion: 1,
    proofMode: 'social-alert-report-audit-history-bridge-proof',
    generatedAt,
    branch: await gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    commit: await gitOutput(['rev-parse', 'HEAD']),
    commands,
    checks,
    summary: summarize(localOutboxReadModel, readModel),
    claimsProved: [
      'Social alert/report local outbox bridge rows parse before audit-history handoff',
      'Linked social alert/report local outbox rows become queued audit-history entries with evidence, policy, and audit refs',
      'Manual-required and unavailable social alert/report rows become blocked audit-history entries without queued provider sends',
      'Audit-history entries reuse the logging-domain redaction-safe payload and no Ocentra-hosted child data custody states',
      'Provider delivery, receipt ingestion, credentials, cloud routing, parent UI, child delivery, retry execution, quiet-hours timer execution, durable outbox storage, adapter dispatch, final policy execution, connector/native runtime, and enforcement remain unclaimed',
    ],
    claimsNotProved: [
      'provider adapter implementation or delivery',
      'provider webhook receipt ingestion',
      'provider credentials',
      'production retry worker or quiet-hours timer execution',
      'durable production local outbox storage',
      'parent notification history UI or parent preference UI',
      'child app, overlay, push, or local notification delivery',
      'external report delivery execution',
      'final policy evaluator execution',
      'connector or native-app runtime',
      'enforcement',
      'product checklist completion',
    ],
    evidence: {
      existingAuditHandoffSource: 'packages/logging-domain/src/notification-audit-history-handoff.ts',
      existingAuditHandoffTest: 'packages/logging-domain/tests/notification-audit-history-handoff.test.ts',
      existingAuditHistoryContract: 'packages/logging-domain/src/notification-audit-history.ts',
      socialLocalOutboxBridge: 'packages/parent-domain/src/social-alert-report-local-outbox-bridge.ts',
      harness: 'scripts/test/social-alert-report-audit-history-bridge-proof.mjs',
      handoffArtifact: 'test-results/social-alert-report-audit-history-bridge-proof/audit-history-handoff.json',
      proof: 'test-results/social-alert-report-audit-history-bridge-proof/proof.json',
      manifest:
        'output/browser-plan-proof/social-alert-report-audit-history-bridge-proof/01-social-alert-report-audit-history-bridge-proof.md',
    },
    sourceRows,
    readModel,
    failures,
  };

  if (failures.length > 0) {
    throw new Error(`Social alert/report audit-history bridge proof failed:\n${failures.join('\n')}`);
  }

  await writeJson(join(resultDirectory, 'audit-history-handoff.json'), readModel);
  await writeJson(join(resultDirectory, 'proof.json'), proof);
  await writeFile(join(outputDirectory, '00-source-snapshot.md'), await sourceSnapshot(proof), 'utf8');
  await writeFile(
    join(outputDirectory, '01-social-alert-report-audit-history-bridge-proof.md'),
    markdownFor(proof),
    'utf8'
  );
  await writeFile(join(outputDirectory, '08-security-negative-proof.md'), securityProofFor(proof), 'utf8');
  await writeFile(join(outputDirectory, '10-validation-commands.log'), validationLogFor(proof), 'utf8');
  await writeFile(
    join(outputDirectory, 'ui-not-applicable.md'),
    '# UI Not Applicable\n\nThis proof maps social alert/report local outbox rows to existing logging-domain audit-history handoff entries. It does not render notification history UI.\n',
    'utf8'
  );

  console.log('social-alert-report-audit-history-bridge-proof-ok=true');
  console.log(`proof=${relativePath(join(resultDirectory, 'proof.json'))}`);
  console.log(
    `manifest=${relativePath(join(outputDirectory, '01-social-alert-report-audit-history-bridge-proof.md'))}`
  );
}

async function importDist(packageName, moduleName) {
  return import(pathToFileURL(join(repoRoot, 'packages', packageName, 'dist', moduleName)).href);
}

function socialOutboxRowToAuditHandoffRow(auditHandoff, row) {
  const linked = row.status === 'linked-local-outbox-record';
  const sourceStatus = linked
    ? auditHandoff.NotificationAuditHistoryHandoffSourceStatus.QueuedLocalOutbox
    : row.status === 'unavailable'
      ? auditHandoff.NotificationAuditHistoryHandoffSourceStatus.Unavailable
      : auditHandoff.NotificationAuditHistoryHandoffSourceStatus.ManualRequired;

  return {
    handoffEntryId: `social-alert-report-audit-${row.bridgeRecordId}`,
    sourceStatus,
    sourceNotificationIntentRef: row.intent.alertReportIntentId,
    sourceOutboxRecordRef: row.outboxRecord?.entryId ?? null,
    providerChannelRef: row.outboxRecord?.envelope.providerChannel ?? row.intent.providerChannelPreference,
    reasonCodeRef: row.intent.notificationReasonCode,
    auditRefs: row.intent.auditRefs,
    evidenceRefs: row.intent.evidenceReferences.map((evidence) => evidence.evidenceReferenceId),
    policyRefs: row.intent.policyRefs,
    blockedReasonRefs: row.blockedReasonRefs,
  };
}

function summarize(localOutboxReadModel, auditReadModel) {
  return {
    sourceRows: localOutboxReadModel.rows.length,
    linkedLocalOutboxRows: localOutboxReadModel.linkedRecordCount,
    manualRequiredRows: localOutboxReadModel.manualRequiredCount,
    unavailableRows: localOutboxReadModel.unavailableCount,
    auditHistoryEntries: auditReadModel.auditHistoryEntries.length,
    queuedAuditEntryCount: auditReadModel.queuedAuditEntryCount,
    manualRequiredAuditEntryCount: auditReadModel.manualRequiredAuditEntryCount,
    unavailableAuditEntryCount: auditReadModel.unavailableAuditEntryCount,
    providerStatuses: countBy(auditReadModel.auditHistoryEntries.map((entry) => entry.providerStatus)),
    retryLifecycleStates: countBy(auditReadModel.auditHistoryEntries.map((entry) => entry.retryLifecycleState)),
    quietHoursStates: countBy(auditReadModel.auditHistoryEntries.map((entry) => entry.quietHoursState)),
    escalationStates: countBy(auditReadModel.auditHistoryEntries.map((entry) => entry.escalationState)),
    providerDeliveryRuntimeClaimed: auditReadModel.providerDeliveryRuntimeClaimed,
    providerReceiptIngestionClaimed: auditReadModel.providerReceiptIngestionClaimed,
    providerCredentialsClaimed: auditReadModel.providerCredentialsClaimed,
    cloudRoutingClaimed: auditReadModel.cloudRoutingClaimed,
    parentNotificationUiClaimed: auditReadModel.parentNotificationUiClaimed,
    childDeliveryClaimed: auditReadModel.childDeliveryClaimed,
    retryExecutionRuntimeClaimed: auditReadModel.retryExecutionRuntimeClaimed,
    quietHoursTimerRuntimeClaimed: auditReadModel.quietHoursTimerRuntimeClaimed,
    productionDurableOutboxStorageClaimed: auditReadModel.productionDurableOutboxStorageClaimed,
    adapterDispatchClaimed: auditReadModel.adapterDispatchClaimed,
  };
}

function bridgeOptions() {
  return {
    family: { familyId: 'family-social-alert-report-audit-history-proof' },
    parentAction: {
      actionReferenceId: 'parent-action-social-alert-report-audit-history-proof',
      actor: { actorId: 'parent-social-alert-report-audit-history-proof', role: 'parent' },
      policyVersion: 'policy-social-alert-report-audit-history-proof-v1',
      createdAt: generatedAt,
    },
    generatedAt,
    bridgeId: 'social-alert-report-local-outbox-bridge-audit-history-proof',
    outboxRootRef: 'parent-owned-social-alert-report-local-outbox-root-for-audit-history-proof',
    outboxFileRef: 'parent-owned-social-alert-report-local-outbox-jsonl-ref-for-audit-history-proof',
    localDataPathRef: 'parent-owned-social-alert-report-local-outbox-data-path-ref-for-audit-history-proof',
  };
}

function proofIntents() {
  const base = {
    schemaVersion: 'v0.6',
    alertReportIntentId: 'social-alert-report-high-risk-audit-history-proof',
    intentKind: 'high-risk-signal',
    intentStatus: 'local-outbox-eligible',
    priority: 'urgent',
    severity: 'critical',
    device: {
      deviceId: 'device-social-alert-report-audit-history-proof',
      childProfileId: 'child-social-alert-report-audit-history-proof',
      label: 'Study Phone',
      platform: 'android',
    },
    notificationReasonCode: 'social-high-risk-signal',
    providerChannelPreference: 'in-app',
    parentTitleToken: 'social.alert.highRisk.title',
    parentBodyToken: 'social.alert.highRisk.body',
    parentActionToken: 'social.alert.action.openParentReview',
    dashboardPanelRefs: ['panel-feed-video-gates'],
    explanationSnapshotRef: 'social-explanation-snapshot-audit-history-proof',
    explanationEventRefs: ['social-explanation-event-audit-history-proof'],
    evidenceReferences: [
      {
        evidenceReferenceId: 'evidence-social-alert-report-audit-history-proof',
        kind: 'policy-decision',
        observedAt: generatedAt,
      },
    ],
    policyRefs: ['policy-ref-social-alert-report-audit-history-proof'],
    auditRefs: ['audit-ref-social-alert-report-audit-history-proof'],
    parentReportRef: null,
    parentActionRef: null,
    localOutboxRecordRef: 'local-outbox-record-social-alert-report-high-risk-audit-history-proof',
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
      alertReportIntentId: 'social-alert-report-account-approval-audit-history-proof',
      intentKind: 'account-approval-needed',
      priority: 'attention',
      severity: 'warning',
      notificationReasonCode: 'social-account-approval-needed',
      providerChannelPreference: 'email',
      parentTitleToken: 'social.alert.accountApproval.title',
      parentBodyToken: 'social.alert.accountApproval.body',
      parentActionRef: bridgeOptions().parentAction,
      localOutboxRecordRef: 'local-outbox-record-social-alert-report-account-approval-audit-history-proof',
    },
    {
      ...base,
      alertReportIntentId: 'social-alert-report-manual-required-audit-history-proof',
      intentKind: 'manual-required',
      intentStatus: 'manual-required',
      priority: 'attention',
      severity: 'warning',
      notificationReasonCode: 'social-manual-review-required',
      parentTitleToken: 'social.alert.manualRequired.title',
      parentBodyToken: 'social.alert.manualRequired.body',
      parentActionToken: 'social.alert.action.reviewManually',
      localOutboxRecordRef: null,
      manualProofRequirements: ['provider preference setup before social alert/report audit history can queue'],
      deliveryClaimState: 'manual-required',
    },
    {
      ...base,
      alertReportIntentId: 'social-alert-report-unavailable-audit-history-proof',
      intentKind: 'capability-unavailable',
      intentStatus: 'unavailable',
      priority: 'info',
      severity: 'info',
      notificationReasonCode: 'social-capability-unavailable',
      parentTitleToken: 'social.alert.unavailable.title',
      parentBodyToken: 'social.alert.unavailable.body',
      parentActionToken: 'social.alert.action.reviewManually',
      localOutboxRecordRef: null,
      manualProofRequirements: [
        'local evidence and policy readiness before unavailable social alert/report audit history can queue',
      ],
      deliveryClaimState: 'manual-required',
    },
  ];
}

async function sourceSnapshot(proof) {
  return [
    '# Social Alert Report Audit History Bridge Source Snapshot',
    '',
    `- Branch: ${proof.branch}`,
    `- Commit: ${proof.commit}`,
    '- Scope: social alert/report local outbox bridge rows projected into the existing logging-domain notification audit-history handoff.',
    '- Source inspected: social alert/report local outbox bridge, notification audit-history handoff, notification audit-history contract, social/video feature doc, social/video expectation doc, notifications expectation doc, and browser-plan social rollout gates.',
    '- No package exports were changed; this proof reuses existing built modules directly.',
    '',
  ].join('\n');
}

function markdownFor(proof) {
  return [
    '# Social Alert Report Audit History Bridge Proof',
    '',
    `Generated: ${proof.generatedAt}`,
    '',
    `Source rows: ${proof.summary.sourceRows}`,
    `Queued audit entries: ${proof.summary.queuedAuditEntryCount}`,
    `Manual-required audit entries: ${proof.summary.manualRequiredAuditEntryCount}`,
    `Unavailable audit entries: ${proof.summary.unavailableAuditEntryCount}`,
    `Provider delivery runtime claimed: ${proof.summary.providerDeliveryRuntimeClaimed}`,
    `Parent notification UI claimed: ${proof.summary.parentNotificationUiClaimed}`,
    `Child delivery claimed: ${proof.summary.childDeliveryClaimed}`,
    `Quiet-hours timer runtime claimed: ${proof.summary.quietHoursTimerRuntimeClaimed}`,
    `Retry execution runtime claimed: ${proof.summary.retryExecutionRuntimeClaimed}`,
    '',
    'This proof consumes the social alert/report local outbox bridge and maps',
    'its rows into the existing logging-domain notification audit-history',
    'handoff. Linked local outbox rows become queued audit-history entries;',
    'manual-required and unavailable rows become blocked audit-history entries.',
    '',
    'It does not claim provider delivery, provider receipt ingestion, provider',
    'credentials, parent notification UI, child delivery, retry worker',
    'execution, quiet-hours timer execution, report delivery execution, final',
    'policy execution, connector/native runtime, or enforcement.',
    '',
  ].join('\n');
}

function securityProofFor(proof) {
  return [
    '# Social Alert Report Audit History Bridge Security Negative Proof',
    '',
    `Provider delivery runtime claimed: ${proof.summary.providerDeliveryRuntimeClaimed}`,
    `Provider receipt ingestion claimed: ${proof.summary.providerReceiptIngestionClaimed}`,
    `Provider credentials claimed: ${proof.summary.providerCredentialsClaimed}`,
    `Parent notification UI claimed: ${proof.summary.parentNotificationUiClaimed}`,
    `Child delivery claimed: ${proof.summary.childDeliveryClaimed}`,
    `Quiet-hours timer runtime claimed: ${proof.summary.quietHoursTimerRuntimeClaimed}`,
    `Retry execution runtime claimed: ${proof.summary.retryExecutionRuntimeClaimed}`,
    `Adapter dispatch claimed: ${proof.summary.adapterDispatchClaimed}`,
    '',
    'The existing logging-domain handoff contract rejects queued audit-history',
    'rows without source outbox refs, manual/unavailable rows without blocked',
    'reason refs, and read models that overclaim provider/runtime/UI/child',
    'delivery fields.',
    '',
  ].join('\n');
}

function assertNoUnsafeClaims(readModel) {
  const falseFields = [
    'providerDeliveryRuntimeClaimed',
    'providerReceiptIngestionClaimed',
    'providerCredentialsClaimed',
    'cloudRoutingClaimed',
    'parentNotificationUiClaimed',
    'childDeliveryClaimed',
    'retryExecutionRuntimeClaimed',
    'quietHoursTimerRuntimeClaimed',
    'productionDurableOutboxStorageClaimed',
    'adapterDispatchClaimed',
  ];
  for (const field of falseFields) {
    if (readModel[field] !== false) {
      throw new Error(`Unsafe audit-history handoff claim was true: ${field}`);
    }
  }
  for (const entry of readModel.auditHistoryEntries) {
    if (entry.sendAttemptExecuted || entry.retryExecutionObserved || entry.webhookReceiptIngested) {
      throw new Error(`Unsafe audit-history entry runtime claim: ${entry.auditEntryId}`);
    }
  }
}

function countBy(values) {
  return values.reduce((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}

function validationLogFor(proof) {
  return proof.commands.map((command) => `${command.exitCode === 0 ? 'PASS' : 'FAIL'} ${command.command}`).join('\n');
}

function checkFile(path) {
  return {
    label: `${path} exists`,
    pass: existsSync(join(repoRoot, path)),
  };
}

function checkIncludes(text, expected, label) {
  return {
    label,
    pass: text.includes(expected),
  };
}

async function readText(path) {
  return readFile(join(repoRoot, path), 'utf8');
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}

async function runCommand(command, args) {
  const commandLine = [command, ...args].join(' ');
  const result = await new Promise((resolve) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: 'inherit', shell: false });
    child.on('close', (exitCode) => resolve({ exitCode: exitCode ?? 1 }));
  });
  commands.push({ command: commandLine, exitCode: result.exitCode });
  if (result.exitCode !== 0) {
    throw new Error(`${commandLine} exited with ${result.exitCode}`);
  }
}

async function gitOutput(args) {
  const result = await new Promise((resolve) => {
    const child = spawn('git', args, { cwd: repoRoot, stdio: ['ignore', 'pipe', 'pipe'], shell: false });
    let stdout = '';
    child.stdout.on('data', (chunk) => {
      stdout += chunk.toString();
    });
    child.on('close', (exitCode) => resolve({ exitCode: exitCode ?? 1, stdout }));
  });
  if (result.exitCode !== 0) {
    return 'unknown';
  }
  return result.stdout.trim();
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}
