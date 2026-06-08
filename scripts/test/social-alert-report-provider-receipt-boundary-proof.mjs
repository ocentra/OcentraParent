import { spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const root = process.cwd();
const proofName = 'social-alert-report-provider-receipt-boundary-proof';
const outputDirectory = join(root, 'output', 'browser-plan-proof', proofName);
const resultDirectory = join(root, 'test-results', proofName);
const generatedAt = '2026-06-08T04:13:00Z';
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
    'social-alert-report-provider-receipt-boundary-proof.test.ts',
  ]);

  const source = await readText('packages/parent-domain/src/social-alert-report-provider-receipt-boundary-proof.ts');
  const test = await readText(
    'packages/parent-domain/tests/social-alert-report-provider-receipt-boundary-proof.test.ts'
  );
  const workpackReadme = await readText('docs/plans/browser-plan/social-platform-account-feed/README.md');
  const checklist = await readText('docs/plans/browser-plan/implementation-checklist.md');
  const preflightModule = await importDist('social-alert-report-provider-preflight-proof.js');
  const statusHandoffModule = await importDist('social-alert-report-provider-status-handoff-proof.js');
  const receiptModule = await importDist('social-alert-report-provider-receipt-boundary-proof.js');
  const intentModule = await importDist('social-alert-report-intent.js');
  const refs = await importDist('reference-primitives.js');

  const preflightReadModel = preflightModule.buildSocialAlertReportProviderPreflightReadModel(
    {
      generatedAt,
      providerPreflightId: 'social-alert-report-provider-preflight-for-receipt-boundary',
      sourceContractRefs: ['social-alert-report-intent', 'social-alert-report-provider-status-handoff-proof'],
    },
    proofIntents(intentModule, refs)
  );
  const statusHandoffReadModel = statusHandoffModule.buildSocialAlertReportProviderStatusHandoffReadModel(
    {
      generatedAt,
      handoffId: 'social-alert-report-provider-status-handoff-for-receipt-boundary',
      sourceContractRefs: [
        'social-alert-report-provider-preflight-proof',
        'v0-8-notification-provider-status-boundary',
        'notifications-expectation-provider-boundary',
      ],
    },
    preflightReadModel
  );
  const readModel = receiptModule.buildSocialAlertReportProviderReceiptBoundaryReadModel(
    {
      generatedAt,
      receiptBoundaryId: proofName,
      sourceContractRefs: [
        'social-alert-report-provider-status-handoff-proof',
        'v0-8-notification-provider-status-boundary',
        'notifications-expectation-provider-boundary',
      ],
    },
    statusHandoffReadModel
  );
  const summary = receiptModule.summarizeSocialAlertReportProviderReceiptBoundary(readModel);
  const checks = [
    checkFile('packages/parent-domain/src/social-alert-report-provider-receipt-boundary-proof.ts'),
    checkFile('packages/parent-domain/tests/social-alert-report-provider-receipt-boundary-proof.test.ts'),
    checkFile('scripts/test/social-alert-report-provider-receipt-boundary-proof.mjs'),
    checkIncludes(source, 'providerReceiptIngestionRuntimeClaimed: Schema.Literal(false)', 'receipt runtime non-claim'),
    checkIncludes(source, 'providerWebhookRuntimeClaimed: Schema.Literal(false)', 'webhook runtime non-claim'),
    checkIncludes(source, 'finalPolicyExecutionClaimed: Schema.Literal(false)', 'final policy non-claim'),
    checkIncludes(source, 'enforcementClaimed: Schema.Literal(false)', 'enforcement non-claim'),
    checkIncludes(test, "providerReceiptRefs: ['provider-receipt-observed']", 'receipt overclaim rejection test'),
    checkIncludes(
      workpackReadme,
      'social-alert-report-provider-receipt-boundary-proof',
      'social README receipt boundary reference'
    ),
    checkIncludes(
      checklist,
      'social-alert-report-provider-receipt-boundary-proof',
      'browser checklist receipt boundary reference'
    ),
  ];
  const failures = checks.filter((check) => !check.pass).map((check) => check.label);
  const proof = {
    schemaVersion: 1,
    proofMode: proofName,
    generatedAt,
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    commands,
    checks,
    summary,
    nonClaims: {
      providerDeliveryRuntimeClaimed: readModel.providerDeliveryRuntimeClaimed,
      providerReceiptIngestionRuntimeClaimed: readModel.providerReceiptIngestionRuntimeClaimed,
      providerWebhookRuntimeClaimed: readModel.providerWebhookRuntimeClaimed,
      providerCredentialsClaimed: readModel.providerCredentialsClaimed,
      cloudRoutingClaimed: readModel.cloudRoutingClaimed,
      parentNotificationUiDeliveryClaimed: readModel.parentNotificationUiDeliveryClaimed,
      reportDeliveryExecutionClaimed: readModel.reportDeliveryExecutionClaimed,
      finalPolicyExecutionClaimed: readModel.finalPolicyExecutionClaimed,
      connectorNativeRuntimeClaimed: readModel.connectorNativeRuntimeClaimed,
      enforcementClaimed: readModel.enforcementClaimed,
    },
    rows: readModel.rows.map((row) => ({
      receiptRowId: row.receiptRowId,
      sourceProviderStatusHandoffRowRef: row.sourceProviderStatusHandoffRowRef,
      sourcePreflightRowRef: row.sourcePreflightRowRef,
      sourceIntentRef: row.sourceIntentRef,
      sourceLocalOutboxRecordRef: row.sourceLocalOutboxRecordRef,
      sourceProviderStatus: row.sourceProviderStatus,
      receiptBoundaryState: row.receiptBoundaryState,
      providerAttemptRef: row.providerAttemptRef,
      providerReceiptRefs: row.providerReceiptRefs,
      receiptProofRequirements: row.receiptProofRequirements,
    })),
    proofPaths: {
      source: 'packages/parent-domain/src/social-alert-report-provider-receipt-boundary-proof.ts',
      test: 'packages/parent-domain/tests/social-alert-report-provider-receipt-boundary-proof.test.ts',
      harness: 'scripts/test/social-alert-report-provider-receipt-boundary-proof.mjs',
      evidence: 'test-results/social-alert-report-provider-receipt-boundary-proof/proof.json',
      readModel:
        'test-results/social-alert-report-provider-receipt-boundary-proof/provider-receipt-boundary-read-model.json',
      manifest:
        'output/browser-plan-proof/social-alert-report-provider-receipt-boundary-proof/01-social-alert-report-provider-receipt-boundary-proof.md',
    },
    failures,
  };

  if (failures.length > 0) {
    throw new Error(`Social alert/report provider receipt boundary proof failed:\n${failures.join('\n')}`);
  }
  assertProof(proof);

  const proofPath = join(resultDirectory, 'proof.json');
  const readModelPath = join(resultDirectory, 'provider-receipt-boundary-read-model.json');
  const markdownPath = join(outputDirectory, '01-social-alert-report-provider-receipt-boundary-proof.md');
  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(readModelPath, `${JSON.stringify(readModel, null, 2)}\n`);
  await writeFile(markdownPath, `${markdownFor(proof)}\n`);

  console.log('social-alert-report-provider-receipt-boundary-proof-ok=true');
  console.log(`proof=${relativePath(proofPath)}`);
  console.log(`manifest=${relativePath(markdownPath)}`);
}

function importDist(name) {
  return import(pathToFileURL(join(root, 'packages', 'parent-domain', 'dist', name)).href);
}

function proofIntents(intent, refs) {
  const base = {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    alertReportIntentId: 'social-provider-receipt-high-risk',
    intentKind: intent.SocialAlertReportIntentKind.HighRiskSignal,
    intentStatus: intent.SocialAlertReportIntentStatus.LocalOutboxEligible,
    priority: 'urgent',
    severity: 'critical',
    device: {
      deviceId: 'device-social-provider-receipt',
      childProfileId: 'child-social-provider-receipt',
      label: 'Study Phone',
      platform: refs.ParentPlatform.Android,
    },
    notificationReasonCode: intent.SocialAlertReportReasonCode.HighRiskSignal,
    providerChannelPreference: 'in-app',
    parentTitleToken: intent.SocialAlertReportParentCopyToken.HighRiskTitle,
    parentBodyToken: intent.SocialAlertReportParentCopyToken.HighRiskBody,
    parentActionToken: intent.SocialAlertReportParentCopyToken.OpenParentReviewAction,
    dashboardPanelRefs: ['panel-feed-video-gates'],
    explanationSnapshotRef: 'social-explanation-snapshot-provider-receipt',
    explanationEventRefs: ['social-explanation-event-provider-receipt'],
    evidenceReferences: [
      {
        evidenceReferenceId: 'evidence-social-provider-receipt',
        kind: refs.ParentEvidenceReferenceKind.PolicyDecision,
        observedAt: generatedAt,
      },
    ],
    policyRefs: ['policy-ref-social-provider-receipt'],
    auditRefs: ['audit-ref-social-provider-receipt'],
    parentReportRef: null,
    parentActionRef: null,
    localOutboxRecordRef: 'local-outbox-social-provider-receipt',
    providerAttemptRefs: [],
    providerReceiptRefs: [],
    manualProofRequirements: [],
    minimalPayloadFields: Object.values(intent.SocialAlertReportPayloadField),
    deliveryClaimState: intent.SocialAlertReportDeliveryClaimState.LocalOutboxOnly,
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
    adapterDispatchState: intent.SocialAlertReportAdapterDispatchState.NotDispatched,
    adapterActionClaimed: false,
    createdAt: generatedAt,
  };
  return [base, manualIntent(base, intent), unavailableIntent(base, intent)];
}

function manualIntent(base, intent) {
  return {
    ...base,
    alertReportIntentId: 'social-provider-receipt-manual-required',
    intentKind: intent.SocialAlertReportIntentKind.ManualRequired,
    intentStatus: intent.SocialAlertReportIntentStatus.ManualRequired,
    priority: 'attention',
    severity: 'warning',
    notificationReasonCode: intent.SocialAlertReportReasonCode.ManualRequired,
    parentTitleToken: intent.SocialAlertReportParentCopyToken.ManualRequiredTitle,
    parentBodyToken: intent.SocialAlertReportParentCopyToken.ManualRequiredBody,
    parentActionToken: intent.SocialAlertReportParentCopyToken.ReviewManuallyAction,
    localOutboxRecordRef: null,
    deliveryClaimState: intent.SocialAlertReportDeliveryClaimState.ManualRequired,
    manualProofRequirements: ['manual-proof-social-provider-receipt-required'],
  };
}

function unavailableIntent(base, intent) {
  return {
    ...manualIntent(base, intent),
    alertReportIntentId: 'social-provider-receipt-unavailable',
    intentKind: intent.SocialAlertReportIntentKind.CapabilityUnavailable,
    intentStatus: intent.SocialAlertReportIntentStatus.Unavailable,
    notificationReasonCode: intent.SocialAlertReportReasonCode.CapabilityUnavailable,
    parentTitleToken: intent.SocialAlertReportParentCopyToken.UnavailableTitle,
    parentBodyToken: intent.SocialAlertReportParentCopyToken.UnavailableBody,
    manualProofRequirements: ['manual-proof-social-provider-receipt-unavailable'],
  };
}

function assertProof(proof) {
  if (proof.summary.rows !== 3 || proof.summary.providerDispatchRequiredCount !== 1) {
    throw new Error('Expected exactly one provider-dispatch-required social receipt boundary row');
  }
  if (proof.summary.manualReceiptRequiredCount !== 1 || proof.summary.providerUnavailableCount !== 1) {
    throw new Error('Expected one manual receipt row and one provider-unavailable row');
  }
  if (Object.values(proof.nonClaims).some((claim) => claim !== false)) {
    throw new Error(
      'Social receipt proof attempted to claim provider delivery, receipt runtime, policy, or enforcement'
    );
  }
  if (proof.rows.some((row) => row.providerReceiptRefs.length > 0)) {
    throw new Error('Social receipt proof must not ingest provider receipt refs before provider runtime proof exists');
  }
}

function markdownFor(proof) {
  const rows = proof.rows
    .map(
      (row) =>
        `| ${row.sourceIntentRef} | ${row.sourceProviderStatus} | ${row.receiptBoundaryState} | ${row.providerReceiptRefs.length} |`
    )
    .join('\n');

  return [
    '# Social Alert Report Provider Receipt Boundary Proof',
    '',
    `- Generated at: ${proof.generatedAt}`,
    `- Branch: ${proof.branch}`,
    `- Rows: ${proof.summary.rows}`,
    `- Provider dispatch required: ${proof.summary.providerDispatchRequiredCount}`,
    `- Manual receipt required: ${proof.summary.manualReceiptRequiredCount}`,
    `- Provider unavailable: ${proof.summary.providerUnavailableCount}`,
    '',
    '## Rows',
    '',
    '| Source Intent | Provider Status | Receipt Boundary | Provider Receipt Refs |',
    '| --- | --- | --- | --- |',
    rows,
    '',
    '## No-Claim Boundary',
    '',
    '- Provider delivery runtime: false',
    '- Provider receipt ingestion runtime: false',
    '- Provider webhook runtime: false',
    '- Parent notification UI delivery: false',
    '- Report delivery execution: false',
    '- Final policy execution: false',
    '- Connector/native runtime: false',
    '- Enforcement: false',
    '- Package subpath export: deferred because `packages/parent-domain/package.json` is currently owned by another lane.',
  ].join('\n');
}

function run(command, args) {
  const printable = [command, ...args].join(' ');
  const result = spawnSync(command, args, { cwd: root, encoding: 'utf8' });
  commands.push({
    command: printable,
    status: result.status,
    stdout: result.stdout.trim(),
    stderr: result.stderr.trim(),
  });
  if (result.status !== 0) {
    throw new Error(`${printable} failed\n${result.stdout}\n${result.stderr}`);
  }
}

function gitOutput(args) {
  const result = spawnSync('git', args, { cwd: root, encoding: 'utf8' });
  if (result.status !== 0) {
    throw new Error(`git ${args.join(' ')} failed\n${result.stderr}`);
  }
  return result.stdout.trim();
}

function checkFile(path) {
  return { label: `${path} exists`, pass: existsSync(join(root, path)) };
}

function checkIncludes(text, expected, label) {
  return { label, pass: text.includes(expected) };
}

async function readText(path) {
  return readFile(join(root, path), 'utf8');
}

function relativePath(path) {
  return relative(root, path).replaceAll('\\', '/');
}
