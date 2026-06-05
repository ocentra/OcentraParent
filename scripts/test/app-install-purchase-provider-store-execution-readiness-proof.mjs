import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'app-install-purchase-provider-store-execution-readiness-proof');
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
    'tests/app-install-purchase-provider-store-execution-readiness-proof.test.ts',
  ]);

  const proofModule = await loadProviderStoreExecutionReadinessProofModule();
  const packageModule =
    await import('@ocentra-parent/parent-domain/app-install-purchase-provider-store-execution-readiness-proof');
  const parentDomainPackageJson = await loadParentDomainPackageJson();
  const parsedReadModel = proofModule.AppInstallPurchaseProviderStoreExecutionReadinessProofReadModel;
  const summary = proofModule.summarizeAppInstallPurchaseProviderStoreExecutionReadinessProof(parsedReadModel);
  const packageExportKey = './app-install-purchase-provider-store-execution-readiness-proof';

  assert.equal(packageModule.AppInstallPurchaseProviderStoreExecutionReadinessProofReadModel, parsedReadModel);
  assert.deepEqual(parentDomainPackageJson.exports[packageExportKey], {
    import: './dist/app-install-purchase-provider-store-execution-readiness-proof.js',
    types: './dist/app-install-purchase-provider-store-execution-readiness-proof.d.ts',
  });
  assert.deepEqual(summary, {
    providerStoreExecutionReadinessRows: 5,
    executionReadyRows: 1,
    manualRequiredRows: 3,
    unavailableRows: 1,
    packageSourceAdapterLinkedRows: 5,
    parentActionReadinessLinkedRows: 5,
    providerExecutedRows: 0,
    childDeliveredRows: 0,
  });
  assert.deepEqual(
    parsedReadModel.providerStoreExecutionReadinessRows.map(
      (row) =>
        `${row.platform}:${row.storeSurface}:${row.sourceApiEntitlementEvidenceStatus}:${row.sourceStoreStatusHandoffState}:${row.sourcePackageSourceAdapterExecutionState}:${row.providerStoreExecutionReadinessState}`
    ),
    [
      'windows:microsoft-store:approved-api-evidence-required:approved-api-status-proof-required:local-adapter-executed:provider-store-execution-ready',
      'macos:mac-app-store:manual-platform-review-required:manual-platform-status-review-required:manual-host-proof-required:manual-required',
      'linux:linux-package-manager:platform-unavailable:platform-store-status-unavailable:platform-unavailable:unavailable',
      'android:google-play:store-entitlement-evidence-required:store-entitlement-status-proof-required:device-management-required:manual-required',
      'ios:apple-app-store:store-entitlement-evidence-required:store-entitlement-status-proof-required:apple-entitlement-required:manual-required',
    ]
  );
  assert.equal(parsedReadModel.nonClaims.includes('no-google-play-execution'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-apple-app-store-execution'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-microsoft-store-execution'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-billing-provider-contact'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-provider-api-execution'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-child-device-delivery'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-ocentra-hosted-family-data-custody'), true);

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    proofMode: 'app-install-purchase-provider-store-execution-readiness-proof',
    commands,
    packageExportState: 'validated-public-package-export',
    checklistState:
      'PENDING_LOCK: docs/product-capability-checklist.md is intentionally untouched while locked by another lane.',
    evidence: {
      providerStoreExecutionReadinessContract:
        'packages/parent-domain/src/app-install-purchase-provider-store-execution-readiness-proof.ts',
      sourceApprovedApiEntitlementContract:
        'packages/parent-domain/src/app-install-purchase-approved-api-entitlement-proof.ts',
      sourceStoreStatusHandoffContract: 'packages/parent-domain/src/app-install-purchase-store-status-handoff-proof.ts',
      sourcePackageSourceAdapterExecutionContract:
        'packages/parent-domain/src/app-install-purchase-package-source-adapter-execution-proof.ts',
      sourceParentActionDeliveryReadinessContract:
        'packages/parent-domain/src/app-install-purchase-parent-action-delivery-readiness-proof.ts',
      contractTest:
        'packages/parent-domain/tests/app-install-purchase-provider-store-execution-readiness-proof.test.ts',
      featureDoc: 'docs/features/app-install-purchase-approval.md',
      expectationDoc: 'docs/expectations/app-install-purchase-approval.md',
      packageExport:
        'COMPLETED: packages/parent-domain/package.json exports ./app-install-purchase-provider-store-execution-readiness-proof.',
      checklistRow:
        'PENDING: update docs/product-capability-checklist.md Install/purchase approval row after the checklist lock clears.',
      output: relative(repoRoot, proofPath),
    },
    providerStoreExecutionReadinessSummary: summary,
    providerStoreExecutionReadinessRows: parsedReadModel.providerStoreExecutionReadinessRows.map((row) => ({
      platform: row.platform,
      storeSurface: row.storeSurface,
      sourceApprovedApiEntitlementRowId: row.sourceApprovedApiEntitlementRowId,
      sourceStoreStatusHandoffRowId: row.sourceStoreStatusHandoffRowId,
      sourcePackageSourceAdapterExecutionRowId: row.sourcePackageSourceAdapterExecutionRowId,
      sourceParentActionDeliveryReadinessRefs: row.sourceParentActionDeliveryReadinessRefs,
      sourceParentActionDeliveryReadinessStates: row.sourceParentActionDeliveryReadinessStates,
      sourceApiEntitlementEvidenceStatus: row.sourceApiEntitlementEvidenceStatus,
      sourceStoreStatusHandoffState: row.sourceStoreStatusHandoffState,
      sourcePackageSourceAdapterExecutionState: row.sourcePackageSourceAdapterExecutionState,
      providerStoreExecutionReadinessState: row.providerStoreExecutionReadinessState,
      approvedApiEvidenceRefs: row.approvedApiEvidenceRefs,
      entitlementEvidenceRefs: row.entitlementEvidenceRefs,
      storeStatusHandoffEvidenceRefs: row.storeStatusHandoffEvidenceRefs,
      packageSourceAdapterArtifactRefs: row.packageSourceAdapterArtifactRefs,
      parentActionAuditEventRefs: row.parentActionAuditEventRefs,
      reportRuntimeRefs: row.reportRuntimeRefs,
      requiredProofRefs: row.requiredProofRefs,
      googlePlayExecutionClaim: row.googlePlayExecutionClaim,
      appleAppStoreExecutionClaim: row.appleAppStoreExecutionClaim,
      microsoftStoreExecutionClaim: row.microsoftStoreExecutionClaim,
      billingProviderContactClaim: row.billingProviderContactClaim,
      providerApiExecutionClaim: row.providerApiExecutionClaim,
      storeIntegrationClaim: row.storeIntegrationClaim,
      platformInterceptionClaim: row.platformInterceptionClaim,
      platformAdapterClaim: row.platformAdapterClaim,
      childDeviceDeliveryClaim: row.childDeviceDeliveryClaim,
      runtimeWriterDeliveryClaim: row.runtimeWriterDeliveryClaim,
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
  console.log(`app-install-purchase-provider-store-execution-readiness-proof-ok:${relative(repoRoot, proofPath)}`);
}

async function loadProviderStoreExecutionReadinessProofModule() {
  const modulePath = join(
    repoRoot,
    'packages',
    'parent-domain',
    'dist',
    'app-install-purchase-provider-store-execution-readiness-proof.js'
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
