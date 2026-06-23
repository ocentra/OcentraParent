import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { tsImport } from 'tsx/esm/api';
import { runNpmCommand } from './run-npm-command.mjs';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofMode = 'tracking-notification-local-outbox-readiness-proof';
const testOutputDir = join(repoRoot, 'test-results', proofMode);
const wp26ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '26-alert-severity-and-notification-model');
const wp33ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const focusedProofDir = join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const sourceReceiptProofRef = 'test-results/tracking-notification-receipt-boundary-proof/proof.json';
const sourceReceiptProofPath = join(repoRoot, sourceReceiptProofRef);
const timestamp = '2026-06-07T15:03:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await main();

async function main() {
  await rm(testOutputDir, { recursive: true, force: true });
  await rm(focusedProofDir, { recursive: true, force: true });
  await mkdir(testOutputDir, { recursive: true });
  await mkdir(wp26ProofDir, { recursive: true });
  await mkdir(wp33ProofDir, { recursive: true });
  await mkdir(focusedProofDir, { recursive: true });

  run('node', ['scripts/test/tracking-notification-receipt-boundary-proof.mjs']);
  runNpmCommand(run, [
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/notification-domain',
    '--',
    'tests/unit/notification-local-outbox-adapter-proof.test.ts',
    'tests/unit/notification-local-outbox-scheduler-proof.test.ts',
  ]);
  runNpmCommand(run, [
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/tracking-domain',
    '--',
    'tests/contract/tracking-notification-local-outbox-readiness-proof.test.ts',
  ]);

  const sourceReceiptProof = JSON.parse(await readFile(sourceReceiptProofPath, 'utf8'));
  const readinessProofModule = await tsImport(
    pathToFileURL(
      join(repoRoot, 'packages', 'schema-domain', 'src', 'tracking-notification-local-outbox-readiness-proof.ts')
    ).href,
    import.meta.url
  );
  const readModel = readinessProofModule.buildTrackingNotificationLocalOutboxReadinessReadModel(
    {
      generatedAt: timestamp,
      proofId: proofMode,
      sourceContractRefs: [
        'tracking-notification-receipt-boundary-proof',
        'notification-local-outbox-adapter-proof',
        'notification-local-outbox-scheduler-proof',
        'notifications-expectations',
        'location-geofence-device-status',
        'reports-notifications-sync',
      ],
    },
    sourceReceiptProof.readModel,
    (await loadSchemaModule('notification-local-outbox-adapter-proof.ts')).NotificationLocalOutboxAdapterProofReadModel,
    (await loadSchemaModule('notification-local-outbox-scheduler-proof.ts'))
      .NotificationLocalOutboxSchedulerProofReadModel
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
    proofPaths: {
      source: 'packages/schema-domain/src/tracking-notification-local-outbox-readiness-proof.ts',
      test: 'packages/tracking-domain/tests/contract/tracking-notification-local-outbox-readiness-proof.test.ts',
      harness: 'scripts/test/tracking-notification-local-outbox-readiness-proof.mjs',
      evidence: 'test-results/tracking-notification-local-outbox-readiness-proof/proof.json',
      focusedProofRoot: 'output/tracking-plan-proof/tracking-notification-local-outbox-readiness-proof',
      wp26Proof:
        'output/tracking-plan-proof/26-alert-severity-and-notification-model/28-notification-local-outbox-readiness-proof.json',
      wp33Proof:
        'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/42-notification-local-outbox-readiness-proof.json',
    },
    sourceReceiptBoundaryProof: sourceReceiptProofRef,
    readModel,
  };

  assertProof(proof);
  await writeJson(join(testOutputDir, 'tracking-notification-local-outbox-readiness-read-model.json'), readModel);
  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeFocusedProofPack(focusedProofDir, proof);
  await writeJson(join(wp26ProofDir, '28-notification-local-outbox-readiness-proof.json'), proof);
  await writeJson(join(wp33ProofDir, '42-notification-local-outbox-readiness-proof.json'), proof);

  console.log('tracking-notification-local-outbox-readiness-proof-ok');
  console.log(`evidence=${join('test-results', proofMode, 'proof.json')}`);
}

async function loadSchemaModule(fileName) {
  return tsImport(pathToFileURL(join(repoRoot, 'packages', 'schema-domain', 'src', fileName)).href, import.meta.url);
}

function summarize(readModel) {
  return {
    rows: readModel.rows.length,
    receiptRequiredCount: readModel.receiptRequiredCount,
    manualRequiredCount: readModel.manualRequiredCount,
    providerUnavailableCount: readModel.providerUnavailableCount,
    readinessStates: countBy(readModel.rows.map((row) => row.readinessState)),
    localOutboxStates: countBy(readModel.rows.map((row) => row.localOutboxStateRef)),
    schedulerStates: countBy(readModel.rows.map((row) => row.schedulerStateRef)),
    sourceLocalOutboxAdapterProofRef: readModel.sourceLocalOutboxAdapterProofRef,
    sourceLocalOutboxSchedulerProofRef: readModel.sourceLocalOutboxSchedulerProofRef,
  };
}

function nonClaims(readModel) {
  return {
    providerDeliveryRuntimeClaimed: readModel.providerDeliveryRuntimeClaimed,
    providerReceiptIngestionRuntimeClaimed: readModel.providerReceiptIngestionRuntimeClaimed,
    providerCredentialsClaimed: readModel.providerCredentialsClaimed,
    cloudRoutingClaimed: readModel.cloudRoutingClaimed,
    parentNotificationUiClaimed: readModel.parentNotificationUiClaimed,
    retryExecutionRuntimeClaimed: readModel.retryExecutionRuntimeClaimed,
    quietHoursTimerRuntimeClaimed: readModel.quietHoursTimerRuntimeClaimed,
    productionDurableOutboxStorageClaimed: readModel.productionDurableOutboxStorageClaimed,
    childDeviceDeliveryClaimed: readModel.childDeviceDeliveryClaimed,
    mobilePhysicalDeviceProofClaimed: readModel.mobilePhysicalDeviceProofClaimed,
    authorityProofClaimed: readModel.authorityProofClaimed,
  };
}

function assertProof(proof) {
  if (
    proof.summary.rows !== 3 ||
    proof.summary.receiptRequiredCount !== 1 ||
    proof.summary.manualRequiredCount !== 1 ||
    proof.summary.providerUnavailableCount !== 1 ||
    proof.summary.sourceLocalOutboxAdapterProofRef !== 'notification-local-outbox-adapter-proof' ||
    proof.summary.sourceLocalOutboxSchedulerProofRef !== 'notification-local-outbox-scheduler-proof'
  ) {
    throw new Error(`Unexpected tracking notification local outbox summary: ${JSON.stringify(proof.summary)}`);
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(
      `Tracking notification local outbox proof overclaimed behavior: ${JSON.stringify(proof.nonClaims)}`
    );
  }
}

async function writeFocusedProofPack(path, proof) {
  await writeFile(
    join(path, '00-source-snapshot.md'),
    [
      '# Tracking Notification Local Outbox Readiness Source Snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
      '- Scope: tracking notification receipt rows mapped to existing local outbox and scheduler proof artifacts.',
      '- Source inspected: tracking notification receipt boundary proof, notification local outbox adapter proof, notification local outbox scheduler proof, notification expectations, location/geofence feature doc, and WP26/WP33 tracking workpacks.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '01-contract-proof.log'),
    [
      'Contract proof:',
      '',
      '- node scripts/test/tracking-notification-receipt-boundary-proof.mjs: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/notification-domain -- tests/unit/notification-local-outbox-adapter-proof.test.ts tests/unit/notification-local-outbox-scheduler-proof.test.ts: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/tracking-domain -- tests/contract/tracking-notification-local-outbox-readiness-proof.test.ts: PASS',
      '- Tracking receipt-required rows map to the existing local receipt-required outbox and scheduler records.',
      '- Tracking manual-required rows map to the existing local manual-required outbox and scheduler records.',
      '- Tracking provider-unavailable rows map to the existing local dead-letter/manual-review scheduler path.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '13-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- Provider delivery, provider receipt ingestion runtime, credentials, cloud routing, parent notification UI, retry worker runtime, quiet-hours timer runtime, production durable outbox storage, child-device delivery, physical-device proof, and authority proof remain false.',
      '- The proof references notification-domain local outbox and scheduler artifact refs only; it does not store raw child evidence, provider secrets, private paths, or delivered receipt payloads.',
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
