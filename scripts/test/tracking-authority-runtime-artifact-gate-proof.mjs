import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdir, readFile, stat, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'tracking-authority-runtime-artifact-gate-proof';
const generatedAt = '2026-06-08T00:15:00.000Z';
const resultDir = path.join(repoRoot, 'test-results', proofMode);
const focusedProofDir = path.join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const output31 = path.join(
  repoRoot,
  'output',
  'tracking-plan-proof',
  '31-platform-extension-checklists-and-proof-routing'
);
const output33 = path.join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const commands = [];

await main();

async function main() {
  await mkdir(resultDir, { recursive: true });
  await mkdir(focusedProofDir, { recursive: true });
  await mkdir(output31, { recursive: true });
  await mkdir(output33, { recursive: true });

  run('node', ['scripts/test/tracking-authority-runtime-readiness-blocker-proof.mjs']);
  run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  run('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'tracking-authority-runtime-artifact-gate-proof',
  ]);

  const proofModule = await importDist('tracking-authority-runtime-artifact-gate-proof.js');
  const sourceProof = await assertSourceRuntimeReadinessProofExists(
    proofModule.RequiredTrackingAuthorityRuntimeArtifactPlan.sourceRuntimeReadinessProofRef
  );
  const requiredArtifacts = [...new Set(sourceProof.readModel.blockers.flatMap((row) => row.blockingEvidenceRefs))];
  const inventory = {
    presentArtifacts: await presentArtifactsForRoot(
      path.join(repoRoot, proofModule.RequiredTrackingAuthorityRuntimeArtifactPlan.proofRoot),
      requiredArtifacts
    ),
  };
  const readModel = proofModule.buildTrackingAuthorityRuntimeArtifactGateProof(
    generatedAt,
    sourceProof.readModel,
    inventory
  );
  const proof = buildProof({ readModel });

  assertProof(proof);
  await writeArtifacts(proof);

  console.log('tracking-authority-runtime-artifact-gate-proof-ok');
  console.log(`evidence=${relativePath(path.join(resultDir, 'proof.json'))}`);
}

async function assertSourceRuntimeReadinessProofExists(proofRef) {
  const sourceProof = JSON.parse(await readFile(path.join(repoRoot, proofRef), 'utf8'));
  assert.equal(sourceProof.proofMode, 'tracking-authority-runtime-readiness-blocker-proof');
  assert.equal(sourceProof.productClaims.productClaimReady, false);
  return sourceProof;
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
    workpackIds: ['31-platform-extension-checklists-and-proof-routing', '33-proof-gates-fixtures-rollout-and-pr-gate'],
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    currentProofTier: 'P0_CONTRACT',
    status: readModel.rows.every((row) => row.authorityRuntimeArtifactSetComplete)
      ? 'artifact_set_present'
      : 'manual_required',
    readModel,
    summary: {
      rowCount: readModel.rows.length,
      completeRows: readModel.rows.filter((row) => row.authorityRuntimeArtifactSetComplete).length,
      manualRequiredRows: readModel.rows.filter((row) => !row.authorityRuntimeArtifactSetComplete).length,
      missingArtifactCount: readModel.rows.reduce((total, row) => total + row.missingArtifacts.length, 0),
      requiredArtifactCount: readModel.rows.reduce((total, row) => total + row.requiredArtifacts.length, 0),
      authorityEnrollmentClaimedRows: readModel.rows.filter((row) => row.authorityEnrollmentClaimed).length,
      hardControlRuntimeClaimedRows: readModel.rows.filter((row) => row.hardControlRuntimeClaimed).length,
      productReadyRows: readModel.rows.filter((row) => row.productClaimReady).length,
    },
    proofLabels: [
      'tracking-authority-runtime.artifact-gate',
      'tracking-authority-runtime.required-artifacts-from-runtime-blockers',
      'tracking-authority-runtime.no-authority-runtime-claim-from-file-presence',
      'tracking-authority-runtime.product-ready-false',
    ],
    productClaims: readModel.productClaims,
    missingProofReason:
      'Actual authority runtime still requires enrolled Android/iOS/desktop authority evidence, hard-control runtime, parent-visible authority status, and physical-device proof artifacts. This gate validates artifact presence only and keeps authority enrollment, hard-control runtime, parent-visible authority status, physical-device, provider delivery, production worker, and product-ready claims false.',
    commands,
  };
}

function assertProof(proof) {
  assert.equal(proof.summary.rowCount, 1, 'expected one authority runtime artifact gate row');
  assert.equal(proof.summary.requiredArtifactCount, 20, 'expected authority runtime artifact refs from five modes');
  assert.equal(proof.summary.authorityEnrollmentClaimedRows, 0, 'no authority enrollment claims');
  assert.equal(proof.summary.hardControlRuntimeClaimedRows, 0, 'no hard-control runtime claims');
  assert.equal(proof.summary.productReadyRows, 0, 'no product-ready rows');
  assert.deepEqual(Object.values(proof.productClaims), [false, false, false, false, false, false, false]);
}

async function writeArtifacts(proof) {
  await writeJson(path.join(resultDir, 'proof.json'), proof);
  await writeJson(path.join(resultDir, 'read-model.json'), proof.readModel);
  await writeJson(path.join(focusedProofDir, 'proof.json'), proof);
  await writeJson(path.join(focusedProofDir, 'read-model.json'), proof.readModel);
  await writeFile(
    path.join(focusedProofDir, '00-source-snapshot.md'),
    [
      '# Tracking Authority Runtime Artifact Gate Source Snapshot',
      '',
      `- generatedAt: ${generatedAt}`,
      `- commit: ${proof.commit}`,
      `- status: ${proof.status}`,
      `- requiredArtifactCount: ${proof.summary.requiredArtifactCount}`,
      `- missingArtifactCount: ${proof.summary.missingArtifactCount}`,
      '- required artifact refs are derived from the authority runtime-readiness blocker proof',
      '- artifact presence alone does not claim authority enrollment, hard-control runtime, parent-visible authority status, physical-device behavior, or product readiness',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeJson(path.join(output31, '23-authority-runtime-artifact-gate-proof.json'), proof);
  await writeJson(path.join(output33, '61-authority-runtime-artifact-gate-proof.json'), proof);
  await writeFile(
    path.join(output31, '23-authority-runtime-artifact-gate-validation.log'),
    `${commands.map((entry) => entry.command).join('\n')}\n`,
    'utf8'
  );
  await writeFile(
    path.join(output33, '61-authority-runtime-artifact-gate-validation.log'),
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
