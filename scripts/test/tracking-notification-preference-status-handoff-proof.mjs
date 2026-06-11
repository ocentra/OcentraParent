import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { runNpmCommand } from './run-npm-command.mjs';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofMode = 'tracking-notification-preference-status-handoff-proof';
const testOutputDir = join(repoRoot, 'test-results', proofMode);
const wp26Dir = join(repoRoot, 'output', 'tracking-plan-proof', '26-alert-severity-and-notification-model');
const wp33Dir = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const proofDir = join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const sourcePreflightProofPath = join(
  repoRoot,
  'test-results',
  'tracking-notification-preference-preflight-proof',
  'proof.json'
);
const timestamp = '2026-06-07T21:20:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(testOutputDir, { recursive: true, force: true });
await rm(proofDir, { recursive: true, force: true });
await mkdir(testOutputDir, { recursive: true });
await mkdir(wp26Dir, { recursive: true });
await mkdir(wp33Dir, { recursive: true });
await mkdir(proofDir, { recursive: true });

run('node', ['scripts/test/tracking-notification-preference-preflight-proof.mjs']);
runNpmCommand(run, ['run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
run('cmd', [
  '/c',
  'npm',
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'tracking-notification-preference-status-handoff',
  'tracking-notification-preference-preflight-proof',
  'v3-notification-rule-provider-retry-contract',
]);

const sourcePreflightProof = JSON.parse(await readFile(sourcePreflightProofPath, 'utf8'));
const preferenceStatusHandoff = await importDist('tracking-notification-preference-status-handoff.js');
const readModel = preferenceStatusHandoff.buildTrackingNotificationPreferenceStatusHandoffReadModel(
  {
    generatedAt: timestamp,
    handoffId: 'tracking-notification-preference-status-handoff-proof',
    sourceContractRefs: [
      'tracking-notification-preference-preflight-proof',
      'v3-notification-rule-provider-retry-contract',
      'notification-parent-preference-boundary',
      'notification-quiet-hours-policy-boundary',
    ],
  },
  sourcePreflightProof.readModel
);
const proof = {
  proofMode,
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: summarize(readModel),
  nonClaims: nonClaims(readModel),
  sourcePreferencePreflightProof: 'test-results/tracking-notification-preference-preflight-proof/proof.json',
  proofPaths: proofPaths(),
  readModel,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'tracking-notification-preference-status-handoff-read-model.json'), readModel);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(proofDir, proof);
await writeJson(join(wp26Dir, '31-notification-preference-status-handoff-proof.json'), proof);
await writeJson(join(wp33Dir, '54-notification-preference-status-handoff-proof.json'), proof);

console.log('tracking-notification-preference-status-handoff-proof-ok');
console.log(`evidence=${join('test-results', proofMode, 'proof.json')}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
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

function nonClaims(readModel) {
  return {
    parentNotificationPreferenceUiClaimed: readModel.parentNotificationPreferenceUiClaimed,
    parentNotificationHistoryUiClaimed: readModel.parentNotificationHistoryUiClaimed,
    parentFrequencyControlUiClaimed: readModel.parentFrequencyControlUiClaimed,
    parentNotificationUiClaimed: readModel.parentNotificationUiClaimed,
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
    source: 'packages/parent-domain/src/tracking-notification-preference-status-handoff.ts',
    test: 'packages/parent-domain/tests/tracking-notification-preference-status-handoff.test.ts',
    harness: 'scripts/test/tracking-notification-preference-status-handoff-proof.mjs',
    evidence: 'test-results/tracking-notification-preference-status-handoff-proof/proof.json',
    wp26: 'output/tracking-plan-proof/26-alert-severity-and-notification-model/31-notification-preference-status-handoff-proof.json',
    wp33: 'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/54-notification-preference-status-handoff-proof.json',
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
    throw new Error(`Unexpected tracking preference status handoff summary: ${JSON.stringify(proof.summary)}`);
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(
      `Tracking preference status handoff overclaimed runtime behavior: ${JSON.stringify(proof.nonClaims)}`
    );
  }
}

async function writeProofPack(targetDir, proof) {
  await writeFile(
    join(targetDir, '00-source-snapshot.md'),
    [
      '# Tracking Notification Preference Status Handoff Source Snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
      '- Scope: tracking preference-preflight rows to V3 notification preference and quiet-hours status entries.',
      '- No parent notification preference UI, parent history UI, quiet-hours timer, provider delivery, receipt ingestion, credentials, child delivery, production outbox, or adapter dispatch is claimed.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(targetDir, '16-validation-commands.log'),
    `${commands.map((entry) => `${entry.command} exit=${entry.status}`).join('\n')}\n`,
    'utf8'
  );
  await writeFile(
    join(targetDir, '13-security-negative-proof.log'),
    [
      'workpack=26-alert-severity-and-notification-model',
      'workpack=33-proof-gates-fixtures-rollout-and-pr-gate',
      'Preference status handoff rows map parent preference and quiet-hours requirements into V3 notification status entries.',
      'Rows do not send provider payloads, observe receipts, claim provider credentials, mutate parent preferences, run quiet-hours timers, or claim production durable outbox storage.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeJson(join(targetDir, 'proof.json'), proof);
}

function run(command, args) {
  const printable = [command, ...args].join(' ');
  const result = spawnSync(command, args, { cwd: repoRoot, encoding: 'utf8', shell: false });
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
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}
