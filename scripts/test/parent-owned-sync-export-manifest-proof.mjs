import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'output', 'data-custody-storage-plan-proof', '03-parent-owned-cloud-sync');
const proofPath = join(outputDir, 'parent-owned-sync-export-manifest-proof.json');
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });

  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/production-domain']));
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/production-domain',
      '--',
      'tests/unit/parent-owned-sync-export.test.ts',
    ])
  );

  const proofModule = await loadContractProofModule();
  await assertPackageExport(proofModule);
  const readModel = proofModule.ParentOwnedSyncExportContractProofReadModel;
  const dataClassCounts = proofModule.summarizeParentOwnedSyncExportDataClasses(readModel.manifest.items);
  const connectorStatusCounts = proofModule.summarizeParentOwnedSyncExportConnectorStatuses(
    readModel.connectorStatuses
  );
  const recoveryBundleStateCounts = proofModule.summarizeParentOwnedSyncExportRecoveryBundleStates(
    readModel.recoveryBundles
  );
  const recoveryHandoffStateCounts = proofModule.summarizeParentOwnedSyncExportRecoveryHandoffStates(
    readModel.recoveryBundles
  );

  assert.equal(
    Object.values(dataClassCounts).every((count) => count === 1),
    true
  );
  assert.equal(
    Object.values(connectorStatusCounts).every((count) => count === 1),
    true
  );
  assert.equal(readModel.transferRuntimeClaimed, false);
  assert.equal(readModel.connectorOAuthClaimed, false);
  assert.equal(readModel.ocentraHostedChildEvidenceStored, false);
  assert.deepEqual(
    readModel.importResults.map((result) => result.resultState),
    ['accepted-preview', 'rejected-schema-version', 'rejected-scope', 'not-applied']
  );
  assert.deepEqual(
    readModel.deleteResults.map((result) => result.resultState),
    ['pending', 'confirmed', 'failed', 'not-requested']
  );
  assert.equal(recoveryBundleStateCounts.bundlePreviewOnly, 1);
  assert.equal(recoveryBundleStateCounts.bundleApplyPending, 1);
  assert.equal(recoveryBundleStateCounts.bundleApplied, 1);
  assert.equal(recoveryBundleStateCounts.bundleWrongHousehold, 1);
  assert.equal(recoveryBundleStateCounts.bundleWrongKey, 1);
  assert.equal(recoveryBundleStateCounts.bundleCorrupt, 1);
  assert.equal(recoveryBundleStateCounts.bundleManualRequired, 1);
  assert.equal(recoveryHandoffStateCounts['delete-pending'], 1);
  assert.equal(recoveryHandoffStateCounts['delete-confirmed'], 1);

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    proofMode: 'parent-owned-sync-export-manifest-proof',
    commands,
    evidence: {
      contract: 'packages/production-domain/src/parent-owned-sync-export.ts',
      contractTest: 'packages/production-domain/tests/unit/parent-owned-sync-export.test.ts',
      builtModule: 'packages/production-domain/dist/src/parent-owned-sync-export.js',
      packageExport: '@ocentra-parent/production-domain/parent-owned-sync-export',
      featureDoc: 'docs/features/reports-notifications-sync.md',
      expectationDoc: 'docs/expectations/sync-export.md',
      output: relative(repoRoot, proofPath),
    },
    dataClassCounts,
    connectorStatusCounts,
    syncCursorStates: readModel.syncCursors.map((cursor) => cursor.cursorState),
    conflictResolutions: readModel.conflictRecords.map((record) => record.resolution),
    importResultStates: readModel.importResults.map((result) => result.resultState),
    deleteResultStates: readModel.deleteResults.map((result) => result.resultState),
    recoveryBundleStates: recoveryBundleStateCounts,
    recoveryHandoffStates: recoveryHandoffStateCounts,
    recoveryHandoffTargets: [...new Set(readModel.recoveryBundles.map((bundle) => bundle.handoff.handoffTarget))],
    nonClaims: readModel.nonClaims,
    claimBoundaries: {
      transferRuntimeClaimed: readModel.transferRuntimeClaimed,
      connectorOAuthClaimed: readModel.connectorOAuthClaimed,
      portalUiClaimed: readModel.portalUiClaimed,
      reportCompilerRuntimeClaimed: readModel.reportCompilerRuntimeClaimed,
      accountSubscriptionBackendClaimed: readModel.accountSubscriptionBackendClaimed,
      ocentraHostedChildEvidenceStored: readModel.ocentraHostedChildEvidenceStored,
    },
    knownGaps: proofModule.ParentOwnedSyncExportKnownGaps,
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`parent-owned-sync-export-manifest-proof-ok:${relative(repoRoot, proofPath)}`);
}

async function loadContractProofModule() {
  const modulePath = join(repoRoot, 'packages', 'production-domain', 'dist', 'src', 'parent-owned-sync-export.js');
  return import(pathToFileURL(modulePath).href);
}

async function assertPackageExport(proofModule) {
  const exportedModule = await import('@ocentra-parent/production-domain/parent-owned-sync-export');
  assert.equal(
    exportedModule.ParentOwnedSyncExportContractProofReadModel.manifest.manifestId,
    proofModule.ParentOwnedSyncExportContractProofReadModel.manifest.manifestId
  );
}

async function gitHead() {
  const output = await commandOutput('git', ['rev-parse', 'HEAD']);
  return output.trim();
}

async function commandOutput(command, args) {
  const chunks = [];
  const child = spawn(command, args, { cwd: repoRoot, stdio: ['ignore', 'pipe', 'pipe'] });
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
  const child = spawn(command, args, { cwd: repoRoot, stdio: 'inherit' });
  const exitCode = await new Promise((resolve) => {
    child.on('close', resolve);
  });
  commands.push({ command: `${command} ${args.join(' ')}`, startedAt, exitCode });
  if (exitCode !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed with ${exitCode}`);
  }
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
