import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofSlug = '94-source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff';
const testOutputDir = join(
  repoRoot,
  'test-results',
  'app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff-proof'
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

runNpm(['run', 'build', '--workspace', '@ocentra-parent/schema-domain']);
runNpm(['run', 'build', '--workspace', '@ocentra-parent/app-game-domain']);
runNpm([
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/app-game-domain',
  '--',
  'app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff',
  'app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-handoff',
]);

const consumerContract = await importAppGameDist(
  'app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff.js'
);
const refs = await importSchemaDist('reference-primitives.js');
const responseHandoff = await readJson(
  join(
    repoRoot,
    'test-results',
    'app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-handoff-proof',
    'timer-service-readiness-read-api-response-handoff.json'
  )
);
const readApiResponseConsumerHandoff =
  consumerContract.buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoff(
    readApiResponseConsumerHandoffOptions(refs),
    responseHandoff
  );
const proof = {
  proofMode: 'app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  stackedOn: {
    wp93Branch: 'codex/app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-handoff',
    reason:
      'Schema-domain owns the read-api-response-consumer handoff contract surface; app-game-domain consumes WP93 response handoff rows while actual response implementation, service consumer implementation, portal rendering, service command registration, service handler implementation, service event emission, service read API implementation, timer runtime, durable scheduler/audit storage, rollback execution, adapter dispatch, child delivery, platform enforcement, and package exports remain sequenced separately.',
  },
  summary: summarize(readApiResponseConsumerHandoff),
  nonClaims: {
    serviceCommandRegistered: readApiResponseConsumerHandoff.serviceCommandRegistered,
    serviceHandlerImplemented: readApiResponseConsumerHandoff.serviceHandlerImplemented,
    serviceReadApiImplemented: readApiResponseConsumerHandoff.serviceReadApiImplemented,
    serviceReadApiResponseImplemented: readApiResponseConsumerHandoff.serviceReadApiResponseImplemented,
    serviceReadApiResponseConsumerImplemented: readApiResponseConsumerHandoff.serviceReadApiResponseConsumerImplemented,
    serviceEventEmitted: readApiResponseConsumerHandoff.serviceEventEmitted,
    agentProtocolImplemented: readApiResponseConsumerHandoff.agentProtocolImplemented,
    rustProtocolMirrored: readApiResponseConsumerHandoff.rustProtocolMirrored,
    portalUiRendered: readApiResponseConsumerHandoff.portalUiRendered,
    portalResponseConsumerRendered: readApiResponseConsumerHandoff.portalResponseConsumerRendered,
    policyEvaluatorRuntimeClaimed: readApiResponseConsumerHandoff.policyEvaluatorRuntimeClaimed,
    timerRuntimeClaimed: readApiResponseConsumerHandoff.timerRuntimeClaimed,
    timerScheduled: readApiResponseConsumerHandoff.timerScheduled,
    schedulerPersistenceRuntimeClaimed: readApiResponseConsumerHandoff.schedulerPersistenceRuntimeClaimed,
    durableSchedulerStorageClaimed: readApiResponseConsumerHandoff.durableSchedulerStorageClaimed,
    auditRuntimeClaimed: readApiResponseConsumerHandoff.auditRuntimeClaimed,
    durableAuditLogClaimed: readApiResponseConsumerHandoff.durableAuditLogClaimed,
    rollbackRuntimeClaimed: readApiResponseConsumerHandoff.rollbackRuntimeClaimed,
    rollbackExecutionClaimed: readApiResponseConsumerHandoff.rollbackExecutionClaimed,
    adapterDispatchClaimed: readApiResponseConsumerHandoff.adapterDispatchClaimed,
    childDeliveryClaimed: readApiResponseConsumerHandoff.childDeliveryClaimed,
    platformEnforcementClaimed: readApiResponseConsumerHandoff.platformEnforcementClaimed,
    rawPrivateSourceRowsIncluded: readApiResponseConsumerHandoff.rawPrivateSourceRowsIncluded,
  },
  proofPaths: {
    schemaSource:
      'packages/schema-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff.ts',
    schemaRules:
      'packages/schema-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff-rules.ts',
    consumerSource:
      'packages/app-game-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff.ts',
    consumerTest:
      'packages/app-game-domain/tests/unit/app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff.test.ts',
    harness:
      'scripts/test/app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff-proof.mjs',
    evidence:
      'test-results/app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff-proof/proof.json',
    appGameProofPack: `output/app-game-plan-proof/${proofSlug}`,
    appProofPack: `output/app-plan-proof/${proofSlug}`,
  },
  readApiResponseConsumerHandoff,
};

assertProof(proof);
await writeJson(
  join(testOutputDir, 'timer-service-readiness-read-api-response-consumer-handoff.json'),
  readApiResponseConsumerHandoff
);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(appGameProofDir, proof, 'app-game WP94');
await writeProofPack(appProofDir, proof, 'app WP94');

console.log('app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff-proof-ok');
console.log(
  `evidence=${join(
    'test-results',
    'app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff-proof',
    'proof.json'
  )}`
);

function importAppGameDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'app-game-domain', 'dist', name)).href);
}

function importSchemaDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'schema-domain', 'dist', name)).href);
}

function readApiResponseConsumerHandoffOptions(refs) {
  return {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    readApiResponseConsumerHandoffId:
      'app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff-proof',
    generatedAt: timestamp,
    sourceContractRefs: [
      'app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-handoff',
      'docs/expectations/app-game-evidence.md',
      'docs/expectations/enforcement.md',
    ],
    readApiResponseConsumerProofRefs: ['future-app-game-timer-service-readiness-read-api-response-consumer-proof'],
    readApiResponseConsumerSummaryRef: 'future-service-readiness-read-api-response-consumer-handoff-summary-proof',
  };
}

function summarize(readApiResponseConsumerHandoff) {
  return {
    sourceReadApiResponseHandoffId: readApiResponseConsumerHandoff.sourceReadApiResponseHandoffId,
    nativeAppRowCount: readApiResponseConsumerHandoff.nativeAppRowCount,
    nativeGameRowCount: readApiResponseConsumerHandoff.nativeGameRowCount,
    readApiResponseConsumerProofRequiredCount: readApiResponseConsumerHandoff.readApiResponseConsumerProofRequiredCount,
    blockedBySourceFreshnessCount: readApiResponseConsumerHandoff.blockedBySourceFreshnessCount,
    blockedByCompilerDecisionCount: readApiResponseConsumerHandoff.blockedByCompilerDecisionCount,
  };
}

function assertProof(proof) {
  if (proof.summary.readApiResponseConsumerProofRequiredCount < 1) {
    throw new Error('Expected at least one read API response consumer proof-required row');
  }
  if (proof.summary.blockedBySourceFreshnessCount < 1 || proof.summary.blockedByCompilerDecisionCount < 1) {
    throw new Error('Expected source-freshness and compiler-decision blocked rows');
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(
      'Expected WP94 proof to avoid service runtime, response/consumer implementation, portal UI, evaluator, timer, scheduler storage, audit log, rollback execution, adapter, child, platform, and raw-source claims'
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
