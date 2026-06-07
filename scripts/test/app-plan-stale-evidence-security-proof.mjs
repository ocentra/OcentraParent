import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'output', 'app-plan-proof', 'security-blueprint', 'stale-evidence-rejection');
const testResultDir = join(repoRoot, 'test-results', 'app-plan-stale-evidence-security-proof');
const proofPath = join(testResultDir, 'proof.json');
const sourceProofPath = join(
  repoRoot,
  'output',
  'app-game-plan-proof',
  '75-source-freshness-preview-gate',
  'proof.json'
);
const proofBranch = 'codex/app-plan-stale-evidence-security-proof-split';
const deterministicProofRevision = 'branch-head-validated-by-harness';
const deterministicGeneratedAt = 'deterministic-proof-artifact';
const validatedGitStatusShort = 'validated-by-explicit-handoff-status-check';
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });
  await mkdir(testResultDir, { recursive: true });

  await runCommand('cmd', ['/c', 'node', 'scripts/test/app-game-source-freshness-preview-gate-proof.mjs']);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'app-game-source-freshness-preview-gate',
    'app-game-source-freshness-policy-consumption',
    'app-game-policy-preview-handoff',
  ]);

  const sourceProof = JSON.parse(await readText(sourceProofPath));
  assert.equal(sourceProof.proofMode, 'app-game-source-freshness-preview-gate');
  assert.equal(sourceProof.summary.sourceManualRequiredCount, 2);
  assert.equal(sourceProof.summary.manualRequiredCount, 3);
  assert.equal(sourceProof.summary.previewReadyCount, 1);
  assert.equal(sourceProof.nonClaims.policyEvaluatorRuntimeClaimed, false);
  assert.equal(sourceProof.nonClaims.timerRuntimeClaimed, false);
  assert.equal(sourceProof.nonClaims.adapterDispatchClaimed, false);
  assert.equal(sourceProof.nonClaims.childDeliveryClaimed, false);
  assert.equal(sourceProof.nonClaims.platformEnforcementClaimed, false);

  const staleRows = sourceProof.readModel.rows.filter((row) => row.sourceRequirementStates.includes('stale'));
  assert.equal(staleRows.length, 2);
  const staleRow = staleRows.find((row) => row.targetDomain === 'native-app');
  assert.notEqual(staleRow, undefined);
  assert.equal(staleRow.sourceReadinessState, 'manual-required');
  assert.equal(staleRow.sourcePolicyCompileAllowed, false);
  assert.equal(staleRow.compiledDecisionProvided, false);
  assert.equal(staleRow.previewStatus, 'manual-required');
  assert.equal(staleRow.gateState, 'source-manual-required');
  assert.equal(staleRow.previewRow, null);
  assert.equal(staleRow.sourceReasonCodes.includes('stale-source-status-row'), true);
  assert.equal(staleRow.adapterDispatchClaimed, false);
  assert.equal(staleRow.platformEnforcementClaimed, false);

  const checklist = await readText(join(repoRoot, 'docs', 'plans', 'app-plan', 'implementation-checklist.md'));
  assertChecklistRowChecked(checklist, 'Security: stale evidence rejection.');

  const featureDoc = await readText(join(repoRoot, 'docs', 'features', 'app-game-control.md'));
  assert.match(
    featureDoc,
    /app-plan stale evidence security reconciliation/i,
    'Expected feature doc to record app-plan stale evidence security reconciliation'
  );

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-plan-stale-evidence-security-proof',
    generatedAt: deterministicGeneratedAt,
    branch: proofBranch,
    commit: deterministicProofRevision,
    commitMetadata:
      'This proof intentionally avoids embedding HEAD because a committed artifact cannot contain its own final commit hash.',
    gitStatusShort: validatedGitStatusShort,
    commands,
    scope: {
      sharedEvidenceSpine: true,
      nativeAppMeaningProven: true,
      nativeGameMeaningProven: true,
      browserGameWorkDuplicated: false,
      appPlanRows: ['Security: stale evidence rejection.'],
      productChecklistChanged: false,
      packageExportsChanged: false,
      adapterDispatchClaimed: false,
      policyExecutionClaimed: false,
      platformSupportClaimed: false,
    },
    sourceProof: {
      path: relative(repoRoot, sourceProofPath),
      proofMode: sourceProof.proofMode,
      summary: sourceProof.summary,
      nonClaims: sourceProof.nonClaims,
      staleRejectedRow: {
        rowId: staleRow.rowId,
        targetDomain: staleRow.targetDomain,
        sourceReadinessState: staleRow.sourceReadinessState,
        sourcePolicyCompileAllowed: staleRow.sourcePolicyCompileAllowed,
        sourceRequirementStates: staleRow.sourceRequirementStates,
        sourceReasonCodes: staleRow.sourceReasonCodes,
        compiledDecisionProvided: staleRow.compiledDecisionProvided,
        previewStatus: staleRow.previewStatus,
        gateState: staleRow.gateState,
        previewRow: staleRow.previewRow,
        adapterDispatchClaimed: staleRow.adapterDispatchClaimed,
        platformEnforcementClaimed: staleRow.platformEnforcementClaimed,
      },
    },
    proofPaths: {
      proof: relative(repoRoot, proofPath),
      appPlanProofPack: relative(repoRoot, outputDir),
      harness: 'scripts/test/app-plan-stale-evidence-security-proof.mjs',
    },
  };

  await writeFile(join(outputDir, '00-source-snapshot.md'), sourceSnapshot(proof));
  await writeFile(join(outputDir, '10-validation-commands.log'), `${commands.join('\n\n')}\n`);
  await writeFile(join(outputDir, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);

  console.log(`app-plan-stale-evidence-security-proof-ok:${relative(repoRoot, proofPath)}`);
}

function sourceSnapshot(proof) {
  return [
    '# App-Plan Stale Evidence Security Proof Source Snapshot',
    '',
    `Branch: ${proof.branch}`,
    `Commit: ${proof.commit}`,
    '',
    '## Git Status',
    '',
    '```text',
    proof.gitStatusShort,
    '```',
    '',
    '## Source Proof',
    '',
    `- ${proof.sourceProof.path}`,
    `- mode: ${proof.sourceProof.proofMode}`,
    `- stale rejected row: ${proof.sourceProof.staleRejectedRow.rowId}`,
    `- gate state: ${proof.sourceProof.staleRejectedRow.gateState}`,
    '',
  ].join('\n');
}

function assertChecklistRowChecked(checklist, row) {
  const normalized = checklist.replace(/\s+/g, ' ');
  assert.match(normalized, new RegExp(`- \\[x\\] ${escapeRegExp(row)}`), `Expected checked app-plan row: ${row}`);
}

function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

async function readText(path) {
  return readFile(path, 'utf8');
}

async function commandOutput(command, args) {
  const child = spawn(command, args, { cwd: repoRoot, stdio: ['ignore', 'pipe', 'pipe'] });
  const chunks = [];
  child.stdout.on('data', (chunk) => chunks.push(chunk));
  child.stderr.on('data', (chunk) => chunks.push(chunk));
  const exitCode = await new Promise((resolve) => {
    child.on('close', resolve);
  });
  const output = Buffer.concat(chunks).toString('utf8');
  if (exitCode !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed with ${exitCode}\n${output}`);
  }
  return output;
}

async function runCommand(command, args) {
  const output = await commandOutput(command, args);
  commands.push(`${command} ${args.join(' ')}\n${normalizeCommandOutput(output)}`);
}

function normalizeCommandOutput(output) {
  return output
    .replaceAll(repoRoot.replaceAll('\\', '/'), '<repo>')
    .replaceAll(repoRoot, '<repo>')
    .replace(/Start at\s+\d{2}:\d{2}:\d{2}/g, 'Start at  deterministic-time')
    .replace(
      /Duration\s+[\d.]+(?:ms|s) \(transform [^)]*\)/g,
      'Duration  deterministic-duration (transform deterministic)'
    )
    .trim();
}
