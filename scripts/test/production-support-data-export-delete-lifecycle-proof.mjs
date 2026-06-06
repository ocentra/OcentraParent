import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'production-support-data-export-delete-lifecycle-proof';
const resultDir = join(repoRoot, 'test-results', proofMode);
const outputDir = join(repoRoot, 'output', proofMode);
const proofPath = join(resultDir, 'proof.json');
const summaryPath = join(outputDir, 'proof-summary.json');
const commands = [];

await main();

async function main() {
  await mkdir(resultDir, { recursive: true });
  await mkdir(outputDir, { recursive: true });
  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/logging-domain']);
  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/logging-domain',
    '--',
    'tests/data-export-delete-lifecycle.test.ts',
  ]);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'tests/production-support-data-export-delete-lifecycle-proof.test.ts',
  ]);

  const logging = await assertBuiltLoggingContract();
  const parent = await assertBuiltParentContract();
  const documentation = await assertDocumentationProof();
  const commit = await gitHead();
  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit,
    proofMode,
    commands,
    evidence: {
      loggingContract: 'packages/logging-domain/src/data-export-delete-lifecycle.ts',
      loggingReadModel: 'packages/logging-domain/src/data-export-delete-lifecycle-read-model.ts',
      loggingTest: 'packages/logging-domain/tests/data-export-delete-lifecycle.test.ts',
      parentContract: 'packages/parent-domain/src/production-support-data-export-delete-lifecycle-proof.ts',
      parentReadModel: 'packages/parent-domain/src/production-support-data-export-delete-lifecycle-read-model.ts',
      parentTest: 'packages/parent-domain/tests/production-support-data-export-delete-lifecycle-proof.test.ts',
      packageExport: 'not-added-parent-package-json-locked-by-another-lane',
      documentation,
      proofOutput: relativePath(proofPath),
      summaryOutput: relativePath(summaryPath),
    },
    loggingRows: logging.rows,
    parentRows: parent.rows,
    nonClaims: parent.nonClaims,
    knownGaps: parent.knownGaps,
  };
  const summary = {
    schemaVersion: 1,
    checkedAt: proof.checkedAt,
    commit,
    proofMode,
    loggingRowCount: proof.loggingRows.length,
    parentRowCount: proof.parentRows.length,
    output: relativePath(proofPath),
    knownGaps: proof.knownGaps,
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(summaryPath, `${JSON.stringify(summary, null, 2)}\n`);
  console.log(`${proofMode}-ok:${relativePath(proofPath)} ${relativePath(summaryPath)}`);
}

async function assertBuiltLoggingContract() {
  const contractModule = await importBuiltModule('packages/logging-domain/dist/data-export-delete-lifecycle.js');
  const readModelModule = await importBuiltModule(
    'packages/logging-domain/dist/data-export-delete-lifecycle-read-model.js'
  );
  const readModel = contractModule.DataExportDeleteLifecycleReadModelSchema.parse(
    readModelModule.DataExportDeleteLifecycleReadModel
  );

  assert.equal(typeof contractModule.decodeDataExportDeleteLifecycleReadModel, 'function');
  assert.equal(readModel.entries.length, 14);
  assert.deepEqual(countBy(readModel.entries.map((entry) => entry.lifecycleState)), {
    requested: 2,
    authorized: 2,
    queued: 2,
    running: 2,
    succeeded: 2,
    failed: 2,
    'manual-required': 2,
  });

  return {
    rows: readModel.entries.map((entry) => ({
      lifecycleId: entry.lifecycleId,
      operation: entry.operation,
      lifecycleState: entry.lifecycleState,
      custodyState: entry.custodyState,
      payloadState: entry.payloadState,
    })),
  };
}

async function assertBuiltParentContract() {
  const contractModule = await importBuiltModule(
    'packages/parent-domain/dist/production-support-data-export-delete-lifecycle-proof.js'
  );
  const readModelModule = await importBuiltModule(
    'packages/parent-domain/dist/production-support-data-export-delete-lifecycle-read-model.js'
  );
  const proof = contractModule.ProductionSupportDataExportDeleteLifecycleProofSchema.parse(
    readModelModule.ProductionSupportDataExportDeleteLifecycleReadModel
  );

  assert.equal(typeof contractModule.decodeProductionSupportDataExportDeleteLifecycleProof, 'function');
  assert.equal(proof.rows.length, 14);
  assert.equal(proof.backendUploadExecutionState, 'not-implemented');
  assert.equal(proof.publicRuntimeExecutionState, 'not-implemented');
  assert.equal(proof.providerExecutionState, 'not-implemented');
  assert.equal(proof.productionSlaState, 'not-implemented');
  assert.equal(proof.remoteSupportSessionState, 'not-implemented');
  assert.equal(proof.childActivityCustodyState, 'not-implemented');

  return {
    rows: proof.rows.map((row) => ({
      surface: row.surface,
      operation: row.operation,
      lifecycleState: row.lifecycleState,
      sourceProof: row.sourceProof,
      supportSafeDataClasses: row.supportSafeDataClasses,
    })),
    nonClaims: proof.nonClaims,
    knownGaps: readModelModule.ProductionSupportDataExportDeleteLifecycleKnownGaps,
  };
}

async function assertDocumentationProof() {
  const docs = [
    'docs/features/production-distribution-support.md',
    'docs/expectations/data-custody.md',
    'docs/expectations/documentation.md',
    'docs/product-capability-checklist.md',
    'packages/logging-domain/README.md',
  ];
  for (const path of docs) {
    assertIncludes(await readRepoFile(path), proofMode, `${path} proof note`);
  }
  return docs;
}

async function importBuiltModule(path) {
  return import(pathToFileURL(join(repoRoot, path)).href);
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

function countBy(values) {
  return values.reduce((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}
