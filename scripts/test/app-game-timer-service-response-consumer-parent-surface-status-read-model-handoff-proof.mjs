import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofSlug = '111-timer-service-response-consumer-parent-surface-status-read-model';
const testOutputDir = join(
  repoRoot,
  'test-results',
  'app-game-timer-service-response-consumer-parent-surface-status-read-model-handoff-proof'
);
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', proofSlug);
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', proofSlug);
const timestamp = '2026-06-06T16:25:00Z';
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
  'app-game-timer-service-response-consumer-parent-surface-status-read-model-handoff',
  'app-game-timer-service-response-consumer-parent-surface-status-handoff',
]);

const contract = await importDist(
  'app-game-timer-service-response-consumer-parent-surface-status-read-model-handoff.js'
);
const parentSurfaceStatusContract = await importDist(
  'app-game-timer-service-response-consumer-parent-surface-status-handoff.js'
);
const refs = await importDist('reference-primitives.js');
const sourceParentSurfaceStatusHandoff =
  parentSurfaceStatusContract.AppGameTimerServiceResponseConsumerParentSurfaceStatusHandoffSchema.parse(
    await readJson(
      join(
        repoRoot,
        'test-results',
        'app-game-timer-service-response-consumer-parent-surface-status-handoff-proof',
        'handoff.json'
      )
    )
  );
const parentSurfaceStatusReadModelHandoff =
  contract.buildAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoff(
    parentSurfaceStatusReadModelHandoffOptions(refs),
    sourceParentSurfaceStatusHandoff
  );
const proof = {
  proofMode: 'app-game-timer-service-response-consumer-parent-surface-status-read-model-handoff',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  stackedOn: {
    wp110Branch: 'codex/app-game-timer-service-response-consumer-parent-surface-status-handoff-wp110',
    reason:
      'WP111 consumes WP110 parent-domain service response-consumer parent-surface status handoff rows and records the future parent-surface status read-model proof needed before status read-model runtime, persistence, rendering, portal response consumer rendering, adapters, child delivery, platform enforcement, or raw source rows are claimed.',
  },
  summary: summarize(parentSurfaceStatusReadModelHandoff),
  nonClaims: pickNonClaims(parentSurfaceStatusReadModelHandoff),
  proofPaths: {
    source:
      'packages/parent-domain/src/app-game-timer-service-response-consumer-parent-surface-status-read-model-handoff.ts',
    rules:
      'packages/parent-domain/src/app-game-timer-service-response-consumer-parent-surface-status-read-model-handoff-rules.ts',
    test: 'packages/parent-domain/tests/app-game-timer-service-response-consumer-parent-surface-status-read-model-handoff.test.ts',
    harness: 'scripts/test/app-game-timer-service-response-consumer-parent-surface-status-read-model-handoff-proof.mjs',
    evidence:
      'test-results/app-game-timer-service-response-consumer-parent-surface-status-read-model-handoff-proof/proof.json',
    appGameProofPack: `output/app-game-plan-proof/${proofSlug}`,
    appProofPack: `output/app-plan-proof/${proofSlug}`,
  },
  parentSurfaceStatusReadModelHandoff,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'handoff.json'), parentSurfaceStatusReadModelHandoff);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(appGameProofDir, proof, 'app-game WP111');
await writeProofPack(appProofDir, proof, 'app WP111');

console.log('app-game-timer-service-response-consumer-parent-surface-status-read-model-handoff-proof-ok');
console.log(
  `evidence=${join(
    'test-results',
    'app-game-timer-service-response-consumer-parent-surface-status-read-model-handoff-proof',
    'proof.json'
  )}`
);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function parentSurfaceStatusReadModelHandoffOptions(refs) {
  return {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    serviceResponseConsumerParentSurfaceStatusReadModelHandoffId:
      'app-game-timer-service-response-consumer-parent-surface-status-read-model-handoff-proof',
    generatedAt: timestamp,
    sourceContractRefs: [
      'app-game-timer-service-response-consumer-parent-surface-status-handoff',
      'docs/expectations/app-game-evidence.md',
      'docs/expectations/enforcement.md',
    ],
    parentSurfaceStatusReadModelProofRefs: [
      'future-app-game-timer-service-read-api-response-consumer-parent-surface-status-read-model-proof',
    ],
  };
}

function summarize(parentSurfaceStatusReadModelHandoff) {
  return {
    sourceServiceResponseConsumerParentSurfaceStatusHandoffId:
      parentSurfaceStatusReadModelHandoff.sourceServiceResponseConsumerParentSurfaceStatusHandoffId,
    nativeAppRowCount: parentSurfaceStatusReadModelHandoff.nativeAppRowCount,
    nativeGameRowCount: parentSurfaceStatusReadModelHandoff.nativeGameRowCount,
    parentSurfaceStatusReadModelProofRequiredCount:
      parentSurfaceStatusReadModelHandoff.parentSurfaceStatusReadModelProofRequiredCount,
    blockedBySourceFreshnessCount: parentSurfaceStatusReadModelHandoff.blockedBySourceFreshnessCount,
    blockedByCompilerDecisionCount: parentSurfaceStatusReadModelHandoff.blockedByCompilerDecisionCount,
  };
}

function pickNonClaims(parentSurfaceStatusReadModelHandoff) {
  return {
    serviceCommandRegistered: parentSurfaceStatusReadModelHandoff.serviceCommandRegistered,
    serviceHandlerImplemented: parentSurfaceStatusReadModelHandoff.serviceHandlerImplemented,
    serviceReadModelRuntimeEmitted: parentSurfaceStatusReadModelHandoff.serviceReadModelRuntimeEmitted,
    serviceEventRuntimeEmitted: parentSurfaceStatusReadModelHandoff.serviceEventRuntimeEmitted,
    serviceEventEmitted: parentSurfaceStatusReadModelHandoff.serviceEventEmitted,
    serviceReadApiImplemented: parentSurfaceStatusReadModelHandoff.serviceReadApiImplemented,
    serviceReadApiResponseImplemented: parentSurfaceStatusReadModelHandoff.serviceReadApiResponseImplemented,
    serviceReadApiResponseConsumerImplemented:
      parentSurfaceStatusReadModelHandoff.serviceReadApiResponseConsumerImplemented,
    agentProtocolImplemented: parentSurfaceStatusReadModelHandoff.agentProtocolImplemented,
    rustProtocolMirrored: parentSurfaceStatusReadModelHandoff.rustProtocolMirrored,
    portalUiRendered: parentSurfaceStatusReadModelHandoff.portalUiRendered,
    portalResponseConsumerRendered: parentSurfaceStatusReadModelHandoff.portalResponseConsumerRendered,
    parentSurfaceRendered: parentSurfaceStatusReadModelHandoff.parentSurfaceRendered,
    parentSurfaceReadModelRuntimeImplemented:
      parentSurfaceStatusReadModelHandoff.parentSurfaceReadModelRuntimeImplemented,
    parentSurfaceReadModelPersisted: parentSurfaceStatusReadModelHandoff.parentSurfaceReadModelPersisted,
    parentSurfaceStatusRuntimeImplemented: parentSurfaceStatusReadModelHandoff.parentSurfaceStatusRuntimeImplemented,
    parentSurfaceStatusPersisted: parentSurfaceStatusReadModelHandoff.parentSurfaceStatusPersisted,
    parentSurfaceStatusReadModelRuntimeImplemented:
      parentSurfaceStatusReadModelHandoff.parentSurfaceStatusReadModelRuntimeImplemented,
    parentSurfaceStatusReadModelPersisted: parentSurfaceStatusReadModelHandoff.parentSurfaceStatusReadModelPersisted,
    policyEvaluatorRuntimeClaimed: parentSurfaceStatusReadModelHandoff.policyEvaluatorRuntimeClaimed,
    timerRuntimeClaimed: parentSurfaceStatusReadModelHandoff.timerRuntimeClaimed,
    timerScheduled: parentSurfaceStatusReadModelHandoff.timerScheduled,
    schedulerPersistenceRuntimeClaimed: parentSurfaceStatusReadModelHandoff.schedulerPersistenceRuntimeClaimed,
    durableSchedulerStorageClaimed: parentSurfaceStatusReadModelHandoff.durableSchedulerStorageClaimed,
    auditRuntimeClaimed: parentSurfaceStatusReadModelHandoff.auditRuntimeClaimed,
    durableAuditLogClaimed: parentSurfaceStatusReadModelHandoff.durableAuditLogClaimed,
    rollbackRuntimeClaimed: parentSurfaceStatusReadModelHandoff.rollbackRuntimeClaimed,
    rollbackExecutionClaimed: parentSurfaceStatusReadModelHandoff.rollbackExecutionClaimed,
    adapterDispatchClaimed: parentSurfaceStatusReadModelHandoff.adapterDispatchClaimed,
    childDeliveryClaimed: parentSurfaceStatusReadModelHandoff.childDeliveryClaimed,
    platformEnforcementClaimed: parentSurfaceStatusReadModelHandoff.platformEnforcementClaimed,
    rawPrivateSourceRowsIncluded: parentSurfaceStatusReadModelHandoff.rawPrivateSourceRowsIncluded,
  };
}

function assertProof(proof) {
  if (proof.summary.parentSurfaceStatusReadModelProofRequiredCount < 1) {
    throw new Error('Expected at least one parent-surface-status-read-model-proof-required row');
  }
  if (proof.summary.blockedBySourceFreshnessCount < 1 || proof.summary.blockedByCompilerDecisionCount < 1) {
    throw new Error('Expected source-freshness and compiler-decision blocked rows');
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(
      'Expected WP111 proof to avoid service runtime, response consumer implementation, parent-surface status read-model runtime, status read-model persistence, parent-surface rendering, portal rendering, protocol, evaluator, timer, scheduler storage, audit log, rollback execution, adapter, child, platform, and raw-source claims'
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
