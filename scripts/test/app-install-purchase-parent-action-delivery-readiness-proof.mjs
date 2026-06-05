import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'app-install-purchase-parent-action-delivery-readiness-proof');
const proofPath = join(outputDir, 'proof.json');
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });
  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'tests/app-install-purchase-parent-action-delivery-readiness-proof.test.ts',
  ]);

  const proofModule = await loadParentActionDeliveryReadinessProofModule();
  const parsedReadModel = proofModule.AppInstallPurchaseParentActionDeliveryReadinessProofReadModel;
  const summary = proofModule.summarizeAppInstallPurchaseParentActionDeliveryReadinessProof(parsedReadModel);

  assert.deepEqual(summary, {
    parentActionDeliveryReadinessRows: 4,
    parentActionDeliveryReadyRows: 3,
    manualReviewRequiredRows: 1,
    childEnvelopeLinkedRows: 4,
    parentActionDeliveredRows: 0,
    runtimeWriterExecutedRows: 0,
  });
  assert.deepEqual(
    parsedReadModel.parentActionDeliveryReadinessRows.map(
      (row) =>
        `${row.sourceDecisionAction}:${row.sourceRuntimeHandoffStatus}:${row.sourceChildDeliveryEnvelopeState}:${row.parentActionDeliveryReadinessState}`
    ),
    [
      'approve:queued-for-runtime-writer:child-delivery-envelope-ready:parent-action-delivery-ready',
      'deny:queued-for-runtime-writer:child-delivery-envelope-ready:parent-action-delivery-ready',
      'time-box:queued-for-runtime-writer:child-delivery-envelope-ready:parent-action-delivery-ready',
      'review-needed:manual-review-required:manual-review-required:manual-review-required',
    ]
  );
  assert.equal(parsedReadModel.nonClaims.includes('no-parent-action-runtime-delivery'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-runtime-writer-execution'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-runtime-writer-delivery'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-provider-api-execution'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-child-device-delivery'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-ocentra-hosted-family-data-custody'), true);

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    proofMode: 'app-install-purchase-parent-action-delivery-readiness-proof',
    commands,
    packageExportState:
      'PENDING_LOCK: packages/parent-domain/package.json was locked by codex-b when this proof was added.',
    checklistState:
      'PENDING_LOCK: docs/product-capability-checklist.md was locked by codex-a when this proof was added.',
    evidence: {
      parentActionDeliveryReadinessContract:
        'packages/parent-domain/src/app-install-purchase-parent-action-delivery-readiness-proof.ts',
      sourceParentActionRuntimeHandoffContract:
        'packages/parent-domain/src/app-install-purchase-parent-action-runtime-handoff-proof.ts',
      sourceChildDeviceDeliveryRuntimeWriterContract:
        'packages/parent-domain/src/app-install-purchase-child-device-delivery-runtime-writer-proof.ts',
      contractTest: 'packages/parent-domain/tests/app-install-purchase-parent-action-delivery-readiness-proof.test.ts',
      featureDoc: 'docs/features/app-install-purchase-approval.md',
      expectationDoc: 'docs/expectations/app-install-purchase-approval.md',
      packageExport:
        'PENDING: add packages/parent-domain package export for ./app-install-purchase-parent-action-delivery-readiness-proof after codex-b releases package.json.',
      checklistRow:
        'PENDING: update docs/product-capability-checklist.md Install/purchase approval row after codex-a releases the checklist lock.',
      output: relative(repoRoot, proofPath),
    },
    parentActionDeliveryReadinessSummary: summary,
    parentActionDeliveryReadinessRows: parsedReadModel.parentActionDeliveryReadinessRows.map((row) => ({
      sourceDecisionAction: row.sourceDecisionAction,
      sourceRuntimeHandoffStatus: row.sourceRuntimeHandoffStatus,
      sourceChildDeliveryEnvelopeState: row.sourceChildDeliveryEnvelopeState,
      parentActionDeliveryReadinessState: row.parentActionDeliveryReadinessState,
      parentActionAuditEventRefs: row.parentActionAuditEventRefs,
      childDeliveryTargetRefs: row.childDeliveryTargetRefs,
      reportRuntimeRefs: row.reportRuntimeRefs,
      parentActionRuntimeDeliveryClaim: row.parentActionRuntimeDeliveryClaim,
      runtimeWriterExecutionClaim: row.runtimeWriterExecutionClaim,
      runtimeWriterDeliveryClaim: row.runtimeWriterDeliveryClaim,
      providerApiExecutionClaim: row.providerApiExecutionClaim,
      storeIntegrationClaim: row.storeIntegrationClaim,
      platformAdapterClaim: row.platformAdapterClaim,
      childDeviceDeliveryClaim: row.childDeviceDeliveryClaim,
      runtimeReportDeliveryClaim: row.runtimeReportDeliveryClaim,
      interceptionClaim: row.interceptionClaim,
      appBlockingClaim: row.appBlockingClaim,
      childDataCustody: row.childDataCustody,
      ocentraHostedFamilyDataCustodyClaim: row.ocentraHostedFamilyDataCustodyClaim,
      claimBoundary: row.claimBoundary,
    })),
    nonClaims: parsedReadModel.nonClaims,
    knownGaps: parsedReadModel.knownGaps,
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`app-install-purchase-parent-action-delivery-readiness-proof-ok:${relative(repoRoot, proofPath)}`);
}

async function loadParentActionDeliveryReadinessProofModule() {
  const modulePath = join(
    repoRoot,
    'packages',
    'parent-domain',
    'dist',
    'app-install-purchase-parent-action-delivery-readiness-proof.js'
  );
  return import(pathToFileURL(modulePath).href);
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
