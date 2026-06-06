import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofSlug = '83-source-gated-policy-preview-timer-audit-rollback-handoff';
const testOutputDir = join(
  repoRoot,
  'test-results',
  'app-game-source-gated-policy-preview-timer-audit-rollback-handoff-proof'
);
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', proofSlug);
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', proofSlug);
const timestamp = '2026-06-06T05:15:00Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

for (const path of [testOutputDir, appGameProofDir, appProofDir]) {
  await rm(path, { recursive: true, force: true });
  await mkdir(path, { recursive: true });
}
for (const path of [join(appGameProofDir, '06-ui-snapshots'), join(appProofDir, '06-ui-snapshots')]) {
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
  'app-game-source-gated-policy-preview-timer-audit-rollback-handoff',
  'app-game-source-gated-policy-preview-timer-scheduler-persistence',
]);

const auditRollbackContract = await importDist('app-game-source-gated-policy-preview-timer-audit-rollback-handoff.js');
const wp82Proof = await readJson(
  join(repoRoot, 'test-results', 'app-game-source-gated-policy-preview-timer-scheduler-persistence-proof', 'proof.json')
);
const handoff = auditRollbackContract.buildAppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoff(
  auditRollbackOptions(await importDist('reference-primitives.js')),
  wp82Proof.persistence
);
const proof = {
  proofMode: 'app-game-source-gated-policy-preview-timer-audit-rollback-handoff',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  stackedOn: {
    wp82Branch: 'codex/app-game-source-gated-policy-preview-timer-scheduler-persistence',
    reason:
      'WP83 consumes WP82 timer scheduler-persistence rows and remains parent-domain only while durable audit logs, rollback execution, timer runtime, service scheduling, and package exports are sequenced separately.',
  },
  summary: summarize(handoff),
  nonClaims: {
    serviceRuntimeEventClaimed: handoff.serviceRuntimeEventClaimed,
    portalUiRendered: handoff.portalUiRendered,
    policyEvaluatorRuntimeClaimed: handoff.policyEvaluatorRuntimeClaimed,
    timerRuntimeClaimed: handoff.timerRuntimeClaimed,
    timerScheduled: handoff.timerScheduled,
    schedulerPersistenceRuntimeClaimed: handoff.schedulerPersistenceRuntimeClaimed,
    durableSchedulerStorageClaimed: handoff.durableSchedulerStorageClaimed,
    auditRuntimeClaimed: handoff.auditRuntimeClaimed,
    durableAuditLogClaimed: handoff.durableAuditLogClaimed,
    rollbackRuntimeClaimed: handoff.rollbackRuntimeClaimed,
    rollbackExecutionClaimed: handoff.rollbackExecutionClaimed,
    adapterDispatchClaimed: handoff.adapterDispatchClaimed,
    childDeliveryClaimed: handoff.childDeliveryClaimed,
    platformEnforcementClaimed: handoff.platformEnforcementClaimed,
    rawPrivateSourceRowsIncluded: handoff.rawPrivateSourceRowsIncluded,
  },
  proofPaths: {
    source: 'packages/parent-domain/src/app-game-source-gated-policy-preview-timer-audit-rollback-handoff.ts',
    rules: 'packages/parent-domain/src/app-game-source-gated-policy-preview-timer-audit-rollback-handoff-rules.ts',
    test: 'packages/parent-domain/tests/app-game-source-gated-policy-preview-timer-audit-rollback-handoff.test.ts',
    harness: 'scripts/test/app-game-source-gated-policy-preview-timer-audit-rollback-handoff-proof.mjs',
    evidence: 'test-results/app-game-source-gated-policy-preview-timer-audit-rollback-handoff-proof/proof.json',
    appGameProofPack: `output/app-game-plan-proof/${proofSlug}`,
    appProofPack: `output/app-plan-proof/${proofSlug}`,
  },
  handoff,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'timer-audit-rollback-handoff.json'), handoff);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(appGameProofDir, proof, 'app-game WP83');
await writeProofPack(appProofDir, proof, 'app WP83');

console.log('app-game-source-gated-policy-preview-timer-audit-rollback-handoff-proof-ok');
console.log(
  `evidence=${join('test-results', 'app-game-source-gated-policy-preview-timer-audit-rollback-handoff-proof', 'proof.json')}`
);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function auditRollbackOptions(refs) {
  return {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    handoffId: 'app-game-source-gated-policy-preview-timer-audit-rollback-handoff-proof',
    generatedAt: timestamp,
    sourceContractRefs: [
      'app-game-source-gated-policy-preview-timer-scheduler-persistence',
      'docs/expectations/app-game-evidence.md',
      'docs/expectations/policy.md',
    ],
    serviceTimerRuntimeProofRef: 'future-service-timer-runtime-proof',
    schedulerPersistenceProofRef: 'future-scheduler-persistence-proof',
    schedulerStateStoreProofRef: 'future-scheduler-state-store-proof',
    auditTrailProofRef: 'future-timer-audit-trail-proof',
    rollbackPlanProofRef: 'future-timer-rollback-plan-proof',
    auditRollbackReadModelProofRef: 'future-timer-audit-rollback-read-model-proof',
  };
}

function summarize(handoff) {
  return {
    sourceSchedulerPersistenceId: handoff.sourceSchedulerPersistenceId,
    nativeAppRowCount: handoff.nativeAppRowCount,
    nativeGameRowCount: handoff.nativeGameRowCount,
    auditRollbackProofRequiredCount: handoff.auditRollbackProofRequiredCount,
    blockedBySourceFreshnessCount: handoff.blockedBySourceFreshnessCount,
    blockedByCompilerDecisionCount: handoff.blockedByCompilerDecisionCount,
  };
}

function assertProof(proof) {
  if (proof.summary.auditRollbackProofRequiredCount < 1) {
    throw new Error('Expected at least one audit rollback proof-required row');
  }
  if (proof.summary.blockedBySourceFreshnessCount < 1 || proof.summary.blockedByCompilerDecisionCount < 1) {
    throw new Error('Expected source-freshness and compiler-decision blocked rows');
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(
      'Expected WP83 proof to avoid service, UI, evaluator, timer, scheduler storage, audit log, rollback execution, adapter, child, platform, and raw-source claims'
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
