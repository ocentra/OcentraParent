import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { tsImport } from 'tsx/esm/api';
import { runNpmCommand } from './run-npm-command.mjs';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofMode = 'tracking-notification-preference-preflight-proof';
const testOutputDir = join(repoRoot, 'test-results', proofMode);
const wp26Dir = join(repoRoot, 'output', 'tracking-plan-proof', '26-alert-severity-and-notification-model');
const wp33Dir = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const proofDir = join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const timestamp = '2026-06-06T08:02:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await main();

async function main() {
  await rm(testOutputDir, { recursive: true, force: true });
  await rm(proofDir, { recursive: true, force: true });
  await mkdir(testOutputDir, { recursive: true });
  await mkdir(wp26Dir, { recursive: true });
  await mkdir(wp33Dir, { recursive: true });
  await mkdir(proofDir, { recursive: true });

  runNpmCommand(run, [
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/notification-domain',
    '--',
    'tests/unit/v0-8-notification-provider-status-boundary.test.ts',
    'tests/unit/v3-notification-rule-provider-retry-contract.test.ts',
  ]);
  runNpmCommand(run, [
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/tracking-domain',
    '--',
    'tests/contract/tracking-provider-notification-proof.test.ts',
    'tests/contract/tracking-notification-preference-preflight-proof.test.ts',
  ]);

  const proof = await buildProof();
  assertProof(proof);
  await writeJson(join(testOutputDir, 'tracking-notification-preference-preflight-read-model.json'), proof.readModel);
  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeProofPack(proofDir, proof);
  await writeJson(join(wp26Dir, '24-notification-preference-preflight-proof.json'), proof);
  await writeJson(join(wp33Dir, '24-notification-preference-preflight-proof.json'), proof);

  console.log('tracking-notification-preference-preflight-proof-ok');
  console.log(`evidence=${join('test-results', proofMode, 'proof.json')}`);
}

async function buildProof() {
  const trackingLocationPolicyModule = await tsImport(
    pathToFileURL(join(repoRoot, 'packages', 'tracking-domain', 'src', 'tracking-location-policy.ts')).href,
    import.meta.url
  );
  const providerModule = await tsImport(
    pathToFileURL(join(repoRoot, 'packages', 'tracking-domain', 'src', 'tracking-provider-notification-proof.ts')).href,
    import.meta.url
  );
  const preferenceModule = await tsImport(
    pathToFileURL(
      join(repoRoot, 'packages', 'tracking-domain', 'src', 'tracking-notification-preference-preflight-proof.ts')
    ).href,
    import.meta.url
  );

  const sourceReadModel = trackingLocationPolicyModule.TrackingLocationPolicyReadModelSchema.parse(
    sourceTrackingReadModel(trackingLocationPolicyModule)
  );
  const providerReadModel = providerModule.buildTrackingProviderNotificationProofReadModel(
    providerOptions(),
    sourceReadModel
  );
  const readModel = preferenceModule.buildTrackingNotificationPreferencePreflightReadModel(
    preferenceOptions(),
    providerReadModel
  );

  return {
    proofMode,
    generatedAt: timestamp,
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    commit: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: initialGitStatusShort,
    commands,
    summary: summarize(readModel),
    nonClaims: nonClaims(readModel),
    proofPaths: proofPaths(),
    sourceProviderNotificationProof: providerReadModel.proofId,
    readModel,
  };
}

function providerOptions() {
  return {
    generatedAt: timestamp,
    proofId: 'tracking-provider-notification-proof-for-preference-preflight',
    familyId: 'family-tracking-preference-preflight',
    sourceTrackingReadModelRef: 'tracking-location-policy-read-model-preference-preflight',
    sourceContractRefs: ['tracking-location-policy', 'v0-8-notification-provider-status-boundary'],
  };
}

function preferenceOptions() {
  return {
    generatedAt: timestamp,
    preferencePreflightId: proofMode,
    sourceContractRefs: [
      'tracking-provider-notification-proof',
      'v3-notification-rule-provider-retry-contract',
      'notification-parent-preference-boundary',
      'notification-quiet-hours-policy-boundary',
    ],
  };
}

function sourceTrackingReadModel(tracking) {
  return {
    schemaVersion: tracking.TrackingPolicySchemaVersion,
    generatedAt: timestamp,
    rules: [],
    decisions: [],
    acknowledgements: [],
    checkInRequests: [],
    checkInResponses: [],
    aiRoutes: [],
    aiResults: [],
    alerts: [
      alert(tracking, 'tracking-alert-home-arrival', 'info', ['tracking-notification-intent-home-arrival']),
      alert(tracking, 'tracking-alert-left-expected-place', 'urgent', ['tracking-notification-intent-left-school']),
      alert(tracking, 'tracking-alert-provider-unavailable', 'warning', []),
    ],
    escalations: [],
    temporaryLiveGrants: [],
    missingDeviceCases: [],
    platformProofRoutes: [],
  };
}

function alert(tracking, alertId, severity, notificationStatusRefs) {
  return {
    schemaVersion: tracking.TrackingPolicySchemaVersion,
    alertId,
    severity,
    sensitiveDetailMode: severity === 'urgent' ? 'authenticated-drill-in-only' : 'minimal-provider-body',
    policyDecisionId: `tracking-decision-${alertId.replace('tracking-alert-', '')}`,
    notificationStatusRefs,
    reasonCodes: [`reason-${alertId}`],
    createdAt: timestamp,
    evidenceReferences: [
      {
        evidenceReferenceId: 'location-evidence-geofence-entry',
        kind: 'journal-event',
        observedAt: '2026-06-06T08:00:00.000Z',
      },
    ],
    acknowledgementId: null,
  };
}

function summarize(readModel) {
  return {
    rows: readModel.rows.length,
    parentPreferenceRequiredCount: readModel.parentPreferenceRequiredCount,
    sourceManualRequiredCount: readModel.sourceManualRequiredCount,
    sourceUnavailableCount: readModel.sourceUnavailableCount,
    statuses: countBy(readModel.rows.map((row) => row.status)),
    providerStatusKinds: countBy(readModel.rows.map((row) => row.providerStatusKind)),
  };
}

function nonClaims(readModel) {
  return {
    parentNotificationPreferenceUiClaimed: readModel.parentNotificationPreferenceUiClaimed,
    parentNotificationHistoryUiClaimed: readModel.parentNotificationHistoryUiClaimed,
    parentFrequencyControlUiClaimed: readModel.parentFrequencyControlUiClaimed,
    quietHoursTimerRuntimeClaimed: readModel.quietHoursTimerRuntimeClaimed,
    providerDeliveryRuntimeClaimed: readModel.providerDeliveryRuntimeClaimed,
    providerReceiptIngestionRuntimeClaimed: readModel.providerReceiptIngestionRuntimeClaimed,
    providerCredentialsClaimed: readModel.providerCredentialsClaimed,
    cloudRoutingClaimed: readModel.cloudRoutingClaimed,
    childDeviceDeliveryClaimed: readModel.childDeviceDeliveryClaimed,
    mobilePhysicalDeviceProofClaimed: readModel.mobilePhysicalDeviceProofClaimed,
    retryExecutionRuntimeClaimed: readModel.retryExecutionRuntimeClaimed,
    productionDurableOutboxStorageClaimed: readModel.productionDurableOutboxStorageClaimed,
    adapterDispatchClaimed: readModel.adapterDispatchClaimed,
  };
}

function proofPaths() {
  return {
    source: 'packages/tracking-domain/src/tracking-notification-preference-preflight-proof.ts',
    test: 'packages/tracking-domain/tests/contract/tracking-notification-preference-preflight-proof.test.ts',
    harness: 'scripts/test/tracking-notification-preference-preflight-proof.mjs',
    evidence: 'test-results/tracking-notification-preference-preflight-proof/proof.json',
    wp26: 'output/tracking-plan-proof/26-alert-severity-and-notification-model/24-notification-preference-preflight-proof.json',
    wp33: 'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/24-notification-preference-preflight-proof.json',
  };
}

function assertProof(proof) {
  if (
    proof.summary.rows !== 3 ||
    proof.summary.parentPreferenceRequiredCount !== 1 ||
    proof.summary.sourceManualRequiredCount !== 1 ||
    proof.summary.sourceUnavailableCount !== 1
  ) {
    throw new Error(`Unexpected tracking preference preflight summary: ${JSON.stringify(proof.summary)}`);
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(`Tracking preference preflight overclaimed runtime behavior: ${JSON.stringify(proof.nonClaims)}`);
  }
}

async function writeProofPack(path, proof) {
  await writeFile(
    join(path, '00-source-snapshot.md'),
    [
      '# Tracking Notification Preference Preflight Source Snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      `- Source provider proof: ${proof.sourceProviderNotificationProof}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '01-contract-proof.log'),
    [
      'Contract proof:',
      '',
      '- cmd /c npm run test --workspace @ocentra-parent/notification-domain -- tests/unit/v0-8-notification-provider-status-boundary.test.ts tests/unit/v3-notification-rule-provider-retry-contract.test.ts: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/tracking-domain -- tests/contract/tracking-provider-notification-proof.test.ts tests/contract/tracking-notification-preference-preflight-proof.test.ts: PASS',
      '- Preference preflight rows preserve source evidence, policy, reason, provider attempt, and provider preference refs while keeping UI/runtime claims false.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '13-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- Parent preference UI, notification history UI, quiet-hours timers, provider delivery, receipts, credentials, cloud routing, child-device delivery, physical-device proof, retry runtime, durable outbox storage, and adapter dispatch remain unclaimed.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(join(path, '16-validation-commands.log'), `${validationLog()}\n`, 'utf8');
  await writeJson(join(path, 'proof.json'), proof);
}

function run(command, args) {
  const printable = [command, ...args].join(' ');
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: false,
  });
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

function validationLog() {
  return commands.map((entry) => `${entry.command} exit=${entry.status}`).join('\n');
}

function gitOutput(args) {
  const result = spawnSync('git', args, { cwd: repoRoot, encoding: 'utf8', shell: false });
  if (result.status !== 0) return '';
  return result.stdout.trim();
}

function countBy(values) {
  return values.reduce((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}

async function writeJson(path, value) {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}
