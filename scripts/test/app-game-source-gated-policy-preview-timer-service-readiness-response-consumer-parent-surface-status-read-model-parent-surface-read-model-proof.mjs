import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofSlug =
  '101-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model';
const testOutputDir = join(
  repoRoot,
  'test-results',
  'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof'
);
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', proofSlug);
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', proofSlug);
const timestamp = '2026-06-06T11:30:00Z';
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
  'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model',
  'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff',
]);

const contract = await importDist(
  'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model.js'
);
const refs = await importDist('reference-primitives.js');
const sourceHandoff = await readJson(
  join(
    repoRoot,
    'test-results',
    'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff-proof',
    'timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff.json'
  )
);
const parentSurfaceReadModel =
  contract.buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModel(
    parentSurfaceReadModelOptions(refs),
    sourceHandoff
  );
const proof = {
  proofMode:
    'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  stackedOn: {
    wp100Branch:
      'codex/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff',
    reason:
      'WP101 consumes WP100 parent-surface read-model handoff rows and produces a parent-domain parent-surface read-model contract while service runtime, persistence, portal rendering, protocol implementation, timer runtime, scheduler/audit storage, rollback execution, adapter dispatch, child delivery, platform enforcement, package exports, and raw source rows remain sequenced separately.',
  },
  summary: summarize(parentSurfaceReadModel),
  nonClaims: {
    serviceCommandRegistered: parentSurfaceReadModel.serviceCommandRegistered,
    serviceHandlerImplemented: parentSurfaceReadModel.serviceHandlerImplemented,
    serviceReadApiImplemented: parentSurfaceReadModel.serviceReadApiImplemented,
    serviceReadApiResponseImplemented: parentSurfaceReadModel.serviceReadApiResponseImplemented,
    serviceReadApiResponseConsumerImplemented: parentSurfaceReadModel.serviceReadApiResponseConsumerImplemented,
    serviceEventEmitted: parentSurfaceReadModel.serviceEventEmitted,
    agentProtocolImplemented: parentSurfaceReadModel.agentProtocolImplemented,
    rustProtocolMirrored: parentSurfaceReadModel.rustProtocolMirrored,
    portalUiRendered: parentSurfaceReadModel.portalUiRendered,
    portalResponseConsumerRendered: parentSurfaceReadModel.portalResponseConsumerRendered,
    parentSurfaceRendered: parentSurfaceReadModel.parentSurfaceRendered,
    parentSurfaceReadModelRuntimeImplemented: parentSurfaceReadModel.parentSurfaceReadModelRuntimeImplemented,
    parentSurfaceReadModelPersisted: parentSurfaceReadModel.parentSurfaceReadModelPersisted,
    policyEvaluatorRuntimeClaimed: parentSurfaceReadModel.policyEvaluatorRuntimeClaimed,
    timerRuntimeClaimed: parentSurfaceReadModel.timerRuntimeClaimed,
    timerScheduled: parentSurfaceReadModel.timerScheduled,
    schedulerPersistenceRuntimeClaimed: parentSurfaceReadModel.schedulerPersistenceRuntimeClaimed,
    durableSchedulerStorageClaimed: parentSurfaceReadModel.durableSchedulerStorageClaimed,
    auditRuntimeClaimed: parentSurfaceReadModel.auditRuntimeClaimed,
    durableAuditLogClaimed: parentSurfaceReadModel.durableAuditLogClaimed,
    rollbackRuntimeClaimed: parentSurfaceReadModel.rollbackRuntimeClaimed,
    rollbackExecutionClaimed: parentSurfaceReadModel.rollbackExecutionClaimed,
    adapterDispatchClaimed: parentSurfaceReadModel.adapterDispatchClaimed,
    childDeliveryClaimed: parentSurfaceReadModel.childDeliveryClaimed,
    platformEnforcementClaimed: parentSurfaceReadModel.platformEnforcementClaimed,
    rawPrivateSourceRowsIncluded: parentSurfaceReadModel.rawPrivateSourceRowsIncluded,
  },
  proofPaths: {
    source:
      'packages/parent-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model.ts',
    rules:
      'packages/parent-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-rules.ts',
    test: 'packages/parent-domain/tests/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model.test.ts',
    harness:
      'scripts/test/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof.mjs',
    evidence:
      'test-results/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof/proof.json',
    appGameProofPack: `output/app-game-plan-proof/${proofSlug}`,
    appProofPack: `output/app-plan-proof/${proofSlug}`,
  },
  parentSurfaceReadModel,
};

assertProof(proof);
await writeJson(
  join(
    testOutputDir,
    'timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model.json'
  ),
  parentSurfaceReadModel
);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(appGameProofDir, proof, 'app-game WP101');
await writeProofPack(appProofDir, proof, 'app WP101');

console.log(
  'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof-ok'
);
console.log(
  `evidence=${join(
    'test-results',
    'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof',
    'proof.json'
  )}`
);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function parentSurfaceReadModelOptions(refs) {
  return {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    parentSurfaceReadModelId:
      'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof',
    generatedAt: timestamp,
    sourceContractRefs: [
      'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff',
      'docs/expectations/app-game-evidence.md',
      'docs/expectations/enforcement.md',
    ],
  };
}

function summarize(parentSurfaceReadModel) {
  return {
    sourceParentSurfaceReadModelHandoffId: parentSurfaceReadModel.sourceParentSurfaceReadModelHandoffId,
    nativeAppRowCount: parentSurfaceReadModel.nativeAppRowCount,
    nativeGameRowCount: parentSurfaceReadModel.nativeGameRowCount,
    readyForParentSurfaceReadModelCount: parentSurfaceReadModel.readyForParentSurfaceReadModelCount,
    blockedBySourceFreshnessCount: parentSurfaceReadModel.blockedBySourceFreshnessCount,
    blockedByCompilerDecisionCount: parentSurfaceReadModel.blockedByCompilerDecisionCount,
  };
}

function assertProof(proof) {
  if (proof.summary.readyForParentSurfaceReadModelCount < 1) {
    throw new Error('Expected at least one ready parent-surface read-model contract row');
  }
  if (proof.summary.blockedBySourceFreshnessCount < 1 || proof.summary.blockedByCompilerDecisionCount < 1) {
    throw new Error('Expected source-freshness and compiler-decision blocked rows');
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(
      'Expected WP101 proof to avoid service runtime, read-model runtime/persistence, portal UI, evaluator, timer, scheduler storage, audit log, rollback execution, adapter, child, platform, and raw-source claims'
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
