import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'app-install-purchase-external-runtime-writer-readiness-proof');
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
    'tests/app-install-purchase-external-runtime-writer-readiness-proof.test.ts',
  ]);

  const proofModule = await loadExternalRuntimeWriterReadinessProofModule();
  const packageProofModule =
    await import('@ocentra-parent/parent-domain/app-install-purchase-external-runtime-writer-readiness-proof');
  assert.equal(
    packageProofModule.AppInstallPurchaseExternalRuntimeWriterReadinessProofReadModel.schemaVersion,
    proofModule.AppInstallPurchaseExternalRuntimeWriterReadinessProofReadModel.schemaVersion
  );

  const parsedReadModel = proofModule.AppInstallPurchaseExternalRuntimeWriterReadinessProofReadModel;
  const summary = proofModule.summarizeAppInstallPurchaseExternalRuntimeWriterReadinessProof(parsedReadModel);

  assert.deepEqual(summary, {
    externalRuntimeWriterReadinessRows: 4,
    writerHandoffReadyRows: 3,
    queuePreflightReadyRows: 3,
    manualRequiredRows: 1,
    externalRuntimeWriterExecutedRows: 0,
    externalRuntimeWriterDeliveredRows: 0,
  });
  assert.deepEqual(
    parsedReadModel.externalRuntimeWriterReadinessRows.map(
      (row) => `${row.sourceDecisionAction}:${row.externalRuntimeWriterReadinessState}`
    ),
    [
      'approve:writer-handoff-ready',
      'deny:writer-handoff-ready',
      'time-box:writer-handoff-ready',
      'review-needed:manual-required',
    ]
  );
  assert.equal(parsedReadModel.nonClaims.includes('no-external-runtime-writer-execution'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-external-runtime-writer-delivery'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-parent-action-runtime-delivery'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-platform-interception'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-child-device-delivery'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-app-blocking'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-ocentra-hosted-family-data-custody'), true);

  const proof = {
    schemaVersion: 1,
    checkedAt: parsedReadModel.updatedAt,
    commitMetadataState: 'omitted-for-deterministic-proof-artifact',
    proofMode: 'app-install-purchase-external-runtime-writer-readiness-proof',
    commands,
    packageExportState: 'validated-via-public-parent-domain-subpath-export',
    checklistState: 'updated-app-install-purchase-approval-row',
    evidence: {
      externalRuntimeWriterReadinessContract:
        'packages/parent-domain/src/app-install-purchase-external-runtime-writer-readiness-proof.ts',
      sourceExternalRuntimeDeviceDeliveryContract:
        'packages/parent-domain/src/app-install-purchase-external-runtime-device-delivery-proof.ts',
      contractTest: 'packages/parent-domain/tests/app-install-purchase-external-runtime-writer-readiness-proof.test.ts',
      featureDoc: 'docs/features/app-install-purchase-approval.md',
      expectationDoc: 'docs/expectations/app-install-purchase-approval.md',
      platformExpectationDoc: 'docs/expectations/platforms.md',
      packageExport: '@ocentra-parent/parent-domain/app-install-purchase-external-runtime-writer-readiness-proof',
      packageReadme: 'packages/parent-domain/README.md',
      checklistRow: 'docs/product-capability-checklist.md row Install/purchase approval',
      output: relative(repoRoot, proofPath),
    },
    externalRuntimeWriterReadinessSummary: summary,
    externalRuntimeWriterReadinessRows: parsedReadModel.externalRuntimeWriterReadinessRows.map((row) => ({
      sourceDecisionAction: row.sourceDecisionAction,
      sourceExternalRuntimeDeviceDeliveryRowId: row.sourceExternalRuntimeDeviceDeliveryRowId,
      sourceExternalRuntimeEvidenceState: row.sourceExternalRuntimeEvidenceState,
      sourceRuntimeWriterEnvelopeRef: row.sourceRuntimeWriterEnvelopeRef,
      sourceDeliveryResultReceiptRef: row.sourceDeliveryResultReceiptRef,
      sourceExternalRuntimeWriterTargetRefs: row.sourceExternalRuntimeWriterTargetRefs,
      externalRuntimeWriterReadinessState: row.externalRuntimeWriterReadinessState,
      externalRuntimeWriterQueueState: row.externalRuntimeWriterQueueState,
      externalRuntimeWriterPreflightRef: row.externalRuntimeWriterPreflightRef,
      externalRuntimeWriterReceiptRef: row.externalRuntimeWriterReceiptRef,
      externalRuntimeWriterTargetRefs: row.externalRuntimeWriterTargetRefs,
      externalRuntimeWriterAuditEventRefs: row.externalRuntimeWriterAuditEventRefs,
      childDeliveryAuditEventRefs: row.childDeliveryAuditEventRefs,
      reportRuntimeRefs: row.reportRuntimeRefs,
      externalRuntimeWriterExecutionClaim: row.externalRuntimeWriterExecutionClaim,
      externalRuntimeWriterDeliveryClaim: row.externalRuntimeWriterDeliveryClaim,
      parentActionRuntimeDeliveryClaim: row.parentActionRuntimeDeliveryClaim,
      providerApiExecutionClaim: row.providerApiExecutionClaim,
      storeIntegrationClaim: row.storeIntegrationClaim,
      platformInterceptionClaim: row.platformInterceptionClaim,
      platformAdapterClaim: row.platformAdapterClaim,
      childDeviceDeliveryClaim: row.childDeviceDeliveryClaim,
      runtimeReportDeliveryClaim: row.runtimeReportDeliveryClaim,
      appBlockingClaim: row.appBlockingClaim,
      childDataCustody: row.childDataCustody,
      ocentraHostedFamilyDataCustodyClaim: row.ocentraHostedFamilyDataCustodyClaim,
      claimBoundary: row.claimBoundary,
    })),
    nonClaims: parsedReadModel.nonClaims,
    knownGaps: parsedReadModel.knownGaps,
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`app-install-purchase-external-runtime-writer-readiness-proof-ok:${relative(repoRoot, proofPath)}`);
}

async function loadExternalRuntimeWriterReadinessProofModule() {
  const modulePath = join(
    repoRoot,
    'packages',
    'parent-domain',
    'dist',
    'app-install-purchase-external-runtime-writer-readiness-proof.js'
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
