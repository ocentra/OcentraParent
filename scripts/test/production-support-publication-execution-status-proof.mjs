import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'production-support-publication-execution-status-proof';
const resultDir = join(repoRoot, 'test-results', proofMode);
const outputDir = join(repoRoot, 'output', proofMode);
const proofPath = join(resultDir, 'proof.json');
const summaryPath = join(outputDir, 'proof-summary.json');
const commands = [];
const schemaPackageExports = {
  './production-support-publication-execution-status-proof': {
    import: './dist/production-support-publication-execution-status-proof.js',
    types: './dist/production-support-publication-execution-status-proof.d.ts',
  },
  './production-support-publication-execution-status-read-model': {
    import: './dist/production-support-publication-execution-status-read-model.js',
    types: './dist/production-support-publication-execution-status-read-model.d.ts',
  },
  './production-support-publication-execution-status-values': {
    import: './dist/production-support-publication-execution-status-values.js',
    types: './dist/production-support-publication-execution-status-values.d.ts',
  },
};

await main();

async function main() {
  await mkdir(resultDir, { recursive: true });
  await mkdir(outputDir, { recursive: true });
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/schema-domain']));
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/production-domain']));
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/production-domain',
      '--',
      'tests/unit/production-support-publication-execution-status-proof.test.ts',
    ])
  );

  const contract = await assertBuiltContract();
  const documentation = await assertDocumentationProof();
  const packageExport = await assertPackageExports();
  const commit = await gitHead();
  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit,
    proofMode,
    commands,
    evidence: {
      contract: 'packages/schema-domain/src/production-support-publication-execution-status-proof.ts',
      values: 'packages/schema-domain/src/production-support-publication-execution-status-values.ts',
      readModel: 'packages/schema-domain/src/production-support-publication-execution-status-read-model.ts',
      contractTest: 'packages/production-domain/tests/unit/production-support-publication-execution-status-proof.test.ts',
      documentation,
      proofOutput: relativePath(proofPath),
      summaryOutput: relativePath(summaryPath),
      packageExport,
    },
    rowCount: contract.rows.length,
    rows: contract.rows,
    nonClaims: contract.nonClaims,
    knownGaps: contract.knownGaps,
  };
  const summary = {
    schemaVersion: 1,
    checkedAt: proof.checkedAt,
    commit,
    proofMode,
    rowCount: proof.rowCount,
    targets: contract.targets,
    lifecycleStates: contract.lifecycleStates,
    output: relativePath(proofPath),
    knownGaps: proof.knownGaps,
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(summaryPath, `${JSON.stringify(summary, null, 2)}\n`);
  console.log(`${proofMode}-ok:${relativePath(proofPath)} ${relativePath(summaryPath)}`);
}

async function assertBuiltContract() {
  const contractModule = await importBuiltSchemaDomainModule('production-support-publication-execution-status-proof');
  const readModelModule = await importBuiltSchemaDomainModule('production-support-publication-execution-status-read-model');
  const valuesModule = await importBuiltSchemaDomainModule('production-support-publication-execution-status-values');
  const proof = contractModule.ProductionSupportPublicationExecutionStatusProofSchema.parse(
    readModelModule.ProductionSupportPublicationExecutionStatusReadModel
  );
  const summary = contractModule.summarizeProductionSupportPublicationExecutionStatusRows(proof.rows);

  assert.equal(typeof contractModule.decodeProductionSupportPublicationExecutionStatusProof, 'function');
  assert.deepEqual(valuesModule.RequiredPublicationExecutionStatusLifecycleStates, [
    'requested',
    'queued',
    'running',
    'succeeded',
    'failed',
    'manual-required',
  ]);
  for (const target of valuesModule.RequiredPublicationExecutionStatusTargets) {
    assert.deepEqual(summary[target], {
      requested: 1,
      queued: 1,
      running: 1,
      succeeded: 1,
      failed: 1,
      'manual-required': 1,
    });
  }
  assert.equal(proof.publicRuntimeExecutionClaim, 'not-implemented');
  assert.equal(proof.publicationRunnerExecutionClaim, 'manual-required');
  assert.equal(proof.statusBackendExecutionClaim, 'manual-required');
  assert.equal(proof.supportBackendUploadExecutionClaim, 'manual-required');
  assert.equal(proof.accountLookupExecutionClaim, 'manual-required');
  assert.equal(proof.billingProviderContactClaim, 'manual-required');
  assert.equal(proof.productionSlaClaim, 'not-implemented');
  assert.equal(proof.legalDisclosureExecutionClaim, 'manual-required');
  assert.equal(proof.childActivityCustodyClaim, 'not-implemented');
  assertPublicationExecutionRowsRemainManual(proof.rows);

  return {
    targets: valuesModule.RequiredPublicationExecutionStatusTargets,
    lifecycleStates: valuesModule.RequiredPublicationExecutionStatusLifecycleStates,
    rows: proof.rows.map((row) => ({
      target: row.target,
      lifecycleStatus: row.lifecycleStatus,
      sourceProof: row.sourceProof,
      publicRuntimeState: row.publicRuntimeState,
      publicationRunnerState: row.publicationRunnerState,
      statusBackendState: row.statusBackendState,
      supportBackendUploadState: row.supportBackendUploadState,
      legalExecutionState: row.legalExecutionState,
      supportSafeDataClasses: row.supportSafeDataClasses,
    })),
    nonClaims: proof.nonClaims,
    knownGaps: readModelModule.ProductionSupportPublicationExecutionStatusKnownGaps,
  };
}

function assertPublicationExecutionRowsRemainManual(rows) {
  for (const row of rows) {
    assert.notEqual(row.publicRuntimeState, 'implemented', `${row.target} must not claim public runtime`);
    assert.notEqual(row.publicRuntimeState, 'executed', `${row.target} must not claim public runtime execution`);
    assert.notEqual(row.publicationRunnerState, 'implemented', `${row.target} must not claim publication runner`);
    assert.notEqual(
      row.publicationRunnerState,
      'executed',
      `${row.target} must not claim publication runner execution`
    );
    assert.notEqual(row.statusBackendState, 'implemented', `${row.target} must not claim status backend`);
    assert.notEqual(row.statusBackendState, 'executed', `${row.target} must not claim status backend execution`);
    assert.notEqual(row.supportBackendUploadState, 'executed', `${row.target} must not claim support upload execution`);
    assert.notEqual(row.legalExecutionState, 'executed', `${row.target} must not claim legal execution`);
    for (const dataClass of row.forbiddenDataClasses) {
      assert(!row.supportSafeDataClasses.includes(dataClass), `${row.target} unexpectedly allows ${dataClass}`);
    }
  }
}

async function assertDocumentationProof() {
  const docs = [
    'docs/features/production-distribution-support.md',
    'docs/expectations/release-installer.md',
    'docs/expectations/documentation.md',
    'docs/product-capability-checklist.md',
  ];
  for (const path of docs) {
    assertIncludes(await readRepoFile(path), proofMode, `${path} proof note`);
  }
  return docs;
}

async function assertPackageExports() {
  const productionPackageJson = JSON.parse(await readRepoFile('packages/production-domain/package.json'));
  const schemaPackageJson = JSON.parse(await readRepoFile('packages/schema-domain/package.json'));
  const retiredLocalExports = [];
  for (const [exportPath, expectedTarget] of Object.entries(schemaPackageExports)) {
    assert.deepEqual(schemaPackageJson.exports[exportPath], expectedTarget, `${exportPath} schema package export`);
    if (productionPackageJson.exports[exportPath] !== null) {
      retiredLocalExports.push(exportPath);
    }
  }
  assert.deepEqual(retiredLocalExports, []);
  return {
    owner: 'schema-domain',
    schemaExports: Object.keys(schemaPackageExports),
    retiredLocalExports,
  };
}

async function importBuiltSchemaDomainModule(moduleName) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'schema-domain', 'dist', `${moduleName}.js`)).href);
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

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
