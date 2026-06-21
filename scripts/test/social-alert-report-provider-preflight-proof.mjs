import { existsSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';
import { spawnSync } from 'node:child_process';

const root = process.cwd();
const outputDirectory = join(root, 'output', 'browser-plan-proof', 'social-alert-report-provider-preflight-proof');
const resultDirectory = join(root, 'test-results', 'social-alert-report-provider-preflight-proof');
const generatedAt = '2026-06-07T05:30:00Z';
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
    'social-alert-report-preference-preflight.test.ts',
  ]);

  const source = await readText('packages/schema-domain/src/social-alert-report-provider-preflight-proof.ts');
  const test = await readText('packages/browser-domain/tests/unit/social-alert-report-preference-preflight.test.ts');
  const socialFeature = await readText('docs/features/social-video-control.md');
  const workpackReadme = await readText('docs/plans/browser-plan/social-platform-account-feed/readme.md');
  const proofModule = await importDist('social-alert-report-provider-preflight-proof.js');
  const intentModule = await importDist('social-alert-report-intent.js');
  const refs = await importDist('reference-primitives.js');

  const readModel = proofModule.buildSocialAlertReportProviderPreflightReadModel(
    options(),
    proofIntents(intentModule, refs)
  );
  const summary = proofModule.summarizeSocialAlertReportProviderPreflight(readModel);
  const checks = [
    checkFile('packages/schema-domain/src/social-alert-report-provider-preflight-proof.ts'),
    checkFile('packages/browser-domain/tests/unit/social-alert-report-preference-preflight.test.ts'),
    checkFile('scripts/test/social-alert-report-provider-preflight-proof.mjs'),
    checkIncludes(source, 'providerDeliveryRuntimeClaimed: Schema.Literal(false)', 'provider delivery non-claim guard'),
    checkIncludes(source, 'finalPolicyExecutionClaimed: Schema.Literal(false)', 'final policy non-claim guard'),
    checkIncludes(source, 'enforcementClaimed: Schema.Literal(false)', 'enforcement non-claim guard'),
    checkIncludes(test, 'providerDeliveryRuntimeClaimed: true', 'provider delivery rejection test'),
    checkIncludes(test, 'parentNotificationPreferenceUiClaimed: true', 'preference UI overclaim rejection test'),
    checkIncludes(test, 'quietHoursTimerRuntimeClaimed: true', 'quiet-hours runtime overclaim rejection test'),
    checkIncludes(socialFeature, 'social-alert-report-intent-ui-proof', 'social feature alert/report UI proof note'),
    checkIncludes(workpackReadme, 'social-alert-report-intent-ui-proof', 'social README alert/report UI proof note'),
  ];
  const failures = checks.filter((check) => !check.pass).map((check) => check.label);
  const proof = {
    schemaVersion: 1,
    proofMode: 'social-alert-report-provider-preflight-proof',
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
      enforcementClaimed: readModel.enforcementClaimed,
    },
    rows: readModel.rows.map((row) => ({
      preflightRowId: row.preflightRowId,
      sourceIntentRef: row.sourceIntentRef,
      status: row.status,
      sourceLocalOutboxRecordRef: row.sourceLocalOutboxRecordRef,
      adapterRequirementRefs: row.adapterRequirementRefs,
      manualProofRequirements: row.manualProofRequirements,
    })),
    proofPaths: {
      source: 'packages/schema-domain/src/social-alert-report-provider-preflight-proof.ts',
      test: 'packages/browser-domain/tests/unit/social-alert-report-preference-preflight.test.ts',
      harness: 'scripts/test/social-alert-report-provider-preflight-proof.mjs',
      evidence: 'test-results/social-alert-report-provider-preflight-proof/proof.json',
      manifest:
        'output/browser-plan-proof/social-alert-report-provider-preflight-proof/01-social-alert-report-provider-preflight-proof.md',
    },
    failures,
  };

  if (failures.length > 0) {
    throw new Error(`Social alert/report provider preflight proof failed:\n${failures.join('\n')}`);
  }

  const proofPath = join(resultDirectory, 'proof.json');
  const readModelPath = join(resultDirectory, 'provider-preflight-read-model.json');
  const markdownPath = join(outputDirectory, '01-social-alert-report-provider-preflight-proof.md');
  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(readModelPath, `${JSON.stringify(readModel, null, 2)}\n`);
  await writeFile(markdownPath, `${markdownFor(proof)}\n`);

  console.log('social-alert-report-provider-preflight-proof-ok=true');
  console.log(`proof=${relativePath(proofPath)}`);
  console.log(`manifest=${relativePath(markdownPath)}`);
}

function importDist(name) {
  return import(pathToFileURL(join(root, 'packages', 'schema-domain', 'dist', name)).href);
}

function options() {
  return {
    generatedAt,
    providerPreflightId: 'social-alert-report-provider-preflight-proof',
    sourceContractRefs: ['social-alert-report-intent', 'social-alert-report-intent-ui-proof'],
  };
}

function proofIntents(intent, refs) {
  const base = {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    alertReportIntentId: 'social-provider-preflight-high-risk',
    intentKind: intent.SocialAlertReportIntentKind.HighRiskSignal,
    intentStatus: intent.SocialAlertReportIntentStatus.LocalOutboxEligible,
    priority: 'urgent',
    severity: 'critical',
    device: {
      deviceId: 'device-social-provider-preflight',
      childProfileId: 'child-social-provider-preflight',
      label: 'Study Phone',
      platform: refs.ParentPlatform.Android,
    },
    notificationReasonCode: intent.SocialAlertReportReasonCode.HighRiskSignal,
    providerChannelPreference: 'in-app',
    parentTitleToken: intent.SocialAlertReportParentCopyToken.HighRiskTitle,
    parentBodyToken: intent.SocialAlertReportParentCopyToken.HighRiskBody,
    parentActionToken: intent.SocialAlertReportParentCopyToken.OpenParentReviewAction,
    dashboardPanelRefs: ['panel-feed-video-gates'],
    explanationSnapshotRef: 'social-explanation-snapshot-provider-preflight',
    explanationEventRefs: ['social-explanation-event-provider-preflight'],
    evidenceReferences: [
      {
        evidenceReferenceId: 'evidence-social-provider-preflight',
        kind: refs.ParentEvidenceReferenceKind.PolicyDecision,
        observedAt: generatedAt,
      },
    ],
    policyRefs: ['policy-ref-social-provider-preflight'],
    auditRefs: ['audit-ref-social-provider-preflight'],
    parentReportRef: null,
    parentActionRef: null,
    localOutboxRecordRef: 'local-outbox-social-provider-preflight',
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
    alertReportIntentId: 'social-provider-preflight-manual-required',
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
    manualProofRequirements: ['manual-proof-social-provider-preflight-required'],
  };
}

function unavailableIntent(base, intent) {
  return {
    ...manualIntent(base, intent),
    alertReportIntentId: 'social-provider-preflight-unavailable',
    intentKind: intent.SocialAlertReportIntentKind.CapabilityUnavailable,
    intentStatus: intent.SocialAlertReportIntentStatus.Unavailable,
    notificationReasonCode: intent.SocialAlertReportReasonCode.CapabilityUnavailable,
    parentTitleToken: intent.SocialAlertReportParentCopyToken.UnavailableTitle,
    parentBodyToken: intent.SocialAlertReportParentCopyToken.UnavailableBody,
    manualProofRequirements: ['manual-proof-social-provider-capability-unavailable'],
  };
}

function markdownFor(proof) {
  return [
    '# Social Alert Report Provider Preflight Proof',
    '',
    `Generated: ${proof.generatedAt}`,
    '',
    `Rows: ${proof.summary.totalRows}`,
    `Provider adapter required rows: ${proof.summary.providerAdapterRequiredCount}`,
    `Manual-required rows: ${proof.summary.manualRequiredCount}`,
    `Unavailable rows: ${proof.summary.unavailableCount}`,
    `Provider delivery runtime claimed: ${proof.summary.providerDeliveryRuntimeClaimed}`,
    `Final policy execution claimed: ${proof.summary.finalPolicyExecutionClaimed}`,
    `Enforcement claimed: ${proof.summary.enforcementClaimed}`,
    '',
    'This proof consumes parsed social alert/report intents and turns local-outbox',
    'rows into provider-adapter-required preflight rows. It requires provider',
    'adapter, credential, and smoke proof refs before delivery can be claimed.',
    'Manual-required and unavailable source rows remain blocked.',
    '',
    'It does not claim provider delivery execution, receipt ingestion, provider',
    'credentials, cloud routing, parent notification UI delivery, report delivery',
    'execution, final policy execution, or enforcement.',
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
