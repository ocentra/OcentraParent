import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const testOutputDir = join(repoRoot, 'test-results', 'tracking-provider-notification-proof');
const proofDir = join(repoRoot, 'output', 'tracking-plan-proof', '26-alert-severity-and-notification-model');
const timestamp = '2026-06-05T10:46:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(testOutputDir, { recursive: true, force: true });
await mkdir(testOutputDir, { recursive: true });
await mkdir(proofDir, { recursive: true });

runNpm(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
runNpm([
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'tracking-provider-notification-proof',
  'tracking-location-policy',
  'v0-8-notification-provider-status-boundary',
]);

const tracking = await importDist('tracking-location-policy.js');
const proofModule = await importDist('tracking-provider-notification-proof.js');
const sourceReadModel = tracking.TrackingLocationPolicyReadModelSchema.parse(sourceTrackingReadModel(tracking));
const readModel = proofModule.buildTrackingProviderNotificationProofReadModel(
  {
    generatedAt: timestamp,
    proofId: 'tracking-provider-notification-proof',
    familyId: 'family-tracking-provider-notification',
    sourceTrackingReadModelRef: 'tracking-location-policy-read-model-provider-notification',
    sourceContractRefs: [
      'tracking-location-policy',
      'v0-8-notification-provider-status-boundary',
      'notification-local-outbox-adapter-proof',
      'location-geofence-device-status',
      'reports-notifications-sync',
    ],
  },
  sourceReadModel
);

const proof = {
  proofMode: 'tracking-provider-notification-proof',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: summarize(readModel),
  nonClaims: nonClaims(readModel),
  proofPaths: {
    source: 'packages/parent-domain/src/tracking-provider-notification-proof.ts',
    test: 'packages/parent-domain/tests/tracking-provider-notification-proof.test.ts',
    harness: 'scripts/test/tracking-provider-notification-proof.mjs',
    evidence: 'test-results/tracking-provider-notification-proof/proof.json',
    trackingProofPack: 'output/tracking-plan-proof/26-alert-severity-and-notification-model',
  },
  readModel,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'tracking-provider-notification-read-model.json'), readModel);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(proofDir, proof);

console.log('tracking-provider-notification-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-provider-notification-proof', 'proof.json')}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
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
      alert(tracking, {
        alertId: 'tracking-alert-home-arrival',
        severity: 'info',
        sensitiveDetailMode: 'minimal-provider-body',
        policyDecisionId: 'tracking-decision-home-arrival',
        notificationStatusRefs: ['tracking-notification-intent-home-arrival'],
        reasonCodes: ['home-arrival-notification'],
      }),
      alert(tracking, {
        alertId: 'tracking-alert-left-expected-place',
        severity: 'urgent',
        sensitiveDetailMode: 'authenticated-drill-in-only',
        policyDecisionId: 'tracking-decision-left-school',
        notificationStatusRefs: ['tracking-notification-intent-left-school'],
        reasonCodes: ['left-expected-place'],
      }),
      alert(tracking, {
        alertId: 'tracking-alert-provider-unavailable',
        severity: 'warning',
        sensitiveDetailMode: 'minimal-provider-body',
        policyDecisionId: 'tracking-decision-provider-unavailable',
        notificationStatusRefs: [],
        reasonCodes: ['provider-unavailable'],
      }),
    ],
    escalations: [],
    temporaryLiveGrants: [],
    missingDeviceCases: [],
    platformProofRoutes: [],
  };
}

function alert(tracking, input) {
  return {
    schemaVersion: tracking.TrackingPolicySchemaVersion,
    createdAt: timestamp,
    evidenceReferences: [
      {
        evidenceReferenceId: 'location-evidence-geofence-entry',
        kind: 'journal-event',
        observedAt: '2026-06-05T10:40:00.000Z',
      },
    ],
    acknowledgementId: null,
    ...input,
  };
}

function summarize(readModel) {
  return {
    rows: readModel.rows.length,
    providerAdapterRequiredCount: readModel.providerAdapterRequiredCount,
    manualRequiredCount: readModel.manualRequiredCount,
    unavailableCount: readModel.unavailableCount,
    providerStatuses: countBy(readModel.rows.map((row) => row.providerStatusBoundaryEntry.providerStatus)),
    sensitiveDetailModes: countBy(readModel.rows.map((row) => row.sensitiveDetailMode)),
    boundaryCoverageRefs: readModel.providerStatusBoundaryCoverageRefs.length,
  };
}

function nonClaims(readModel) {
  return {
    providerDeliveryRuntimeClaimed: readModel.providerDeliveryRuntimeClaimed,
    providerReceiptIngestionClaimed: readModel.providerReceiptIngestionClaimed,
    providerCredentialsClaimed: readModel.providerCredentialsClaimed,
    cloudRoutingClaimed: readModel.cloudRoutingClaimed,
    parentNotificationUiClaimed: readModel.parentNotificationUiClaimed,
    childDeviceDeliveryClaimed: readModel.childDeviceDeliveryClaimed,
    mobilePhysicalDeviceProofClaimed: readModel.mobilePhysicalDeviceProofClaimed,
    retryExecutionRuntimeClaimed: readModel.retryExecutionRuntimeClaimed,
    quietHoursTimerRuntimeClaimed: readModel.quietHoursTimerRuntimeClaimed,
    productionDurableOutboxStorageClaimed: readModel.productionDurableOutboxStorageClaimed,
    adapterDispatchClaimed: readModel.adapterDispatchClaimed,
  };
}

function assertProof(proof) {
  if (
    proof.summary.rows !== 3 ||
    proof.summary.providerAdapterRequiredCount !== 1 ||
    proof.summary.manualRequiredCount !== 1 ||
    proof.summary.unavailableCount !== 1 ||
    proof.summary.boundaryCoverageRefs !== 5
  ) {
    throw new Error(`Unexpected tracking provider notification summary: ${JSON.stringify(proof.summary)}`);
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(`Tracking provider notification overclaimed runtime behavior: ${JSON.stringify(proof.nonClaims)}`);
  }
}

async function writeProofPack(path, proof) {
  await writeFile(
    join(path, '00-source-snapshot.md'),
    [
      '# WP26 Tracking Provider Notification Source Snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
      '- Scope: tracking alert intents to provider-status boundary evidence.',
      '- Source inspected: tracking location policy, V0.8 notification provider-status boundary, location/geofence feature doc, notifications expectations, reports/notifications feature doc, and WP26 checklist.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '01-contract-proof.log'),
    [
      'Contract proof:',
      '',
      '- cmd /c npm run build --workspace @ocentra-parent/parent-domain: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/parent-domain -- tracking-provider-notification-proof tracking-location-policy v0-8-notification-provider-status-boundary: PASS',
      '- Tracking alert rows preserve evidence refs, policy decision refs, notification status refs, reason refs, and sensitive-detail modes.',
      '- Tracking rows map into existing V0.8 provider-status boundary rows while provider delivery remains unclaimed.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeJson(join(path, '09-policy-alert-proof.json'), proof.summary);
  await writeFile(
    join(path, '13-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- Provider delivery, receipt ingestion, credentials, cloud routing, parent notification UI, child-device delivery, physical-device proof, retry runtime, quiet-hours runtime, durable outbox storage, and adapter dispatch remain false.',
      '- Provider payloads are minimal or authenticated-drill-in only and do not include raw location evidence.',
      '- Unavailable tracking rows remain visible as provider-unavailable contract state instead of pretending delivery occurred.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(join(path, '16-validation-commands.log'), `${proof.commands.join('\n')}\n`, 'utf8');
  await writeFile(
    join(path, 'README.md'),
    '# WP26 Tracking Provider Notification Proof\n\nThis proof pack records tracking alert-intent rows mapped into provider-status boundary evidence without provider delivery, provider receipts, credentials, parent notification UI, child-device delivery, physical-device proof, or adapter dispatch claims.\n',
    'utf8'
  );
  await writeJson(join(path, 'proof.json'), proof);
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
