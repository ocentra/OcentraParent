import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdir, stat, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { tsImport } from 'tsx/esm/api';
import { runNpmCommand } from './run-npm-command.mjs';

const repoRoot = process.cwd();
const proofMode = 'tracking-child-runtime-artifact-gate-proof';
const resultDir = path.join(repoRoot, 'test-results', proofMode);
const output30 = path.join(repoRoot, 'output', 'tracking-plan-proof', '30-parent-and-child-ui-ux-surfaces');
const output33 = path.join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const commands = [];

await main();

async function main() {
  await mkdir(resultDir, { recursive: true });
  await mkdir(output30, { recursive: true });
  await mkdir(output33, { recursive: true });

  runNpmCommand(run, [
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/tracking-domain',
    '--',
    'tests/contract/tracking-child-runtime-artifact-gate-proof.test.ts',
  ]);

  const proofModule = await tsImport(
    pathToFileURL(
      path.join(repoRoot, 'packages', 'tracking-domain', 'src', 'tracking-child-runtime-artifact-gate-proof.ts')
    ).href,
    import.meta.url
  );
  const generatedAt = '2026-06-07T18:55:00.000Z';
  const inventory = {
    presentArtifacts: await presentArtifactsForRoot(
      path.join(repoRoot, proofModule.RequiredTrackingChildRuntimeArtifactPlan.proofRoot),
      proofModule.RequiredTrackingChildRuntimeArtifactPlan.requiredArtifacts
    ),
  };
  const readModel = proofModule.buildTrackingChildRuntimeArtifactGateProof(generatedAt, inventory);
  const proof = buildProof({ generatedAt, readModel });

  assertProof(proof);
  await writeArtifacts(proof);

  console.log('tracking-child-runtime-artifact-gate-proof-ok');
  console.log(`evidence=${relativePath(path.join(resultDir, 'proof.json'))}`);
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
    workpackIds: ['30-parent-and-child-ui-ux-surfaces', '33-proof-gates-fixtures-rollout-and-pr-gate'],
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    status: readModel.rows.every((row) => row.childRuntimeArtifactSetComplete)
      ? 'artifact_set_present'
      : 'manual_required',
    readModel,
    summary: {
      rowCount: readModel.rows.length,
      completeRows: readModel.rows.filter((row) => row.childRuntimeArtifactSetComplete).length,
      manualRequiredRows: readModel.rows.filter((row) => !row.childRuntimeArtifactSetComplete).length,
      missingArtifactCount: readModel.rows.reduce((total, row) => total + row.missingArtifacts.length, 0),
      childDeviceDeliveryRuntimeClaimedRows: readModel.rows.filter((row) => row.childDeviceDeliveryRuntimeClaimed)
        .length,
      childDeviceExecutionRuntimeClaimedRows: readModel.rows.filter((row) => row.childDeviceExecutionRuntimeClaimed)
        .length,
      renderedChildDeviceUiRuntimeClaimedRows: readModel.rows.filter((row) => row.renderedChildDeviceUiRuntimeClaimed)
        .length,
      productReadyRows: readModel.rows.filter((row) => row.productClaimReady).length,
    },
    proofLabels: [
      'tracking-child-runtime.artifact-gate',
      'tracking-child-runtime.delivery-execution-artifact-requirements',
      'tracking-child-runtime.no-runtime-claim-from-file-presence',
      'tracking-child-runtime.product-ready-false',
    ],
    productClaims: readModel.productClaims,
    missingProofReason:
      'Actual child-device delivery/execution still requires a real child-device run with delivery envelope, execution result, rendered child UI snapshot, parent receipt, runtime observation, consent state, and device logs. This gate validates required artifact presence only and keeps runtime, physical-device, authority, provider delivery, production, and product-ready claims false.',
    commands,
  };
}

function assertProof(proof) {
  assert.equal(proof.summary.rowCount, 1, 'expected one child runtime artifact gate row');
  assert.equal(proof.summary.childDeviceDeliveryRuntimeClaimedRows, 0, 'no child-device delivery runtime claims');
  assert.equal(proof.summary.childDeviceExecutionRuntimeClaimedRows, 0, 'no child-device execution runtime claims');
  assert.equal(proof.summary.renderedChildDeviceUiRuntimeClaimedRows, 0, 'no rendered child UI runtime claims');
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
  ]);
}

async function writeArtifacts(proof) {
  await writeJson(path.join(resultDir, 'proof.json'), proof);
  await writeJson(path.join(resultDir, 'read-model.json'), proof.readModel);
  await writeJson(path.join(output30, '30-child-runtime-artifact-gate-proof.json'), proof);
  await writeJson(path.join(output33, '50-child-runtime-artifact-gate-proof.json'), proof);
  await writeFile(
    path.join(output33, '50-child-runtime-artifact-gate-validation-commands.log'),
    `${commands.map((entry) => entry.command).join('\n')}\n`,
    'utf8'
  );
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
