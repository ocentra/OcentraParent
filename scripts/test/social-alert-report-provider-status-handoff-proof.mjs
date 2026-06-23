import { existsSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';
import { spawnSync } from 'node:child_process';

const root = process.cwd();
const outputDirectory = join(root, 'output', 'browser-plan-proof', 'social-alert-report-provider-status-handoff-proof');
const resultDirectory = join(root, 'test-results', 'social-alert-report-provider-status-handoff-proof');
const generatedAt = '2026-06-07T05:46:00Z';
const commands = [];

await main();

async function main() {
  await mkdir(outputDirectory, { recursive: true });
  await mkdir(resultDirectory, { recursive: true });

  runNpm(['run', 'build', '--workspace', '@ocentra-parent/schema-domain']);
  runNpm([
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/browser-domain',
    '--',
    'social-alert-report-preference-status-handoff.test.ts',
  ]);

  const source = await readText('packages/schema-domain/src/social-alert-report-provider-status-handoff-proof.ts');
  const test = await readText(
    'packages/browser-domain/tests/unit/social-alert-report-preference-status-handoff.test.ts'
  );
  const socialFeature = await readText('docs/features/social-video-control.md');
  const workpackReadme = await readText('docs/plans/browser-plan/social-platform-account-feed/readme.md');
  const preflightModule = await importDist('social-alert-report-provider-preflight-proof.js');
  const handoffModule = await importDist('social-alert-report-provider-status-handoff-proof.js');
  const intentModule = await importDist('social-alert-report-intent.js');
  const refs = await importDist('reference-primitives.js');

  const preflightReadModel = preflightModule.buildSocialAlertReportProviderPreflightReadModel(
    {
      generatedAt,
      providerPreflightId: 'social-alert-report-provider-preflight-for-status-handoff',
      sourceContractRefs: ['social-alert-report-intent', 'social-alert-report-intent-ui-proof'],
    },
    proofIntents(intentModule, refs)
  );
  const readModel = handoffModule.buildSocialAlertReportProviderStatusHandoffReadModel(
    {
      generatedAt,
      handoffId: 'social-alert-report-provider-status-handoff-proof',
      sourceContractRefs: [
        'social-alert-report-provider-preflight-proof',
        'v0-8-notification-provider-status-boundary',
        'notifications-expectation-provider-boundary',
      ],
    },
    preflightReadModel
  );
  const summary = handoffModule.summarizeSocialAlertReportProviderStatusHandoff(readModel);
  const checks = [
    checkFile('packages/schema-domain/src/social-alert-report-provider-status-handoff-proof.ts'),
    checkFile('packages/browser-domain/tests/unit/social-alert-report-preference-status-handoff.test.ts'),
    checkFile('scripts/test/social-alert-report-provider-status-handoff-proof.mjs'),
    checkIncludes(source, 'providerDeliveryRuntimeClaimed: Schema.Literal(false)', 'provider delivery non-claim guard'),
    checkIncludes(source, 'providerReceiptIngestionClaimed: Schema.Literal(false)', 'provider receipt non-claim guard'),
    checkIncludes(source, 'finalPolicyExecutionClaimed: Schema.Literal(false)', 'final policy non-claim guard'),
    checkIncludes(source, 'enforcementClaimed: Schema.Literal(false)', 'enforcement non-claim guard'),
    checkIncludes(test, 'providerDeliveryRuntimeClaimed: true', 'provider delivery rejection test'),
    checkIncludes(test, 'parentNotificationPreferenceUiClaimed: true', 'preference UI overclaim rejection test'),
    checkIncludes(test, 'providerReceiptRefs.length === 0', 'receipt ref boundary test'),
    checkIncludes(
      socialFeature,
      'social-alert-report-provider-preflight-proof',
      'social feature provider preflight note'
    ),
    checkIncludes(
      workpackReadme,
      'social-alert-report-provider-preflight-proof',
      'social README provider preflight note'
    ),
  ];
  const failures = checks.filter((check) => !check.pass).map((check) => check.label);
  const proof = {
    schemaVersion: 1,
    proofMode: 'social-alert-report-provider-status-handoff-proof',
    generatedAt,
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    commit: gitOutput(['rev-parse', 'HEAD']),
    commands,
    checks,
    summary,
    nonClaims: {
      providerDeliveryRuntimeClaimed: readModel.providerDeliveryRuntimeClaimed,
      providerReceiptIngestionClaimed: readModel.providerReceiptIngestionClaimed,
      providerCredentialsClaimed: readModel.providerCredentialsClaimed,
      cloudRoutingClaimed: readModel.cloudRoutingClaimed,
      parentNotificationUiDeliveryClaimed: readModel.parentNotificationUiDeliveryClaimed,
      reportDeliveryExecutionClaimed: readModel.reportDeliveryExecutionClaimed,
      finalPolicyExecutionClaimed: readModel.finalPolicyExecutionClaimed,
      connectorNativeRuntimeClaimed: readModel.connectorNativeRuntimeClaimed,
      enforcementClaimed: readModel.enforcementClaimed,
    },
    rows: readModel.rows.map((row) => ({
      handoffRowId: row.handoffRowId,
      sourcePreflightRowId: row.sourcePreflightRowId,
      sourceIntentRef: row.sourceIntentRef,
      sourcePreflightStatus: row.sourcePreflightStatus,
      providerStatus: row.providerStatusBoundaryEntry.providerStatus,
      statusProofState: row.providerStatusBoundaryEntry.statusProofState,
      readinessRefs: row.providerStatusBoundaryEntry.readinessRefs,
      providerReceiptRefs: row.providerStatusBoundaryEntry.providerReceiptRefs,
    })),
    proofPaths: {
      source: 'packages/schema-domain/src/social-alert-report-provider-status-handoff-proof.ts',
      test: 'packages/browser-domain/tests/unit/social-alert-report-preference-status-handoff.test.ts',
      harness: 'scripts/test/social-alert-report-provider-status-handoff-proof.mjs',
      evidence: 'test-results/social-alert-report-provider-status-handoff-proof/proof.json',
      readModel:
        'test-results/social-alert-report-provider-status-handoff-proof/provider-status-handoff-read-model.json',
      manifest:
        'output/browser-plan-proof/social-alert-report-provider-status-handoff-proof/01-social-alert-report-provider-status-handoff-proof.md',
    },
    failures,
  };

  if (failures.length > 0) {
    throw new Error(`Social alert/report provider status handoff proof failed:\n${failures.join('\n')}`);
  }
  assertProof(proof);

  const proofPath = join(resultDirectory, 'proof.json');
  const readModelPath = join(resultDirectory, 'provider-status-handoff-read-model.json');
  const markdownPath = join(outputDirectory, '01-social-alert-report-provider-status-handoff-proof.md');
  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(readModelPath, `${JSON.stringify(readModel, null, 2)}\n`);
  await writeFile(markdownPath, `${markdownFor(proof)}\n`);

  console.log('social-alert-report-provider-status-handoff-proof-ok=true');
  console.log(`proof=${relativePath(proofPath)}`);
  console.log(`manifest=${relativePath(markdownPath)}`);
}

function importDist(name) {
  return import(pathToFileURL(join(root, 'packages', 'schema-domain', 'dist', name)).href);
}

function proofIntents(intent, refs) {
  const base = {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    alertReportIntentId: 'social-provider-status-high-risk',
    intentKind: intent.SocialAlertReportIntentKind.HighRiskSignal,
    intentStatus: intent.SocialAlertReportIntentStatus.LocalOutboxEligible,
    priority: 'urgent',
    severity: 'critical',
    device: {
      deviceId: 'device-social-provider-status',
      childProfileId: 'child-social-provider-status',
      label: 'Study Phone',
      platform: refs.ParentPlatform.Android,
    },
    notificationReasonCode: intent.SocialAlertReportReasonCode.HighRiskSignal,
    providerChannelPreference: 'in-app',
    parentTitleToken: intent.SocialAlertReportParentCopyToken.HighRiskTitle,
    parentBodyToken: intent.SocialAlertReportParentCopyToken.HighRiskBody,
    parentActionToken: intent.SocialAlertReportParentCopyToken.OpenParentReviewAction,
    dashboardPanelRefs: ['panel-feed-video-gates'],
    explanationSnapshotRef: 'social-explanation-snapshot-provider-status',
    explanationEventRefs: ['social-explanation-event-provider-status'],
    evidenceReferences: [
      {
        evidenceReferenceId: 'evidence-social-provider-status',
        kind: refs.ParentEvidenceReferenceKind.PolicyDecision,
        observedAt: generatedAt,
      },
    ],
    policyRefs: ['policy-ref-social-provider-status'],
    auditRefs: ['audit-ref-social-provider-status'],
    parentReportRef: null,
    parentActionRef: null,
    localOutboxRecordRef: 'local-outbox-social-provider-status',
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
    alertReportIntentId: 'social-provider-status-manual-required',
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
    manualProofRequirements: ['manual-proof-social-provider-status-required'],
  };
}

function unavailableIntent(base, intent) {
  return {
    ...manualIntent(base, intent),
    alertReportIntentId: 'social-provider-status-unavailable',
    intentKind: intent.SocialAlertReportIntentKind.CapabilityUnavailable,
    intentStatus: intent.SocialAlertReportIntentStatus.Unavailable,
    notificationReasonCode: intent.SocialAlertReportReasonCode.CapabilityUnavailable,
    parentTitleToken: intent.SocialAlertReportParentCopyToken.UnavailableTitle,
    parentBodyToken: intent.SocialAlertReportParentCopyToken.UnavailableBody,
    manualProofRequirements: ['manual-proof-social-provider-status-unavailable'],
  };
}

function assertProof(proof) {
  if (
    proof.summary.rows !== 3 ||
    proof.summary.providerStatusManualRequiredCount !== 2 ||
    proof.summary.providerStatusUnavailableCount !== 1
  ) {
    throw new Error(`Unexpected provider status handoff summary: ${JSON.stringify(proof.summary)}`);
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(`Provider status handoff overclaimed runtime behavior: ${JSON.stringify(proof.nonClaims)}`);
  }
}

function markdownFor(proof) {
  return [
    '# Social Alert Report Provider Status Handoff Proof',
    '',
    `Generated: ${proof.generatedAt}`,
    '',
    `Rows: ${proof.summary.rows}`,
    `Provider status manual-required rows: ${proof.summary.providerStatusManualRequiredCount}`,
    `Provider status unavailable rows: ${proof.summary.providerStatusUnavailableCount}`,
    `Provider delivery runtime claimed: ${proof.summary.providerDeliveryRuntimeClaimed}`,
    `Provider receipt ingestion claimed: ${proof.summary.providerReceiptIngestionClaimed}`,
    `Final policy execution claimed: ${proof.summary.finalPolicyExecutionClaimed}`,
    `Enforcement claimed: ${proof.summary.enforcementClaimed}`,
    '',
    'This proof maps parsed social alert/report provider-preflight rows into the',
    'existing V0.8 notification provider status boundary. Provider-adapter',
    'required and manual-required rows remain manual-required; unavailable rows',
    'remain unavailable. Delivered status remains unclaimed until real provider',
    'delivery execution and receipt ingestion proof exists.',
    '',
    'It does not claim provider delivery execution, receipt ingestion, provider',
    'credentials, cloud routing, parent notification UI delivery, report delivery',
    'execution, final policy execution, connector/native runtime, or enforcement.',
  ].join('\n');
}

function checkFile(path) {
  return {
    label: `${path} exists`,
    pass: existsSync(join(root, path)),
  };
}

function checkIncludes(text, expected, label) {
  return {
    label,
    pass: text.includes(expected),
  };
}

function run(command, args) {
  const commandLine = [command, ...args].join(' ');
  const result = spawnSync(command, args, { cwd: root, stdio: 'inherit', shell: false });
  commands.push({ command: commandLine, exitCode: result.status ?? 1 });
  if (result.status !== 0) {
    throw new Error(`${commandLine} exited with ${result.status}`);
  }
}

function gitOutput(args) {
  const result = spawnSync('git', args, { cwd: root, encoding: 'utf8' });
  if (result.status !== 0) {
    throw new Error(`git ${args.join(' ')} failed: ${result.stderr}`);
  }
  return result.stdout.trim();
}

async function readText(path) {
  return readFile(join(root, path), 'utf8');
}

function relativePath(path) {
  return relative(root, path).replaceAll('\\', '/');
}

function runNpm(args, ...rest) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return run(command, commandArgs, ...rest);
}
