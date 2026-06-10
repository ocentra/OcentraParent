import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';

const repoRoot = process.cwd();
const outputDir = resolve(repoRoot, 'output', 'screen-ai-pipeline-proof', 'upstream-adapter-prerequisite-bridge');
const proofPath = join(outputDir, 'proof-summary.json');
const bridgePath = join(outputDir, 'upstream-adapter-prerequisite-bridge.json');
const snapshotPath = join(outputDir, '00-upstream-adapter-prerequisite-bridge.md');
const commandsPath = join(outputDir, '10-validation-commands.log');

const sourceArtifacts = {
  blockerLedger: 'output/screen-ai-pipeline-proof/adapter-blocker-ledger/proof-summary.json',
  appGameBroadBlocking: 'output/app-game-plan-proof/23-broad-blocking-proof-gates/03-runtime-evidence.json',
  appGameBroadBlockingRollback: 'output/app-game-plan-proof/23-broad-blocking-proof-gates/12-rollback-proof.md',
  appInstallPackageSourceExecution:
    'test-results/app-install-purchase-package-source-adapter-execution-proof/proof.json',
  appInstallProviderStoreExecution: 'test-results/app-install-purchase-provider-store-api-execution-proof/proof.json',
  appInstallExternalWriterTransportExecution:
    'test-results/app-install-purchase-external-runtime-writer-transport-execution-proof/proof.json',
  networkActionResultState: 'output/network-plan-proof/53-action-result-state-proof/proof-summary.json',
  managedBrowserCdpCapture:
    'output/screen-plan-proof/33-managed-browser-cdp-screenshot-capture-path/proof-summary.json',
  androidMediaProjectionCapability: 'output/screen-plan-proof/android/proof-summary.json',
  iosReplayKitCapability: 'output/screen-plan-proof/ios/proof-summary.json',
  linuxCaptureCapability: 'output/screen-plan-proof/linux/proof-summary.json',
  screenAiPipelineChecklist: 'docs/plans/screen-ai-pipeline-plan/implementation-checklist.md',
};

const failures = [];
const blockerLedger = readJson(sourceArtifacts.blockerLedger);
const appGameBroadBlocking = readJson(sourceArtifacts.appGameBroadBlocking);
const appGameBroadBlockingRollback = readText(sourceArtifacts.appGameBroadBlockingRollback);
const appInstallPackageSourceExecution = readJson(sourceArtifacts.appInstallPackageSourceExecution);
const appInstallProviderStoreExecution = readJson(sourceArtifacts.appInstallProviderStoreExecution);
const appInstallExternalWriterTransportExecution = readJson(sourceArtifacts.appInstallExternalWriterTransportExecution);
const networkActionResultState = readJson(sourceArtifacts.networkActionResultState);
const managedBrowserCdpCapture = readJson(sourceArtifacts.managedBrowserCdpCapture);
const androidMediaProjectionCapability = readJson(sourceArtifacts.androidMediaProjectionCapability);
const iosReplayKitCapability = readJson(sourceArtifacts.iosReplayKitCapability);
const linuxCaptureCapability = readJson(sourceArtifacts.linuxCaptureCapability);
const screenAiPipelineChecklist = readText(sourceArtifacts.screenAiPipelineChecklist);

assert(blockerLedger.status === 'blocked-but-actionable', 'blocker ledger must stay blocked-but-actionable');
assert(blockerLedger.closure?.adapterCompletionStillBlocked === true, 'adapter completion must remain blocked');
assert(
  appGameBroadBlocking.proofMode === 'app-game-broad-blocking-proof-gates',
  'app/game broad blocking proof mode mismatch'
);
assert(appGameBroadBlocking.counts?.gateCount === 7, 'app/game broad blocking gate count mismatch');
assert(appGameBroadBlocking.counts?.dispatchEligible === 0, 'app/game broad blocking unexpectedly dispatch eligible');
assert(
  appGameBroadBlocking.counts?.adapterCallAllowed === 0,
  'app/game broad blocking unexpectedly allows adapter calls'
);
assert(
  appGameBroadBlocking.counts?.broadBlockingClaimed === 0,
  'app/game broad blocking unexpectedly claims broad blocking'
);
assert(
  appGameBroadBlockingRollback.includes('Rollback execution was not implemented in WP23.'),
  'app/game rollback proof must remain explicit non-execution'
);
assert(
  appInstallPackageSourceExecution.proofMode === 'app-install-purchase-package-source-adapter-execution-proof',
  'app-install package-source execution proof mode mismatch'
);
assert(
  appInstallPackageSourceExecution.packageSourceAdapterExecutionSummary?.localAdapterExecutedRows === 1,
  'app-install package-source proof must include one local adapter execution row'
);
assert(
  appInstallPackageSourceExecution.packageSourceAdapterExecutionRows?.every(
    (row) => row.appBlockingClaim === 'not-claimed'
  ),
  'app-install package-source proof must not claim app blocking'
);
assert(
  appInstallProviderStoreExecution.proofMode === 'app-install-purchase-provider-store-api-execution-proof',
  'app-install provider-store execution proof mode mismatch'
);
assert(
  appInstallProviderStoreExecution.providerStoreApiExecutionSummary?.providerExecutedRows === 0,
  'app-install provider-store proof unexpectedly executed provider rows'
);
assert(
  appInstallProviderStoreExecution.providerStoreApiExecutionRows?.every(
    (row) => row.appBlockingClaim === 'not-claimed'
  ),
  'app-install provider-store proof must not claim app blocking'
);
assert(
  appInstallExternalWriterTransportExecution.proofMode ===
    'app-install-purchase-external-runtime-writer-transport-execution-proof',
  'app-install external writer transport execution proof mode mismatch'
);
assert(
  appInstallExternalWriterTransportExecution.externalRuntimeWriterTransportExecutionSummary
    ?.externalRuntimeWriterExecutedRows === 0,
  'app-install external writer transport proof unexpectedly executed writer rows'
);
assert(
  appInstallExternalWriterTransportExecution.externalRuntimeWriterTransportExecutionRows?.every(
    (row) => row.appBlockingClaim === 'not-claimed'
  ),
  'app-install external writer transport proof must not claim app blocking'
);
assert(
  networkActionResultState.proof === 'network-action-result-state-proof',
  'network action result state proof mode mismatch'
);
assert(
  networkActionResultState.claimsProved?.includes(
    'blocked result state requires grade A block policy and apply-ready adapter proof refs'
  ),
  'network action result state proof must require apply-ready adapter proof refs'
);
assert(
  networkActionResultState.notClaimed?.includes('live host DNS mutation'),
  'network proof must not claim live host DNS mutation'
);
assert(
  networkActionResultState.notClaimed?.includes('adapter command invocation'),
  'network proof must not claim adapter command invocation'
);
assert(
  managedBrowserCdpCapture.proofTopic === 'screen-managed-browser-cdp-screenshot-capture-path',
  'managed browser CDP capture proof mode mismatch'
);
assert(
  managedBrowserCdpCapture.captureSummary?.allDeleted === true,
  'managed browser CDP capture proof must delete captured image bytes'
);
assert(
  managedBrowserCdpCapture.nonClaims?.includes(
    'this proof does not claim managed-browser production URL-trigger ownership'
  ),
  'managed browser CDP proof must not claim production URL-trigger ownership'
);
assert(
  androidMediaProjectionCapability.gapStatus?.emulatorMediaProjectionProofExists === true,
  'Android MediaProjection emulator proof must be present'
);
assert(
  androidMediaProjectionCapability.gapStatus?.productAndroidCaptureReady === false,
  'Android capability proof must keep product capture readiness false'
);
assert(
  iosReplayKitCapability.gapStatus?.productIosCaptureReady === false,
  'iOS ReplayKit capability proof must keep product capture readiness false'
);
assert(
  iosReplayKitCapability.gapStatus?.arbitraryBackgroundOtherAppCaptureClaimed === false,
  'iOS ReplayKit proof must reject arbitrary background capture'
);
assert(
  linuxCaptureCapability.gapStatus?.wslgX11SelectedWindowProofExists === true,
  'Linux WSLg selected-window proof must be present'
);
assert(
  linuxCaptureCapability.gapStatus?.productLinuxCaptureReady === false,
  'Linux capability proof must keep product capture readiness false'
);
assert(
  screenAiPipelineChecklist.includes(
    '- [ ] Browser, network, mobile, and broad block adapters proven from screen-derived decisions before product-complete action claims.'
  ),
  'screen-AI final adapter row must remain open'
);

const rows = blockerLedger.rows.map((row) => bridgeRowFor(row));

assert(rows.length === 5, 'expected five remaining upstream bridge rows');
assert(
  rows.filter((row) => row.upstreamPrerequisiteState === 'readiness-proof-present-execution-missing').length === 2,
  'expected exactly two blockers with upstream readiness proof present'
);
assert(
  rows.filter((row) => row.upstreamPrerequisiteState === 'capture-prerequisite-present-control-execution-missing')
    .length === 2,
  'expected exactly two blockers with capture prerequisites present but control execution missing'
);
assert(
  rows.filter((row) => row.upstreamPrerequisiteState === 'source-doc-prerequisite-present-physical-execution-missing')
    .length === 1,
  'expected exactly one blocker with source-doc prerequisite present but physical execution missing'
);
assert(
  rows.every((row) => row.finalAdapterCompletionClaimed === false),
  'no bridge row may claim final adapter completion'
);

if (failures.length > 0) {
  throw new Error(
    `Screen AI upstream adapter prerequisite bridge proof failed:\n${failures
      .map((failure) => `- ${failure}`)
      .join('\n')}`
  );
}

const bridge = {
  schemaVersion: 'v0.6',
  bridgeId: 'screen-ai-upstream-adapter-prerequisite-bridge',
  generatedAt: new Date().toISOString(),
  sourceArtifacts,
  rows,
};

const proof = {
  status: 'upstream-prerequisites-partial-final-adapters-blocked',
  proofKind: 'screen-ai-upstream-adapter-prerequisite-bridge-proof',
  generatedAt: bridge.generatedAt,
  sourceArtifacts,
  bridge: relativePath(bridgePath),
  closure: {
    finalAdapterRowStillOpen: true,
    appGameBroadBlockingReadinessPresent: true,
    appGameBroadBlockingExecutionMissing: true,
    appInstallPackageSourceExecutionPresent: true,
    appInstallProviderStoreExecutionContextPresent: true,
    appInstallExternalWriterTransportStillBlocked: true,
    appInstallAppBlockingClaimed: false,
    networkActionResultReadinessPresent: true,
    networkActionExecutionMissing: true,
    managedBrowserCapturePrerequisitePresent: true,
    androidCapturePrerequisitePresentPhysicalControlMissing: true,
    iosSourceDocPrerequisitePresentPhysicalControlMissing: true,
    linuxWsl2HostExecutionHandledBySeparateArtifact: true,
    finalAdapterCompletionClaimed: false,
  },
  rows,
  nonClaims: [
    'This proof does not implement broad installed-app, browser exact URL, host network/domain, Android, iOS, or Linux adapters.',
    'This proof consumes app-install package-source/provider/transport context from current main but does not treat it as screen-derived broad installed-app blocking because those artifacts explicitly keep appBlockingClaim not-claimed.',
    'This proof does not consume unmerged worker branches; it only records upstream artifacts present in the current rebased branch.',
    'This proof does not close the final product-complete adapter row.',
  ],
};

writeOutputs(bridge, proof);
console.log(`screen-ai-upstream-adapter-prerequisite-bridge-proof-ok:${relativePath(proofPath)}`);

function bridgeRowFor(blockerRow) {
  if (blockerRow.rowId === 'screen-ai-broad-installed-app-manual-required') {
    return {
      rowId: blockerRow.rowId,
      adapterClass: blockerRow.adapterClass,
      upstreamPrerequisiteState: 'readiness-proof-present-execution-missing',
      upstreamProofArtifact: sourceArtifacts.appGameBroadBlocking,
      upstreamRollbackArtifact: sourceArtifacts.appGameBroadBlockingRollback,
      upstreamContextArtifacts: [
        sourceArtifacts.appInstallPackageSourceExecution,
        sourceArtifacts.appInstallProviderStoreExecution,
        sourceArtifacts.appInstallExternalWriterTransportExecution,
      ],
      upstreamReadinessProved: true,
      upstreamExecutionProved: false,
      upstreamAppInstallContextProved: true,
      upstreamAppBlockingClaimed: false,
      requiredFinalArtifact: blockerRow.requiredProofArtifact,
      finalAdapterCompletionClaimed: false,
      nextAction:
        'Wait for app/game or app-install ownership to provide a screen-derived broad installed-app apply, rollback, and audit custody execution artifact with app blocking explicitly claimed before closing this row.',
    };
  }

  if (blockerRow.rowId === 'screen-ai-host-network-domain-manual-required') {
    return {
      rowId: blockerRow.rowId,
      adapterClass: blockerRow.adapterClass,
      upstreamPrerequisiteState: 'readiness-proof-present-execution-missing',
      upstreamProofArtifact: sourceArtifacts.networkActionResultState,
      upstreamRollbackArtifact: null,
      upstreamReadinessProved: true,
      upstreamExecutionProved: false,
      requiredFinalArtifact: blockerRow.requiredProofArtifact,
      finalAdapterCompletionClaimed: false,
      nextAction:
        'Wait for the network lane to provide a screen-derived host network/domain apply, rollback, and audit custody execution artifact before closing this row.',
    };
  }

  if (blockerRow.rowId === 'screen-ai-managed-active-tab-not-claimed') {
    return capturePrerequisiteRow({
      blockerRow,
      upstreamProofArtifact: sourceArtifacts.managedBrowserCdpCapture,
      nextAction:
        'Wait for the browser lane to provide a screen-derived exact active-tab URL apply, rollback, and audit custody execution artifact before closing this row.',
    });
  }

  if (blockerRow.rowId === 'screen-ai-android-mobile-control-manual-required') {
    return capturePrerequisiteRow({
      blockerRow,
      upstreamProofArtifact: sourceArtifacts.androidMediaProjectionCapability,
      nextAction:
        'Wait for mobile platform work to provide physical Android Device Owner or managed-profile control execution with rollback, audit, and custody proof before closing this row.',
    });
  }

  if (blockerRow.rowId === 'screen-ai-ios-mobile-control-manual-required') {
    return {
      rowId: blockerRow.rowId,
      adapterClass: blockerRow.adapterClass,
      upstreamPrerequisiteState: 'source-doc-prerequisite-present-physical-execution-missing',
      upstreamProofArtifact: sourceArtifacts.iosReplayKitCapability,
      upstreamRollbackArtifact: null,
      upstreamReadinessProved: true,
      upstreamExecutionProved: false,
      requiredFinalArtifact: blockerRow.requiredProofArtifact,
      finalAdapterCompletionClaimed: false,
      nextAction:
        'Wait for physical iOS Family Controls or DeviceActivity control execution with rollback, audit, and custody proof before closing this row.',
    };
  }

  if (blockerRow.rowId === 'screen-ai-linux-host-adapter-unavailable') {
    return capturePrerequisiteRow({
      blockerRow,
      upstreamProofArtifact: sourceArtifacts.linuxCaptureCapability,
      nextAction:
        'Wait for Linux host control execution with native session/platform proof, rollback, audit, and custody before closing this row.',
    });
  }

  return {
    rowId: blockerRow.rowId,
    adapterClass: blockerRow.adapterClass,
    upstreamPrerequisiteState: 'upstream-proof-missing-or-owned-elsewhere',
    upstreamProofArtifact: null,
    upstreamRollbackArtifact: null,
    upstreamReadinessProved: false,
    upstreamExecutionProved: false,
    requiredFinalArtifact: blockerRow.requiredProofArtifact,
    finalAdapterCompletionClaimed: false,
    nextAction:
      'Wait for the owning browser, network, mobile, or Linux adapter lane to provide readiness and screen-derived execution custody artifacts.',
  };
}

function capturePrerequisiteRow({ blockerRow, upstreamProofArtifact, nextAction }) {
  return {
    rowId: blockerRow.rowId,
    adapterClass: blockerRow.adapterClass,
    upstreamPrerequisiteState: 'capture-prerequisite-present-control-execution-missing',
    upstreamProofArtifact,
    upstreamRollbackArtifact: null,
    upstreamReadinessProved: true,
    upstreamExecutionProved: false,
    requiredFinalArtifact: blockerRow.requiredProofArtifact,
    finalAdapterCompletionClaimed: false,
    nextAction,
  };
}

function readJson(path) {
  return JSON.parse(readText(path));
}

function readText(path) {
  const absolute = resolve(repoRoot, path);
  assert(existsSync(absolute), `missing source artifact ${path}`);
  return readFileSync(absolute, 'utf8');
}

function writeOutputs(bridge, proof) {
  mkdirSync(outputDir, { recursive: true });
  writeFileSync(bridgePath, `${JSON.stringify(bridge, null, 2)}\n`);
  writeFileSync(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  writeFileSync(snapshotPath, markdownSnapshot(proof));
  writeFileSync(commandsPath, validationCommands());
}

function markdownSnapshot(proof) {
  const rows = proof.rows
    .map(
      (row) =>
        `- ${row.adapterClass}: ${row.upstreamPrerequisiteState}; final completion claimed: ${row.finalAdapterCompletionClaimed}; app blocking claimed: ${row.upstreamAppBlockingClaimed ?? 'n/a'}.`
    )
    .join('\n');
  return `# Screen AI Upstream Adapter Prerequisite Bridge\n\nGenerated: ${proof.generatedAt}\n\nStatus: ${proof.status}\n\n## Rows\n\n${rows}\n\n## Closure\n\n\`\`\`json\n${JSON.stringify(proof.closure, null, 2)}\n\`\`\`\n`;
}

function validationCommands() {
  return [
    'node --check scripts/test/screen-ai-upstream-adapter-prerequisite-bridge-proof.mjs',
    'node scripts/test/screen-ai-upstream-adapter-prerequisite-bridge-proof.mjs',
    'git diff --check',
    'npm run lanes:guard',
    'npm run hub:guard',
    '',
  ].join('\n');
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}

function assert(condition, message) {
  if (!condition) {
    failures.push(message);
  }
}
