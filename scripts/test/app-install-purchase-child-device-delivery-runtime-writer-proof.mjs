import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'app-install-purchase-child-device-delivery-runtime-writer-proof');
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
    'tests/app-install-purchase-child-device-delivery-runtime-writer-proof.test.ts',
  ]);

  const proofModule = await loadChildDeviceDeliveryRuntimeWriterProofModule();
  const parsedReadModel = proofModule.AppInstallPurchaseChildDeviceDeliveryRuntimeWriterProofReadModel;
  const summary = proofModule.summarizeAppInstallPurchaseChildDeviceDeliveryRuntimeWriterProof(parsedReadModel);
  const parentDomainPackageJson = JSON.parse(
    await readFile(join(repoRoot, 'packages', 'parent-domain', 'package.json'), 'utf8')
  );
  const packageExportKey = './app-install-purchase-child-device-delivery-runtime-writer-proof';

  assert.deepEqual(summary, {
    childDeviceDeliveryRuntimeWriterRows: 4,
    childDeliveryEnvelopeReadyRows: 3,
    manualReviewRequiredRows: 1,
    packageSourceCaptureLinkedRows: 4,
    runtimeWriterExecutedRows: 0,
    childDeviceDeliveredRows: 0,
  });
  assert.deepEqual(
    parsedReadModel.childDeviceDeliveryRuntimeWriterRows.map(
      (row) => `${row.sourceDecisionAction}:${row.sourceRuntimeWriterDeliveryState}:${row.childDeliveryEnvelopeState}`
    ),
    [
      'approve:writer-envelope-ready:child-delivery-envelope-ready',
      'deny:writer-envelope-ready:child-delivery-envelope-ready',
      'time-box:writer-envelope-ready:child-delivery-envelope-ready',
      'review-needed:manual-review-required:manual-review-required',
    ]
  );
  assert.equal(parsedReadModel.nonClaims.includes('no-runtime-writer-execution'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-runtime-writer-delivery'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-child-device-delivery'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-provider-api-execution'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-ocentra-hosted-family-data-custody'), true);

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    proofMode: 'app-install-purchase-child-device-delivery-runtime-writer-proof',
    commands,
    packageExportState: parentDomainPackageJson.exports[packageExportKey]
      ? 'validated-public-package-export'
      : 'blocked-by-parent-domain-package-json-lock',
    evidence: {
      childDeviceDeliveryRuntimeWriterContract:
        'packages/parent-domain/src/app-install-purchase-child-device-delivery-runtime-writer-proof.ts',
      sourceRuntimeWriterDeliveryContract:
        'packages/parent-domain/src/app-install-purchase-runtime-writer-delivery-proof.ts',
      sourcePackageSourceCaptureStatusContract:
        'packages/parent-domain/src/app-install-purchase-package-source-capture-status-proof.ts',
      contractTest:
        'packages/parent-domain/tests/app-install-purchase-child-device-delivery-runtime-writer-proof.test.ts',
      featureDoc: 'docs/features/app-install-purchase-approval.md',
      expectationDoc: 'docs/expectations/app-install-purchase-approval.md',
      checklistRow: 'docs/product-capability-checklist.md row Install/purchase approval',
      packageExport: 'PENDING: packages/parent-domain/package.json is locked by E-C; add public export when released.',
      output: relative(repoRoot, proofPath),
    },
    childDeviceDeliveryRuntimeWriterSummary: summary,
    childDeviceDeliveryRuntimeWriterRows: parsedReadModel.childDeviceDeliveryRuntimeWriterRows.map((row) => ({
      sourceDecisionAction: row.sourceDecisionAction,
      sourceRuntimeWriterDeliveryState: row.sourceRuntimeWriterDeliveryState,
      childDeliveryEnvelopeState: row.childDeliveryEnvelopeState,
      sourcePackageSourceCaptureRefs: row.sourcePackageSourceCaptureRefs,
      sourcePackageSourceCaptureStatuses: row.sourcePackageSourceCaptureStatuses,
      childDeliveryTargetRefs: row.childDeliveryTargetRefs,
      runtimeWriterAuditEventRefs: row.runtimeWriterAuditEventRefs,
      packageSourceAuditEventRefs: row.packageSourceAuditEventRefs,
      reportRuntimeRefs: row.reportRuntimeRefs,
      runtimeWriterExecutionClaim: row.runtimeWriterExecutionClaim,
      runtimeWriterDeliveryClaim: row.runtimeWriterDeliveryClaim,
      parentActionRuntimeDeliveryClaim: row.parentActionRuntimeDeliveryClaim,
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
  console.log(`app-install-purchase-child-device-delivery-runtime-writer-proof-ok:${relative(repoRoot, proofPath)}`);
}

async function loadChildDeviceDeliveryRuntimeWriterProofModule() {
  const modulePath = join(
    repoRoot,
    'packages',
    'parent-domain',
    'dist',
    'app-install-purchase-child-device-delivery-runtime-writer-proof.js'
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
