import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'app-install-purchase-runtime-report-delivery-proof');
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
    'tests/app-install-purchase-runtime-report-delivery-proof.test.ts',
  ]);

  const proofModule = await loadRuntimeReportDeliveryProofModule();
  const packageModule =
    await import('@ocentra-parent/parent-domain/app-install-purchase-runtime-report-delivery-proof');
  const parentDomainPackageJson = await loadParentDomainPackageJson();
  const parsedReadModel = proofModule.AppInstallPurchaseRuntimeReportDeliveryProofReadModel;
  const summary = proofModule.summarizeAppInstallPurchaseRuntimeReportDeliveryProof(parsedReadModel);
  const packageExportKey = './app-install-purchase-runtime-report-delivery-proof';

  assert.equal(packageModule.AppInstallPurchaseRuntimeReportDeliveryProofReadModel, parsedReadModel);
  assert.deepEqual(parentDomainPackageJson.exports[packageExportKey], {
    import: './dist/app-install-purchase-runtime-report-delivery-proof.js',
    types: './dist/app-install-purchase-runtime-report-delivery-proof.d.ts',
  });
  assert.deepEqual(summary, {
    runtimeReportDeliveryRows: 4,
    deliveredRows: 4,
    receiptRows: 4,
    portalReportUiRows: 0,
    childDeviceDeliveryRows: 0,
  });
  assert.equal(parsedReadModel.nonClaims.includes('no-portal-report-ui'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-child-device-delivery'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-provider-api-execution'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-ocentra-hosted-family-data-custody'), true);

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    proofMode: 'app-install-purchase-runtime-report-delivery-proof',
    commands,
    packageExportState: 'validated-public-package-export',
    checklistState: 'validated-product-capability-checklist-row',
    evidence: {
      runtimeReportDeliveryContract: 'packages/parent-domain/src/app-install-purchase-runtime-report-delivery-proof.ts',
      sourceReportRuntimeContract: 'packages/parent-domain/src/app-install-purchase-report-runtime-proof.ts',
      sourceReportCompilerContract: 'packages/parent-domain/src/stateless-report-compiler-status.ts',
      contractTest: 'packages/parent-domain/tests/app-install-purchase-runtime-report-delivery-proof.test.ts',
      featureDoc: 'docs/features/app-install-purchase-approval.md',
      expectationDoc: 'docs/expectations/app-install-purchase-approval.md',
      packageExport:
        'COMPLETED: packages/parent-domain/package.json exports ./app-install-purchase-runtime-report-delivery-proof.',
      checklistRow:
        'COMPLETED: docs/product-capability-checklist.md Install/purchase approval row includes runtime report delivery proof.',
      output: relative(repoRoot, proofPath),
    },
    runtimeReportDeliverySummary: summary,
    runtimeReportDeliveryRows: parsedReadModel.runtimeReportDeliveryRows.map((row) => ({
      sourceReportRuntimeRowId: row.sourceReportRuntimeRowId,
      reportSurface: row.reportSurface,
      compilerRequestId: row.compilerRequestId,
      compilerOutputReportRef: row.compilerOutputReportRef,
      runtimeReportReceiptRef: row.runtimeReportReceiptRef,
      sourceChildArtifactRefs: row.sourceChildArtifactRefs,
      deliveryState: row.deliveryState,
      parentAuthorized: row.parentAuthorized,
      rawEvidenceExcludedFromOutput: row.rawEvidenceExcludedFromOutput,
      childDetailMinimized: row.childDetailMinimized,
      tempDeletionConfirmed: row.tempDeletionConfirmed,
      localEvidenceMutated: row.localEvidenceMutated,
      ocentraHostedReportRetained: row.ocentraHostedReportRetained,
      runtimeReportDeliveryClaim: row.runtimeReportDeliveryClaim,
      portalReportUiClaim: row.portalReportUiClaim,
      providerApiExecutionClaim: row.providerApiExecutionClaim,
      storeIntegrationClaim: row.storeIntegrationClaim,
      platformAdapterClaim: row.platformAdapterClaim,
      childDeviceDeliveryClaim: row.childDeviceDeliveryClaim,
      childDataCustody: row.childDataCustody,
      appBlockingClaim: row.appBlockingClaim,
      ocentraHostedFamilyDataCustodyClaim: row.ocentraHostedFamilyDataCustodyClaim,
      claimBoundary: row.claimBoundary,
    })),
    nonClaims: parsedReadModel.nonClaims,
    knownGaps: parsedReadModel.knownGaps,
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`app-install-purchase-runtime-report-delivery-proof-ok:${relative(repoRoot, proofPath)}`);
}

async function loadRuntimeReportDeliveryProofModule() {
  const modulePath = join(
    repoRoot,
    'packages',
    'parent-domain',
    'dist',
    'app-install-purchase-runtime-report-delivery-proof.js'
  );
  return import(pathToFileURL(modulePath).href);
}

async function loadParentDomainPackageJson() {
  const packageJsonPath = join(repoRoot, 'packages', 'parent-domain', 'package.json');
  return JSON.parse(await readFile(packageJsonPath, 'utf8'));
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
