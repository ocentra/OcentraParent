import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofSlug = '88-source-gated-policy-preview-timer-service-readiness-protocol-handoff';
const testOutputDir = join(
  repoRoot,
  'test-results',
  'app-game-source-gated-policy-preview-timer-service-readiness-protocol-handoff-proof'
);
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', proofSlug);
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', proofSlug);
const timestamp = '2026-06-06T07:12:00Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

for (const path of [testOutputDir, appGameProofDir, appProofDir]) {
  await rm(path, { recursive: true, force: true });
  await mkdir(path, { recursive: true });
}
for (const path of [join(appGameProofDir, '06-ui-snapshots'), join(appProofDir, '06-ui-snapshots')]) {
  await mkdir(path, { recursive: true });
}

runNpm(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
runNpm([
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'app-game-source-gated-policy-preview-timer-service-readiness-protocol-handoff',
  'app-game-source-gated-policy-preview-timer-service-readiness-read-model',
]);

const protocolHandoffContract = await importDist(
  'app-game-source-gated-policy-preview-timer-service-readiness-protocol-handoff.js'
);
const refs = await importDist('reference-primitives.js');
const serviceReadinessReadModel = await readJson(
  join(
    repoRoot,
    'test-results',
    'app-game-source-gated-policy-preview-timer-service-readiness-read-model-proof',
    'timer-service-readiness-read-model.json'
  )
);
const protocolHandoff =
  protocolHandoffContract.buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoff(
    protocolHandoffOptions(refs),
    serviceReadinessReadModel
  );
const proof = {
  proofMode: 'app-game-source-gated-policy-preview-timer-service-readiness-protocol-handoff',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  stackedOn: {
    wp87Branch: 'codex/app-game-source-gated-policy-preview-timer-service-readiness-read-model',
    reason:
      'WP88 consumes WP87 service-readiness read-model rows and creates a parent-domain protocol handoff manifest while agent-protocol contracts, Rust protocol mirrors, service command registration, service event emission, read API implementation, portal rendering, timer runtime, durable scheduler/audit storage, rollback execution, adapter dispatch, child delivery, platform enforcement, and package exports are sequenced separately.',
  },
  summary: summarize(protocolHandoff),
  nonClaims: {
    agentProtocolContractImplemented: protocolHandoff.agentProtocolContractImplemented,
    rustProtocolMirrored: protocolHandoff.rustProtocolMirrored,
    serviceCommandRegistered: protocolHandoff.serviceCommandRegistered,
    serviceEventEmitted: protocolHandoff.serviceEventEmitted,
    serviceReadApiImplemented: protocolHandoff.serviceReadApiImplemented,
    portalUiRendered: protocolHandoff.portalUiRendered,
    policyEvaluatorRuntimeClaimed: protocolHandoff.policyEvaluatorRuntimeClaimed,
    timerRuntimeClaimed: protocolHandoff.timerRuntimeClaimed,
    timerScheduled: protocolHandoff.timerScheduled,
    schedulerPersistenceRuntimeClaimed: protocolHandoff.schedulerPersistenceRuntimeClaimed,
    durableSchedulerStorageClaimed: protocolHandoff.durableSchedulerStorageClaimed,
    auditRuntimeClaimed: protocolHandoff.auditRuntimeClaimed,
    durableAuditLogClaimed: protocolHandoff.durableAuditLogClaimed,
    rollbackRuntimeClaimed: protocolHandoff.rollbackRuntimeClaimed,
    rollbackExecutionClaimed: protocolHandoff.rollbackExecutionClaimed,
    adapterDispatchClaimed: protocolHandoff.adapterDispatchClaimed,
    childDeliveryClaimed: protocolHandoff.childDeliveryClaimed,
    platformEnforcementClaimed: protocolHandoff.platformEnforcementClaimed,
    rawPrivateSourceRowsIncluded: protocolHandoff.rawPrivateSourceRowsIncluded,
  },
  proofPaths: {
    source:
      'packages/parent-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-protocol-handoff.ts',
    rules:
      'packages/parent-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-protocol-handoff-rules.ts',
    test: 'packages/parent-domain/tests/app-game-source-gated-policy-preview-timer-service-readiness-protocol-handoff.test.ts',
    harness: 'scripts/test/app-game-source-gated-policy-preview-timer-service-readiness-protocol-handoff-proof.mjs',
    evidence:
      'test-results/app-game-source-gated-policy-preview-timer-service-readiness-protocol-handoff-proof/proof.json',
    appGameProofPack: `output/app-game-plan-proof/${proofSlug}`,
    appProofPack: `output/app-plan-proof/${proofSlug}`,
  },
  protocolHandoff,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'timer-service-readiness-protocol-handoff.json'), protocolHandoff);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(appGameProofDir, proof, 'app-game WP88');
await writeProofPack(appProofDir, proof, 'app WP88');

console.log('app-game-source-gated-policy-preview-timer-service-readiness-protocol-handoff-proof-ok');
console.log(
  `evidence=${join('test-results', 'app-game-source-gated-policy-preview-timer-service-readiness-protocol-handoff-proof', 'proof.json')}`
);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function protocolHandoffOptions(refs) {
  return {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    handoffId: 'app-game-source-gated-policy-preview-timer-service-readiness-protocol-handoff-proof',
    generatedAt: timestamp,
    sourceContractRefs: [
      'app-game-source-gated-policy-preview-timer-service-readiness-read-model',
      'docs/expectations/app-game-evidence.md',
      'packages/agent-protocol-domain',
      'crates/agent-protocol',
      'crates/agent-service',
    ],
    protocolCommandContractProofRef: 'future-agent-protocol-command-contract-proof',
    protocolEventContractProofRef: 'future-agent-protocol-event-contract-proof',
    rustProtocolMirrorProofRef: 'future-rust-protocol-mirror-proof',
    serviceHandlerProofRef: 'future-service-handler-proof',
  };
}

function summarize(handoff) {
  return {
    sourceServiceReadinessReadModelId: handoff.sourceServiceReadinessReadModelId,
    sourceServiceReadinessHandoffId: handoff.sourceServiceReadinessHandoffId,
    nativeAppRowCount: handoff.nativeAppRowCount,
    nativeGameRowCount: handoff.nativeGameRowCount,
    protocolProofRequiredCount: handoff.protocolProofRequiredCount,
    blockedBySourceFreshnessCount: handoff.blockedBySourceFreshnessCount,
    blockedByCompilerDecisionCount: handoff.blockedByCompilerDecisionCount,
  };
}

function assertProof(proof) {
  if (proof.summary.protocolProofRequiredCount < 1) {
    throw new Error('Expected at least one protocol proof-required row');
  }
  if (proof.summary.blockedBySourceFreshnessCount < 1 || proof.summary.blockedByCompilerDecisionCount < 1) {
    throw new Error('Expected source-freshness and compiler-decision blocked rows');
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(
      'Expected WP88 proof to avoid protocol implementation, service runtime, read API implementation, portal UI, evaluator, timer, scheduler storage, audit log, rollback execution, adapter, child, platform, and raw-source claims'
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
