import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { Logger } from '@ocentra-parent/logging-domain/core/logger';
import { getStackTrace } from '@ocentra-parent/logging-domain/core/stackTrace';

const repoRoot = process.cwd();
const proofMode = 'parent-owned-local-export-runtime-proof';
const contractPackageExport = '@ocentra-parent/schema-domain/parent-owned-local-export-runtime';
const valuesPackageExport = '@ocentra-parent/schema-domain/parent-owned-local-export-runtime-values';
const executorPackageExport = '@ocentra-parent/parent-domain/parent-owned-local-export-runtime-executor';
const outputDir = join(repoRoot, 'output', 'data-custody-storage-plan-proof', '05-export-import-backup-recovery');
const proofPath = join(outputDir, `${proofMode}.json`);
const testResultDir = join(repoRoot, 'test-results', proofMode);
const runtimeSmokeRoot = join(testResultDir, 'runtime-smoke');
const commands = [];
const log = Logger.instance;

log.register((import.meta && import.meta.url) || 'scripts/test/parent-owned-local-export-runtime-proof.mjs');

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });
  await rm(testResultDir, { recursive: true, force: true });
  await mkdir(testResultDir, { recursive: true });

  try {
    log.logInfo('Starting parent-owned local export runtime proof', getStackTrace(), { proofMode }, true);
    await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']));
    await runCommand(
      ...npmCommand([
        'run',
        'test',
        '--workspace',
        '@ocentra-parent/parent-domain',
        '--',
        'tests/unit/parent-owned-local-export-runtime.test.ts',
        'tests/unit/parent-owned-local-export-runtime-executor.test.ts',
      ]),
      { OCENTRA_PARENT_DOMAIN_TEST_SKIP_PROOF_CHAIN: '1' }
    );

    const proofModule = await loadContractProofModule();
    const valuesModule = await loadValuesModule();
    const executorModule = await loadExecutorModule();
    const readModel = proofModule.ParentOwnedLocalExportRuntimeProofReadModel;
    const stateCounts = proofModule.summarizeParentOwnedLocalExportRuntimeStates(readModel.jobs);
    const dataClassCounts = proofModule.summarizeParentOwnedLocalExportRuntimeDataClasses(readModel.jobs);
    const exportFixture = readModel.jobs.find((job) => job.state === 'export-written');
    if (exportFixture === undefined || exportFixture.output === null) {
      throw new Error('missing export-written proof fixture');
    }

    const runtimeExecutor = executorModule.createParentOwnedLocalExportRuntimeExecutor({
      runtimeRoot: runtimeSmokeRoot,
      encryptionSecret: 'parent-owned-local-export-runtime-proof-secret',
      loggingEnabled: true,
    });
    const exportResult = await runtimeExecutor.executeExport({
      scope: exportFixture.scope,
      payload: {
        recoveryBundleState: 'applied',
        deleteSettlementState: 'delete-confirmed',
        source: 'device-trust-recovery-persistence-proof',
      },
      sourceEvidenceRefs: exportFixture.output.sourceEvidenceRefs,
      auditRefs: exportFixture.auditRefs,
      jobId: 'proof-export-runtime-job',
      queueRef: 'proof-export-runtime-queue',
      requestedAt: '2026-06-18T06:20:00.000Z',
    });
    const bundleArtifactContainsPlaintextPayload = (await readFile(exportResult.bundlePath, 'utf8')).includes(
      'device-trust-recovery-persistence-proof'
    );
    const deleteResult = await runtimeExecutor.executeDelete({
      scope: exportFixture.scope,
      output: exportResult.job.output,
      auditRefs: exportFixture.auditRefs,
      jobId: 'proof-delete-runtime-job',
      queueRef: 'proof-delete-runtime-queue',
      requestedAt: '2026-06-18T06:21:00.000Z',
    });
    const missingTargetDeleteResult = await runtimeExecutor.executeDelete({
      scope: exportFixture.scope,
      output: exportResult.job.output,
      auditRefs: exportFixture.auditRefs,
      jobId: 'proof-delete-missing-runtime-job',
      queueRef: 'proof-delete-missing-runtime-queue',
      requestedAt: '2026-06-18T06:22:00.000Z',
    });
    const runtimeAuditEntries = await runtimeExecutor.readAuditEntries();

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
    assert.equal(exportResult.job.state, 'export-written');
    assert.equal(deleteResult.job.state, 'delete-confirmed');
    assert.equal(missingTargetDeleteResult.job.state, 'delete-failed');
    assert.equal(runtimeAuditEntries.length, 3);

    const proof = {
      schemaVersion: 1,
      checkedAt: new Date().toISOString(),
      commit: await gitHead(),
      proofMode,
      result: 'pass',
      commands,
      evidence: {
        contract: 'packages/schema-domain/src/parent-owned-local-export-runtime.ts',
        runtimeExecutor: 'packages/parent-domain/src/parent-owned-local-export-runtime-executor.ts',
        values: 'packages/schema-domain/src/parent-owned-local-export-runtime-values.ts',
        contractTest: 'packages/parent-domain/tests/unit/parent-owned-local-export-runtime.test.ts',
        runtimeExecutorTest: 'packages/parent-domain/tests/unit/parent-owned-local-export-runtime-executor.test.ts',
        runtimeExecutorPackageExport: executorPackageExport,
        packageExport: contractPackageExport,
        valuesPackageExport,
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
      runtimeExecution: {
        hostPlatform: process.platform,
        exportState: exportResult.job.state,
        deleteState: deleteResult.job.state,
        missingTargetDeleteState: missingTargetDeleteResult.job.state,
        missingTargetFailureReasonRef: missingTargetDeleteResult.job.deleteReceipt?.failureReasonRef ?? null,
        bundlePath: relativePath(exportResult.bundlePath),
        outputPath: relativePath(exportResult.outputPath),
        auditLogPath: relativePath(exportResult.auditLogPath),
        auditEntryCount: runtimeAuditEntries.length,
        bundleArtifactContainsPlaintextPayload,
      },
      knownGaps: valuesModule.ParentOwnedLocalExportRuntimeKnownGaps,
    };

    await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
    await writeFile(join(testResultDir, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
    log.logInfo(
      'Parent-owned local export runtime proof finished',
      getStackTrace(),
      { proofPath: relativePath(proofPath), auditEntryCount: runtimeAuditEntries.length },
      true
    );
    console.log(`${proofMode}-ok:${relativePath(proofPath)}`);
  } catch (error) {
    const blockedProof = {
      schemaVersion: 1,
      checkedAt: new Date().toISOString(),
      commit: await safeGitHead(),
      proofMode,
      result: 'blocked',
      commands,
      blocker: error instanceof Error ? error.message : String(error),
      evidence: {
        contract: 'packages/schema-domain/src/parent-owned-local-export-runtime.ts',
        runtimeExecutor: 'packages/parent-domain/src/parent-owned-local-export-runtime-executor.ts',
        contractTest: 'packages/parent-domain/tests/unit/parent-owned-local-export-runtime.test.ts',
        runtimeExecutorTest: 'packages/parent-domain/tests/unit/parent-owned-local-export-runtime-executor.test.ts',
        runtimeExecutorPackageExport: executorPackageExport,
        packageExport: contractPackageExport,
        valuesPackageExport,
        output: relativePath(proofPath),
      },
      noClaimBoundary:
        'Schema ownership is centralized in schema-domain. This blocked artifact does not prove parent-domain executor closure or broader parent-domain health.',
      nextAction:
        'Unblock parent-domain targeted build/test for the local export executor or its schema-domain contract consumption path.',
    };

    await writeFile(proofPath, `${JSON.stringify(blockedProof, null, 2)}\n`);
    console.warn(`${proofMode}-blocked:${relativePath(proofPath)}`);
  }
}

async function loadContractProofModule() {
  return import(contractPackageExport);
}

async function loadValuesModule() {
  return import(valuesPackageExport);
}

async function loadExecutorModule() {
  return import(executorPackageExport);
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

async function safeGitHead() {
  try {
    return await gitHead();
  } catch {
    return null;
  }
}

async function runCommand(command, args, extraEnv = {}) {
  const startedAt = new Date().toISOString();
  const child = spawn(command, args, {
    cwd: repoRoot,
    stdio: 'inherit',
    windowsHide: true,
    env: { ...process.env, ...extraEnv },
  });
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
