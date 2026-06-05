import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'production-release-public-runtime-handoff-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const proofPath = join(outputDir, 'proof.json');
const commands = [];

await main();

async function main() {
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
    'tests/production-release-public-runtime-handoff.test.ts',
  ]);

  const contract = await assertBuiltContract();
  const packageExports = await assertPublicPackageExports();
  const documentation = await assertDocumentationProof();
  const commit = await gitHead();
  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit,
    proofMode,
    commands,
    evidence: {
      contract: 'packages/parent-domain/src/production-release-public-runtime-handoff.ts',
      values: 'packages/parent-domain/src/production-release-public-runtime-handoff-values.ts',
      readModel: 'packages/parent-domain/src/production-release-public-runtime-handoff-read-model.ts',
      contractTest: 'packages/parent-domain/tests/production-release-public-runtime-handoff.test.ts',
      packageExports,
      documentation,
      output: relativePath(proofPath),
    },
    handoffRows: contract.handoffRows,
    adapterRows: contract.adapterRows,
    nonClaims: contract.nonClaims,
    knownGaps: contract.knownGaps,
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`${proofMode}-ok:${relativePath(proofPath)}`);
}

async function assertBuiltContract() {
  const contractModulePath = pathToFileURL(
    join(repoRoot, 'packages', 'parent-domain', 'dist', 'production-release-public-runtime-handoff.js')
  );
  const readModelPath = pathToFileURL(
    join(repoRoot, 'packages', 'parent-domain', 'dist', 'production-release-public-runtime-handoff-read-model.js')
  );
  const contractModule = await import(contractModulePath.href);
  const readModelModule = await import(readModelPath.href);
  const proof = contractModule.ProductionReleasePublicRuntimeHandoffProofSchema.parse(
    readModelModule.ProductionReleasePublicRuntimeHandoffReadModel
  );

  assert.deepEqual(contractModule.summarizeProductionReleasePublicRuntimeHandoffs(proof.handoffRows), {
    'public-download': 1,
    'release-status': 1,
    'update-status': 1,
    'account-status': 1,
    'subscription-status': 1,
    'support-status': 1,
  });
  assert.deepEqual(contractModule.summarizeProductionReleasePublicRuntimeAdapters(proof.adapterRows), {
    'public-website-runtime': 1,
    'download-status-backend': 1,
    'release-publishing-pipeline': 1,
    'updater-status-runtime': 1,
    'account-backend': 1,
    'billing-provider-runtime': 1,
    'support-backend-upload': 1,
  });
  assert.equal(proof.publicWebsiteRuntimeClaim, 'not-implemented');
  assert.equal(proof.accountBackendRuntimeClaim, 'backend-required');
  assert.equal(proof.billingProviderRuntimeClaim, 'not-implemented');
  assert.equal(proof.supportBackendUploadClaim, 'manual-required');
  assert.equal(proof.productionPublishingState, 'production-promotion-required');
  assert.equal(proof.signingStoreProofState, 'manual-required');
  assert.equal(proof.updaterExecutionState, 'manual-required');
  assert.equal(proof.childActivityCustodyClaim, 'not-implemented');

  return {
    handoffRows: proof.handoffRows.map((row) => ({
      surface: row.surface,
      handoffTarget: row.handoffTarget,
      routeState: row.routeState,
      runtimeAdapterState: row.runtimeAdapterState,
      backendAdapterState: row.backendAdapterState,
      sourceProof: row.sourceProof,
    })),
    adapterRows: proof.adapterRows.map((row) => ({
      adapter: row.adapter,
      adapterState: row.adapterState,
      executionClaim: row.executionClaim,
    })),
    nonClaims: proof.nonClaims,
    knownGaps: readModelModule.ProductionReleasePublicRuntimeHandoffKnownGaps,
  };
}

async function assertPublicPackageExports() {
  const contractModule = await import('@ocentra-parent/parent-domain/production-release-public-runtime-handoff');
  const readModelModule =
    await import('@ocentra-parent/parent-domain/production-release-public-runtime-handoff-read-model');
  const valuesModule = await import('@ocentra-parent/parent-domain/production-release-public-runtime-handoff-values');

  assert.equal(typeof contractModule.decodeProductionReleasePublicRuntimeHandoffProof, 'function');
  assert.ok(contractModule.ProductionReleasePublicRuntimeHandoffProofSchema);
  assert.ok(readModelModule.ProductionReleasePublicRuntimeHandoffReadModel);
  assert.deepEqual(valuesModule.RequiredPublicRuntimeSurfaces, [
    'public-download',
    'release-status',
    'update-status',
    'account-status',
    'subscription-status',
    'support-status',
  ]);

  return [
    '@ocentra-parent/parent-domain/production-release-public-runtime-handoff',
    '@ocentra-parent/parent-domain/production-release-public-runtime-handoff-read-model',
    '@ocentra-parent/parent-domain/production-release-public-runtime-handoff-values',
  ];
}

async function assertDocumentationProof() {
  const docs = [
    'docs/features/production-distribution-support.md',
    'docs/expectations/release-installer.md',
    'docs/expectations/platform-deliverables.md',
    'docs/expectations/cloud.md',
    'docs/expectations/billing.md',
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
