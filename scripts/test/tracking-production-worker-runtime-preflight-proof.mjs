import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'tracking-production-worker-runtime-preflight-proof';
const generatedAt = '2026-06-08T13:10:00.000Z';
const resultDir = path.join(repoRoot, 'test-results', proofMode);
const focusedProofDir = path.join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const output33 = path.join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await main();

async function main() {
  await mkdir(resultDir, { recursive: true });
  await mkdir(focusedProofDir, { recursive: true });
  await mkdir(output33, { recursive: true });

  run('node', ['scripts/test/tracking-production-worker-runtime-artifact-gate-proof.mjs']);
  run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  run('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'tracking-production-worker-runtime-preflight-proof',
    'tracking-production-worker-runtime-artifact-gate-proof',
  ]);

  const proofModule = await importDist('tracking-production-worker-runtime-preflight-proof.js');
  const runtimeGateProof = JSON.parse(
    await readFile(
      path.join(
        repoRoot,
        proofModule.RequiredTrackingProductionWorkerRuntimePreflightPlan.sourceRuntimeArtifactGateProofRef
      ),
      'utf8'
    )
  );
  const readModel = proofModule.buildTrackingProductionWorkerRuntimePreflightProof(
    generatedAt,
    runtimeGateProof.readModel
  );
  const proof = buildProof({ readModel, runtimeGateProof });

  assertProof(proof);
  await writeArtifacts(proof);

  console.log('tracking-production-worker-runtime-preflight-proof-ok');
  console.log(`evidence=${relativePath(path.join(resultDir, 'proof.json'))}`);
}

function buildProof({ readModel, runtimeGateProof }) {
  return {
    schemaVersion: 1,
    proofMode,
    generatedAt,
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    commit: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: initialGitStatusShort,
    workpackIds: ['33-proof-gates-fixtures-rollout-and-pr-gate'],
    requiredProofTier: 'P4_PRODUCTION_RUNTIME',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    status: 'manual_required',
    sourceRuntimeArtifactGateProofRef: readModel.sourceRuntimeArtifactGateProofRef,
    sourceRuntimeArtifactGateStatus: runtimeGateProof.status,
    sourceRuntimeArtifactGateMissingArtifactCount: runtimeGateProof.summary.missingArtifactCount,
    readModel,
    summary: readModel.summary,
    proofLabels: [
      'tracking-production-worker-runtime.preflight',
      'tracking-production-worker-runtime.production-artifacts-manual-required',
      'tracking-production-worker-runtime.acceptance-rows-generated',
      'tracking-production-worker-runtime.product-ready-false',
    ],
    productClaims: readModel.productClaims,
    missingProofReason:
      'Tracking production worker runtime still requires location upload, retention cleanup, notification outbox, escalation timeout, provider receipt, child-device delivery, authority status, and audit durable storage artifacts. This preflight defines required acceptance rows and artifact paths only; it does not claim production worker execution, physical-device proof, authority proof, provider delivery receipt runtime, or product readiness.',
    commands,
  };
}

function assertProof(proof) {
  assert.equal(proof.summary.rowCount, 8, 'expected eight production worker runtime preflight rows');
  assert.equal(proof.summary.manualRequiredRowCount, 8, 'expected all production worker rows manual-required');
  assert.equal(proof.summary.requiredArtifactCount, 8, 'expected eight production worker runtime artifacts');
  assert.equal(proof.summary.presentArtifactCount, 0, 'preflight must not mark production artifacts present');
  assert.equal(proof.summary.productReadyRowCount, 0, 'preflight must not include product-ready rows');
  assert.equal(proof.productClaims.productionWorkersClaimed, false);
  assert.equal(proof.productClaims.productClaimReady, false);
  for (const row of proof.readModel.rows) {
    assert.ok(row.acceptanceCriteria.length >= 3, `${row.rowId} acceptance criteria missing`);
    assert.ok(row.manualValidationCommands.length >= 2, `${row.rowId} validation commands missing`);
    assert.ok(row.missingArtifacts.length > 0, `${row.rowId} missing artifact refs missing`);
  }
}

async function writeArtifacts(proof) {
  await writeJson(path.join(resultDir, 'proof.json'), proof);
  await writeJson(path.join(resultDir, 'read-model.json'), proof.readModel);
  await writeJson(path.join(focusedProofDir, 'proof.json'), proof);
  await writeJson(path.join(focusedProofDir, 'read-model.json'), proof.readModel);
  await writeFile(
    path.join(focusedProofDir, '00-source-snapshot.md'),
    [
      '# Tracking Production Worker Runtime Preflight Source Snapshot',
      '',
      `- generatedAt: ${generatedAt}`,
      `- commit: ${proof.commit}`,
      `- status: ${proof.status}`,
      `- rowCount: ${proof.summary.rowCount}`,
      `- missingArtifactCount: ${proof.summary.missingArtifactCount}`,
      '- production worker runtime artifacts remain manual-required',
      '- production worker, authority, provider delivery receipt, and product-ready claims remain false',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(path.join(focusedProofDir, 'manual-validation-runbook.md'), manualValidationRunbook(proof), 'utf8');
  await writeJson(path.join(output33, '72-production-worker-runtime-preflight-proof.json'), proof);
  await writeFile(
    path.join(output33, '72-production-worker-runtime-preflight-validation.log'),
    `${commands.map((entry) => entry.command).join('\n')}\n`,
    'utf8'
  );
}

function manualValidationRunbook(proof) {
  const lines = [
    '# Tracking Production Worker Runtime Preflight Manual Runbook',
    '',
    `- generatedAt: ${generatedAt}`,
    `- status: ${proof.status}`,
    '- This runbook is not production/product-ready proof. It names the production worker artifacts still missing.',
    '',
  ];
  for (const row of proof.readModel.rows) {
    lines.push(`## ${row.area}`, '');
    lines.push('Acceptance criteria:');
    for (const criterion of row.acceptanceCriteria) lines.push(`- ${criterion}`);
    lines.push('', 'Manual commands:');
    for (const command of row.manualValidationCommands) lines.push(`- ${command}`);
    lines.push('', 'Required artifacts:');
    for (const artifact of row.requiredArtifacts) lines.push(`- ${artifact}`);
    lines.push('');
  }
  return `${lines.join('\n')}\n`;
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
