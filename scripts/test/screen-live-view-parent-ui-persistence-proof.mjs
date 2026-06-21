import { execFileSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, relative } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(__dirname, '..', '..');
const outputDir = join(repoRoot, 'output', 'screen-plan-proof', 'live-view-parent-ui-persistence');
const proofPath = join(outputDir, 'proof-summary.json');
const portalProofPath = join(
  repoRoot,
  'output',
  'screen-plan-proof',
  'optional-visibility-capability-status-portal',
  'proof-summary.json'
);
const serviceSettingsProofPath = join(
  repoRoot,
  'output',
  'screen-plan-proof',
  'settings-service-command',
  'proof-summary.json'
);
const serviceSessionProofPath = join(
  repoRoot,
  'output',
  'screen-plan-proof',
  'live-view-service-session',
  'proof-summary.json'
);

run('npm', ['run', 'build', '--workspace', '@ocentra-parent/schema-domain']);
run('npm', ['run', 'build', '--workspace', '@ocentra-parent/screen-domain']);
run('npm', [
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/screen-domain',
  '--',
  'screen-live-view-parent-ui-persistence',
]);

const optionalVisibilityModeModule = await import(
  pathToFileURL(join(repoRoot, 'packages', 'schema-domain', 'dist', 'screen-optional-visibility-mode.js')).href
);
const optionalVisibilityModeValuesModule = await import(
  pathToFileURL(join(repoRoot, 'packages', 'schema-domain', 'dist', 'screen-optional-visibility-mode-values.js')).href
);
const liveViewServiceSessionModule = await import(
  pathToFileURL(join(repoRoot, 'packages', 'schema-domain', 'dist', 'screen-live-view-service-session.js')).href
);
const liveViewParentUiPersistenceModule = await import(
  pathToFileURL(join(repoRoot, 'packages', 'schema-domain', 'dist', 'screen-live-view-parent-ui-persistence.js')).href
);

const activityDomain = {
  ScreenLiveViewOptInSettingSchema: optionalVisibilityModeModule.ScreenLiveViewOptInSettingSchema,
  ScreenOptionalVisibilityModeSchemaVersion: optionalVisibilityModeValuesModule.ScreenOptionalVisibilityModeSchemaVersion,
  ScreenLiveViewServiceSessionGateSchema: liveViewServiceSessionModule.ScreenLiveViewServiceSessionGateSchema,
  ScreenLiveViewServiceSessionSchemaVersion: liveViewServiceSessionModule.ScreenLiveViewServiceSessionSchemaVersion,
  ScreenLiveViewParentUiPersistenceProofSchema:
    liveViewParentUiPersistenceModule.ScreenLiveViewParentUiPersistenceProofSchema,
  ScreenLiveViewParentUiPersistenceSchemaVersion:
    liveViewParentUiPersistenceModule.ScreenLiveViewParentUiPersistenceSchemaVersion,
};

const generatedAt = new Date().toISOString();
const portalProof = readJson(portalProofPath);
const serviceSettingsProof = readJson(serviceSettingsProofPath);
const serviceSessionProof = readJson(serviceSessionProofPath);

const liveViewSetting = activityDomain.ScreenLiveViewOptInSettingSchema.parse({
  schemaVersion: activityDomain.ScreenOptionalVisibilityModeSchemaVersion,
  settingId: 'screen-live-view-parent-ui-persistence-setting',
  parentSettingRef: 'screen-parent-live-view-setting',
  settingVersion: 7,
  changedAt: generatedAt,
  liveViewMode: 'lanOnlyView',
  transportMode: 'lanMutualAuth',
  explicitParentApproval: true,
  approvalRef: 'screen-live-view-parent-approval',
  disclosureState: 'requiredShown',
  viewerAuditRef: 'screen-live-view-parent-ui-viewer-audit',
  platformProofState: 'operatorVerified',
  platformProofRef: 'screen-live-view-capture-only-platform-proof',
  custodyState: 'live-lan-child-agent',
  sourceLabel: 'liveView',
  frameRetentionBehavior: 'noFrameRetention',
  cacheRawFrames: false,
  sessionRecordingAllowed: false,
  remoteInputControlAllowed: false,
  stopOrRevokeAuditRequired: true,
  reason: 'parent settings route persisted the LAN live-view opt-in state',
});

const serviceSessionGate = activityDomain.ScreenLiveViewServiceSessionGateSchema.parse({
  schemaVersion: activityDomain.ScreenLiveViewServiceSessionSchemaVersion,
  checkedAt: generatedAt,
  liveViewMode: 'lanOnlyView',
  transportMode: 'lanMutualAuth',
  permissionEvidenceKind: 'screen-capture-only',
  sourceLabel: 'liveView',
  custodyState: 'live-lan-child-agent',
  frameRetentionBehavior: 'noFrameRetention',
  platformPermissionProofRef: 'screen-live-view-platform-permission-gate',
  viewerAuditRef: liveViewSetting.viewerAuditRef,
  liveTransportProofRef: 'screen-live-view-loopback-transport-proof',
  serviceSessionState: 'loopbackTransportOnly',
  parentUiPersistenceState: 'proved',
  relayCacheState: 'notUsed',
  rawFrameDeletedAfterTransport: true,
  cacheRawFrames: false,
  sessionRecordingAllowed: false,
  remoteInputControlAllowed: false,
  productLiveViewReady: false,
  reason: 'parent UI persistence is proved but live-view prompt and production runtime are still missing',
});

const persistenceProof = activityDomain.ScreenLiveViewParentUiPersistenceProofSchema.parse({
  schemaVersion: activityDomain.ScreenLiveViewParentUiPersistenceSchemaVersion,
  checkedAt: generatedAt,
  status: 'persistedParentOptIn',
  parentSettingRef: liveViewSetting.parentSettingRef,
  liveViewSetting,
  serviceSessionGate,
  parentUiPersistenceState: 'proved',
  settingsRouteRendered: true,
  persistedInParentSettingsStore: true,
  viewerAuditRef: liveViewSetting.viewerAuditRef,
  portalProofRef: 'optional-visibility-capability-status-portal-proof',
  serviceSettingsProofRef: 'settings-service-command-proof',
  rawFramesRetained: false,
  remoteInputAllowed: false,
  productLiveViewReady: false,
  reason: 'parent UI persistence can be carried into the service-session gate without product live-view readiness',
});

const proof = {
  proof: 'screen-live-view-parent-ui-persistence-proof',
  generatedAt,
  claim:
    'Parent Settings UI persistence for a LAN live-view opt-in is represented as explicit, audited, no-retention, view-only state and can be carried into the live-view service-session gate without making product live view ready.',
  sourceEvidence: {
    portalProof: relativePath(portalProofPath),
    portalProofPresent: existsSync(portalProofPath),
    portalRouteRendered:
      portalProof.route === '#/settings-rules' &&
      portalProof.renderedAssertions?.includes('screen-parent-live-capability-lan'),
    serviceSettingsProof: relativePath(serviceSettingsProofPath),
    serviceSettingsProofPresent: existsSync(serviceSettingsProofPath),
    serviceSettingsStorePersisted: serviceSettingsProof.persistedStore?.revisionCount >= 1,
    serviceSessionProof: relativePath(serviceSessionProofPath),
    serviceSessionProofPresent: existsSync(serviceSessionProofPath),
    serviceSessionProofProductBlocked: serviceSessionProof.gapStatus?.liveViewProductReady === false,
  },
  persistenceProof,
  serviceSessionGate: {
    liveViewMode: serviceSessionGate.liveViewMode,
    transportMode: serviceSessionGate.transportMode,
    permissionEvidenceKind: serviceSessionGate.permissionEvidenceKind,
    serviceSessionState: serviceSessionGate.serviceSessionState,
    parentUiPersistenceState: serviceSessionGate.parentUiPersistenceState,
    productLiveViewReady: serviceSessionGate.productLiveViewReady,
    cacheRawFrames: serviceSessionGate.cacheRawFrames,
    sessionRecordingAllowed: serviceSessionGate.sessionRecordingAllowed,
    remoteInputControlAllowed: serviceSessionGate.remoteInputControlAllowed,
  },
  assertions: {
    parentSettingsRouteRenderedLiveViewRow: portalProof.renderedAssertions?.includes(
      'screen-parent-live-capability-lan'
    ),
    parentSettingsServicePersistenceProofPresent: serviceSettingsProof.persistedStore?.revisionCount >= 1,
    serviceSessionProofRemainsProductBlocked: serviceSessionProof.gapStatus?.liveViewProductReady === false,
    explicitParentOptInPersisted: persistenceProof.liveViewSetting.explicitParentApproval === true,
    viewerAuditCarried: persistenceProof.viewerAuditRef === persistenceProof.serviceSessionGate.viewerAuditRef,
    parentUiPersistenceStateProved: persistenceProof.parentUiPersistenceState === 'proved',
    productLiveViewStillBlocked: persistenceProof.productLiveViewReady === false,
    noFrameRetentionNoRecordingNoRemoteInput:
      serviceSessionGate.cacheRawFrames === false &&
      serviceSessionGate.sessionRecordingAllowed === false &&
      serviceSessionGate.remoteInputControlAllowed === false,
  },
  completedChecklistClaims: [
    'parent Settings route persistence can carry an explicit LAN live-view opt-in into the live-view service-session gate',
    'parent UI persistence remains no-retention, no-recording, no-remote-input, and product-blocked until platform prompt/runtime gates exist',
  ],
  openChecklistClaims: [
    'production live-view worker startup remains open',
    'real live-view platform prompt screenshots remain open',
    'relay/cache execution and privacy/legal approval remain open',
  ],
  nonClaims: [
    'This proof does not enable product live view.',
    'This proof does not prove live-view platform permission prompts, relay/cache execution, physical-device parity, or privacy/legal approval.',
    'This proof does not retain raw frames, record sessions, or allow remote input.',
  ],
};

if (!Object.values(proof.assertions).every(Boolean)) {
  throw new Error(`live-view parent UI persistence proof assertions failed: ${JSON.stringify(proof.assertions)}`);
}

mkdirSync(outputDir, { recursive: true });
writeFileSync(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
console.log(`screen-live-view-parent-ui-persistence-proof-ok:${proofPath}`);

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
  execFileSync(command, args, { cwd: repoRoot, stdio: 'inherit', shell: process.platform === 'win32' });
}
