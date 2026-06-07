import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofSlug =
  '114-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-contract';
const testOutputDir = join(
  repoRoot,
  'test-results',
  'app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof'
);
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', proofSlug);
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', proofSlug);
const timestamp = '2026-06-06T19:05:00Z';
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
  'app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model.test',
  'app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff',
]);

const contract = await importDist(
  'app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model.js'
);
const handoffContract = await importDist(
  'app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff.js'
);
const refs = await importDist('reference-primitives.js');
const sourceHandoff =
  handoffContract.AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffSchema.parse(
    await readJson(
      join(
        repoRoot,
        'test-results',
        'app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff-proof',
        'handoff.json'
      )
    )
  );
const parentSurfaceReadModel =
  contract.buildAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModel(
    parentSurfaceReadModelOptions(refs),
    sourceHandoff
  );
const proof = {
  proofMode: 'app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  stackedOn: {
    wp113Branch:
      'codex/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff-wp113',
    reason:
      'WP114 consumes WP113 parent-domain parent-surface read-model handoff rows and emits parent-safe native app/native game rows without claiming package exports, runtime persistence, service runtime, protocol, portal UI, adapters, child delivery, platform enforcement, or raw source rows.',
  },
  summary: summarize(parentSurfaceReadModel),
  nonClaims: pickNonClaims(parentSurfaceReadModel),
  proofPaths: {
    source:
      'packages/parent-domain/src/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model.ts',
    rules:
      'packages/parent-domain/src/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-rules.ts',
    test: 'packages/parent-domain/tests/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model.test.ts',
    harness:
      'scripts/test/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof.mjs',
    evidence:
      'test-results/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof/proof.json',
    appGameProofPack: `output/app-game-plan-proof/${proofSlug}`,
    appProofPack: `output/app-plan-proof/${proofSlug}`,
  },
  parentSurfaceReadModel,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'read-model.json'), parentSurfaceReadModel);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(appGameProofDir, proof, 'app-game WP114');
await writeProofPack(appProofDir, proof, 'app WP114');

console.log(
  'app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof-ok'
);
console.log(
  `evidence=${join(
    'test-results',
    'app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof',
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
      'app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof',
    generatedAt: timestamp,
    sourceContractRefs: [
      'app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff',
      'docs/expectations/app-game-evidence.md',
      'docs/expectations/enforcement.md',
    ],
  };
}

function summarize(readModel) {
  return {
    sourceParentSurfaceReadModelHandoffId: readModel.sourceParentSurfaceReadModelHandoffId,
    nativeAppRowCount: readModel.nativeAppRowCount,
    nativeGameRowCount: readModel.nativeGameRowCount,
    readyForParentSurfaceReadModelCount: readModel.readyForParentSurfaceReadModelCount,
    blockedBySourceFreshnessCount: readModel.blockedBySourceFreshnessCount,
    blockedByCompilerDecisionCount: readModel.blockedByCompilerDecisionCount,
  };
}

function pickNonClaims(readModel) {
  return {
    packageExported: readModel.packageExported,
    serviceCommandRegistered: readModel.serviceCommandRegistered,
    serviceHandlerImplemented: readModel.serviceHandlerImplemented,
    serviceReadModelRuntimeEmitted: readModel.serviceReadModelRuntimeEmitted,
    serviceEventRuntimeEmitted: readModel.serviceEventRuntimeEmitted,
    serviceEventEmitted: readModel.serviceEventEmitted,
    serviceReadApiImplemented: readModel.serviceReadApiImplemented,
    serviceReadApiResponseImplemented: readModel.serviceReadApiResponseImplemented,
    serviceReadApiResponseConsumerImplemented: readModel.serviceReadApiResponseConsumerImplemented,
    agentProtocolImplemented: readModel.agentProtocolImplemented,
    rustProtocolMirrored: readModel.rustProtocolMirrored,
    portalUiRendered: readModel.portalUiRendered,
    portalResponseConsumerRendered: readModel.portalResponseConsumerRendered,
    parentSurfaceRendered: readModel.parentSurfaceRendered,
    runtimeReadModelPersisted: readModel.runtimeReadModelPersisted,
    parentSurfaceReadModelRuntimeImplemented: readModel.parentSurfaceReadModelRuntimeImplemented,
    parentSurfaceReadModelPersisted: readModel.parentSurfaceReadModelPersisted,
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
  };
}

function assertProof(proof) {
  if (proof.summary.readyForParentSurfaceReadModelCount < 1) {
    throw new Error('Expected at least one ready parent-surface read-model row');
  }
  if (proof.summary.blockedBySourceFreshnessCount < 1 || proof.summary.blockedByCompilerDecisionCount < 1) {
    throw new Error('Expected source-freshness and compiler-decision blocked rows');
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(
      'Expected WP114 proof to avoid package export, runtime, rendering, adapter, child, platform, and raw-source claims'
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
