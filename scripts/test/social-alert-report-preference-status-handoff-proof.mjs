import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join, relative } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const testOutputDir = join(repoRoot, 'test-results', 'social-alert-report-preference-status-handoff-proof');
const proofDir = join(repoRoot, 'output', 'browser-plan-proof', 'social-alert-report-preference-status-handoff-proof');
const timestamp = '2026-06-07T08:48:00Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(testOutputDir, { recursive: true, force: true });
await rm(proofDir, { recursive: true, force: true });
await mkdir(testOutputDir, { recursive: true });
await mkdir(proofDir, { recursive: true });

run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
run('cmd', [
  '/c',
  'npm',
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'social-alert-report-preference-status-handoff',
  'social-alert-report-preference-preflight',
  'v3-notification-rule-provider-retry-contract',
]);

const preferencePreflight = await importDist('social-alert-report-preference-preflight.js');
const preferenceStatusHandoff = await importDist('social-alert-report-preference-status-handoff.js');
const refs = await importDist('reference-primitives.js');

const sourcePreflight = preferencePreflight.SocialAlertReportPreferencePreflightReadModelSchema.parse(
  sourcePreflightReadModel(preferencePreflight, refs)
);
const readModel = preferenceStatusHandoff.buildSocialAlertReportPreferenceStatusHandoffReadModel(
  {
    generatedAt: timestamp,
    handoffId: 'social-alert-report-preference-status-handoff-proof',
    sourceContractRefs: [
      'social-alert-report-preference-preflight',
      'v3-notification-rule-provider-retry-contract',
      'notifications-expectation-preference-boundary',
      'social-video-control-expectation',
    ],
  },
  sourcePreflight
);
const proof = {
  proofMode: 'social-alert-report-preference-status-handoff',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: summarize(readModel),
  nonClaims: {
    parentNotificationPreferenceUiClaimed: readModel.parentNotificationPreferenceUiClaimed,
    parentNotificationHistoryUiClaimed: readModel.parentNotificationHistoryUiClaimed,
    parentFrequencyControlUiClaimed: readModel.parentFrequencyControlUiClaimed,
    parentNotificationUiClaimed: readModel.parentNotificationUiClaimed,
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
  proofPaths: {
    source: 'packages/parent-domain/src/social-alert-report-preference-status-handoff.ts',
    test: 'packages/parent-domain/tests/social-alert-report-preference-status-handoff.test.ts',
    harness: 'scripts/test/social-alert-report-preference-status-handoff-proof.mjs',
    evidence: 'test-results/social-alert-report-preference-status-handoff-proof/proof.json',
    readModel:
      'test-results/social-alert-report-preference-status-handoff-proof/preference-status-handoff-read-model.json',
    proofPack: 'output/browser-plan-proof/social-alert-report-preference-status-handoff-proof',
  },
  claimsProved: [
    'Social alert/report preference-preflight rows can be projected into V3 notification parent preference and quiet-hours status entries',
    'Scheduled and manual-required social rows remain manual-required until parent notification preference and quiet-hours proof exists',
    'Unavailable social rows become disabled/not-sent status entries without provider dispatch',
    'The proof reuses the existing V3 notification rule/provider/retry contract without adding a separate notification truth path',
  ],
  claimsNotProved: [
    'parent notification preference UI',
    'parent notification history UI',
    'parent frequency-control UI',
    'parent notification UI',
    'quiet-hours timer runtime',
    'provider dispatch or delivery',
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
  sourcePreflight,
  readModel,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'preference-status-handoff-read-model.json'), readModel);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(proofDir, proof);

console.log('social-alert-report-preference-status-handoff-proof-ok=true');
console.log(`proof=${relativePath(join(testOutputDir, 'proof.json'))}`);
console.log(`manifest=${relativePath(join(proofDir, '01-social-alert-report-preference-status-handoff-proof.md'))}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function sourcePreflightReadModel(preferencePreflight, refs) {
  return {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    preferencePreflightId: 'social-alert-report-preference-preflight-for-status-handoff',
    generatedAt: timestamp,
    family: { familyId: 'family-social-preference-status-handoff' },
    sourceSchedulerBridgeId: 'scheduler-bridge-social-preference-status-handoff',
    sourceContractRefs: [
      'social-alert-report-scheduler-bridge',
      'notification-parent-preference-boundary',
      'notification-quiet-hours-policy-boundary',
    ],
    rows: [
      parentPreferenceRequiredRow(preferencePreflight),
      manualRequiredRow(preferencePreflight),
      unavailableRow(preferencePreflight),
    ],
    parentPreferenceRequiredCount: 1,
    manualRequiredCount: 1,
    unavailableCount: 1,
    preflightNonClaims: [
      'no-parent-notification-preference-ui',
      'no-parent-notification-history-ui',
      'no-parent-frequency-control-ui',
      'no-quiet-hours-timer-runtime',
      'no-provider-delivery-execution',
      'no-provider-receipt-ingestion',
      'no-provider-credentials',
      'no-cloud-routing',
      'no-child-delivery',
      'no-retry-worker-runtime',
      'no-production-durable-outbox-storage',
      'no-adapter-dispatch',
      'no-report-delivery-execution',
      'no-final-policy-execution',
      'no-enforcement',
    ],
    parentNotificationPreferenceUiClaimed: false,
    parentNotificationHistoryUiClaimed: false,
    parentFrequencyControlUiClaimed: false,
    quietHoursTimerRuntimeClaimed: false,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    childDeliveryClaimed: false,
    retryExecutionRuntimeClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
    reportDeliveryExecutionClaimed: false,
    finalPolicyExecutionClaimed: false,
    enforcementClaimed: false,
  };
}

function parentPreferenceRequiredRow(preferencePreflight) {
  return {
    preferenceRowId: 'preference-preflight-social-high-risk',
    sourceSchedulerBridgeRecordId: 'scheduler-bridge-row-social-high-risk',
    status: preferencePreflight.SocialAlertReportPreferencePreflightStatus.ParentPreferenceRequired,
    sourceSchedulerEntryRef: 'scheduler-entry-social-high-risk',
    sourceOutboxRecordRef: 'outbox-record-social-high-risk',
    providerChannelRef: 'in-app',
    reasonCodeRef: 'policy-violation',
    schedulerDecisionRef: 'scheduler-decision-social-high-risk',
    parentPreferenceState: 'manual-setup-required',
    quietHoursDecision: 'manual-required',
    parentPreferenceRequirementRefs: [
      'social-parent-notification-preference-required-scheduler-entry-social-high-risk',
      'social-notification-frequency-control-required-scheduler-entry-social-high-risk',
    ],
    quietHoursRequirementRefs: ['social-quiet-hours-policy-required-scheduler-entry-social-high-risk'],
    manualProofRequirements: [
      'social-parent-notification-preference-required-scheduler-entry-social-high-risk',
      'social-notification-frequency-control-required-scheduler-entry-social-high-risk',
      'social-quiet-hours-policy-required-scheduler-entry-social-high-risk',
    ],
  };
}

function manualRequiredRow(preferencePreflight) {
  return {
    preferenceRowId: 'preference-preflight-social-manual-required',
    sourceSchedulerBridgeRecordId: 'scheduler-bridge-row-social-manual-required',
    status: preferencePreflight.SocialAlertReportPreferencePreflightStatus.ManualRequired,
    sourceSchedulerEntryRef: null,
    sourceOutboxRecordRef: null,
    providerChannelRef: null,
    reasonCodeRef: null,
    schedulerDecisionRef: null,
    parentPreferenceState: null,
    quietHoursDecision: null,
    parentPreferenceRequirementRefs: ['provider preference setup before social alert/report can be queued'],
    quietHoursRequirementRefs: ['provider preference setup before social alert/report can be queued'],
    manualProofRequirements: ['provider preference setup before social alert/report can be queued'],
  };
}

function unavailableRow(preferencePreflight) {
  return {
    preferenceRowId: 'preference-preflight-social-unavailable',
    sourceSchedulerBridgeRecordId: 'scheduler-bridge-row-social-unavailable',
    status: preferencePreflight.SocialAlertReportPreferencePreflightStatus.Unavailable,
    sourceSchedulerEntryRef: null,
    sourceOutboxRecordRef: null,
    providerChannelRef: null,
    reasonCodeRef: null,
    schedulerDecisionRef: null,
    parentPreferenceState: null,
    quietHoursDecision: null,
    parentPreferenceRequirementRefs: [
      'local evidence and policy readiness before unavailable social alert/report can be queued',
    ],
    quietHoursRequirementRefs: [
      'local evidence and policy readiness before unavailable social alert/report can be queued',
    ],
    manualProofRequirements: [
      'local evidence and policy readiness before unavailable social alert/report can be queued',
    ],
  };
}

function summarize(readModel) {
  return {
    rows: readModel.rows.length,
    parentPreferenceManualSetupRequiredCount: readModel.parentPreferenceManualSetupRequiredCount,
    quietHoursManualRequiredCount: readModel.quietHoursManualRequiredCount,
    preferenceStatusUnavailableCount: readModel.preferenceStatusUnavailableCount,
    deliveryResults: countBy(readModel.rows.map((row) => row.notificationPreferenceStatusEntry.deliveryResultState)),
    parentPreferenceStates: countBy(
      readModel.rows.map((row) => row.notificationPreferenceStatusEntry.parentPreferenceState)
    ),
    retryCoverageRefs: readModel.notificationRuleProviderRetryCoverageRefs.length,
  };
}

function assertProof(proof) {
  if (
    proof.summary.rows !== 3 ||
    proof.summary.parentPreferenceManualSetupRequiredCount !== 2 ||
    proof.summary.quietHoursManualRequiredCount !== 2 ||
    proof.summary.preferenceStatusUnavailableCount !== 1 ||
    proof.summary.retryCoverageRefs !== 6
  ) {
    throw new Error(`Unexpected social preference status handoff summary: ${JSON.stringify(proof.summary)}`);
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(
      `Social preference status handoff overclaimed runtime behavior: ${JSON.stringify(proof.nonClaims)}`
    );
  }
}

async function writeProofPack(directory, proof) {
  await writeFile(
    join(directory, '00-source-snapshot.md'),
    [
      '# Social Alert/Report Preference Status Handoff Source Snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
      '- Scope: social alert/report preference-preflight rows to V3 notification preference and quiet-hours status entries.',
      '- Source inspected: social alert/report preference preflight, V3 notification rule/provider retry contract, social video control expectations, notification expectations, and browser-plan checklist.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(directory, '01-social-alert-report-preference-status-handoff-proof.md'),
    [
      '# Social Alert/Report Preference Status Handoff Proof',
      '',
      `Generated: ${proof.generatedAt}`,
      '',
      '## What This Proves',
      '',
      ...proof.claimsProved.map((claim) => `- ${claim}`),
      '',
      '## What This Does Not Prove',
      '',
      ...proof.claimsNotProved.map((claim) => `- ${claim}`),
      '',
      '## Summary',
      '',
      `- Rows: ${proof.summary.rows}`,
      `- Parent preference manual-setup-required rows: ${proof.summary.parentPreferenceManualSetupRequiredCount}`,
      `- Quiet-hours manual-required rows: ${proof.summary.quietHoursManualRequiredCount}`,
      `- Preference status unavailable rows: ${proof.summary.preferenceStatusUnavailableCount}`,
      `- V3 notification retry/preference coverage refs: ${proof.summary.retryCoverageRefs}`,
      '',
      '## Evidence',
      '',
      `- Source: \`${proof.proofPaths.source}\``,
      `- Test: \`${proof.proofPaths.test}\``,
      `- Harness: \`${proof.proofPaths.harness}\``,
      `- Proof JSON: \`${proof.proofPaths.evidence}\``,
      `- Read model JSON: \`${proof.proofPaths.readModel}\``,
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(directory, '08-security-negative-proof.md'),
    [
      '# Security Negative Proof',
      '',
      '- Social alert/report preference-status rows preserve scheduler/outbox/preference refs without provider payload dispatch.',
      '- Parent notification preference setup, frequency control, and quiet-hours policy remain manual proof requirements.',
      '- Parent notification preference UI, notification history UI, notification UI, provider delivery, receipt ingestion, credentials, cloud routing, child delivery, retry workers, quiet-hours timers, durable outbox storage, report delivery execution, final policy execution, connector/native runtime, and enforcement remain false.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(join(directory, '10-validation-commands.log'), `${proof.commands.join('\n')}\n`, 'utf8');
  await writeFile(
    join(directory, 'ui-not-applicable.md'),
    '# UI Not Applicable\n\nThis proof adds a parent-domain social alert/report preference-status handoff boundary. It does not render parent notification preference UI, notification history UI, child UI, provider delivery UI, or enforcement UI.\n',
    'utf8'
  );
  await writeJson(join(directory, 'proof.json'), proof);
}

function run(command, args) {
  commands.push([command, ...args].join(' '));
  const result = spawnSync(command, args, { cwd: repoRoot, stdio: 'inherit', shell: false });
  if (result.status !== 0) {
    throw new Error(`Command failed: ${command} ${args.join(' ')}`);
  }
}

function gitOutput(args) {
  return spawnSync('git', args, { cwd: repoRoot, encoding: 'utf8' }).stdout.trim();
}

function countBy(values) {
  return values.reduce((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}
