import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofSlug = '84-source-gated-policy-preview-timer-audit-rollback-read-model';
const testOutputDir = join(
  repoRoot,
  'test-results',
  'app-game-source-gated-policy-preview-timer-audit-rollback-read-model-proof'
);
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', proofSlug);
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', proofSlug);
const timestamp = '2026-06-06T05:45:00Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

for (const path of [testOutputDir, appGameProofDir, appProofDir]) {
  await rm(path, { recursive: true, force: true });
  await mkdir(path, { recursive: true });
}
for (const path of [join(appGameProofDir, '06-ui-snapshots'), join(appProofDir, '06-ui-snapshots')]) {
  await mkdir(path, { recursive: true });
}

runNpm(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
runNpm([
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'app-game-source-gated-policy-preview-timer-audit-rollback-read-model',
  'app-game-source-gated-policy-preview-timer-audit-rollback-handoff',
]);

const readModelContract = await importDist('app-game-source-gated-policy-preview-timer-audit-rollback-read-model.js');
const wp83Proof = await readJson(
  join(
    repoRoot,
    'test-results',
    'app-game-source-gated-policy-preview-timer-audit-rollback-handoff-proof',
    'proof.json'
  )
);
const readModel = readModelContract.buildAppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModel(
  readModelOptions(await importDist('reference-primitives.js')),
  wp83Proof.handoff
);
const proof = {
  proofMode: 'app-game-source-gated-policy-preview-timer-audit-rollback-read-model',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  stackedOn: {
    wp83Branch: 'codex/app-game-source-gated-policy-preview-timer-audit-rollback-handoff',
    reason:
      'WP84 consumes WP83 timer audit/rollback handoff rows and remains parent-domain only while service read APIs, portal UI, durable audit storage, rollback execution, and package exports are sequenced separately.',
  },
  summary: summarize(readModel),
  nonClaims: {
    serviceRuntimeEventClaimed: readModel.serviceRuntimeEventClaimed,
    portalUiRendered: readModel.portalUiRendered,
    policyEvaluatorRuntimeClaimed: readModel.policyEvaluatorRuntimeClaimed,
    timerRuntimeClaimed: readModel.timerRuntimeClaimed,
    timerScheduled: readModel.timerScheduled,
    schedulerPersistenceRuntimeClaimed: readModel.schedulerPersistenceRuntimeClaimed,
    durableSchedulerStorageClaimed: readModel.durableSchedulerStorageClaimed,
    auditRuntimeClaimed: readModel.auditRuntimeClaimed,
    durableAuditLogClaimed: readModel.durableAuditLogClaimed,
    rollbackRuntimeClaimed: readModel.rollbackRuntimeClaimed,
    rollbackExecutionClaimed: readModel.rollbackExecutionClaimed,
    adapterDispatchClaimed: readModel.adapterDispatchClaimed,
    childDeliveryClaimed: readModel.childDeliveryClaimed,
    platformEnforcementClaimed: readModel.platformEnforcementClaimed,
    rawPrivateSourceRowsIncluded: readModel.rawPrivateSourceRowsIncluded,
  },
  proofPaths: {
    source: 'packages/parent-domain/src/app-game-source-gated-policy-preview-timer-audit-rollback-read-model.ts',
    rules: 'packages/parent-domain/src/app-game-source-gated-policy-preview-timer-audit-rollback-read-model-rules.ts',
    test: 'packages/parent-domain/tests/app-game-source-gated-policy-preview-timer-audit-rollback-read-model.test.ts',
    harness: 'scripts/test/app-game-source-gated-policy-preview-timer-audit-rollback-read-model-proof.mjs',
    evidence: 'test-results/app-game-source-gated-policy-preview-timer-audit-rollback-read-model-proof/proof.json',
    appGameProofPack: `output/app-game-plan-proof/${proofSlug}`,
    appProofPack: `output/app-plan-proof/${proofSlug}`,
  },
  readModel,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'timer-audit-rollback-read-model.json'), readModel);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(appGameProofDir, proof, 'app-game WP84');
await writeProofPack(appProofDir, proof, 'app WP84');

console.log('app-game-source-gated-policy-preview-timer-audit-rollback-read-model-proof-ok');
console.log(
  `evidence=${join('test-results', 'app-game-source-gated-policy-preview-timer-audit-rollback-read-model-proof', 'proof.json')}`
);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function readModelOptions(refs) {
  return {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    readModelId: 'app-game-source-gated-policy-preview-timer-audit-rollback-read-model-proof',
    generatedAt: timestamp,
    sourceContractRefs: [
      'app-game-source-gated-policy-preview-timer-audit-rollback-handoff',
      'docs/expectations/app-game-evidence.md',
      'docs/expectations/policy.md',
    ],
    parentVisibleSummaryRef: 'future-parent-visible-audit-rollback-summary-proof',
  };
}

function summarize(readModel) {
  return {
    sourceAuditRollbackHandoffId: readModel.sourceAuditRollbackHandoffId,
    nativeAppRowCount: readModel.nativeAppRowCount,
    nativeGameRowCount: readModel.nativeGameRowCount,
    auditRollbackReadModelProofRequiredCount: readModel.auditRollbackReadModelProofRequiredCount,
    blockedBySourceFreshnessCount: readModel.blockedBySourceFreshnessCount,
    blockedByCompilerDecisionCount: readModel.blockedByCompilerDecisionCount,
  };
}

function assertProof(proof) {
  if (proof.summary.auditRollbackReadModelProofRequiredCount < 1) {
    throw new Error('Expected at least one audit rollback read-model proof-required row');
  }
  if (proof.summary.blockedBySourceFreshnessCount < 1 || proof.summary.blockedByCompilerDecisionCount < 1) {
    throw new Error('Expected source-freshness and compiler-decision blocked rows');
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(
      'Expected WP84 proof to avoid service, UI, evaluator, timer, scheduler storage, audit log, rollback execution, adapter, child, platform, and raw-source claims'
    );
  }
}

async function readJson(path) {
  return JSON.parse(await readFile(path, 'utf8'));
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}

async function writeProofPack(dir, proof, label) {
  await writeFile(
    join(dir, '00-source-snapshot.md'),
    [
      `# ${label} source snapshot`,
      '',
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      `- Git status: ${proof.gitStatusShort.length === 0 ? 'clean before proof generation' : proof.gitStatusShort}`,
      '',
    ].join('\n')
  );
  await writeFile(join(dir, '10-validation-commands.log'), `${proof.commands.join('\n\n').trimEnd()}\n`);
  await writeJson(join(dir, 'proof.json'), proof);
}

function run(command, args) {
  const rendered = `${command} ${args.join(' ')}`;
  const result = spawnSync(command, args, { cwd: repoRoot, encoding: 'utf8', shell: false });
  commands.push(`${rendered}\nexit=${result.status}\n${result.stdout}${result.stderr}`);
  if (result.status !== 0) {
    throw new Error(`${rendered} failed with exit ${result.status}`);
  }
}

function gitOutput(args) {
  const result = spawnSync('git', args, { cwd: repoRoot, encoding: 'utf8', shell: false });
  if (result.status !== 0) {
    throw new Error(`git ${args.join(' ')} failed: ${result.stderr}`);
  }
  return result.stdout.trim();
}

function runNpm(args, ...rest) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return run(command, commandArgs, ...rest);
}
