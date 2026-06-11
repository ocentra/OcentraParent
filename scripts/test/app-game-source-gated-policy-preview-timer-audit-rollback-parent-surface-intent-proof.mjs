import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofSlug = '85-source-gated-policy-preview-timer-audit-rollback-parent-surface-intent';
const testOutputDir = join(
  repoRoot,
  'test-results',
  'app-game-source-gated-policy-preview-timer-audit-rollback-parent-surface-intent-proof'
);
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', proofSlug);
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', proofSlug);
const timestamp = '2026-06-06T06:00:00Z';
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
  'app-game-source-gated-policy-preview-timer-audit-rollback-parent-surface-intent',
  'app-game-source-gated-policy-preview-timer-audit-rollback-read-model',
]);

const intentContract = await importDist(
  'app-game-source-gated-policy-preview-timer-audit-rollback-parent-surface-intent.js'
);
const wp84Proof = await readJson(
  join(
    repoRoot,
    'test-results',
    'app-game-source-gated-policy-preview-timer-audit-rollback-read-model-proof',
    'proof.json'
  )
);
const parentSurfaceIntent = intentContract.buildAppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntent(
  intentOptions(await importDist('reference-primitives.js')),
  wp84Proof.readModel
);
const proof = {
  proofMode: 'app-game-source-gated-policy-preview-timer-audit-rollback-parent-surface-intent',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  stackedOn: {
    wp84Branch: 'codex/app-game-source-gated-policy-preview-timer-audit-rollback-read-model',
    reason:
      'WP85 consumes WP84 timer audit/rollback read-model rows and remains parent-domain only while service read APIs, portal rendering, durable audit storage, rollback execution, and package exports are sequenced separately.',
  },
  summary: summarize(parentSurfaceIntent),
  nonClaims: {
    serviceRuntimeEventClaimed: parentSurfaceIntent.serviceRuntimeEventClaimed,
    portalUiRendered: parentSurfaceIntent.portalUiRendered,
    policyEvaluatorRuntimeClaimed: parentSurfaceIntent.policyEvaluatorRuntimeClaimed,
    timerRuntimeClaimed: parentSurfaceIntent.timerRuntimeClaimed,
    timerScheduled: parentSurfaceIntent.timerScheduled,
    schedulerPersistenceRuntimeClaimed: parentSurfaceIntent.schedulerPersistenceRuntimeClaimed,
    durableSchedulerStorageClaimed: parentSurfaceIntent.durableSchedulerStorageClaimed,
    auditRuntimeClaimed: parentSurfaceIntent.auditRuntimeClaimed,
    durableAuditLogClaimed: parentSurfaceIntent.durableAuditLogClaimed,
    rollbackRuntimeClaimed: parentSurfaceIntent.rollbackRuntimeClaimed,
    rollbackExecutionClaimed: parentSurfaceIntent.rollbackExecutionClaimed,
    adapterDispatchClaimed: parentSurfaceIntent.adapterDispatchClaimed,
    childDeliveryClaimed: parentSurfaceIntent.childDeliveryClaimed,
    platformEnforcementClaimed: parentSurfaceIntent.platformEnforcementClaimed,
    rawPrivateSourceRowsIncluded: parentSurfaceIntent.rawPrivateSourceRowsIncluded,
  },
  proofPaths: {
    source:
      'packages/parent-domain/src/app-game-source-gated-policy-preview-timer-audit-rollback-parent-surface-intent.ts',
    rules:
      'packages/parent-domain/src/app-game-source-gated-policy-preview-timer-audit-rollback-parent-surface-intent-rules.ts',
    test: 'packages/parent-domain/tests/app-game-source-gated-policy-preview-timer-audit-rollback-parent-surface-intent.test.ts',
    harness: 'scripts/test/app-game-source-gated-policy-preview-timer-audit-rollback-parent-surface-intent-proof.mjs',
    evidence:
      'test-results/app-game-source-gated-policy-preview-timer-audit-rollback-parent-surface-intent-proof/proof.json',
    appGameProofPack: `output/app-game-plan-proof/${proofSlug}`,
    appProofPack: `output/app-plan-proof/${proofSlug}`,
  },
  parentSurfaceIntent,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'timer-audit-rollback-parent-surface-intent.json'), parentSurfaceIntent);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(appGameProofDir, proof, 'app-game WP85');
await writeProofPack(appProofDir, proof, 'app WP85');

console.log('app-game-source-gated-policy-preview-timer-audit-rollback-parent-surface-intent-proof-ok');
console.log(
  `evidence=${join('test-results', 'app-game-source-gated-policy-preview-timer-audit-rollback-parent-surface-intent-proof', 'proof.json')}`
);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function intentOptions(refs) {
  return {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    intentId: 'app-game-source-gated-policy-preview-timer-audit-rollback-parent-surface-intent-proof',
    generatedAt: timestamp,
    sourceContractRefs: [
      'app-game-source-gated-policy-preview-timer-audit-rollback-read-model',
      'docs/expectations/app-game-evidence.md',
      'docs/expectations/policy.md',
    ],
    parentSurfaceDrillInRef: 'future-parent-surface-audit-rollback-drill-in-proof',
    parentSurfaceProofRef: 'future-parent-surface-audit-rollback-intent-proof',
  };
}

function summarize(intent) {
  return {
    sourceAuditRollbackReadModelId: intent.sourceAuditRollbackReadModelId,
    nativeAppRowCount: intent.nativeAppRowCount,
    nativeGameRowCount: intent.nativeGameRowCount,
    auditRollbackParentSurfaceProofRequiredCount: intent.auditRollbackParentSurfaceProofRequiredCount,
    blockedBySourceFreshnessCount: intent.blockedBySourceFreshnessCount,
    blockedByCompilerDecisionCount: intent.blockedByCompilerDecisionCount,
  };
}

function assertProof(proof) {
  if (proof.summary.auditRollbackParentSurfaceProofRequiredCount < 1) {
    throw new Error('Expected at least one audit rollback parent-surface proof-required row');
  }
  if (proof.summary.blockedBySourceFreshnessCount < 1 || proof.summary.blockedByCompilerDecisionCount < 1) {
    throw new Error('Expected source-freshness and compiler-decision blocked rows');
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(
      'Expected WP85 proof to avoid service, UI, evaluator, timer, scheduler storage, audit log, rollback execution, adapter, child, platform, and raw-source claims'
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
