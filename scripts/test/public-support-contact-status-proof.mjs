import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'public-support-contact-status-proof';
const resultDir = join(repoRoot, 'test-results', proofMode);
const outputDir = join(repoRoot, 'output', proofMode);
const proofPath = join(resultDir, 'proof.json');
const summaryPath = join(outputDir, 'proof-summary.json');
const deterministicCheckedAt = 'deterministic-proof-artifact';
const deterministicCommit = 'branch-head-validated-by-harness';
const commands = [];
const requiredPackageExports = [
  '@ocentra-parent/schema-domain/public-support-contact-status-proof',
  '@ocentra-parent/schema-domain/public-support-contact-status-read-model',
  '@ocentra-parent/schema-domain/public-support-contact-status-values',
];
const retiredProductionDomainExport = './public-support-contact-status-proof';

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
      'tests/unit/public-support-contact-status-proof.test.ts',
    ])
  );

  const packageExport = await assertPackageExports();
  const contract = await assertBuiltContract();
  const documentation = await assertDocumentationProof();
  const proof = {
    schemaVersion: 1,
    checkedAt: deterministicCheckedAt,
    commit: deterministicCommit,
    proofMode,
    packageExport,
    commands,
    evidence: {
      contract: 'packages/schema-domain/src/public-support-contact-status-proof.ts',
      values: 'packages/schema-domain/src/public-support-contact-status-values.ts',
      readModel: 'packages/schema-domain/src/public-support-contact-status-read-model.ts',
      contractTest: 'packages/production-domain/tests/unit/public-support-contact-status-proof.test.ts',
      proofHarness: 'scripts/test/public-support-contact-status-proof.mjs',
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
    commit: proof.commit,
    proofMode,
    packageExport: proof.packageExport.state,
    rowCount: proof.rows.length,
    rows: proof.rows.map((row) => row.surface),
    output: relativePath(proofPath),
    knownGaps: proof.knownGaps,
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(summaryPath, `${JSON.stringify(summary, null, 2)}\n`);
  console.log(`${proofMode}-ok:${relativePath(proofPath)} ${relativePath(summaryPath)}`);
}

async function assertPackageExports() {
  const productionPackageJson = JSON.parse(await readRepoFile('packages/production-domain/package.json'));
  const [contract, readModel, values] = await Promise.all(requiredPackageExports.map((specifier) => import(specifier)));

  assert.equal(typeof contract.PublicSupportContactStatusProofSchema.parse, 'function');
  assert.equal(typeof readModel.PublicSupportContactStatusReadModel, 'object');
  assert(Object.keys(values).length > 0);
  assert.equal(productionPackageJson.exports[retiredProductionDomainExport], null);

  return {
    state: 'schema-domain-live-export-with-production-domain-retired',
    liveExports: requiredPackageExports,
    retiredExport: retiredProductionDomainExport,
  };
}

async function assertBuiltContract() {
  const contractModule = await importBuiltSchemaDomainModule('public-support-contact-status-proof');
  const readModelModule = await importBuiltSchemaDomainModule('public-support-contact-status-read-model');
  const valuesModule = await importBuiltSchemaDomainModule('public-support-contact-status-values');
  const proof = contractModule.PublicSupportContactStatusProofSchema.parse(
    readModelModule.PublicSupportContactStatusReadModel
  );

  assert.equal(typeof contractModule.decodePublicSupportContactStatusProof, 'function');
  assert.deepEqual(valuesModule.RequiredPublicSupportContactStatusSurfaces, [
    'public-support-contact',
    'support-status-page-contact',
    'support-runbook-contact',
    'incident-status-contact',
    'backend-upload-support-contact',
    'billing-support-contact',
  ]);
  assert.deepEqual(contractModule.summarizePublicSupportContactStatusRows(proof.rows), {
    'public-support-contact': 1,
    'support-status-page-contact': 1,
    'support-runbook-contact': 1,
    'incident-status-contact': 1,
    'backend-upload-support-contact': 1,
    'billing-support-contact': 1,
  });
  assert.equal(proof.publicRuntimeExecutionClaim, 'not-implemented');
  assert.equal(proof.supportBackendUploadExecutionClaim, 'manual-required');
  assert.equal(proof.accountLookupExecutionClaim, 'manual-required');
  assert.equal(proof.billingProviderContactClaim, 'manual-required');
  assert.equal(proof.remoteSupportSessionClaim, 'not-implemented');
  assert.equal(proof.productionSlaClaim, 'not-implemented');
  assert.equal(proof.legalDisclosureExecutionClaim, 'manual-required');
  assert.equal(proof.childActivityCustodyClaim, 'not-implemented');
  assert.deepEqual(
    proof.rows.map((row) => [row.surface, row.contactStatusBoundaryState, row.statusBoundaryReference]),
    [
      ['public-support-contact', 'backend-required', 'public-support-contact-status-boundary-public-support-contact'],
      [
        'support-status-page-contact',
        'manual-required',
        'public-support-contact-status-boundary-support-status-page-contact',
      ],
      ['support-runbook-contact', 'backend-required', 'public-support-contact-status-boundary-support-runbook-contact'],
      ['incident-status-contact', 'backend-required', 'public-support-contact-status-boundary-incident-status-contact'],
      [
        'backend-upload-support-contact',
        'backend-required',
        'public-support-contact-status-boundary-backend-upload-support-contact',
      ],
      ['billing-support-contact', 'backend-required', 'public-support-contact-status-boundary-billing-support-contact'],
    ]
  );
  assertContactRowsRemainManual(proof.rows);

  return {
    rows: proof.rows.map((row) => ({
      surface: row.surface,
      sourceProof: row.sourceProof,
      sourceContractState: row.sourceContractState,
      publicRouteState: row.publicRouteState,
      publicRuntimeState: row.publicRuntimeState,
      contactExecutionState: row.contactExecutionState,
      contactStatusBoundaryState: row.contactStatusBoundaryState,
      supportBackendUploadState: row.supportBackendUploadState,
      statusBoundaryReference: row.statusBoundaryReference,
      supportSafeDataClasses: row.supportSafeDataClasses,
    })),
    nonClaims: proof.nonClaims,
    knownGaps: readModelModule.PublicSupportContactStatusKnownGaps,
  };
}

async function importBuiltSchemaDomainModule(moduleName) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'schema-domain', 'dist', `${moduleName}.js`)).href);
}

function assertContactRowsRemainManual(rows) {
  for (const row of rows) {
    assert.notEqual(row.publicRouteState, 'implemented', `${row.surface} must not claim public route implementation`);
    assert.notEqual(row.publicRuntimeState, 'implemented', `${row.surface} must not claim public runtime`);
    assert.notEqual(row.contactExecutionState, 'executed', `${row.surface} must not claim executed contact`);
    assert.notEqual(row.contactStatusBoundaryState, 'implemented', `${row.surface} must not claim status backend`);
    assert.notEqual(
      row.contactStatusBoundaryState,
      'executed',
      `${row.surface} must not claim status backend execution`
    );
    assert.notEqual(row.supportBackendUploadState, 'executed', `${row.surface} must not claim upload execution`);
    for (const dataClass of row.forbiddenDataClasses) {
      assert(!row.supportSafeDataClasses.includes(dataClass), `${row.surface} unexpectedly allows ${dataClass}`);
    }
  }
}

async function assertDocumentationProof() {
  const docs = [
    'docs/features/production-distribution-support.md',
    'docs/expectations/release-installer.md',
    'docs/expectations/documentation.md',
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
