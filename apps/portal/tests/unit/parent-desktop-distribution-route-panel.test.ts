import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import {
  ParentBridgeConnectionState,
  ParentDesktopDistributionRuntime,
  ParentHostBridgeRuntime,
  ParentRoute,
  ParentRouteDataSource,
  ParentServiceHealthAuthenticationState,
  ParentServiceHealthReason,
  ParentServiceHealthState,
  decodeParentDesktopDistributionSnapshot,
  type ParentDesktopDistributionSnapshot,
  type ParentRouteSnapshot,
  type ParentServiceHealthReason as ParentServiceHealthReasonValue,
} from '../../generated/parent-ui-bridge';
import {
  createParentDesktopDistributionFixturePanelState,
  ParentDesktopDistributionRoutePanel,
  resolveParentDesktopDistributionPanelState,
  shouldRenderParentDesktopDistributionRoute,
} from '../../src/ParentDesktopDistributionRoutePanel';

describe('parent desktop distribution route panel', () => {
  registerRouteAndRuntimeTests();
  registerUnavailableAndProjectionTests();
});

function registerRouteAndRuntimeTests(): void {
  it('mounts only on the Platforms and Install Updates routes', () => {
    expect(shouldRenderParentDesktopDistributionRoute(ParentRoute.PlatformsInstall)).toBe(true);
    expect(shouldRenderParentDesktopDistributionRoute(ParentRoute.InstallUpdates)).toBe(true);
    expect(shouldRenderParentDesktopDistributionRoute(ParentRoute.Devices)).toBe(false);
  });

  it('renders decoded Rust-owned package status with explicit custody and no execution claim', () => {
    const markup = renderToStaticMarkup(
      createElement(ParentDesktopDistributionRoutePanel, {
        onNavigate: () => true,
        route: ParentRoute.PlatformsInstall,
        state: { kind: 'runtime', snapshot: distributionSnapshot() },
      })
    );

    expect(markup).toContain('aria-label="Platforms and install status"');
    expect(markup).toContain('data-ocentra-desktop-distribution-state="runtime"');
    expect(markup).toContain('data-ocentra-desktop-distribution-actions="unavailable"');
    expect(markup).toContain('Desktop package');
    expect(markup).toContain('Portal shell');
    expect(markup).toContain('built portal dist');
    expect(markup).toContain('Signing');
    expect(markup).toContain('signing manual required');
    expect(markup).toContain('Source and custody');
    expect(markup).toContain('rust parent runtime');
    expect(markup).toContain('source custody manual required');
    expect(markup).toContain('no installer updater rollback signing notarization store execution');
    expect(markup).toContain('Open Start Here');
    expect(markup).toContain('Review install updates');
  });

  it('renders typed update and rollback status without an updater claim', () => {
    const markup = renderToStaticMarkup(
      createElement(ParentDesktopDistributionRoutePanel, {
        onNavigate: () => true,
        route: ParentRoute.InstallUpdates,
        state: { kind: 'runtime', snapshot: distributionSnapshot() },
      })
    );

    expect(markup).toContain('aria-label="Install and update status"');
    expect(markup).toContain('Update channel');
    expect(markup).toContain('update channel scaffold');
    expect(markup).toContain('rollback unavailable');
    expect(markup).toContain('production promotion required');
    expect(markup).toContain(
      'Installer, updater, rollback, signing, notarization, and store execution remain unavailable'
    );
    expect(markup).toContain('Open Start Here');
    expect(markup).toContain('Review platform status');
  });
}

function registerUnavailableAndProjectionTests(): void {
  it('renders an explicit unavailable state when the typed snapshot is absent', () => {
    const markup = renderToStaticMarkup(
      createElement(ParentDesktopDistributionRoutePanel, {
        onNavigate: () => true,
        route: ParentRoute.PlatformsInstall,
        state: { kind: 'missing', snapshot: null },
      })
    );

    expect(markup).toContain('data-ocentra-desktop-distribution-state="missing"');
    expect(markup).toContain('Status unavailable');
    expect(markup).toContain('not reported');
    expect(markup).toContain('Platform boundary');
    expect(markup).toContain('Recovery path');
    expect(markup).toContain('connect in Start Here');
  });

  it('keeps fixtures and schema-invalid host responses visibly distinct', () => {
    const fixtureMarkup = renderToStaticMarkup(
      createElement(ParentDesktopDistributionRoutePanel, {
        onNavigate: () => true,
        route: ParentRoute.PlatformsInstall,
        state: createParentDesktopDistributionFixturePanelState(distributionSnapshot()),
      })
    );
    const invalidMarkup = renderToStaticMarkup(
      createElement(ParentDesktopDistributionRoutePanel, {
        onNavigate: () => true,
        route: ParentRoute.PlatformsInstall,
        state: { kind: 'invalid', snapshot: null },
      })
    );

    expect(fixtureMarkup).toContain('data-ocentra-desktop-distribution-state="fixture"');
    expect(fixtureMarkup).toContain('Demo fixture only — not runtime data');
    expect(invalidMarkup).toContain('data-ocentra-desktop-distribution-state="invalid"');
    expect(invalidMarkup).toContain('Invalid host status');
    expect(invalidMarkup).toContain('failed Rust-owned schema decoding');
  });

  it('derives runtime, invalid, and missing states from the decoded route snapshot', () => {
    expect(resolveParentDesktopDistributionPanelState(routeSnapshot(distributionSnapshot())).kind).toBe('runtime');
    expect(
      resolveParentDesktopDistributionPanelState(routeSnapshot(null, ParentServiceHealthReason.ResponseSchemaMismatch))
        .kind
    ).toBe('invalid');
    expect(resolveParentDesktopDistributionPanelState(null).kind).toBe('missing');
  });
}

function routeSnapshot(
  distribution: ParentDesktopDistributionSnapshot | null,
  reason: ParentServiceHealthReasonValue = ParentServiceHealthReason.Ready
): ParentRouteSnapshot {
  return {
    schemaVersion: ParentHostBridgeRuntime.SchemaVersion,
    route: ParentRoute.PlatformsInstall,
    generatedAt: '2026-08-30T10:00:00Z',
    seasonLabel: 'LOCAL',
    lastUpdated: '2026-08-30T10:00:00Z',
    connectionState: ParentBridgeConnectionState.Connected,
    commandEnabled: false,
    agentEndpoint: 'parent-local-bridge',
    dataSource: ParentRouteDataSource.HostBridge,
    summary: {
      title: 'Parent desktop distribution',
      routeCapability: 'read-only',
      parentAccess: 'available',
      household: 'not reported',
      childDevice: 'not reported',
    },
    serviceHealth: {
      state:
        reason === ParentServiceHealthReason.Ready
          ? ParentServiceHealthState.Ready
          : ParentServiceHealthState.Unavailable,
      authenticationState:
        reason === ParentServiceHealthReason.Ready
          ? ParentServiceHealthAuthenticationState.Authenticated
          : ParentServiceHealthAuthenticationState.Unavailable,
      reason,
      trace: {},
    },
    parentDesktopDistribution: distribution,
    diagnosticPanelsEnabled: false,
  };
}

function distributionSnapshot(): ParentDesktopDistributionSnapshot {
  return decodeParentDesktopDistributionSnapshot({
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
  });
}
