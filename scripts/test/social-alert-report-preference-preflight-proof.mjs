import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const generatedAt = '2026-06-07T08:27:00Z';
const outputDirectory = join(
  repoRoot,
  'output',
  'browser-plan-proof',
  'social-alert-report-preference-preflight-proof'
);
const resultDirectory = join(repoRoot, 'test-results', 'social-alert-report-preference-preflight-proof');
const commands = [];

await main();

async function main() {
  await mkdir(outputDirectory, { recursive: true });
  await mkdir(resultDirectory, { recursive: true });

  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']));
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/parent-domain',
      '--',
      'social-alert-report-preference-preflight',
      'social-alert-report-scheduler-bridge',
      'social-alert-report-local-outbox-bridge',
    ])
  );

  const bridge = await importDist('social-alert-report-local-outbox-bridge.js');
  const scheduler = await importDist('social-alert-report-scheduler-bridge.js');
  const preference = await importDist('social-alert-report-preference-preflight.js');
  const sourceReadModel = bridge.buildSocialAlertReportLocalOutboxBridgeReadModel(bridgeOptions(), proofIntents());
  const schedulerReadModel = scheduler.buildSocialAlertReportSchedulerBridgeReadModel(
    schedulerOptions(),
    sourceReadModel
  );
  const readModel = preference.buildSocialAlertReportPreferencePreflightReadModel(
    preferenceOptions(),
    schedulerReadModel
  );
  const source = await readText('packages/parent-domain/src/social-alert-report-preference-preflight.ts');
  const test = await readText('packages/parent-domain/tests/social-alert-report-preference-preflight.test.ts');
  const socialFeature = await readText('docs/features/social-video-control.md');
  const socialExpectation = await readText('docs/expectations/social-video-control.md');
  const notificationExpectation = await readText('docs/expectations/notifications.md');
  const workpackReadme = await readText('docs/plans/browser-plan/social-platform-account-feed/readme.md');

  const checks = [
    checkFile('packages/parent-domain/src/social-alert-report-preference-preflight.ts'),
    checkFile('packages/parent-domain/tests/social-alert-report-preference-preflight.test.ts'),
    checkFile('scripts/test/social-alert-report-preference-preflight-proof.mjs'),
    checkIncludes(source, 'parentNotificationPreferenceUiClaimed: Schema.Literal(false)', 'preference UI non-claim'),
    checkIncludes(source, 'quietHoursTimerRuntimeClaimed: Schema.Literal(false)', 'quiet-hours runtime non-claim'),
    checkIncludes(source, 'providerDeliveryRuntimeClaimed: Schema.Literal(false)', 'provider delivery non-claim'),
    checkIncludes(source, 'finalPolicyExecutionClaimed: Schema.Literal(false)', 'final policy non-claim'),
    checkIncludes(source, 'enforcementClaimed: Schema.Literal(false)', 'enforcement non-claim'),
    checkIncludes(test, 'quietHoursTimerRuntimeClaimed: true', 'quiet-hours overclaim rejection test'),
    checkIncludes(test, 'providerDeliveryRuntimeClaimed: true', 'provider delivery overclaim rejection test'),
    checkIncludes(socialFeature, 'alert/report preference preflight', 'social feature preference preflight note'),
    checkIncludes(
      socialExpectation,
      'Social alert/report preference preflight',
      'social expectation preference preflight note'
    ),
    checkIncludes(
      notificationExpectation,
      'social alert/report preference preflight',
      'notification expectation preference preflight note'
    ),
    checkIncludes(
      workpackReadme,
      'social-alert-report-preference-preflight-proof',
      'social README preference preflight proof note'
    ),
  ];

  assertNoUnsafeClaims(readModel);
  const failures = checks.filter((check) => !check.pass).map((check) => check.label);
  const proof = {
    schemaVersion: 1,
    proofMode: 'social-alert-report-preference-preflight-proof',
    generatedAt,
    branch: await gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    commit: await gitOutput(['rev-parse', 'HEAD']),
    commands,
    checks,
    summary: {
      sourceSchedulerRows: schedulerReadModel.rows.length,
      parentPreferenceRequiredCount: readModel.parentPreferenceRequiredCount,
      manualRequiredCount: readModel.manualRequiredCount,
      unavailableCount: readModel.unavailableCount,
      parentNotificationPreferenceUiClaimed: readModel.parentNotificationPreferenceUiClaimed,
      parentNotificationHistoryUiClaimed: readModel.parentNotificationHistoryUiClaimed,
      parentFrequencyControlUiClaimed: readModel.parentFrequencyControlUiClaimed,
      quietHoursTimerRuntimeClaimed: readModel.quietHoursTimerRuntimeClaimed,
      providerDeliveryRuntimeClaimed: readModel.providerDeliveryRuntimeClaimed,
      providerReceiptIngestionClaimed: readModel.providerReceiptIngestionClaimed,
      providerCredentialsClaimed: readModel.providerCredentialsClaimed,
      cloudRoutingClaimed: readModel.cloudRoutingClaimed,
      childDeliveryClaimed: readModel.childDeliveryClaimed,
      retryExecutionRuntimeClaimed: readModel.retryExecutionRuntimeClaimed,
      productionDurableOutboxStorageClaimed: readModel.productionDurableOutboxStorageClaimed,
      adapterDispatchClaimed: readModel.adapterDispatchClaimed,
      reportDeliveryExecutionClaimed: readModel.reportDeliveryExecutionClaimed,
      finalPolicyExecutionClaimed: readModel.finalPolicyExecutionClaimed,
      enforcementClaimed: readModel.enforcementClaimed,
    },
    claimsProved: [
      'Social alert/report scheduler bridge rows can be projected into parent notification preference preflight rows',
      'Scheduled local rows require parent notification preference setup, frequency control setup, and quiet-hours policy proof before delivery can be claimed',
      'Manual-required and unavailable scheduler rows stay blocked before preference preflight',
      'The proof reuses the existing notification preference and quiet-hours state contracts',
    ],
    claimsNotProved: [
      'parent notification preference UI',
      'parent notification history UI',
      'parent frequency control UI',
      'production quiet-hours timer execution',
      'provider dispatch or provider delivery',
      'provider credentials or receipt ingestion',
      'child delivery',
      'retry worker execution',
      'durable production outbox storage',
      'external report delivery execution',
      'final policy evaluator execution',
      'connector or native-app runtime',
      'enforcement',
      'product checklist completion',
    ],
    evidence: {
      source: 'packages/parent-domain/src/social-alert-report-preference-preflight.ts',
      test: 'packages/parent-domain/tests/social-alert-report-preference-preflight.test.ts',
      schedulerBridge: 'packages/parent-domain/src/social-alert-report-scheduler-bridge.ts',
      localOutboxBridge: 'packages/parent-domain/src/social-alert-report-local-outbox-bridge.ts',
      harness: 'scripts/test/social-alert-report-preference-preflight-proof.mjs',
      proof: 'test-results/social-alert-report-preference-preflight-proof/proof.json',
      readModel: 'test-results/social-alert-report-preference-preflight-proof/preference-preflight-read-model.json',
      manifest:
        'output/browser-plan-proof/social-alert-report-preference-preflight-proof/01-social-alert-report-preference-preflight-proof.md',
    },
    sourceReadModel,
    schedulerReadModel,
    readModel,
    failures,
  };

  if (failures.length > 0) {
    throw new Error(`Social alert/report preference preflight proof failed:\n${failures.join('\n')}`);
  }

  await writeJson(join(resultDirectory, 'proof.json'), proof);
  await writeJson(join(resultDirectory, 'preference-preflight-read-model.json'), readModel);
  await writeFile(join(outputDirectory, '00-source-snapshot.md'), sourceSnapshotFor(proof), 'utf8');
  await writeFile(
    join(outputDirectory, '01-social-alert-report-preference-preflight-proof.md'),
    markdownFor(proof),
    'utf8'
  );
  await writeFile(join(outputDirectory, '08-security-negative-proof.md'), securityProofFor(proof), 'utf8');
  await writeFile(join(outputDirectory, '10-validation-commands.log'), validationLogFor(proof), 'utf8');
  await writeFile(
    join(outputDirectory, 'ui-not-applicable.md'),
    '# UI Not Applicable\n\nThis proof adds a parent-domain social alert/report preference preflight boundary. It does not render parent notification preference UI, notification history UI, child UI, or provider delivery UI.\n',
    'utf8'
  );

  console.log('social-alert-report-preference-preflight-proof-ok=true');
  console.log(`proof=${relativePath(join(resultDirectory, 'proof.json'))}`);
  console.log(
    `manifest=${relativePath(join(outputDirectory, '01-social-alert-report-preference-preflight-proof.md'))}`
  );
}

function preferenceOptions() {
  return {
    generatedAt,
    preferencePreflightId: 'social-alert-report-preference-preflight-proof',
    sourceContractRefs: [
      'social-alert-report-scheduler-bridge',
      'notification-parent-preference-boundary',
      'notification-quiet-hours-policy-boundary',
      'social-video-control-expectation',
    ],
  };
}

function schedulerOptions() {
  return {
    generatedAt,
    schedulerBridgeId: 'social-alert-report-scheduler-bridge-for-preference-preflight',
    schedulerArtifactRootRef: 'parent-owned-social-alert-report-scheduler-root-for-preference',
    schedulerArtifactRef: 'parent-owned-social-alert-report-scheduler-jsonl-ref-for-preference',
    schedulerNowAt: generatedAt,
  };
}

function bridgeOptions() {
  return {
    family: { familyId: 'family-social-alert-report-preference-preflight' },
    parentAction: {
      actionReferenceId: 'parent-action-social-alert-report-preference-preflight',
      actor: { actorId: 'parent-social-alert-report-preference-preflight', role: 'parent' },
      policyVersion: 'policy-social-alert-report-preference-preflight-v1',
      createdAt: generatedAt,
    },
    generatedAt,
    bridgeId: 'social-alert-report-local-outbox-bridge-for-preference-preflight',
    outboxRootRef: 'parent-owned-social-alert-report-local-outbox-root-for-preference',
    outboxFileRef: 'parent-owned-social-alert-report-local-outbox-jsonl-for-preference',
    localDataPathRef: 'parent-owned-social-alert-report-local-outbox-data-path-for-preference',
  };
}

function proofIntents() {
  const base = {
    schemaVersion: 'v0.6',
    alertReportIntentId: 'social-alert-report-high-risk-preference',
    intentKind: 'high-risk-signal',
    intentStatus: 'local-outbox-eligible',
    priority: 'urgent',
    severity: 'critical',
    device: {
      deviceId: 'device-social-alert-report-preference-preflight',
      childProfileId: 'child-social-alert-report-preference-preflight',
      label: 'Study Phone',
      platform: 'android',
    },
    notificationReasonCode: 'social-high-risk-signal',
    providerChannelPreference: 'in-app',
    parentTitleToken: 'social.alert.highRisk.title',
    parentBodyToken: 'social.alert.highRisk.body',
    parentActionToken: 'social.alert.action.openParentReview',
    dashboardPanelRefs: ['panel-feed-video-gates'],
    explanationSnapshotRef: 'social-explanation-snapshot-preference',
    explanationEventRefs: ['social-explanation-event-preference'],
    evidenceReferences: [
      {
        evidenceReferenceId: 'evidence-social-alert-report-preference',
        kind: 'policy-decision',
        observedAt: generatedAt,
      },
    ],
    policyRefs: ['policy-ref-social-alert-report-preference'],
    auditRefs: ['audit-ref-social-alert-report-preference'],
    parentReportRef: null,
    parentActionRef: bridgeOptions().parentAction,
    localOutboxRecordRef: 'local-outbox-record-social-alert-report-high-risk-preference',
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
      alertReportIntentId: 'social-alert-report-account-approval-preference',
      intentKind: 'account-approval-needed',
      priority: 'attention',
      severity: 'warning',
      notificationReasonCode: 'social-account-approval-needed',
      providerChannelPreference: 'email',
      parentTitleToken: 'social.alert.accountApproval.title',
      parentBodyToken: 'social.alert.accountApproval.body',
      localOutboxRecordRef: 'local-outbox-record-social-alert-report-account-approval-preference',
    },
    {
      ...base,
      alertReportIntentId: 'social-alert-report-manual-required-preference',
      intentKind: 'manual-required',
      intentStatus: 'manual-required',
      priority: 'attention',
      severity: 'warning',
      notificationReasonCode: 'social-manual-review-required',
      parentTitleToken: 'social.alert.manualRequired.title',
      parentBodyToken: 'social.alert.manualRequired.body',
      parentActionToken: 'social.alert.action.reviewManually',
      localOutboxRecordRef: null,
      manualProofRequirements: ['provider preference setup before social alert/report can be queued'],
      deliveryClaimState: 'manual-required',
    },
    {
      ...base,
      alertReportIntentId: 'social-alert-report-unavailable-preference',
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
        'local evidence and policy readiness before unavailable social alert/report can be queued',
      ],
      deliveryClaimState: 'manual-required',
    },
  ];
}

function sourceSnapshotFor(proof) {
  return [
    '# Social Alert Report Preference Preflight Source Snapshot',
    '',
    `- Branch: ${proof.branch}`,
    `- Commit: ${proof.commit}`,
    '- Scope: social alert/report scheduler rows projected into parent preference and quiet-hours preflight rows.',
    '- Package exports were intentionally not changed because another lane owns packages/parent-domain/package.json.',
    '',
  ].join('\n');
}

function markdownFor(proof) {
  return [
    '# Social Alert Report Preference Preflight Proof',
    '',
    `Generated: ${proof.generatedAt}`,
    '',
    `Source scheduler rows: ${proof.summary.sourceSchedulerRows}`,
    `Parent preference required rows: ${proof.summary.parentPreferenceRequiredCount}`,
    `Manual-required rows: ${proof.summary.manualRequiredCount}`,
    `Unavailable rows: ${proof.summary.unavailableCount}`,
    `Parent notification preference UI claimed: ${proof.summary.parentNotificationPreferenceUiClaimed}`,
    `Quiet-hours timer runtime claimed: ${proof.summary.quietHoursTimerRuntimeClaimed}`,
    `Provider delivery runtime claimed: ${proof.summary.providerDeliveryRuntimeClaimed}`,
    `Final policy execution claimed: ${proof.summary.finalPolicyExecutionClaimed}`,
    `Enforcement claimed: ${proof.summary.enforcementClaimed}`,
    '',
    'This proof consumes the social alert/report scheduler bridge. Scheduled',
    'local rows become parent-preference-required preflight rows that require',
    'parent notification preference, frequency-control, and quiet-hours policy',
    'proof before delivery can be claimed. Manual-required and unavailable rows',
    'remain blocked before preflight.',
    '',
    'It proves only the parent-domain preflight boundary. It does not claim',
    'parent notification preference UI, notification history UI, quiet-hours',
    'timer execution, provider delivery, retry worker execution, child delivery,',
    'report delivery execution, final policy execution, connector/native runtime,',
    'or enforcement.',
    '',
  ].join('\n');
}

function securityProofFor(proof) {
  return [
    '# Social Alert Report Preference Preflight Security Negative Proof',
    '',
    `Provider delivery runtime claimed: ${proof.summary.providerDeliveryRuntimeClaimed}`,
    `Provider receipt ingestion claimed: ${proof.summary.providerReceiptIngestionClaimed}`,
    `Provider credentials claimed: ${proof.summary.providerCredentialsClaimed}`,
    `Parent notification preference UI claimed: ${proof.summary.parentNotificationPreferenceUiClaimed}`,
    `Parent notification history UI claimed: ${proof.summary.parentNotificationHistoryUiClaimed}`,
    `Child delivery claimed: ${proof.summary.childDeliveryClaimed}`,
    `Quiet-hours timer runtime claimed: ${proof.summary.quietHoursTimerRuntimeClaimed}`,
    `Retry execution runtime claimed: ${proof.summary.retryExecutionRuntimeClaimed}`,
    `Report delivery execution claimed: ${proof.summary.reportDeliveryExecutionClaimed}`,
    `Final policy execution claimed: ${proof.summary.finalPolicyExecutionClaimed}`,
    `Enforcement claimed: ${proof.summary.enforcementClaimed}`,
    '',
    'The unit test rejects parent notification preference UI, quiet-hours',
    'runtime, and provider-delivery overclaims at the read-model boundary.',
    '',
  ].join('\n');
}

function assertNoUnsafeClaims(readModel) {
  const falseFields = [
    'parentNotificationPreferenceUiClaimed',
    'parentNotificationHistoryUiClaimed',
    'parentFrequencyControlUiClaimed',
    'quietHoursTimerRuntimeClaimed',
    'providerDeliveryRuntimeClaimed',
    'providerReceiptIngestionClaimed',
    'providerCredentialsClaimed',
    'cloudRoutingClaimed',
    'childDeliveryClaimed',
    'retryExecutionRuntimeClaimed',
    'productionDurableOutboxStorageClaimed',
    'adapterDispatchClaimed',
    'reportDeliveryExecutionClaimed',
    'finalPolicyExecutionClaimed',
    'enforcementClaimed',
  ];
  for (const field of falseFields) {
    if (readModel[field] !== false) {
      throw new Error(`Unsafe social alert/report preference preflight claim was true: ${field}`);
    }
  }
  if (
    readModel.parentPreferenceRequiredCount !== 2 ||
    readModel.manualRequiredCount !== 1 ||
    readModel.unavailableCount !== 1
  ) {
    throw new Error(`Unexpected preference preflight counts: ${JSON.stringify(readModel)}`);
  }
}

function validationLogFor(proof) {
  return proof.commands.map((command) => `${command.exitCode === 0 ? 'PASS' : 'FAIL'} ${command.command}`).join('\n');
}

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function checkFile(path) {
  return { label: `${path} exists`, pass: existsSync(join(repoRoot, path)) };
}

function checkIncludes(text, expected, label) {
  return { label, pass: text.includes(expected) };
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
  return result.exitCode === 0 ? result.stdout.trim() : 'unknown';
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
