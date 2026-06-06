import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { pathToFileURL } from 'node:url';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const proofMode = 'production-support-runtime-gap-proof';
const resultDir = join(repoRoot, 'test-results', proofMode);
const outputDir = join(repoRoot, 'output', proofMode);
const proofPath = join(resultDir, 'proof.json');
const summaryPath = join(outputDir, 'proof-summary.json');
const commands = [];

await main();

async function main() {
  await mkdir(resultDir, { recursive: true });
  await mkdir(outputDir, { recursive: true });
  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'tests/production-support-runtime-gap-proof.test.ts',
  ]);

  const contract = await assertBuiltContract();
  const documentation = await assertDocumentationProof();
  const commit = await gitHead();
  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit,
    proofMode,
    commands,
    evidence: {
      contract: 'packages/parent-domain/src/production-support-runtime-gap-proof.ts',
      values: 'packages/parent-domain/src/production-support-runtime-gap-values.ts',
      readModel: 'packages/parent-domain/src/production-support-runtime-gap-read-model.ts',
      contractTest: 'packages/parent-domain/tests/production-support-runtime-gap-proof.test.ts',
      documentation,
      proofOutput: relativePath(proofPath),
      summaryOutput: relativePath(summaryPath),
    },
    rows: contract.rows,
    nonClaims: contract.nonClaims,
    knownGaps: contract.knownGaps,
  };
  const summary = {
    schemaVersion: 1,
    checkedAt: proof.checkedAt,
    commit,
    proofMode,
    rowCount: proof.rows.length,
    rows: proof.rows.map((row) => row.item),
    output: relativePath(proofPath),
    knownGaps: proof.knownGaps,
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(summaryPath, `${JSON.stringify(summary, null, 2)}\n`);
  console.log(`${proofMode}-ok:${relativePath(proofPath)} ${relativePath(summaryPath)}`);
}

async function assertBuiltContract() {
  const contractModule = await importBuiltModule('production-support-runtime-gap-proof.js');
  const readModelModule = await importBuiltModule('production-support-runtime-gap-read-model.js');
  const valuesModule = await importBuiltModule('production-support-runtime-gap-values.js');
  const proof = contractModule.ProductionSupportRuntimeGapProofSchema.parse(
    readModelModule.ProductionSupportRuntimeGapReadModel
  );

  assert.equal(typeof contractModule.decodeProductionSupportRuntimeGapProof, 'function');
  assert.deepEqual(valuesModule.RequiredRuntimeGapItems, [
    'public-website-runtime-gap',
    'support-publication-execution-gap',
    'support-backend-upload-execution-gap',
    'account-billing-provider-runtime-gap',
    'legal-export-delete-runtime-gap',
    'remote-support-sla-runtime-gap',
  ]);
  assert.deepEqual(contractModule.summarizeProductionSupportRuntimeGapRows(proof.rows), {
    'public-website-runtime-gap': 1,
    'support-publication-execution-gap': 1,
    'support-backend-upload-execution-gap': 1,
    'account-billing-provider-runtime-gap': 1,
    'legal-export-delete-runtime-gap': 1,
    'remote-support-sla-runtime-gap': 1,
  });
  assert.equal(proof.publicRuntimeClaim, 'not-implemented');
  assert.equal(proof.supportPublicationExecutionClaim, 'manual-required');
  assert.equal(proof.supportBackendUploadExecutionClaim, 'manual-required');
  assert.equal(proof.accountBackendRuntimeClaim, 'backend-required');
  assert.equal(proof.billingProviderRuntimeClaim, 'provider-required');
  assert.equal(proof.legalExportDeleteRuntimeClaim, 'manual-required');
  assert.equal(proof.remoteSupportSessionClaim, 'not-implemented');
  assert.equal(proof.productionSlaClaim, 'not-implemented');
  assert.equal(proof.childActivityCustodyClaim, 'not-implemented');
  assertRuntimeGapRowsRemainManual(proof.rows);

  return {
    rows: proof.rows.map((row) => ({
      item: row.item,
      sourceProof: row.sourceProof,
      sourceContractState: row.sourceContractState,
      runtimeExecutionState: row.runtimeExecutionState,
      backendRuntimeState: row.backendRuntimeState,
      providerRuntimeState: row.providerRuntimeState,
      publicationState: row.publicationState,
      supportSafeDataClasses: row.supportSafeDataClasses,
    })),
    nonClaims: proof.nonClaims,
    knownGaps: readModelModule.ProductionSupportRuntimeGapKnownGaps,
  };
}

function assertRuntimeGapRowsRemainManual(rows) {
  for (const row of rows) {
    assert.notEqual(row.runtimeExecutionState, 'implemented', `${row.item} must not claim runtime implementation`);
    assert.notEqual(row.runtimeExecutionState, 'executed', `${row.item} must not claim runtime execution`);
    assert.notEqual(row.backendRuntimeState, 'executed', `${row.item} must not claim backend execution`);
    assert.notEqual(row.providerRuntimeState, 'executed', `${row.item} must not claim provider execution`);
    for (const dataClass of row.forbiddenDataClasses) {
      assert(!row.supportSafeDataClasses.includes(dataClass), `${row.item} unexpectedly allows ${dataClass}`);
    }
  }
}

async function assertDocumentationProof() {
  const docs = [
    'docs/features/production-distribution-support.md',
    'docs/expectations/release-installer.md',
    'docs/expectations/billing.md',
    'docs/expectations/data-custody.md',
    'docs/expectations/documentation.md',
    'docs/product-capability-checklist.md',
  ];
  for (const path of docs) {
    assertIncludes(await readRepoFile(path), proofMode, `${path} proof note`);
  }
  return docs;
}

async function importBuiltModule(fileName) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', fileName)).href);
}

async function readRepoFile(path) {
  return readFile(join(repoRoot, path), 'utf8');
}

async function runCommand(commandName, args) {
  commands.push([commandName, ...args].join(' '));
  await new Promise((resolve, reject) => {
    const child = spawn(commandName, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) =>
      code === 0 ? resolve() : reject(new Error(`${commandName} ${args.join(' ')} exited with ${code}`))
    );
    child.once('error', reject);
  });
}

async function gitHead() {
  const chunks = [];
  await new Promise((resolve, reject) => {
    const child = spawn('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, stdio: ['ignore', 'pipe', 'pipe'] });
    child.stdout.on('data', (chunk) => chunks.push(String(chunk)));
    child.once('exit', (code) => (code === 0 ? resolve() : reject(new Error('git rev-parse HEAD failed'))));
    child.once('error', reject);
  });
  return chunks.join('').trim();
}

function assertIncludes(value, expected, label) {
  if (!value.includes(expected)) {
    throw new Error(`${label}: missing ${expected}`);
  }
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}
