import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofSlug = '89-source-gated-policy-preview-timer-service-readiness-protocol-read-model';
const testOutputDir = join(
  repoRoot,
  'test-results',
  'app-game-source-gated-policy-preview-timer-service-readiness-protocol-read-model-proof'
);
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', proofSlug);
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', proofSlug);
const timestamp = '2026-06-06T07:23:00Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

for (const path of [testOutputDir, appGameProofDir, appProofDir]) {
  await rm(path, { recursive: true, force: true });
  await mkdir(path, { recursive: true });
}
for (const path of [join(appGameProofDir, '06-ui-snapshots'), join(appProofDir, '06-ui-snapshots')]) {
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
  'app-game-source-gated-policy-preview-timer-service-readiness-protocol-read-model',
  'app-game-source-gated-policy-preview-timer-service-readiness-protocol-handoff',
]);

const protocolReadModelContract = await importDist(
  'app-game-source-gated-policy-preview-timer-service-readiness-protocol-read-model.js'
);
const refs = await importDist('reference-primitives.js');
const protocolHandoff = await readJson(
  join(
    repoRoot,
    'test-results',
    'app-game-source-gated-policy-preview-timer-service-readiness-protocol-handoff-proof',
    'timer-service-readiness-protocol-handoff.json'
  )
);
const protocolReadModel =
  protocolReadModelContract.buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModel(
    protocolReadModelOptions(refs),
    protocolHandoff
  );
const proof = {
  proofMode: 'app-game-source-gated-policy-preview-timer-service-readiness-protocol-read-model',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  stackedOn: {
    wp88Branch: 'codex/app-game-source-gated-policy-preview-timer-service-readiness-protocol-handoff',
    reason:
      'WP89 consumes WP88 protocol handoff rows and creates a parent-domain protocol readiness read model while agent-protocol contracts, Rust protocol mirrors, service command registration, service event emission, service read API implementation, portal rendering, timer runtime, durable scheduler/audit storage, rollback execution, adapter dispatch, child delivery, platform enforcement, and package exports are sequenced separately.',
  },
  summary: summarize(protocolReadModel),
  nonClaims: {
    agentProtocolContractImplemented: protocolReadModel.agentProtocolContractImplemented,
    rustProtocolMirrored: protocolReadModel.rustProtocolMirrored,
    serviceCommandRegistered: protocolReadModel.serviceCommandRegistered,
    serviceEventEmitted: protocolReadModel.serviceEventEmitted,
    serviceReadApiImplemented: protocolReadModel.serviceReadApiImplemented,
    serviceReadModelEventEmitted: protocolReadModel.serviceReadModelEventEmitted,
    portalUiRendered: protocolReadModel.portalUiRendered,
    policyEvaluatorRuntimeClaimed: protocolReadModel.policyEvaluatorRuntimeClaimed,
    timerRuntimeClaimed: protocolReadModel.timerRuntimeClaimed,
    timerScheduled: protocolReadModel.timerScheduled,
    schedulerPersistenceRuntimeClaimed: protocolReadModel.schedulerPersistenceRuntimeClaimed,
    durableSchedulerStorageClaimed: protocolReadModel.durableSchedulerStorageClaimed,
    auditRuntimeClaimed: protocolReadModel.auditRuntimeClaimed,
    durableAuditLogClaimed: protocolReadModel.durableAuditLogClaimed,
    rollbackRuntimeClaimed: protocolReadModel.rollbackRuntimeClaimed,
    rollbackExecutionClaimed: protocolReadModel.rollbackExecutionClaimed,
    adapterDispatchClaimed: protocolReadModel.adapterDispatchClaimed,
    childDeliveryClaimed: protocolReadModel.childDeliveryClaimed,
    platformEnforcementClaimed: protocolReadModel.platformEnforcementClaimed,
    rawPrivateSourceRowsIncluded: protocolReadModel.rawPrivateSourceRowsIncluded,
  },
  proofPaths: {
    source:
      'packages/parent-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-protocol-read-model.ts',
    rules:
      'packages/parent-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-protocol-read-model-rules.ts',
    test: 'packages/parent-domain/tests/app-game-source-gated-policy-preview-timer-service-readiness-protocol-read-model.test.ts',
    harness: 'scripts/test/app-game-source-gated-policy-preview-timer-service-readiness-protocol-read-model-proof.mjs',
    evidence:
      'test-results/app-game-source-gated-policy-preview-timer-service-readiness-protocol-read-model-proof/proof.json',
    appGameProofPack: `output/app-game-plan-proof/${proofSlug}`,
    appProofPack: `output/app-plan-proof/${proofSlug}`,
  },
  protocolReadModel,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'timer-service-readiness-protocol-read-model.json'), protocolReadModel);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(appGameProofDir, proof, 'app-game WP89');
await writeProofPack(appProofDir, proof, 'app WP89');

console.log('app-game-source-gated-policy-preview-timer-service-readiness-protocol-read-model-proof-ok');
console.log(
  `evidence=${join('test-results', 'app-game-source-gated-policy-preview-timer-service-readiness-protocol-read-model-proof', 'proof.json')}`
);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function protocolReadModelOptions(refs) {
  return {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    readModelId: 'app-game-source-gated-policy-preview-timer-service-readiness-protocol-read-model-proof',
    generatedAt: timestamp,
    sourceContractRefs: [
      'app-game-source-gated-policy-preview-timer-service-readiness-protocol-handoff',
      'docs/expectations/app-game-evidence.md',
      'packages/agent-protocol-domain',
      'crates/agent-protocol',
      'crates/agent-service',
    ],
    protocolSummaryRef: 'future-service-readiness-protocol-read-model-summary-proof',
  };
}

function summarize(readModel) {
  return {
    sourceProtocolHandoffId: readModel.sourceProtocolHandoffId,
    nativeAppRowCount: readModel.nativeAppRowCount,
    nativeGameRowCount: readModel.nativeGameRowCount,
    protocolReadModelProofRequiredCount: readModel.protocolReadModelProofRequiredCount,
    blockedBySourceFreshnessCount: readModel.blockedBySourceFreshnessCount,
    blockedByCompilerDecisionCount: readModel.blockedByCompilerDecisionCount,
  };
}

function assertProof(proof) {
  if (proof.summary.protocolReadModelProofRequiredCount < 1) {
    throw new Error('Expected at least one protocol read-model proof-required row');
  }
  if (proof.summary.blockedBySourceFreshnessCount < 1 || proof.summary.blockedByCompilerDecisionCount < 1) {
    throw new Error('Expected source-freshness and compiler-decision blocked rows');
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(
      'Expected WP89 proof to avoid protocol implementation, service runtime, read API implementation, read-model events, portal UI, evaluator, timer, scheduler storage, audit log, rollback execution, adapter, child, platform, and raw-source claims'
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
