import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'production-support-process-runtime-status-proof';
const resultDir = join(repoRoot, 'test-results', proofMode);
const outputDir = join(repoRoot, 'output', proofMode);
const proofPath = join(resultDir, 'proof.json');
const summaryPath = join(outputDir, 'proof-summary.json');
const commands = [];
const deterministicCheckedAt = 'deterministic-proof-artifact';
const deterministicCommit = 'branch-head-validated-by-harness';
const requiredPackageExports = [
  '@ocentra-parent/schema-domain/production-support-process-runtime-status-proof',
  '@ocentra-parent/schema-domain/production-support-process-runtime-status-read-model',
  '@ocentra-parent/schema-domain/production-support-process-runtime-status-values',
];
const retiredProductionDomainExport = './production-support-process-runtime-status-proof';

await main();

async function main() {
  await mkdir(resultDir, { recursive: true });
  await mkdir(outputDir, { recursive: true });
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/schema-domain']));
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/production-domain',
      '--',
      'tests/unit/production-support-process-runtime-status-proof.test.ts',
    ])
  );

  const contract = await assertBuiltContract();
  const documentation = await assertDocumentationProof();
  const packageExport = await assertPackageExports();
  const proof = {
    schemaVersion: 1,
    checkedAt: deterministicCheckedAt,
    commit: deterministicCommit,
    proofMode,
    commands,
    evidence: {
      contract: 'packages/schema-domain/src/production-support-process-runtime-status-proof.ts',
      values: 'packages/schema-domain/src/production-support-process-runtime-status-values.ts',
      readModel: 'packages/schema-domain/src/production-support-process-runtime-status-read-model.ts',
      contractTest: 'packages/production-domain/tests/unit/production-support-process-runtime-status-proof.test.ts',
      proofHarness: 'scripts/test/production-support-process-runtime-status-proof.mjs',
      packageExport,
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
    commit: deterministicCommit,
    proofMode,
    rowCount: proof.rows.length,
    rows: proof.rows.map((row) => row.surface),
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
    'schema-domain',
    'production-support-process-runtime-status-proof.js'
  );
  const readModelModule = await importBuiltModule(
    'schema-domain',
    'production-support-process-runtime-status-read-model.js'
  );
  const proof = contractModule.ProductionSupportProcessRuntimeStatusProofSchema.parse(
    readModelModule.ProductionSupportProcessRuntimeStatusReadModel
  );

  assert.equal(typeof contractModule.decodeProductionSupportProcessRuntimeStatusProof, 'function');
  assert.deepEqual(contractModule.summarizeProductionSupportProcessRuntimeStatusRows(proof.rows), {
    'support-process-requested': 1,
    'parent-consent-authorized': 1,
    'privacy-legal-queued': 1,
    'redaction-review-running': 1,
    'backend-upload-failed': 1,
    'case-resolution-succeeded': 1,
    'support-process-manual-required': 1,
    'incident-runtime-requested': 1,
    'incident-runtime-authorized': 1,
    'incident-runtime-running': 1,
    'incident-runtime-evidence-ready': 1,
    'incident-runtime-manual-required': 1,
  });
  assert.equal(proof.backendUploadExecutionState, 'manual-required');
  assert.equal(proof.publicRuntimeExecutionState, 'not-implemented');
  assert.equal(proof.providerExecutionState, 'not-implemented');
  assert.equal(proof.incidentRuntimeExecutionState, 'manual-required');
  assert.equal(proof.productionSlaState, 'not-implemented');
  assert.equal(proof.remoteSupportSessionState, 'not-implemented');
  assert.equal(proof.childActivityCustodyState, 'not-implemented');

  return {
    rows: proof.rows.map((row) => ({
      surface: row.surface,
      sourceProof: row.sourceProof,
      runtimeState: row.runtimeState,
      backendUploadState: row.backendUploadState,
      supportSafeDataClasses: row.supportSafeDataClasses,
      runtimeReference: row.runtimeReference,
      auditReference: row.auditReference,
      manualRequirement: row.manualRequirement,
    })),
    nonClaims: proof.nonClaims,
    knownGaps: readModelModule.ProductionSupportProcessRuntimeStatusKnownGaps,
  };
}

async function assertDocumentationProof() {
  const docs = [
    'docs/features/production-distribution-support.md',
    'docs/expectations/data-custody.md',
    'docs/expectations/documentation.md',
    'docs/product-capability-checklist.md',
  ];
  for (const path of docs) {
    assertIncludes(await readRepoFile(path), proofMode, `${path} proof note`);
  }
  return docs;
}

async function assertPackageExports() {
  const [contract, readModel, values] = await Promise.all(requiredPackageExports.map((specifier) => import(specifier)));
  const productionDomainPackageJson = JSON.parse(await readRepoFile('packages/production-domain/package.json'));

  assert.equal(typeof contract.ProductionSupportProcessRuntimeStatusProofSchema.parse, 'function');
  assert.equal(typeof readModel.ProductionSupportProcessRuntimeStatusReadModel, 'object');
  assert(Object.keys(values).length > 0);
  assert.equal(productionDomainPackageJson.exports[retiredProductionDomainExport], null);

  return {
    state: 'schema-domain-live-export-with-production-domain-retired',
    liveExports: requiredPackageExports,
    retiredExport: retiredProductionDomainExport,
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
