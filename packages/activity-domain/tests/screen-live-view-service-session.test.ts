import { describe, expect, it } from 'vitest';
import {
  ScreenLiveViewServiceSessionGateSchema,
  ScreenLiveViewServiceSessionSchemaVersion,
} from '../src/screen-live-view-service-session';

const CheckedAt = '2026-06-07T08:44:00Z';

const DisabledSession = {
  schemaVersion: ScreenLiveViewServiceSessionSchemaVersion,
  checkedAt: CheckedAt,
  liveViewMode: 'disabled',
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
  reason: 'live view is disabled by default',
} as const;

const LoopbackTransportOnlySession = {
  ...DisabledSession,
  liveViewMode: 'lanOnlyView',
  transportMode: 'lanMutualAuth',
  permissionEvidenceKind: 'screen-capture-only',
  sourceLabel: 'liveView',
  custodyState: 'live-lan-child-agent',
  platformPermissionProofRef: 'screen-live-view-platform-permission-gate',
  viewerAuditRef: 'screen-live-view-loopback-viewer-audit',
  liveTransportProofRef: 'screen-live-view-loopback-transport-proof',
  serviceSessionState: 'loopbackTransportOnly',
  parentUiPersistenceState: 'missing',
  reason: 'loopback transport proof exists but production service session runtime is missing',
} as const;

const ProductionLanReadySession = {
  ...LoopbackTransportOnlySession,
  permissionEvidenceKind: 'live-view-permission',
  platformPermissionProofRef: 'screen-live-view-platform-prompt-proof',
  liveTransportProofRef: 'screen-live-view-service-session-proof',
  serviceSessionState: 'serviceRuntimeReady',
  parentUiPersistenceState: 'proved',
  productLiveViewReady: true,
  reason: 'LAN live view has platform prompt, viewer audit, service runtime, UI persistence, and transport proof',
} as const;

const ProductionRelayReadySession = {
  ...ProductionLanReadySession,
  liveViewMode: 'relayBackedView',
  transportMode: 'relayEndToEndEncrypted',
  sourceLabel: 'relay',
  custodyState: 'ocentra-hosted-non-activity',
  liveTransportProofRef: 'screen-live-view-relay-service-session-proof',
  relayCacheState: 'proved',
  reason: 'relay live view has prompt, viewer audit, service runtime, UI persistence, cache proof, and transport proof',
} as const;

describe('screen live-view service session gate', () => {
  it('accepts disabled, loopback-only, and fully ready production session shapes with exact readiness states', () => {
    const disabled = ScreenLiveViewServiceSessionGateSchema.parse(DisabledSession);
    const loopbackOnly = ScreenLiveViewServiceSessionGateSchema.parse(LoopbackTransportOnlySession);
    const lanReady = ScreenLiveViewServiceSessionGateSchema.parse(ProductionLanReadySession);
    const relayReady = ScreenLiveViewServiceSessionGateSchema.parse(ProductionRelayReadySession);

    expect(disabled.productLiveViewReady).toBe(false);
    expect(loopbackOnly.serviceSessionState).toBe('loopbackTransportOnly');
    expect(loopbackOnly.productLiveViewReady).toBe(false);
    expect(lanReady.productLiveViewReady).toBe(true);
    expect(lanReady.relayCacheState).toBe('notUsed');
    expect(relayReady.productLiveViewReady).toBe(true);
    expect(relayReady.relayCacheState).toBe('proved');
  });

  it('rejects product readiness from loopback transport, missing UI, missing prompt, recording, retention, or remote input', () => {
    const loopbackOverclaim = ScreenLiveViewServiceSessionGateSchema.safeParse({
      ...LoopbackTransportOnlySession,
      productLiveViewReady: true,
    });
    const missingPrompt = ScreenLiveViewServiceSessionGateSchema.safeParse({
      ...ProductionLanReadySession,
      permissionEvidenceKind: 'screen-capture-only',
    });
    const missingUi = ScreenLiveViewServiceSessionGateSchema.safeParse({
      ...ProductionLanReadySession,
      parentUiPersistenceState: 'missing',
    });
    const cachedFrames = ScreenLiveViewServiceSessionGateSchema.safeParse({
      ...ProductionLanReadySession,
      cacheRawFrames: true,
    });
    const recordedSession = ScreenLiveViewServiceSessionGateSchema.safeParse({
      ...ProductionLanReadySession,
      sessionRecordingAllowed: true,
    });
    const remoteInput = ScreenLiveViewServiceSessionGateSchema.safeParse({
      ...ProductionLanReadySession,
      remoteInputControlAllowed: true,
    });

    expect(loopbackOverclaim.success).toBe(false);
    expect(missingPrompt.success).toBe(false);
    expect(missingUi.success).toBe(false);
    expect(cachedFrames.success).toBe(false);
    expect(recordedSession.success).toBe(false);
    expect(remoteInput.success).toBe(false);
  });
});
