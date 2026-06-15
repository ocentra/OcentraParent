import { describe, expect, it } from 'vitest';
import {
  ScreenLiveViewParentUiPersistenceProofSchema,
  ScreenLiveViewParentUiPersistenceSchemaVersion,
} from '../../src/screen-live-view-parent-ui-persistence';
import { ScreenLiveViewServiceSessionSchemaVersion } from '../../src/screen-live-view-service-session';
import { ScreenOptionalVisibilityModeSchemaVersion } from '../../src/screen-optional-visibility-mode-values';

const CheckedAt = '2026-06-07T09:35:00Z';

const LiveViewSetting = {
  schemaVersion: ScreenOptionalVisibilityModeSchemaVersion,
  settingId: 'screen-live-view-parent-ui-persistence-setting',
  parentSettingRef: 'screen-parent-live-view-setting',
  settingVersion: 7,
  changedAt: CheckedAt,
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
} as const;

const ServiceGate = {
  schemaVersion: ScreenLiveViewServiceSessionSchemaVersion,
  checkedAt: CheckedAt,
  liveViewMode: 'lanOnlyView',
  transportMode: 'lanMutualAuth',
  permissionEvidenceKind: 'screen-capture-only',
  sourceLabel: 'liveView',
  custodyState: 'live-lan-child-agent',
  frameRetentionBehavior: 'noFrameRetention',
  platformPermissionProofRef: 'screen-live-view-platform-permission-gate',
  viewerAuditRef: LiveViewSetting.viewerAuditRef,
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
} as const;

const Proof = {
  schemaVersion: ScreenLiveViewParentUiPersistenceSchemaVersion,
  checkedAt: CheckedAt,
  status: 'persistedParentOptIn',
  parentSettingRef: LiveViewSetting.parentSettingRef,
  liveViewSetting: LiveViewSetting,
  serviceSessionGate: ServiceGate,
  parentUiPersistenceState: 'proved',
  settingsRouteRendered: true,
  persistedInParentSettingsStore: true,
  viewerAuditRef: LiveViewSetting.viewerAuditRef,
  portalProofRef: 'optional-visibility-capability-status-portal-proof',
  serviceSettingsProofRef: 'settings-service-command-proof',
  rawFramesRetained: false,
  remoteInputAllowed: false,
  productLiveViewReady: false,
  reason: 'parent UI persistence can be carried into the service-session gate without product live-view readiness',
} as const;

describe('screen live-view parent UI persistence proof', () => {
  it('accepts a persisted parent opt-in that keeps product live view blocked', () => {
    const parsed = ScreenLiveViewParentUiPersistenceProofSchema.parse(Proof);

    expect(parsed.parentUiPersistenceState).toBe('proved');
    expect(parsed.serviceSessionGate.productLiveViewReady).toBe(false);
    expect(parsed.liveViewSetting.remoteInputControlAllowed).toBe(false);
  });

  it('rejects missing UI persistence, raw frame retention, remote input, or product overclaim', () => {
    const missingUiPersistence = ScreenLiveViewParentUiPersistenceProofSchema.safeParse({
      ...Proof,
      parentUiPersistenceState: 'missing',
    });
    const cachedFrames = ScreenLiveViewParentUiPersistenceProofSchema.safeParse({
      ...Proof,
      liveViewSetting: { ...LiveViewSetting, cacheRawFrames: true },
    });
    const remoteInput = ScreenLiveViewParentUiPersistenceProofSchema.safeParse({
      ...Proof,
      remoteInputAllowed: true,
    });
    const productOverclaim = ScreenLiveViewParentUiPersistenceProofSchema.safeParse({
      ...Proof,
      productLiveViewReady: true,
    });

    expect(missingUiPersistence.success).toBe(false);
    expect(cachedFrames.success).toBe(false);
    expect(remoteInput.success).toBe(false);
    expect(productOverclaim.success).toBe(false);
  });
});
