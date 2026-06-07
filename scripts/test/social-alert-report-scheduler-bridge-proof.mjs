import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const generatedAt = '2026-06-07T08:12:00Z';
const outputDirectory = join(repoRoot, 'output', 'browser-plan-proof', 'social-alert-report-scheduler-bridge-proof');
const resultDirectory = join(repoRoot, 'test-results', 'social-alert-report-scheduler-bridge-proof');
const commands = [];

await main();

async function main() {
  await mkdir(outputDirectory, { recursive: true });
  await mkdir(resultDirectory, { recursive: true });

  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'social-alert-report-scheduler-bridge',
    'social-alert-report-local-outbox-bridge',
    'notification-local-outbox-scheduler-proof',
  ]);

  const bridge = await importDist('social-alert-report-local-outbox-bridge.js');
  const scheduler = await importDist('social-alert-report-scheduler-bridge.js');
  const sourceReadModel = bridge.buildSocialAlertReportLocalOutboxBridgeReadModel(bridgeOptions(), proofIntents());
  const readModel = scheduler.buildSocialAlertReportSchedulerBridgeReadModel(schedulerOptions(), sourceReadModel);
  const jsonl = scheduler.serializeSocialAlertReportSchedulerJsonl(readModel);
  const rereadRecords = scheduler.parseSocialAlertReportSchedulerJsonl(jsonl);
  const source = await readText('packages/parent-domain/src/social-alert-report-scheduler-bridge.ts');
  const test = await readText('packages/parent-domain/tests/social-alert-report-scheduler-bridge.test.ts');
  const socialFeature = await readText('docs/features/social-video-control.md');
  const socialExpectation = await readText('docs/expectations/social-video-control.md');
  const notificationExpectation = await readText('docs/expectations/notifications.md');
  const workpackReadme = await readText('docs/plans/browser-plan/social-platform-account-feed/readme.md');

  await writeFile(join(resultDirectory, 'scheduler-records.jsonl'), jsonl, 'utf8');

  const checks = [
    checkFile('packages/parent-domain/src/social-alert-report-scheduler-bridge.ts'),
    checkFile('packages/parent-domain/tests/social-alert-report-scheduler-bridge.test.ts'),
    checkFile('scripts/test/social-alert-report-scheduler-bridge-proof.mjs'),
    checkIncludes(
      source,
      'quietHoursTimerRuntimeClaimed: Schema.Literal(false)',
      'quiet-hours runtime non-claim guard'
    ),
    checkIncludes(source, 'retryExecutionRuntimeClaimed: Schema.Literal(false)', 'retry runtime non-claim guard'),
    checkIncludes(source, 'providerDeliveryRuntimeClaimed: Schema.Literal(false)', 'provider delivery non-claim guard'),
    checkIncludes(source, 'finalPolicyExecutionClaimed: Schema.Literal(false)', 'final policy non-claim guard'),
    checkIncludes(source, 'enforcementClaimed: Schema.Literal(false)', 'enforcement non-claim guard'),
    checkIncludes(test, 'quietHoursTimerRuntimeClaimed: true', 'quiet-hours overclaim rejection test'),
    checkIncludes(test, 'providerDeliveryObserved: true', 'provider delivery overclaim rejection test'),
    checkIncludes(socialFeature, 'alert/report scheduler bridge', 'social feature scheduler bridge note'),
    checkIncludes(
      socialExpectation,
      'Social alert/report scheduler bridge',
      'social expectation scheduler bridge note'
    ),
    checkIncludes(
      notificationExpectation,
      'social alert/report scheduler bridge',
      'notification expectation scheduler bridge note'
    ),
    checkIncludes(
      workpackReadme,
      'social-alert-report-scheduler-bridge-proof',
      'social README scheduler bridge proof note'
    ),
  ];
  const failures = checks.filter((check) => !check.pass).map((check) => check.label);

  assertNoForbiddenDetails(jsonl);
  assertNoUnsafeClaims(readModel, rereadRecords);

  const proof = {
    schemaVersion: 1,
    proofMode: 'social-alert-report-scheduler-bridge-proof',
    generatedAt,
    branch: await gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    commit: await gitOutput(['rev-parse', 'HEAD']),
    commands,
    checks,
    summary: {
      sourceRowCount: sourceReadModel.rows.length,
      scheduledRecordCount: readModel.scheduledRecordCount,
      unscheduledManualRequiredCount: readModel.unscheduledManualRequiredCount,
      unscheduledUnavailableCount: readModel.unscheduledUnavailableCount,
      jsonlRecordCount: rereadRecords.length,
      providerDeliveryRuntimeClaimed: readModel.providerDeliveryRuntimeClaimed,
      providerReceiptIngestionClaimed: readModel.providerReceiptIngestionClaimed,
      providerCredentialsClaimed: readModel.providerCredentialsClaimed,
      cloudRoutingClaimed: readModel.cloudRoutingClaimed,
      parentNotificationUiClaimed: readModel.parentNotificationUiClaimed,
      childDeliveryClaimed: readModel.childDeliveryClaimed,
      retryExecutionRuntimeClaimed: readModel.retryExecutionRuntimeClaimed,
      quietHoursTimerRuntimeClaimed: readModel.quietHoursTimerRuntimeClaimed,
      productionDurableOutboxStorageClaimed: readModel.productionDurableOutboxStorageClaimed,
      adapterDispatchClaimed: readModel.adapterDispatchClaimed,
      reportDeliveryExecutionClaimed: readModel.reportDeliveryExecutionClaimed,
      finalPolicyExecutionClaimed: readModel.finalPolicyExecutionClaimed,
      enforcementClaimed: readModel.enforcementClaimed,
    },
    claimsProved: [
      'Parent-owned social alert/report local outbox rows can be projected into the existing notification local outbox scheduler record schema',
      'Only linked local outbox rows are written to scheduler JSONL and reread through the existing scheduler parser',
      'Manual-required and unavailable rows remain visible in the scheduler bridge read model but are not written to scheduler JSONL',
      'The bridge carries quiet-hours and retry scheduler readiness as deterministic local due rows without claiming timer or retry-worker execution',
      'Provider delivery, receipts, credentials, cloud routing, parent/child notification UI delivery, report delivery execution, final policy execution, and enforcement remain unclaimed',
    ],
    claimsNotProved: [
      'provider adapter dispatch or provider delivery',
      'provider credentials or receipt ingestion',
      'production quiet-hours timer loop',
      'production retry worker execution',
      'durable production outbox storage',
      'parent notification UI delivery or child notification delivery',
      'external report delivery execution',
      'final policy evaluator execution',
      'connector or native-app runtime',
      'enforcement',
      'product checklist completion',
    ],
    evidence: {
      source: 'packages/parent-domain/src/social-alert-report-scheduler-bridge.ts',
      test: 'packages/parent-domain/tests/social-alert-report-scheduler-bridge.test.ts',
      existingSchedulerContract: 'packages/parent-domain/src/notification-local-outbox-scheduler-proof.ts',
      existingLocalOutboxBridge: 'packages/parent-domain/src/social-alert-report-local-outbox-bridge.ts',
      harness: 'scripts/test/social-alert-report-scheduler-bridge-proof.mjs',
      jsonl: 'test-results/social-alert-report-scheduler-bridge-proof/scheduler-records.jsonl',
      proof: 'test-results/social-alert-report-scheduler-bridge-proof/proof.json',
      readModel: 'test-results/social-alert-report-scheduler-bridge-proof/scheduler-bridge-read-model.json',
      manifest:
        'output/browser-plan-proof/social-alert-report-scheduler-bridge-proof/01-social-alert-report-scheduler-bridge-proof.md',
    },
    sourceReadModel,
    readModel,
    failures,
  };

  if (failures.length > 0) {
    throw new Error(`Social alert/report scheduler bridge proof failed:\n${failures.join('\n')}`);
  }

  await writeJson(join(resultDirectory, 'proof.json'), proof);
  await writeJson(join(resultDirectory, 'scheduler-bridge-read-model.json'), readModel);
  await writeFile(join(outputDirectory, '00-source-snapshot.md'), await sourceSnapshot(proof), 'utf8');
  await writeFile(
    join(outputDirectory, '01-social-alert-report-scheduler-bridge-proof.md'),
    markdownFor(proof),
    'utf8'
  );
  await writeFile(join(outputDirectory, '08-security-negative-proof.md'), securityProofFor(proof), 'utf8');
  await writeFile(join(outputDirectory, '10-validation-commands.log'), validationLogFor(proof), 'utf8');
  await writeFile(
    join(outputDirectory, 'ui-not-applicable.md'),
    '# UI Not Applicable\n\nThis proof adds a parent-domain scheduler bridge from social alert/report local outbox rows to the existing local notification scheduler record schema. It does not render portal UI or deliver notifications.\n',
    'utf8'
  );

  console.log('social-alert-report-scheduler-bridge-proof-ok=true');
  console.log(`proof=${relativePath(join(resultDirectory, 'proof.json'))}`);
  console.log(`manifest=${relativePath(join(outputDirectory, '01-social-alert-report-scheduler-bridge-proof.md'))}`);
}

function bridgeOptions() {
  return {
    family: { familyId: 'family-social-alert-report-scheduler-proof' },
    parentAction: {
      actionReferenceId: 'parent-action-social-alert-report-scheduler-proof',
      actor: { actorId: 'parent-social-alert-report-scheduler-proof', role: 'parent' },
      policyVersion: 'policy-social-alert-report-scheduler-proof-v1',
      createdAt: generatedAt,
    },
    generatedAt,
    bridgeId: 'social-alert-report-local-outbox-bridge-scheduler-proof',
    outboxRootRef: 'parent-owned-social-alert-report-local-outbox-root-for-scheduler-proof',
    outboxFileRef: 'parent-owned-social-alert-report-local-outbox-jsonl-ref-for-scheduler-proof',
    localDataPathRef: 'parent-owned-social-alert-report-local-outbox-data-path-ref-for-scheduler-proof',
  };
}

function schedulerOptions() {
  return {
    generatedAt,
    schedulerBridgeId: 'social-alert-report-scheduler-bridge-proof',
    schedulerArtifactRootRef: 'parent-owned-social-alert-report-scheduler-root',
    schedulerArtifactRef: 'parent-owned-social-alert-report-scheduler-jsonl-ref',
    schedulerNowAt: generatedAt,
  };
}

function proofIntents() {
  const base = {
    schemaVersion: 'v0.6',
    alertReportIntentId: 'social-alert-report-high-risk-scheduler-proof',
    intentKind: 'high-risk-signal',
    intentStatus: 'local-outbox-eligible',
    priority: 'urgent',
    severity: 'critical',
    device: {
      deviceId: 'device-social-alert-report-scheduler-proof',
      childProfileId: 'child-social-alert-report-scheduler-proof',
      label: 'Study Phone',
      platform: 'android',
    },
    notificationReasonCode: 'social-high-risk-signal',
    providerChannelPreference: 'in-app',
    parentTitleToken: 'social.alert.highRisk.title',
    parentBodyToken: 'social.alert.highRisk.body',
    parentActionToken: 'social.alert.action.openParentReview',
    dashboardPanelRefs: ['panel-feed-video-gates'],
    explanationSnapshotRef: 'social-explanation-snapshot-scheduler-proof',
    explanationEventRefs: ['social-explanation-event-scheduler-proof'],
    evidenceReferences: [
      {
        evidenceReferenceId: 'evidence-social-alert-report-scheduler-proof',
        kind: 'policy-decision',
        observedAt: generatedAt,
      },
    ],
    policyRefs: ['policy-ref-social-alert-report-scheduler-proof'],
    auditRefs: ['audit-ref-social-alert-report-scheduler-proof'],
    parentReportRef: null,
    parentActionRef: null,
    localOutboxRecordRef: 'local-outbox-record-social-alert-report-high-risk-scheduler-proof',
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
      alertReportIntentId: 'social-alert-report-account-approval-scheduler-proof',
      intentKind: 'account-approval-needed',
      priority: 'attention',
      severity: 'warning',
      notificationReasonCode: 'social-account-approval-needed',
      providerChannelPreference: 'email',
      parentTitleToken: 'social.alert.accountApproval.title',
      parentBodyToken: 'social.alert.accountApproval.body',
      parentActionRef: bridgeOptions().parentAction,
      localOutboxRecordRef: 'local-outbox-record-social-alert-report-account-approval-scheduler-proof',
    },
    {
      ...base,
      alertReportIntentId: 'social-alert-report-manual-required-scheduler-proof',
      intentKind: 'manual-required',
      intentStatus: 'manual-required',
      priority: 'attention',
      severity: 'warning',
      notificationReasonCode: 'social-manual-review-required',
      parentTitleToken: 'social.alert.manualRequired.title',
      parentBodyToken: 'social.alert.manualRequired.body',
      parentActionToken: 'social.alert.action.reviewManually',
      localOutboxRecordRef: null,
      manualProofRequirements: ['provider preference setup before social alert/report can be scheduled'],
      deliveryClaimState: 'manual-required',
    },
    {
      ...base,
      alertReportIntentId: 'social-alert-report-unavailable-scheduler-proof',
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
        'local evidence and policy readiness before unavailable social alert/report can be scheduled',
      ],
      deliveryClaimState: 'manual-required',
    },
  ];
}

async function sourceSnapshot(proof) {
  return [
    '# Social Alert Report Scheduler Bridge Source Snapshot',
    '',
    `- Branch: ${proof.branch}`,
    `- Commit: ${proof.commit}`,
    '- Scope: social alert/report local outbox bridge rows projected into existing local notification scheduler records.',
    '- Source inspected: social alert/report local outbox bridge, notification local outbox scheduler proof, social/video feature doc, social/video expectation doc, notifications expectation doc, and browser-plan social rollout gates.',
    '- Package exports were intentionally not changed because another lane owns packages/parent-domain/package.json.',
    '',
  ].join('\n');
}

function markdownFor(proof) {
  return [
    '# Social Alert Report Scheduler Bridge Proof',
    '',
    `Generated: ${proof.generatedAt}`,
    '',
    `Source bridge rows: ${proof.summary.sourceRowCount}`,
    `Scheduled local rows: ${proof.summary.scheduledRecordCount}`,
    `Manual-required unscheduled rows: ${proof.summary.unscheduledManualRequiredCount}`,
    `Unavailable unscheduled rows: ${proof.summary.unscheduledUnavailableCount}`,
    `Scheduler JSONL records reread: ${proof.summary.jsonlRecordCount}`,
    `Quiet-hours timer runtime claimed: ${proof.summary.quietHoursTimerRuntimeClaimed}`,
    `Retry execution runtime claimed: ${proof.summary.retryExecutionRuntimeClaimed}`,
    `Provider delivery runtime claimed: ${proof.summary.providerDeliveryRuntimeClaimed}`,
    `Final policy execution claimed: ${proof.summary.finalPolicyExecutionClaimed}`,
    `Enforcement claimed: ${proof.summary.enforcementClaimed}`,
    '',
    'This proof consumes the parent-owned social alert/report local outbox bridge',
    'and writes only linked local outbox rows into the existing notification',
    'local outbox scheduler record schema. The generated scheduler JSONL is',
    'reread through that scheduler parser. Manual-required and unavailable rows',
    'remain visible in the scheduler bridge read model but do not produce',
    'scheduler JSONL records.',
    '',
    'It proves a deterministic handoff into local scheduler records only. It',
    'does not claim provider dispatch, provider receipt ingestion, quiet-hours',
    'timer execution, retry worker execution, parent or child notification UI',
    'delivery, report delivery execution, final policy execution, connector or',
    'native runtime, or enforcement.',
    '',
  ].join('\n');
}

function securityProofFor(proof) {
  return [
    '# Social Alert Report Scheduler Bridge Security Negative Proof',
    '',
    `Provider delivery runtime claimed: ${proof.summary.providerDeliveryRuntimeClaimed}`,
    `Provider receipt ingestion claimed: ${proof.summary.providerReceiptIngestionClaimed}`,
    `Provider credentials claimed: ${proof.summary.providerCredentialsClaimed}`,
    `Parent notification UI claimed: ${proof.summary.parentNotificationUiClaimed}`,
    `Child delivery claimed: ${proof.summary.childDeliveryClaimed}`,
    `Quiet-hours timer runtime claimed: ${proof.summary.quietHoursTimerRuntimeClaimed}`,
    `Retry execution runtime claimed: ${proof.summary.retryExecutionRuntimeClaimed}`,
    `Report delivery execution claimed: ${proof.summary.reportDeliveryExecutionClaimed}`,
    `Final policy execution claimed: ${proof.summary.finalPolicyExecutionClaimed}`,
    `Enforcement claimed: ${proof.summary.enforcementClaimed}`,
    '',
    'The unit test rejects quiet-hours runtime overclaims, provider-delivery',
    'overclaims, and unsafe scheduler JSONL rows that attempt to include raw',
    'message text.',
    '',
  ].join('\n');
}

function validationLogFor(proof) {
  return proof.commands.map((command) => `${command.exitCode === 0 ? 'PASS' : 'FAIL'} ${command.command}`).join('\n');
}

function assertNoUnsafeClaims(readModel, records) {
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
    'reportDeliveryExecutionClaimed',
    'finalPolicyExecutionClaimed',
    'enforcementClaimed',
  ];
  for (const field of falseFields) {
    if (readModel[field] !== false) {
      throw new Error(`Unsafe scheduler bridge claim was true: ${field}`);
    }
  }
  for (const record of records) {
    if (record.providerDeliveryAttempted || record.providerDeliveryObserved || record.providerReceiptIngested) {
      throw new Error(`Unsafe scheduler record provider claim: ${record.schedulerEntryId}`);
    }
  }
}

function assertNoForbiddenDetails(serialized) {
  const lowerSerialized = serialized.toLowerCase();
  for (const fragment of [
    'http://',
    'https://',
    'screenshot-bytes',
    'raw-title-value',
    'raw-message-body',
    'sqlite-private-path',
    'oauth-secret',
    'provider-token',
    'report-body',
  ]) {
    if (lowerSerialized.includes(fragment)) {
      throw new Error(`Forbidden scheduler detail leaked: ${fragment}`);
    }
  }
}

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
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
