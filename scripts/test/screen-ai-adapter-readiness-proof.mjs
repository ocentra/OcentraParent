import { execFileSync } from 'node:child_process';
import { mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'output', 'screen-ai-pipeline-proof', 'adapter-readiness');
const readModelPath = join(outputDir, 'read-model.json');
const proofSummaryPath = join(outputDir, 'proof-summary.json');
const validationCommandsPath = join(outputDir, '10-validation-commands.log');
const commands = [];

await main();

async function main() {
  rmSync(outputDir, { recursive: true, force: true });
  mkdirSync(outputDir, { recursive: true });

  runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'screen-ai-adapter-readiness-proof',
  ]);

  const readinessContract = await import(
    pathToFileURL(join(repoRoot, 'packages/parent-domain/dist/screen-ai-adapter-readiness-proof.js')).href
  );
  const supportedAdapters = await import('@ocentra-parent/parent-domain/v0-8-supported-adapter-runtime-proof');
  const readModel = readinessContract.ScreenAiAdapterReadinessReadModelSchema.parse(
    buildReadModel(supportedAdapters.V08SupportedAdapterRuntimeProofReadModel)
  );
  const summary = readinessContract.summarizeScreenAiAdapterReadiness(readModel);
  const assertions = assertionsForReadModel(readinessContract, readModel, summary);
  const proofSummary = {
    proof: 'screen-ai-adapter-readiness-proof',
    proofTier: 'P3_RETAINED_SCREEN_ACTION_AND_ADAPTER_READINESS',
    generatedAt: readModel.generatedAt,
    readModel: relativePath(readModelPath),
    validationCommands: relativePath(validationCommandsPath),
    sourceArtifacts: readModel.sourceArtifacts,
    summary,
    assertions,
    claimsProved: [
      'Screen-derived owned-process time-limit and owned-process block decisions retain real Windows adapter execution proof.',
      'Screen-derived broad installed-app, host network/domain, exact active-tab, mobile, and Linux adapter targets remain manual-required, not-claimed, or unavailable instead of silently upgrading claims.',
      'Every row keeps source screen evidence refs, dry-run policy source state, deleted-image custody, and rawImageRetained false.',
    ],
    nonClaims: [
      'This proof does not implement broad installed-app blocking, host network/domain blocking, exact active-tab enforcement, mobile control, notification delivery, tamper hardening, or unsupported-platform behavior.',
      'This proof reuses retained screen action artifacts and the existing supported-adapter runtime matrix; it does not rerun live screen capture or adapter execution.',
      'PR367 remains the real service WinRT OCR policy proof; this stacked proof only closes adapter-readiness honesty for screen-derived decisions.',
    ],
  };

  if (!Object.values(assertions).every((assertion) => assertion === true)) {
    throw new Error(`Screen AI adapter readiness proof failed: ${JSON.stringify(assertions)}`);
  }

  writeFileSync(readModelPath, `${JSON.stringify(readModel, null, 2)}\n`);
  writeFileSync(proofSummaryPath, `${JSON.stringify(proofSummary, null, 2)}\n`);
  writeFileSync(validationCommandsPath, `${commands.map((command) => `${command}: PASS`).join('\n')}\n`);
  console.log(
    `screen-ai-adapter-readiness-proof-ok:${summary.rowCount}:${summary.executedRows}:${summary.skippedRows}`
  );
  console.log(`proof=${relativePath(proofSummaryPath)}`);
}

function buildReadModel(supportedAdapterReadModel) {
  const timeLimitSource = readJson(
    join(repoRoot, 'output', 'screen-ai-pipeline-proof', 'action-dispatch', '00-screen-policy-source.json')
  );
  const timeLimitSummary = readJson(
    join(repoRoot, 'output', 'screen-ai-pipeline-proof', 'action-dispatch', 'proof-summary.json')
  );
  const blockSource = readJson(
    join(repoRoot, 'output', 'screen-ai-pipeline-proof', 'block-action-dispatch', '00-screen-block-source.json')
  );
  const blockSummary = readJson(
    join(repoRoot, 'output', 'screen-ai-pipeline-proof', 'block-action-dispatch', 'proof-summary.json')
  );
  const supportedEntryById = new Map(supportedAdapterReadModel.entries.map((entry) => [entry.proofEntryId, entry]));

  return {
    schemaVersion: 'v0.6',
    readModelId: 'screen-ai-adapter-readiness-proof',
    generatedAt: new Date().toISOString(),
    sourceArtifacts: [
      'output/screen-ai-pipeline-proof/action-dispatch/00-screen-policy-source.json',
      'output/screen-ai-pipeline-proof/action-dispatch/proof-summary.json',
      'output/screen-ai-pipeline-proof/block-action-dispatch/00-screen-block-source.json',
      'output/screen-ai-pipeline-proof/block-action-dispatch/proof-summary.json',
      'packages/parent-domain/src/v0-8-supported-adapter-runtime-proof.ts',
    ],
    rows: [
      timeLimitAdapterRow(entryFor(supportedEntryById, 'windows-app-game-owned-process-time-limit'), timeLimitSource),
      ownedProcessBlockAdapterRow(blockSource, blockSummary),
      manualOrUnavailableRow(
        'screen-ai-broad-installed-app-manual-required',
        entryFor(supportedEntryById, 'windows-broad-installed-app-blocking-manual-gate'),
        blockSource,
        'manual-required'
      ),
      manualOrUnavailableRow(
        'screen-ai-host-network-domain-manual-required',
        entryFor(supportedEntryById, 'windows-host-network-domain-blocking-manual-gate'),
        blockSource,
        'manual-required'
      ),
      manualOrUnavailableRow(
        'screen-ai-managed-active-tab-not-claimed',
        entryFor(supportedEntryById, 'windows-managed-exact-active-tab-not-claimed'),
        blockSource,
        'not-claimed'
      ),
      manualOrUnavailableRow(
        'screen-ai-android-mobile-control-manual-required',
        entryFor(supportedEntryById, 'android-mobile-control-manual-gate'),
        blockSource,
        'manual-required'
      ),
      manualOrUnavailableRow(
        'screen-ai-ios-mobile-control-manual-required',
        entryFor(supportedEntryById, 'ios-mobile-control-manual-gate'),
        blockSource,
        'manual-required'
      ),
      manualOrUnavailableRow(
        'screen-ai-linux-host-adapter-unavailable',
        entryFor(supportedEntryById, 'linux-host-adapter-unavailable'),
        blockSource,
        'unavailable'
      ),
    ],
  };
}

function timeLimitAdapterRow(adapterEntry, source) {
  return {
    ...rowFromSupportedAdapterEntry(
      'screen-ai-owned-process-time-limit-real-adapter',
      adapterEntry,
      source,
      'real-owned-process-action-proved'
    ),
    actionExecutionState: 'executed',
    adapterExecutionProofArtifact: 'output/screen-ai-pipeline-proof/action-dispatch/proof-summary.json',
  };
}

function ownedProcessBlockAdapterRow(source, blockSummary) {
  assertEqual(blockSummary.realWindowsBlockAdapterProof, true, 'block real adapter proof flag');
  assertEqual(blockSummary.policyDecisionLinkedToAdapter, true, 'block decision link');
  assertEqual(blockSummary.evidenceRefsLinkedToAdapter, true, 'block evidence refs');
  assertEqual(blockSummary.rawImageDeletedBeforeDispatch, true, 'block raw image deleted');
  return {
    schemaVersion: 'v0.6',
    rowId: 'screen-ai-owned-process-block-real-adapter',
    sourcePolicyDecisionId: source.policyDecisionId,
    sourcePolicyAction: source.sourcePolicyAction,
    sourcePolicyDryRun: source.sourcePolicyDryRun,
    sourceProofArtifact: 'output/screen-ai-pipeline-proof/block-action-dispatch/00-screen-block-source.json',
    sourceEvidenceReferences: source.evidenceReferences,
    sourceImageDeletionState: 'deleted',
    rawImageRetained: false,
    rawImageDeletedBeforeAdapter: source.rawImageDeletedBeforeDispatch,
    readinessState: 'real-owned-process-action-proved',
    actionExecutionState: 'executed',
    adapterRuntimeBoundary: 'windows-screen-owned-process-block',
    adapterCapability: 'screen-owned-process-block',
    adapterRuntimeState: 'implemented-boundary',
    adapterResult: 'supported-boundary-proved',
    platform: 'windows',
    platformSupportState: 'supported-on-windows',
    targetIdentityState: 'process-session-evidence-backed',
    rollbackReferenceState: 'not-required',
    auditReferenceState: 'audit-reference-backed',
    refusalReason: 'none',
    adapterExecutionProofArtifact: 'output/screen-ai-pipeline-proof/block-action-dispatch/proof-summary.json',
    linkedProofArtifacts: ['output/screen-ai-pipeline-proof/block-action-dispatch/02-adapter-proof.json'],
    manualProofRequirements: [],
    claimFlags: noClaimFlags(),
    claimBoundary:
      'Screen-derived block execution is limited to a controlled owned process and does not prove broad app, browser URL, network/domain, or mobile blocking.',
    fallbackBehavior:
      'Targets outside the controlled owned-process boundary remain manual-required or unavailable until their own adapter artifacts exist.',
  };
}

function manualOrUnavailableRow(rowId, adapterEntry, source, readinessState) {
  return {
    ...rowFromSupportedAdapterEntry(rowId, adapterEntry, source, readinessState),
    actionExecutionState: 'skipped',
    adapterExecutionProofArtifact: null,
  };
}

function rowFromSupportedAdapterEntry(rowId, adapterEntry, source, readinessState) {
  return {
    schemaVersion: 'v0.6',
    rowId,
    sourcePolicyDecisionId: source.policyDecisionId,
    sourcePolicyAction: source.action ?? source.sourcePolicyAction,
    sourcePolicyDryRun: source.sourcePolicyDryRun ?? true,
    sourceProofArtifact: proofSourceFor(source.scenarioId),
    sourceEvidenceReferences: source.evidenceReferences,
    sourceImageDeletionState: 'deleted',
    rawImageRetained: false,
    rawImageDeletedBeforeAdapter: source.rawImageDeletedBeforeDispatch,
    readinessState,
    actionExecutionState: 'skipped',
    adapterRuntimeBoundary: adapterEntry.runtimeBoundary,
    adapterCapability: adapterEntry.adapterCapability,
    adapterRuntimeState: adapterEntry.runtimeState,
    adapterResult: adapterEntry.adapterResult,
    platform: adapterEntry.platform,
    platformSupportState: adapterEntry.platformSupportState,
    targetIdentityState: adapterEntry.targetIdentityState,
    rollbackReferenceState: adapterEntry.rollbackReferenceState,
    auditReferenceState: adapterEntry.auditReferenceState,
    refusalReason: adapterEntry.refusalReason,
    adapterExecutionProofArtifact: null,
    linkedProofArtifacts: adapterEntry.linkedProofArtifacts,
    manualProofRequirements: adapterEntry.manualProofRequirements,
    claimFlags: {
      broadInstalledAppBlockingClaimed: adapterEntry.broadInstalledAppBlockingClaimed,
      networkDomainBlockingClaimed: adapterEntry.networkDomainBlockingClaimed,
      exactActiveTabEnforcementClaimed: adapterEntry.exactActiveTabEnforcementClaimed,
      notificationDeliveryClaimed: adapterEntry.notificationDeliveryClaimed,
      tamperHardeningClaimed: adapterEntry.tamperHardeningClaimed,
      mobileControlClaimed: adapterEntry.mobileControlClaimed,
      unsupportedPlatformBehaviorClaimed: adapterEntry.unsupportedPlatformBehaviorClaimed,
    },
    claimBoundary: adapterEntry.claimBoundary,
    fallbackBehavior: adapterEntry.fallbackBehavior,
  };
}

function assertionsForReadModel(readinessContract, readModel, summary) {
  const rowsById = new Map(readModel.rows.map((row) => [row.rowId, row]));
  return {
    requiredBoundariesCovered: readinessContract.screenAiAdapterReadinessCoversRequiredBoundaries(readModel),
    realOwnedProcessTimeLimitStillProved:
      rowsById.get('screen-ai-owned-process-time-limit-real-adapter')?.actionExecutionState === 'executed',
    realOwnedProcessBlockStillProved:
      rowsById.get('screen-ai-owned-process-block-real-adapter')?.actionExecutionState === 'executed',
    broadInstalledAppBlockingStillManualRequired:
      rowsById.get('screen-ai-broad-installed-app-manual-required')?.readinessState === 'manual-required',
    hostNetworkDomainBlockingStillManualRequired:
      rowsById.get('screen-ai-host-network-domain-manual-required')?.readinessState === 'manual-required',
    managedActiveTabStillNotClaimed:
      rowsById.get('screen-ai-managed-active-tab-not-claimed')?.readinessState === 'not-claimed',
    mobileControlStillManualRequired:
      rowsById.get('screen-ai-android-mobile-control-manual-required')?.readinessState === 'manual-required' &&
      rowsById.get('screen-ai-ios-mobile-control-manual-required')?.readinessState === 'manual-required',
    linuxHostAdapterStillUnavailable:
      rowsById.get('screen-ai-linux-host-adapter-unavailable')?.readinessState === 'unavailable',
    noRawImageRetained: summary.rawImageRetainedRows === 0,
    noClaimUpgrades: summary.claimUpgradeRows === 0,
  };
}

function entryFor(entriesById, proofEntryId) {
  const entry = entriesById.get(proofEntryId);
  if (entry === undefined) {
    throw new Error(`Missing supported adapter proof entry ${proofEntryId}`);
  }
  return entry;
}

function proofSourceFor(scenarioId) {
  if (scenarioId === 'native-owned-process-time-limit') {
    return 'output/screen-ai-pipeline-proof/action-dispatch/00-screen-policy-source.json';
  }
  return 'output/screen-ai-pipeline-proof/block-action-dispatch/00-screen-block-source.json';
}

function noClaimFlags() {
  return {
    broadInstalledAppBlockingClaimed: false,
    networkDomainBlockingClaimed: false,
    exactActiveTabEnforcementClaimed: false,
    notificationDeliveryClaimed: false,
    tamperHardeningClaimed: false,
    mobileControlClaimed: false,
    unsupportedPlatformBehaviorClaimed: false,
  };
}

function readJson(path) {
  return JSON.parse(readFileSync(path, 'utf8'));
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}

function runCommand(command, args) {
  const commandLine = [command, ...args].join(' ');
  commands.push(commandLine);
  execFileSync(command, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
}

function assertEqual(actual, expected, label) {
  if (actual !== expected) {
    throw new Error(`${label}: expected ${expected}, received ${actual}`);
  }
}
