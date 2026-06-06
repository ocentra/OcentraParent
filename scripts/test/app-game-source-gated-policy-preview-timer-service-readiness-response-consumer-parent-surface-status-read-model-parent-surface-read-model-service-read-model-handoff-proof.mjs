import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofSlug = '103-timer-service-read-model';
const testOutputDir = join(repoRoot, 'test-results', 'app-game-timer-service-read-model-handoff-proof');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', proofSlug);
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', proofSlug);
const timestamp = '2026-06-06T12:00:00Z';
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
  'app-game-timer-service-read-model-handoff',
  'app-game-timer-service-handoff',
]);

const contract = await importDist('app-game-timer-service-read-model-handoff.js');
const serviceHandoffContract = await importDist('app-game-timer-service-handoff.js');
const refs = await importDist('reference-primitives.js');
const sourceServiceHandoff =
  serviceHandoffContract.AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffSchema.parse(
    await readJson(join(repoRoot, 'test-results', 'app-game-timer-service-handoff-proof', 'handoff.json'))
  );
const serviceReadModelHandoff =
  contract.buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoff(
    serviceReadModelHandoffOptions(refs),
    sourceServiceHandoff
  );
const proof = {
  proofMode: 'app-game-timer-service-read-model-handoff',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  stackedOn: {
    wp102Branch: 'codex/app-game-timer-service-handoff',
    reason:
      'WP103 consumes WP102 parent-domain service handoff rows and records the future service read-model proof needed before runtime service emission, events, read APIs, protocol, portal rendering, adapters, child delivery, platform enforcement, or raw source rows are claimed.',
  },
  summary: summarize(serviceReadModelHandoff),
  nonClaims: pickNonClaims(serviceReadModelHandoff),
  proofPaths: {
    source: 'packages/parent-domain/src/app-game-timer-service-read-model-handoff.ts',
    rules: 'packages/parent-domain/src/app-game-timer-service-read-model-handoff-rules.ts',
    test: 'packages/parent-domain/tests/app-game-timer-service-read-model-handoff.test.ts',
    harness: 'scripts/test/app-game-timer-service-read-model-handoff-proof.mjs',
    evidence: 'test-results/app-game-timer-service-read-model-handoff-proof/proof.json',
    appGameProofPack: `output/app-game-plan-proof/${proofSlug}`,
    appProofPack: `output/app-plan-proof/${proofSlug}`,
  },
  serviceReadModelHandoff,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'handoff.json'), serviceReadModelHandoff);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(appGameProofDir, proof, 'app-game WP103');
await writeProofPack(appProofDir, proof, 'app WP103');

console.log('app-game-timer-service-read-model-handoff-proof-ok');
console.log(`evidence=${join('test-results', 'app-game-timer-service-read-model-handoff-proof', 'proof.json')}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function serviceReadModelHandoffOptions(refs) {
  return {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    parentSurfaceReadModelServiceReadModelHandoffId: 'app-game-timer-service-read-model-handoff-proof',
    generatedAt: timestamp,
    sourceContractRefs: [
      'app-game-timer-service-handoff',
      'docs/expectations/app-game-evidence.md',
      'docs/expectations/enforcement.md',
    ],
    serviceReadModelProofRefs: [
      'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-read-model-proof',
    ],
  };
}

function summarize(serviceReadModelHandoff) {
  return {
    sourceParentSurfaceReadModelServiceHandoffId: serviceReadModelHandoff.sourceParentSurfaceReadModelServiceHandoffId,
    nativeAppRowCount: serviceReadModelHandoff.nativeAppRowCount,
    nativeGameRowCount: serviceReadModelHandoff.nativeGameRowCount,
    serviceReadModelProofRequiredCount: serviceReadModelHandoff.serviceReadModelProofRequiredCount,
    blockedBySourceFreshnessCount: serviceReadModelHandoff.blockedBySourceFreshnessCount,
    blockedByCompilerDecisionCount: serviceReadModelHandoff.blockedByCompilerDecisionCount,
  };
}

function pickNonClaims(serviceReadModelHandoff) {
  return {
    serviceCommandRegistered: serviceReadModelHandoff.serviceCommandRegistered,
    serviceHandlerImplemented: serviceReadModelHandoff.serviceHandlerImplemented,
    serviceReadModelRuntimeEmitted: serviceReadModelHandoff.serviceReadModelRuntimeEmitted,
    serviceEventEmitted: serviceReadModelHandoff.serviceEventEmitted,
    serviceReadApiImplemented: serviceReadModelHandoff.serviceReadApiImplemented,
    serviceReadApiResponseImplemented: serviceReadModelHandoff.serviceReadApiResponseImplemented,
    serviceReadApiResponseConsumerImplemented: serviceReadModelHandoff.serviceReadApiResponseConsumerImplemented,
    agentProtocolImplemented: serviceReadModelHandoff.agentProtocolImplemented,
    rustProtocolMirrored: serviceReadModelHandoff.rustProtocolMirrored,
    portalUiRendered: serviceReadModelHandoff.portalUiRendered,
    portalResponseConsumerRendered: serviceReadModelHandoff.portalResponseConsumerRendered,
    parentSurfaceRendered: serviceReadModelHandoff.parentSurfaceRendered,
    parentSurfaceReadModelRuntimeImplemented: serviceReadModelHandoff.parentSurfaceReadModelRuntimeImplemented,
    parentSurfaceReadModelPersisted: serviceReadModelHandoff.parentSurfaceReadModelPersisted,
    policyEvaluatorRuntimeClaimed: serviceReadModelHandoff.policyEvaluatorRuntimeClaimed,
    timerRuntimeClaimed: serviceReadModelHandoff.timerRuntimeClaimed,
    timerScheduled: serviceReadModelHandoff.timerScheduled,
    schedulerPersistenceRuntimeClaimed: serviceReadModelHandoff.schedulerPersistenceRuntimeClaimed,
    durableSchedulerStorageClaimed: serviceReadModelHandoff.durableSchedulerStorageClaimed,
    auditRuntimeClaimed: serviceReadModelHandoff.auditRuntimeClaimed,
    durableAuditLogClaimed: serviceReadModelHandoff.durableAuditLogClaimed,
    rollbackRuntimeClaimed: serviceReadModelHandoff.rollbackRuntimeClaimed,
    rollbackExecutionClaimed: serviceReadModelHandoff.rollbackExecutionClaimed,
    adapterDispatchClaimed: serviceReadModelHandoff.adapterDispatchClaimed,
    childDeliveryClaimed: serviceReadModelHandoff.childDeliveryClaimed,
    platformEnforcementClaimed: serviceReadModelHandoff.platformEnforcementClaimed,
    rawPrivateSourceRowsIncluded: serviceReadModelHandoff.rawPrivateSourceRowsIncluded,
  };
}

function assertProof(proof) {
  if (proof.summary.serviceReadModelProofRequiredCount < 1) {
    throw new Error('Expected at least one service-read-model-proof-required row');
  }
  if (proof.summary.blockedBySourceFreshnessCount < 1 || proof.summary.blockedByCompilerDecisionCount < 1) {
    throw new Error('Expected source-freshness and compiler-decision blocked rows');
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(
      'Expected WP103 proof to avoid service read-model runtime emission, events, API/runtime, protocol, read-model persistence, portal UI, evaluator, timer, scheduler storage, audit log, rollback execution, adapter, child, platform, and raw-source claims'
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
