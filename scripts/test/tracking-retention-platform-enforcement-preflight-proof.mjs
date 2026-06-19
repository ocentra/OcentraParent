import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { tsImport } from 'tsx/esm/api';
import { runNpmCommand } from './run-npm-command.mjs';

const repoRoot = process.cwd();
const proofMode = 'tracking-retention-platform-enforcement-preflight-proof';
const generatedAt = '2026-06-08T11:50:00.000Z';
const resultDir = path.join(repoRoot, 'test-results', proofMode);
const focusedProofDir = path.join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const output07 = path.join(repoRoot, 'output', 'tracking-plan-proof', '07-retention-and-custody-model');
const output33 = path.join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await main();

async function main() {
  await mkdir(resultDir, { recursive: true });
  await mkdir(focusedProofDir, { recursive: true });
  await mkdir(output07, { recursive: true });
  await mkdir(output33, { recursive: true });

  run('node', ['scripts/test/tracking-retention-runtime-artifact-gate-proof.mjs']);
  runNpmCommand(run, [
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/tracking-domain',
    '--',
    'tests/contract/tracking-retention-platform-enforcement-preflight-proof.test.ts',
  ]);

  const proofModule = await tsImport(
    pathToFileURL(
      path.join(repoRoot, 'packages', 'tracking-domain', 'src', 'tracking-retention-platform-enforcement-preflight-proof.ts')
    ).href,
    import.meta.url
  );
  const runtimeGateProof = JSON.parse(
    await readFile(
      path.join(
        repoRoot,
        proofModule.RequiredTrackingRetentionPlatformEnforcementPreflightPlan.sourceRuntimeArtifactGateProofRef
      ),
      'utf8'
    )
  );
  const readModel = proofModule.buildTrackingRetentionPlatformEnforcementPreflightProof(
    generatedAt,
    runtimeGateProof.readModel
  );
  const proof = buildProof({ readModel, runtimeGateProof });

  assertProof(proof);
  await writeArtifacts(proof);

  console.log('tracking-retention-platform-enforcement-preflight-proof-ok');
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
    workpackIds: ['07-retention-and-custody-model', '33-proof-gates-fixtures-rollout-and-pr-gate'],
    requiredProofTier: 'P4_PRODUCTION_RUNTIME',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    status: 'manual_required',
    sourceRuntimeArtifactGateProofRef: readModel.sourceRuntimeArtifactGateProofRef,
    sourceRuntimeArtifactGateStatus: runtimeGateProof.status,
    sourceRuntimeArtifactGateMissingArtifactCount: runtimeGateProof.summary.missingArtifactCount,
    readModel,
    summary: readModel.summary,
    proofLabels: [
      'tracking-retention-platform-enforcement.preflight',
      'tracking-retention-platform-enforcement.android-ios-desktop-acceptance',
      'tracking-retention-platform-enforcement.platform-artifact-still-missing',
      'tracking-retention-platform-enforcement.product-ready-false',
    ],
    productClaims: readModel.productClaims,
    missingProofReason:
      'Platform runtime retention enforcement still needs real Android, entitled iOS, and service-backed desktop runtime artifacts. This preflight defines acceptance rows and artifact paths only; it does not claim platform runtime retention enforcement, writable product settings execution, child-device delivery, physical-device proof, authority proof, production workers, or product readiness.',
    commands,
  };
}

function assertProof(proof) {
  assert.equal(proof.summary.rowCount, 3, 'expected Android, iOS, and desktop preflight rows');
  assert.equal(proof.summary.manualRequiredRowCount, 3, 'expected all platform preflight rows manual-required');
  assert.equal(proof.summary.requiredArtifactCount, 6, 'expected platform-specific required artifacts');
  assert.equal(proof.summary.presentArtifactCount, 0, 'preflight must not mark platform artifacts present');
  assert.equal(proof.summary.productReadyRowCount, 0, 'preflight must not include product-ready rows');
  assert.equal(proof.productClaims.platformRuntimeRetentionEnforcementClaimed, false);
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
      '# Tracking Retention Platform Enforcement Preflight Source Snapshot',
      '',
      `- generatedAt: ${generatedAt}`,
      `- commit: ${proof.commit}`,
      `- status: ${proof.status}`,
      `- rowCount: ${proof.summary.rowCount}`,
      `- missingArtifactCount: ${proof.summary.missingArtifactCount}`,
      '- Android, iOS, and desktop platform runtime retention rows remain manual-required',
      '- platform runtime retention enforcement and product-ready claims remain false',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(path.join(focusedProofDir, 'manual-validation-runbook.md'), manualValidationRunbook(proof), 'utf8');
  await writeJson(path.join(output07, '27-retention-platform-enforcement-preflight-proof.json'), proof);
  await writeJson(path.join(output33, '70-retention-platform-enforcement-preflight-proof.json'), proof);
  await writeFile(
    path.join(output07, '27-retention-platform-enforcement-preflight-validation.log'),
    `${commands.map((entry) => entry.command).join('\n')}\n`,
    'utf8'
  );
  await writeFile(
    path.join(output33, '70-retention-platform-enforcement-preflight-validation.log'),
    `${commands.map((entry) => entry.command).join('\n')}\n`,
    'utf8'
  );
}

function manualValidationRunbook(proof) {
  const lines = [
    '# Tracking Retention Platform Enforcement Preflight Manual Runbook',
    '',
    `- generatedAt: ${generatedAt}`,
    `- status: ${proof.status}`,
    '- This runbook is not product-ready proof. It names the required platform evidence still missing.',
    '',
  ];
  for (const row of proof.readModel.rows) {
    lines.push(`## ${row.platform}`, '');
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
