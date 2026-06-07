import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'tracking-production-durable-workers-readiness-blocker-proof';
const timestamp = '2026-06-07T22:15:00.000Z';
const resultDir = join(repoRoot, 'test-results', proofMode);
const focusedProofDir = join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const wp33ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

const sourceProofRefs = [
  'packages/parent-domain/src/production-support-status-backend-durable-queue-runtime-proof.ts',
  'packages/parent-domain/src/production-support-status-backend-durable-queue-runtime-read-model.ts',
  'test-results/tracking-provider-runtime-readiness-blocker-proof/proof.json',
  'test-results/tracking-escalation-runtime-readiness-blocker-proof/proof.json',
  'test-results/tracking-retention-durable-settings-proof/proof.json',
];

await main();

async function main() {
  await rm(resultDir, { recursive: true, force: true });
  await rm(focusedProofDir, { recursive: true, force: true });
  await mkdir(resultDir, { recursive: true });
  await mkdir(focusedProofDir, { recursive: true });
  await mkdir(wp33ProofDir, { recursive: true });

  run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  run('cmd', ['/c', 'npm', 'run', 'test', '--workspace', '@ocentra-parent/parent-domain', '--', proofMode]);

  await assertSourceProofsExist();
  const trackingProductionModule = await importDist('tracking-production-durable-workers-readiness-blocker-proof.js');
  const productionSupportReadModelModule = await importDist(
    'production-support-status-backend-durable-queue-runtime-read-model.js'
  );
  const proof = buildProof(
    trackingProductionModule,
    productionSupportReadModelModule.ProductionSupportStatusBackendDurableQueueRuntimeReadModel
  );

  assertProof(proof);
  await writeArtifacts(proof);

  console.log('tracking-production-durable-workers-readiness-blocker-proof-ok');
  console.log(`evidence=${relativePath(join(resultDir, 'proof.json'))}`);
}

function buildProof(trackingProductionModule, productionSupportReadModel) {
  const readModel = trackingProductionModule.buildTrackingProductionDurableWorkersReadinessBlockerProof(
    {
      generatedAt: timestamp,
      proofId: proofMode,
      sourceProofRefs,
      requiredTrackingWorkerArtifactRefs: trackingProductionModule.RequiredTrackingProductionDurableWorkerArtifactRefs,
    },
    productionSupportReadModel
  );

  return {
    proofMode,
    generatedAt: timestamp,
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    commit: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: initialGitStatusShort,
    workpackIds: ['33-proof-gates-fixtures-rollout-and-pr-gate'],
    requiredProofTier: 'P4_PRODUCTION_RUNTIME',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    status: 'manual_required',
    sourceProofRefs,
    summary: {
      productionSupportDurableQueueRows: readModel.productionSupportDurableQueueRows,
      productionSupportManualClaimCount: readModel.productionSupportManualClaimCount,
      requiredTrackingWorkerArtifactCount: readModel.requiredTrackingWorkerArtifactCount,
      blockerCount: readModel.blockers.length,
      productReadyBlockers: readModel.blockers.filter(
        (row) => row.blockerId === 'tracking-production-product-ready-closure'
      ).length,
    },
    proofLabels: [
      'tracking-production-durable-workers.production-support-boundary-observed',
      'tracking-production-durable-workers.tracking-worker-artifacts-required',
      'tracking-production-durable-workers.manual-required-until-production-artifacts',
      'tracking-production-durable-workers.product-ready-false',
    ],
    productClaims: readModel.productClaims,
    readModel,
    commands,
  };
}

function assertProof(proof) {
  assert.equal(proof.summary.productionSupportDurableQueueRows > 0, true, 'expected production support rows');
  assert.equal(proof.summary.productionSupportManualClaimCount > 0, true, 'expected manual production claims');
  assert.equal(proof.summary.requiredTrackingWorkerArtifactCount, 8, 'expected tracking worker artifact refs');
  assert.equal(proof.summary.blockerCount, 9, 'expected every tracking production blocker');
  assert.equal(proof.summary.productReadyBlockers, 1, 'expected product-ready production blocker row');
  assert.equal(proof.productClaims.productionSupportBoundaryObserved, true, 'expected production support boundary');
  assert.equal(proof.productClaims.productClaimReady, false, 'expected product claim false');
}

async function writeArtifacts(proof) {
  await writeJson(join(resultDir, 'proof.json'), proof);
  await writeJson(join(resultDir, 'read-model.json'), proof.readModel);
  await writeJson(join(focusedProofDir, 'proof.json'), proof);
  await writeJson(join(focusedProofDir, 'read-model.json'), proof.readModel);
  await writeFile(
    join(focusedProofDir, '00-source-snapshot.md'),
    [
      '# Tracking Production Durable Workers Readiness Blocker Source Snapshot',
      '',
      `- generatedAt: ${timestamp}`,
      `- commit: ${proof.commit}`,
      `- status: ${proof.status}`,
      `- productionSupportDurableQueueRows: ${proof.summary.productionSupportDurableQueueRows}`,
      `- requiredTrackingWorkerArtifactCount: ${proof.summary.requiredTrackingWorkerArtifactCount}`,
      '- production support durable queue boundary is observed but does not prove tracking production workers',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeJson(join(wp33ProofDir, '57-production-durable-workers-readiness-blocker-proof.json'), proof);
  await writeFile(
    join(wp33ProofDir, '57-production-durable-workers-readiness-blocker-validation-commands.log'),
    commandLog(),
    'utf8'
  );
}

async function assertSourceProofsExist() {
  for (const sourceProofRef of sourceProofRefs) {
    const contents = await readFile(join(repoRoot, sourceProofRef), 'utf8');
    if (sourceProofRef.endsWith('.json')) JSON.parse(contents);
  }
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
