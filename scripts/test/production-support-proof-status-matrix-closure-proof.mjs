import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'production-support-proof-status-matrix-closure-proof';
const resultDir = join(repoRoot, 'test-results', proofMode);
const outputDir = join(repoRoot, 'output', proofMode);
const proofPath = join(resultDir, 'proof.json');
const summaryPath = join(outputDir, 'proof-summary.json');
const commands = [];
const expectedPackageExports = [
  './production-support-proof-status-matrix-closure',
  './production-support-proof-status-matrix-closure-read-model',
  './production-support-proof-status-matrix-closure-values',
];

await main();

async function main() {
  await mkdir(resultDir, { recursive: true });
  await mkdir(outputDir, { recursive: true });
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/schema-domain']));
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/logging-domain']));
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']));
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/parent-domain',
      '--',
      'tests/production-support-proof-status-matrix-closure-proof.test.ts',
    ])
  );

  const contract = await assertBuiltContract();
  const linkedProofs = await assertLinkedProofs();
  const documentation = await assertDocumentationProof();
  const packageExport = await assertPackageExports();
  const commit = 'branch-head-validated-by-harness';
  const proof = {
    schemaVersion: 1,
    checkedAt: 'deterministic-proof-artifact',
    commit,
    proofMode,
    commands,
    evidence: {
      contract: 'packages/parent-domain/src/production-support-proof-status-matrix-closure-proof.ts',
      values: 'packages/parent-domain/src/production-support-proof-status-matrix-closure-values.ts',
      readModel: 'packages/parent-domain/src/production-support-proof-status-matrix-closure-read-model.ts',
      contractTest: 'packages/parent-domain/tests/production-support-proof-status-matrix-closure-proof.test.ts',
      linkedProofs,
      documentation,
      proofOutput: relativePath(proofPath),
      summaryOutput: relativePath(summaryPath),
      packageExport,
    },
    rows: contract.rows,
    sourceProofRefs: contract.sourceProofRefs,
    nonClaims: contract.nonClaims,
    knownGaps: contract.knownGaps,
  };
  const summary = {
    schemaVersion: 1,
    checkedAt: proof.checkedAt,
    commit,
    proofMode,
    areas: contract.areas,
    sourceProofRefs: contract.sourceProofRefs,
    output: relativePath(proofPath),
    knownGaps: proof.knownGaps,
    packageExport,
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(summaryPath, `${JSON.stringify(summary, null, 2)}\n`);
  console.log(`${proofMode}-ok:${relativePath(proofPath)} ${relativePath(summaryPath)}`);
}

async function assertBuiltContract() {
  const contractModule = await importBuiltModule(
    'parent-domain',
    'production-support-proof-status-matrix-closure-proof.js'
  );
  const readModelModule = await importBuiltModule(
    'parent-domain',
    'production-support-proof-status-matrix-closure-read-model.js'
  );
  const valuesModule = await importBuiltModule(
    'parent-domain',
    'production-support-proof-status-matrix-closure-values.js'
  );
  const proof = contractModule.ProductionSupportProofStatusMatrixClosureProofSchema.parse(
    readModelModule.ProductionSupportProofStatusMatrixClosureReadModel
  );
  assert.equal(typeof contractModule.decodeProductionSupportProofStatusMatrixClosureProof, 'function');
  assert.deepEqual(contractModule.summarizeProductionSupportProofStatusMatrixClosureRows(proof.rows), {
    'status-backend-runtime': 1,
    'public-runtime-publication': 1,
    'privacy-legal-disclosure': 1,
    'provider-secret-custody': 1,
    'export-delete-lifecycle': 1,
    'release-installer-support': 1,
  });
  assert.deepEqual(proof.sourceProofRefs, valuesModule.RequiredProofStatusMatrixClosureSourceProofs);
  assert.deepEqual(proof.nonClaims, valuesModule.RequiredProofStatusMatrixClosureNonClaims);
  assert.equal(proof.publicRuntimeClaim, 'not-implemented');
  assert.equal(proof.statusBackendExecutionClaim, 'manual-required');
  assert.equal(proof.signingStoreClaim, 'manual-required');
  assert.equal(proof.updaterExecutionClaim, 'manual-required');
  assert.equal(proof.supportBackendUploadExecutionClaim, 'manual-required');
  assert.equal(proof.accountBillingProviderExecutionClaim, 'manual-required');
  assert.equal(proof.legalDisclosureExecutionClaim, 'manual-required');
  assert.equal(proof.providerSecretCustodyClaim, 'not-implemented');
  assert.equal(proof.childActivityCustodyClaim, 'not-implemented');

  return {
    areas: valuesModule.RequiredProofStatusMatrixClosureAreas,
    sourceProofRefs: proof.sourceProofRefs,
    nonClaims: proof.nonClaims,
    rows: proof.rows.map((row) => ({
      area: row.area,
      sourceProofRefs: row.sourceProofRefs,
      proofState: row.proofState,
      runtimeState: row.runtimeState,
      backendExecutionState: row.backendExecutionState,
      publicRuntimeState: row.publicRuntimeState,
      legalExecutionState: row.legalExecutionState,
      providerSecretCustodyState: row.providerSecretCustodyState,
      childActivityCustodyState: row.childActivityCustodyState,
    })),
    knownGaps: readModelModule.ProductionSupportProofStatusMatrixClosureKnownGaps,
  };
}

async function assertLinkedProofs() {
  const parentModules = [
    [
      'production-support-status-backend-execution-continuation-read-model.js',
      'ProductionSupportStatusBackendExecutionContinuationReadModel',
    ],
    [
      'production-support-status-backend-runtime-closure-read-model.js',
      'ProductionSupportStatusBackendRuntimeClosureReadModel',
    ],
    [
      'production-support-status-backend-durable-queue-runtime-read-model.js',
      'ProductionSupportStatusBackendDurableQueueRuntimeReadModel',
    ],
    [
      'production-support-publication-execution-status-read-model.js',
      'ProductionSupportPublicationExecutionStatusReadModel',
    ],
    [
      'production-support-publication-status-freshness-read-model.js',
      'ProductionSupportPublicationStatusFreshnessReadModel',
    ],
    ['production-support-legal-provider-readiness-read-model.js', 'ProductionSupportLegalProviderReadinessReadModel'],
    [
      'production-support-data-export-delete-lifecycle-read-model.js',
      'ProductionSupportDataExportDeleteLifecycleReadModel',
    ],
  ];
  const loggingModules = [
    ['status-backend-payload-custody-read-model.js', 'StatusBackendPayloadCustodyReadModel'],
    ['status-backend-redaction-manifest-read-model.js', 'StatusBackendRedactionManifestReadModel'],
    ['privacy-legal-disclosure-status-read-model.js', 'PrivacyLegalDisclosureReadModel'],
    ['provider-secret-rotation-revocation-status-read-model.js', 'ProviderSecretRotationRevocationStatusReadModel'],
    ['delete-executor-read-model.js', 'DeleteExecutorReadModel'],
  ];

  const parent = await readModelIds('parent-domain', parentModules);
  const logging = await readModelIds('logging-domain', loggingModules);
  return { parent, logging };
}

async function readModelIds(packageDirectory, modules) {
  const ids = {};
  for (const [fileName, exportName] of modules) {
    const module = await importBuiltModule(packageDirectory, fileName);
    const readModel = module[exportName];
    ids[fileName] = String(readModel.readModelId ?? readModel.schemaVersion);
    assert.equal(typeof ids[fileName], 'string', `${fileName} must expose a read model id`);
  }
  return ids;
}

async function assertDocumentationProof() {
  const docs = [
    'docs/features/production-distribution-support.md',
    'docs/product-capability-checklist.md',
    'packages/parent-domain/README.md',
  ];
  for (const path of docs) {
    assertIncludes(await readRepoFile(path), proofMode, `${path} proof note`);
  }
  return docs;
}

async function assertPackageExports() {
  const packageJson = JSON.parse(await readRepoFile('packages/parent-domain/package.json'));
  const missingExports = expectedPackageExports.filter((exportPath) => !packageJson.exports[exportPath]);
  assert.deepEqual(missingExports, []);
  return {
    state: 'added-package-json-exports',
    exports: expectedPackageExports,
    missingExports,
  };
}

async function importBuiltModule(packageDirectory, fileName) {
  return import(pathToFileURL(join(repoRoot, 'packages', packageDirectory, 'dist', fileName)).href);
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
