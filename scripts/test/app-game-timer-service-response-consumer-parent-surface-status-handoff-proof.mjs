import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofSlug = '110-timer-service-response-consumer-parent-surface-status';
const testOutputDir = join(
  repoRoot,
  'test-results',
  'app-game-timer-service-response-consumer-parent-surface-status-handoff-proof'
);
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', proofSlug);
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', proofSlug);
const timestamp = '2026-06-06T16:10:00Z';
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
  'app-game-timer-service-response-consumer-parent-surface-status-handoff',
  'app-game-timer-service-response-consumer-parent-surface-read-model-handoff',
]);

const contract = await importDist('app-game-timer-service-response-consumer-parent-surface-status-handoff.js');
const parentSurfaceReadModelContract = await importDist(
  'app-game-timer-service-response-consumer-parent-surface-read-model-handoff.js'
);
const refs = await importDist('reference-primitives.js');
const sourceParentSurfaceReadModelHandoff =
  parentSurfaceReadModelContract.AppGameTimerServiceResponseConsumerParentSurfaceReadModelHandoffSchema.parse(
    await readJson(
      join(
        repoRoot,
        'test-results',
        'app-game-timer-service-response-consumer-parent-surface-read-model-handoff-proof',
        'handoff.json'
      )
    )
  );
const parentSurfaceStatusHandoff = contract.buildAppGameTimerServiceResponseConsumerParentSurfaceStatusHandoff(
  parentSurfaceStatusHandoffOptions(refs),
  sourceParentSurfaceReadModelHandoff
);
const proof = {
  proofMode: 'app-game-timer-service-response-consumer-parent-surface-status-handoff',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  stackedOn: {
    wp109Branch: 'codex/app-game-timer-service-response-consumer-parent-surface-read-model-handoff-wp109-clean',
    reason:
      'WP110 consumes WP109 parent-domain service response-consumer parent-surface read-model handoff rows and records the future parent-surface status proof needed before status runtime, persistence, rendering, portal response consumer rendering, adapters, child delivery, platform enforcement, or raw source rows are claimed.',
  },
  summary: summarize(parentSurfaceStatusHandoff),
  nonClaims: pickNonClaims(parentSurfaceStatusHandoff),
  proofPaths: {
    source: 'packages/parent-domain/src/app-game-timer-service-response-consumer-parent-surface-status-handoff.ts',
    rules: 'packages/parent-domain/src/app-game-timer-service-response-consumer-parent-surface-status-handoff-rules.ts',
    test: 'packages/parent-domain/tests/app-game-timer-service-response-consumer-parent-surface-status-handoff.test.ts',
    harness: 'scripts/test/app-game-timer-service-response-consumer-parent-surface-status-handoff-proof.mjs',
    evidence: 'test-results/app-game-timer-service-response-consumer-parent-surface-status-handoff-proof/proof.json',
    appGameProofPack: `output/app-game-plan-proof/${proofSlug}`,
    appProofPack: `output/app-plan-proof/${proofSlug}`,
  },
  parentSurfaceStatusHandoff,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'handoff.json'), parentSurfaceStatusHandoff);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(appGameProofDir, proof, 'app-game WP110');
await writeProofPack(appProofDir, proof, 'app WP110');

console.log('app-game-timer-service-response-consumer-parent-surface-status-handoff-proof-ok');
console.log(
  `evidence=${join(
    'test-results',
    'app-game-timer-service-response-consumer-parent-surface-status-handoff-proof',
    'proof.json'
  )}`
);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function parentSurfaceStatusHandoffOptions(refs) {
  return {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    serviceResponseConsumerParentSurfaceStatusHandoffId:
      'app-game-timer-service-response-consumer-parent-surface-status-handoff-proof',
    generatedAt: timestamp,
    sourceContractRefs: [
      'app-game-timer-service-response-consumer-parent-surface-read-model-handoff',
      'docs/expectations/app-game-evidence.md',
      'docs/expectations/enforcement.md',
    ],
    parentSurfaceStatusProofRefs: [
      'future-app-game-timer-service-read-api-response-consumer-parent-surface-status-proof',
    ],
  };
}

function summarize(parentSurfaceStatusHandoff) {
  return {
    sourceServiceResponseConsumerParentSurfaceReadModelHandoffId:
      parentSurfaceStatusHandoff.sourceServiceResponseConsumerParentSurfaceReadModelHandoffId,
    nativeAppRowCount: parentSurfaceStatusHandoff.nativeAppRowCount,
    nativeGameRowCount: parentSurfaceStatusHandoff.nativeGameRowCount,
    parentSurfaceStatusProofRequiredCount: parentSurfaceStatusHandoff.parentSurfaceStatusProofRequiredCount,
    blockedBySourceFreshnessCount: parentSurfaceStatusHandoff.blockedBySourceFreshnessCount,
    blockedByCompilerDecisionCount: parentSurfaceStatusHandoff.blockedByCompilerDecisionCount,
  };
}

function pickNonClaims(parentSurfaceStatusHandoff) {
  return {
    serviceCommandRegistered: parentSurfaceStatusHandoff.serviceCommandRegistered,
    serviceHandlerImplemented: parentSurfaceStatusHandoff.serviceHandlerImplemented,
    serviceReadModelRuntimeEmitted: parentSurfaceStatusHandoff.serviceReadModelRuntimeEmitted,
    serviceEventRuntimeEmitted: parentSurfaceStatusHandoff.serviceEventRuntimeEmitted,
    serviceEventEmitted: parentSurfaceStatusHandoff.serviceEventEmitted,
    serviceReadApiImplemented: parentSurfaceStatusHandoff.serviceReadApiImplemented,
    serviceReadApiResponseImplemented: parentSurfaceStatusHandoff.serviceReadApiResponseImplemented,
    serviceReadApiResponseConsumerImplemented: parentSurfaceStatusHandoff.serviceReadApiResponseConsumerImplemented,
    agentProtocolImplemented: parentSurfaceStatusHandoff.agentProtocolImplemented,
    rustProtocolMirrored: parentSurfaceStatusHandoff.rustProtocolMirrored,
    portalUiRendered: parentSurfaceStatusHandoff.portalUiRendered,
    portalResponseConsumerRendered: parentSurfaceStatusHandoff.portalResponseConsumerRendered,
    parentSurfaceRendered: parentSurfaceStatusHandoff.parentSurfaceRendered,
    parentSurfaceReadModelRuntimeImplemented: parentSurfaceStatusHandoff.parentSurfaceReadModelRuntimeImplemented,
    parentSurfaceReadModelPersisted: parentSurfaceStatusHandoff.parentSurfaceReadModelPersisted,
    parentSurfaceStatusRuntimeImplemented: parentSurfaceStatusHandoff.parentSurfaceStatusRuntimeImplemented,
    parentSurfaceStatusPersisted: parentSurfaceStatusHandoff.parentSurfaceStatusPersisted,
    policyEvaluatorRuntimeClaimed: parentSurfaceStatusHandoff.policyEvaluatorRuntimeClaimed,
    timerRuntimeClaimed: parentSurfaceStatusHandoff.timerRuntimeClaimed,
    timerScheduled: parentSurfaceStatusHandoff.timerScheduled,
    schedulerPersistenceRuntimeClaimed: parentSurfaceStatusHandoff.schedulerPersistenceRuntimeClaimed,
    durableSchedulerStorageClaimed: parentSurfaceStatusHandoff.durableSchedulerStorageClaimed,
    auditRuntimeClaimed: parentSurfaceStatusHandoff.auditRuntimeClaimed,
    durableAuditLogClaimed: parentSurfaceStatusHandoff.durableAuditLogClaimed,
    rollbackRuntimeClaimed: parentSurfaceStatusHandoff.rollbackRuntimeClaimed,
    rollbackExecutionClaimed: parentSurfaceStatusHandoff.rollbackExecutionClaimed,
    adapterDispatchClaimed: parentSurfaceStatusHandoff.adapterDispatchClaimed,
    childDeliveryClaimed: parentSurfaceStatusHandoff.childDeliveryClaimed,
    platformEnforcementClaimed: parentSurfaceStatusHandoff.platformEnforcementClaimed,
    rawPrivateSourceRowsIncluded: parentSurfaceStatusHandoff.rawPrivateSourceRowsIncluded,
  };
}

function assertProof(proof) {
  if (proof.summary.parentSurfaceStatusProofRequiredCount < 1) {
    throw new Error('Expected at least one parent-surface-status-proof-required row');
  }
  if (proof.summary.blockedBySourceFreshnessCount < 1 || proof.summary.blockedByCompilerDecisionCount < 1) {
    throw new Error('Expected source-freshness and compiler-decision blocked rows');
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(
      'Expected WP110 proof to avoid service runtime, response consumer implementation, parent-surface status runtime, status persistence, parent-surface rendering, portal rendering, protocol, evaluator, timer, scheduler storage, audit log, rollback execution, adapter, child, platform, and raw-source claims'
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
