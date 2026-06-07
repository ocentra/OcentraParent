import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdir, readFile, stat, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'tracking-retention-runtime-artifact-gate-proof';
const generatedAt = '2026-06-07T23:45:00.000Z';
const resultDir = path.join(repoRoot, 'test-results', proofMode);
const focusedProofDir = path.join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const output07 = path.join(repoRoot, 'output', 'tracking-plan-proof', '07-retention-and-custody-model');
const output33 = path.join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const commands = [];

await main();

async function main() {
  await mkdir(resultDir, { recursive: true });
  await mkdir(focusedProofDir, { recursive: true });
  await mkdir(output07, { recursive: true });
  await mkdir(output33, { recursive: true });

  run('node', ['scripts/test/tracking-retention-product-readiness-proof.mjs']);
  run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  run('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'tracking-retention-runtime-artifact-gate-proof',
  ]);

  const proofModule = await importDist('tracking-retention-runtime-artifact-gate-proof.js');
  await assertSourceProductReadinessProofExists(
    proofModule.RequiredTrackingRetentionRuntimeArtifactPlan.sourceProductReadinessProofRef
  );
  const inventory = {
    presentArtifacts: await presentArtifactsForRoot(
      path.join(repoRoot, proofModule.RequiredTrackingRetentionRuntimeArtifactPlan.proofRoot),
      proofModule.RequiredTrackingRetentionRuntimeArtifactPlan.requiredArtifacts
    ),
  };
  const readModel = proofModule.buildTrackingRetentionRuntimeArtifactGateProof(generatedAt, inventory);
  const proof = buildProof({ readModel });

  assertProof(proof);
  await writeArtifacts(proof);

  console.log('tracking-retention-runtime-artifact-gate-proof-ok');
  console.log(`evidence=${relativePath(path.join(resultDir, 'proof.json'))}`);
}

async function assertSourceProductReadinessProofExists(proofRef) {
  const sourceProof = JSON.parse(await readFile(path.join(repoRoot, proofRef), 'utf8'));
  assert.equal(sourceProof.proofMode, 'tracking-retention-product-readiness-proof');
  assert.equal(sourceProof.productClaims.productClaimReady, false);
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
    workpackIds: ['07-retention-and-custody-model', '33-proof-gates-fixtures-rollout-and-pr-gate'],
    requiredProofTier: 'P4_PRODUCTION_RUNTIME',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    status: readModel.rows.every((row) => row.retentionRuntimeArtifactSetComplete)
      ? 'artifact_set_present'
      : 'manual_required',
    readModel,
    summary: {
      rowCount: readModel.rows.length,
      completeRows: readModel.rows.filter((row) => row.retentionRuntimeArtifactSetComplete).length,
      manualRequiredRows: readModel.rows.filter((row) => !row.retentionRuntimeArtifactSetComplete).length,
      missingArtifactCount: readModel.rows.reduce((total, row) => total + row.missingArtifacts.length, 0),
      requiredArtifactCount: readModel.rows.reduce((total, row) => total + row.requiredArtifacts.length, 0),
      writableProductSettingsExecutionClaimedRows: readModel.rows.filter(
        (row) => row.writableProductSettingsExecutionClaimed
      ).length,
      platformRuntimeRetentionEnforcementClaimedRows: readModel.rows.filter(
        (row) => row.platformRuntimeRetentionEnforcementClaimed
      ).length,
      productReadyRows: readModel.rows.filter((row) => row.productClaimReady).length,
    },
    proofLabels: [
      'tracking-retention-runtime.artifact-gate',
      'tracking-retention-runtime.required-artifacts-from-product-readiness-blockers',
      'tracking-retention-runtime.no-platform-runtime-claim-from-file-presence',
      'tracking-retention-runtime.product-ready-false',
    ],
    productClaims: readModel.productClaims,
    missingProofReason:
      'Actual retention product runtime still requires writable product settings execution and platform runtime retention enforcement artifacts. This gate validates artifact presence only and keeps writable product settings execution, platform runtime retention enforcement, child-device delivery, provider delivery, notification receipts, physical-device, authority, production worker, and product-ready claims false.',
    commands,
  };
}

function assertProof(proof) {
  assert.equal(proof.summary.rowCount, 1, 'expected one retention runtime artifact gate row');
  assert.equal(proof.summary.requiredArtifactCount, 2, 'expected both retention runtime artifact refs');
  assert.equal(proof.summary.writableProductSettingsExecutionClaimedRows, 0, 'no writable product settings claims');
  assert.equal(proof.summary.platformRuntimeRetentionEnforcementClaimedRows, 0, 'no platform runtime retention claims');
  assert.equal(proof.summary.productReadyRows, 0, 'no product-ready rows');
  assert.deepEqual(Object.values(proof.productClaims), [false, false, false, false, false, false, false, false, false]);
}

async function writeArtifacts(proof) {
  await writeJson(path.join(resultDir, 'proof.json'), proof);
  await writeJson(path.join(resultDir, 'read-model.json'), proof.readModel);
  await writeJson(path.join(focusedProofDir, 'proof.json'), proof);
  await writeJson(path.join(focusedProofDir, 'read-model.json'), proof.readModel);
  await writeFile(
    path.join(focusedProofDir, '00-source-snapshot.md'),
    [
      '# Tracking Retention Runtime Artifact Gate Source Snapshot',
      '',
      `- generatedAt: ${generatedAt}`,
      `- commit: ${proof.commit}`,
      `- status: ${proof.status}`,
      `- requiredArtifactCount: ${proof.summary.requiredArtifactCount}`,
      `- missingArtifactCount: ${proof.summary.missingArtifactCount}`,
      '- required artifact refs are tied to retention product-readiness blocker source',
      '- artifact presence alone does not claim writable retention settings, platform runtime retention enforcement, or product readiness',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeJson(path.join(output07, '25-retention-runtime-artifact-gate-proof.json'), proof);
  await writeJson(path.join(output33, '60-retention-runtime-artifact-gate-proof.json'), proof);
  await writeFile(
    path.join(output07, '25-retention-runtime-artifact-gate-validation.log'),
    `${commands.map((entry) => entry.command).join('\n')}\n`,
    'utf8'
  );
  await writeFile(
    path.join(output33, '60-retention-runtime-artifact-gate-validation.log'),
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
