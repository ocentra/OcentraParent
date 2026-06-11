import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofSlug =
  '99-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-handoff';
const testOutputDir = join(
  repoRoot,
  'test-results',
  'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-handoff-proof'
);
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', proofSlug);
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', proofSlug);
const timestamp = '2026-06-06T10:45:00Z';
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
  'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-handoff',
  'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-handoff',
]);

const contract = await importDist(
  'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-handoff.js'
);
const refs = await importDist('reference-primitives.js');
const sourceHandoff = await readJson(
  join(repoRoot, 'test-results', 'app-game-timer-parent-status-rm-proof', 'handoff.json')
);
const parentSurfaceHandoff =
  contract.buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoff(
    parentSurfaceHandoffOptions(refs),
    sourceHandoff
  );
const proof = {
  proofMode:
    'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-handoff',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  stackedOn: {
    wp98Branch:
      'codex/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-handoff',
    reason:
      'WP99 consumes WP98 response-consumer parent-surface status read-model rows and creates a parent-domain parent-surface handoff proof while actual parent-surface rendering, portal rendering, service runtime, protocol implementation, timer runtime, durable scheduler/audit storage, rollback execution, adapter dispatch, child delivery, platform enforcement, and package exports are sequenced separately.',
  },
  summary: summarize(parentSurfaceHandoff),
  nonClaims: {
    serviceCommandRegistered: parentSurfaceHandoff.serviceCommandRegistered,
    serviceHandlerImplemented: parentSurfaceHandoff.serviceHandlerImplemented,
    serviceReadApiImplemented: parentSurfaceHandoff.serviceReadApiImplemented,
    serviceReadApiResponseImplemented: parentSurfaceHandoff.serviceReadApiResponseImplemented,
    serviceReadApiResponseConsumerImplemented: parentSurfaceHandoff.serviceReadApiResponseConsumerImplemented,
    parentSurfaceReadModelImplemented: parentSurfaceHandoff.parentSurfaceReadModelImplemented,
    parentSurfaceStatusImplemented: parentSurfaceHandoff.parentSurfaceStatusImplemented,
    parentSurfaceStatusReadModelImplemented: parentSurfaceHandoff.parentSurfaceStatusReadModelImplemented,
    parentSurfaceStatusReadModelParentSurfaceImplemented:
      parentSurfaceHandoff.parentSurfaceStatusReadModelParentSurfaceImplemented,
    serviceEventEmitted: parentSurfaceHandoff.serviceEventEmitted,
    agentProtocolImplemented: parentSurfaceHandoff.agentProtocolImplemented,
    rustProtocolMirrored: parentSurfaceHandoff.rustProtocolMirrored,
    portalUiRendered: parentSurfaceHandoff.portalUiRendered,
    portalResponseConsumerRendered: parentSurfaceHandoff.portalResponseConsumerRendered,
    parentSurfaceRendered: parentSurfaceHandoff.parentSurfaceRendered,
    policyEvaluatorRuntimeClaimed: parentSurfaceHandoff.policyEvaluatorRuntimeClaimed,
    timerRuntimeClaimed: parentSurfaceHandoff.timerRuntimeClaimed,
    timerScheduled: parentSurfaceHandoff.timerScheduled,
    schedulerPersistenceRuntimeClaimed: parentSurfaceHandoff.schedulerPersistenceRuntimeClaimed,
    durableSchedulerStorageClaimed: parentSurfaceHandoff.durableSchedulerStorageClaimed,
    auditRuntimeClaimed: parentSurfaceHandoff.auditRuntimeClaimed,
    durableAuditLogClaimed: parentSurfaceHandoff.durableAuditLogClaimed,
    rollbackRuntimeClaimed: parentSurfaceHandoff.rollbackRuntimeClaimed,
    rollbackExecutionClaimed: parentSurfaceHandoff.rollbackExecutionClaimed,
    adapterDispatchClaimed: parentSurfaceHandoff.adapterDispatchClaimed,
    childDeliveryClaimed: parentSurfaceHandoff.childDeliveryClaimed,
    platformEnforcementClaimed: parentSurfaceHandoff.platformEnforcementClaimed,
    rawPrivateSourceRowsIncluded: parentSurfaceHandoff.rawPrivateSourceRowsIncluded,
  },
  proofPaths: {
    source:
      'packages/parent-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-handoff.ts',
    rules:
      'packages/parent-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-handoff-rules.ts',
    test: 'packages/parent-domain/tests/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-handoff.test.ts',
    harness:
      'scripts/test/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-handoff-proof.mjs',
    evidence:
      'test-results/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-handoff-proof/proof.json',
    appGameProofPack: `output/app-game-plan-proof/${proofSlug}`,
    appProofPack: `output/app-plan-proof/${proofSlug}`,
  },
  parentSurfaceHandoff,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'handoff.json'), parentSurfaceHandoff);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(appGameProofDir, proof, 'app-game WP99');
await writeProofPack(appProofDir, proof, 'app WP99');

console.log(
  'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-handoff-proof-ok'
);
console.log(
  `evidence=${join(
    'test-results',
    'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-handoff-proof',
    'proof.json'
  )}`
);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function parentSurfaceHandoffOptions(refs) {
  return {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    responseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffId:
      'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-handoff-proof',
    generatedAt: timestamp,
    sourceContractRefs: [
      'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-handoff',
      'docs/expectations/app-game-evidence.md',
      'docs/expectations/enforcement.md',
    ],
    parentSurfaceProofRefs: [
      'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-proof',
    ],
    parentSurfaceRef:
      'future-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-proof',
  };
}

function summarize(parentSurfaceHandoff) {
  return {
    sourceResponseConsumerParentSurfaceStatusReadModelHandoffId:
      parentSurfaceHandoff.sourceResponseConsumerParentSurfaceStatusReadModelHandoffId,
    nativeAppRowCount: parentSurfaceHandoff.nativeAppRowCount,
    nativeGameRowCount: parentSurfaceHandoff.nativeGameRowCount,
    parentSurfaceProofRequiredCount: parentSurfaceHandoff.parentSurfaceProofRequiredCount,
    blockedBySourceFreshnessCount: parentSurfaceHandoff.blockedBySourceFreshnessCount,
    blockedByCompilerDecisionCount: parentSurfaceHandoff.blockedByCompilerDecisionCount,
  };
}

function assertProof(proof) {
  if (proof.summary.parentSurfaceProofRequiredCount < 1) {
    throw new Error('Expected at least one parent-surface proof-required row');
  }
  if (proof.summary.blockedBySourceFreshnessCount < 1 || proof.summary.blockedByCompilerDecisionCount < 1) {
    throw new Error('Expected source-freshness and compiler-decision blocked rows');
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(
      'Expected WP99 proof to avoid service runtime, parent-surface implementation/rendering, portal UI, evaluator, timer, scheduler storage, audit log, rollback execution, adapter, child, platform, and raw-source claims'
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
