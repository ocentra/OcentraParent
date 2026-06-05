import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'public-status-surface-readiness-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const outputProofDir = join(repoRoot, 'output', proofMode);
const proofPath = join(outputDir, 'proof.json');
const summaryPath = join(outputProofDir, 'proof-summary.json');
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });
  await mkdir(outputProofDir, { recursive: true });
  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'tests/public-status-surface-readiness.test.ts',
  ]);

  const publicStatus = await assertPublicStatusSurfaceReadinessProof();
  const existingStatus = await assertExistingPublicStatusProof();
  const existingRuntime = await assertExistingRuntimeHandoffProof();
  await assertDocumentationProof();
  const commit = await gitHead();
  const knownGaps = [
    'family.ocentra.ca public runtime is not implemented.',
    'Public download, account, subscription, release, update, and support status remain source-contract or adapter-boundary readiness only.',
    'Account backend, billing provider runtime, support backend upload, signing/store proof, updater execution, production SLA, legal execution, and remote support sessions remain manual-required or unimplemented.',
    'No child activity, raw support bundle payloads, provider secrets, billing-provider contact records, remote transcripts, or parent rules are hosted by this proof.',
  ];
  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit,
    proofMode,
    commands,
    evidence: {
      publicStatusSurfaceReadinessContract: 'packages/parent-domain/src/public-status-surface-readiness.ts',
      publicStatusSurfaceReadinessTest: 'packages/parent-domain/tests/public-status-surface-readiness.test.ts',
      existingPublicStatusSurfaceCount: existingStatus.surfaces.length,
      existingRuntimeHandoffCount: existingRuntime.handoffRows.length,
      output: relativePath(proofPath),
      summary: relativePath(summaryPath),
    },
    rows: publicStatus.rows,
    nonClaims: publicStatus.nonClaims,
    knownGaps,
  };
  const summary = {
    proofMode,
    commit,
    publicHost: publicStatus.publicHost,
    rowCount: publicStatus.rows.length,
    accountBackendRuntimeClaim: publicStatus.accountBackendRuntimeClaim,
    billingProviderRuntimeClaim: publicStatus.billingProviderRuntimeClaim,
    supportBackendUploadClaim: publicStatus.supportBackendUploadClaim,
    childActivityCustodyClaim: publicStatus.childActivityCustodyClaim,
    knownGaps,
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(summaryPath, `${JSON.stringify(summary, null, 2)}\n`);
  console.log(`${proofMode}-ok:${relativePath(proofPath)} ${relativePath(summaryPath)}`);
}

async function assertPublicStatusSurfaceReadinessProof() {
  const modulePath = pathToFileURL(
    join(repoRoot, 'packages', 'parent-domain', 'dist', 'public-status-surface-readiness.js')
  );
  const proofModule = await import(modulePath.href);
  const proof = proofModule.PublicStatusSurfaceReadinessProofSchema.parse(
    proofModule.PublicStatusSurfaceReadinessReadModel
  );

  assert.deepEqual(proofModule.summarizePublicStatusSurfaceReadinessRows(proof.rows), {
    'family-public-site': 1,
    'public-download': 1,
    'release-status': 1,
    'update-status': 1,
    'account-status': 1,
    'subscription-status': 1,
    'support-status': 1,
  });
  assert.equal(proof.publicWebsiteRuntimeClaim, 'not-implemented');
  assert.equal(proof.accountBackendRuntimeClaim, 'backend-required');
  assert.equal(proof.billingProviderRuntimeClaim, 'not-implemented');
  assert.equal(proof.supportBackendUploadClaim, 'manual-required');
  assert.equal(proof.productionSlaClaim, 'not-implemented');
  assert.equal(proof.legalExecutionClaim, 'manual-required');
  assert.equal(proof.remoteSupportSessionClaim, 'not-implemented');
  assert.equal(proof.childActivityCustodyClaim, 'not-implemented');
  for (const row of proof.rows) {
    assertForbiddenDataExcluded(row.supportSafeDataClasses, row.forbiddenDataClasses, row.surface);
  }
  return proof;
}

async function assertExistingPublicStatusProof() {
  const modulePath = pathToFileURL(
    join(repoRoot, 'packages', 'parent-domain', 'dist', 'production-release-public-status-proof.js')
  );
  const proofModule = await import(modulePath.href);
  const proof = proofModule.ProductionReleasePublicStatusProofSchema.parse(
    proofModule.ProductionReleasePublicStatusProofReadModel
  );

  assert.equal(proof.publicHostState, 'not-implemented');
  assert.equal(proof.childActivityCustodyClaim, 'not-implemented');
  return proof;
}

async function assertExistingRuntimeHandoffProof() {
  const contractModule = await import('@ocentra-parent/parent-domain/production-release-public-runtime-handoff');
  const readModelModule =
    await import('@ocentra-parent/parent-domain/production-release-public-runtime-handoff-read-model');
  const proof = contractModule.ProductionReleasePublicRuntimeHandoffProofSchema.parse(
    readModelModule.ProductionReleasePublicRuntimeHandoffReadModel
  );

  assert.equal(proof.publicWebsiteRuntimeClaim, 'not-implemented');
  assert.equal(proof.accountBackendRuntimeClaim, 'backend-required');
  assert.equal(proof.billingProviderRuntimeClaim, 'not-implemented');
  return proof;
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
}

function assertForbiddenDataExcluded(allowedDataClasses, forbiddenDataClasses, label) {
  for (const dataClass of forbiddenDataClasses) {
    assert(!allowedDataClasses.includes(dataClass), `${label} unexpectedly allows ${dataClass}`);
  }
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
