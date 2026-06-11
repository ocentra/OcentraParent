import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'production-support-account-sla-status-proof';
const resultDir = join(repoRoot, 'test-results', proofMode);
const outputDir = join(repoRoot, 'output', proofMode);
const proofPath = join(resultDir, 'proof.json');
const summaryPath = join(outputDir, 'proof-summary.json');
const commands = [];

await main();

async function main() {
  await mkdir(resultDir, { recursive: true });
  await mkdir(outputDir, { recursive: true });
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']));
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/parent-domain',
      '--',
      'tests/production-support-account-sla-status-proof.test.ts',
    ])
  );

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
      contract: 'packages/parent-domain/src/production-support-account-sla-status-proof.ts',
      values: 'packages/parent-domain/src/production-support-account-sla-status-values.ts',
      readModel: 'packages/parent-domain/src/production-support-account-sla-status-read-model.ts',
      contractTest: 'packages/parent-domain/tests/production-support-account-sla-status-proof.test.ts',
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
  const contractModule = await importBuiltParentDomainModule('production-support-account-sla-status-proof');
  const readModelModule = await importBuiltParentDomainModule('production-support-account-sla-status-read-model');
  const proof = contractModule.ProductionSupportAccountSlaStatusProofSchema.parse(
    readModelModule.ProductionSupportAccountSlaStatusReadModel
  );

  assert.equal(typeof contractModule.decodeProductionSupportAccountSlaStatusProof, 'function');
  assert.deepEqual(contractModule.summarizeProductionSupportAccountSlaStatusRows(proof.rows), {
    'account-lookup-request-status': 1,
    'account-lookup-result-boundary': 1,
    'billing-provider-contact-status': 1,
    'remote-support-request-status': 1,
    'remote-support-session-boundary': 1,
    'production-sla-status': 1,
  });
  assert.equal(proof.accountLookupExecutionState, 'manual-required');
  assert.equal(proof.billingProviderContactState, 'manual-required');
  assert.equal(proof.remoteSupportSessionState, 'not-implemented');
  assert.equal(proof.productionSlaState, 'not-implemented');
  assert.equal(proof.supportBackendUploadExecutionState, 'manual-required');
  assert.equal(proof.familyOcentraRuntimeState, 'not-implemented');
  assert.equal(proof.childActivityCustodyState, 'not-implemented');

  return {
    rows: proof.rows.map((row) => ({
      surface: row.surface,
      sourceProof: row.sourceProof,
      accountLookupState: row.accountLookupState,
      billingProviderContactState: row.billingProviderContactState,
      remoteSupportSessionState: row.remoteSupportSessionState,
      productionSlaState: row.productionSlaState,
      supportSafeDataClasses: row.supportSafeDataClasses,
    })),
    nonClaims: proof.nonClaims,
    knownGaps: readModelModule.ProductionSupportAccountSlaStatusKnownGaps,
  };
}

async function assertDocumentationProof() {
  const docs = [
    'docs/features/production-distribution-support.md',
    'docs/expectations/release-installer.md',
    'docs/expectations/billing.md',
    'docs/expectations/data-custody.md',
  ];
  for (const path of docs) {
    assertIncludes(await readRepoFile(path), proofMode, `${path} proof note`);
  }
  return docs;
}

async function importBuiltParentDomainModule(moduleName) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', `${moduleName}.js`)).href);
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
