import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'app-install-purchase-windows-package-source-adapter-evidence-proof');
const proofPath = join(outputDir, 'proof.json');
const hostEvidencePath = join(outputDir, 'windows-host-evidence.json');
const checkedAt = '2026-06-07T02:30:00.000Z';
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']));
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/parent-domain',
      '--',
      'tests/app-install-purchase-windows-package-source-adapter-evidence.test.ts',
    ])
  );

  const hostEvidenceArtifact = await collectWindowsHostEvidence();
  await writeFile(hostEvidencePath, `${JSON.stringify(hostEvidenceArtifact, null, 2)}\n`);

  const proofModule = await loadProofModule();
  const packageProofModule =
    await import('@ocentra-parent/app-game-domain/app-install-purchase-windows-package-source-adapter-evidence');
  assert.equal(
    packageProofModule.AppInstallPurchaseWindowsPackageSourceAdapterEvidenceProofReadModel.schemaVersion,
    proofModule.AppInstallPurchaseWindowsPackageSourceAdapterEvidenceProofReadModel.schemaVersion
  );
  assert.equal(
    packageProofModule.AppInstallPurchaseWindowsPackageSourceRuntimeHandoffProofReadModel.schemaVersion,
    proofModule.AppInstallPurchaseWindowsPackageSourceRuntimeHandoffProofReadModel.schemaVersion
  );

  const parsedReadModel =
    proofModule.buildAppInstallPurchaseWindowsPackageSourceAdapterEvidenceProof(hostEvidenceArtifact);
  const summary = proofModule.summarizeAppInstallPurchaseWindowsPackageSourceAdapterEvidenceProof(parsedReadModel);
  const runtimeHandoffReadModel =
    proofModule.buildAppInstallPurchaseWindowsPackageSourceRuntimeHandoffProof(hostEvidenceArtifact);
  const runtimeHandoffSummary =
    proofModule.summarizeAppInstallPurchaseWindowsPackageSourceRuntimeHandoffProof(runtimeHandoffReadModel);

  assert.equal(summary.windowsPackageSourceAdapterEvidenceRows, 5);
  assert.equal(summary.manualAdapterEvidenceRows, 1);
  assert.equal(summary.platformUnavailableRows, 1);
  assert.equal(summary.blockedBeforeClaimRows, 2);
  assert.equal(summary.providerExecutedRows, 0);
  assert.equal(summary.childDeliveredRows, 0);
  assert.equal(
    summary.windowsHostEvidenceCollectedRows + summary.windowsHostManualRows,
    1,
    'Windows row must be exactly host-evidence-collected or manual-required'
  );
  assert.equal(runtimeHandoffSummary.runtimeHandoffRows, 5);
  assert.equal(runtimeHandoffSummary.manualRuntimeHandoffRows, 1);
  assert.equal(runtimeHandoffSummary.platformUnavailableRows, 1);
  assert.equal(runtimeHandoffSummary.blockedBeforeClaimRows, 2);
  assert.equal(runtimeHandoffSummary.providerExecutedRows, 0);
  assert.equal(runtimeHandoffSummary.childDeliveredRows, 0);
  assert.equal(
    runtimeHandoffSummary.windowsRuntimeHandoffReadyRows + runtimeHandoffSummary.windowsRuntimeHandoffManualRows,
    1,
    'Windows runtime handoff row must be exactly ready or manual-required'
  );

  const proof = {
    schemaVersion: 1,
    checkedAt,
    commitMetadataState: 'omitted-for-deterministic-proof-artifact',
    proofMode: 'app-install-purchase-windows-package-source-adapter-evidence',
    baseMainState: 'after-pr487-platform-adapter-evidence-gap-proof-merged',
    commands,
    packageExportState: 'validated-via-public-parent-domain-subpath-export',
    checklistState: 'updated-app-install-purchase-approval-row',
    evidence: {
      windowsPackageSourceAdapterEvidenceContract:
        'packages/parent-domain/src/app-install-purchase-windows-package-source-adapter-evidence.ts',
      windowsPackageSourceRuntimeHandoffContract:
        'packages/parent-domain/src/app-install-purchase-windows-package-source-adapter-evidence.ts',
      sourcePlatformAdapterEvidenceGapContract:
        'packages/parent-domain/src/app-install-purchase-platform-adapter-evidence-gap-proof.ts',
      sourcePackageSourceAdapterExecutionContract:
        'packages/parent-domain/src/app-install-purchase-package-source-adapter-execution-proof.ts',
      contractTest: 'packages/parent-domain/tests/app-install-purchase-windows-package-source-adapter-evidence.test.ts',
      hostEvidenceArtifact: relative(repoRoot, hostEvidencePath),
      featureDoc: 'docs/features/app-install-purchase-approval.md',
      expectationDoc: 'docs/expectations/app-install-purchase-approval.md',
      platformExpectationDoc: 'docs/expectations/platforms.md',
      checklistDoc: 'docs/product-capability-checklist.md',
      packageReadme: 'packages/parent-domain/README.md',
      packageExport: '@ocentra-parent/app-game-domain/app-install-purchase-windows-package-source-adapter-evidence',
      output: relative(repoRoot, proofPath),
    },
    windowsHostEvidenceSummary: {
      artifactRef: hostEvidenceArtifact.artifactRef,
      hostPlatform: hostEvidenceArtifact.hostPlatform,
      commandName: hostEvidenceArtifact.commandName,
      commandAvailable: hostEvidenceArtifact.commandAvailable,
      commandExitCode: hostEvidenceArtifact.commandExitCode,
      evidenceSummary: hostEvidenceArtifact.evidenceSummary,
    },
    windowsPackageSourceAdapterEvidenceSummary: summary,
    windowsPackageSourceAdapterEvidenceRows: parsedReadModel.windowsPackageSourceAdapterEvidenceRows,
    windowsPackageSourceRuntimeHandoffSummary: runtimeHandoffSummary,
    windowsPackageSourceRuntimeHandoffRows: runtimeHandoffReadModel.runtimeHandoffRows,
    nonClaims: parsedReadModel.nonClaims,
    knownGaps: [...parsedReadModel.knownGaps, ...runtimeHandoffReadModel.knownGaps],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`app-install-purchase-windows-package-source-adapter-evidence-proof-ok:${relative(repoRoot, proofPath)}`);
}

async function collectWindowsHostEvidence() {
  const collectedAt = checkedAt;
  if (process.platform !== 'win32') {
    return {
      artifactRef: 'windows-package-source-host-evidence-non-windows-host',
      hostPlatform: process.platform,
      commandName: 'Get-AppxPackage',
      commandAvailable: false,
      commandExitCode: 1,
      evidenceSummary: 'Proof harness did not run on Windows; Windows package-source evidence remains manual-required.',
      collectedAt,
    };
  }

  const output = await commandOutput('powershell', [
    '-NoProfile',
    '-ExecutionPolicy',
    'Bypass',
    '-Command',
    [
      '$command = Get-Command Get-AppxPackage -ErrorAction SilentlyContinue;',
      'if ($null -eq $command) { exit 11 }',
      '$store = Get-AppxPackage -Name Microsoft.WindowsStore -ErrorAction SilentlyContinue | Select-Object -First 1 Name, Publisher, SignatureKind;',
      "if ($null -eq $store) { Write-Output 'Get-AppxPackage available; Microsoft.WindowsStore package not present on this host.'; exit 0 }",
      '$store | ConvertTo-Json -Compress',
    ].join(' '),
  ]);
  const commandAvailable = output.exitCode === 0;
  return {
    artifactRef: commandAvailable
      ? 'windows-package-source-host-evidence-get-appxpackage'
      : 'windows-package-source-host-evidence-get-appxpackage-unavailable',
    hostPlatform: process.platform,
    commandName: 'Get-AppxPackage',
    commandAvailable,
    commandExitCode: output.exitCode,
    evidenceSummary: commandAvailable
      ? sanitizedEvidenceSummary(output.stdout)
      : 'Get-AppxPackage is unavailable or failed on this Windows host; Windows package-source evidence remains manual-required.',
    collectedAt,
  };
}

async function loadProofModule() {
  const modulePath = join(
    repoRoot,
    'packages',
    'parent-domain',
    'dist',
    'app-install-purchase-windows-package-source-adapter-evidence.js'
  );
  return import(pathToFileURL(modulePath).href);
}

async function commandOutput(command, args) {
  const chunks = [];
  const child = spawn(command, args, { cwd: repoRoot, stdio: ['ignore', 'pipe', 'pipe'] });
  child.stdout.on('data', (chunk) => chunks.push(chunk));
  child.stderr.on('data', (chunk) => chunks.push(chunk));
  const exitCode = await new Promise((resolve) => {
    child.on('close', resolve);
  });
  return { exitCode, stdout: Buffer.concat(chunks).toString('utf8') };
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

function sanitizedEvidenceSummary(output) {
  const trimmed = output.trim();
  if (trimmed.length === 0) {
    return 'Get-AppxPackage is available; no package details were emitted by the sanitized host probe.';
  }
  return `Get-AppxPackage available; sanitized Microsoft Store package-source probe returned ${trimmed}`;
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
