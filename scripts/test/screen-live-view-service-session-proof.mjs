import { execFileSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, relative } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(__dirname, '..', '..');
const outputDir = join(repoRoot, 'output', 'screen-plan-proof', 'live-view-service-session');
const proofPath = join(outputDir, 'proof-summary.json');
const transportProofPath = join(
  repoRoot,
  'output',
  'screen-plan-proof',
  'live-view-session-transport',
  'proof-summary.json'
);
const platformPermissionProofPath = join(
  repoRoot,
  'output',
  'screen-plan-proof',
  'live-view-platform-permission',
  'proof-summary.json'
);

run('npm', ['run', 'build', '--workspace', '@ocentra-parent/activity-domain']);

const liveView = await import(
  pathToFileURL(join(repoRoot, 'packages', 'activity-domain', 'dist', 'screen-live-view-service-session.js')).href
);

const generatedAt = new Date().toISOString();
const transportProof = readJson(transportProofPath);
const platformPermissionProof = readJson(platformPermissionProofPath);

const disabled = sessionGate('disabled', {
  reason: 'live view is disabled by default',
});
const loopbackTransportOnly = sessionGate('lanOnlyView', {
  transportMode: 'lanMutualAuth',
  permissionEvidenceKind: 'screen-capture-only',
  sourceLabel: 'liveView',
  custodyState: 'live-lan-child-agent',
  platformPermissionProofRef: 'screen-live-view-platform-permission-gate',
  viewerAuditRef: transportProof.session.viewerAuditRef,
  liveTransportProofRef: transportProof.session.transportProofRef,
  serviceSessionState: 'loopbackTransportOnly',
  parentUiPersistenceState: 'missing',
  reason:
    'Real loopback frame transport proof exists, but production service session runtime, live-view platform prompt proof, and parent UI persistence are still missing.',
});
const productionLanReadyReference = sessionGate('lanOnlyView', {
  transportMode: 'lanMutualAuth',
  permissionEvidenceKind: 'live-view-permission',
  sourceLabel: 'liveView',
  custodyState: 'live-lan-child-agent',
  platformPermissionProofRef: 'screen-live-view-platform-prompt-proof',
  viewerAuditRef: 'screen-live-view-production-viewer-audit',
  liveTransportProofRef: 'screen-live-view-production-lan-session-proof',
  serviceSessionState: 'serviceRuntimeReady',
  parentUiPersistenceState: 'proved',
  productLiveViewReady: true,
  reason:
    'Reference shape for future production LAN live view after service runtime, platform prompt, parent UI persistence, and transport evidence exist.',
});
const productionRelayReadyReference = sessionGate('relayBackedView', {
  transportMode: 'relayEndToEndEncrypted',
  permissionEvidenceKind: 'live-view-permission',
  sourceLabel: 'relay',
  custodyState: 'ocentra-hosted-non-activity',
  platformPermissionProofRef: 'screen-live-view-platform-prompt-proof',
  viewerAuditRef: 'screen-live-view-production-viewer-audit',
  liveTransportProofRef: 'screen-live-view-production-relay-session-proof',
  serviceSessionState: 'serviceRuntimeReady',
  parentUiPersistenceState: 'proved',
  relayCacheState: 'proved',
  productLiveViewReady: true,
  reason:
    'Reference shape for future production relay live view after relay/cache, service runtime, platform prompt, parent UI persistence, and transport evidence exist.',
});

const currentRows = [disabled, loopbackTransportOnly].map((row) =>
  liveView.ScreenLiveViewServiceSessionGateSchema.parse(row)
);
const futureReferenceRows = [productionLanReadyReference, productionRelayReadyReference].map((row) =>
  liveView.ScreenLiveViewServiceSessionGateSchema.parse(row)
);
const negativeChecks = [
  rejects('loopback transport cannot mark product live view ready', () =>
    liveView.ScreenLiveViewServiceSessionGateSchema.safeParse({
      ...loopbackTransportOnly,
      productLiveViewReady: true,
    })
  ),
  rejects('product live view requires live-view permission evidence', () =>
    liveView.ScreenLiveViewServiceSessionGateSchema.safeParse({
      ...productionLanReadyReference,
      permissionEvidenceKind: 'screen-capture-only',
    })
  ),
  rejects('product live view requires parent UI persistence proof', () =>
    liveView.ScreenLiveViewServiceSessionGateSchema.safeParse({
      ...productionLanReadyReference,
      parentUiPersistenceState: 'missing',
    })
  ),
  rejects('product live view cannot cache frames', () =>
    liveView.ScreenLiveViewServiceSessionGateSchema.safeParse({
      ...productionLanReadyReference,
      cacheRawFrames: true,
    })
  ),
  rejects('product live view cannot record sessions', () =>
    liveView.ScreenLiveViewServiceSessionGateSchema.safeParse({
      ...productionLanReadyReference,
      sessionRecordingAllowed: true,
    })
  ),
  rejects('product live view cannot allow remote input', () =>
    liveView.ScreenLiveViewServiceSessionGateSchema.safeParse({
      ...productionLanReadyReference,
      remoteInputControlAllowed: true,
    })
  ),
];

if (negativeChecks.some((check) => !check.rejected)) {
  throw new Error(`Unexpected live-view service session gate result: ${JSON.stringify(negativeChecks)}`);
}

const proof = {
  proof: 'screen-live-view-service-session-proof',
  generatedAt,
  claim:
    'The real loopback live-frame transport artifact can be represented as a non-product-ready service-session readiness row, while product live view remains blocked until service runtime, live-view platform prompt evidence, parent UI persistence, and no-retention/no-input gates are proven.',
  sourceEvidence: {
    transportProof: relativePath(transportProofPath),
    transportProofPresent: existsSync(transportProofPath),
    realPixelsCaptured: transportProof.assertions.realPixelsCaptured === true,
    localTransportDeliveredFrame: transportProof.assertions.localTransportDeliveredFrame === true,
    rawFrameDeletedAfterTransport: transportProof.assertions.rawFrameDeletedAfterTransport === true,
    platformPermissionProof: relativePath(platformPermissionProofPath),
    platformPermissionProofPresent: existsSync(platformPermissionProofPath),
    platformGateLiveViewProductReady: platformPermissionProof.gapStatus.liveViewProductReady === true,
  },
  currentRows: currentRows.map((row) => ({
    liveViewMode: row.liveViewMode,
    transportMode: row.transportMode,
    permissionEvidenceKind: row.permissionEvidenceKind,
    serviceSessionState: row.serviceSessionState,
    parentUiPersistenceState: row.parentUiPersistenceState,
    relayCacheState: row.relayCacheState,
    liveTransportProofRef: row.liveTransportProofRef,
    rawFrameDeletedAfterTransport: row.rawFrameDeletedAfterTransport,
    productLiveViewReady: row.productLiveViewReady,
  })),
  futureReferenceRows: futureReferenceRows.map((row) => ({
    referenceOnly: true,
    liveViewMode: row.liveViewMode,
    transportMode: row.transportMode,
    requiredPermissionEvidenceKind: row.permissionEvidenceKind,
    requiredServiceSessionState: row.serviceSessionState,
    requiredParentUiPersistenceState: row.parentUiPersistenceState,
    requiredRelayCacheState: row.relayCacheState,
    readyOnlyAfterEvidenceExists: row.productLiveViewReady,
  })),
  negativeChecks,
  gapStatus: {
    loopbackTransportProofExists: transportProof.assertions.localTransportDeliveredFrame === true,
    serviceSessionRuntimeProofExists: false,
    parentUiPersistenceProofExists: false,
    relayCacheProofExists: false,
    liveViewProductReady: false,
  },
  assertions: {
    realLoopbackTransportArtifactConsumed: transportProof.assertions.localTransportDeliveredFrame === true,
    rawFrameDeletionCarriedForward: transportProof.assertions.rawFrameDeletedAfterTransport === true,
    loopbackTransportRemainsNonProductReady: currentRows[1].productLiveViewReady === false,
    productReadinessOverclaimRejected: negativeChecks.every((check) => check.rejected),
    noRemoteInputNoRecordingNoFrameCache:
      currentRows[1].remoteInputControlAllowed === false &&
      currentRows[1].sessionRecordingAllowed === false &&
      currentRows[1].cacheRawFrames === false,
  },
  nonClaims: [
    'This proof does not implement production service live-view session workers.',
    'This proof does not claim platform live-view permission-prompt screenshots or privacy/legal approval.',
    'This proof does not claim parent UI persistence, relay/cache execution, session recording, remote input, or product-complete live view.',
  ],
};

if (!Object.values(proof.assertions).every(Boolean)) {
  throw new Error(`live-view service session proof assertions failed: ${JSON.stringify(proof.assertions)}`);
}

mkdirSync(outputDir, { recursive: true });
writeFileSync(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
console.log(`screen-live-view-service-session-proof-ok:${proofPath}`);

function sessionGate(liveViewMode, overrides) {
  return {
    schemaVersion: liveView.ScreenLiveViewServiceSessionSchemaVersion,
    checkedAt: generatedAt,
    liveViewMode,
    transportMode: 'none',
    permissionEvidenceKind: 'missing',
    sourceLabel: 'unavailable',
    custodyState: 'unavailable',
    frameRetentionBehavior: 'noFrameRetention',
    platformPermissionProofRef: null,
    viewerAuditRef: null,
    liveTransportProofRef: null,
    serviceSessionState: 'disabled',
    parentUiPersistenceState: 'notRequired',
    relayCacheState: 'notUsed',
    rawFrameDeletedAfterTransport: true,
    cacheRawFrames: false,
    sessionRecordingAllowed: false,
    remoteInputControlAllowed: false,
    productLiveViewReady: false,
    ...overrides,
  };
}

function rejects(name, parseAttempt) {
  return { name, rejected: !parseAttempt().success };
}

function readJson(path) {
  if (!existsSync(path)) {
    throw new Error(`Expected proof artifact at ${path}`);
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
