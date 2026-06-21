import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofSlug =
  '98-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-handoff';
const testOutputDir = join(repoRoot, 'test-results', 'app-game-timer-parent-status-rm-proof');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', proofSlug);
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', proofSlug);
const timestamp = '2026-06-06T10:30:00Z';
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
  'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-handoff',
  'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-handoff',
]);

const statusReadModelContract = await importAppGameDist(
  'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-handoff.js'
);
const refs = await importSchemaDist('reference-primitives.js');
const parentSurfaceStatusHandoff = await readJson(
  join(repoRoot, 'test-results', 'app-game-timer-parent-status-proof', 'handoff.json')
);
const parentSurfaceStatusReadModelHandoff =
  statusReadModelContract.buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoff(
    parentSurfaceStatusReadModelHandoffOptions(refs),
    parentSurfaceStatusHandoff
  );
const proof = {
  proofMode:
    'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-handoff',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  stackedOn: {
    wp97Branch:
      'codex/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-handoff',
    reason:
      'Schema-domain owns the response-consumer parent-surface status read-model handoff contract surface; app-game-domain consumes WP97 response-consumer parent-surface status handoff rows while actual status read-model implementation, status implementation, parent-surface rendering, portal rendering, service consumer implementation, service command registration, service handler implementation, service event emission, service read API implementation, timer runtime, durable scheduler/audit storage, rollback execution, adapter dispatch, child delivery, platform enforcement, and package exports remain sequenced separately.',
  },
  summary: summarize(parentSurfaceStatusReadModelHandoff),
  nonClaims: {
    serviceCommandRegistered: parentSurfaceStatusReadModelHandoff.serviceCommandRegistered,
    serviceHandlerImplemented: parentSurfaceStatusReadModelHandoff.serviceHandlerImplemented,
    serviceReadApiImplemented: parentSurfaceStatusReadModelHandoff.serviceReadApiImplemented,
    serviceReadApiResponseImplemented: parentSurfaceStatusReadModelHandoff.serviceReadApiResponseImplemented,
    serviceReadApiResponseConsumerImplemented:
      parentSurfaceStatusReadModelHandoff.serviceReadApiResponseConsumerImplemented,
    parentSurfaceReadModelImplemented: parentSurfaceStatusReadModelHandoff.parentSurfaceReadModelImplemented,
    parentSurfaceStatusImplemented: parentSurfaceStatusReadModelHandoff.parentSurfaceStatusImplemented,
    parentSurfaceStatusReadModelImplemented:
      parentSurfaceStatusReadModelHandoff.parentSurfaceStatusReadModelImplemented,
    serviceEventEmitted: parentSurfaceStatusReadModelHandoff.serviceEventEmitted,
    agentProtocolImplemented: parentSurfaceStatusReadModelHandoff.agentProtocolImplemented,
    rustProtocolMirrored: parentSurfaceStatusReadModelHandoff.rustProtocolMirrored,
    portalUiRendered: parentSurfaceStatusReadModelHandoff.portalUiRendered,
    portalResponseConsumerRendered: parentSurfaceStatusReadModelHandoff.portalResponseConsumerRendered,
    parentSurfaceRendered: parentSurfaceStatusReadModelHandoff.parentSurfaceRendered,
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
  },
  proofPaths: {
    schemaSource:
      'packages/schema-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-handoff.ts',
    schemaRules:
      'packages/schema-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-handoff-rules.ts',
    consumerSource:
      'packages/app-game-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-handoff.ts',
    consumerTest:
      'packages/app-game-domain/tests/unit/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-handoff.test.ts',
    harness:
      'scripts/test/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-handoff-proof.mjs',
    evidence: 'test-results/app-game-timer-parent-status-rm-proof/proof.json',
    appGameProofPack: `output/app-game-plan-proof/${proofSlug}`,
    appProofPack: `output/app-plan-proof/${proofSlug}`,
  },
  parentSurfaceStatusReadModelHandoff,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'handoff.json'), parentSurfaceStatusReadModelHandoff);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(appGameProofDir, proof, 'app-game WP98');
await writeProofPack(appProofDir, proof, 'app WP98');

console.log('app-game-timer-parent-status-rm-proof-ok');
console.log(`evidence=${join('test-results', 'app-game-timer-parent-status-rm-proof', 'proof.json')}`);

function importAppGameDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'app-game-domain', 'dist', name)).href);
}

function importSchemaDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'schema-domain', 'dist', name)).href);
}

function parentSurfaceStatusReadModelHandoffOptions(refs) {
  return {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    responseConsumerParentSurfaceStatusReadModelHandoffId: 'app-game-timer-parent-status-rm-proof',
    generatedAt: timestamp,
    sourceContractRefs: [
      'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-handoff',
      'docs/expectations/app-game-evidence.md',
      'docs/expectations/enforcement.md',
    ],
    parentSurfaceStatusReadModelProofRefs: [
      'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-proof',
    ],
    parentSurfaceStatusReadModelRef:
      'future-service-readiness-response-consumer-parent-surface-status-read-model-proof',
  };
}

function summarize(parentSurfaceStatusReadModelHandoff) {
  return {
    sourceResponseConsumerParentSurfaceStatusHandoffId:
      parentSurfaceStatusReadModelHandoff.sourceResponseConsumerParentSurfaceStatusHandoffId,
    nativeAppRowCount: parentSurfaceStatusReadModelHandoff.nativeAppRowCount,
    nativeGameRowCount: parentSurfaceStatusReadModelHandoff.nativeGameRowCount,
    parentSurfaceStatusReadModelProofRequiredCount:
      parentSurfaceStatusReadModelHandoff.parentSurfaceStatusReadModelProofRequiredCount,
    blockedBySourceFreshnessCount: parentSurfaceStatusReadModelHandoff.blockedBySourceFreshnessCount,
    blockedByCompilerDecisionCount: parentSurfaceStatusReadModelHandoff.blockedByCompilerDecisionCount,
  };
}

function assertProof(proof) {
  if (proof.summary.parentSurfaceStatusReadModelProofRequiredCount < 1) {
    throw new Error('Expected at least one response consumer parent-surface status read-model proof-required row');
  }
  if (proof.summary.blockedBySourceFreshnessCount < 1 || proof.summary.blockedByCompilerDecisionCount < 1) {
    throw new Error('Expected source-freshness and compiler-decision blocked rows');
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(
      'Expected WP98 proof to avoid service runtime, response/consumer/read-model/status/status-read-model implementation, portal UI, evaluator, timer, scheduler storage, audit log, rollback execution, adapter, child, platform, and raw-source claims'
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
