import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
import { tsImport } from 'tsx/esm/api';
import { runNpmCommand } from './run-npm-command.mjs';

const repoRoot = process.cwd();
const proofMode = 'tracking-authority-runtime-readiness-blocker-proof';
const timestamp = '2026-06-07T21:45:00.000Z';
const resultDir = join(repoRoot, 'test-results', proofMode);
const focusedProofDir = join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const wp31ProofDir = join(
  repoRoot,
  'output',
  'tracking-plan-proof',
  '31-platform-extension-checklists-and-proof-routing'
);
const wp33ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await main();

async function main() {
  await rm(resultDir, { recursive: true, force: true });
  await rm(focusedProofDir, { recursive: true, force: true });
  await mkdir(resultDir, { recursive: true });
  await mkdir(focusedProofDir, { recursive: true });
  await mkdir(wp31ProofDir, { recursive: true });
  await mkdir(wp33ProofDir, { recursive: true });

  run('node', ['scripts/test/tracking-authority-enrollment-manual-required-proof.mjs']);
  runNpmCommand(run, [
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/tracking-domain',
    '--',
    'tests/contract/tracking-authority-runtime-readiness-blocker-proof.test.ts',
  ]);

  const authorityRuntimeModule = await tsImport(
    pathToFileURL(
      join(repoRoot, 'packages', 'schema-domain', 'src', 'tracking-authority-runtime-readiness-blocker-proof.ts')
    ).href,
    import.meta.url
  );
  const authorityProof = await readProofJson(
    'test-results/tracking-authority-enrollment-manual-required-proof/proof.json'
  );
  const sourceProofRefs = ['test-results/tracking-authority-enrollment-manual-required-proof/proof.json'];
  const readModel = authorityRuntimeModule.buildTrackingAuthorityRuntimeReadinessBlockerProof(
    {
      generatedAt: timestamp,
      proofId: proofMode,
      sourceProofRefs,
    },
    authorityProof.readModel
  );
  const proof = buildProof(readModel, sourceProofRefs);

  assertProof(proof);
  await writeArtifacts(proof);

  console.log('tracking-authority-runtime-readiness-blocker-proof-ok');
  console.log(`evidence=${relativePath(join(resultDir, 'proof.json'))}`);
}

function buildProof(readModel, sourceProofRefs) {
  return {
    proofMode,
    generatedAt: timestamp,
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    commit: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: initialGitStatusShort,
    workpackIds: ['31-platform-extension-checklists-and-proof-routing', '33-proof-gates-fixtures-rollout-and-pr-gate'],
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    currentProofTier: 'P0_CONTRACT',
    status: 'authority_required',
    sourceProofRefs,
    summary: {
      authorityEnrollmentRows: readModel.authorityEnrollmentRows,
      authorityRequiredRows: readModel.authorityRequiredRows,
      manualRequiredRows: readModel.manualRequiredRows,
      missingAuthorityRuntimeEvidenceCount: readModel.missingAuthorityRuntimeEvidenceCount,
      blockerCount: readModel.blockers.length,
      productReadyBlockers: readModel.blockers.filter((row) => row.blockerId === 'product-ready-authority').length,
    },
    proofLabels: [
      'tracking-authority-runtime.aggregate-blocker',
      'tracking-authority-runtime.authority-enrollment-linked',
      'tracking-authority-runtime.hard-control-runtime-unclaimed',
      'tracking-authority-runtime.product-ready-false',
    ],
    productClaims: readModel.productClaims,
    readModel,
    commands,
  };
}

function assertProof(proof) {
  assert.equal(proof.summary.authorityEnrollmentRows, 5, 'expected authority rows');
  assert.equal(proof.summary.authorityRequiredRows, 4, 'expected four authority-required rows');
  assert.equal(proof.summary.manualRequiredRows, 1, 'expected one manual row');
  assert.equal(
    proof.summary.missingAuthorityRuntimeEvidenceCount > 0,
    true,
    'expected missing authority runtime evidence'
  );
  assert.equal(proof.summary.blockerCount, 10, 'expected every authority blocker');
  assert.equal(proof.summary.productReadyBlockers, 1, 'expected product blocker');
  assert.equal(
    Object.values(proof.productClaims).every((claim) => claim === false),
    true,
    'no product claims'
  );
}

async function writeArtifacts(proof) {
  await writeJson(join(resultDir, 'proof.json'), proof);
  await writeJson(join(resultDir, 'read-model.json'), proof.readModel);
  await writeJson(join(focusedProofDir, 'proof.json'), proof);
  await writeJson(join(focusedProofDir, 'read-model.json'), proof.readModel);
  await writeFile(join(focusedProofDir, '00-source-snapshot.md'), sourceSnapshot(proof), 'utf8');
  await writeFile(join(focusedProofDir, '16-validation-commands.log'), commandLog(), 'utf8');
  await writeJson(join(wp31ProofDir, '22-authority-runtime-readiness-blocker-proof.json'), proof);
  await writeJson(join(wp33ProofDir, '55-authority-runtime-readiness-blocker-proof.json'), proof);
  await writeFile(
    join(wp33ProofDir, '55-authority-runtime-readiness-blocker-validation-commands.log'),
    commandLog(),
    'utf8'
  );
}

function sourceSnapshot(proof) {
  return [
    '# Tracking Authority Runtime Readiness Blocker Source Snapshot',
    '',
    `- generatedAt: ${timestamp}`,
    `- commit: ${proof.commit}`,
    `- status: ${proof.status}`,
    `- missingAuthorityRuntimeEvidenceCount: ${proof.summary.missingAuthorityRuntimeEvidenceCount}`,
    '',
  ].join('\n');
}

async function readProofJson(relativePath) {
  return JSON.parse(await readFile(join(repoRoot, relativePath), 'utf8'));
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

function commandLog() {
  return `${commands.map((entry) => entry.command).join('\n')}\n`;
}

function relativePath(filePath) {
  return filePath
    .replace(repoRoot, '')
    .replace(/^[/\\]/, '')
    .replaceAll('\\', '/');
}
