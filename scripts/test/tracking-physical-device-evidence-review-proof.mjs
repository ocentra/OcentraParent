import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { tsImport } from 'tsx/esm/api';
import { runNpmCommand } from './run-npm-command.mjs';

const repoRoot = process.cwd();
const proofMode = 'tracking-physical-device-evidence-review-proof';
const resultDir = path.join(repoRoot, 'test-results', proofMode);
const namedProofRoot = path.join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const output33 = path.join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const sourceGateProofRef = 'test-results/tracking-physical-device-artifact-gate-proof/proof.json';
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
    'tests/contract/tracking-physical-device-evidence-review-proof.test.ts',
  ]);

  const proofModule = await tsImport(
    pathToFileURL(
      path.join(repoRoot, 'packages', 'tracking-domain', 'src', 'tracking-physical-device-evidence-review-proof.ts')
    ).href,
    import.meta.url
  );
  const artifactGateProof = (await readJson(sourceGateProofRef)).readModel;
  const generatedAt = '2026-06-08T14:25:00.000Z';
  const readModel = proofModule.buildTrackingPhysicalDeviceEvidenceReviewProof(generatedAt, artifactGateProof);
  const proof = buildProof({ generatedAt, readModel });

  assertProof(proof);
  await writeArtifacts(proof);

  console.log('tracking-physical-device-evidence-review-proof-ok');
  console.log(`evidence=${relativePath(path.join(resultDir, 'proof.json'))}`);
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
    requiredProofTier: 'P4_PHYSICAL_DEVICE_CONTENT_REVIEW',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    currentStatus: readModel.summary.artifactMissingRows > 0 ? 'artifact_missing' : 'content_review_required',
    sourceGateProofRef,
    readModel,
    rows: readModel.rows,
    summary: readModel.summary,
    productClaims: readModel.productClaims,
    proofLabels: [
      'tracking-physical-device.review-android',
      'tracking-physical-device.review-ios',
      'tracking-physical-device.file-presence-is-not-content-approval',
      'tracking-physical-device.product-ready-false',
    ],
    missingProofReason:
      'Android and iOS physical-device artifact files still require content review before any behavior claim. Missing files keep rows artifact-missing; complete file sets move only to content-review-required. This proof keeps authority, provider, production, and product-ready claims false.',
    commands,
  };
}

function assertProof(proof) {
  assert.equal(proof.summary.rowCount, 2, 'expected Android and iOS review rows');
  assert.equal(proof.summary.contentAcceptedRows, 0, 'no physical artifact content accepted');
  assert.equal(proof.summary.physicalDeviceBehaviorClaimedRows, 0, 'no physical-device behavior claims');
  assert.equal(proof.summary.productReadyRows, 0, 'no product-ready rows');
  assert.ok(
    proof.summary.physicalDeviceStatusObservedRows <= 1,
    'only Android physical status support can be local on this host'
  );
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
}

async function writeArtifacts(proof) {
  await writeJson(path.join(resultDir, 'proof.json'), proof);
  await writeJson(path.join(resultDir, 'read-model.json'), proof.readModel);
  await writeJson(path.join(namedProofRoot, 'proof.json'), proof);
  await writeFile(path.join(namedProofRoot, '00-source-snapshot.md'), sourceSnapshot(proof), 'utf8');
  await writeFile(path.join(namedProofRoot, 'manual-review-runbook.md'), manualReviewRunbook(proof), 'utf8');
  await writeJson(path.join(output33, '73-physical-device-evidence-review-proof.json'), proof);
  await writeFile(
    path.join(output33, '73-physical-device-evidence-review-validation-commands.log'),
    validationLog(),
    'utf8'
  );
}

function sourceSnapshot(proof) {
  return [
    '# Tracking Physical Device Evidence Review Proof',
    '',
    `- generatedAt: ${proof.generatedAt}`,
    `- commit: ${proof.commit}`,
    '- requiredProofTier: P4_PHYSICAL_DEVICE_CONTENT_REVIEW',
    '- currentProofTier: P3_LOCAL_DEV_MACHINE',
    `- currentStatus: ${proof.currentStatus}`,
    `- sourceGateProofRef: ${proof.sourceGateProofRef}`,
    `- rowCount: ${proof.summary.rowCount}`,
    `- artifactMissingRows: ${proof.summary.artifactMissingRows}`,
    `- contentReviewRequiredRows: ${proof.summary.contentReviewRequiredRows}`,
    `- contentAcceptedRows: ${proof.summary.contentAcceptedRows}`,
    `- physicalDeviceStatusObservedRows: ${proof.summary.physicalDeviceStatusObservedRows}`,
    `- supportingStatusArtifactCount: ${proof.summary.supportingStatusArtifactCount}`,
    '- physicalDeviceBehaviorClaimedRows: 0',
    '- productReadyRows: 0',
    '- proof module: packages/tracking-domain/src/tracking-physical-device-evidence-review-proof.ts',
    '- proof tests: packages/tracking-domain/tests/contract/tracking-physical-device-evidence-review-proof.test.ts',
    '- proof harness: scripts/test/tracking-physical-device-evidence-review-proof.mjs',
    '',
  ].join('\n');
}

function manualReviewRunbook(proof) {
  const lines = [
    '# Tracking Physical Device Evidence Review Runbook',
    '',
    'Use this after the required Android/iOS physical-device artifact files exist. This runbook is a content review gate, not a product-claim approval gate.',
    '',
  ];

  for (const row of proof.rows) {
    lines.push(`## ${row.platform}`);
    lines.push('');
    lines.push(`- proofRoot: ${row.proofRoot}`);
    lines.push(`- status: ${row.status}`);
    lines.push(`- artifactSetComplete: ${row.artifactSetComplete}`);
    lines.push(`- physicalDeviceStatusObserved: ${row.physicalDeviceStatusObserved}`);
    lines.push(`- supportingStatusProofRef: ${row.supportingStatusProofRef}`);
    lines.push(`- supportingStatusArtifacts: ${row.supportingStatusArtifacts.length}`);
    lines.push(`- contentAccepted: ${row.contentAccepted}`);
    lines.push('');
    lines.push('Review criteria:');
    for (const criterion of row.acceptanceCriteria) lines.push(`- ${criterion}`);
    lines.push('');
    lines.push('Commands to reproduce or inspect:');
    for (const command of row.manualValidationCommands) lines.push(`- ${command}`);
    lines.push('');
    lines.push('Review notes:');
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

function validationLog() {
  return `${commands.map((entry) => `${entry.command} exit=${entry.status}`).join('\n')}\n`;
}

async function readJson(relativePathValue) {
  return JSON.parse(await readFile(path.join(repoRoot, relativePathValue), 'utf8'));
}

function run(command, args) {
  const printable = [command, ...args].join(' ');
  const result = spawnSync(command, args, { cwd: repoRoot, encoding: 'utf8', shell: false });
  commands.push({
    command: printable,
    status: result.status,
    stdout: result.stdout.trim(),
    stderr: result.stderr.trim(),
  });
  if (result.status !== 0) {
    throw new Error(`${printable} failed\n${result.stdout}\n${result.stderr}`);
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
