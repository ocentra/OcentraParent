import { describe, expect, it } from 'vitest';
import {
  applyScreenOptionalVisibilityRuntimeSettingsRequest,
  createDisabledScreenOptionalVisibilityRuntimeSettingsState,
  ScreenLiveViewOptInSettingSchema,
  ScreenOptionalVisibilityModeSchemaVersion,
  ScreenOptionalVisibilityRuntimeSettingsSchemaVersion,
  ScreenOptionalVisibilityRuntimeUpdateRequestSchema,
  ScreenRawScreenshotRetentionOptInSettingSchema,
} from '../src/screen-evidence';

const ChangedAt = '2026-06-07T20:00:00Z';

describe('screen optional visibility runtime settings', () => {
  it('persists explicit live-view opt-in without product-live-view readiness', () => {
    const state = createDisabledScreenOptionalVisibilityRuntimeSettingsState({
      updatedAt: ChangedAt,
      rawRetentionSetting: disabledRawRetention(),
      liveViewSetting: disabledLiveView(),
      reason: 'optional visibility starts disabled',
    });
    const response = applyScreenOptionalVisibilityRuntimeSettingsRequest(
      state,
      ScreenOptionalVisibilityRuntimeUpdateRequestSchema.parse({
        schemaVersion: ScreenOptionalVisibilityRuntimeSettingsSchemaVersion,
        requestId: 'optional-visibility-request-1',
        kind: 'replaceLiveView',
        baseRevision: 1,
        rawRetentionSetting: null,
        liveViewSetting: lanLiveView(),
        changedAt: '2026-06-07T20:01:00Z',
        reason: 'parent approved LAN live view',
      })
    );

    expect(response.status).toBe('accepted');
    expect(response.state?.revision).toBe(2);
    expect(response.state?.liveViewSetting.liveViewMode).toBe('lanOnlyView');
    expect(response.state?.rawRetentionSetting.mode).toBe('disabled');
    expect(response.state?.rawScreenshotRemoteUploadEnabled).toBe(false);
    expect(response.state?.productLiveViewReady).toBe(false);
  });
});

describe('screen optional visibility runtime settings rejections', () => {
  it('rejects stale revisions and mixed raw-retention plus live-view custody', () => {
    const state = createDisabledScreenOptionalVisibilityRuntimeSettingsState({
      updatedAt: ChangedAt,
      rawRetentionSetting: localRawRetention(),
      liveViewSetting: disabledLiveView(),
      reason: 'raw retention approved separately',
    });
    const stale = applyScreenOptionalVisibilityRuntimeSettingsRequest(
      state,
      ScreenOptionalVisibilityRuntimeUpdateRequestSchema.parse({
        schemaVersion: ScreenOptionalVisibilityRuntimeSettingsSchemaVersion,
        requestId: 'optional-visibility-request-stale',
        kind: 'replaceLiveView',
        baseRevision: 999,
        rawRetentionSetting: null,
        liveViewSetting: lanLiveView(),
        changedAt: '2026-06-07T20:02:00Z',
        reason: 'stale live view request',
      })
    );
    const conflict = applyScreenOptionalVisibilityRuntimeSettingsRequest(
      state,
      ScreenOptionalVisibilityRuntimeUpdateRequestSchema.parse({
        schemaVersion: ScreenOptionalVisibilityRuntimeSettingsSchemaVersion,
        requestId: 'optional-visibility-request-conflict',
        kind: 'replaceLiveView',
        baseRevision: 1,
        rawRetentionSetting: null,
        liveViewSetting: lanLiveView(),
        changedAt: '2026-06-07T20:03:00Z',
        reason: 'live view cannot mix with raw retention',
      })
    );

    expect(stale.status).toBe('rejected');
    expect(stale.rejectionReason).toBe('stale-revision');
    expect(conflict.status).toBe('rejected');
    expect(conflict.rejectionReason).toBe('mode-conflict');
  });

  it('rejects unsafe live-view settings before runtime persistence', () => {
    const unsafeLiveView = ScreenLiveViewOptInSettingSchema.safeParse({
      ...lanLiveView(),
      cacheRawFrames: true,
    });
    const malformedRequest = ScreenOptionalVisibilityRuntimeUpdateRequestSchema.safeParse({
      schemaVersion: ScreenOptionalVisibilityRuntimeSettingsSchemaVersion,
      requestId: 'optional-visibility-request-malformed',
      kind: 'replaceLiveView',
      baseRevision: 1,
      rawRetentionSetting: localRawRetention(),
      liveViewSetting: lanLiveView(),
      changedAt: '2026-06-07T20:04:00Z',
      reason: 'malformed request carries two settings',
    });

    expect(unsafeLiveView.success).toBe(false);
    expect(malformedRequest.success).toBe(false);
  });
});

function disabledRawRetention() {
  return ScreenRawScreenshotRetentionOptInSettingSchema.parse({
    schemaVersion: ScreenOptionalVisibilityModeSchemaVersion,
    settingId: 'screen-retention-disabled',
    parentSettingRef: 'screen-retention-parent-disabled',
    settingVersion: 1,
    changedAt: ChangedAt,
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
  return ScreenRawScreenshotRetentionOptInSettingSchema.parse({
    ...disabledRawRetention(),
    settingId: 'screen-retention-local',
    parentSettingRef: 'screen-retention-parent-local',
    mode: 'localShortTtl',
    explicitParentApproval: true,
    approvalRef: 'screen-retention-approval-local',
    disclosureState: 'requiredShown',
    auditRef: 'screen-retention-audit-local',
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
  return ScreenLiveViewOptInSettingSchema.parse({
    schemaVersion: ScreenOptionalVisibilityModeSchemaVersion,
    settingId: 'screen-live-view-disabled',
    parentSettingRef: 'screen-live-view-parent-disabled',
    settingVersion: 1,
    changedAt: ChangedAt,
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
  return ScreenLiveViewOptInSettingSchema.parse({
    ...disabledLiveView(),
    settingId: 'screen-live-view-lan',
    parentSettingRef: 'screen-live-view-parent-lan',
    liveViewMode: 'lanOnlyView',
    transportMode: 'lanMutualAuth',
    explicitParentApproval: true,
    approvalRef: 'screen-live-view-approval-lan',
    disclosureState: 'requiredShown',
    viewerAuditRef: 'screen-live-view-audit-lan',
    platformProofState: 'operatorVerified',
    platformProofRef: 'screen-live-view-platform-proof-lan',
    custodyState: 'live-lan-child-agent',
    sourceLabel: 'liveView',
    reason: 'parent approved LAN live view',
  });
}
