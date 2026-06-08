import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdir, stat, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'tracking-full-product-ui-runtime-artifact-gate-proof';
const generatedAt = '2026-06-07T23:10:00.000Z';
const resultDir = path.join(repoRoot, 'test-results', proofMode);
const focusedProofDir = path.join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const output30 = path.join(repoRoot, 'output', 'tracking-plan-proof', '30-parent-and-child-ui-ux-surfaces');
const output33 = path.join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const commands = [];

await main();

async function main() {
  await mkdir(resultDir, { recursive: true });
  await mkdir(focusedProofDir, { recursive: true });
  await mkdir(output30, { recursive: true });
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
    'tracking-full-product-ui-runtime-artifact-gate-proof',
  ]);

  const proofModule = await importDist('tracking-full-product-ui-runtime-artifact-gate-proof.js');
  const inventory = {
    presentArtifacts: await presentArtifactsForRoot(proofModule.RequiredTrackingFullProductUiRuntimeArtifactPlan),
  };
  const readModel = proofModule.buildTrackingFullProductUiRuntimeArtifactGateProof(generatedAt, inventory);
  const proof = buildProof({ readModel });

  assertProof(proof);
  await writeArtifacts(proof);

  console.log('tracking-full-product-ui-runtime-artifact-gate-proof-ok');
  console.log(`evidence=${relativePath(path.join(resultDir, 'proof.json'))}`);
}

async function presentArtifactsForRoot(plan) {
  const present = [];
  for (const artifact of plan.requiredArtifacts) {
    const artifactPath = path.join(repoRoot, artifact);
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
    workpackIds: ['30-parent-and-child-ui-ux-surfaces', '33-proof-gates-fixtures-rollout-and-pr-gate'],
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    currentProofTier: 'P2_HOSTED_CI',
    status: readModel.rows.every((row) => row.fullProductUiArtifactSetComplete)
      ? 'artifact_set_present'
      : 'manual_required',
    readModel,
    summary: {
      rowCount: readModel.rows.length,
      completeRows: readModel.rows.filter((row) => row.fullProductUiArtifactSetComplete).length,
      manualRequiredRows: readModel.rows.filter((row) => !row.fullProductUiArtifactSetComplete).length,
      missingArtifactCount: readModel.rows.reduce((total, row) => total + row.missingArtifacts.length, 0),
      requiredArtifactCount: readModel.rows.reduce((total, row) => total + row.requiredArtifacts.length, 0),
      parentOverviewRuntimeUiClaimedRows: readModel.rows.filter((row) => row.parentOverviewRuntimeUiClaimed).length,
      renderedChildDeviceRuntimeUiClaimedRows: readModel.rows.filter((row) => row.renderedChildDeviceRuntimeUiClaimed)
        .length,
      productionProductUiClaimedRows: readModel.rows.filter((row) => row.productionProductUiClaimed).length,
      productReadyRows: readModel.rows.filter((row) => row.productClaimReady).length,
    },
    proofLabels: [
      'tracking-full-product-ui-runtime.artifact-gate',
      'tracking-full-product-ui-runtime.required-artifacts-from-blocker-source',
      'tracking-full-product-ui-runtime.no-product-claim-from-file-presence',
      'tracking-full-product-ui-runtime.product-ready-false',
    ],
    productClaims: readModel.productClaims,
    missingProofReason:
      'Actual full parent/child tracking UI still requires product runtime artifacts for parent overview, parent device detail, notification history/preferences, production retention settings write result, rendered child-device check-in, child-device location consent, child-device safe/help response, cross-surface accessibility, and end-to-end product UI trace. This gate validates required artifact presence only and keeps full product UI, child-device delivery runtime, physical-device, authority, provider delivery, production product UI, and product-ready claims false.',
    commands,
  };
}

function assertProof(proof) {
  assert.equal(proof.summary.rowCount, 1, 'expected one full product UI artifact gate row');
  assert.equal(proof.summary.requiredArtifactCount, 9, 'expected all required full product UI artifact refs');
  assert.equal(proof.summary.parentOverviewRuntimeUiClaimedRows, 0, 'no parent overview runtime UI claims');
  assert.equal(proof.summary.renderedChildDeviceRuntimeUiClaimedRows, 0, 'no rendered child runtime UI claims');
  assert.equal(proof.summary.productionProductUiClaimedRows, 0, 'no production product UI claims');
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
      '# Tracking Full Product UI Runtime Artifact Gate Source Snapshot',
      '',
      `- generatedAt: ${generatedAt}`,
      `- commit: ${proof.commit}`,
      `- status: ${proof.status}`,
      `- requiredArtifactCount: ${proof.summary.requiredArtifactCount}`,
      `- missingArtifactCount: ${proof.summary.missingArtifactCount}`,
      '- required artifact refs are imported from tracking full product UI readiness blocker source',
      '- artifact presence alone does not claim full product UI runtime or product readiness',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeJson(path.join(output30, '32-full-product-ui-runtime-artifact-gate-proof.json'), proof);
  await writeJson(path.join(output33, '59-full-product-ui-runtime-artifact-gate-proof.json'), proof);
  await writeFile(
    path.join(output33, '59-full-product-ui-runtime-artifact-gate-validation-commands.log'),
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
