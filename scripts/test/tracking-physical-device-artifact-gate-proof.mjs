import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdir, readFile, stat, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { tsImport } from 'tsx/esm/api';
import { runNpmCommand } from './run-npm-command.mjs';

const repoRoot = process.cwd();
const proofMode = 'tracking-physical-device-artifact-gate-proof';
const resultDir = path.join(repoRoot, 'test-results', proofMode);
const namedProofRoot = path.join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const output33 = path.join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const androidPhysicalStatusProofRef = 'test-results/tracking-android-physical-device-runtime-proof/proof.json';
const commands = [];

await main();

async function main() {
  await mkdir(resultDir, { recursive: true });
  await mkdir(namedProofRoot, { recursive: true });
  await mkdir(output33, { recursive: true });

  runNpmCommand(run, [
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/tracking-domain',
    '--',
    'tests/contract/tracking-physical-device-artifact-gate-proof.test.ts',
  ]);

  const proofModule = await tsImport(
    pathToFileURL(
      path.join(repoRoot, 'packages', 'schema-domain', 'src', 'tracking-physical-device-artifact-gate-proof.ts')
    ).href,
    import.meta.url
  );
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
  const androidPhysicalStatusProof = await readOptionalJson(androidPhysicalStatusProofRef);

  return Promise.all(
    plans.map(async (plan) => {
      const inventory = {
        platform: plan.platform,
        presentArtifacts: await presentArtifactsForRoot(path.join(repoRoot, plan.proofRoot), plan.requiredArtifacts),
      };

      if (plan.platform !== 'android' || !androidPhysicalStatusProof) {
        return inventory;
      }

      return {
        ...inventory,
        supportingStatusProofRef: androidPhysicalStatusProofRef,
        supportingStatusArtifacts: physicalStatusArtifactsFrom(androidPhysicalStatusProof),
      };
    })
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

async function readOptionalJson(relativePathValue) {
  try {
    return JSON.parse(await readFile(path.join(repoRoot, relativePathValue), 'utf8'));
  } catch (error) {
    if (error?.code === 'ENOENT') return null;
    throw error;
  }
}

function physicalStatusArtifactsFrom(proof) {
  const row = proof.rows?.[0];
  if (!row || row.physicalDeviceRuntimeObserved !== true) return [];
  if (
    row.physicalLocationRuntimeClaimed ||
    row.physicalGeofenceRuntimeClaimed ||
    row.androidSystemGeofenceDeliveryClaimed
  ) {
    throw new Error('Android physical status proof unexpectedly claims physical behavior');
  }

  return [...(row.presentArtifacts ?? [])];
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
      physicalDeviceStatusObservedRows: readModel.rows.filter((row) => row.physicalDeviceStatusObserved).length,
      supportingStatusArtifactCount: readModel.rows.reduce(
        (total, row) => total + row.supportingStatusArtifacts.length,
        0
      ),
      physicalDeviceBehaviorClaimedRows: readModel.rows.filter((row) => row.physicalDeviceBehaviorClaimed).length,
      productReadyRows: readModel.rows.filter((row) => row.productClaimReady).length,
      acceptanceCriteriaCount: readModel.rows.reduce((total, row) => total + row.acceptanceCriteria.length, 0),
      manualValidationCommandCount: readModel.rows.reduce(
        (total, row) => total + row.manualValidationCommands.length,
        0
      ),
      artifactAcceptanceNoteCount: readModel.rows.reduce((total, row) => total + row.artifactAcceptanceNotes.length, 0),
    },
    proofLabels: [
      'tracking-physical-device.android-artifact-gate',
      'tracking-physical-device.ios-artifact-gate',
      'tracking-physical-device.no-behavior-claim-from-file-presence',
      'tracking-physical-device.product-ready-false',
      'tracking-physical-device.manual-runbook-ready',
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
  assert.ok(proof.summary.physicalDeviceStatusObservedRows <= 1, 'only Android status support can be local on Windows');
  assert.ok(proof.summary.supportingStatusArtifactCount >= 0, 'supporting status artifacts are counted separately');
  assert.equal(proof.summary.acceptanceCriteriaCount, proof.summary.rowCount * 4, 'expected acceptance criteria');
  assert.equal(
    proof.summary.manualValidationCommandCount,
    proof.summary.rowCount * 4,
    'expected manual validation commands'
  );
  assert.equal(
    proof.summary.artifactAcceptanceNoteCount,
    proof.summary.rowCount * 4,
    'expected artifact acceptance notes'
  );
  assert.deepEqual(Object.values(proof.productClaims), [false, false, false, false, false]);
  assert.ok(
    proof.readModel.rows.every((row) =>
      row.artifactAcceptanceNotes.some((note) => note.includes('Product claims stay false'))
    ),
    'manual acceptance notes should keep product claims false'
  );
}

async function writeArtifacts(proof) {
  await writeJson(path.join(resultDir, 'proof.json'), proof);
  await writeJson(path.join(resultDir, 'read-model.json'), proof.readModel);
  await writeJson(path.join(namedProofRoot, 'proof.json'), proof);
  await writeFile(path.join(namedProofRoot, '00-source-snapshot.md'), sourceSnapshot(proof), 'utf8');
  await writeFile(path.join(namedProofRoot, 'manual-validation-runbook.md'), manualValidationRunbook(proof), 'utf8');
  await writeJson(path.join(output33, '49-physical-device-artifact-gate-proof.json'), proof);
  await writeFile(
    path.join(output33, '49-physical-device-artifact-gate-validation-commands.log'),
    `${commands.map((entry) => entry.command).join('\n')}\n`,
    'utf8'
  );
}

function sourceSnapshot(proof) {
  return [
    '# Tracking Physical Device Artifact Gate Proof',
    '',
    `- generatedAt: ${proof.generatedAt}`,
    `- commit: ${proof.commit}`,
    '- requiredProofTier: P4_PHYSICAL_DEVICE',
    '- currentProofTier: P3_LOCAL_DEV_MACHINE',
    `- status: ${proof.status}`,
    `- rowCount: ${proof.summary.rowCount}`,
    `- manualRequiredRows: ${proof.summary.manualRequiredRows}`,
    `- missingArtifactCount: ${proof.summary.missingArtifactCount}`,
    `- physicalDeviceStatusObservedRows: ${proof.summary.physicalDeviceStatusObservedRows}`,
    `- supportingStatusArtifactCount: ${proof.summary.supportingStatusArtifactCount}`,
    `- acceptanceCriteriaCount: ${proof.summary.acceptanceCriteriaCount}`,
    `- manualValidationCommandCount: ${proof.summary.manualValidationCommandCount}`,
    `- artifactAcceptanceNoteCount: ${proof.summary.artifactAcceptanceNoteCount}`,
    '- physicalDeviceBehaviorClaimedRows: 0',
    '- productReadyRows: 0',
    '- proof module: packages/schema-domain/src/tracking-physical-device-artifact-gate-proof.ts',
    '- proof tests: packages/tracking-domain/tests/contract/tracking-physical-device-artifact-gate-proof.test.ts',
    '- proof harness: scripts/test/tracking-physical-device-artifact-gate-proof.mjs',
    '',
  ].join('\n');
}

function manualValidationRunbook(proof) {
  const lines = [
    '# Tracking Physical Device Manual Validation Runbook',
    '',
    'This runbook names the physical-device artifacts required before Android or iOS tracking behavior can be claimed. File presence alone does not approve behavior; a reviewer must inspect artifact contents and keep authority, provider, production, and product-ready gates separate.',
    '',
  ];

  for (const row of proof.readModel.rows) {
    lines.push(`## ${row.platform}`);
    lines.push('');
    lines.push(`- proofRoot: ${row.proofRoot}`);
    lines.push(`- status: ${row.status}`);
    lines.push(`- missingArtifacts: ${row.missingArtifacts.length}`);
    lines.push(`- physicalDeviceStatusObserved: ${row.physicalDeviceStatusObserved}`);
    lines.push(`- supportingStatusProofRef: ${row.supportingStatusProofRef}`);
    lines.push(`- supportingStatusArtifacts: ${row.supportingStatusArtifacts.length}`);
    lines.push('');
    lines.push('Acceptance criteria:');
    for (const criterion of row.acceptanceCriteria) lines.push(`- ${criterion}`);
    lines.push('');
    lines.push('Manual validation commands:');
    for (const command of row.manualValidationCommands) lines.push(`- ${command}`);
    lines.push('');
    lines.push('Required artifacts:');
    for (const artifact of row.requiredArtifacts) lines.push(`- ${artifact}`);
    lines.push('');
    lines.push('Acceptance notes:');
    for (const note of row.artifactAcceptanceNotes) lines.push(`- ${note}`);
    if (row.supportingStatusArtifacts.length > 0) {
      lines.push('');
      lines.push('Supporting status artifacts:');
      for (const artifact of row.supportingStatusArtifacts) lines.push(`- ${artifact}`);
    }
    lines.push('');
  }

  return `${lines.join('\n')}\n`;
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
