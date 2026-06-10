import { spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const root = process.cwd();
const proofName = 'social-alert-report-provider-receipt-ingestion-readiness-proof';
const outputDirectory = join(root, 'output', 'browser-plan-proof', proofName);
const resultDirectory = join(root, 'test-results', proofName);
const generatedAt = '2026-06-08T05:55:00Z';
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
    'social-alert-report-provider-receipt-ingestion-readiness.test.ts',
  ]);

  const source = await readText(
    'packages/parent-domain/src/social-alert-report-provider-receipt-ingestion-readiness.ts'
  );
  const test = await readText(
    'packages/parent-domain/tests/social-alert-report-provider-receipt-ingestion-readiness.test.ts'
  );
  const workpackReadme = await readText('docs/plans/browser-plan/social-platform-account-feed/README.md');
  const checklist = await readText('docs/plans/browser-plan/implementation-checklist.md');
  const featureDoc = await readText('docs/features/browser-web-control.md');
  const preflightModule = await importDist('social-alert-report-provider-preflight-proof.js');
  const statusHandoffModule = await importDist('social-alert-report-provider-status-handoff-proof.js');
  const receiptBoundaryModule = await importDist('social-alert-report-provider-receipt-boundary-proof.js');
  const ingestionModule = await importDist('social-alert-report-provider-receipt-ingestion-readiness.js');
  const intentModule = await importDist('social-alert-report-intent.js');
  const refs = await importDist('reference-primitives.js');

  const preflightReadModel = preflightModule.buildSocialAlertReportProviderPreflightReadModel(
    {
      generatedAt,
      providerPreflightId: 'social-alert-report-provider-preflight-for-receipt-ingestion',
      sourceContractRefs: ['social-alert-report-intent', 'social-alert-report-provider-status-handoff-proof'],
    },
    proofIntents(intentModule, refs)
  );
  const statusHandoffReadModel = statusHandoffModule.buildSocialAlertReportProviderStatusHandoffReadModel(
    {
      generatedAt,
      handoffId: 'social-alert-report-provider-status-handoff-for-receipt-ingestion',
      sourceContractRefs: [
        'social-alert-report-provider-preflight-proof',
        'v0-8-notification-provider-status-boundary',
        'notifications-expectation-provider-boundary',
      ],
    },
    preflightReadModel
  );
  const receiptBoundaryReadModel = receiptBoundaryModule.buildSocialAlertReportProviderReceiptBoundaryReadModel(
    {
      generatedAt,
      receiptBoundaryId: 'social-alert-report-provider-receipt-boundary-for-ingestion',
      sourceContractRefs: [
        'social-alert-report-provider-status-handoff-proof',
        'v0-8-notification-provider-status-boundary',
        'notifications-expectation-provider-boundary',
      ],
    },
    statusHandoffReadModel
  );
  const readModel = ingestionModule.buildSocialAlertReportProviderReceiptIngestionReadinessReadModel(
    {
      generatedAt,
      readinessId: proofName,
      sourceContractRefs: [
        'social-alert-report-provider-receipt-boundary-proof',
        'provider-receipt-webhook-contract',
        'provider-receipt-durable-store-contract',
      ],
    },
    receiptBoundaryReadModel
  );
  const summary = ingestionModule.summarizeSocialAlertReportProviderReceiptIngestionReadiness(readModel);
  const checks = [
    checkFile('packages/parent-domain/src/social-alert-report-provider-receipt-ingestion-readiness.ts'),
    checkFile('packages/parent-domain/tests/social-alert-report-provider-receipt-ingestion-readiness.test.ts'),
    checkFile('scripts/test/social-alert-report-provider-receipt-ingestion-readiness-proof.mjs'),
    checkIncludes(
      source,
      'providerReceiptIngestionRuntimeClaimed: Schema.Literal(false)',
      'ingestion runtime non-claim'
    ),
    checkIncludes(source, 'providerWebhookRuntimeClaimed: Schema.Literal(false)', 'webhook runtime non-claim'),
    checkIncludes(source, 'providerCredentialsClaimed: Schema.Literal(false)', 'credentials non-claim'),
    checkIncludes(source, 'providerReceiptObservedRefs.length === 0', 'provider receipt observed ref gate'),
    checkIncludes(test, "webhookEndpointRef: 'provider-webhook-endpoint-observed'", 'webhook overclaim rejection'),
    checkIncludes(
      workpackReadme,
      'social-alert-report-provider-receipt-ingestion-readiness-proof',
      'social README receipt ingestion reference'
    ),
    checkIncludes(
      checklist,
      'social-alert-report-provider-receipt-ingestion-readiness-proof',
      'browser checklist receipt ingestion reference'
    ),
    checkIncludes(featureDoc, 'receipt ingestion readiness', 'feature doc receipt ingestion reference'),
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
      providerReceiptObservedClaimed: readModel.providerReceiptObservedClaimed,
      cloudRoutingClaimed: readModel.cloudRoutingClaimed,
      parentNotificationUiDeliveryClaimed: readModel.parentNotificationUiDeliveryClaimed,
      reportDeliveryExecutionClaimed: readModel.reportDeliveryExecutionClaimed,
      finalPolicyExecutionClaimed: readModel.finalPolicyExecutionClaimed,
      connectorNativeRuntimeClaimed: readModel.connectorNativeRuntimeClaimed,
      enforcementClaimed: readModel.enforcementClaimed,
    },
    rows: readModel.rows.map((row) => ({
      ingestionRowId: row.ingestionRowId,
      sourceReceiptRowRef: row.sourceReceiptRowRef,
      sourceIntentRef: row.sourceIntentRef,
      sourceProviderAttemptRef: row.sourceProviderAttemptRef,
      sourceReceiptBoundaryState: row.sourceReceiptBoundaryState,
      ingestionReadinessState: row.ingestionReadinessState,
      webhookEndpointRef: row.webhookEndpointRef,
      providerCredentialRef: row.providerCredentialRef,
      durableReceiptResultRef: row.durableReceiptResultRef,
      providerReceiptObservedRefs: row.providerReceiptObservedRefs,
      ingestionProofRequirements: row.ingestionProofRequirements,
    })),
    proofPaths: {
      source: 'packages/parent-domain/src/social-alert-report-provider-receipt-ingestion-readiness.ts',
      test: 'packages/parent-domain/tests/social-alert-report-provider-receipt-ingestion-readiness.test.ts',
      harness: 'scripts/test/social-alert-report-provider-receipt-ingestion-readiness-proof.mjs',
      evidence: 'test-results/social-alert-report-provider-receipt-ingestion-readiness-proof/proof.json',
      readModel:
        'test-results/social-alert-report-provider-receipt-ingestion-readiness-proof/provider-receipt-ingestion-readiness-read-model.json',
      manifest:
        'output/browser-plan-proof/social-alert-report-provider-receipt-ingestion-readiness-proof/01-social-alert-report-provider-receipt-ingestion-readiness-proof.md',
    },
    failures,
  };

  if (failures.length > 0) {
    throw new Error(`Social alert/report provider receipt ingestion readiness proof failed:\n${failures.join('\n')}`);
  }
  assertProof(proof);

  const proofPath = join(resultDirectory, 'proof.json');
  const readModelPath = join(resultDirectory, 'provider-receipt-ingestion-readiness-read-model.json');
  const markdownPath = join(outputDirectory, '01-social-alert-report-provider-receipt-ingestion-readiness-proof.md');
  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(readModelPath, `${JSON.stringify(readModel, null, 2)}\n`);
  await writeFile(markdownPath, `${markdownFor(proof)}\n`);

  console.log('social-alert-report-provider-receipt-ingestion-readiness-proof-ok=true');
  console.log(`proof=${relativePath(proofPath)}`);
  console.log(`manifest=${relativePath(markdownPath)}`);
}

function importDist(name) {
  return import(pathToFileURL(join(root, 'packages', 'parent-domain', 'dist', name)).href);
}

function proofIntents(intent, refs) {
  const base = {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    alertReportIntentId: 'social-provider-ingestion-high-risk',
    intentKind: intent.SocialAlertReportIntentKind.HighRiskSignal,
    intentStatus: intent.SocialAlertReportIntentStatus.LocalOutboxEligible,
    priority: 'urgent',
    severity: 'critical',
    device: {
      deviceId: 'device-social-provider-ingestion',
      childProfileId: 'child-social-provider-ingestion',
      label: 'Study Phone',
      platform: refs.ParentPlatform.Android,
    },
    notificationReasonCode: intent.SocialAlertReportReasonCode.HighRiskSignal,
    providerChannelPreference: 'in-app',
    parentTitleToken: intent.SocialAlertReportParentCopyToken.HighRiskTitle,
    parentBodyToken: intent.SocialAlertReportParentCopyToken.HighRiskBody,
    parentActionToken: intent.SocialAlertReportParentCopyToken.OpenParentReviewAction,
    dashboardPanelRefs: ['panel-feed-video-gates'],
    explanationSnapshotRef: 'social-explanation-snapshot-provider-ingestion',
    explanationEventRefs: ['social-explanation-event-provider-ingestion'],
    evidenceReferences: [
      {
        evidenceReferenceId: 'evidence-social-provider-ingestion',
        kind: refs.ParentEvidenceReferenceKind.PolicyDecision,
        observedAt: generatedAt,
      },
    ],
    policyRefs: ['policy-ref-social-provider-ingestion'],
    auditRefs: ['audit-ref-social-provider-ingestion'],
    parentReportRef: null,
    parentActionRef: null,
    localOutboxRecordRef: 'local-outbox-social-provider-ingestion',
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
    alertReportIntentId: 'social-provider-ingestion-manual-required',
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
    manualProofRequirements: ['manual-proof-social-provider-ingestion-required'],
  };
}

function unavailableIntent(base, intent) {
  return {
    ...manualIntent(base, intent),
    alertReportIntentId: 'social-provider-ingestion-unavailable',
    intentKind: intent.SocialAlertReportIntentKind.CapabilityUnavailable,
    intentStatus: intent.SocialAlertReportIntentStatus.Unavailable,
    notificationReasonCode: intent.SocialAlertReportReasonCode.CapabilityUnavailable,
    parentTitleToken: intent.SocialAlertReportParentCopyToken.UnavailableTitle,
    parentBodyToken: intent.SocialAlertReportParentCopyToken.UnavailableBody,
    manualProofRequirements: ['manual-proof-social-provider-ingestion-unavailable'],
  };
}

function assertProof(proof) {
  if (proof.summary.rows !== 3 || proof.summary.ingestionContractRequiredCount !== 1) {
    throw new Error('Expected exactly one provider receipt ingestion contract-required row');
  }
  if (proof.summary.manualReceiptRequiredCount !== 1 || proof.summary.providerUnavailableCount !== 1) {
    throw new Error('Expected one manual receipt row and one provider-unavailable row');
  }
  if (proof.summary.providerReceiptObservedCount !== 0) {
    throw new Error('Provider receipt ingestion readiness proof must not observe provider receipts');
  }
  if (Object.values(proof.nonClaims).some((claim) => claim !== false)) {
    throw new Error('Receipt ingestion readiness proof attempted to claim provider runtime, policy, or enforcement');
  }
  if (proof.rows.some((row) => row.webhookEndpointRef !== null || row.providerCredentialRef !== null)) {
    throw new Error('Webhook and credential refs must remain manual-required in readiness proof');
  }
}

function markdownFor(proof) {
  const rows = proof.rows
    .map(
      (row) =>
        `| ${row.sourceIntentRef} | ${row.sourceReceiptBoundaryState} | ${row.ingestionReadinessState} | ${row.providerReceiptObservedRefs.length} |`
    )
    .join('\n');

  return [
    '# Social Alert Report Provider Receipt Ingestion Readiness Proof',
    '',
    `- Generated at: ${proof.generatedAt}`,
    `- Branch: ${proof.branch}`,
    `- Rows: ${proof.summary.rows}`,
    `- Ingestion contract required: ${proof.summary.ingestionContractRequiredCount}`,
    `- Manual receipt required: ${proof.summary.manualReceiptRequiredCount}`,
    `- Provider unavailable: ${proof.summary.providerUnavailableCount}`,
    `- Provider receipts observed: ${proof.summary.providerReceiptObservedCount}`,
    '',
    '## Rows',
    '',
    '| Source Intent | Source Receipt Boundary | Ingestion Readiness | Provider Receipt Refs |',
    '| --- | --- | --- | --- |',
    rows,
    '',
    '## No-Claim Boundary',
    '',
    '- Provider delivery runtime: false',
    '- Provider receipt ingestion runtime: false',
    '- Provider webhook runtime: false',
    '- Provider credentials: false',
    '- Provider receipt observed: false',
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
