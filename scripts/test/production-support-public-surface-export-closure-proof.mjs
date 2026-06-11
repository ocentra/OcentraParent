import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const proofMode = 'production-support-public-surface-export-closure-proof';
const resultDir = join(repoRoot, 'test-results', proofMode);
const outputDir = join(repoRoot, 'output', proofMode);
const proofPath = join(resultDir, 'proof.json');
const summaryPath = join(outputDir, 'proof-summary.json');
const checkedAt = 'deterministic-proof-artifact';
const commit = 'branch-head-validated-by-harness';
const commands = [];

const expectedPackageExports = [
  './production-release-public-status',
  './production-release-public-status-values',
  './production-release-public-status-freshness',
  './production-release-public-status-freshness-values',
  './production-release-public-docs-freshness',
  './production-release-public-docs-freshness-values',
  './production-support-publication-runtime-readiness',
  './production-support-publication-runtime-readiness-read-model',
  './production-support-publication-runtime-readiness-values',
  './production-support-publication-status-freshness',
  './production-support-publication-status-freshness-read-model',
  './production-support-publication-status-freshness-values',
  './public-support-contact-status',
  './public-support-contact-status-read-model',
  './public-support-contact-status-values',
];

await main();

async function main() {
  await mkdir(resultDir, { recursive: true });
  await mkdir(outputDir, { recursive: true });
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']));

  const packageExport = await assertPackageExports();
  const publicStatus = await assertProductionReleasePublicStatus();
  const publicStatusFreshness = await assertProductionReleasePublicStatusFreshness();
  const publicDocsFreshness = await assertProductionReleasePublicDocsFreshness();
  const publicationRuntime = await assertProductionSupportPublicationRuntimeReadiness();
  const publicationFreshness = await assertProductionSupportPublicationStatusFreshness();
  const publicSupportContact = await assertPublicSupportContactStatus();
  const documentation = await assertDocumentationProof();
  const proof = {
    schemaVersion: 1,
    checkedAt,
    commit,
    proofMode,
    commands,
    evidence: {
      packageJson: 'packages/parent-domain/package.json',
      packageExport,
      publicStatus,
      publicStatusFreshness,
      publicDocsFreshness,
      publicationRuntime,
      publicationFreshness,
      publicSupportContact,
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
    exportCount: expectedPackageExports.length,
    packageExport,
    documentation,
    output: relativePath(proofPath),
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(summaryPath, `${JSON.stringify(summary, null, 2)}\n`);
  console.log(`${proofMode}-ok:${relativePath(proofPath)} ${relativePath(summaryPath)}`);
}

async function assertPackageExports() {
  const packageJson = JSON.parse(await readRepoFile('packages/parent-domain/package.json'));
  const missingExports = expectedPackageExports.filter((exportPath) => !packageJson.exports[exportPath]);
  assert.deepEqual(missingExports, []);
  return {
    state: 'public-surface-package-exports-present',
    exports: expectedPackageExports,
    missingExports,
  };
}

async function assertProductionReleasePublicStatus() {
  const contract = await import('@ocentra-parent/parent-domain/production-release-public-status');
  const values = await import('@ocentra-parent/parent-domain/production-release-public-status-values');
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

  return { exportPath: './production-release-public-status', rowCount: proof.surfaces.length };
}

async function assertProductionReleasePublicStatusFreshness() {
  const contract = await import('@ocentra-parent/parent-domain/production-release-public-status-freshness');
  const values = await import('@ocentra-parent/parent-domain/production-release-public-status-freshness-values');
  const proof = contract.ProductionReleasePublicStatusFreshnessProofSchema.parse(
    contract.ProductionReleasePublicStatusFreshnessReadModel
  );

  assert.equal(typeof contract.decodeProductionReleasePublicStatusFreshnessProof, 'function');
  assert.equal(values.RequiredPublicStatusFreshnessSurfaces.length, 6);
  assert.equal(proof.publicRuntimeExecutionClaim, 'not-implemented');
  assert.equal(proof.accountBackendRuntimeClaim, 'backend-required');
  assert.equal(proof.supportBackendUploadState, 'manual-required');
  assert.equal(proof.childActivityCustodyClaim, 'not-implemented');

  return { exportPath: './production-release-public-status-freshness', rowCount: proof.rows.length };
}

async function assertProductionReleasePublicDocsFreshness() {
  const contract = await import('@ocentra-parent/parent-domain/production-release-public-docs-freshness');
  const values = await import('@ocentra-parent/parent-domain/production-release-public-docs-freshness-values');
  const proof = contract.ProductionReleasePublicDocsFreshnessProofSchema.parse(
    contract.ProductionReleasePublicDocsFreshnessReadModel
  );

  assert.equal(typeof contract.decodeProductionReleasePublicDocsFreshnessProof, 'function');
  assert.equal(values.RequiredPublicDocsFreshnessDocuments.length, 6);
  assert.equal(proof.publicPublicationClaim, 'manual-required');
  assert.equal(proof.legalDisclosureExecutionClaim, 'manual-required');
  assert.equal(proof.childActivityCustodyClaim, 'not-implemented');

  return { exportPath: './production-release-public-docs-freshness', rowCount: proof.rows.length };
}

async function assertProductionSupportPublicationRuntimeReadiness() {
  const contract = await import('@ocentra-parent/parent-domain/production-support-publication-runtime-readiness');
  const readModel =
    await import('@ocentra-parent/parent-domain/production-support-publication-runtime-readiness-read-model');
  const values = await import('@ocentra-parent/parent-domain/production-support-publication-runtime-readiness-values');
  const proof = contract.ProductionSupportPublicationRuntimeReadinessProofSchema.parse(
    readModel.ProductionSupportPublicationRuntimeReadinessReadModel
  );

  assert.equal(typeof contract.decodeProductionSupportPublicationRuntimeReadinessProof, 'function');
  assert.equal(values.RequiredPublicationRuntimeReadinessItems.length, 6);
  assert.equal(proof.publicRuntimeExecutionClaim, 'not-implemented');
  assert.equal(proof.publicationRunnerExecutionClaim, 'manual-required');
  assert.equal(proof.childActivityCustodyClaim, 'not-implemented');

  return { exportPath: './production-support-publication-runtime-readiness', rowCount: proof.rows.length };
}

async function assertProductionSupportPublicationStatusFreshness() {
  const contract = await import('@ocentra-parent/parent-domain/production-support-publication-status-freshness');
  const readModel =
    await import('@ocentra-parent/parent-domain/production-support-publication-status-freshness-read-model');
  const values = await import('@ocentra-parent/parent-domain/production-support-publication-status-freshness-values');
  const proof = contract.ProductionSupportPublicationStatusFreshnessProofSchema.parse(
    readModel.ProductionSupportPublicationStatusFreshnessReadModel
  );

  assert.equal(typeof contract.decodeProductionSupportPublicationStatusFreshnessProof, 'function');
  assert.equal(values.RequiredPublicationStatusFreshnessSurfaces.length, 6);
  assert.equal(proof.publicRuntimeClaim, 'not-implemented');
  assert.equal(proof.supportPublicationExecutionClaim, 'manual-required');
  assert.equal(proof.childActivityCustodyClaim, 'not-implemented');

  return { exportPath: './production-support-publication-status-freshness', rowCount: proof.rows.length };
}

async function assertPublicSupportContactStatus() {
  const contract = await import('@ocentra-parent/parent-domain/public-support-contact-status');
  const readModel = await import('@ocentra-parent/parent-domain/public-support-contact-status-read-model');
  const values = await import('@ocentra-parent/parent-domain/public-support-contact-status-values');
  const proof = contract.PublicSupportContactStatusProofSchema.parse(readModel.PublicSupportContactStatusReadModel);

  assert.equal(typeof contract.decodePublicSupportContactStatusProof, 'function');
  assert.equal(values.RequiredPublicSupportContactStatusSurfaces.length, 6);
  assert.equal(proof.publicRuntimeExecutionClaim, 'not-implemented');
  assert.equal(proof.supportBackendUploadExecutionClaim, 'manual-required');
  assert.equal(proof.childActivityCustodyClaim, 'not-implemented');

  return { exportPath: './public-support-contact-status', rowCount: proof.rows.length };
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
