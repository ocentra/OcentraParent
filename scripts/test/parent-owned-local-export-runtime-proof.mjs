import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'parent-owned-local-export-runtime-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const proofPath = join(outputDir, 'proof.json');
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });

  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']));
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/parent-domain',
      '--',
      'tests/parent-owned-local-export-runtime.test.ts',
    ])
  );

  const proofModule = await loadContractProofModule();
  await assertPackageExport(proofModule);
  const readModel = proofModule.ParentOwnedLocalExportRuntimeProofReadModel;
  const stateCounts = proofModule.summarizeParentOwnedLocalExportRuntimeStates(readModel.jobs);
  const dataClassCounts = proofModule.summarizeParentOwnedLocalExportRuntimeDataClasses(readModel.jobs);

  assert.equal(
    Object.values(stateCounts).every((count) => count === 1),
    true
  );
  assert.equal(dataClassCounts['encrypted-journal-segment'], 8);
  assert.equal(dataClassCounts['sqlite-query-row'], 8);
  assert.equal(dataClassCounts['generated-summary'], 8);
  assert.equal(readModel.cloudTransferRuntimeClaimed, false);
  assert.equal(readModel.connectorOAuthClaimed, false);
  assert.equal(readModel.ocentraHostedFamilyDataCustodyClaimed, false);
  assert.equal(readModel.rawEvidenceUploadClaimed, false);

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    proofMode,
    commands,
    evidence: {
      contract: 'packages/parent-domain/src/parent-owned-local-export-runtime.ts',
      values: 'packages/parent-domain/src/parent-owned-local-export-runtime-values.ts',
      contractTest: 'packages/parent-domain/tests/parent-owned-local-export-runtime.test.ts',
      builtModule: 'packages/parent-domain/dist/parent-owned-local-export-runtime.js',
      packageExport: '@ocentra-parent/parent-domain/parent-owned-local-export-runtime',
      featureDoc: 'docs/features/reports-notifications-sync.md',
      expectationDocs: ['docs/expectations/sync-export.md', 'docs/expectations/data-custody.md'],
      output: relativePath(proofPath),
    },
    stateCounts,
    dataClassCounts,
    storageStates: readModel.jobs.map((job) => job.storageState),
    deleteStates: readModel.jobs
      .filter((job) => job.operation === 'delete')
      .map((job) => ({ state: job.state, confirmed: job.deleteReceipt?.deleteConfirmed ?? false })),
    nonClaims: readModel.nonClaims,
    claimBoundaries: {
      cloudTransferRuntimeClaimed: readModel.cloudTransferRuntimeClaimed,
      connectorOAuthClaimed: readModel.connectorOAuthClaimed,
      providerApiClaimed: readModel.providerApiClaimed,
      portalUiClaimed: readModel.portalUiClaimed,
      ocentraHostedFamilyDataCustodyClaimed: readModel.ocentraHostedFamilyDataCustodyClaimed,
      remoteReportCompilerClaimed: readModel.remoteReportCompilerClaimed,
      childDeviceMutationClaimed: readModel.childDeviceMutationClaimed,
      rawEvidenceUploadClaimed: readModel.rawEvidenceUploadClaimed,
    },
    knownGaps: proofModule.ParentOwnedLocalExportRuntimeKnownGaps,
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`${proofMode}-ok:${relativePath(proofPath)}`);
}

async function loadContractProofModule() {
  const modulePath = join(repoRoot, 'packages', 'parent-domain', 'dist', 'parent-owned-local-export-runtime.js');
  return import(pathToFileURL(modulePath).href);
}

async function assertPackageExport(proofModule) {
  const packageJson = JSON.parse(await readFile(join(repoRoot, 'packages', 'parent-domain', 'package.json'), 'utf8'));
  assert.deepEqual(packageJson.exports['./parent-owned-local-export-runtime'], {
    import: './dist/parent-owned-local-export-runtime.js',
    types: './dist/parent-owned-local-export-runtime.d.ts',
  });

  const exportedModule = await import('@ocentra-parent/parent-domain/parent-owned-local-export-runtime');
  assert.equal(
    exportedModule.ParentOwnedLocalExportRuntimeProofReadModel.schemaVersion,
    proofModule.ParentOwnedLocalExportRuntimeProofReadModel.schemaVersion
  );
}

async function gitHead() {
  const output = await commandOutput('git', ['rev-parse', 'HEAD']);
  return output.trim();
}

async function commandOutput(command, args) {
  const chunks = [];
  const child = spawn(command, args, { cwd: repoRoot, stdio: ['ignore', 'pipe', 'pipe'], windowsHide: true });
  child.stdout.on('data', (chunk) => chunks.push(chunk));
  child.stderr.on('data', (chunk) => chunks.push(chunk));
  const exitCode = await new Promise((resolve) => {
    child.on('close', resolve);
  });
  const output = Buffer.concat(chunks).toString('utf8');
  if (exitCode !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed with ${exitCode}\n${output}`);
  }
  return output;
}

async function runCommand(command, args) {
  const startedAt = new Date().toISOString();
  const child = spawn(command, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
  const exitCode = await new Promise((resolve) => {
    child.on('close', resolve);
  });
  commands.push({ command: `${command} ${args.join(' ')}`, startedAt, exitCode });
  if (exitCode !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed with ${exitCode}`);
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
