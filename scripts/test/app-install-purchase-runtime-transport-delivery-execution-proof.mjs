import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'app-install-purchase-runtime-transport-delivery-execution-proof');
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
    'tests/app-install-purchase-runtime-transport-delivery-execution-proof.test.ts',
  ]);

  const proofModule = await loadRuntimeTransportDeliveryExecutionProofModule();
  const parsedReadModel = proofModule.AppInstallPurchaseRuntimeTransportDeliveryExecutionProofReadModel;
  const summary = proofModule.summarizeAppInstallPurchaseRuntimeTransportDeliveryExecutionProof(parsedReadModel);

  assert.deepEqual(summary, {
    runtimeTransportDeliveryExecutionRows: 4,
    withheldExecutionRows: 3,
    manualRequiredRows: 1,
    transportAttemptsStartedRows: 0,
    deliveryResultRecordedRows: 0,
    childDeviceReceiptHandoffReadyRows: 0,
    externalRuntimeWriterDeliveredRows: 0,
  });
  assert.deepEqual(
    parsedReadModel.runtimeTransportDeliveryExecutionRows.map(
      (row) => `${row.sourceDecisionAction}:${row.runtimeTransportExecutionState}:${row.childDeviceReceiptHandoffState}`
    ),
    [
      'approve:execution-withheld-missing-artifacts:receipt-handoff-missing',
      'deny:execution-withheld-missing-artifacts:receipt-handoff-missing',
      'time-box:execution-withheld-missing-artifacts:receipt-handoff-missing',
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
    proofMode: 'app-install-purchase-runtime-transport-delivery-execution-proof',
    commands,
    packageExportState: 'added-app-install-purchase-runtime-transport-delivery-execution-proof-export',
    checklistState: 'deferred-docs-product-capability-checklist-locked-by-codex-b',
    evidence: {
      runtimeTransportDeliveryExecutionContract:
        'packages/parent-domain/src/app-install-purchase-runtime-transport-delivery-execution-proof.ts',
      sourceReceiptBoundaryContract:
        'packages/parent-domain/src/app-install-purchase-runtime-delivery-receipt-boundary-proof.ts',
      contractTest:
        'packages/parent-domain/tests/app-install-purchase-runtime-transport-delivery-execution-proof.test.ts',
      featureDoc: 'docs/features/app-install-purchase-approval.md',
      expectationDoc: 'docs/expectations/app-install-purchase-approval.md',
      platformExpectationDoc: 'docs/expectations/platforms.md',
      packageExport: 'packages/parent-domain/package.json',
      packageReadme: 'packages/parent-domain/README.md',
      checklistRow: 'deferred',
      output: relative(repoRoot, proofPath),
    },
    runtimeTransportDeliveryExecutionSummary: summary,
    runtimeTransportDeliveryExecutionRows: parsedReadModel.runtimeTransportDeliveryExecutionRows.map((row) => ({
      sourceDecisionAction: row.sourceDecisionAction,
      sourceReceiptBoundaryRowId: row.sourceReceiptBoundaryRowId,
      sourceReceiptBoundaryState: row.sourceReceiptBoundaryState,
      sourceChildDeviceTransportReceiptState: row.sourceChildDeviceTransportReceiptState,
      sourceParentOwnedDispatchPacketRef: row.sourceParentOwnedDispatchPacketRef,
      sourceParentOwnedReceiptBoundaryRef: row.sourceParentOwnedReceiptBoundaryRef,
      parentOwnedTransportExecutionAttemptRef: row.parentOwnedTransportExecutionAttemptRef,
      parentOwnedDeliveryResultReceiptRef: row.parentOwnedDeliveryResultReceiptRef,
      childDeviceReceiptHandoffRef: row.childDeviceReceiptHandoffRef,
      runtimeTransportExecutionState: row.runtimeTransportExecutionState,
      runtimeTransportAttemptState: row.runtimeTransportAttemptState,
      runtimeDeliveryResultState: row.runtimeDeliveryResultState,
      childDeviceReceiptHandoffState: row.childDeviceReceiptHandoffState,
      requiredRuntimeExecutionBlockers: row.requiredRuntimeExecutionBlockers,
      externalWriterDispatchExecutionProofRefs: row.externalWriterDispatchExecutionProofRefs,
      providerStoreExecutionReceiptProofRefs: row.providerStoreExecutionReceiptProofRefs,
      platformAdapterExecutionReceiptProofRefs: row.platformAdapterExecutionReceiptProofRefs,
      childDeviceTransportReceiptProofRefs: row.childDeviceTransportReceiptProofRefs,
      executionWithheldReasonRefs: row.executionWithheldReasonRefs,
      runtimeTransportDeliveryExecutionAuditEventRefs: row.runtimeTransportDeliveryExecutionAuditEventRefs,
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
  console.log(`app-install-purchase-runtime-transport-delivery-execution-proof-ok:${relative(repoRoot, proofPath)}`);
}

async function loadRuntimeTransportDeliveryExecutionProofModule() {
  const modulePath = join(
    repoRoot,
    'packages',
    'parent-domain',
    'dist',
    'app-install-purchase-runtime-transport-delivery-execution-proof.js'
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
