import { describe, expect, it } from 'vitest';
import {
  ScreenLiveViewOptInSettingSchema,
  ScreenOptionalVisibilityModeSchemaVersion,
  ScreenRawScreenshotRetentionOptInSettingSchema,
} from '../src/screen-evidence';

const ChangedAt = '2026-06-05T03:44:00Z';

const DisabledRetention = {
  schemaVersion: ScreenOptionalVisibilityModeSchemaVersion,
  settingId: 'screen-retention-setting-disabled',
  parentSettingRef: 'screen-parent-setting-disabled',
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
  reason: 'raw screenshots are not retained by default',
} as const;

const LocalRetention = {
  ...DisabledRetention,
  settingId: 'screen-retention-setting-local-ttl',
  parentSettingRef: 'screen-parent-setting-local-ttl',
  mode: 'localShortTtl',
  explicitParentApproval: true,
  approvalRef: 'screen-retention-approval-local-ttl',
  disclosureState: 'requiredShown',
  auditRef: 'screen-retention-audit-local-ttl',
  ttlSeconds: 300,
  custodyState: 'child-device-temp-queue',
  sourceLabel: 'rawScreenshotRetention',
  retentionBehavior: 'deleteAfterTtl',
  deleteAfterTtl: true,
  deleteProofRequired: true,
  reason: 'parent approved short local TTL retention for support review',
} as const;

const ParentExportRetention = {
  ...LocalRetention,
  settingId: 'screen-retention-setting-parent-export',
  parentSettingRef: 'screen-parent-setting-parent-export',
  mode: 'parentOwnedExport',
  approvalRef: 'screen-retention-approval-parent-export',
  auditRef: 'screen-retention-audit-parent-export',
  custodyState: 'parent-owned-export',
  exportRef: 'screen-retention-export-parent-owned',
  retentionBehavior: 'parentOwnedExportDeleteOnRevoke',
  reason: 'parent approved parent-owned screenshot export with revoke/delete behavior',
} as const;

const DisabledLiveView = {
  schemaVersion: ScreenOptionalVisibilityModeSchemaVersion,
  settingId: 'screen-live-view-setting-disabled',
  parentSettingRef: 'screen-parent-setting-live-disabled',
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
  reason: 'live view is disabled by default',
} as const;

const LanLiveView = {
  ...DisabledLiveView,
  settingId: 'screen-live-view-setting-lan',
  parentSettingRef: 'screen-parent-setting-live-lan',
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
  reason: 'parent approved LAN-only view-only live screen after platform proof',
} as const;

const RelayLiveView = {
  ...LanLiveView,
  settingId: 'screen-live-view-setting-relay',
  parentSettingRef: 'screen-parent-setting-live-relay',
  liveViewMode: 'relayBackedView',
  transportMode: 'relayEndToEndEncrypted',
  approvalRef: 'screen-live-view-approval-relay',
  viewerAuditRef: 'screen-live-view-audit-relay',
  platformProofRef: 'screen-live-view-platform-proof-relay',
  custodyState: 'ocentra-hosted-non-activity',
  sourceLabel: 'relay',
  reason: 'parent approved relay-backed view-only live screen after platform proof',
} as const;

describe('screen optional visibility mode contracts', () => {
  specifyRawRetentionAcceptedModes();
  specifyRawRetentionRejections();
  specifyLiveViewAcceptedModes();
  specifyLiveViewRejections();
});

function specifyRawRetentionAcceptedModes() {
  it('accepts disabled, local short TTL, and parent-owned export raw screenshot retention settings', () => {
    const disabled = ScreenRawScreenshotRetentionOptInSettingSchema.parse(DisabledRetention);
    const local = ScreenRawScreenshotRetentionOptInSettingSchema.parse(LocalRetention);
    const exportMode = ScreenRawScreenshotRetentionOptInSettingSchema.parse(ParentExportRetention);

    expect(disabled.mode).toBe('disabled');
    expect(disabled.rawScreenshotRemoteUploadEnabled).toBe(false);
    expect(local.ttlSeconds).toBe(300);
    expect(local.deleteProofRequired).toBe(true);
    expect(exportMode.custodyState).toBe('parent-owned-export');
    expect(exportMode.exportRef).toBe('screen-retention-export-parent-owned');
  });
}

function specifyRawRetentionRejections() {
  it('rejects silent raw retention, missing TTL, missing delete proof, and remote screenshot upload', () => {
    const noApproval = ScreenRawScreenshotRetentionOptInSettingSchema.safeParse({
      ...LocalRetention,
      explicitParentApproval: false,
    });
    const missingTtl = ScreenRawScreenshotRetentionOptInSettingSchema.safeParse({
      ...LocalRetention,
      ttlSeconds: null,
    });
    const missingDeleteProof = ScreenRawScreenshotRetentionOptInSettingSchema.safeParse({
      ...LocalRetention,
      deleteProofRequired: false,
    });
    const remoteUpload = ScreenRawScreenshotRetentionOptInSettingSchema.safeParse({
      ...LocalRetention,
      rawScreenshotRemoteUploadEnabled: true,
    });

    expect(noApproval.success).toBe(false);
    expect(missingTtl.success).toBe(false);
    expect(missingDeleteProof.success).toBe(false);
    expect(remoteUpload.success).toBe(false);
  });
}

function specifyLiveViewAcceptedModes() {
  it('accepts disabled, LAN-only, and relay-backed view-only live view settings', () => {
    const disabled = ScreenLiveViewOptInSettingSchema.parse(DisabledLiveView);
    const lan = ScreenLiveViewOptInSettingSchema.parse(LanLiveView);
    const relay = ScreenLiveViewOptInSettingSchema.parse(RelayLiveView);

    expect(disabled.liveViewMode).toBe('disabled');
    expect(lan.transportMode).toBe('lanMutualAuth');
    expect(lan.remoteInputControlAllowed).toBe(false);
    expect(relay.transportMode).toBe('relayEndToEndEncrypted');
    expect(relay.custodyState).toBe('ocentra-hosted-non-activity');
  });
}

function specifyLiveViewRejections() {
  it('rejects live view without platform proof, cached frames, session recording, or remote input', () => {
    const missingPlatformProof = ScreenLiveViewOptInSettingSchema.safeParse({
      ...LanLiveView,
      platformProofState: 'missing',
    });
    const cachedFrames = ScreenLiveViewOptInSettingSchema.safeParse({
      ...LanLiveView,
      cacheRawFrames: true,
    });
    const sessionRecording = ScreenLiveViewOptInSettingSchema.safeParse({
      ...LanLiveView,
      sessionRecordingAllowed: true,
    });
    const remoteInput = ScreenLiveViewOptInSettingSchema.safeParse({
      ...LanLiveView,
      remoteInputControlAllowed: true,
    });

    expect(missingPlatformProof.success).toBe(false);
    expect(cachedFrames.success).toBe(false);
    expect(sessionRecording.success).toBe(false);
    expect(remoteInput.success).toBe(false);
  });
}
