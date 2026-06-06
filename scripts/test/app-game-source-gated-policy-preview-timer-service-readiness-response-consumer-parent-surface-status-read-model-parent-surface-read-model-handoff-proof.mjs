import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofSlug =
  '100-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff';
const testOutputDir = join(
  repoRoot,
  'test-results',
  'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff-proof'
);
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', proofSlug);
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', proofSlug);
const timestamp = '2026-06-06T11:05:00Z';
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
  'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff',
  'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-handoff',
]);

const contract = await importDist(
  'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff.js'
);
const refs = await importDist('reference-primitives.js');
const sourceHandoff = await readJson(
  join(
    repoRoot,
    'test-results',
    'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-handoff-proof',
    'timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-handoff.json'
  )
);
const parentSurfaceReadModelHandoff =
  contract.buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoff(
    parentSurfaceReadModelHandoffOptions(refs),
    sourceHandoff
  );
const proof = {
  proofMode:
    'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  stackedOn: {
    wp99Branch:
      'codex/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-handoff',
    reason:
      'WP100 consumes WP99 response-consumer parent-surface status read-model parent-surface rows and creates a parent-domain parent-surface read-model handoff proof while actual parent-surface read-model implementation, parent-surface rendering, portal rendering, service runtime, protocol implementation, timer runtime, durable scheduler/audit storage, rollback execution, adapter dispatch, child delivery, platform enforcement, and package exports are sequenced separately.',
  },
  summary: summarize(parentSurfaceReadModelHandoff),
  nonClaims: {
    serviceCommandRegistered: parentSurfaceReadModelHandoff.serviceCommandRegistered,
    serviceHandlerImplemented: parentSurfaceReadModelHandoff.serviceHandlerImplemented,
    serviceReadApiImplemented: parentSurfaceReadModelHandoff.serviceReadApiImplemented,
    serviceReadApiResponseImplemented: parentSurfaceReadModelHandoff.serviceReadApiResponseImplemented,
    serviceReadApiResponseConsumerImplemented: parentSurfaceReadModelHandoff.serviceReadApiResponseConsumerImplemented,
    parentSurfaceReadModelImplemented: parentSurfaceReadModelHandoff.parentSurfaceReadModelImplemented,
    parentSurfaceStatusImplemented: parentSurfaceReadModelHandoff.parentSurfaceStatusImplemented,
    parentSurfaceStatusReadModelImplemented: parentSurfaceReadModelHandoff.parentSurfaceStatusReadModelImplemented,
    parentSurfaceStatusReadModelParentSurfaceImplemented:
      parentSurfaceReadModelHandoff.parentSurfaceStatusReadModelParentSurfaceImplemented,
    parentSurfaceStatusReadModelParentSurfaceReadModelImplemented:
      parentSurfaceReadModelHandoff.parentSurfaceStatusReadModelParentSurfaceReadModelImplemented,
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
      'packages/parent-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff.ts',
    rules:
      'packages/parent-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff-rules.ts',
    test: 'packages/parent-domain/tests/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff.test.ts',
    harness:
      'scripts/test/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff-proof.mjs',
    evidence:
      'test-results/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff-proof/proof.json',
    appGameProofPack: `output/app-game-plan-proof/${proofSlug}`,
    appProofPack: `output/app-plan-proof/${proofSlug}`,
  },
  parentSurfaceReadModelHandoff,
};

assertProof(proof);
await writeJson(
  join(
    testOutputDir,
    'timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff.json'
  ),
  parentSurfaceReadModelHandoff
);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(appGameProofDir, proof, 'app-game WP100');
await writeProofPack(appProofDir, proof, 'app WP100');

console.log(
  'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff-proof-ok'
);
console.log(
  `evidence=${join(
    'test-results',
    'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff-proof',
    'proof.json'
  )}`
);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function parentSurfaceReadModelHandoffOptions(refs) {
  return {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    responseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffId:
      'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff-proof',
    generatedAt: timestamp,
    sourceContractRefs: [
      'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-handoff',
      'docs/expectations/app-game-evidence.md',
      'docs/expectations/enforcement.md',
    ],
    parentSurfaceReadModelProofRefs: [
      'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof',
    ],
    parentSurfaceReadModelRef:
      'future-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof',
  };
}

function summarize(parentSurfaceReadModelHandoff) {
  return {
    sourceResponseConsumerParentSurfaceStatusReadModelHandoffId:
      parentSurfaceReadModelHandoff.sourceResponseConsumerParentSurfaceStatusReadModelHandoffId,
    nativeAppRowCount: parentSurfaceReadModelHandoff.nativeAppRowCount,
    nativeGameRowCount: parentSurfaceReadModelHandoff.nativeGameRowCount,
    parentSurfaceReadModelProofRequiredCount: parentSurfaceReadModelHandoff.parentSurfaceReadModelProofRequiredCount,
    blockedBySourceFreshnessCount: parentSurfaceReadModelHandoff.blockedBySourceFreshnessCount,
    blockedByCompilerDecisionCount: parentSurfaceReadModelHandoff.blockedByCompilerDecisionCount,
  };
}

function assertProof(proof) {
  if (proof.summary.parentSurfaceReadModelProofRequiredCount < 1) {
    throw new Error('Expected at least one parent-surface read-model proof-required row');
  }
  if (proof.summary.blockedBySourceFreshnessCount < 1 || proof.summary.blockedByCompilerDecisionCount < 1) {
    throw new Error('Expected source-freshness and compiler-decision blocked rows');
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(
      'Expected WP100 proof to avoid service runtime, parent-surface implementation/rendering, portal UI, evaluator, timer, scheduler storage, audit log, rollback execution, adapter, child, platform, and raw-source claims'
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
