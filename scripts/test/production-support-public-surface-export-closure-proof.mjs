import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'production-support-public-surface-export-closure-proof';
const resultDir = join(repoRoot, 'test-results', proofMode);
const outputDir = join(repoRoot, 'output', proofMode);
const proofPath = join(resultDir, 'proof.json');
const summaryPath = join(outputDir, 'proof-summary.json');
const checkedAt = 'deterministic-proof-artifact';
const commit = 'branch-head-validated-by-harness';
const commands = [];

const expectedSchemaExports = [
  './production-release-public-status-proof',
  './production-release-public-status-proof-values',
  './production-release-public-status-freshness-proof',
  './production-release-public-status-freshness-values',
  './production-release-public-docs-freshness-proof',
  './production-release-public-docs-freshness-values',
  './public-support-contact-status-proof',
  './public-support-contact-status-read-model',
  './public-support-contact-status-values',
  './production-support-publication-execution-status-proof',
  './production-support-publication-execution-status-read-model',
  './production-support-publication-execution-status-values',
  './production-support-publication-runtime-readiness-proof',
  './production-support-publication-runtime-readiness-read-model',
  './production-support-publication-runtime-readiness-values',
  './production-support-publication-status-freshness-proof',
  './production-support-publication-status-freshness-read-model',
  './production-support-publication-status-freshness-values',
  './production-support-publication-workflow',
  './production-support-publication-workflow-read-model',
  './production-support-publication-workflow-values',
];

await main();

async function main() {
  await mkdir(resultDir, { recursive: true });
  await mkdir(outputDir, { recursive: true });
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/schema-domain']));
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/production-domain']));

  const packageExport = await assertPackageExports();
  const publicStatus = await assertProductionReleasePublicStatus();
  const publicStatusFreshness = await assertProductionReleasePublicStatusFreshness();
  const publicDocsFreshness = await assertProductionReleasePublicDocsFreshness();
  const publicationExecution = await assertProductionSupportPublicationExecutionStatus();
  const publicationRuntime = await assertProductionSupportPublicationRuntimeReadiness();
  const publicationFreshness = await assertProductionSupportPublicationStatusFreshness();
  const publicationWorkflow = await assertProductionSupportPublicationWorkflow();
  const publicSupportContact = await assertPublicSupportContactStatus();
  const documentation = await assertDocumentationProof();
  const proof = {
    schemaVersion: 1,
    checkedAt,
    commit,
    proofMode,
    commands,
    evidence: {
      packageJson: ['packages/production-domain/package.json', 'packages/schema-domain/package.json'],
      packageExport,
      publicStatus,
      publicStatusFreshness,
      publicDocsFreshness,
      publicationExecution,
      publicationRuntime,
      publicationFreshness,
      publicationWorkflow,
      publicSupportContact,
      proofHarness: 'scripts/test/production-support-public-surface-export-closure-proof.mjs',
      documentation,
      proofOutput: relativePath(proofPath),
      summaryOutput: relativePath(summaryPath),
    },
    knownGaps: [
      'This closure exposes existing source-contract proof modules through package exports; it does not implement public runtime execution.',
      'Status backend execution, support backend upload execution, account lookup, billing provider contact, legal disclosure execution, remote support sessions, production SLA, provider-secret custody, and child activity custody remain unclaimed.',
    ],
  };
  const summary = {
    schemaVersion: 1,
    checkedAt,
    commit,
    proofMode,
    exportCount: expectedSchemaExports.length,
    packageExport,
    documentation,
    output: relativePath(proofPath),
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(summaryPath, `${JSON.stringify(summary, null, 2)}\n`);
  console.log(`${proofMode}-ok:${relativePath(proofPath)} ${relativePath(summaryPath)}`);
}

async function assertPackageExports() {
  const productionPackageJson = JSON.parse(await readRepoFile('packages/production-domain/package.json'));
  const schemaPackageJson = JSON.parse(await readRepoFile('packages/schema-domain/package.json'));
  const missingSchemaExports = expectedSchemaExports.filter((exportPath) => !schemaPackageJson.exports[exportPath]);
  const retiredLocalExports = expectedSchemaExports.filter((exportPath) => productionPackageJson.exports[exportPath] !== null);
  assert.deepEqual(productionPackageJson.exports['./*'], {
    import: './dist/src/*.js',
    types: './dist/src/*.d.ts',
  });
  assert.deepEqual(missingSchemaExports, []);
  assert.deepEqual(retiredLocalExports, []);
  return {
    state: 'public-surface-package-exports-centralized',
    exports: ['wildcard-production-domain-surface'],
    schemaExports: expectedSchemaExports,
    missingSchemaExports,
    retiredLocalExports,
  };
}

async function assertProductionReleasePublicStatus() {
  const contract = await importBuiltSchemaDomainModule('production-release-public-status-proof');
  const values = await importBuiltSchemaDomainModule('production-release-public-status-proof-values');
  const proof = contract.ProductionReleasePublicStatusProofSchema.parse(
    contract.ProductionReleasePublicStatusProofReadModel
  );

  assert.equal(typeof contract.decodeProductionReleasePublicStatusProof, 'function');
  assert.deepEqual(values.RequiredPublicSurfaces, [
    'public-download',
    'release-status',
    'update-status',
    'account-status',
    'subscription-status',
    'support-status',
  ]);
  assert.deepEqual(contract.summarizeProductionReleasePublicStatusSurfaces(proof.surfaces), {
    'public-download': 1,
    'release-status': 1,
    'update-status': 1,
    'account-status': 1,
    'subscription-status': 1,
    'support-status': 1,
  });
  assert.equal(proof.publicHostState, 'not-implemented');
  assert.equal(proof.publicSupportRuntimeClaim, 'not-implemented');
  assert.equal(proof.childActivityCustodyClaim, 'not-implemented');

  return { exportPath: './production-release-public-status-proof', owner: 'schema-domain', rowCount: proof.surfaces.length };
}

async function assertProductionReleasePublicStatusFreshness() {
  const contract = await importBuiltSchemaDomainModule('production-release-public-status-freshness-proof');
  const values = await importBuiltSchemaDomainModule('production-release-public-status-freshness-values');
  const proof = contract.ProductionReleasePublicStatusFreshnessProofSchema.parse(
    contract.ProductionReleasePublicStatusFreshnessReadModel
  );

  assert.equal(typeof contract.decodeProductionReleasePublicStatusFreshnessProof, 'function');
  assert.equal(values.RequiredPublicStatusFreshnessSurfaces.length, 6);
  assert.equal(proof.publicRuntimeExecutionClaim, 'not-implemented');
  assert.equal(proof.accountBackendRuntimeClaim, 'backend-required');
  assert.equal(proof.supportBackendUploadState, 'manual-required');
  assert.equal(proof.childActivityCustodyClaim, 'not-implemented');

  return {
    exportPath: './production-release-public-status-freshness-proof',
    owner: 'schema-domain',
    rowCount: proof.rows.length,
  };
}

async function assertProductionReleasePublicDocsFreshness() {
  const contract = await importBuiltSchemaDomainModule('production-release-public-docs-freshness-proof');
  const values = await importBuiltSchemaDomainModule('production-release-public-docs-freshness-values');
  const proof = contract.ProductionReleasePublicDocsFreshnessProofSchema.parse(
    contract.ProductionReleasePublicDocsFreshnessReadModel
  );

  assert.equal(typeof contract.decodeProductionReleasePublicDocsFreshnessProof, 'function');
  assert.equal(values.RequiredPublicDocsFreshnessDocuments.length, 6);
  assert.equal(proof.publicPublicationClaim, 'manual-required');
  assert.equal(proof.legalDisclosureExecutionClaim, 'manual-required');
  assert.equal(proof.childActivityCustodyClaim, 'not-implemented');

  return {
    exportPath: './production-release-public-docs-freshness-proof',
    owner: 'schema-domain',
    rowCount: proof.rows.length,
  };
}

async function assertProductionSupportPublicationExecutionStatus() {
  const contract = await importBuiltSchemaDomainModule('production-support-publication-execution-status-proof');
  const readModel = await importBuiltSchemaDomainModule('production-support-publication-execution-status-read-model');
  const values = await importBuiltSchemaDomainModule('production-support-publication-execution-status-values');
  const proof = contract.ProductionSupportPublicationExecutionStatusProofSchema.parse(
    readModel.ProductionSupportPublicationExecutionStatusReadModel
  );

  assert.equal(typeof contract.decodeProductionSupportPublicationExecutionStatusProof, 'function');
  assert.equal(values.RequiredPublicationExecutionStatusTargets.length, 6);
  assert.deepEqual(values.RequiredPublicationExecutionStatusLifecycleStates, [
    'requested',
    'queued',
    'running',
    'succeeded',
    'failed',
    'manual-required',
  ]);
  assert.equal(proof.publicRuntimeExecutionClaim, 'not-implemented');
  assert.equal(proof.publicationRunnerExecutionClaim, 'manual-required');
  assert.equal(proof.childActivityCustodyClaim, 'not-implemented');

  return {
    exportPath: './production-support-publication-execution-status-proof',
    owner: 'schema-domain',
    rowCount: proof.rows.length,
  };
}

async function assertProductionSupportPublicationRuntimeReadiness() {
  const contract = await importBuiltSchemaDomainModule('production-support-publication-runtime-readiness-proof');
  const readModel = await importBuiltSchemaDomainModule('production-support-publication-runtime-readiness-read-model');
  const values = await importBuiltSchemaDomainModule('production-support-publication-runtime-readiness-values');
  const proof = contract.ProductionSupportPublicationRuntimeReadinessProofSchema.parse(
    readModel.ProductionSupportPublicationRuntimeReadinessReadModel
  );

  assert.equal(typeof contract.decodeProductionSupportPublicationRuntimeReadinessProof, 'function');
  assert.equal(values.RequiredPublicationRuntimeReadinessItems.length, 6);
  assert.equal(proof.publicRuntimeExecutionClaim, 'not-implemented');
  assert.equal(proof.publicationRunnerExecutionClaim, 'manual-required');
  assert.equal(proof.childActivityCustodyClaim, 'not-implemented');

  return {
    exportPath: './production-support-publication-runtime-readiness-proof',
    owner: 'schema-domain',
    rowCount: proof.rows.length,
  };
}

async function assertProductionSupportPublicationStatusFreshness() {
  const contract = await importBuiltSchemaDomainModule('production-support-publication-status-freshness-proof');
  const readModel = await importBuiltSchemaDomainModule('production-support-publication-status-freshness-read-model');
  const values = await importBuiltSchemaDomainModule('production-support-publication-status-freshness-values');
  const proof = contract.ProductionSupportPublicationStatusFreshnessProofSchema.parse(
    readModel.ProductionSupportPublicationStatusFreshnessReadModel
  );

  assert.equal(typeof contract.decodeProductionSupportPublicationStatusFreshnessProof, 'function');
  assert.equal(values.RequiredPublicationStatusFreshnessSurfaces.length, 6);
  assert.equal(proof.publicRuntimeClaim, 'not-implemented');
  assert.equal(proof.supportPublicationExecutionClaim, 'manual-required');
  assert.equal(proof.childActivityCustodyClaim, 'not-implemented');

  return {
    exportPath: './production-support-publication-status-freshness-proof',
    owner: 'schema-domain',
    rowCount: proof.rows.length,
  };
}

async function assertProductionSupportPublicationWorkflow() {
  const contract = await importBuiltSchemaDomainModule('production-support-publication-workflow');
  const readModel = await importBuiltSchemaDomainModule('production-support-publication-workflow-read-model');
  const values = await importBuiltSchemaDomainModule('production-support-publication-workflow-values');
  const proof = contract.ProductionSupportPublicationWorkflowProofSchema.parse(
    readModel.ProductionSupportPublicationWorkflowReadModel
  );

  assert.equal(typeof contract.decodeProductionSupportPublicationWorkflowProof, 'function');
  assert.equal(values.RequiredPublicationWorkflowItems.length, 6);
  assert.equal(proof.publicRuntimeClaim, 'not-implemented');
  assert.equal(proof.legalExecutionClaim, 'manual-required');
  assert.equal(proof.childActivityCustodyClaim, 'not-implemented');

  return {
    exportPath: './production-support-publication-workflow',
    owner: 'schema-domain',
    rowCount: proof.rows.length,
  };
}

async function assertPublicSupportContactStatus() {
  const contract = await importBuiltSchemaDomainModule('public-support-contact-status-proof');
  const readModel = await importBuiltSchemaDomainModule('public-support-contact-status-read-model');
  const values = await importBuiltSchemaDomainModule('public-support-contact-status-values');
  const proof = contract.PublicSupportContactStatusProofSchema.parse(readModel.PublicSupportContactStatusReadModel);

  assert.equal(typeof contract.decodePublicSupportContactStatusProof, 'function');
  assert.equal(values.RequiredPublicSupportContactStatusSurfaces.length, 6);
  assert.equal(proof.publicRuntimeExecutionClaim, 'not-implemented');
  assert.equal(proof.supportBackendUploadExecutionClaim, 'manual-required');
  assert.equal(proof.childActivityCustodyClaim, 'not-implemented');

  return { exportPath: './public-support-contact-status-proof', owner: 'schema-domain', rowCount: proof.rows.length };
}

async function importBuiltSchemaDomainModule(moduleName) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'schema-domain', 'dist', `${moduleName}.js`)).href);
}

async function assertDocumentationProof() {
  const docs = [
    'docs/features/production-distribution-support.md',
    'docs/product-capability-checklist.md',
  ];
  for (const path of docs) {
    assertIncludes(await readRepoFile(path), proofMode, `${path} proof note`);
  }
  return docs;
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
