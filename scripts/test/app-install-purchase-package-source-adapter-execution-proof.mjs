import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'app-install-purchase-package-source-adapter-execution-proof');
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
    'tests/app-install-purchase-package-source-adapter-execution-proof.test.ts',
  ]);

  const proofModule = await loadPackageSourceAdapterExecutionProofModule();
  const packageModule =
    await import('@ocentra-parent/parent-domain/app-install-purchase-package-source-adapter-execution-proof');
  const parsedReadModel = proofModule.AppInstallPurchasePackageSourceAdapterExecutionProofReadModel;
  const summary = proofModule.summarizeAppInstallPurchasePackageSourceAdapterExecutionProof(parsedReadModel);
  const parentDomainPackageJson = JSON.parse(
    await readFile(join(repoRoot, 'packages', 'parent-domain', 'package.json'), 'utf8')
  );

  assert.deepEqual(summary, {
    packageSourceAdapterExecutionRows: 5,
    localAdapterExecutedRows: 1,
    manualHostProofRows: 1,
    blockedRows: 2,
    unavailableRows: 1,
    artifactLinkedRows: 5,
    providerExecutedRows: 0,
    childDeliveredRows: 0,
  });
  assert.deepEqual(
    packageModule.summarizeAppInstallPurchasePackageSourceAdapterExecutionProof(
      packageModule.AppInstallPurchasePackageSourceAdapterExecutionProofReadModel
    ),
    summary
  );
  assert.deepEqual(parentDomainPackageJson.exports['./app-install-purchase-package-source-adapter-execution-proof'], {
    import: './dist/app-install-purchase-package-source-adapter-execution-proof.js',
    types: './dist/app-install-purchase-package-source-adapter-execution-proof.d.ts',
  });
  assert.deepEqual(
    parsedReadModel.packageSourceAdapterExecutionRows.map(
      (row) =>
        `${row.platform}:${row.storeSurface}:${row.adapterKind}:${row.adapterExecutionState}:${row.sourcePackageSourceCaptureStatus}`
    ),
    [
      'windows:microsoft-store:windows-local-package-source-reader:local-adapter-executed:captured',
      'macos:mac-app-store:macos-manual-host-proof:manual-host-proof-required:manual-required',
      'linux:linux-package-manager:linux-package-manager-unavailable:platform-unavailable:unavailable',
      'android:google-play:android-device-owner-required:device-management-required:blocked',
      'ios:apple-app-store:ios-family-controls-entitlement-required:apple-entitlement-required:blocked',
    ]
  );
  assert.equal(parsedReadModel.nonClaims.includes('no-provider-api-execution'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-production-platform-adapter'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-child-device-delivery'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-ocentra-hosted-family-data-custody'), true);

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    proofMode: 'app-install-purchase-package-source-adapter-execution-proof',
    commands,
    packageExportState: 'validated-public-package-export',
    evidence: {
      packageSourceAdapterExecutionContract:
        'packages/parent-domain/src/app-install-purchase-package-source-adapter-execution-proof.ts',
      sourcePackageSourceCaptureStatusContract:
        'packages/parent-domain/src/app-install-purchase-package-source-capture-status-proof.ts',
      contractTest: 'packages/parent-domain/tests/app-install-purchase-package-source-adapter-execution-proof.test.ts',
      featureDoc: 'docs/features/app-install-purchase-approval.md',
      expectationDoc: 'docs/expectations/app-install-purchase-approval.md',
      checklistDelta:
        'PENDING: docs/product-capability-checklist.md Install/purchase approval row needs package-source adapter execution proof once checklist lock is available.',
      packageExport:
        'COMPLETED: packages/parent-domain/package.json exports ./app-install-purchase-package-source-adapter-execution-proof.',
      output: relative(repoRoot, proofPath),
    },
    packageSourceAdapterExecutionSummary: summary,
    packageSourceAdapterExecutionRows: parsedReadModel.packageSourceAdapterExecutionRows.map((row) => ({
      platform: row.platform,
      storeSurface: row.storeSurface,
      sourcePackageSourceCaptureRowId: row.sourcePackageSourceCaptureRowId,
      sourcePackageSourceCaptureStatus: row.sourcePackageSourceCaptureStatus,
      sourcePackageSourceCaptureArtifactRefs: row.sourcePackageSourceCaptureArtifactRefs,
      sourcePackageSourceAuditRefs: row.sourcePackageSourceAuditRefs,
      adapterKind: row.adapterKind,
      adapterExecutionState: row.adapterExecutionState,
      adapterExecutionAttemptRefs: row.adapterExecutionAttemptRefs,
      adapterExecutionArtifactRefs: row.adapterExecutionArtifactRefs,
      auditEventRefs: row.auditEventRefs,
      reportRefs: row.reportRefs,
      requiredProofRefs: row.requiredProofRefs,
      packageSourceAdapterExecutionClaim: row.packageSourceAdapterExecutionClaim,
      providerApiExecutionClaim: row.providerApiExecutionClaim,
      storeIntegrationClaim: row.storeIntegrationClaim,
      portalApprovalUiClaim: row.portalApprovalUiClaim,
      productionPlatformAdapterClaim: row.productionPlatformAdapterClaim,
      childDeliveryClaim: row.childDeliveryClaim,
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
  console.log(`app-install-purchase-package-source-adapter-execution-proof-ok:${relative(repoRoot, proofPath)}`);
}

async function loadPackageSourceAdapterExecutionProofModule() {
  const modulePath = join(
    repoRoot,
    'packages',
    'parent-domain',
    'dist',
    'app-install-purchase-package-source-adapter-execution-proof.js'
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
