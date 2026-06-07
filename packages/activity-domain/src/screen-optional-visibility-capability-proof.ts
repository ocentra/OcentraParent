import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityTimestampSchema } from './primitives';
import { ScreenEvidenceReasonSchema } from './screen-evidence-primitives';
import {
  ScreenLiveViewPlatformPermissionGateSchema,
  type ScreenLiveViewPlatformPermissionGate,
} from './screen-live-view-platform-permission';
import {
  ScreenLiveViewOptInSettingSchema,
  ScreenRawScreenshotRetentionOptInSettingSchema,
  type ScreenLiveViewOptInSetting,
  type ScreenRawScreenshotRetentionOptInSetting,
} from './screen-optional-visibility-mode';
import { ScreenOptionalVisibilityPlatformProofRefSchema } from './screen-optional-visibility-mode-values';
import {
  ScreenOptionalVisibilityCapabilityStatusSchema,
  ScreenOptionalVisibilityCapabilityStatusSchemaVersion,
  type ScreenOptionalVisibilityCapabilityStatus,
} from './screen-optional-visibility-capability-status';

export const ScreenOptionalVisibilityCapabilityProofGeneratedAt = '2026-06-07T05:55:00Z';

const OptionalVisibilityProofRef = withParser(ScreenOptionalVisibilityPlatformProofRefSchema);

export const ScreenOptionalVisibilityCapabilityProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ScreenOptionalVisibilityCapabilityStatusSchemaVersion),
    generatedAt: ActivityTimestampSchema,
    proofId: Schema.Literal('screen-optional-visibility-capability-status-proof'),
    rows: Schema.Array(ScreenOptionalVisibilityCapabilityStatusSchema).pipe(
      Schema.filter((value) => value.length >= 5 || 'Expected disabled, manual-required, ready, and blocked rows')
    ),
    nonClaims: Schema.Array(ScreenEvidenceReasonSchema).pipe(
      Schema.filter((value) => value.length >= 3 || 'Expected explicit optional visibility non-claims')
    ),
  })
);

export function screenOptionalVisibilityCapabilityStatusProof(generatedAt: string) {
  const rows = [
    rawRetentionCapabilityRow(generatedAt, disabledRawRetention(generatedAt)),
    rawRetentionCapabilityRow(generatedAt, approvedRawRetentionWithoutRuntime(generatedAt)),
    rawRetentionCapabilityRow(generatedAt, approvedRawRetentionWithRuntimeProof(generatedAt), {
      runtimeProofRef: OptionalVisibilityProofRef.parse(
        'output/screen-plan-proof/screen-settings-service-command/proof-summary.json'
      ),
      deletionProofRef: OptionalVisibilityProofRef.parse(
        'output/screen-plan-proof/screen-service-deletion-event-producer/proof-summary.json'
      ),
      childDisclosureReady: true,
      childDeviceCapabilityReady: true,
      productModeReady: true,
    }),
    liveViewCapabilityRow(generatedAt, disabledLiveView(generatedAt), null),
    liveViewCapabilityRow(
      generatedAt,
      approvedLiveViewWithCaptureOnlyGate(generatedAt),
      captureOnlyLiveViewGate(generatedAt)
    ),
  ];

  return ScreenOptionalVisibilityCapabilityProofSchema.parse({
    schemaVersion: ScreenOptionalVisibilityCapabilityStatusSchemaVersion,
    generatedAt,
    proofId: 'screen-optional-visibility-capability-status-proof',
    rows,
    nonClaims: [
      'This proof proves raw screenshot retention readiness only after explicit parent approval, runtime proof, deletion proof, child disclosure readiness, and child device readiness.',
      'This proof does not enable live-view transport, relay, cache, or remote input.',
      'This proof does not satisfy privacy/legal approval or physical platform live-view prompt screenshots.',
    ],
  });
}

function rawRetentionCapabilityRow(
  checkedAt: string,
  setting: ScreenRawScreenshotRetentionOptInSetting,
  overrides: Partial<ScreenOptionalVisibilityCapabilityStatus> = {}
) {
  const hasRuntimeProof =
    overrides.runtimeProofRef !== undefined &&
    overrides.runtimeProofRef !== null &&
    overrides.deletionProofRef !== undefined &&
    overrides.deletionProofRef !== null;
  return ScreenOptionalVisibilityCapabilityStatusSchema.parse({
    ...baseRow(checkedAt, 'rawScreenshotRetention', setting.parentSettingRef),
    readinessState: setting.mode === 'disabled' ? 'disabled' : hasRuntimeProof ? 'ready' : 'manualRequired',
    rawRetentionSetting: setting,
    reason:
      setting.mode === 'disabled'
        ? 'raw screenshot retention is disabled by default'
        : hasRuntimeProof
          ? 'raw screenshot retention is ready only with parent approval, runtime proof, deletion proof, child disclosure, and child device readiness'
          : 'raw screenshot retention needs runtime and deletion proof before product readiness',
    ...overrides,
  });
}

function liveViewCapabilityRow(
  checkedAt: string,
  setting: ScreenLiveViewOptInSetting,
  gate: ScreenLiveViewPlatformPermissionGate | null,
  overrides: Partial<ScreenOptionalVisibilityCapabilityStatus> = {}
) {
  const ready = gate !== null && gate.productLiveViewReady && gate.liveTransportProofRef !== null;
  return ScreenOptionalVisibilityCapabilityStatusSchema.parse({
    ...baseRow(checkedAt, 'liveView', setting.parentSettingRef),
    readinessState: setting.liveViewMode === 'disabled' ? 'disabled' : ready ? 'ready' : 'blocked',
    liveViewSetting: setting,
    liveViewPermissionGate: gate,
    transportProofRef: gate?.liveTransportProofRef ?? null,
    reason:
      setting.liveViewMode === 'disabled'
        ? 'live view is disabled by default'
        : 'live view remains blocked until live-view permission and transport proof are present',
    ...overrides,
  });
}

function baseRow(checkedAt: string, capabilityKind: 'rawScreenshotRetention' | 'liveView', parentSettingRef: string) {
  return {
    schemaVersion: ScreenOptionalVisibilityCapabilityStatusSchemaVersion,
    checkedAt,
    capabilityKind,
    parentSettingRef,
    rawRetentionSetting: null,
    liveViewSetting: null,
    liveViewPermissionGate: null,
    runtimeProofRef: null,
    deletionProofRef: null,
    transportProofRef: null,
    childDisclosureReady: false,
    childDeviceCapabilityReady: false,
    productModeReady: false,
    rawFramesRetained: false,
    rawRemoteUploadAllowed: false,
    remoteInputAllowed: false,
  };
}

function disabledRawRetention(changedAt: string): ScreenRawScreenshotRetentionOptInSetting {
  return ScreenRawScreenshotRetentionOptInSettingSchema.parse({
    schemaVersion: ScreenOptionalVisibilityCapabilityStatusSchemaVersion,
    settingId: 'screen-retention-capability-disabled',
    parentSettingRef: 'screen-parent-retention-capability-disabled',
    settingVersion: 1,
    changedAt,
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
    reason: 'raw screenshot retention is disabled by default',
  });
}

function approvedRawRetentionWithoutRuntime(changedAt: string): ScreenRawScreenshotRetentionOptInSetting {
  return ScreenRawScreenshotRetentionOptInSettingSchema.parse({
    ...disabledRawRetention(changedAt),
    settingId: 'screen-retention-capability-local-ttl',
    parentSettingRef: 'screen-parent-retention-capability-local-ttl',
    mode: 'localShortTtl',
    explicitParentApproval: true,
    approvalRef: 'screen-retention-capability-approval',
    disclosureState: 'requiredShown',
    auditRef: 'screen-retention-capability-audit',
    ttlSeconds: 300,
    custodyState: 'child-device-temp-queue',
    sourceLabel: 'rawScreenshotRetention',
    retentionBehavior: 'deleteAfterTtl',
    deleteAfterTtl: true,
    deleteProofRequired: true,
    reason: 'parent approved local short TTL raw screenshot retention',
  });
}

function approvedRawRetentionWithRuntimeProof(changedAt: string): ScreenRawScreenshotRetentionOptInSetting {
  return ScreenRawScreenshotRetentionOptInSettingSchema.parse({
    ...approvedRawRetentionWithoutRuntime(changedAt),
    settingId: 'screen-retention-capability-local-ttl-runtime',
    parentSettingRef: 'screen-parent-retention-capability-local-ttl-runtime',
    settingVersion: 2,
    approvalRef: 'screen-retention-runtime-approval',
    auditRef: 'screen-retention-runtime-audit',
    ttlSeconds: 120,
    reason: 'parent approved local short TTL raw screenshot retention with runtime and deletion proof',
  });
}

function disabledLiveView(changedAt: string): ScreenLiveViewOptInSetting {
  return ScreenLiveViewOptInSettingSchema.parse({
    schemaVersion: ScreenOptionalVisibilityCapabilityStatusSchemaVersion,
    settingId: 'screen-live-capability-disabled',
    parentSettingRef: 'screen-parent-live-capability-disabled',
    settingVersion: 1,
    changedAt,
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
    reason: 'live view is disabled by default',
  });
}

function approvedLiveViewWithCaptureOnlyGate(changedAt: string): ScreenLiveViewOptInSetting {
  return ScreenLiveViewOptInSettingSchema.parse({
    ...disabledLiveView(changedAt),
    settingId: 'screen-live-capability-lan',
    parentSettingRef: 'screen-parent-live-capability-lan',
    liveViewMode: 'lanOnlyView',
    transportMode: 'lanMutualAuth',
    explicitParentApproval: true,
    approvalRef: 'screen-live-capability-approval',
    disclosureState: 'requiredShown',
    viewerAuditRef: 'screen-live-capability-audit',
    platformProofState: 'operatorVerified',
    platformProofRef: 'screen-live-capability-platform-proof',
    custodyState: 'live-lan-child-agent',
    sourceLabel: 'liveView',
    reason: 'parent approved LAN live view but capture-only evidence is insufficient',
  });
}

function captureOnlyLiveViewGate(checkedAt: string): ScreenLiveViewPlatformPermissionGate {
  return ScreenLiveViewPlatformPermissionGateSchema.parse({
    schemaVersion: ScreenOptionalVisibilityCapabilityStatusSchemaVersion,
    checkedAt,
    platform: 'android-mediaprojection',
    liveViewMode: 'lanOnlyView',
    transportMode: 'lanMutualAuth',
    permissionEvidenceKind: 'screen-capture-only',
    platformProofState: 'operatorVerified',
    platformProofRef: 'screen-live-capability-capture-only-proof',
    viewerAuditRef: 'screen-live-capability-audit',
    sourceLabel: 'liveView',
    custodyState: 'live-lan-child-agent',
    frameRetentionBehavior: 'noFrameRetention',
    liveTransportProofRef: null,
    explicitViewerDisclosure: true,
    cacheRawFrames: false,
    sessionRecordingAllowed: false,
    remoteInputControlAllowed: false,
    productLiveViewReady: false,
    reason: 'capture-only permission cannot satisfy live-view readiness',
  });
}

export type ScreenOptionalVisibilityCapabilityProof = Infer<typeof ScreenOptionalVisibilityCapabilityProofSchema>;
