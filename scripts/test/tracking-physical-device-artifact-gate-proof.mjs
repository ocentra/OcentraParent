import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdir, stat, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'tracking-physical-device-artifact-gate-proof';
const resultDir = path.join(repoRoot, 'test-results', proofMode);
const output33 = path.join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const commands = [];

await main();

async function main() {
  await mkdir(resultDir, { recursive: true });
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
    'tracking-physical-device-artifact-gate-proof',
  ]);

  const proofModule = await importDist('tracking-physical-device-artifact-gate-proof.js');
  const generatedAt = '2026-06-07T18:20:00.000Z';
  const inventories = await collectInventories(proofModule.RequiredTrackingPhysicalDeviceArtifactPlans);
  const readModel = proofModule.buildTrackingPhysicalDeviceArtifactGateProof(generatedAt, inventories);
  const proof = buildProof({ generatedAt, readModel });

  assertProof(proof);
  await writeArtifacts(proof);

  console.log('tracking-physical-device-artifact-gate-proof-ok');
  console.log(`evidence=${relativePath(path.join(resultDir, 'proof.json'))}`);
}

async function collectInventories(plans) {
  return Promise.all(
    plans.map(async (plan) => ({
      platform: plan.platform,
      presentArtifacts: await presentArtifactsForRoot(path.join(repoRoot, plan.proofRoot), plan.requiredArtifacts),
    }))
  );
}

async function presentArtifactsForRoot(rootPath, requiredArtifacts) {
  const present = [];
  for (const artifact of requiredArtifacts) {
    const artifactPath = path.join(rootPath, artifact);
    if (await pathExists(artifactPath)) {
      present.push(artifact);
    }
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

function buildProof({ generatedAt, readModel }) {
  return {
    schemaVersion: 1,
    proofMode,
    generatedAt,
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    commit: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: gitOutput(['status', '--short']),
    workpackId: '33-proof-gates-fixtures-rollout-and-pr-gate',
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    status: readModel.rows.every((row) => row.physicalArtifactSetComplete) ? 'artifact_set_present' : 'manual_required',
    readModel,
    summary: {
      rowCount: readModel.rows.length,
      completeRows: readModel.rows.filter((row) => row.physicalArtifactSetComplete).length,
      manualRequiredRows: readModel.rows.filter((row) => !row.physicalArtifactSetComplete).length,
      missingArtifactCount: readModel.rows.reduce((total, row) => total + row.missingArtifacts.length, 0),
      physicalDeviceBehaviorClaimedRows: readModel.rows.filter((row) => row.physicalDeviceBehaviorClaimed).length,
      productReadyRows: readModel.rows.filter((row) => row.productClaimReady).length,
    },
    proofLabels: [
      'tracking-physical-device.android-artifact-gate',
      'tracking-physical-device.ios-artifact-gate',
      'tracking-physical-device.no-behavior-claim-from-file-presence',
      'tracking-physical-device.product-ready-false',
    ],
    productClaims: readModel.productClaims,
    missingProofReason:
      'Android and iOS physical-device behavior still require real-device runs and review of the collected artifacts. This gate validates required artifact presence only and keeps behavior, authority, provider delivery, production, and product-ready claims false.',
    commands,
  };
}

function assertProof(proof) {
  assert.equal(proof.summary.rowCount, 2, 'expected Android and iOS physical artifact rows');
  assert.equal(proof.summary.physicalDeviceBehaviorClaimedRows, 0, 'no physical-device behavior claims');
  assert.equal(proof.summary.productReadyRows, 0, 'no product-ready rows');
  assert.deepEqual(Object.values(proof.productClaims), [false, false, false, false, false]);
}

async function writeArtifacts(proof) {
  await writeJson(path.join(resultDir, 'proof.json'), proof);
  await writeJson(path.join(resultDir, 'read-model.json'), proof.readModel);
  await writeJson(path.join(output33, '49-physical-device-artifact-gate-proof.json'), proof);
  await writeFile(
    path.join(output33, '49-physical-device-artifact-gate-validation-commands.log'),
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
  if (result.status !== 0) {
    throw new Error(`Command failed: ${command} ${args.join(' ')}`);
  }
}

function gitOutput(args) {
  return spawnSync('git', args, { cwd: repoRoot, encoding: 'utf8' }).stdout.trim();
}

async function writeJson(filePath, value) {
  await mkdir(path.dirname(filePath), { recursive: true });
  await writeFile(filePath, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}

function relativePath(filePath) {
  return path.relative(repoRoot, filePath).replaceAll(path.sep, '/');
}
