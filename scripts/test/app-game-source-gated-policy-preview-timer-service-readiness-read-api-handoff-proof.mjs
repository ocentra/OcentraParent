import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofSlug = '92-source-gated-policy-preview-timer-service-readiness-read-api-handoff';
const testOutputDir = join(
  repoRoot,
  'test-results',
  'app-game-source-gated-policy-preview-timer-service-readiness-read-api-handoff-proof'
);
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', proofSlug);
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', proofSlug);
const timestamp = '2026-06-06T07:58:00Z';
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
  'app-game-source-gated-policy-preview-timer-service-readiness-read-api-handoff',
  'app-game-source-gated-policy-preview-timer-service-readiness-service-handler-handoff',
]);

const serviceReadApiContract = await importDist(
  'app-game-source-gated-policy-preview-timer-service-readiness-read-api-handoff.js'
);
const refs = await importDist('reference-primitives.js');
const serviceHandlerHandoff = await readJson(
  join(
    repoRoot,
    'test-results',
    'app-game-source-gated-policy-preview-timer-service-readiness-service-handler-handoff-proof',
    'timer-service-readiness-service-handler-handoff.json'
  )
);
const serviceReadApiHandoff =
  serviceReadApiContract.buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoff(
    serviceReadApiHandoffOptions(refs),
    serviceHandlerHandoff
  );
const proof = {
  proofMode: 'app-game-source-gated-policy-preview-timer-service-readiness-read-api-handoff',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  stackedOn: {
    wp91Branch: 'codex/app-game-source-gated-policy-preview-timer-service-readiness-service-handler-handoff',
    reason:
      'WP92 consumes WP91 service handler handoff rows and creates a parent-domain service read API handoff while actual service command registration, service handler implementation, service event emission, service read API implementation, portal rendering, timer runtime, durable scheduler/audit storage, rollback execution, adapter dispatch, child delivery, platform enforcement, and package exports are sequenced separately.',
  },
  summary: summarize(serviceReadApiHandoff),
  nonClaims: {
    serviceCommandRegistered: serviceReadApiHandoff.serviceCommandRegistered,
    serviceHandlerImplemented: serviceReadApiHandoff.serviceHandlerImplemented,
    serviceReadApiImplemented: serviceReadApiHandoff.serviceReadApiImplemented,
    serviceEventEmitted: serviceReadApiHandoff.serviceEventEmitted,
    agentProtocolImplemented: serviceReadApiHandoff.agentProtocolImplemented,
    rustProtocolMirrored: serviceReadApiHandoff.rustProtocolMirrored,
    portalUiRendered: serviceReadApiHandoff.portalUiRendered,
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
  },
  proofPaths: {
    source:
      'packages/parent-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-read-api-handoff.ts',
    rules:
      'packages/parent-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-read-api-handoff-rules.ts',
    test: 'packages/parent-domain/tests/app-game-source-gated-policy-preview-timer-service-readiness-read-api-handoff.test.ts',
    harness: 'scripts/test/app-game-source-gated-policy-preview-timer-service-readiness-read-api-handoff-proof.mjs',
    evidence:
      'test-results/app-game-source-gated-policy-preview-timer-service-readiness-read-api-handoff-proof/proof.json',
    appGameProofPack: `output/app-game-plan-proof/${proofSlug}`,
    appProofPack: `output/app-plan-proof/${proofSlug}`,
  },
  serviceReadApiHandoff,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'timer-service-readiness-read-api-handoff.json'), serviceReadApiHandoff);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(appGameProofDir, proof, 'app-game WP92');
await writeProofPack(appProofDir, proof, 'app WP92');

console.log('app-game-source-gated-policy-preview-timer-service-readiness-read-api-handoff-proof-ok');
console.log(
  `evidence=${join(
    'test-results',
    'app-game-source-gated-policy-preview-timer-service-readiness-read-api-handoff-proof',
    'proof.json'
  )}`
);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function serviceReadApiHandoffOptions(refs) {
  return {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    serviceReadApiHandoffId: 'app-game-source-gated-policy-preview-timer-service-readiness-read-api-handoff-proof',
    generatedAt: timestamp,
    sourceContractRefs: [
      'app-game-source-gated-policy-preview-timer-service-readiness-service-handler-handoff',
      'docs/expectations/app-game-evidence.md',
      'crates/agent-service',
    ],
    serviceReadApiProofRefs: ['future-app-game-timer-service-readiness-read-api-proof'],
    serviceReadApiSummaryRef: 'future-service-readiness-read-api-handoff-summary-proof',
  };
}

function summarize(serviceReadApiHandoff) {
  return {
    sourceServiceHandlerHandoffId: serviceReadApiHandoff.sourceServiceHandlerHandoffId,
    nativeAppRowCount: serviceReadApiHandoff.nativeAppRowCount,
    nativeGameRowCount: serviceReadApiHandoff.nativeGameRowCount,
    serviceReadApiProofRequiredCount: serviceReadApiHandoff.serviceReadApiProofRequiredCount,
    blockedBySourceFreshnessCount: serviceReadApiHandoff.blockedBySourceFreshnessCount,
    blockedByCompilerDecisionCount: serviceReadApiHandoff.blockedByCompilerDecisionCount,
  };
}

function assertProof(proof) {
  if (proof.summary.serviceReadApiProofRequiredCount < 1) {
    throw new Error('Expected at least one service read API proof-required row');
  }
  if (proof.summary.blockedBySourceFreshnessCount < 1 || proof.summary.blockedByCompilerDecisionCount < 1) {
    throw new Error('Expected source-freshness and compiler-decision blocked rows');
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(
      'Expected WP92 proof to avoid service runtime, read API implementation, portal UI, evaluator, timer, scheduler storage, audit log, rollback execution, adapter, child, platform, and raw-source claims'
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
