import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofSlug = '104-timer-service-event';
const testOutputDir = join(repoRoot, 'test-results', 'app-game-timer-service-event-handoff-proof');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', proofSlug);
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', proofSlug);
const timestamp = '2026-06-06T12:10:00Z';
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
  'app-game-timer-service-event-handoff',
  'app-game-timer-service-read-model-handoff',
]);

const contract = await importDist('app-game-timer-service-event-handoff.js');
const serviceReadModelHandoffContract = await importDist('app-game-timer-service-read-model-handoff.js');
const refs = await importDist('reference-primitives.js');
const sourceServiceReadModelHandoff =
  serviceReadModelHandoffContract.AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffSchema.parse(
    await readJson(join(repoRoot, 'test-results', 'app-game-timer-service-read-model-handoff-proof', 'handoff.json'))
  );
const serviceEventHandoff =
  contract.buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoff(
    serviceEventHandoffOptions(refs),
    sourceServiceReadModelHandoff
  );
const proof = {
  proofMode: 'app-game-timer-service-event-handoff',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  stackedOn: {
    wp103Branch: 'codex/app-game-timer-service-read-model-handoff',
    reason:
      'WP104 consumes WP103 parent-domain service read-model handoff rows and records the future service event proof needed before runtime service event emission, read APIs, protocol, portal rendering, adapters, child delivery, platform enforcement, or raw source rows are claimed.',
  },
  summary: summarize(serviceEventHandoff),
  nonClaims: pickNonClaims(serviceEventHandoff),
  proofPaths: {
    source: 'packages/parent-domain/src/app-game-timer-service-event-handoff.ts',
    rules: 'packages/parent-domain/src/app-game-timer-service-event-handoff-rules.ts',
    test: 'packages/parent-domain/tests/app-game-timer-service-event-handoff.test.ts',
    harness: 'scripts/test/app-game-timer-service-event-handoff-proof.mjs',
    evidence: 'test-results/app-game-timer-service-event-handoff-proof/proof.json',
    appGameProofPack: `output/app-game-plan-proof/${proofSlug}`,
    appProofPack: `output/app-plan-proof/${proofSlug}`,
  },
  serviceEventHandoff,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'handoff.json'), serviceEventHandoff);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(appGameProofDir, proof, 'app-game WP104');
await writeProofPack(appProofDir, proof, 'app WP104');

console.log('app-game-timer-service-event-handoff-proof-ok');
console.log(`evidence=${join('test-results', 'app-game-timer-service-event-handoff-proof', 'proof.json')}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function serviceEventHandoffOptions(refs) {
  return {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    parentSurfaceReadModelServiceEventHandoffId: 'app-game-timer-service-event-handoff-proof',
    generatedAt: timestamp,
    sourceContractRefs: [
      'app-game-timer-service-read-model-handoff',
      'docs/expectations/app-game-evidence.md',
      'docs/expectations/enforcement.md',
    ],
    serviceEventProofRefs: [
      'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-event-proof',
    ],
  };
}

function summarize(serviceEventHandoff) {
  return {
    sourceParentSurfaceReadModelServiceReadModelHandoffId:
      serviceEventHandoff.sourceParentSurfaceReadModelServiceReadModelHandoffId,
    nativeAppRowCount: serviceEventHandoff.nativeAppRowCount,
    nativeGameRowCount: serviceEventHandoff.nativeGameRowCount,
    serviceEventProofRequiredCount: serviceEventHandoff.serviceEventProofRequiredCount,
    blockedBySourceFreshnessCount: serviceEventHandoff.blockedBySourceFreshnessCount,
    blockedByCompilerDecisionCount: serviceEventHandoff.blockedByCompilerDecisionCount,
  };
}

function pickNonClaims(serviceEventHandoff) {
  return {
    serviceCommandRegistered: serviceEventHandoff.serviceCommandRegistered,
    serviceHandlerImplemented: serviceEventHandoff.serviceHandlerImplemented,
    serviceReadModelRuntimeEmitted: serviceEventHandoff.serviceReadModelRuntimeEmitted,
    serviceEventRuntimeEmitted: serviceEventHandoff.serviceEventRuntimeEmitted,
    serviceEventEmitted: serviceEventHandoff.serviceEventEmitted,
    serviceReadApiImplemented: serviceEventHandoff.serviceReadApiImplemented,
    serviceReadApiResponseImplemented: serviceEventHandoff.serviceReadApiResponseImplemented,
    serviceReadApiResponseConsumerImplemented: serviceEventHandoff.serviceReadApiResponseConsumerImplemented,
    agentProtocolImplemented: serviceEventHandoff.agentProtocolImplemented,
    rustProtocolMirrored: serviceEventHandoff.rustProtocolMirrored,
    portalUiRendered: serviceEventHandoff.portalUiRendered,
    portalResponseConsumerRendered: serviceEventHandoff.portalResponseConsumerRendered,
    parentSurfaceRendered: serviceEventHandoff.parentSurfaceRendered,
    parentSurfaceReadModelRuntimeImplemented: serviceEventHandoff.parentSurfaceReadModelRuntimeImplemented,
    parentSurfaceReadModelPersisted: serviceEventHandoff.parentSurfaceReadModelPersisted,
    policyEvaluatorRuntimeClaimed: serviceEventHandoff.policyEvaluatorRuntimeClaimed,
    timerRuntimeClaimed: serviceEventHandoff.timerRuntimeClaimed,
    timerScheduled: serviceEventHandoff.timerScheduled,
    schedulerPersistenceRuntimeClaimed: serviceEventHandoff.schedulerPersistenceRuntimeClaimed,
    durableSchedulerStorageClaimed: serviceEventHandoff.durableSchedulerStorageClaimed,
    auditRuntimeClaimed: serviceEventHandoff.auditRuntimeClaimed,
    durableAuditLogClaimed: serviceEventHandoff.durableAuditLogClaimed,
    rollbackRuntimeClaimed: serviceEventHandoff.rollbackRuntimeClaimed,
    rollbackExecutionClaimed: serviceEventHandoff.rollbackExecutionClaimed,
    adapterDispatchClaimed: serviceEventHandoff.adapterDispatchClaimed,
    childDeliveryClaimed: serviceEventHandoff.childDeliveryClaimed,
    platformEnforcementClaimed: serviceEventHandoff.platformEnforcementClaimed,
    rawPrivateSourceRowsIncluded: serviceEventHandoff.rawPrivateSourceRowsIncluded,
  };
}

function assertProof(proof) {
  if (proof.summary.serviceEventProofRequiredCount < 1) {
    throw new Error('Expected at least one service-event-proof-required row');
  }
  if (proof.summary.blockedBySourceFreshnessCount < 1 || proof.summary.blockedByCompilerDecisionCount < 1) {
    throw new Error('Expected source-freshness and compiler-decision blocked rows');
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(
      'Expected WP104 proof to avoid service read-model runtime, service event runtime emission, events, API/runtime, protocol, read-model persistence, portal UI, evaluator, timer, scheduler storage, audit log, rollback execution, adapter, child, platform, and raw-source claims'
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
