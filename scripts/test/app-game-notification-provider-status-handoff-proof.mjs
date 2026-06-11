import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const testOutputDir = join(repoRoot, 'test-results', 'app-game-notification-provider-status-handoff-proof');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', '64-notification-provider-status-handoff');
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', '64-notification-provider-status-handoff');
const timestamp = '2026-06-05T06:44:00Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

for (const path of [testOutputDir, appGameProofDir, appProofDir]) {
  await rm(path, { recursive: true, force: true });
  await mkdir(path, { recursive: true });
}

runNpm(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
runNpm([
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'app-game-notification-provider-status-handoff',
  'app-game-notification-provider-preflight',
  'v0-8-notification-provider-status-boundary',
]);

const providerPreflight = await importDist('app-game-notification-provider-preflight.js');
const providerStatusHandoff = await importDist('app-game-notification-provider-status-handoff.js');
const refs = await importDist('reference-primitives.js');

const sourcePreflight = providerPreflight.AppGameNotificationProviderPreflightReadModelSchema.parse(
  sourcePreflightReadModel(providerPreflight, refs)
);
const readModel = providerStatusHandoff.buildAppGameNotificationProviderStatusHandoffReadModel(
  {
    generatedAt: timestamp,
    handoffId: 'app-game-notification-provider-status-handoff-proof',
    sourceContractRefs: [
      'app-game-notification-provider-preflight',
      'v0-8-notification-provider-status-boundary',
      'notifications-expectation-provider-boundary',
    ],
  },
  sourcePreflight
);
const proof = {
  proofMode: 'app-game-notification-provider-status-handoff',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: summarize(readModel),
  nonClaims: {
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
  },
  proofPaths: {
    source: 'packages/parent-domain/src/app-game-notification-provider-status-handoff.ts',
    test: 'packages/parent-domain/tests/app-game-notification-provider-status-handoff.test.ts',
    harness: 'scripts/test/app-game-notification-provider-status-handoff-proof.mjs',
    evidence: 'test-results/app-game-notification-provider-status-handoff-proof/proof.json',
    appGameProofPack: 'output/app-game-plan-proof/64-notification-provider-status-handoff',
    appProofPack: 'output/app-plan-proof/64-notification-provider-status-handoff',
  },
  readModel,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'provider-status-handoff-read-model.json'), readModel);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(appGameProofDir, proof, 'app-game WP64');
await writeProofPack(appProofDir, proof, 'app WP64');

console.log('app-game-notification-provider-status-handoff-proof-ok');
console.log(`evidence=${join('test-results', 'app-game-notification-provider-status-handoff-proof', 'proof.json')}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function sourcePreflightReadModel(providerPreflight, refs) {
  return {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    providerPreflightId: 'app-game-notification-provider-preflight-for-status-handoff',
    generatedAt: timestamp,
    family: { familyId: 'family-app-game-provider-status-handoff' },
    sourceSchedulerBridgeId: 'scheduler-bridge-app-game-provider-status-handoff',
    sourceContractRefs: [
      'app-game-notification-scheduler-bridge',
      'notification-local-outbox-scheduler-proof',
      'notification-provider-adapter-boundary-required',
    ],
    rows: [
      providerAdapterRequiredRow(providerPreflight),
      manualRequiredRow(providerPreflight),
      unavailableRow(providerPreflight),
    ],
    providerAdapterRequiredCount: 1,
    manualRequiredCount: 1,
    unavailableCount: 1,
    preflightNonClaims: [
      'no-provider-delivery-execution',
      'no-provider-receipt-ingestion',
      'no-provider-credentials',
      'no-cloud-routing',
      'no-parent-notification-ui',
      'no-child-delivery',
      'no-retry-worker-runtime',
      'no-quiet-hours-timer-runtime',
      'no-production-durable-outbox-storage',
      'no-adapter-dispatch',
    ],
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    parentNotificationUiClaimed: false,
    childDeliveryClaimed: false,
    retryExecutionRuntimeClaimed: false,
    quietHoursTimerRuntimeClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
  };
}

function providerAdapterRequiredRow(providerPreflight) {
  return {
    preflightRowId: 'provider-preflight-app-game-time-limit',
    sourceSchedulerBridgeRecordId: 'scheduler-bridge-row-app-game-time-limit',
    status: providerPreflight.AppGameNotificationProviderPreflightStatus.ProviderAdapterRequired,
    sourceSchedulerEntryRef: 'scheduler-entry-app-game-time-limit',
    sourceOutboxRecordRef: 'outbox-record-app-game-time-limit',
    schedulerDecisionRef: 'scheduler-decision-app-game-time-limit',
    providerChannelRef: 'in-app',
    reasonCodeRef: 'policy-violation',
    adapterRequirementRefs: [
      'provider-adapter-required-scheduler-entry-app-game-time-limit',
      'provider-credentials-required-scheduler-entry-app-game-time-limit',
      'provider-smoke-proof-required-scheduler-entry-app-game-time-limit',
    ],
    manualProofRequirements: [
      'provider-adapter-required-scheduler-entry-app-game-time-limit',
      'provider-credentials-required-scheduler-entry-app-game-time-limit',
      'provider-smoke-proof-required-scheduler-entry-app-game-time-limit',
    ],
  };
}

function manualRequiredRow(providerPreflight) {
  return {
    preflightRowId: 'provider-preflight-app-game-manual-required',
    sourceSchedulerBridgeRecordId: 'scheduler-bridge-row-app-game-manual-required',
    status: providerPreflight.AppGameNotificationProviderPreflightStatus.ManualRequired,
    sourceSchedulerEntryRef: null,
    sourceOutboxRecordRef: null,
    schedulerDecisionRef: null,
    providerChannelRef: null,
    reasonCodeRef: null,
    adapterRequirementRefs: ['provider preference setup before app game notification can be scheduled'],
    manualProofRequirements: ['provider preference setup before app game notification can be scheduled'],
  };
}

function unavailableRow(providerPreflight) {
  return {
    preflightRowId: 'provider-preflight-app-game-unavailable',
    sourceSchedulerBridgeRecordId: 'scheduler-bridge-row-app-game-unavailable',
    status: providerPreflight.AppGameNotificationProviderPreflightStatus.Unavailable,
    sourceSchedulerEntryRef: null,
    sourceOutboxRecordRef: null,
    schedulerDecisionRef: null,
    providerChannelRef: null,
    reasonCodeRef: null,
    adapterRequirementRefs: ['local evidence and policy readiness before unavailable notification can be scheduled'],
    manualProofRequirements: ['local evidence and policy readiness before unavailable notification can be scheduled'],
  };
}

function summarize(readModel) {
  return {
    rows: readModel.rows.length,
    providerStatusManualRequiredCount: readModel.providerStatusManualRequiredCount,
    providerStatusUnavailableCount: readModel.providerStatusUnavailableCount,
    providerStatuses: countBy(readModel.rows.map((row) => row.providerStatusBoundaryEntry.providerStatus)),
    boundaryCoverageRefs: readModel.providerStatusBoundaryCoverageRefs.length,
  };
}

function assertProof(proof) {
  if (
    proof.summary.rows !== 3 ||
    proof.summary.providerStatusManualRequiredCount !== 2 ||
    proof.summary.providerStatusUnavailableCount !== 1 ||
    proof.summary.boundaryCoverageRefs !== 5
  ) {
    throw new Error(`Unexpected provider status handoff summary: ${JSON.stringify(proof.summary)}`);
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(`Provider status handoff overclaimed runtime behavior: ${JSON.stringify(proof.nonClaims)}`);
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
      '- Scope: app/game provider-preflight rows to V0.8 provider-status boundary rows.',
      '- Source inspected: app/game notification provider preflight, V0.8 notification provider status boundary, notification expectations, app/game feature doc, reports/notifications feature doc, and implementation checklists.',
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
      '- cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-notification-provider-status-handoff app-game-notification-provider-preflight v0-8-notification-provider-status-boundary: PASS',
      '- Provider-preflight rows become manual-required or unavailable provider-status boundary rows.',
      '- The existing V0.8 boundary coverage is referenced, but no provider adapter delivery is claimed.',
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
    providerStatusManualRequiredCount: proof.summary.providerStatusManualRequiredCount,
    providerStatusUnavailableCount: proof.summary.providerStatusUnavailableCount,
    providerDeliveryRuntimeClaimed: false,
    adapterDispatchClaimed: false,
  });
  await writeFile(
    join(proofDir, '08-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- App/game rows are linked to provider-status boundary states without sending provider payloads.',
      '- Delivered status remains covered only by the existing V0.8 boundary and still requires real provider receipts before any delivery claim.',
      '- Provider delivery, receipt ingestion, credentials, cloud routing, retry workers, quiet-hours timers, parent UI, child delivery, durable outbox storage, and adapter dispatch remain false.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(join(proofDir, '10-validation-commands.log'), `${commands.join('\n')}\n`, 'utf8');
  await writeFile(
    join(proofDir, 'README.md'),
    `# ${label} Provider Status Handoff Proof\n\nThis proof pack records app/game provider-preflight rows mapped into V0.8 provider-status boundary states without provider delivery, receipt ingestion, credentials, UI, child delivery, production runtime, or adapter-dispatch claims.\n`,
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

function runNpm(args, ...rest) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return run(command, commandArgs, ...rest);
}
