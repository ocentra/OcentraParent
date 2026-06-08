import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const outputDir = join(
  repoRoot,
  'test-results',
  'app-install-purchase-external-runtime-writer-transport-execution-proof'
);
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
    'tests/app-install-purchase-external-runtime-writer-transport-execution-proof.test.ts',
  ]);

  const proofModule = await loadExternalRuntimeWriterTransportExecutionProofModule();
  const parsedReadModel = proofModule.AppInstallPurchaseExternalRuntimeWriterTransportExecutionProofReadModel;
  const summary = proofModule.summarizeAppInstallPurchaseExternalRuntimeWriterTransportExecutionProof(parsedReadModel);

  assert.deepEqual(summary, {
    externalRuntimeWriterTransportExecutionRows: 4,
    blockedTransportExecutionRows: 3,
    manualRequiredRows: 1,
    withheldTransportPackets: 3,
    recordedTransportAcks: 0,
    externalRuntimeWriterExecutedRows: 0,
  });
  assert.deepEqual(
    parsedReadModel.externalRuntimeWriterTransportExecutionRows.map(
      (row) =>
        `${row.sourceDecisionAction}:${row.externalWriterTransportExecutionState}:${row.externalWriterTransportAckState}`
    ),
    [
      'approve:transport-execution-blocked:ack-not-recorded',
      'deny:transport-execution-blocked:ack-not-recorded',
      'time-box:transport-execution-blocked:ack-not-recorded',
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
    proofMode: 'app-install-purchase-external-runtime-writer-transport-execution-proof',
    commands,
    packageExportState: 'added-app-install-purchase-external-runtime-writer-transport-execution-proof-export',
    checklistState: 'updated-docs-product-capability-checklist-install-purchase-row',
    evidence: {
      externalRuntimeWriterTransportExecutionContract:
        'packages/parent-domain/src/app-install-purchase-external-runtime-writer-transport-execution-proof.ts',
      sourceRuntimeTransportDeliveryExecutionContract:
        'packages/parent-domain/src/app-install-purchase-runtime-transport-delivery-execution-proof.ts',
      contractTest:
        'packages/parent-domain/tests/app-install-purchase-external-runtime-writer-transport-execution-proof.test.ts',
      featureDoc: 'docs/features/app-install-purchase-approval.md',
      expectationDoc: 'docs/expectations/app-install-purchase-approval.md',
      platformExpectationDoc: 'docs/expectations/platforms.md',
      packageExport: 'packages/parent-domain/package.json',
      packageReadme: 'packages/parent-domain/README.md',
      checklistRow: 'docs/product-capability-checklist.md#install-purchase-approval',
      output: relative(repoRoot, proofPath),
    },
    externalRuntimeWriterTransportExecutionSummary: summary,
    externalRuntimeWriterTransportExecutionRows: parsedReadModel.externalRuntimeWriterTransportExecutionRows.map(
      (row) => ({
        sourceDecisionAction: row.sourceDecisionAction,
        sourceRuntimeTransportDeliveryExecutionRowId: row.sourceRuntimeTransportDeliveryExecutionRowId,
        sourceRuntimeTransportExecutionState: row.sourceRuntimeTransportExecutionState,
        sourceRuntimeTransportAttemptState: row.sourceRuntimeTransportAttemptState,
        sourceRuntimeDeliveryResultState: row.sourceRuntimeDeliveryResultState,
        sourceParentOwnedTransportExecutionAttemptRef: row.sourceParentOwnedTransportExecutionAttemptRef,
        sourceParentOwnedDeliveryResultReceiptRef: row.sourceParentOwnedDeliveryResultReceiptRef,
        sourceChildDeviceReceiptHandoffRef: row.sourceChildDeviceReceiptHandoffRef,
        parentOwnedExternalWriterTransportPacketRef: row.parentOwnedExternalWriterTransportPacketRef,
        parentOwnedExternalWriterTransportExecutionStatusRef: row.parentOwnedExternalWriterTransportExecutionStatusRef,
        parentOwnedExternalWriterTransportAckRef: row.parentOwnedExternalWriterTransportAckRef,
        externalWriterTransportExecutionState: row.externalWriterTransportExecutionState,
        externalWriterTransportPacketState: row.externalWriterTransportPacketState,
        externalWriterTransportAckState: row.externalWriterTransportAckState,
        requiredExternalWriterTransportExecutionBlockers: row.requiredExternalWriterTransportExecutionBlockers,
        externalWriterDispatchExecutorProofRefs: row.externalWriterDispatchExecutorProofRefs,
        providerStoreExecutionReceiptProofRefs: row.providerStoreExecutionReceiptProofRefs,
        platformAdapterExecutionReceiptProofRefs: row.platformAdapterExecutionReceiptProofRefs,
        childDeviceTransportReceiptProofRefs: row.childDeviceTransportReceiptProofRefs,
        transportExecutionBlockedReasonRefs: row.transportExecutionBlockedReasonRefs,
        externalWriterTransportExecutionAuditEventRefs: row.externalWriterTransportExecutionAuditEventRefs,
        externalRuntimeWriterExecutionClaim: row.externalRuntimeWriterExecutionClaim,
        externalRuntimeWriterDeliveryClaim: row.externalRuntimeWriterDeliveryClaim,
        providerApiExecutionClaim: row.providerApiExecutionClaim,
        platformAdapterClaim: row.platformAdapterClaim,
        childDeviceDeliveryClaim: row.childDeviceDeliveryClaim,
        appBlockingClaim: row.appBlockingClaim,
        claimBoundary: row.claimBoundary,
      })
    ),
    nonClaims: parsedReadModel.nonClaims,
    knownGaps: parsedReadModel.knownGaps,
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(
    `app-install-purchase-external-runtime-writer-transport-execution-proof-ok:${relative(repoRoot, proofPath)}`
  );
}

async function loadExternalRuntimeWriterTransportExecutionProofModule() {
  const modulePath = join(
    repoRoot,
    'packages',
    'parent-domain',
    'dist',
    'app-install-purchase-external-runtime-writer-transport-execution-proof.js'
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
