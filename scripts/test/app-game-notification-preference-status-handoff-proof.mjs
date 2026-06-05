import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const testOutputDir = join(repoRoot, 'test-results', 'app-game-notification-preference-status-handoff-proof');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', '65-notification-preference-status-handoff');
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', '65-notification-preference-status-handoff');
const timestamp = '2026-06-05T08:39:00Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

for (const path of [testOutputDir, appGameProofDir, appProofDir]) {
  await rm(path, { recursive: true, force: true });
  await mkdir(path, { recursive: true });
}

run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
run('cmd', [
  '/c',
  'npm',
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'app-game-notification-preference-status-handoff',
  'app-game-notification-preference-preflight',
  'v3-notification-rule-provider-retry-contract',
]);

const preferencePreflight = await importDist('app-game-notification-preference-preflight.js');
const preferenceStatusHandoff = await importDist('app-game-notification-preference-status-handoff.js');
const refs = await importDist('reference-primitives.js');

const sourcePreflight = preferencePreflight.AppGameNotificationPreferencePreflightReadModelSchema.parse(
  sourcePreflightReadModel(preferencePreflight, refs)
);
const readModel = preferenceStatusHandoff.buildAppGameNotificationPreferenceStatusHandoffReadModel(
  {
    generatedAt: timestamp,
    handoffId: 'app-game-notification-preference-status-handoff-proof',
    sourceContractRefs: [
      'app-game-notification-preference-preflight',
      'v3-notification-rule-provider-retry-contract',
      'notifications-expectation-preference-boundary',
    ],
  },
  sourcePreflight
);
const proof = {
  proofMode: 'app-game-notification-preference-status-handoff',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: summarize(readModel),
  nonClaims: {
    parentPreferenceUiClaimed: readModel.parentPreferenceUiClaimed,
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
  },
  proofPaths: {
    source: 'packages/parent-domain/src/app-game-notification-preference-status-handoff.ts',
    test: 'packages/parent-domain/tests/app-game-notification-preference-status-handoff.test.ts',
    harness: 'scripts/test/app-game-notification-preference-status-handoff-proof.mjs',
    evidence: 'test-results/app-game-notification-preference-status-handoff-proof/proof.json',
    appGameProofPack: 'output/app-game-plan-proof/65-notification-preference-status-handoff',
    appProofPack: 'output/app-plan-proof/65-notification-preference-status-handoff',
  },
  readModel,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'preference-status-handoff-read-model.json'), readModel);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(appGameProofDir, proof, 'app-game WP65');
await writeProofPack(appProofDir, proof, 'app WP65');

console.log('app-game-notification-preference-status-handoff-proof-ok');
console.log(`evidence=${join('test-results', 'app-game-notification-preference-status-handoff-proof', 'proof.json')}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function sourcePreflightReadModel(preferencePreflight, refs) {
  return {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    preferencePreflightId: 'app-game-notification-preference-preflight-for-status-handoff',
    generatedAt: timestamp,
    family: { familyId: 'family-app-game-preference-status-handoff' },
    sourceSchedulerBridgeId: 'scheduler-bridge-app-game-preference-status-handoff',
    sourceContractRefs: [
      'app-game-notification-scheduler-bridge',
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
      'no-parent-preference-ui',
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
    ],
    parentPreferenceUiClaimed: false,
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
  };
}

function parentPreferenceRequiredRow(preferencePreflight) {
  return {
    preferenceRowId: 'preference-preflight-app-game-time-limit',
    sourceSchedulerBridgeRecordId: 'scheduler-bridge-row-app-game-time-limit',
    status: preferencePreflight.AppGameNotificationPreferencePreflightStatus.ParentPreferenceRequired,
    sourceSchedulerEntryRef: 'scheduler-entry-app-game-time-limit',
    sourceOutboxRecordRef: 'outbox-record-app-game-time-limit',
    providerChannelRef: 'in-app',
    reasonCodeRef: 'policy-violation',
    parentPreferenceState: 'manual-setup-required',
    quietHoursDecision: 'manual-required',
    parentPreferenceRequirementRefs: [
      'parent-preference-required-in-app-scheduler-entry-app-game-time-limit',
      'notification-frequency-control-required-scheduler-entry-app-game-time-limit',
    ],
    quietHoursRequirementRefs: ['quiet-hours-policy-required-scheduler-entry-app-game-time-limit'],
    manualProofRequirements: [
      'parent-preference-required-in-app-scheduler-entry-app-game-time-limit',
      'notification-frequency-control-required-scheduler-entry-app-game-time-limit',
      'quiet-hours-policy-required-scheduler-entry-app-game-time-limit',
    ],
  };
}

function manualRequiredRow(preferencePreflight) {
  return {
    preferenceRowId: 'preference-preflight-app-game-manual-required',
    sourceSchedulerBridgeRecordId: 'scheduler-bridge-row-app-game-manual-required',
    status: preferencePreflight.AppGameNotificationPreferencePreflightStatus.ManualRequired,
    sourceSchedulerEntryRef: null,
    sourceOutboxRecordRef: null,
    providerChannelRef: null,
    reasonCodeRef: null,
    parentPreferenceState: null,
    quietHoursDecision: null,
    parentPreferenceRequirementRefs: ['provider preference setup before app game notification can be scheduled'],
    quietHoursRequirementRefs: ['provider preference setup before app game notification can be scheduled'],
    manualProofRequirements: ['provider preference setup before app game notification can be scheduled'],
  };
}

function unavailableRow(preferencePreflight) {
  return {
    preferenceRowId: 'preference-preflight-app-game-unavailable',
    sourceSchedulerBridgeRecordId: 'scheduler-bridge-row-app-game-unavailable',
    status: preferencePreflight.AppGameNotificationPreferencePreflightStatus.Unavailable,
    sourceSchedulerEntryRef: null,
    sourceOutboxRecordRef: null,
    providerChannelRef: null,
    reasonCodeRef: null,
    parentPreferenceState: null,
    quietHoursDecision: null,
    parentPreferenceRequirementRefs: [
      'local evidence and policy readiness before unavailable notification can be scheduled',
    ],
    quietHoursRequirementRefs: ['local evidence and policy readiness before unavailable notification can be scheduled'],
    manualProofRequirements: ['local evidence and policy readiness before unavailable notification can be scheduled'],
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
    throw new Error(`Unexpected preference status handoff summary: ${JSON.stringify(proof.summary)}`);
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(`Preference status handoff overclaimed runtime behavior: ${JSON.stringify(proof.nonClaims)}`);
  }
}

async function writeProofPack(proofDir, proof, label) {
  await writeFile(
    join(proofDir, '00-source-snapshot.md'),
    [
      `# ${label} Source Snapshot`,
      '',
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
      '- Scope: app/game preference-preflight rows to V3 notification preference/quiet-hours status entries.',
      '- Source inspected: app/game notification preference preflight, V3 notification rule/provider retry contract, notification expectations, app/game feature doc, reports/notifications feature doc, and implementation checklists.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '01-contract-proof.log'),
    [
      'Contract proof:',
      '',
      '- cmd /c npm run build --workspace @ocentra-parent/parent-domain: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-notification-preference-status-handoff app-game-notification-preference-preflight v3-notification-rule-provider-retry-contract: PASS',
      '- Preference-preflight rows become manual-required or not-sent V3 notification status entries.',
      '- The existing V3 retry/preference coverage is referenced, but no provider adapter delivery or parent preference UI is claimed.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '02-rust-protocol-proof.log'),
    'Rust protocol proof not applicable: this workpack adds a TypeScript parent-domain handoff boundary and does not add a Rust-crossing shape.\n',
    'utf8'
  );
  await writeJson(join(proofDir, '03-runtime-evidence.json'), proof.summary);
  await writeJson(join(proofDir, '05-policy-action-proof.json'), {
    schemaVersion: 1,
    parentPreferenceManualSetupRequiredCount: proof.summary.parentPreferenceManualSetupRequiredCount,
    quietHoursManualRequiredCount: proof.summary.quietHoursManualRequiredCount,
    preferenceStatusUnavailableCount: proof.summary.preferenceStatusUnavailableCount,
    parentPreferenceUiClaimed: false,
    providerDeliveryRuntimeClaimed: false,
    adapterDispatchClaimed: false,
  });
  await writeFile(
    join(proofDir, '08-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- App/game rows are linked to preference and quiet-hours status entries without sending provider payloads.',
      '- Parent preference setup, frequency controls, and quiet-hours policy remain manual proof requirements until real parent-visible controls exist.',
      '- Parent preference UI, parent notification UI, provider delivery, receipt ingestion, credentials, cloud routing, retry workers, quiet-hours timers, child delivery, durable outbox storage, and adapter dispatch remain false.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(join(proofDir, '10-validation-commands.log'), `${commands.join('\n')}\n`, 'utf8');
  await writeFile(
    join(proofDir, 'README.md'),
    `# ${label} Notification Preference Status Handoff Proof\n\nThis proof pack records app/game parent-preference preflight rows mapped into V3 notification preference and quiet-hours status entries without parent preference UI, provider delivery, receipt ingestion, credentials, runtime, child delivery, or adapter-dispatch claims.\n`,
    'utf8'
  );
  await writeJson(join(proofDir, 'proof.json'), proof);
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

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}
