import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofSlug = '79-source-gated-policy-preview-timer-status';
const testOutputDir = join(repoRoot, 'test-results', 'app-game-source-gated-policy-preview-timer-status-proof');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', proofSlug);
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', proofSlug);
const timestamp = '2026-06-06T02:20:00Z';
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
  'app-game-source-gated-policy-preview-timer-status',
  'app-game-source-gated-policy-preview-timer-handoff',
]);

const timerStatusContract = await importDist('app-game-source-gated-policy-preview-timer-status.js');
const wp78Proof = await readJson(
  join(repoRoot, 'test-results', 'app-game-source-gated-policy-preview-timer-handoff-proof', 'proof.json')
);
const status = timerStatusContract.buildAppGameSourceGatedPolicyPreviewTimerStatus(
  timerStatusOptions(await importDist('reference-primitives.js')),
  wp78Proof.handoff
);
const proof = {
  proofMode: 'app-game-source-gated-policy-preview-timer-status',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  stackedOn: {
    wp78Branch: 'codex/app-game-source-gated-policy-preview-timer-handoff-main',
    reason:
      'WP79 consumes WP78 timer-handoff rows and remains parent-domain only while service/protocol/package work is sequenced separately.',
  },
  summary: summarize(status),
  nonClaims: {
    serviceRuntimeEventClaimed: status.serviceRuntimeEventClaimed,
    portalUiRendered: status.portalUiRendered,
    policyEvaluatorRuntimeClaimed: status.policyEvaluatorRuntimeClaimed,
    timerRuntimeClaimed: status.timerRuntimeClaimed,
    timerScheduled: status.timerScheduled,
    adapterDispatchClaimed: status.adapterDispatchClaimed,
    childDeliveryClaimed: status.childDeliveryClaimed,
    platformEnforcementClaimed: status.platformEnforcementClaimed,
    rawPrivateSourceRowsIncluded: status.rawPrivateSourceRowsIncluded,
  },
  proofPaths: {
    source: 'packages/parent-domain/src/app-game-source-gated-policy-preview-timer-status.ts',
    rules: 'packages/parent-domain/src/app-game-source-gated-policy-preview-timer-status-rules.ts',
    test: 'packages/parent-domain/tests/app-game-source-gated-policy-preview-timer-status.test.ts',
    harness: 'scripts/test/app-game-source-gated-policy-preview-timer-status-proof.mjs',
    evidence: 'test-results/app-game-source-gated-policy-preview-timer-status-proof/proof.json',
    appGameProofPack: `output/app-game-plan-proof/${proofSlug}`,
    appProofPack: `output/app-plan-proof/${proofSlug}`,
  },
  status,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'timer-status.json'), status);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(appGameProofDir, proof, 'app-game WP79');
await writeProofPack(appProofDir, proof, 'app WP79');

console.log('app-game-source-gated-policy-preview-timer-status-proof-ok');
console.log(
  `evidence=${join('test-results', 'app-game-source-gated-policy-preview-timer-status-proof', 'proof.json')}`
);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function timerStatusOptions(refs) {
  return {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    statusId: 'app-game-source-gated-policy-preview-timer-status-proof',
    generatedAt: timestamp,
    sourceContractRefs: [
      'app-game-source-gated-policy-preview-timer-handoff',
      'docs/expectations/app-game-evidence.md',
      'docs/expectations/policy.md',
    ],
    timerRuntimeProofRef: 'future-service-timer-runtime-proof',
    sourceFreshnessProofRef: 'source-freshness-proof-required',
    compilerDecisionProofRef: 'compiler-decision-proof-required',
  };
}

function summarize(status) {
  return {
    sourceTimerHandoffId: status.sourceTimerHandoffId,
    nativeAppRowCount: status.nativeAppRowCount,
    nativeGameRowCount: status.nativeGameRowCount,
    timerRuntimeProofRequiredCount: status.timerRuntimeProofRequiredCount,
    sourceFreshnessProofRequiredCount: status.sourceFreshnessProofRequiredCount,
    compilerDecisionProofRequiredCount: status.compilerDecisionProofRequiredCount,
  };
}

function assertProof(proof) {
  if (proof.summary.timerRuntimeProofRequiredCount < 1) {
    throw new Error('Expected at least one timer runtime proof-required row');
  }
  if (proof.summary.sourceFreshnessProofRequiredCount < 1 || proof.summary.compilerDecisionProofRequiredCount < 1) {
    throw new Error('Expected source-freshness and compiler-decision proof-required rows');
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(
      'Expected WP79 proof to avoid runtime, UI, timer scheduling, adapter, child, platform, and raw-source claims'
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
