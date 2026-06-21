import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofSlug = '102-timer-service-handoff';
const testOutputDir = join(repoRoot, 'test-results', 'app-game-timer-service-handoff-proof');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', proofSlug);
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', proofSlug);
const timestamp = '2026-06-06T11:45:00Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

for (const path of [testOutputDir, appGameProofDir, appProofDir]) {
  await rm(path, { recursive: true, force: true });
  await mkdir(path, { recursive: true });
}

runNpm(['run', 'build', '--workspace', '@ocentra-parent/schema-domain']);
runNpm(['run', 'build', '--workspace', '@ocentra-parent/app-game-domain']);
runNpm([
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/app-game-domain',
  '--',
  'app-game-timer-service-handoff',
  'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model',
]);

const contract = await importAppGameDist('app-game-timer-service-handoff.js');
const refs = await importSchemaDist('reference-primitives.js');
const sourceReadModel = await readJson(
  join(repoRoot, 'test-results', 'app-game-timer-parent-read-model-proof', 'handoff.json')
);
const serviceHandoff =
  contract.buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoff(
    serviceHandoffOptions(refs),
    sourceReadModel
  );
const proof = {
  proofMode: 'app-game-timer-service-handoff',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  stackedOn: {
    wp101Branch:
      'codex/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-contract',
    reason:
      'Schema-domain owns the timer-service handoff contract surface; app-game-domain consumes WP101 parent-safe parent-surface read-model rows while service event/read-model emission, read APIs, protocol implementation, runtime persistence, portal rendering, adapter dispatch, child delivery, platform enforcement, package exports, and raw source rows remain sequenced separately.',
  },
  summary: summarize(serviceHandoff),
  nonClaims: {
    serviceCommandRegistered: serviceHandoff.serviceCommandRegistered,
    serviceHandlerImplemented: serviceHandoff.serviceHandlerImplemented,
    serviceReadModelEmitted: serviceHandoff.serviceReadModelEmitted,
    serviceReadApiImplemented: serviceHandoff.serviceReadApiImplemented,
    serviceReadApiResponseImplemented: serviceHandoff.serviceReadApiResponseImplemented,
    serviceReadApiResponseConsumerImplemented: serviceHandoff.serviceReadApiResponseConsumerImplemented,
    serviceEventEmitted: serviceHandoff.serviceEventEmitted,
    agentProtocolImplemented: serviceHandoff.agentProtocolImplemented,
    rustProtocolMirrored: serviceHandoff.rustProtocolMirrored,
    portalUiRendered: serviceHandoff.portalUiRendered,
    portalResponseConsumerRendered: serviceHandoff.portalResponseConsumerRendered,
    parentSurfaceRendered: serviceHandoff.parentSurfaceRendered,
    parentSurfaceReadModelRuntimeImplemented: serviceHandoff.parentSurfaceReadModelRuntimeImplemented,
    parentSurfaceReadModelPersisted: serviceHandoff.parentSurfaceReadModelPersisted,
    policyEvaluatorRuntimeClaimed: serviceHandoff.policyEvaluatorRuntimeClaimed,
    timerRuntimeClaimed: serviceHandoff.timerRuntimeClaimed,
    timerScheduled: serviceHandoff.timerScheduled,
    schedulerPersistenceRuntimeClaimed: serviceHandoff.schedulerPersistenceRuntimeClaimed,
    durableSchedulerStorageClaimed: serviceHandoff.durableSchedulerStorageClaimed,
    auditRuntimeClaimed: serviceHandoff.auditRuntimeClaimed,
    durableAuditLogClaimed: serviceHandoff.durableAuditLogClaimed,
    rollbackRuntimeClaimed: serviceHandoff.rollbackRuntimeClaimed,
    rollbackExecutionClaimed: serviceHandoff.rollbackExecutionClaimed,
    adapterDispatchClaimed: serviceHandoff.adapterDispatchClaimed,
    childDeliveryClaimed: serviceHandoff.childDeliveryClaimed,
    platformEnforcementClaimed: serviceHandoff.platformEnforcementClaimed,
    rawPrivateSourceRowsIncluded: serviceHandoff.rawPrivateSourceRowsIncluded,
  },
  proofPaths: {
    schemaSource: 'packages/schema-domain/src/app-game-timer-service-handoff.ts',
    schemaRules: 'packages/schema-domain/src/app-game-timer-service-handoff-rules.ts',
    consumerSource: 'packages/app-game-domain/src/app-game-timer-service-handoff.ts',
    consumerTest: 'packages/app-game-domain/tests/unit/app-game-timer-service-handoff.test.ts',
    harness:
      'scripts/test/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-handoff-proof.mjs',
    evidence: 'test-results/app-game-timer-service-handoff-proof/proof.json',
    appGameProofPack: `output/app-game-plan-proof/${proofSlug}`,
    appProofPack: `output/app-plan-proof/${proofSlug}`,
  },
  serviceHandoff,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'handoff.json'), serviceHandoff);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(appGameProofDir, proof, 'app-game WP102');
await writeProofPack(appProofDir, proof, 'app WP102');

console.log('app-game-timer-service-handoff-proof-ok');
console.log(`evidence=${join('test-results', 'app-game-timer-service-handoff-proof', 'proof.json')}`);

function importAppGameDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'app-game-domain', 'dist', name)).href);
}

function importSchemaDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'schema-domain', 'dist', name)).href);
}

function serviceHandoffOptions(refs) {
  return {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    parentSurfaceReadModelServiceHandoffId: 'app-game-timer-service-handoff-proof',
    generatedAt: timestamp,
    sourceContractRefs: [
      'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model',
      'docs/expectations/app-game-evidence.md',
      'docs/expectations/enforcement.md',
    ],
    serviceProofRefs: [
      'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-proof',
    ],
  };
}

function summarize(serviceHandoff) {
  return {
    sourceParentSurfaceReadModelId: serviceHandoff.sourceParentSurfaceReadModelId,
    nativeAppRowCount: serviceHandoff.nativeAppRowCount,
    nativeGameRowCount: serviceHandoff.nativeGameRowCount,
    serviceProofRequiredCount: serviceHandoff.serviceProofRequiredCount,
    blockedBySourceFreshnessCount: serviceHandoff.blockedBySourceFreshnessCount,
    blockedByCompilerDecisionCount: serviceHandoff.blockedByCompilerDecisionCount,
  };
}

function assertProof(proof) {
  if (proof.summary.serviceProofRequiredCount < 1) {
    throw new Error('Expected at least one service-proof-required row');
  }
  if (proof.summary.blockedBySourceFreshnessCount < 1 || proof.summary.blockedByCompilerDecisionCount < 1) {
    throw new Error('Expected source-freshness and compiler-decision blocked rows');
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(
      'Expected WP102 proof to avoid service emission/API/runtime, protocol, read-model runtime/persistence, portal UI, evaluator, timer, scheduler storage, audit log, rollback execution, adapter, child, platform, and raw-source claims'
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
