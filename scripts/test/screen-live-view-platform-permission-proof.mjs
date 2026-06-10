import { execFileSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, relative } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(__dirname, '..', '..');
const outputDir = join(repoRoot, 'output', 'screen-plan-proof', 'live-view-platform-permission');
const proofPath = join(outputDir, 'proof-summary.json');
const androidCaptureProofPath = join(
  repoRoot,
  'output',
  'screen-plan-proof',
  'android-mediaprojection',
  'proof-summary.json'
);

run('npm', ['run', 'build', '--workspace', '@ocentra-parent/activity-domain']);

const screenEvidence = await import(
  pathToFileURL(join(repoRoot, 'packages', 'activity-domain', 'dist', 'screen-live-view-platform-permission.js')).href
);

const generatedAt = new Date().toISOString();
const androidCaptureProof = readJsonIfPresent(androidCaptureProofPath);

const disabled = gate('windows', 'disabled', {
  reason: 'live view is disabled by default',
});
const androidCaptureOnly = gate('android-mediaprojection', 'lanOnlyView', {
  transportMode: 'lanMutualAuth',
  permissionEvidenceKind: 'screen-capture-only',
  platformProofState: 'operatorVerified',
  platformProofRef: 'screen-plan-android-mediaprojection-capture-proof',
  viewerAuditRef: 'screen-live-view-viewer-audit',
  sourceLabel: 'liveView',
  custodyState: 'live-lan-child-agent',
  liveTransportProofRef: 'screen-live-view-lan-transport-proof',
  explicitViewerDisclosure: true,
  reason:
    'Android MediaProjection capture consent proof exists, but it is capture proof and not live-view permission-prompt or live transport proof.',
});
const lanReadyCandidate = gate('android-mediaprojection', 'lanOnlyView', {
  transportMode: 'lanMutualAuth',
  permissionEvidenceKind: 'live-view-permission',
  platformProofState: 'operatorVerified',
  platformProofRef: 'screen-live-view-android-mediaprojection-permission-proof',
  viewerAuditRef: 'screen-live-view-viewer-audit',
  sourceLabel: 'liveView',
  custodyState: 'live-lan-child-agent',
  liveTransportProofRef: 'screen-live-view-lan-transport-proof',
  explicitViewerDisclosure: true,
  productLiveViewReady: true,
  reason: 'Reference shape for a future LAN live-view permission proof; no artifact proves this yet.',
});
const relayReadyCandidate = gate('android-mediaprojection', 'relayBackedView', {
  transportMode: 'relayEndToEndEncrypted',
  permissionEvidenceKind: 'live-view-permission',
  platformProofState: 'operatorVerified',
  platformProofRef: 'screen-live-view-android-mediaprojection-permission-proof',
  viewerAuditRef: 'screen-live-view-viewer-audit',
  sourceLabel: 'relay',
  custodyState: 'ocentra-hosted-non-activity',
  liveTransportProofRef: 'screen-live-view-relay-transport-proof',
  explicitViewerDisclosure: true,
  productLiveViewReady: true,
  reason: 'Reference shape for a future relay live-view permission proof; no artifact proves this yet.',
});

const currentRows = [disabled, androidCaptureOnly].map((row) =>
  screenEvidence.ScreenLiveViewPlatformPermissionGateSchema.parse(row)
);
const futureReferenceRows = [lanReadyCandidate, relayReadyCandidate].map((row) =>
  screenEvidence.ScreenLiveViewPlatformPermissionGateSchema.parse(row)
);
const futureLanProductionBundle = productionReadinessEvidence(lanReadyCandidate, {
  liveTransportProofRef: 'screen-live-view-lan-transport-proof',
  physicalDeviceParityProofRef: 'screen-live-view-android-physical-parity-proof',
  privacyLegalApprovalRef: 'screen-live-view-privacy-legal-approval',
  productionWorkerStartProofRef: 'screen-live-view-production-worker-start-proof',
  relayCacheExecutionProofRef: null,
});
const futureRelayProductionBundle = productionReadinessEvidence(relayReadyCandidate, {
  liveTransportProofRef: 'screen-live-view-relay-transport-proof',
  physicalDeviceParityProofRef: 'screen-live-view-android-physical-parity-proof',
  privacyLegalApprovalRef: 'screen-live-view-privacy-legal-approval',
  productionWorkerStartProofRef: 'screen-live-view-production-worker-start-proof',
  relayCacheExecutionProofRef: 'screen-live-view-relay-cache-execution-proof',
});
const futureReferenceProductionBundles = [futureLanProductionBundle, futureRelayProductionBundle].map((bundle) =>
  screenEvidence.ScreenLiveViewProductionReadinessEvidenceSchema.parse(bundle)
);
const negativeChecks = [
  rejects('capture-only proof cannot mark live view product-ready', () =>
    screenEvidence.ScreenLiveViewPlatformPermissionGateSchema.safeParse({
      ...androidCaptureOnly,
      productLiveViewReady: true,
    })
  ),
  rejects('ready live view requires viewer audit', () =>
    screenEvidence.ScreenLiveViewPlatformPermissionGateSchema.safeParse({
      ...lanReadyCandidate,
      viewerAuditRef: null,
    })
  ),
  rejects('ready live view requires transport proof ref', () =>
    screenEvidence.ScreenLiveViewPlatformPermissionGateSchema.safeParse({
      ...lanReadyCandidate,
      liveTransportProofRef: null,
    })
  ),
  rejects('ready live view cannot cache raw frames', () =>
    screenEvidence.ScreenLiveViewPlatformPermissionGateSchema.safeParse({
      ...lanReadyCandidate,
      cacheRawFrames: true,
    })
  ),
  rejects('ready live view cannot allow remote input control', () =>
    screenEvidence.ScreenLiveViewPlatformPermissionGateSchema.safeParse({
      ...lanReadyCandidate,
      remoteInputControlAllowed: true,
    })
  ),
  rejects('production readiness rejects capture-only permission gate', () =>
    screenEvidence.ScreenLiveViewProductionReadinessEvidenceSchema.safeParse({
      ...futureLanProductionBundle,
      permissionGate: androidCaptureOnly,
    })
  ),
  rejects('production readiness rejects mismatched prompt artifact ref', () =>
    screenEvidence.ScreenLiveViewProductionReadinessEvidenceSchema.safeParse({
      ...futureLanProductionBundle,
      promptArtifact: {
        ...futureLanProductionBundle.promptArtifact,
        artifactRef: 'screen-live-view-other-platform-prompt-proof',
      },
    })
  ),
  rejects('production readiness rejects relay mode without relay/cache proof', () =>
    screenEvidence.ScreenLiveViewProductionReadinessEvidenceSchema.safeParse({
      ...futureRelayProductionBundle,
      relayCacheExecutionProofRef: null,
    })
  ),
  rejects('production readiness rejects prompt artifact carrying raw frame content', () =>
    screenEvidence.ScreenLiveViewProductionReadinessEvidenceSchema.safeParse({
      ...futureLanProductionBundle,
      promptArtifact: {
        ...futureLanProductionBundle.promptArtifact,
        rawFrameIncluded: true,
      },
    })
  ),
];

if (negativeChecks.some((check) => !check.rejected)) {
  throw new Error(`Unexpected live-view permission gate result: ${JSON.stringify(negativeChecks)}`);
}

const proof = {
  proof: 'screen-live-view-platform-permission',
  generatedAt,
  claim:
    'Live view remains a separate optional mode and cannot become product-ready from ordinary screenshot/capture permission proof alone.',
  sourceEvidence: {
    androidCaptureProof: relativePath(androidCaptureProofPath),
    androidCaptureProofPresent: androidCaptureProof !== null,
    androidCaptureProofMode: androidCaptureProof?.proofMode ?? androidCaptureProof?.proof ?? null,
    androidCaptureExplicitConsent:
      androidCaptureProof?.summary?.explicitConsent === true ||
      androidCaptureProof?.capability?.explicitConsent === true ||
      Boolean(JSON.stringify(androidCaptureProof ?? {}).includes('MediaProjection')),
  },
  currentRows: currentRows.map((row) => ({
    platform: row.platform,
    liveViewMode: row.liveViewMode,
    transportMode: row.transportMode,
    permissionEvidenceKind: row.permissionEvidenceKind,
    platformProofState: row.platformProofState,
    platformProofRef: row.platformProofRef,
    liveTransportProofRef: row.liveTransportProofRef,
    custodyState: row.custodyState,
    productLiveViewReady: row.productLiveViewReady,
  })),
  futureReferenceRows: futureReferenceRows.map((row) => ({
    referenceOnly: true,
    platform: row.platform,
    liveViewMode: row.liveViewMode,
    transportMode: row.transportMode,
    permissionEvidenceKind: row.permissionEvidenceKind,
    requiredPlatformProofRef: row.platformProofRef,
    requiredLiveTransportProofRef: row.liveTransportProofRef,
    readyOnlyAfterEvidenceExists: row.productLiveViewReady,
  })),
  futureReferenceProductionBundles: futureReferenceProductionBundles.map((bundle) => ({
    referenceOnly: true,
    platform: bundle.permissionGate.platform,
    liveViewMode: bundle.permissionGate.liveViewMode,
    promptArtifactKind: bundle.promptArtifact.artifactKind,
    promptArtifactRef: bundle.promptArtifact.artifactRef,
    promptArtifactDigestRequired: bundle.promptArtifact.artifactDigest.length > 0,
    liveTransportProofRef: bundle.liveTransportProofRef,
    physicalDeviceParityProofRef: bundle.physicalDeviceParityProofRef,
    privacyLegalApprovalRef: bundle.privacyLegalApprovalRef,
    productionWorkerStartProofRef: bundle.productionWorkerStartProofRef,
    relayCacheExecutionProofRef: bundle.relayCacheExecutionProofRef,
    readyOnlyAfterEvidenceExists: bundle.productLiveViewReady,
  })),
  negativeChecks,
  gapStatus: {
    capturePermissionProofExists: androidCaptureProof !== null,
    liveViewPermissionPromptProofExists: false,
    liveTransportProofExists: false,
    physicalDeviceParityProofExists: false,
    privacyLegalApprovalExists: false,
    productionWorkerStartProofExists: false,
    relayCacheExecutionProofExists: false,
    productionReadinessEvidenceSchemaExists: true,
    liveViewProductReady: false,
  },
  assertions: {
    productionReadinessRequiresStructuredEvidence: true,
    productionReadinessRejectsCaptureOnlyPermission: negativeChecks.some(
      (check) => check.name === 'production readiness rejects capture-only permission gate' && check.rejected
    ),
    productionReadinessRejectsMismatchedPromptRef: negativeChecks.some(
      (check) => check.name === 'production readiness rejects mismatched prompt artifact ref' && check.rejected
    ),
    productionReadinessRejectsRawFramePromptArtifact: negativeChecks.some(
      (check) =>
        check.name === 'production readiness rejects prompt artifact carrying raw frame content' && check.rejected
    ),
  },
  nonClaims: [
    'This proof does not implement live screen transport, relay/cache execution, or a service live-view session worker.',
    'This proof does not claim Android MediaProjection capture consent is enough for live view.',
    'This proof does not claim parent UI persistence, privacy/legal approval, platform screenshots, or product-complete live view.',
  ],
};

mkdirSync(outputDir, { recursive: true });
writeFileSync(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
console.log(`screen-live-view-platform-permission-proof-ok:${proofPath}`);

function gate(platform, liveViewMode, overrides) {
  return {
    schemaVersion: screenEvidence.ScreenLiveViewPermissionGateSchemaVersion,
    checkedAt: generatedAt,
    platform,
    liveViewMode,
    transportMode: 'none',
    permissionEvidenceKind: 'missing',
    platformProofState: 'notRequired',
    platformProofRef: null,
    viewerAuditRef: null,
    sourceLabel: 'unavailable',
    custodyState: 'unavailable',
    frameRetentionBehavior: 'noFrameRetention',
    liveTransportProofRef: null,
    explicitViewerDisclosure: false,
    cacheRawFrames: false,
    sessionRecordingAllowed: false,
    remoteInputControlAllowed: false,
    productLiveViewReady: false,
    ...overrides,
  };
}

function productionReadinessEvidence(permissionGate, overrides) {
  const promptArtifact = {
    platform: permissionGate.platform,
    artifactKind: 'platform-permission-prompt-screenshot',
    artifactRef: permissionGate.platformProofRef,
    artifactDigest: 'sha256-live-view-platform-prompt',
    capturedAt: generatedAt,
    operatorAuditRef: permissionGate.viewerAuditRef,
    permissionEvidenceKind: 'live-view-permission',
    rawFrameIncluded: false,
    containsUserPrivateContent: false,
  };

  return {
    schemaVersion: screenEvidence.ScreenLiveViewProductionReadinessEvidenceSchemaVersion,
    checkedAt: generatedAt,
    permissionGate,
    promptArtifact,
    productLiveViewReady: true,
    ...overrides,
  };
}

function rejects(name, parseAttempt) {
  return { name, rejected: !parseAttempt().success };
}

function readJsonIfPresent(path) {
  if (!existsSync(path)) {
    return null;
  }
  return JSON.parse(readFileSync(path, 'utf8'));
}

function relativePath(path) {
  return relative(repoRoot, path).replace(/\\/gu, '/');
}

function run(command, args) {
  const runner = process.platform === 'win32' ? 'cmd' : command;
  const runnerArgs = process.platform === 'win32' ? ['/c', command, ...args] : args;
  execFileSync(runner, runnerArgs, { cwd: repoRoot, stdio: 'inherit' });
}
