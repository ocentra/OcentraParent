import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofSlug = '105-timer-service-read-api';
const testOutputDir = join(repoRoot, 'test-results', 'app-game-timer-service-read-api-handoff-proof');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', proofSlug);
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', proofSlug);
const timestamp = '2026-06-06T12:20:00Z';
const commands = [];
const buildWorkspaces = [
  ['@ocentra-parent/schema-domain', 'schema-domain'],
  ['@ocentra-parent/logging-domain', 'logging-domain'],
  ['@ocentra-parent/parent-domain', 'parent-domain'],
];
const proofChainStatusPaths = [
  'scripts/test/app-game-timer-service-event-handoff-proof.mjs',
  'scripts/test/app-game-timer-service-read-api-handoff-proof.mjs',
  'scripts/test/app-game-timer-service-read-api-response-handoff-proof.mjs',
  'scripts/test/app-game-timer-service-read-api-response-consumer-handoff-proof.mjs',
  'output/app-game-plan-proof/104-timer-service-event',
  'output/app-game-plan-proof/105-timer-service-read-api',
  'output/app-game-plan-proof/106-timer-service-read-api-response',
  'output/app-game-plan-proof/107-timer-service-read-api-response-consumer',
  'output/app-plan-proof/104-timer-service-event',
  'output/app-plan-proof/105-timer-service-read-api',
  'output/app-plan-proof/106-timer-service-read-api-response',
  'output/app-plan-proof/107-timer-service-read-api-response-consumer',
  'test-results/app-game-timer-service-event-handoff-proof',
  'test-results/app-game-timer-service-read-api-handoff-proof',
  'test-results/app-game-timer-service-read-api-response-handoff-proof',
  'test-results/app-game-timer-service-read-api-response-consumer-handoff-proof',
];
const initialGitStatusShort = filteredGitStatusShort();

for (const path of [testOutputDir, appGameProofDir, appProofDir]) {
  await rm(path, { recursive: true, force: true });
  await mkdir(path, { recursive: true });
}

await buildRequiredWorkspaces();
run('cmd', [
  '/c',
  'npm',
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'app-game-timer-service-read-api-handoff',
  'app-game-timer-service-event-handoff',
]);

const contract = await importDist('app-game-timer-service-read-api-handoff.js');
const serviceEventHandoffContract = await importDist('app-game-timer-service-event-handoff.js');
const refs = await importDist('reference-primitives.js');
const sourceServiceEventHandoff =
  serviceEventHandoffContract.AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffSchema.parse(
    await readJson(join(repoRoot, 'test-results', 'app-game-timer-service-event-handoff-proof', 'handoff.json'))
  );
const serviceReadApiHandoff =
  contract.buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoff(
    serviceReadApiHandoffOptions(refs),
    sourceServiceEventHandoff
  );
const proof = {
  proofMode: 'app-game-timer-service-read-api-handoff',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  stackedOn: {
    wp104Branch: 'codex/app-game-timer-service-event-handoff-wp104',
    reason:
      'WP105 consumes WP104 parent-domain service event handoff rows and records the future service read API proof needed before service read API implementation, responses, response consumers, protocol, portal rendering, adapters, child delivery, platform enforcement, or raw source rows are claimed.',
  },
  summary: summarize(serviceReadApiHandoff),
  nonClaims: pickNonClaims(serviceReadApiHandoff),
  proofPaths: {
    source: 'packages/parent-domain/src/app-game-timer-service-read-api-handoff.ts',
    rules: 'packages/parent-domain/src/app-game-timer-service-read-api-handoff-rules.ts',
    test: 'packages/parent-domain/tests/app-game-timer-service-read-api-handoff.test.ts',
    harness: 'scripts/test/app-game-timer-service-read-api-handoff-proof.mjs',
    evidence: 'test-results/app-game-timer-service-read-api-handoff-proof/proof.json',
    appGameProofPack: `output/app-game-plan-proof/${proofSlug}`,
    appProofPack: `output/app-plan-proof/${proofSlug}`,
  },
  serviceReadApiHandoff,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'handoff.json'), serviceReadApiHandoff);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(appGameProofDir, proof, 'app-game WP105');
await writeProofPack(appProofDir, proof, 'app WP105');

console.log('app-game-timer-service-read-api-handoff-proof-ok');
console.log(`evidence=${join('test-results', 'app-game-timer-service-read-api-handoff-proof', 'proof.json')}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function serviceReadApiHandoffOptions(refs) {
  return {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    parentSurfaceReadModelServiceReadApiHandoffId: 'app-game-timer-service-read-api-handoff-proof',
    generatedAt: timestamp,
    sourceContractRefs: [
      'app-game-timer-service-event-handoff',
      'docs/expectations/app-game-evidence.md',
      'docs/expectations/enforcement.md',
    ],
    serviceReadApiProofRefs: [
      'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-read-api-proof',
    ],
  };
}

function summarize(serviceReadApiHandoff) {
  return {
    sourceParentSurfaceReadModelServiceEventHandoffId:
      serviceReadApiHandoff.sourceParentSurfaceReadModelServiceEventHandoffId,
    nativeAppRowCount: serviceReadApiHandoff.nativeAppRowCount,
    nativeGameRowCount: serviceReadApiHandoff.nativeGameRowCount,
    serviceReadApiProofRequiredCount: serviceReadApiHandoff.serviceReadApiProofRequiredCount,
    blockedBySourceFreshnessCount: serviceReadApiHandoff.blockedBySourceFreshnessCount,
    blockedByCompilerDecisionCount: serviceReadApiHandoff.blockedByCompilerDecisionCount,
  };
}

function pickNonClaims(serviceReadApiHandoff) {
  return {
    serviceCommandRegistered: serviceReadApiHandoff.serviceCommandRegistered,
    serviceHandlerImplemented: serviceReadApiHandoff.serviceHandlerImplemented,
    serviceReadModelRuntimeEmitted: serviceReadApiHandoff.serviceReadModelRuntimeEmitted,
    serviceEventRuntimeEmitted: serviceReadApiHandoff.serviceEventRuntimeEmitted,
    serviceEventEmitted: serviceReadApiHandoff.serviceEventEmitted,
    serviceReadApiImplemented: serviceReadApiHandoff.serviceReadApiImplemented,
    serviceReadApiResponseImplemented: serviceReadApiHandoff.serviceReadApiResponseImplemented,
    serviceReadApiResponseConsumerImplemented: serviceReadApiHandoff.serviceReadApiResponseConsumerImplemented,
    agentProtocolImplemented: serviceReadApiHandoff.agentProtocolImplemented,
    rustProtocolMirrored: serviceReadApiHandoff.rustProtocolMirrored,
    portalUiRendered: serviceReadApiHandoff.portalUiRendered,
    portalResponseConsumerRendered: serviceReadApiHandoff.portalResponseConsumerRendered,
    parentSurfaceRendered: serviceReadApiHandoff.parentSurfaceRendered,
    parentSurfaceReadModelRuntimeImplemented: serviceReadApiHandoff.parentSurfaceReadModelRuntimeImplemented,
    parentSurfaceReadModelPersisted: serviceReadApiHandoff.parentSurfaceReadModelPersisted,
    policyEvaluatorRuntimeClaimed: serviceReadApiHandoff.policyEvaluatorRuntimeClaimed,
    timerRuntimeClaimed: serviceReadApiHandoff.timerRuntimeClaimed,
    timerScheduled: serviceReadApiHandoff.timerScheduled,
    schedulerPersistenceRuntimeClaimed: serviceReadApiHandoff.schedulerPersistenceRuntimeClaimed,
    durableSchedulerStorageClaimed: serviceReadApiHandoff.durableSchedulerStorageClaimed,
    auditRuntimeClaimed: serviceReadApiHandoff.auditRuntimeClaimed,
    durableAuditLogClaimed: serviceReadApiHandoff.durableAuditLogClaimed,
    rollbackRuntimeClaimed: serviceReadApiHandoff.rollbackRuntimeClaimed,
    rollbackExecutionClaimed: serviceReadApiHandoff.rollbackExecutionClaimed,
    adapterDispatchClaimed: serviceReadApiHandoff.adapterDispatchClaimed,
    childDeliveryClaimed: serviceReadApiHandoff.childDeliveryClaimed,
    platformEnforcementClaimed: serviceReadApiHandoff.platformEnforcementClaimed,
    rawPrivateSourceRowsIncluded: serviceReadApiHandoff.rawPrivateSourceRowsIncluded,
  };
}

function assertProof(proof) {
  if (proof.summary.serviceReadApiProofRequiredCount < 1) {
    throw new Error('Expected at least one service-read-api-proof-required row');
  }
  if (proof.summary.blockedBySourceFreshnessCount < 1 || proof.summary.blockedByCompilerDecisionCount < 1) {
    throw new Error('Expected source-freshness and compiler-decision blocked rows');
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(
      'Expected WP105 proof to avoid service read-model runtime, service event runtime, service read API implementation, service read API responses, protocol, read-model persistence, portal UI, evaluator, timer, scheduler storage, audit log, rollback execution, adapter, child, platform, and raw-source claims'
    );
  }
}

async function readJson(path) {
  return JSON.parse(await readFile(path, 'utf8'));
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}

async function buildRequiredWorkspaces() {
  for (const [workspace, packageDir] of buildWorkspaces) {
    await rm(join(repoRoot, 'packages', packageDir, 'tsconfig.tsbuildinfo'), { force: true });
    run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', workspace]);
  }
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

function filteredGitStatusShort() {
  return gitOutput(['status', '--short'])
    .split('\n')
    .filter(Boolean)
    .filter((line) => {
      const path = line.replace(/^[ MADRCU?!]{1,2}\s+/, '').replaceAll('\\', '/');
      return !proofChainStatusPaths.some((proofPath) => path === proofPath || path.startsWith(`${proofPath}/`));
    })
    .join('\n');
}
