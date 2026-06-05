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
    'tests/public-support-contact-status-proof.test.ts',
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
      contract: 'packages/parent-domain/src/public-support-contact-status-proof.ts',
      values: 'packages/parent-domain/src/public-support-contact-status-values.ts',
      readModel: 'packages/parent-domain/src/public-support-contact-status-read-model.ts',
      contractTest: 'packages/parent-domain/tests/public-support-contact-status-proof.test.ts',
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
    rows: proof.rows.map((row) => row.surface),
    output: relativePath(proofPath),
    knownGaps: proof.knownGaps,
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(summaryPath, `${JSON.stringify(summary, null, 2)}\n`);
  console.log(`${proofMode}-ok:${relativePath(proofPath)} ${relativePath(summaryPath)}`);
}

async function assertBuiltContract() {
  const contractModule = await importBuiltParentDomainModule('public-support-contact-status-proof');
  const readModelModule = await importBuiltParentDomainModule('public-support-contact-status-read-model');
  const valuesModule = await importBuiltParentDomainModule('public-support-contact-status-values');
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
  assertContactRowsRemainManual(proof.rows);

  return {
    rows: proof.rows.map((row) => ({
      surface: row.surface,
      sourceProof: row.sourceProof,
      sourceContractState: row.sourceContractState,
      publicRouteState: row.publicRouteState,
      publicRuntimeState: row.publicRuntimeState,
      contactExecutionState: row.contactExecutionState,
      supportBackendUploadState: row.supportBackendUploadState,
      supportSafeDataClasses: row.supportSafeDataClasses,
    })),
    nonClaims: proof.nonClaims,
    knownGaps: readModelModule.PublicSupportContactStatusKnownGaps,
  };
}

async function importBuiltParentDomainModule(moduleName) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', `${moduleName}.js`)).href);
}

function assertContactRowsRemainManual(rows) {
  for (const row of rows) {
    assert.notEqual(row.publicRouteState, 'implemented', `${row.surface} must not claim public route implementation`);
    assert.notEqual(row.publicRuntimeState, 'implemented', `${row.surface} must not claim public runtime`);
    assert.notEqual(row.contactExecutionState, 'executed', `${row.surface} must not claim executed contact`);
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
    'packages/parent-domain/README.md',
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
