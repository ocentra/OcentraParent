import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofSlug = '108-timer-service-read-api-response-consumer-parent-surface';
const testOutputDir = join(
  repoRoot,
  'test-results',
  'app-game-timer-service-read-api-response-consumer-parent-surface-handoff-proof'
);
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', proofSlug);
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', proofSlug);
const timestamp = '2026-06-07T13:55:00Z';
const commands = [];
const buildWorkspaces = [
  ['@ocentra-parent/schema-domain', 'schema-domain'],
  ['@ocentra-parent/logging-domain', 'logging-domain'],
  ['@ocentra-parent/parent-domain', 'parent-domain'],
];
const proofStatusPaths = [
  'scripts/test/app-game-timer-service-read-api-response-consumer-parent-surface-handoff-proof.mjs',
  'output/app-game-plan-proof/108-timer-service-read-api-response-consumer-parent-surface',
  'output/app-plan-proof/108-timer-service-read-api-response-consumer-parent-surface',
  'test-results/app-game-timer-service-read-api-response-consumer-parent-surface-handoff-proof',
];
const initialGitStatusShort = filteredGitStatusShort();

for (const path of [testOutputDir, appGameProofDir, appProofDir]) {
  await rm(path, { recursive: true, force: true });
  await mkdir(path, { recursive: true });
}

await buildRequiredWorkspaces();
runNpm([
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'app-game-timer-service-read-api-response-consumer-parent-surface-handoff',
  'app-game-timer-service-read-api-response-consumer-handoff',
]);

const contract = await importDist('app-game-timer-service-read-api-response-consumer-parent-surface-handoff.js');
const responseConsumerContract = await importDist('app-game-timer-service-read-api-response-consumer-handoff.js');
const refs = await importDist('reference-primitives.js');
const sourceResponseConsumerHandoff =
  responseConsumerContract.AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoffSchema.parse(
    await readJson(
      join(repoRoot, 'test-results', 'app-game-timer-service-read-api-response-consumer-handoff-proof', 'handoff.json')
    )
  );
const parentSurfaceHandoff = contract.buildAppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoff(
  parentSurfaceHandoffOptions(refs),
  sourceResponseConsumerHandoff
);
const proof = {
  proofMode: 'app-game-timer-service-read-api-response-consumer-parent-surface-handoff',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  stackedOn: {
    wp107Branch: 'codex/app-game-timer-service-read-api-response-consumer-handoff-wp107',
    reason:
      'WP108 consumes WP107 parent-domain service read API response consumer handoff rows and records the future parent-surface proof needed before parent-surface rendering, portal rendering, read-model runtime, protocol, adapters, child delivery, platform enforcement, or raw source rows are claimed.',
  },
  summary: summarize(parentSurfaceHandoff),
  nonClaims: pickNonClaims(parentSurfaceHandoff),
  proofPaths: {
    source: 'packages/parent-domain/src/app-game-timer-service-read-api-response-consumer-parent-surface-handoff.ts',
    rules:
      'packages/parent-domain/src/app-game-timer-service-read-api-response-consumer-parent-surface-handoff-rules.ts',
    test: 'packages/parent-domain/tests/app-game-timer-service-read-api-response-consumer-parent-surface-handoff.test.ts',
    harness: 'scripts/test/app-game-timer-service-read-api-response-consumer-parent-surface-handoff-proof.mjs',
    evidence: 'test-results/app-game-timer-service-read-api-response-consumer-parent-surface-handoff-proof/proof.json',
    appGameProofPack: `output/app-game-plan-proof/${proofSlug}`,
    appProofPack: `output/app-plan-proof/${proofSlug}`,
  },
  parentSurfaceHandoff,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'handoff.json'), parentSurfaceHandoff);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(appGameProofDir, proof, 'app-game WP108');
await writeProofPack(appProofDir, proof, 'app WP108');

console.log('app-game-timer-service-read-api-response-consumer-parent-surface-handoff-proof-ok');
console.log(
  `evidence=${join(
    'test-results',
    'app-game-timer-service-read-api-response-consumer-parent-surface-handoff-proof',
    'proof.json'
  )}`
);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function parentSurfaceHandoffOptions(refs) {
  return {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    parentSurfaceHandoffId: 'app-game-timer-service-read-api-response-consumer-parent-surface-handoff-proof',
    generatedAt: timestamp,
    sourceContractRefs: [
      'app-game-timer-service-read-api-response-consumer-handoff',
      'docs/expectations/app-game-evidence.md',
      'docs/expectations/enforcement.md',
    ],
    parentSurfaceProofRefs: ['future-app-game-timer-service-read-api-response-consumer-parent-surface-proof'],
  };
}

function summarize(parentSurfaceHandoff) {
  return {
    sourceResponseConsumerHandoffId: parentSurfaceHandoff.sourceResponseConsumerHandoffId,
    nativeAppRowCount: parentSurfaceHandoff.nativeAppRowCount,
    nativeGameRowCount: parentSurfaceHandoff.nativeGameRowCount,
    parentSurfaceProofRequiredCount: parentSurfaceHandoff.parentSurfaceProofRequiredCount,
    blockedBySourceFreshnessCount: parentSurfaceHandoff.blockedBySourceFreshnessCount,
    blockedByCompilerDecisionCount: parentSurfaceHandoff.blockedByCompilerDecisionCount,
  };
}

function pickNonClaims(parentSurfaceHandoff) {
  return {
    serviceCommandRegistered: parentSurfaceHandoff.serviceCommandRegistered,
    serviceHandlerImplemented: parentSurfaceHandoff.serviceHandlerImplemented,
    serviceReadModelRuntimeEmitted: parentSurfaceHandoff.serviceReadModelRuntimeEmitted,
    serviceEventRuntimeEmitted: parentSurfaceHandoff.serviceEventRuntimeEmitted,
    serviceEventEmitted: parentSurfaceHandoff.serviceEventEmitted,
    serviceReadApiImplemented: parentSurfaceHandoff.serviceReadApiImplemented,
    serviceReadApiResponseImplemented: parentSurfaceHandoff.serviceReadApiResponseImplemented,
    serviceReadApiResponseConsumerImplemented: parentSurfaceHandoff.serviceReadApiResponseConsumerImplemented,
    agentProtocolImplemented: parentSurfaceHandoff.agentProtocolImplemented,
    rustProtocolMirrored: parentSurfaceHandoff.rustProtocolMirrored,
    portalUiRendered: parentSurfaceHandoff.portalUiRendered,
    portalResponseConsumerRendered: parentSurfaceHandoff.portalResponseConsumerRendered,
    parentSurfaceRendered: parentSurfaceHandoff.parentSurfaceRendered,
    parentSurfaceReadModelRuntimeImplemented: parentSurfaceHandoff.parentSurfaceReadModelRuntimeImplemented,
    parentSurfaceReadModelPersisted: parentSurfaceHandoff.parentSurfaceReadModelPersisted,
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
  };
}

function assertProof(proof) {
  if (proof.summary.parentSurfaceProofRequiredCount < 1) {
    throw new Error('Expected at least one parent-surface-proof-required row');
  }
  if (proof.summary.blockedBySourceFreshnessCount < 1 || proof.summary.blockedByCompilerDecisionCount < 1) {
    throw new Error('Expected source-freshness and compiler-decision blocked rows');
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(
      'Expected WP108 proof to avoid service read-model runtime, read API runtime, response consumer implementation, protocol, read-model persistence, parent-surface rendering, portal UI, evaluator, timer, scheduler storage, audit log, rollback execution, adapter, child, platform, and raw-source claims'
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
    runNpm(['run', 'build', '--workspace', workspace]);
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
      return !proofStatusPaths.some((proofPath) => path === proofPath || path.startsWith(`${proofPath}/`));
    })
    .join('\n');
}

function runNpm(args, ...rest) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return run(command, commandArgs, ...rest);
}
