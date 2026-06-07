import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdir, stat, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'tracking-production-worker-runtime-artifact-gate-proof';
const generatedAt = '2026-06-07T22:50:00.000Z';
const resultDir = path.join(repoRoot, 'test-results', proofMode);
const focusedProofDir = path.join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const output33 = path.join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const commands = [];

await main();

async function main() {
  await mkdir(resultDir, { recursive: true });
  await mkdir(focusedProofDir, { recursive: true });
  await mkdir(output33, { recursive: true });

  run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  run('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'tracking-production-worker-runtime-artifact-gate-proof',
  ]);

  const proofModule = await importDist('tracking-production-worker-runtime-artifact-gate-proof.js');
  const inventory = {
    presentArtifacts: await presentArtifactsForRoot(
      path.join(repoRoot, proofModule.RequiredTrackingProductionWorkerRuntimeArtifactPlan.proofRoot),
      proofModule.RequiredTrackingProductionWorkerRuntimeArtifactPlan.requiredArtifacts
    ),
  };
  const readModel = proofModule.buildTrackingProductionWorkerRuntimeArtifactGateProof(generatedAt, inventory);
  const proof = buildProof({ readModel });

  assertProof(proof);
  await writeArtifacts(proof);

  console.log('tracking-production-worker-runtime-artifact-gate-proof-ok');
  console.log(`evidence=${relativePath(path.join(resultDir, 'proof.json'))}`);
}

async function presentArtifactsForRoot(rootPath, requiredArtifacts) {
  const present = [];
  for (const artifact of requiredArtifacts) {
    const artifactPath = path.join(rootPath, artifact);
    if (await pathExists(artifactPath)) present.push(artifact);
  }
  return present;
}

async function pathExists(filePath) {
  try {
    await stat(filePath);
    return true;
  } catch (error) {
    if (error?.code === 'ENOENT') return false;
    throw error;
  }
}

function buildProof({ readModel }) {
  return {
    schemaVersion: 1,
    proofMode,
    generatedAt,
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    commit: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: gitOutput(['status', '--short']),
    workpackIds: ['33-proof-gates-fixtures-rollout-and-pr-gate'],
    requiredProofTier: 'P4_PRODUCTION_RUNTIME',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    status: readModel.rows.every((row) => row.productionWorkerArtifactSetComplete)
      ? 'artifact_set_present'
      : 'manual_required',
    readModel,
    summary: {
      rowCount: readModel.rows.length,
      completeRows: readModel.rows.filter((row) => row.productionWorkerArtifactSetComplete).length,
      manualRequiredRows: readModel.rows.filter((row) => !row.productionWorkerArtifactSetComplete).length,
      missingArtifactCount: readModel.rows.reduce((total, row) => total + row.missingArtifacts.length, 0),
      requiredArtifactCount: readModel.rows.reduce((total, row) => total + row.requiredArtifacts.length, 0),
      locationUploadWorkerRuntimeClaimedRows: readModel.rows.filter((row) => row.locationUploadWorkerRuntimeClaimed)
        .length,
      productionAuditDurableStorageClaimedRows: readModel.rows.filter((row) => row.productionAuditDurableStorageClaimed)
        .length,
      productReadyRows: readModel.rows.filter((row) => row.productClaimReady).length,
    },
    proofLabels: [
      'tracking-production-worker-runtime.artifact-gate',
      'tracking-production-worker-runtime.required-artifacts-from-blocker-source',
      'tracking-production-worker-runtime.no-production-claim-from-file-presence',
      'tracking-production-worker-runtime.product-ready-false',
    ],
    productClaims: readModel.productClaims,
    missingProofReason:
      'Actual tracking production worker execution still requires the tracking-production runtime artifact set for location upload, retention cleanup, notification outbox, escalation timeout, provider receipt, child-device delivery, authority status, and audit durable storage. This gate validates required artifact presence only and keeps production worker execution, durable production storage, physical-device, authority, provider delivery/receipt runtime, and product-ready claims false.',
    commands,
  };
}

function assertProof(proof) {
  assert.equal(proof.summary.rowCount, 1, 'expected one production worker artifact gate row');
  assert.equal(proof.summary.requiredArtifactCount, 8, 'expected all required production worker artifact refs');
  assert.equal(proof.summary.locationUploadWorkerRuntimeClaimedRows, 0, 'no location upload runtime claims');
  assert.equal(proof.summary.productionAuditDurableStorageClaimedRows, 0, 'no production audit storage claims');
  assert.equal(proof.summary.productReadyRows, 0, 'no product-ready rows');
  assert.deepEqual(Object.values(proof.productClaims), [
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
  ]);
}

async function writeArtifacts(proof) {
  await writeJson(path.join(resultDir, 'proof.json'), proof);
  await writeJson(path.join(resultDir, 'read-model.json'), proof.readModel);
  await writeJson(path.join(focusedProofDir, 'proof.json'), proof);
  await writeJson(path.join(focusedProofDir, 'read-model.json'), proof.readModel);
  await writeFile(
    path.join(focusedProofDir, '00-source-snapshot.md'),
    [
      '# Tracking Production Worker Runtime Artifact Gate Source Snapshot',
      '',
      `- generatedAt: ${generatedAt}`,
      `- commit: ${proof.commit}`,
      `- status: ${proof.status}`,
      `- requiredArtifactCount: ${proof.summary.requiredArtifactCount}`,
      `- missingArtifactCount: ${proof.summary.missingArtifactCount}`,
      '- required artifact refs are imported from tracking production durable workers readiness blocker source',
      '- artifact presence alone does not claim production worker execution or product readiness',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeJson(path.join(output33, '58-production-worker-runtime-artifact-gate-proof.json'), proof);
  await writeFile(
    path.join(output33, '58-production-worker-runtime-artifact-gate-validation-commands.log'),
    `${commands.map((entry) => entry.command).join('\n')}\n`,
    'utf8'
  );
}

function importDist(name) {
  return import(pathToFileURL(path.join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
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

function relativePath(filePath) {
  return path.relative(repoRoot, filePath).replaceAll(path.sep, '/');
}
