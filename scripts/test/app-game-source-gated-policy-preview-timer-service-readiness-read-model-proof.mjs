import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofSlug = '87-source-gated-policy-preview-timer-service-readiness-read-model';
const testOutputDir = join(
  repoRoot,
  'test-results',
  'app-game-source-gated-policy-preview-timer-service-readiness-read-model-proof'
);
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', proofSlug);
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', proofSlug);
const timestamp = '2026-06-06T06:55:00Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

for (const path of [testOutputDir, appGameProofDir, appProofDir]) {
  await rm(path, { recursive: true, force: true });
  await mkdir(path, { recursive: true });
}
for (const path of [join(appGameProofDir, '06-ui-snapshots'), join(appProofDir, '06-ui-snapshots')]) {
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
  'app-game-source-gated-policy-preview-timer-service-readiness-read-model',
  'app-game-source-gated-policy-preview-timer-service-readiness-handoff',
]);

const readModelContract = await importAppGameDist(
  'app-game-source-gated-policy-preview-timer-service-readiness-read-model.js'
);
const wp86Proof = await readJson(
  join(
    repoRoot,
    'test-results',
    'app-game-source-gated-policy-preview-timer-service-readiness-handoff-proof',
    'proof.json'
  )
);
const serviceReadinessReadModel = readModelContract.buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModel(
  readModelOptions(await importSchemaDist('reference-primitives.js')),
  wp86Proof.serviceReadinessHandoff
);
const proof = {
  proofMode: 'app-game-source-gated-policy-preview-timer-service-readiness-read-model',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  stackedOn: {
    wp86Branch: 'codex/app-game-source-gated-policy-preview-timer-service-readiness-handoff',
    reason:
      'Schema-domain owns the service-readiness read-model contract surface; app-game-domain consumes WP86 service-readiness handoff rows while agent protocol constants, service runtime events, read API implementation, portal rendering, timer runtime, durable scheduler/audit storage, rollback execution, adapter dispatch, child delivery, platform enforcement, and package exports remain sequenced separately.',
  },
  summary: summarize(serviceReadinessReadModel),
  nonClaims: {
    serviceRuntimeEventClaimed: serviceReadinessReadModel.serviceRuntimeEventClaimed,
    serviceReadApiImplemented: serviceReadinessReadModel.serviceReadApiImplemented,
    portalUiRendered: serviceReadinessReadModel.portalUiRendered,
    policyEvaluatorRuntimeClaimed: serviceReadinessReadModel.policyEvaluatorRuntimeClaimed,
    timerRuntimeClaimed: serviceReadinessReadModel.timerRuntimeClaimed,
    timerScheduled: serviceReadinessReadModel.timerScheduled,
    schedulerPersistenceRuntimeClaimed: serviceReadinessReadModel.schedulerPersistenceRuntimeClaimed,
    durableSchedulerStorageClaimed: serviceReadinessReadModel.durableSchedulerStorageClaimed,
    auditRuntimeClaimed: serviceReadinessReadModel.auditRuntimeClaimed,
    durableAuditLogClaimed: serviceReadinessReadModel.durableAuditLogClaimed,
    rollbackRuntimeClaimed: serviceReadinessReadModel.rollbackRuntimeClaimed,
    rollbackExecutionClaimed: serviceReadinessReadModel.rollbackExecutionClaimed,
    adapterDispatchClaimed: serviceReadinessReadModel.adapterDispatchClaimed,
    childDeliveryClaimed: serviceReadinessReadModel.childDeliveryClaimed,
    platformEnforcementClaimed: serviceReadinessReadModel.platformEnforcementClaimed,
    rawPrivateSourceRowsIncluded: serviceReadinessReadModel.rawPrivateSourceRowsIncluded,
  },
  proofPaths: {
    schemaSource: 'packages/schema-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-read-model.ts',
    schemaRules:
      'packages/schema-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-read-model-rules.ts',
    consumerSource:
      'packages/app-game-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-read-model.ts',
    consumerTest:
      'packages/app-game-domain/tests/unit/app-game-source-gated-policy-preview-timer-service-readiness-read-model.test.ts',
    harness: 'scripts/test/app-game-source-gated-policy-preview-timer-service-readiness-read-model-proof.mjs',
    evidence: 'test-results/app-game-source-gated-policy-preview-timer-service-readiness-read-model-proof/proof.json',
    appGameProofPack: `output/app-game-plan-proof/${proofSlug}`,
    appProofPack: `output/app-plan-proof/${proofSlug}`,
  },
  serviceReadinessReadModel,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'timer-service-readiness-read-model.json'), serviceReadinessReadModel);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(appGameProofDir, proof, 'app-game WP87');
await writeProofPack(appProofDir, proof, 'app WP87');

console.log('app-game-source-gated-policy-preview-timer-service-readiness-read-model-proof-ok');
console.log(
  `evidence=${join('test-results', 'app-game-source-gated-policy-preview-timer-service-readiness-read-model-proof', 'proof.json')}`
);

function importAppGameDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'app-game-domain', 'dist', name)).href);
}

function importSchemaDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'schema-domain', 'dist', name)).href);
}

function readModelOptions(refs) {
  return {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    readModelId: 'app-game-source-gated-policy-preview-timer-service-readiness-read-model-proof',
    generatedAt: timestamp,
    sourceContractRefs: [
      'app-game-source-gated-policy-preview-timer-service-readiness-handoff',
      'docs/expectations/app-game-evidence.md',
      'docs/expectations/policy.md',
    ],
    serviceReadinessSummaryRef: 'future-service-readiness-read-model-summary-proof',
  };
}

function summarize(readModel) {
  return {
    sourceServiceReadinessHandoffId: readModel.sourceServiceReadinessHandoffId,
    nativeAppRowCount: readModel.nativeAppRowCount,
    nativeGameRowCount: readModel.nativeGameRowCount,
    serviceReadModelProofRequiredCount: readModel.serviceReadModelProofRequiredCount,
    blockedBySourceFreshnessCount: readModel.blockedBySourceFreshnessCount,
    blockedByCompilerDecisionCount: readModel.blockedByCompilerDecisionCount,
  };
}

function assertProof(proof) {
  if (proof.summary.serviceReadModelProofRequiredCount < 1) {
    throw new Error('Expected at least one service read-model proof-required row');
  }
  if (proof.summary.blockedBySourceFreshnessCount < 1 || proof.summary.blockedByCompilerDecisionCount < 1) {
    throw new Error('Expected source-freshness and compiler-decision blocked rows');
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(
      'Expected WP87 proof to avoid service runtime, read API implementation, portal UI, evaluator, timer, scheduler storage, audit log, rollback execution, adapter, child, platform, and raw-source claims'
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
