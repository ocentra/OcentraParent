import { describe, expect, it } from 'vitest';
import {
  ParentBridgeConnectionState,
  ParentDesktopDistributionRuntime,
  ParentHostBridgeRuntime,
  ParentRoute,
  ParentRouteDataSource,
  decodeParentBridgeUnsubscribeResult,
  decodeParentDesktopDistributionSnapshot,
  decodeParentRouteSnapshot,
  decodeParentRouteSnapshotForRoute,
  decodeParentRouteSubscriptionId,
  decodeParentSubscriptionEvent,
  decodeParentTrackingStatusPanelSnapshot,
  decodeParentUiActionResult,
  isParentAiRuntimeRoute,
  isParentAppGameParentSurfaceRoute,
  isParentBrowserParentSurfaceRoute,
  isParentPolicyPreviewRoute,
  isParentScreenSettingsRoute,
  isParentTrackingStatusRoute,
  parentRouteFromHashPath,
  parentRouteHashPath,
} from '../../generated/parent-ui-bridge';

describe('Rust-generated parent UI bridge interface', () => {
  it('owns route parsing and product route-family predicates', () => {
    expect(parentRouteFromHashPath(parentRouteHashPath(ParentRoute.Overview))).toBe(ParentRoute.Overview);
    expect(parentRouteFromHashPath('#/app-layout')).toBe(ParentRoute.AppLayout);
    expect(parentRouteFromHashPath('#/frame-tuner?panel=layout')).toBe(ParentRoute.FrameTuner);
    expect(parentRouteFromHashPath('#/not-a-parent-route')).toBeNull();
    expect(isParentAiRuntimeRoute(ParentRoute.AiRuntime)).toBe(true);
    expect(isParentAppGameParentSurfaceRoute(ParentRoute.AppGameSessions)).toBe(true);
    expect(isParentBrowserParentSurfaceRoute(ParentRoute.Browser)).toBe(true);
    expect(isParentBrowserParentSurfaceRoute(ParentRoute.ProofPanels)).toBe(false);
    expect(isParentPolicyPreviewRoute(ParentRoute.RuleManagement)).toBe(true);
    expect(isParentScreenSettingsRoute(ParentRoute.PolicyScreen)).toBe(true);
    expect(isParentScreenSettingsRoute(ParentRoute.SettingsRules)).toBe(false);
    expect(isParentTrackingStatusRoute(ParentRoute.PolicyTracking)).toBe(true);
    expect(isParentTrackingStatusRoute(ParentRoute.Overview)).toBe(false);
  });

  it('decodes a route snapshot and rejects missing, unknown, cross-route, and misplaced state', () => {
    const snapshot = validRouteSnapshot(ParentRoute.Overview);

    expect(decodeParentRouteSnapshot(snapshot).route).toBe(ParentRoute.Overview);
    expect(decodeParentRouteSnapshotForRoute(snapshot, ParentRoute.Overview).dataSource).toBe(
      ParentRouteDataSource.HostBridge
    );
    expect(() => decodeParentRouteSnapshot({ ...snapshot, summary: undefined })).toThrow(
      'parent route snapshot does not match the Rust-owned contract'
    );
    expect(() => decodeParentRouteSnapshot({ ...snapshot, portalOwnedTruth: true })).toThrow(
      'parent route snapshot does not match the Rust-owned contract'
    );
    expect(() => decodeParentRouteSnapshot({ ...snapshot, schemaVersion: 0 })).toThrow(
      'parent route snapshot does not match the Rust-owned contract'
    );
    expect(() => decodeParentRouteSnapshotForRoute(snapshot, ParentRoute.Devices)).toThrow(
      'parent route snapshot does not match the requested route'
    );
    expect(() =>
      decodeParentRouteSnapshot({
        ...snapshot,
        parentDesktopDistribution: validDistributionSnapshot(),
      })
    ).toThrow('parent route snapshot does not match the Rust-owned contract');
  });

  it('requires exact read-only desktop distribution literals', () => {
    const distribution = validDistributionSnapshot();
    const decoded = decodeParentDesktopDistributionSnapshot(distribution);

    expect(decoded.payloadSource).toBe(ParentDesktopDistributionRuntime.PayloadSource);
    expect(decoded.sourceCustodyState).toBe(ParentDesktopDistributionRuntime.SourceCustodyState);
    expect(decoded.productClaimState).toBe(ParentDesktopDistributionRuntime.ProductClaimState);
    expect(decoded.noClaim).toBe(ParentDesktopDistributionRuntime.NoClaim);
    expect(decoded.actionsAvailable).toBe(false);
    expect(() => decodeParentDesktopDistributionSnapshot({ ...distribution, actionsAvailable: true })).toThrow(
      'parent desktop distribution payload does not match the Rust-owned contract'
    );
    expect(() => decodeParentDesktopDistributionSnapshot({ ...distribution, payloadSource: 'portal-fixture' })).toThrow(
      'parent desktop distribution payload does not match the Rust-owned contract'
    );
    expect(() => decodeParentDesktopDistributionSnapshot({ ...distribution, installerReady: true })).toThrow(
      'parent desktop distribution payload does not match the Rust-owned contract'
    );
  });
});

describe('Rust-generated parent UI bridge nested contracts', () => {
  it('validates nested service rows instead of trusting top-level snapshot shape', () => {
    const snapshot = validRouteSnapshot(ParentRoute.Overview);
    const row = {
      label: 'Local service',
      order: 1,
      signalScore: 1,
      readyCount: 1,
      gapCount: 0,
      primaryArea: 'Runtime',
      trend: 'current',
      tone: 'cyan',
    };

    expect(decodeParentRouteSnapshot({ ...snapshot, parentPortalRows: [row] }).parentPortalRows).toEqual([row]);
    expect(() => decodeParentRouteSnapshot({ ...snapshot, parentPortalRows: [{ ...row, readyCount: -1 }] })).toThrow(
      'parent route snapshot does not match the Rust-owned contract'
    );
    expect(() =>
      decodeParentRouteSnapshot({ ...snapshot, parentPortalRows: [{ ...row, portalAuthority: true }] })
    ).toThrow('parent route snapshot does not match the Rust-owned contract');
  });

  it('decodes complete tracking panels and rejects malformed nested or unknown state', () => {
    const panel = validTrackingPanelSnapshot();

    expect(decodeParentTrackingStatusPanelSnapshot(panel).title).toBe('Tracking status');
    expect(() => decodeParentTrackingStatusPanelSnapshot({ ...panel, summaryCards: undefined })).toThrow(
      'parent tracking status panel does not match the Rust-owned contract'
    );
    expect(() => decodeParentTrackingStatusPanelSnapshot({ ...panel, portalAuthority: true })).toThrow(
      'parent tracking status panel does not match the Rust-owned contract'
    );
    expect(() =>
      decodeParentTrackingStatusPanelSnapshot({
        ...panel,
        cards: [{ key: 'citation-1', title: 'Citation', details: [{ label: 'Status', value: false }] }],
      })
    ).toThrow('parent tracking status panel does not match the Rust-owned contract');
  });

  it('decodes action and subscription results while rejecting malformed host payloads', () => {
    const snapshot = validRouteSnapshot(ParentRoute.Devices);
    const actionResult = {
      schemaVersion: ParentHostBridgeRuntime.SchemaVersion,
      accepted: false,
      connectionState: ParentBridgeConnectionState.Connected,
      message: 'read-only',
      snapshot: null,
      events: [],
    };
    const subscriptionEvent = {
      schemaVersion: ParentHostBridgeRuntime.SchemaVersion,
      route: ParentRoute.Devices,
      snapshot,
      events: [],
    };

    expect(decodeParentUiActionResult(actionResult).accepted).toBe(false);
    expect(() => decodeParentUiActionResult({ ...actionResult, accepted: 'yes' })).toThrow(
      'parent UI action result does not match the Rust-owned contract'
    );
    expect(decodeParentSubscriptionEvent(subscriptionEvent).route).toBe(ParentRoute.Devices);
    expect(() => decodeParentSubscriptionEvent({ ...subscriptionEvent, route: ParentRoute.Overview })).toThrow(
      'parent subscription event does not match the Rust-owned contract'
    );
    expect(decodeParentRouteSubscriptionId('subscription-1')).toBe('subscription-1');
    expect(() => decodeParentRouteSubscriptionId('   ')).toThrow(
      'parent route subscription id must be a non-empty string'
    );
    expect(decodeParentBridgeUnsubscribeResult(true)).toBe(true);
    expect(() => decodeParentBridgeUnsubscribeResult('true')).toThrow(
      'parent route unsubscribe result must be a boolean'
    );
  });
});

function validRouteSnapshot(route: string): Readonly<Record<string, unknown>> {
  return {
    schemaVersion: ParentHostBridgeRuntime.SchemaVersion,
    route,
    generatedAt: '2026-08-30T10:00:00Z',
    seasonLabel: 'LOCAL',
    lastUpdated: '2026-08-30T10:00:00Z',
    connectionState: ParentBridgeConnectionState.Connected,
    commandEnabled: false,
    agentEndpoint: 'parent-local-bridge',
    dataSource: ParentRouteDataSource.HostBridge,
    summary: {
      title: 'Parent route',
      routeCapability: 'read-only',
      parentAccess: 'available',
      household: 'not reported',
      childDevice: 'not reported',
    },
    serviceHealth: null,
    parentDesktopDistribution: null,
    diagnosticPanelsEnabled: false,
    parentPortalRows: null,
    parentPortalShellStatus: null,
    liveActivity: null,
    browserPanels: null,
    setupFirstRunPanel: null,
    screenSettingsServiceResponse: null,
  };
}

function validDistributionSnapshot(): Readonly<Record<string, unknown>> {
  return {
    payloadSource: ParentDesktopDistributionRuntime.PayloadSource,
    sourceCustodyState: ParentDesktopDistributionRuntime.SourceCustodyState,
    productClaimState: ParentDesktopDistributionRuntime.ProductClaimState,
    noClaim: ParentDesktopDistributionRuntime.NoClaim,
    packageFrontendState: ParentDesktopDistributionRuntime.PackageFrontendState,
    packageServiceManagerState: ParentDesktopDistributionRuntime.PackageServiceManagerState,
    packageHealthProbeState: ParentDesktopDistributionRuntime.PackageHealthProbeState,
    packagePreviewState: ParentDesktopDistributionRuntime.PackagePreviewState,
    updateChannelState: ParentDesktopDistributionRuntime.UpdateChannelState,
    rollbackState: ParentDesktopDistributionRuntime.RollbackState,
    signingState: ParentDesktopDistributionRuntime.SigningState,
    notarizationState: ParentDesktopDistributionRuntime.NotarizationState,
    storeDistributionState: ParentDesktopDistributionRuntime.StoreDistributionState,
    platformMatrixState: ParentDesktopDistributionRuntime.PlatformMatrixState,
    releaseBranchState: ParentDesktopDistributionRuntime.ReleaseBranchState,
    artifactProofState: ParentDesktopDistributionRuntime.ArtifactProofState,
    actionsAvailable: ParentDesktopDistributionRuntime.ActionsAvailable,
  };
}

function validTrackingPanelSnapshot(): Readonly<Record<string, unknown>> {
  return {
    eyebrow: 'First target',
    title: 'Tracking status',
    body: 'Rust-backed tracking status.',
    summaryCards: [
      {
        key: 'tracking-live-summary',
        title: 'Tracking live summary',
        details: [{ label: 'Status', value: 'Available' }],
      },
    ],
    cards: [],
    emptyMessage: 'No tracking activity is available yet.',
    productClaim: 'Service read model only.',
  };
}
