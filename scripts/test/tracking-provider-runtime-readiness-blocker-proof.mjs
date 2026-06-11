import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
import { runNpmCommand } from './run-npm-command.mjs';

const repoRoot = process.cwd();
const proofMode = 'tracking-provider-runtime-readiness-blocker-proof';
const timestamp = '2026-06-07T20:10:00.000Z';
const resultDir = join(repoRoot, 'test-results', proofMode);
const focusedProofDir = join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const wp26ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '26-alert-severity-and-notification-model');
const wp33ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await main();

async function main() {
  await rm(resultDir, { recursive: true, force: true });
  await rm(focusedProofDir, { recursive: true, force: true });
  await mkdir(resultDir, { recursive: true });
  await mkdir(focusedProofDir, { recursive: true });
  await mkdir(wp26ProofDir, { recursive: true });
  await mkdir(wp33ProofDir, { recursive: true });

  runNpmCommand(run, ['run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  runNpmCommand(run, ['run', 'test', '--workspace', '@ocentra-parent/parent-domain', '--', proofMode]);

  const providerRuntimeModule = await importDist('tracking-provider-runtime-readiness-blocker-proof.js');
  const providerProof = await readProofJson('test-results/tracking-provider-notification-proof/proof.json');
  const receiptProof = await readProofJson('test-results/tracking-notification-receipt-boundary-proof/proof.json');
  const outboxProof = await readProofJson('test-results/tracking-notification-local-outbox-readiness-proof/proof.json');
  const artifactGateProof = await readProofJson(
    'test-results/tracking-provider-delivery-artifact-gate-proof/proof.json'
  );
  const sourceProofRefs = [
    'test-results/tracking-provider-notification-proof/proof.json',
    'test-results/tracking-notification-receipt-boundary-proof/proof.json',
    'test-results/tracking-notification-local-outbox-readiness-proof/proof.json',
    'test-results/tracking-provider-delivery-artifact-gate-proof/proof.json',
  ];
  const readModel = providerRuntimeModule.buildTrackingProviderRuntimeReadinessBlockerProof(
    {
      generatedAt: timestamp,
      proofId: proofMode,
      sourceProofRefs,
    },
    providerProof.readModel,
    receiptProof.readModel,
    outboxProof.readModel,
    artifactGateProof.readModel
  );
  const proof = buildProof(readModel, sourceProofRefs);

  assertProof(proof);
  await writeArtifacts(proof);

  console.log('tracking-provider-runtime-readiness-blocker-proof-ok');
  console.log(`evidence=${relativePath(join(resultDir, 'proof.json'))}`);
}

function buildProof(readModel, sourceProofRefs) {
  return {
    proofMode,
    generatedAt: timestamp,
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    commit: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: initialGitStatusShort,
    workpackIds: ['26-alert-severity-and-notification-model', '33-proof-gates-fixtures-rollout-and-pr-gate'],
    requiredProofTier: 'P4_MANUAL_PROVIDER_RUNTIME',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    status: 'manual_required',
    sourceProofRefs,
    summary: {
      providerNotificationRows: readModel.providerNotificationRows,
      receiptBoundaryRows: readModel.receiptBoundaryRows,
      localOutboxReadinessRows: readModel.localOutboxReadinessRows,
      requiredProviderRuntimeArtifactCount: readModel.requiredProviderRuntimeArtifactCount,
      presentProviderRuntimeArtifactCount: readModel.presentProviderRuntimeArtifactCount,
      missingProviderRuntimeArtifactCount: readModel.missingProviderRuntimeArtifactCount,
      providerRuntimeArtifactSetComplete: readModel.providerRuntimeArtifactSetComplete,
      blockerCount: readModel.blockers.length,
      productReadyBlockers: readModel.blockers.filter((row) => row.blockerId === 'product-ready-tracking').length,
    },
    proofLabels: [
      'tracking-provider-runtime.aggregate-blocker',
      'tracking-provider-runtime.receipt-outbox-artifact-gate-linked',
      'tracking-provider-runtime.manual-required-until-provider-artifacts',
      'tracking-provider-runtime.product-ready-false',
    ],
    productClaims: readModel.productClaims,
    readModel,
    commands,
  };
}

function assertProof(proof) {
  assert.equal(proof.summary.providerNotificationRows > 0, true, 'expected provider notification rows');
  assert.equal(proof.summary.receiptBoundaryRows > 0, true, 'expected receipt boundary rows');
  assert.equal(proof.summary.localOutboxReadinessRows > 0, true, 'expected local outbox readiness rows');
  assert.equal(proof.summary.requiredProviderRuntimeArtifactCount, 11, 'expected every provider runtime artifact');
  assert.equal(proof.summary.presentProviderRuntimeArtifactCount, 0, 'expected no present provider runtime artifacts');
  assert.equal(proof.summary.missingProviderRuntimeArtifactCount, 11, 'expected missing runtime artifacts');
  assert.equal(
    proof.summary.providerRuntimeArtifactSetComplete,
    false,
    'expected incomplete provider runtime artifacts'
  );
  assert.deepEqual(
    proof.readModel.requiredProviderRuntimeArtifactRefs,
    proof.readModel.missingProviderRuntimeArtifactRefs,
    'all required provider runtime artifacts should still be missing'
  );
  assert.equal(proof.summary.blockerCount, 12, 'expected every provider runtime blocker');
  assert.equal(proof.summary.productReadyBlockers, 1, 'expected product-ready blocker row');
  assert.equal(
    Object.values(proof.productClaims).every((claim) => claim === false),
    true,
    'no product claims'
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
      '# Tracking Provider Runtime Readiness Blocker Source Snapshot',
      '',
      `- generatedAt: ${timestamp}`,
      `- commit: ${proof.commit}`,
      `- status: ${proof.status}`,
      `- requiredProviderRuntimeArtifactCount: ${proof.summary.requiredProviderRuntimeArtifactCount}`,
      `- presentProviderRuntimeArtifactCount: ${proof.summary.presentProviderRuntimeArtifactCount}`,
      `- missingProviderRuntimeArtifactCount: ${proof.summary.missingProviderRuntimeArtifactCount}`,
      `- missingProviderRuntimeArtifactRefs: ${proof.readModel.missingProviderRuntimeArtifactRefs.join(', ')}`,
      '',
    ].join('\n'),
    'utf8'
  );
  await writeJson(join(wp26ProofDir, '30-provider-runtime-readiness-blocker-proof.json'), proof);
  await writeJson(join(wp33ProofDir, '52-provider-runtime-readiness-blocker-proof.json'), proof);
  await writeFile(
    join(wp33ProofDir, '52-provider-runtime-readiness-blocker-validation-commands.log'),
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
