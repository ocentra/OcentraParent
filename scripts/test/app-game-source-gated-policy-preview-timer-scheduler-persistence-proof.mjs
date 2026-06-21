import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofSlug = '82-source-gated-policy-preview-timer-scheduler-persistence';
const testOutputDir = join(
  repoRoot,
  'test-results',
  'app-game-source-gated-policy-preview-timer-scheduler-persistence-proof'
);
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', proofSlug);
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', proofSlug);
const timestamp = '2026-06-06T04:45:00Z';
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
  'app-game-source-gated-policy-preview-timer-scheduler-persistence',
  'app-game-source-gated-policy-preview-timer-runtime-readiness',
]);

const schedulerPersistenceContract = await importAppGameDist(
  'app-game-source-gated-policy-preview-timer-scheduler-persistence.js'
);
const wp81Proof = await readJson(
  join(repoRoot, 'test-results', 'app-game-source-gated-policy-preview-timer-runtime-readiness-proof', 'proof.json')
);
const persistence = schedulerPersistenceContract.buildAppGameSourceGatedPolicyPreviewTimerSchedulerPersistence(
  schedulerPersistenceOptions(await importSchemaDist('reference-primitives.js')),
  wp81Proof.readiness
);
const proof = {
  proofMode: 'app-game-source-gated-policy-preview-timer-scheduler-persistence',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  stackedOn: {
    wp81Branch: 'codex/app-game-source-gated-policy-preview-timer-runtime-readiness',
    reason:
      'Schema-domain owns the scheduler-persistence contract surface; app-game-domain consumes WP81 timer runtime-readiness rows while durable scheduler storage, timer runtime, service scheduling, and package behavior remain sequenced separately.',
  },
  summary: summarize(persistence),
  nonClaims: {
    serviceRuntimeEventClaimed: persistence.serviceRuntimeEventClaimed,
    portalUiRendered: persistence.portalUiRendered,
    policyEvaluatorRuntimeClaimed: persistence.policyEvaluatorRuntimeClaimed,
    timerRuntimeClaimed: persistence.timerRuntimeClaimed,
    timerScheduled: persistence.timerScheduled,
    schedulerPersistenceRuntimeClaimed: persistence.schedulerPersistenceRuntimeClaimed,
    durableSchedulerStorageClaimed: persistence.durableSchedulerStorageClaimed,
    auditRuntimeClaimed: persistence.auditRuntimeClaimed,
    rollbackRuntimeClaimed: persistence.rollbackRuntimeClaimed,
    adapterDispatchClaimed: persistence.adapterDispatchClaimed,
    childDeliveryClaimed: persistence.childDeliveryClaimed,
    platformEnforcementClaimed: persistence.platformEnforcementClaimed,
    rawPrivateSourceRowsIncluded: persistence.rawPrivateSourceRowsIncluded,
  },
  proofPaths: {
    schemaSource: 'packages/schema-domain/src/app-game-source-gated-policy-preview-timer-scheduler-persistence.ts',
    schemaRules: 'packages/schema-domain/src/app-game-source-gated-policy-preview-timer-scheduler-persistence-rules.ts',
    consumerSource: 'packages/app-game-domain/src/app-game-source-gated-policy-preview-timer-scheduler-persistence.ts',
    consumerTest: 'packages/app-game-domain/tests/unit/app-game-source-gated-policy-preview-timer-scheduler-persistence.test.ts',
    harness: 'scripts/test/app-game-source-gated-policy-preview-timer-scheduler-persistence-proof.mjs',
    evidence: 'test-results/app-game-source-gated-policy-preview-timer-scheduler-persistence-proof/proof.json',
    appGameProofPack: `output/app-game-plan-proof/${proofSlug}`,
    appProofPack: `output/app-plan-proof/${proofSlug}`,
  },
  persistence,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'timer-scheduler-persistence.json'), persistence);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(appGameProofDir, proof, 'app-game WP82');
await writeProofPack(appProofDir, proof, 'app WP82');

console.log('app-game-source-gated-policy-preview-timer-scheduler-persistence-proof-ok');
console.log(
  `evidence=${join(
    'test-results',
    'app-game-source-gated-policy-preview-timer-scheduler-persistence-proof',
    'proof.json'
  )}`
);

function importAppGameDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'app-game-domain', 'dist', name)).href);
}

function importSchemaDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'schema-domain', 'dist', name)).href);
}

function schedulerPersistenceOptions(refs) {
  return {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    persistenceId: 'app-game-source-gated-policy-preview-timer-scheduler-persistence-proof',
    generatedAt: timestamp,
    sourceContractRefs: [
      'app-game-source-gated-policy-preview-timer-runtime-readiness',
      'docs/expectations/app-game-evidence.md',
      'docs/expectations/policy.md',
    ],
    serviceTimerRuntimeProofRef: 'future-service-timer-runtime-proof',
    schedulerPersistenceProofRef: 'future-scheduler-persistence-proof',
    schedulerStateStoreProofRef: 'future-scheduler-state-store-proof',
    auditProofRef: 'future-timer-audit-proof',
    rollbackProofRef: 'future-timer-rollback-proof',
  };
}

function summarize(persistence) {
  return {
    sourceRuntimeReadinessId: persistence.sourceRuntimeReadinessId,
    nativeAppRowCount: persistence.nativeAppRowCount,
    nativeGameRowCount: persistence.nativeGameRowCount,
    schedulerPersistenceProofRequiredCount: persistence.schedulerPersistenceProofRequiredCount,
    blockedBySourceFreshnessCount: persistence.blockedBySourceFreshnessCount,
    blockedByCompilerDecisionCount: persistence.blockedByCompilerDecisionCount,
  };
}

function assertProof(proof) {
  if (proof.summary.schedulerPersistenceProofRequiredCount < 1) {
    throw new Error('Expected at least one scheduler persistence proof-required row');
  }
  if (proof.summary.blockedBySourceFreshnessCount < 1 || proof.summary.blockedByCompilerDecisionCount < 1) {
    throw new Error('Expected source-freshness and compiler-decision blocked rows');
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(
      'Expected WP82 proof to avoid service, UI, evaluator, timer, scheduler storage, audit, rollback, adapter, child, platform, and raw-source claims'
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
