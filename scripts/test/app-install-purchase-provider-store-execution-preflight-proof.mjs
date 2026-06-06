import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'app-install-purchase-provider-store-execution-preflight-proof');
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
    'tests/app-install-purchase-provider-store-execution-preflight-proof.test.ts',
  ]);

  const proofModule = await loadProviderStoreExecutionPreflightProofModule();
  const parsedReadModel = proofModule.AppInstallPurchaseProviderStoreExecutionPreflightProofReadModel;
  const summary = proofModule.summarizeAppInstallPurchaseProviderStoreExecutionPreflightProof(parsedReadModel);

  assert.deepEqual(summary, {
    providerStoreExecutionPreflightRows: 5,
    preflightReadyRows: 1,
    manualProviderProofRequiredRows: 3,
    providerUnavailableRows: 1,
    providerExecutedRows: 0,
    runtimeDeviceDeliveredRows: 0,
  });
  assert.deepEqual(
    parsedReadModel.providerStoreExecutionPreflightRows.map(
      (row) =>
        `${row.platform}:${row.storeSurface}:${row.sourceProviderStoreExecutionReadinessState}:${row.providerStoreExecutionPreflightState}`
    ),
    [
      'windows:microsoft-store:provider-store-execution-ready:preflight-ready',
      'macos:mac-app-store:manual-required:manual-provider-proof-required',
      'linux:linux-package-manager:unavailable:provider-unavailable',
      'android:google-play:manual-required:manual-provider-proof-required',
      'ios:apple-app-store:manual-required:manual-provider-proof-required',
    ]
  );
  assert.equal(parsedReadModel.nonClaims.includes('no-provider-api-execution'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-runtime-device-delivery'), true);
  assert.equal(parsedReadModel.nonClaims.includes('no-ocentra-hosted-family-data-custody'), true);

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    proofMode: 'app-install-purchase-provider-store-execution-preflight-proof',
    commands,
    packageExportState: 'not-claimed-new-public-export-deferred',
    checklistState: 'not-touched-doc-overlap-pr430-pr433',
    evidence: {
      providerStoreExecutionPreflightContract:
        'packages/parent-domain/src/app-install-purchase-provider-store-execution-preflight-proof.ts',
      sourceProviderStoreExecutionReadinessContract:
        'packages/parent-domain/src/app-install-purchase-provider-store-execution-readiness-proof.ts',
      sourceRuntimeWriterExecutionDeliveryContract:
        'packages/parent-domain/src/app-install-purchase-runtime-writer-execution-delivery-proof.ts',
      contractTest:
        'packages/parent-domain/tests/app-install-purchase-provider-store-execution-preflight-proof.test.ts',
      docsDeferred:
        'docs/features/app-install-purchase-approval.md and docs/expectations/app-install-purchase-approval.md are intentionally untouched while PR430 and PR433 own overlapping app-install docs.',
      output: relative(repoRoot, proofPath),
    },
    providerStoreExecutionPreflightSummary: summary,
    providerStoreExecutionPreflightRows: parsedReadModel.providerStoreExecutionPreflightRows.map((row) => ({
      platform: row.platform,
      storeSurface: row.storeSurface,
      sourceProviderStoreExecutionReadinessRowId: row.sourceProviderStoreExecutionReadinessRowId,
      sourceRuntimeWriterExecutionDeliveryRowIds: row.sourceRuntimeWriterExecutionDeliveryRowIds,
      sourceProviderStoreExecutionReadinessState: row.sourceProviderStoreExecutionReadinessState,
      sourceRuntimeWriterReceiptClaims: row.sourceRuntimeWriterReceiptClaims,
      providerStoreExecutionPreflightState: row.providerStoreExecutionPreflightState,
      requiredProviderEvidenceRefs: row.requiredProviderEvidenceRefs,
      runtimeWriterReceiptRefs: row.runtimeWriterReceiptRefs,
      auditEventRefs: row.auditEventRefs,
      reportRuntimeRefs: row.reportRuntimeRefs,
      googlePlayExecutionClaim: row.googlePlayExecutionClaim,
      appleAppStoreExecutionClaim: row.appleAppStoreExecutionClaim,
      microsoftStoreExecutionClaim: row.microsoftStoreExecutionClaim,
      billingProviderContactClaim: row.billingProviderContactClaim,
      providerApiExecutionClaim: row.providerApiExecutionClaim,
      storeIntegrationClaim: row.storeIntegrationClaim,
      platformInterceptionClaim: row.platformInterceptionClaim,
      platformAdapterClaim: row.platformAdapterClaim,
      runtimeDeviceDeliveryClaim: row.runtimeDeviceDeliveryClaim,
      childDeviceDeliveryClaim: row.childDeviceDeliveryClaim,
      appBlockingClaim: row.appBlockingClaim,
      childDataCustody: row.childDataCustody,
      ocentraHostedFamilyDataCustodyClaim: row.ocentraHostedFamilyDataCustodyClaim,
      claimBoundary: row.claimBoundary,
    })),
    nonClaims: parsedReadModel.nonClaims,
    knownGaps: parsedReadModel.knownGaps,
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`app-install-purchase-provider-store-execution-preflight-proof-ok:${relative(repoRoot, proofPath)}`);
}

async function loadProviderStoreExecutionPreflightProofModule() {
  const modulePath = join(
    repoRoot,
    'packages',
    'parent-domain',
    'dist',
    'app-install-purchase-provider-store-execution-preflight-proof.js'
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
