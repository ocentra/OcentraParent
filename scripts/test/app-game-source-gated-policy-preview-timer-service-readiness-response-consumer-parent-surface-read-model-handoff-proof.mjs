import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofSlug =
  '96-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-read-model-handoff';
const testOutputDir = join(repoRoot, 'test-results', 'app-game-timer-parent-rm-proof');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', proofSlug);
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', proofSlug);
const timestamp = '2026-06-06T09:45:00Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

for (const path of [testOutputDir, appGameProofDir, appProofDir]) {
  await rm(path, { recursive: true, force: true });
  await mkdir(path, { recursive: true });
}

runNpm(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
runNpm([
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-read-model-handoff',
  'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-handoff',
]);

const readModelContract = await importDist(
  'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-read-model-handoff.js'
);
const refs = await importDist('reference-primitives.js');
const parentSurfaceHandoff = await readJson(
  join(repoRoot, 'test-results', 'app-game-timer-parent-surface-proof', 'handoff.json')
);
const parentSurfaceReadModelHandoff =
  readModelContract.buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoff(
    parentSurfaceReadModelHandoffOptions(refs),
    parentSurfaceHandoff
  );
const proof = {
  proofMode:
    'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-read-model-handoff',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  stackedOn: {
    wp95Branch:
      'codex/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-handoff',
    reason:
      'WP96 consumes WP95 response-consumer parent-surface handoff rows and creates a parent-domain parent-surface read-model handoff while actual read-model implementation, parent-surface rendering, portal rendering, service consumer implementation, service command registration, service handler implementation, service event emission, service read API implementation, timer runtime, durable scheduler/audit storage, rollback execution, adapter dispatch, child delivery, platform enforcement, and package exports are sequenced separately.',
  },
  summary: summarize(parentSurfaceReadModelHandoff),
  nonClaims: {
    serviceCommandRegistered: parentSurfaceReadModelHandoff.serviceCommandRegistered,
    serviceHandlerImplemented: parentSurfaceReadModelHandoff.serviceHandlerImplemented,
    serviceReadApiImplemented: parentSurfaceReadModelHandoff.serviceReadApiImplemented,
    serviceReadApiResponseImplemented: parentSurfaceReadModelHandoff.serviceReadApiResponseImplemented,
    serviceReadApiResponseConsumerImplemented: parentSurfaceReadModelHandoff.serviceReadApiResponseConsumerImplemented,
    parentSurfaceReadModelImplemented: parentSurfaceReadModelHandoff.parentSurfaceReadModelImplemented,
    serviceEventEmitted: parentSurfaceReadModelHandoff.serviceEventEmitted,
    agentProtocolImplemented: parentSurfaceReadModelHandoff.agentProtocolImplemented,
    rustProtocolMirrored: parentSurfaceReadModelHandoff.rustProtocolMirrored,
    portalUiRendered: parentSurfaceReadModelHandoff.portalUiRendered,
    portalResponseConsumerRendered: parentSurfaceReadModelHandoff.portalResponseConsumerRendered,
    parentSurfaceRendered: parentSurfaceReadModelHandoff.parentSurfaceRendered,
    policyEvaluatorRuntimeClaimed: parentSurfaceReadModelHandoff.policyEvaluatorRuntimeClaimed,
    timerRuntimeClaimed: parentSurfaceReadModelHandoff.timerRuntimeClaimed,
    timerScheduled: parentSurfaceReadModelHandoff.timerScheduled,
    schedulerPersistenceRuntimeClaimed: parentSurfaceReadModelHandoff.schedulerPersistenceRuntimeClaimed,
    durableSchedulerStorageClaimed: parentSurfaceReadModelHandoff.durableSchedulerStorageClaimed,
    auditRuntimeClaimed: parentSurfaceReadModelHandoff.auditRuntimeClaimed,
    durableAuditLogClaimed: parentSurfaceReadModelHandoff.durableAuditLogClaimed,
    rollbackRuntimeClaimed: parentSurfaceReadModelHandoff.rollbackRuntimeClaimed,
    rollbackExecutionClaimed: parentSurfaceReadModelHandoff.rollbackExecutionClaimed,
    adapterDispatchClaimed: parentSurfaceReadModelHandoff.adapterDispatchClaimed,
    childDeliveryClaimed: parentSurfaceReadModelHandoff.childDeliveryClaimed,
    platformEnforcementClaimed: parentSurfaceReadModelHandoff.platformEnforcementClaimed,
    rawPrivateSourceRowsIncluded: parentSurfaceReadModelHandoff.rawPrivateSourceRowsIncluded,
  },
  proofPaths: {
    source:
      'packages/parent-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-read-model-handoff.ts',
    rules:
      'packages/parent-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-read-model-handoff-rules.ts',
    test: 'packages/parent-domain/tests/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-read-model-handoff.test.ts',
    harness: 'scripts/test/app-game-timer-parent-rm-proof.mjs',
    evidence: 'test-results/app-game-timer-parent-rm-proof/proof.json',
    appGameProofPack: `output/app-game-plan-proof/${proofSlug}`,
    appProofPack: `output/app-plan-proof/${proofSlug}`,
  },
  parentSurfaceReadModelHandoff,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'handoff.json'), parentSurfaceReadModelHandoff);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(appGameProofDir, proof, 'app-game WP96');
await writeProofPack(appProofDir, proof, 'app WP96');

console.log('app-game-timer-parent-rm-proof-ok');
console.log(`evidence=${join('test-results', 'app-game-timer-parent-rm-proof', 'proof.json')}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function parentSurfaceReadModelHandoffOptions(refs) {
  return {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    responseConsumerParentSurfaceReadModelHandoffId: 'app-game-timer-parent-rm-proof',
    generatedAt: timestamp,
    sourceContractRefs: [
      'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-handoff',
      'docs/expectations/app-game-evidence.md',
      'docs/expectations/enforcement.md',
    ],
    parentSurfaceReadModelProofRefs: [
      'future-app-game-timer-service-readiness-response-consumer-parent-surface-read-model-proof',
    ],
    parentSurfaceReadModelRef: 'future-service-readiness-response-consumer-parent-surface-read-model-proof',
  };
}

function summarize(parentSurfaceReadModelHandoff) {
  return {
    sourceResponseConsumerParentSurfaceHandoffId:
      parentSurfaceReadModelHandoff.sourceResponseConsumerParentSurfaceHandoffId,
    nativeAppRowCount: parentSurfaceReadModelHandoff.nativeAppRowCount,
    nativeGameRowCount: parentSurfaceReadModelHandoff.nativeGameRowCount,
    parentSurfaceReadModelProofRequiredCount: parentSurfaceReadModelHandoff.parentSurfaceReadModelProofRequiredCount,
    blockedBySourceFreshnessCount: parentSurfaceReadModelHandoff.blockedBySourceFreshnessCount,
    blockedByCompilerDecisionCount: parentSurfaceReadModelHandoff.blockedByCompilerDecisionCount,
  };
}

function assertProof(proof) {
  if (proof.summary.parentSurfaceReadModelProofRequiredCount < 1) {
    throw new Error('Expected at least one response consumer parent-surface read-model proof-required row');
  }
  if (proof.summary.blockedBySourceFreshnessCount < 1 || proof.summary.blockedByCompilerDecisionCount < 1) {
    throw new Error('Expected source-freshness and compiler-decision blocked rows');
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(
      'Expected WP96 proof to avoid service runtime, response/consumer/read-model implementation, portal UI, evaluator, timer, scheduler storage, audit log, rollback execution, adapter, child, platform, and raw-source claims'
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
