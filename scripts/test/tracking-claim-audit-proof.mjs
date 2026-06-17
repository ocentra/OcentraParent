import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdir, stat, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { tsImport } from 'tsx/esm/api';

const repoRoot = process.cwd();
const proofMode = 'tracking-claim-audit-proof';
const resultDir = path.join(repoRoot, 'test-results', proofMode);
const namedProofRoot = path.join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const output33 = path.join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const commands = [];

await main();

async function main() {
  await mkdir(resultDir, { recursive: true });
  await mkdir(namedProofRoot, { recursive: true });
  await mkdir(output33, { recursive: true });

  run('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/tracking-domain',
    '--',
    'tracking-claim-audit-proof',
  ]);

  const proofModule = await importDist('tracking-claim-audit-proof.js');
  const generatedAt = '2026-06-08T04:05:00.000Z';
  const inventories = await collectInventories(proofModule.RequiredTrackingClaimAuditPlans);
  const proof = buildProof({ generatedAt, inventories, proofModule });

  assertProof(proof);
  await writeArtifacts(proof);

  console.log('tracking-claim-audit-proof-ok');
  console.log(`evidence=${relativePath(path.join(resultDir, 'proof.json'))}`);
}

async function collectInventories(plans) {
  return Promise.all(
    plans.map(async (plan) => ({
      auditArea: plan.auditArea,
      presentArtifacts: await presentArtifactsForRoot(path.join(repoRoot, plan.proofRoot), plan.requiredArtifacts),
    }))
  );
}

async function presentArtifactsForRoot(rootPath, requiredArtifacts) {
  const present = [];
  for (const artifact of requiredArtifacts) {
    if (await pathExists(path.join(rootPath, artifact))) {
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

function buildProof({ generatedAt, inventories, proofModule }) {
  return {
    ...proofModule.buildTrackingClaimAuditProof(generatedAt, inventories),
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: gitOutput(['status', '--short']),
    commands,
    artifactPaths: {
      evidence: 'test-results/tracking-claim-audit-proof/proof.json',
      namedProofRoot: 'output/tracking-plan-proof/tracking-claim-audit-proof/proof.json',
      wp33: 'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/65-claim-audit-proof.json',
    },
  };
}

function assertProof(proof) {
  const fullProductUiRow = proof.rows.find((row) => row.auditArea === 'full-product-parent-child-ui-runtime');
  assert.equal(proof.productClaims.productReadyClaimed, false, 'product ready must remain false');
  assert.equal(proof.summary.approvedClaimCount, 0, 'claim audit must not approve claims');
  assert.equal(proof.summary.productReadyRowCount, 0, 'claim audit must not mark rows product-ready');
  assert.ok(proof.summary.rowCount >= 10, 'claim audit should cover all final claim areas');
  assert.equal(
    proof.summary.acceptanceCriteriaCount,
    proof.summary.rowCount * 4,
    'each claim audit row should carry four acceptance criteria'
  );
  assert.equal(
    proof.summary.manualValidationCommandCount,
    proof.summary.rowCount * 3,
    'each claim audit row should carry the claim/readiness/handoff validation chain'
  );
  assert.equal(
    proof.summary.artifactAcceptanceNoteCount,
    proof.summary.rowCount * 4,
    'each claim audit row should carry four artifact acceptance notes'
  );
  assert.ok(
    proof.rows.every((row) => row.artifactAcceptanceNotes.some((note) => note.includes('claimApproved remains false'))),
    'claim audit acceptance notes must keep claim approval false'
  );
  assert.ok(fullProductUiRow, 'claim audit needs full product UI runtime row');
  assert.equal(fullProductUiRow.requiredArtifacts.length, 9, 'expected nine full product UI runtime artifacts');
  assert.equal(fullProductUiRow.fullProductUiClaimed, false, 'full product UI remains unclaimed');
  assert.equal(fullProductUiRow.productClaimReady, false, 'full product UI row is not product-ready');
  assert.ok(
    fullProductUiRow.supportingProofRefs.includes(
      'test-results/tracking-full-product-ui-local-runtime-artifact-capture-proof/proof.json'
    ),
    'full product UI row should cite local artifact capture proof'
  );
}

async function writeArtifacts(proof) {
  await writeJson(path.join(resultDir, 'proof.json'), proof);
  await writeJson(path.join(resultDir, 'read-model.json'), proof.rows);
  await writeJson(path.join(namedProofRoot, 'proof.json'), proof);
  await writeFile(path.join(namedProofRoot, '00-source-snapshot.md'), sourceSnapshot(proof), 'utf8');
  await writeFile(path.join(namedProofRoot, '16-validation-commands.log'), validationLog(), 'utf8');
  await writeJson(path.join(output33, '65-claim-audit-proof.json'), proof);
  await writeFile(path.join(output33, '65-claim-audit-validation-commands.log'), validationLog(), 'utf8');
}

function sourceSnapshot(proof) {
  return [
    '# Tracking Claim Audit Proof',
    '',
    `- generatedAt: ${proof.generatedAt}`,
    `- commit: ${proof.baseCommitAtGeneration}`,
    '- requiredProofTier: P4_REAL_RUNTIME_HANDOFF',
    '- currentProofTier: P3_LOCAL_DEV_MACHINE',
    `- currentStatus: ${proof.currentStatus}`,
    '- approvedClaimCount: 0',
    '- productReadyRowCount: 0',
    `- physicalDeviceRequiredRowCount: ${proof.summary.physicalDeviceRequiredRowCount}`,
    `- approvedManualRequiredRowCount: ${proof.summary.approvedManualRequiredRowCount}`,
    `- manualProviderRuntimeRequiredRowCount: ${proof.summary.manualProviderRuntimeRequiredRowCount}`,
    `- productionRuntimeRequiredRowCount: ${proof.summary.productionRuntimeRequiredRowCount}`,
    `- acceptanceCriteriaCount: ${proof.summary.acceptanceCriteriaCount}`,
    `- manualValidationCommandCount: ${proof.summary.manualValidationCommandCount}`,
    `- artifactAcceptanceNoteCount: ${proof.summary.artifactAcceptanceNoteCount}`,
    '- proof module: packages/tracking-domain/src/tracking-claim-audit-proof.ts',
    '- proof tests: packages/tracking-domain/tests/contract/tracking-claim-audit-proof.test.ts',
    '- proof harness: scripts/test/tracking-claim-audit-proof.mjs',
    '',
  ].join('\n');
}

function validationLog() {
  return `${commands.map((entry) => `${entry.command} exit=${entry.status}`).join('\n')}\n`;
}

function importDist(name) {
  return tsImport(pathToFileURL(path.join(repoRoot, 'packages', 'tracking-domain', 'src', name.replace(/\.js$/u, '.ts'))).href, import.meta.url);
}

function run(command, args) {
  const printable = [command, ...args].join(' ');
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: false,
  });
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
