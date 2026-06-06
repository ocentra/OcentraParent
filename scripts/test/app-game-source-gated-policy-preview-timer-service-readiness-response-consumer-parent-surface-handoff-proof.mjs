import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofSlug = '95-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-handoff';
const testOutputDir = join(
  repoRoot,
  'test-results',
  'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-handoff-proof'
);
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', proofSlug);
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', proofSlug);
const timestamp = '2026-06-06T08:56:00Z';
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
  'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-handoff',
  'app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff',
]);

const consumerContract = await importDist(
  'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-handoff.js'
);
const refs = await importDist('reference-primitives.js');
const responseHandoff = await readJson(
  join(
    repoRoot,
    'test-results',
    'app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff-proof',
    'timer-service-readiness-read-api-response-consumer-handoff.json'
  )
);
const ResponseConsumerParentSurfaceHandoff =
  consumerContract.buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoff(
    ResponseConsumerParentSurfaceHandoffOptions(refs),
    responseHandoff
  );
const proof = {
  proofMode: 'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-handoff',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  stackedOn: {
    wp94Branch: 'codex/app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff',
    reason:
      'WP95 consumes WP94 response-consumer handoff rows and creates a parent-domain parent-surface handoff while actual parent-surface rendering, portal rendering, service consumer implementation, service command registration, service handler implementation, service event emission, service read API implementation, timer runtime, durable scheduler/audit storage, rollback execution, adapter dispatch, child delivery, platform enforcement, and package exports are sequenced separately.',
  },
  summary: summarize(ResponseConsumerParentSurfaceHandoff),
  nonClaims: {
    serviceCommandRegistered: ResponseConsumerParentSurfaceHandoff.serviceCommandRegistered,
    serviceHandlerImplemented: ResponseConsumerParentSurfaceHandoff.serviceHandlerImplemented,
    serviceReadApiImplemented: ResponseConsumerParentSurfaceHandoff.serviceReadApiImplemented,
    serviceReadApiResponseImplemented: ResponseConsumerParentSurfaceHandoff.serviceReadApiResponseImplemented,
    serviceReadApiResponseConsumerImplemented:
      ResponseConsumerParentSurfaceHandoff.serviceReadApiResponseConsumerImplemented,
    serviceEventEmitted: ResponseConsumerParentSurfaceHandoff.serviceEventEmitted,
    agentProtocolImplemented: ResponseConsumerParentSurfaceHandoff.agentProtocolImplemented,
    rustProtocolMirrored: ResponseConsumerParentSurfaceHandoff.rustProtocolMirrored,
    portalUiRendered: ResponseConsumerParentSurfaceHandoff.portalUiRendered,
    portalResponseConsumerRendered: ResponseConsumerParentSurfaceHandoff.portalResponseConsumerRendered,
    parentSurfaceRendered: ResponseConsumerParentSurfaceHandoff.parentSurfaceRendered,
    policyEvaluatorRuntimeClaimed: ResponseConsumerParentSurfaceHandoff.policyEvaluatorRuntimeClaimed,
    timerRuntimeClaimed: ResponseConsumerParentSurfaceHandoff.timerRuntimeClaimed,
    timerScheduled: ResponseConsumerParentSurfaceHandoff.timerScheduled,
    schedulerPersistenceRuntimeClaimed: ResponseConsumerParentSurfaceHandoff.schedulerPersistenceRuntimeClaimed,
    durableSchedulerStorageClaimed: ResponseConsumerParentSurfaceHandoff.durableSchedulerStorageClaimed,
    auditRuntimeClaimed: ResponseConsumerParentSurfaceHandoff.auditRuntimeClaimed,
    durableAuditLogClaimed: ResponseConsumerParentSurfaceHandoff.durableAuditLogClaimed,
    rollbackRuntimeClaimed: ResponseConsumerParentSurfaceHandoff.rollbackRuntimeClaimed,
    rollbackExecutionClaimed: ResponseConsumerParentSurfaceHandoff.rollbackExecutionClaimed,
    adapterDispatchClaimed: ResponseConsumerParentSurfaceHandoff.adapterDispatchClaimed,
    childDeliveryClaimed: ResponseConsumerParentSurfaceHandoff.childDeliveryClaimed,
    platformEnforcementClaimed: ResponseConsumerParentSurfaceHandoff.platformEnforcementClaimed,
    rawPrivateSourceRowsIncluded: ResponseConsumerParentSurfaceHandoff.rawPrivateSourceRowsIncluded,
  },
  proofPaths: {
    source:
      'packages/parent-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-handoff.ts',
    rules:
      'packages/parent-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-handoff-rules.ts',
    test: 'packages/parent-domain/tests/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-handoff.test.ts',
    harness:
      'scripts/test/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-handoff-proof.mjs',
    evidence:
      'test-results/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-handoff-proof/proof.json',
    appGameProofPack: `output/app-game-plan-proof/${proofSlug}`,
    appProofPack: `output/app-plan-proof/${proofSlug}`,
  },
  ResponseConsumerParentSurfaceHandoff,
};

assertProof(proof);
await writeJson(
  join(testOutputDir, 'timer-service-readiness-response-consumer-parent-surface-handoff.json'),
  ResponseConsumerParentSurfaceHandoff
);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(appGameProofDir, proof, 'app-game WP95');
await writeProofPack(appProofDir, proof, 'app WP95');

console.log(
  'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-handoff-proof-ok'
);
console.log(
  `evidence=${join(
    'test-results',
    'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-handoff-proof',
    'proof.json'
  )}`
);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function ResponseConsumerParentSurfaceHandoffOptions(refs) {
  return {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    responseConsumerParentSurfaceHandoffId:
      'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-handoff-proof',
    generatedAt: timestamp,
    sourceContractRefs: [
      'app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff',
      'docs/expectations/app-game-evidence.md',
      'docs/expectations/enforcement.md',
    ],
    parentSurfaceProofRefs: ['future-app-game-timer-service-readiness-response-consumer-parent-surface-proof'],
    parentSurfaceSummaryRef: 'future-service-readiness-response-consumer-parent-surface-handoff-summary-proof',
  };
}

function summarize(ResponseConsumerParentSurfaceHandoff) {
  return {
    sourceReadApiResponseConsumerHandoffId: ResponseConsumerParentSurfaceHandoff.sourceReadApiResponseConsumerHandoffId,
    nativeAppRowCount: ResponseConsumerParentSurfaceHandoff.nativeAppRowCount,
    nativeGameRowCount: ResponseConsumerParentSurfaceHandoff.nativeGameRowCount,
    parentSurfaceProofRequiredCount: ResponseConsumerParentSurfaceHandoff.parentSurfaceProofRequiredCount,
    blockedBySourceFreshnessCount: ResponseConsumerParentSurfaceHandoff.blockedBySourceFreshnessCount,
    blockedByCompilerDecisionCount: ResponseConsumerParentSurfaceHandoff.blockedByCompilerDecisionCount,
  };
}

function assertProof(proof) {
  if (proof.summary.parentSurfaceProofRequiredCount < 1) {
    throw new Error('Expected at least one response consumer parent-surface proof-required row');
  }
  if (proof.summary.blockedBySourceFreshnessCount < 1 || proof.summary.blockedByCompilerDecisionCount < 1) {
    throw new Error('Expected source-freshness and compiler-decision blocked rows');
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(
      'Expected WP95 proof to avoid service runtime, response/consumer implementation, portal UI, evaluator, timer, scheduler storage, audit log, rollback execution, adapter, child, platform, and raw-source claims'
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
