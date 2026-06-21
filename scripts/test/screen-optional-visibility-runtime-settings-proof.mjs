import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(__dirname, '..', '..');
const proofDir = join(repoRoot, 'output', 'screen-plan-proof', 'optional-visibility-runtime-settings');
const proofPath = join(proofDir, 'proof-summary.json');
const generatedAt = '2026-06-07T20:10:00Z';

run('npm', ['run', 'build', '--workspace', '@ocentra-parent/schema-domain']);
run('npm', [
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/screen-domain',
  '--',
  '--run',
  'tests/unit/screen-optional-visibility-runtime-settings.test.ts',
]);

const optionalVisibilityModeModule = await import(
  pathToFileURL(join(repoRoot, 'packages', 'schema-domain', 'dist', 'screen-optional-visibility-mode.js')).href
);
const optionalVisibilityModeValuesModule = await import(
  pathToFileURL(join(repoRoot, 'packages', 'schema-domain', 'dist', 'screen-optional-visibility-mode-values.js')).href
);
const optionalVisibilityRuntimeSettingsModule = await import(
  pathToFileURL(
    join(repoRoot, 'packages', 'schema-domain', 'dist', 'screen-optional-visibility-runtime-settings.js')
  ).href
);

const screenEvidence = {
  createDisabledScreenOptionalVisibilityRuntimeSettingsState:
    optionalVisibilityRuntimeSettingsModule.createDisabledScreenOptionalVisibilityRuntimeSettingsState,
  applyScreenOptionalVisibilityRuntimeSettingsRequest:
    optionalVisibilityRuntimeSettingsModule.applyScreenOptionalVisibilityRuntimeSettingsRequest,
  ScreenOptionalVisibilityRuntimeUpdateRequestSchema:
    optionalVisibilityRuntimeSettingsModule.ScreenOptionalVisibilityRuntimeUpdateRequestSchema,
  ScreenOptionalVisibilityRuntimeSettingsSchemaVersion:
    optionalVisibilityRuntimeSettingsModule.ScreenOptionalVisibilityRuntimeSettingsSchemaVersion,
  ScreenOptionalVisibilityModeSchemaVersion: optionalVisibilityModeValuesModule.ScreenOptionalVisibilityModeSchemaVersion,
  ScreenRawScreenshotRetentionOptInSettingSchema:
    optionalVisibilityModeModule.ScreenRawScreenshotRetentionOptInSettingSchema,
  ScreenLiveViewOptInSettingSchema: optionalVisibilityModeModule.ScreenLiveViewOptInSettingSchema,
};

const state = screenEvidence.createDisabledScreenOptionalVisibilityRuntimeSettingsState({
  updatedAt: generatedAt,
  rawRetentionSetting: disabledRawRetention(),
  liveViewSetting: disabledLiveView(),
  reason: 'optional visibility runtime settings start disabled',
});
const liveViewAccepted = screenEvidence.applyScreenOptionalVisibilityRuntimeSettingsRequest(
  state,
  runtimeRequest({
    requestId: 'screen-optional-visibility-runtime-live-view',
    kind: 'replaceLiveView',
    baseRevision: state.revision,
    rawRetentionSetting: null,
    liveViewSetting: lanLiveView(),
    reason: 'parent approved LAN live view as view-only runtime setting',
  })
);
const staleRejected = screenEvidence.applyScreenOptionalVisibilityRuntimeSettingsRequest(
  state,
  runtimeRequest({
    requestId: 'screen-optional-visibility-runtime-stale',
    kind: 'replaceLiveView',
    baseRevision: 999,
    rawRetentionSetting: null,
    liveViewSetting: lanLiveView(),
    reason: 'stale request cannot replace optional visibility runtime state',
  })
);
const rawState = screenEvidence.applyScreenOptionalVisibilityRuntimeSettingsRequest(
  state,
  runtimeRequest({
    requestId: 'screen-optional-visibility-runtime-raw',
    kind: 'replaceRawRetention',
    baseRevision: state.revision,
    rawRetentionSetting: localRawRetention(),
    liveViewSetting: null,
    reason: 'parent approved local raw retention setting',
  })
);
const conflictRejected =
  rawState.state === null
    ? null
    : screenEvidence.applyScreenOptionalVisibilityRuntimeSettingsRequest(
        rawState.state,
        runtimeRequest({
          requestId: 'screen-optional-visibility-runtime-conflict',
          kind: 'replaceLiveView',
          baseRevision: rawState.state.revision,
          rawRetentionSetting: null,
          liveViewSetting: lanLiveView(),
          reason: 'live view cannot be enabled while raw retention remains active',
        })
      );

const assertions = {
  liveViewAccepted: liveViewAccepted.status === 'accepted',
  liveViewPersistsWithoutProductReadiness: liveViewAccepted.state?.productLiveViewReady === false,
  liveViewPersistsWithoutRawRemoteUpload: liveViewAccepted.state?.rawScreenshotRemoteUploadEnabled === false,
  staleRevisionRejected: staleRejected.rejectionReason === 'stale-revision',
  rawRetentionAcceptedSeparately: rawState.status === 'accepted',
  mixedRawRetentionAndLiveViewRejected: conflictRejected?.rejectionReason === 'mode-conflict',
};

if (!Object.values(assertions).every(Boolean)) {
  throw new Error(`screen optional visibility runtime settings proof failed: ${JSON.stringify(assertions)}`);
}

mkdirSync(proofDir, { recursive: true });
writeFileSync(
  proofPath,
  `${JSON.stringify(
    {
      proof: 'screen-optional-visibility-runtime-settings-proof',
      generatedAt: new Date().toISOString(),
      claim:
        'Optional raw-retention and live-view settings have a separate runtime settings contract that accepts parent-approved live view as view-only state, rejects stale writes, rejects mixed raw-retention/live-view custody, and keeps product live view false.',
      artifactInputs: {
        contract: 'packages/schema-domain/src/screen-optional-visibility-runtime-settings.ts',
        tests: 'packages/screen-domain/tests/unit/screen-optional-visibility-runtime-settings.test.ts',
      },
      acceptedRows: {
        liveViewRevision: liveViewAccepted.state?.revision,
        liveViewMode: liveViewAccepted.state?.liveViewSetting.liveViewMode,
        rawRetentionRevision: rawState.state?.revision,
        rawRetentionMode: rawState.state?.rawRetentionSetting.mode,
      },
      rejectedRows: {
        staleRevision: staleRejected.rejectionReason,
        modeConflict: conflictRejected?.rejectionReason,
      },
      assertions,
      nonClaims: [
        'This proof does not start a live-view worker or enable product live view.',
        'This proof does not prove platform prompt screenshots, physical-device parity, hosted relay infrastructure, privacy/legal approval, or remote input.',
        'This proof does not mix live view with raw screenshot retention or raw screenshot remote upload.',
      ],
    },
    null,
    2
  )}\n`
);

console.log(`screen-optional-visibility-runtime-settings-proof-ok:${proofPath}`);

function runtimeRequest(overrides) {
  return screenEvidence.ScreenOptionalVisibilityRuntimeUpdateRequestSchema.parse({
    schemaVersion: screenEvidence.ScreenOptionalVisibilityRuntimeSettingsSchemaVersion,
    changedAt: generatedAt,
    ...overrides,
  });
}

function disabledRawRetention() {
  return screenEvidence.ScreenRawScreenshotRetentionOptInSettingSchema.parse({
    schemaVersion: screenEvidence.ScreenOptionalVisibilityModeSchemaVersion,
    settingId: 'screen-retention-disabled',
    parentSettingRef: 'screen-retention-parent-disabled',
    settingVersion: 1,
    changedAt: generatedAt,
    mode: 'disabled',
    explicitParentApproval: false,
    approvalRef: null,
    disclosureState: 'notRequired',
    auditRef: null,
    ttlSeconds: null,
    custodyState: 'unavailable',
    exportRef: null,
    sourceLabel: 'unavailable',
    retentionBehavior: 'noRawRetention',
    deleteAfterTtl: false,
    deleteOnParentDisable: true,
    deleteProofRequired: false,
    rawScreenshotRemoteUploadEnabled: false,
    reason: 'raw retention disabled',
  });
}

function localRawRetention() {
  return screenEvidence.ScreenRawScreenshotRetentionOptInSettingSchema.parse({
    ...disabledRawRetention(),
    settingId: 'screen-retention-local-runtime',
    parentSettingRef: 'screen-retention-parent-local-runtime',
    mode: 'localShortTtl',
    explicitParentApproval: true,
    approvalRef: 'screen-retention-approval-local-runtime',
    disclosureState: 'requiredShown',
    auditRef: 'screen-retention-audit-local-runtime',
    ttlSeconds: 120,
    custodyState: 'child-device-temp-queue',
    sourceLabel: 'rawScreenshotRetention',
    retentionBehavior: 'deleteAfterTtl',
    deleteAfterTtl: true,
    deleteProofRequired: true,
    reason: 'parent approved local short-TTL retention',
  });
}

function disabledLiveView() {
  return screenEvidence.ScreenLiveViewOptInSettingSchema.parse({
    schemaVersion: screenEvidence.ScreenOptionalVisibilityModeSchemaVersion,
    settingId: 'screen-live-view-disabled',
    parentSettingRef: 'screen-live-view-parent-disabled',
    settingVersion: 1,
    changedAt: generatedAt,
    liveViewMode: 'disabled',
    transportMode: 'none',
    explicitParentApproval: false,
    approvalRef: null,
    disclosureState: 'notRequired',
    viewerAuditRef: null,
    platformProofState: 'notRequired',
    platformProofRef: null,
    custodyState: 'unavailable',
    sourceLabel: 'unavailable',
    frameRetentionBehavior: 'noFrameRetention',
    cacheRawFrames: false,
    sessionRecordingAllowed: false,
    remoteInputControlAllowed: false,
    stopOrRevokeAuditRequired: true,
    reason: 'live view disabled',
  });
}

function lanLiveView() {
  return screenEvidence.ScreenLiveViewOptInSettingSchema.parse({
    ...disabledLiveView(),
    settingId: 'screen-live-view-lan-runtime',
    parentSettingRef: 'screen-live-view-parent-lan-runtime',
    liveViewMode: 'lanOnlyView',
    transportMode: 'lanMutualAuth',
    explicitParentApproval: true,
    approvalRef: 'screen-live-view-approval-lan-runtime',
    disclosureState: 'requiredShown',
    viewerAuditRef: 'screen-live-view-audit-lan-runtime',
    platformProofState: 'operatorVerified',
    platformProofRef: 'screen-live-view-platform-proof-lan-runtime',
    custodyState: 'live-lan-child-agent',
    sourceLabel: 'liveView',
    reason: 'parent approved LAN live view',
  });
}

function run(command, args) {
  const runner = process.platform === 'win32' ? 'cmd' : command;
  const runnerArgs = process.platform === 'win32' ? ['/c', command, ...args] : args;
  execFileSync(runner, runnerArgs, { cwd: repoRoot, stdio: 'inherit' });
}
