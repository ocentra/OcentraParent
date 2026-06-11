import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const proofMode = 'production-support-publication-workflow-proof';
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
      'tests/production-support-publication-workflow.test.ts',
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
      contract: 'packages/parent-domain/src/production-support-publication-workflow.ts',
      values: 'packages/parent-domain/src/production-support-publication-workflow-values.ts',
      readModel: 'packages/parent-domain/src/production-support-publication-workflow-read-model.ts',
      contractTest: 'packages/parent-domain/tests/production-support-publication-workflow.test.ts',
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
  const contractModule = await import('@ocentra-parent/parent-domain/production-support-publication-workflow');
  const readModelModule =
    await import('@ocentra-parent/parent-domain/production-support-publication-workflow-read-model');
  const valuesModule = await import('@ocentra-parent/parent-domain/production-support-publication-workflow-values');
  const proof = contractModule.ProductionSupportPublicationWorkflowProofSchema.parse(
    readModelModule.ProductionSupportPublicationWorkflowReadModel
  );

  assert.equal(typeof contractModule.decodeProductionSupportPublicationWorkflowProof, 'function');
  assert.deepEqual(valuesModule.RequiredPublicationWorkflowItems, [
    'public-privacy-policy-publication',
    'privacy-legal-disclosure-execution',
    'support-runbook-publication',
    'support-incident-status-publication',
    'support-backend-upload-publication-handoff',
    'public-support-contact-publication',
  ]);
  assert.deepEqual(contractModule.summarizeProductionSupportPublicationWorkflowRows(proof.rows), {
    'public-privacy-policy-publication': 1,
    'privacy-legal-disclosure-execution': 1,
    'support-runbook-publication': 1,
    'support-incident-status-publication': 1,
    'support-backend-upload-publication-handoff': 1,
    'public-support-contact-publication': 1,
  });
  assert.equal(proof.publicRuntimeClaim, 'not-implemented');
  assert.equal(proof.legalExecutionClaim, 'manual-required');
  assert.equal(proof.supportBackendUploadExecutionClaim, 'manual-required');
  assert.equal(proof.accountLookupExecutionClaim, 'manual-required');
  assert.equal(proof.billingProviderContactClaim, 'manual-required');
  assert.equal(proof.productionSlaClaim, 'not-implemented');
  assert.equal(proof.childActivityCustodyClaim, 'not-implemented');
  assertPublicationRowsRemainManual(proof.rows);

  return {
    rows: proof.rows.map((row) => ({
      item: row.item,
      sourceProof: row.sourceProof,
      sourceContractState: row.sourceContractState,
      publicPublicationState: row.publicPublicationState,
      legalExecutionState: row.legalExecutionState,
      supportBackendUploadState: row.supportBackendUploadState,
      supportSafeDataClasses: row.supportSafeDataClasses,
    })),
    nonClaims: proof.nonClaims,
    knownGaps: readModelModule.ProductionSupportPublicationWorkflowKnownGaps,
  };
}

function assertPublicationRowsRemainManual(rows) {
  for (const row of rows) {
    assert.notEqual(row.publicPublicationState, 'implemented', `${row.item} must not claim implemented publication`);
    assert.notEqual(row.legalExecutionState, 'executed', `${row.item} must not claim legal execution`);
    assert.notEqual(row.supportBackendUploadState, 'executed', `${row.item} must not claim support upload execution`);
    for (const dataClass of row.forbiddenDataClasses) {
      assert(!row.supportSafeDataClasses.includes(dataClass), `${row.item} unexpectedly allows ${dataClass}`);
    }
  }
}

async function assertDocumentationProof() {
  const docs = ['docs/features/production-distribution-support.md', 'packages/parent-domain/README.md'];
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

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
