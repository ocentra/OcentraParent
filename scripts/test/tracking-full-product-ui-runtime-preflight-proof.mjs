import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'tracking-full-product-ui-runtime-preflight-proof';
const generatedAt = '2026-06-08T12:30:00.000Z';
const resultDir = path.join(repoRoot, 'test-results', proofMode);
const focusedProofDir = path.join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const output30 = path.join(repoRoot, 'output', 'tracking-plan-proof', '30-parent-and-child-ui-ux-surfaces');
const output33 = path.join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await main();

async function main() {
  await mkdir(resultDir, { recursive: true });
  await mkdir(focusedProofDir, { recursive: true });
  await mkdir(output30, { recursive: true });
  await mkdir(output33, { recursive: true });

  run('node', ['scripts/test/tracking-full-product-ui-runtime-artifact-gate-proof.mjs']);
  run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  run('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'tracking-full-product-ui-runtime-preflight-proof',
    'tracking-full-product-ui-runtime-artifact-gate-proof',
  ]);

  const proofModule = await importDist('tracking-full-product-ui-runtime-preflight-proof.js');
  const runtimeGateProof = JSON.parse(
    await readFile(
      path.join(
        repoRoot,
        proofModule.RequiredTrackingFullProductUiRuntimePreflightPlan.sourceRuntimeArtifactGateProofRef
      ),
      'utf8'
    )
  );
  const readModel = proofModule.buildTrackingFullProductUiRuntimePreflightProof(
    generatedAt,
    runtimeGateProof.readModel
  );
  const proof = buildProof({ readModel, runtimeGateProof });

  assertProof(proof);
  await writeArtifacts(proof);

  console.log('tracking-full-product-ui-runtime-preflight-proof-ok');
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
    workpackIds: ['30-parent-and-child-ui-ux-surfaces', '33-proof-gates-fixtures-rollout-and-pr-gate'],
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    status: 'manual_required',
    sourceRuntimeArtifactGateProofRef: readModel.sourceRuntimeArtifactGateProofRef,
    sourceRuntimeArtifactGateStatus: runtimeGateProof.status,
    sourceRuntimeArtifactGateMissingArtifactCount: runtimeGateProof.summary.missingArtifactCount,
    readModel,
    summary: readModel.summary,
    proofLabels: [
      'tracking-full-product-ui-runtime.preflight',
      'tracking-full-product-ui-runtime.hard-ui-artifacts-manual-required',
      'tracking-full-product-ui-runtime.child-rendered-ui-artifacts-still-missing',
      'tracking-full-product-ui-runtime.product-ready-false',
    ],
    productClaims: readModel.productClaims,
    missingProofReason:
      'Full product UI runtime still requires production retention write-result UI, rendered child-device check-in UI, rendered child-device location consent UI, and rendered child safe/help response artifacts. This preflight defines acceptance rows and artifact paths only; it does not claim full product UI runtime, child-device runtime, physical-device proof, authority proof, provider delivery runtime, production product UI, or product readiness.',
    commands,
  };
}

function assertProof(proof) {
  assert.equal(proof.summary.rowCount, 4, 'expected four hard full product UI preflight rows');
  assert.equal(proof.summary.manualRequiredRowCount, 4, 'expected all hard UI rows manual-required');
  assert.equal(proof.summary.requiredArtifactCount, 4, 'expected four hard UI runtime artifacts');
  assert.equal(proof.summary.presentArtifactCount, 0, 'preflight must not mark product UI artifacts present');
  assert.equal(proof.summary.productReadyRowCount, 0, 'preflight must not include product-ready rows');
  assert.equal(proof.productClaims.fullProductUiRuntimeClaimed, false);
  assert.equal(proof.productClaims.childDeviceRuntimeClaimed, false);
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
      '# Tracking Full Product UI Runtime Preflight Source Snapshot',
      '',
      `- generatedAt: ${generatedAt}`,
      `- commit: ${proof.commit}`,
      `- status: ${proof.status}`,
      `- rowCount: ${proof.summary.rowCount}`,
      `- missingArtifactCount: ${proof.summary.missingArtifactCount}`,
      '- production retention write UI and rendered child runtime UI artifacts remain manual-required',
      '- full product UI runtime, child-device runtime, and product-ready claims remain false',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(path.join(focusedProofDir, 'manual-validation-runbook.md'), manualValidationRunbook(proof), 'utf8');
  await writeJson(path.join(output30, '34-full-product-ui-runtime-preflight-proof.json'), proof);
  await writeJson(path.join(output33, '71-full-product-ui-runtime-preflight-proof.json'), proof);
  await writeFile(
    path.join(output30, '34-full-product-ui-runtime-preflight-validation.log'),
    `${commands.map((entry) => entry.command).join('\n')}\n`,
    'utf8'
  );
  await writeFile(
    path.join(output33, '71-full-product-ui-runtime-preflight-validation.log'),
    `${commands.map((entry) => entry.command).join('\n')}\n`,
    'utf8'
  );
}

function manualValidationRunbook(proof) {
  const lines = [
    '# Tracking Full Product UI Runtime Preflight Manual Runbook',
    '',
    `- generatedAt: ${generatedAt}`,
    `- status: ${proof.status}`,
    '- This runbook is not product-ready proof. It names the product UI runtime artifacts still missing.',
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
