import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'app-install-purchase-dispatch-executor-receipt-proof');
const proofPath = join(outputDir, 'proof.json');
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });
  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/schema-domain']);
  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'tests/app-install-purchase-dispatch-executor-receipt-proof.test.ts',
  ]);

  const proofModule = await loadDispatchExecutorReceiptProofModule();
  const parsedReadModel = proofModule.AppInstallPurchaseDispatchExecutorReceiptProofReadModel;
  const summary = proofModule.summarizeAppInstallPurchaseDispatchExecutorReceiptProof(parsedReadModel);

  assert.deepEqual(summary, {
    dispatchExecutorReceiptRows: 4,
    blockedDispatchExecutorRows: 3,
    manualRequiredRows: 1,
    acceptedDispatchExecutorArtifacts: 0,
    externalRuntimeWriterExecutedRows: 0,
    childDeviceDeliveredRows: 0,
  });
  assert.deepEqual(
    parsedReadModel.dispatchExecutorReceiptRows.map(
      (row) =>
        `${row.sourceDecisionAction}:${row.dispatchExecutorReceiptState}:${row.dispatchExecutorReceiptArtifactState}`
    ),
    [
      'approve:dispatch-executor-receipt-blocked:artifact-missing',
      'deny:dispatch-executor-receipt-blocked:artifact-missing',
      'time-box:dispatch-executor-receipt-blocked:artifact-missing',
      'review-needed:manual-required:manual-required',
    ]
  );
  assert.equal(parsedReadModel.nonClaims.includes('no-external-runtime-writer-execution'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-external-runtime-writer-delivery'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-provider-api-execution'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-platform-adapter-implementation'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-child-device-delivery'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-app-blocking'), true);

  const proof = {
    schemaVersion: 1,
    checkedAt: parsedReadModel.updatedAt,
    commitMetadataState: 'omitted-for-deterministic-proof-artifact',
    proofMode: 'app-install-purchase-dispatch-executor-receipt-proof',
    commands,
    packageExportState: 'deferred-e-c-currently-locks-packages-parent-domain-package-json',
    checklistState: 'deferred-e-c-currently-locks-docs-product-capability-checklist',
    packageReadmeState: 'deferred-e-c-currently-locks-packages-parent-domain-readme',
    evidence: {
      dispatchExecutorReceiptContract:
        'packages/parent-domain/src/app-install-purchase-dispatch-executor-receipt-proof.ts',
      sourceExecutionReceiptGateContract:
        'packages/parent-domain/src/app-install-purchase-execution-receipt-gate-proof.ts',
      contractTest: 'packages/parent-domain/tests/app-install-purchase-dispatch-executor-receipt-proof.test.ts',
      featureDoc: 'docs/features/app-install-purchase-approval.md',
      expectationDoc: 'docs/expectations/app-install-purchase-approval.md',
      platformExpectationDoc: 'docs/expectations/platforms.md',
      deferredPackageExport: 'packages/parent-domain/package.json',
      deferredPackageReadme: 'packages/parent-domain/README.md',
      deferredChecklistRow: 'docs/product-capability-checklist.md#install-purchase-approval',
      output: relative(repoRoot, proofPath),
    },
    dispatchExecutorReceiptSummary: summary,
    dispatchExecutorReceiptRows: parsedReadModel.dispatchExecutorReceiptRows.map((row) => ({
      sourceDecisionAction: row.sourceDecisionAction,
      sourceExecutionReceiptGateRowId: row.sourceExecutionReceiptGateRowId,
      sourceExecutionReceiptGateState: row.sourceExecutionReceiptGateState,
      sourceExternalWriterDispatchExecutorReceiptState: row.sourceExternalWriterDispatchExecutorReceiptState,
      dispatchExecutorReceiptState: row.dispatchExecutorReceiptState,
      dispatchExecutorReceiptArtifactState: row.dispatchExecutorReceiptArtifactState,
      requiredDispatchExecutorArtifacts: row.requiredDispatchExecutorArtifacts,
      dispatchExecutorHandlerProofRefs: row.dispatchExecutorHandlerProofRefs,
      dispatchExecutorReceiptArtifactRefs: row.dispatchExecutorReceiptArtifactRefs,
      dispatchExecutorAuditArtifactRefs: row.dispatchExecutorAuditArtifactRefs,
      dispatchExecutorBlockedReasonRefs: row.dispatchExecutorBlockedReasonRefs,
      dispatchExecutorAuditEventRefs: row.dispatchExecutorAuditEventRefs,
      externalRuntimeWriterExecutionClaim: row.externalRuntimeWriterExecutionClaim,
      externalRuntimeWriterDeliveryClaim: row.externalRuntimeWriterDeliveryClaim,
      providerApiExecutionClaim: row.providerApiExecutionClaim,
      platformAdapterClaim: row.platformAdapterClaim,
      childDeviceDeliveryClaim: row.childDeviceDeliveryClaim,
      appBlockingClaim: row.appBlockingClaim,
      claimBoundary: row.claimBoundary,
    })),
    nonClaims: parsedReadModel.nonClaims,
    knownGaps: parsedReadModel.knownGaps,
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`app-install-purchase-dispatch-executor-receipt-proof-ok:${relative(repoRoot, proofPath)}`);
}

async function loadDispatchExecutorReceiptProofModule() {
  const modulePath = join(
    repoRoot,
    'packages',
    'parent-domain',
    'dist',
    'app-install-purchase-dispatch-executor-receipt-proof.js'
  );
  return import(pathToFileURL(modulePath).href);
}

async function runCommand(command, args) {
  const child = spawn(command, args, { cwd: repoRoot, stdio: 'inherit' });
  const exitCode = await new Promise((resolve) => {
    child.on('close', resolve);
  });
  commands.push({ command: `${command} ${args.join(' ')}`, exitCode });
  if (exitCode !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed with ${exitCode}`);
  }
}
