import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'app-install-purchase-runtime-delivery-receipt-boundary-proof');
const proofPath = join(outputDir, 'proof.json');
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/schema-domain']));
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/app-game-domain']));
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/app-game-domain',
      '--',
      'tests/unit/app-install-purchase-runtime-delivery-receipt-boundary-proof.test.ts',
    ])
  );

  const proofModule = await loadRuntimeDeliveryReceiptBoundaryProofModule();
  const parsedReadModel = proofModule.AppInstallPurchaseRuntimeDeliveryReceiptBoundaryProofReadModel;
  const summary = proofModule.summarizeAppInstallPurchaseRuntimeDeliveryReceiptBoundaryProof(parsedReadModel);

  assert.deepEqual(summary, {
    runtimeDeliveryReceiptBoundaryRows: 4,
    blockedReceiptRows: 3,
    manualRequiredRows: 1,
    receiptMissingRows: 3,
    readyReceiptRows: 0,
    childDeviceDeliveredRows: 0,
  });
  assert.deepEqual(
    parsedReadModel.runtimeDeliveryReceiptBoundaryRows.map(
      (row) =>
        `${row.sourceDecisionAction}:${row.runtimeDeliveryReceiptBoundaryState}:${row.childDeviceTransportReceiptState}`
    ),
    [
      'approve:receipt-blocked-waiting-runtime-artifacts:receipt-missing',
      'deny:receipt-blocked-waiting-runtime-artifacts:receipt-missing',
      'time-box:receipt-blocked-waiting-runtime-artifacts:receipt-missing',
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
    proofMode: 'app-install-purchase-runtime-delivery-receipt-boundary-proof',
    commands,
    packageExportState: 'not-claimed-new-public-export-deferred',
    checklistState: 'not-touched-shared-checklist-left-unlocked-for-e-c',
    evidence: {
      runtimeDeliveryReceiptBoundaryContract:
        'packages/app-game-domain/src/app-install-purchase-runtime-delivery-receipt-boundary-proof.ts',
      sourceDispatchPreflightContract:
        'packages/app-game-domain/src/app-install-purchase-external-runtime-transport-dispatch-preflight-proof.ts',
      contractTest:
        'packages/app-game-domain/tests/unit/app-install-purchase-runtime-delivery-receipt-boundary-proof.test.ts',
      featureDoc: 'docs/features/app-install-purchase-approval.md',
      expectationDoc: 'docs/expectations/app-install-purchase-approval.md',
      platformExpectationDoc: 'docs/expectations/platforms.md',
      packageExport: 'not-added',
      packageReadme: 'packages/app-game-domain/package.json',
      checklistRow: 'not-touched',
      output: relative(repoRoot, proofPath),
    },
    runtimeDeliveryReceiptBoundarySummary: summary,
    runtimeDeliveryReceiptBoundaryRows: parsedReadModel.runtimeDeliveryReceiptBoundaryRows.map((row) => ({
      sourceDecisionAction: row.sourceDecisionAction,
      sourceDispatchPreflightRowId: row.sourceDispatchPreflightRowId,
      sourceDispatchPreflightState: row.sourceDispatchPreflightState,
      sourceDispatchPacketState: row.sourceDispatchPacketState,
      sourceParentOwnedDispatchPacketRef: row.sourceParentOwnedDispatchPacketRef,
      parentOwnedReceiptBoundaryRef: row.parentOwnedReceiptBoundaryRef,
      childDeviceTransportReceiptExpectationRef: row.childDeviceTransportReceiptExpectationRef,
      runtimeDeliveryReceiptBoundaryState: row.runtimeDeliveryReceiptBoundaryState,
      childDeviceTransportReceiptState: row.childDeviceTransportReceiptState,
      runtimeDeliveryReceiptReadinessState: row.runtimeDeliveryReceiptReadinessState,
      requiredReceiptArtifactBlockers: row.requiredReceiptArtifactBlockers,
      externalWriterDispatchExecutionProofRefs: row.externalWriterDispatchExecutionProofRefs,
      providerStoreExecutionReceiptProofRefs: row.providerStoreExecutionReceiptProofRefs,
      platformAdapterExecutionReceiptProofRefs: row.platformAdapterExecutionReceiptProofRefs,
      childDeviceTransportReceiptProofRefs: row.childDeviceTransportReceiptProofRefs,
      receiptBlockedReasonRefs: row.receiptBlockedReasonRefs,
      receiptBoundaryAuditEventRefs: row.receiptBoundaryAuditEventRefs,
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
  console.log(`app-install-purchase-runtime-delivery-receipt-boundary-proof-ok:${relative(repoRoot, proofPath)}`);
}

async function loadRuntimeDeliveryReceiptBoundaryProofModule() {
  const modulePath = join(
    repoRoot,
    'packages',
    'app-game-domain',
    'dist',
    'app-install-purchase-runtime-delivery-receipt-boundary-proof.js'
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

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
