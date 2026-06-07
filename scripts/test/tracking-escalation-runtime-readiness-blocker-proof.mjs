import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'tracking-escalation-runtime-readiness-blocker-proof';
const timestamp = '2026-06-07T20:20:00.000Z';
const resultDir = join(repoRoot, 'test-results', proofMode);
const focusedProofDir = join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const wp27ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '27-escalation-engine');
const wp33ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await main();

async function main() {
  await rm(resultDir, { recursive: true, force: true });
  await rm(focusedProofDir, { recursive: true, force: true });
  await mkdir(resultDir, { recursive: true });
  await mkdir(focusedProofDir, { recursive: true });
  await mkdir(wp27ProofDir, { recursive: true });
  await mkdir(wp33ProofDir, { recursive: true });

  run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  run('cmd', ['/c', 'npm', 'run', 'test', '--workspace', '@ocentra-parent/parent-domain', '--', proofMode]);
  run('node', ['scripts/test/tracking-escalation-readiness-proof.mjs']);
  run('node', ['scripts/test/tracking-provider-runtime-readiness-blocker-proof.mjs']);

  const blockerModule = await importDist('tracking-escalation-runtime-readiness-blocker-proof.js');
  const escalationProof = await readProofJson('test-results/tracking-escalation-readiness-proof/proof.json');
  const providerRuntimeProof = await readProofJson(
    'test-results/tracking-provider-runtime-readiness-blocker-proof/proof.json'
  );
  const sourceProofRefs = [
    'test-results/tracking-escalation-readiness-proof/proof.json',
    'test-results/tracking-provider-runtime-readiness-blocker-proof/proof.json',
  ];
  const readModel = blockerModule.buildTrackingEscalationRuntimeReadinessBlockerProof(
    {
      generatedAt: timestamp,
      proofId: proofMode,
      sourceProofRefs,
    },
    escalationProof.readModel,
    providerRuntimeProof.readModel
  );
  const proof = buildProof(readModel, sourceProofRefs);

  assertProof(proof);
  await writeArtifacts(proof);

  console.log('tracking-escalation-runtime-readiness-blocker-proof-ok');
  console.log(`evidence=${relativePath(join(resultDir, 'proof.json'))}`);
}

function buildProof(readModel, sourceProofRefs) {
  return {
    proofMode,
    generatedAt: timestamp,
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    commit: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: initialGitStatusShort,
    workpackIds: ['27-escalation-engine', '33-proof-gates-fixtures-rollout-and-pr-gate'],
    requiredProofTier: 'P4_MANUAL_ESCALATION_RUNTIME',
    currentProofTier: 'P1_FIXTURE_SIMULATION',
    status: 'manual_required',
    sourceProofRefs,
    summary: {
      escalationReadinessRows: readModel.escalationReadinessRows,
      escalationManualRequiredRows: readModel.escalationManualRequiredRows,
      providerRuntimeBlockerRows: readModel.providerRuntimeBlockerRows,
      blockerCount: readModel.blockers.length,
      productReadyBlockers: readModel.blockers.filter((row) => row.blockerId === 'product-ready-tracking-escalation')
        .length,
    },
    proofLabels: [
      'tracking-escalation-runtime.aggregate-blocker',
      'tracking-escalation-runtime.provider-runtime-linked',
      'tracking-escalation-runtime.manual-required-until-workers-and-timers',
      'tracking-escalation-runtime.product-ready-false',
    ],
    productClaims: readModel.productClaims,
    readModel,
    commands,
  };
}

function assertProof(proof) {
  assert.equal(proof.summary.escalationReadinessRows > 0, true, 'expected escalation readiness rows');
  assert.equal(proof.summary.providerRuntimeBlockerRows > 0, true, 'expected provider runtime blockers');
  assert.equal(proof.summary.blockerCount, 12, 'expected every escalation runtime blocker');
  assert.equal(proof.summary.productReadyBlockers, 1, 'expected product-ready blocker row');
  assert.equal(
    Object.values(proof.productClaims).every((claim) => claim === false),
    true,
    'no runtime product claims'
  );
}

async function writeArtifacts(proof) {
  await writeJson(join(resultDir, 'proof.json'), proof);
  await writeJson(join(resultDir, 'read-model.json'), proof.readModel);
  await writeJson(join(focusedProofDir, 'proof.json'), proof);
  await writeJson(join(focusedProofDir, 'read-model.json'), proof.readModel);
  await writeFile(
    join(focusedProofDir, '00-source-snapshot.md'),
    [
      '# Tracking Escalation Runtime Readiness Blocker Source Snapshot',
      '',
      `- generatedAt: ${timestamp}`,
      `- commit: ${proof.commit}`,
      `- status: ${proof.status}`,
      `- blockerCount: ${proof.summary.blockerCount}`,
      '',
    ].join('\n'),
    'utf8'
  );
  await writeJson(join(wp27ProofDir, '10-escalation-runtime-readiness-blocker-proof.json'), proof);
  await writeJson(join(wp33ProofDir, '53-escalation-runtime-readiness-blocker-proof.json'), proof);
  await writeFile(
    join(wp33ProofDir, '53-escalation-runtime-readiness-blocker-validation-commands.log'),
    commandLog(),
    'utf8'
  );
}

async function readProofJson(relativePath) {
  return JSON.parse(await readFile(join(repoRoot, relativePath), 'utf8'));
}

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function run(command, args) {
  commands.push({ command: [command, ...args].join(' ') });
  const result = spawnSync(command, args, { cwd: repoRoot, stdio: 'inherit', shell: false });
  if (result.status !== 0) throw new Error(`Command failed: ${command} ${args.join(' ')}`);
}

function gitOutput(args) {
  return spawnSync('git', args, { cwd: repoRoot, encoding: 'utf8' }).stdout.trim();
}

async function writeJson(filePath, value) {
  await writeFile(filePath, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}

function commandLog() {
  return `${commands.map((entry) => entry.command).join('\n')}\n`;
}

function relativePath(filePath) {
  return filePath
    .replace(repoRoot, '')
    .replace(/^[/\\]/, '')
    .replaceAll('\\', '/');
}
