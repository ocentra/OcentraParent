import type { ReactElement } from 'react';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import {
  ParentRoute,
  ParentServiceHealthReason,
  type ParentDesktopDistributionSnapshot,
  type ParentRouteId,
  type ParentRouteSnapshot,
  parentRouteHashPath,
} from '../generated/parent-ui-bridge';

type DistributionDetail = Readonly<{
  label: string;
  state: string;
}>;

type DistributionCard = Readonly<{
  title: string;
  details: readonly DistributionDetail[];
}>;

const DISTRIBUTION_PANEL_COPY: Readonly<Record<ParentDesktopDistributionPanelState['kind'], string>> = {
  runtime:
    'Read-only Rust parent-runtime contract status, not a live installer or updater report. Installer, updater, rollback, signing, notarization, and store execution remain unavailable until their owners are composed.',
  fixture:
    'Demo fixture only — not runtime data and not a product-readiness claim. All distribution actions remain unavailable.',
  invalid:
    'The desktop host response failed Rust-owned schema decoding. No package, update, or execution state was accepted.',
  missing: 'The desktop host did not provide package or update status. Install and update actions remain unavailable.',
};

export type ParentDesktopDistributionPanelState =
  | Readonly<{ kind: 'runtime'; snapshot: ParentDesktopDistributionSnapshot }>
  | Readonly<{ kind: 'fixture'; snapshot: ParentDesktopDistributionSnapshot }>
  | Readonly<{ kind: 'invalid'; snapshot: null }>
  | Readonly<{ kind: 'missing'; snapshot: null }>;

export function shouldRenderParentDesktopDistributionRoute(route: ParentRouteId): boolean {
  return route === ParentRoute.PlatformsInstall || route === ParentRoute.InstallUpdates;
}

export function resolveParentDesktopDistributionPanelState(
  routeSnapshot: ParentRouteSnapshot | null
): ParentDesktopDistributionPanelState {
  if (routeSnapshot?.parentDesktopDistribution != null) {
    return { kind: 'runtime', snapshot: routeSnapshot.parentDesktopDistribution };
  }
  if (routeSnapshot?.serviceHealth?.reason === ParentServiceHealthReason.ResponseSchemaMismatch) {
    return { kind: 'invalid', snapshot: null };
  }
  return { kind: 'missing', snapshot: null };
}

export function createParentDesktopDistributionFixturePanelState(
  snapshot: ParentDesktopDistributionSnapshot
): ParentDesktopDistributionPanelState {
  return { kind: 'fixture', snapshot };
}

export function ParentDesktopDistributionRoutePanel({
  onNavigate,
  route,
  state,
}: {
  readonly onNavigate: (routePath: string) => boolean;
  readonly route: ParentRouteId;
  readonly state: ParentDesktopDistributionPanelState;
}): ReactElement | null {
  if (!shouldRenderParentDesktopDistributionRoute(route)) {
    return null;
  }
  const isPlatformsRoute = route === ParentRoute.PlatformsInstall;
  const title = isPlatformsRoute ? 'Platforms and install status' : 'Install and update status';
  const cards =
    state.kind === 'runtime' || state.kind === 'fixture'
      ? distributionCards(state.snapshot, isPlatformsRoute)
      : unavailableCards(state.kind, isPlatformsRoute);
  return (
    <section
      aria-label={title}
      className={[PortalDom.Classes.TrackingStatusOverlay, PortalDom.Classes.ParentDesktopDistributionRoutePanel].join(
        PortalDom.Classes.ClassNameSeparator
      )}
      data-ocentra-desktop-distribution-actions="unavailable"
      data-ocentra-desktop-distribution-state={state.kind}
    >
      <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <p className={PortalDom.Classes.ProductEyebrow}>Desktop distribution</p>
          <h1>{title}</h1>
          <p>{distributionPanelCopy(state.kind)}</p>
          <DistributionNavigation isPlatformsRoute={isPlatformsRoute} onNavigate={onNavigate} />
        </header>
        <div
          className={[PortalDom.Classes.TrackingStatusOverlayGrid, PortalDom.Classes.ProductDashboard].join(
            PortalDom.Classes.ClassNameSeparator
          )}
        >
          {cards.map((card) => (
            <article
              className={[PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
                PortalDom.Classes.ClassNameSeparator
              )}
              key={card.title}
            >
              <h2>{card.title}</h2>
              <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
                {card.details.map((detail) => (
                  <div key={detail.label}>
                    <dt>{detail.label}</dt>
                    <dd data-state={detail.state}>{humanizeDistributionState(detail.state)}</dd>
                  </div>
                ))}
              </dl>
            </article>
          ))}
        </div>
      </div>
    </section>
  );
}

function DistributionNavigation({
  isPlatformsRoute,
  onNavigate,
}: {
  readonly isPlatformsRoute: boolean;
  readonly onNavigate: (routePath: string) => boolean;
}): ReactElement {
  const destinations = isPlatformsRoute
    ? [
        { label: 'Open Start Here', route: ParentRoute.Start },
        { label: 'Review install updates', route: ParentRoute.InstallUpdates },
      ]
    : [
        { label: 'Open Start Here', route: ParentRoute.Start },
        { label: 'Review platform status', route: ParentRoute.PlatformsInstall },
      ];
  return (
    <nav aria-label="Distribution navigation" className={PortalDom.Classes.RouteTabs}>
      {destinations.map((destination) => (
        <button
          className={PortalDom.Classes.CommandResultTab}
          key={destination.route}
          onClick={() => {
            onNavigate(parentRouteHashPath(destination.route));
          }}
          type={PortalDom.ButtonType.Button}
        >
          {destination.label}
        </button>
      ))}
    </nav>
  );
}

function distributionCards(
  snapshot: ParentDesktopDistributionSnapshot,
  isPlatformsRoute: boolean
): readonly DistributionCard[] {
  return [...(isPlatformsRoute ? platformCards(snapshot) : updateCards(snapshot)), sourceCard(snapshot)];
}

function platformCards(snapshot: ParentDesktopDistributionSnapshot): readonly DistributionCard[] {
  return [
    {
      title: 'Desktop package',
      details: [
        { label: 'Portal shell', state: snapshot.packageFrontendState },
        { label: 'Service startup', state: snapshot.packageServiceManagerState },
        { label: 'Health check', state: snapshot.packageHealthProbeState },
      ],
    },
    {
      title: 'Release boundary',
      details: [
        { label: 'Package preview', state: snapshot.packagePreviewState },
        { label: 'Signing', state: snapshot.signingState },
        { label: 'Notarization', state: snapshot.notarizationState },
        { label: 'Store distribution', state: snapshot.storeDistributionState },
      ],
    },
    {
      title: 'Platform proof',
      details: [
        { label: 'Platform matrix', state: snapshot.platformMatrixState },
        { label: 'Artifact proof', state: snapshot.artifactProofState },
        { label: 'Actions', state: snapshot.actionsAvailable ? 'reported' : 'unavailable' },
      ],
    },
  ];
}

function updateCards(snapshot: ParentDesktopDistributionSnapshot): readonly DistributionCard[] {
  return [
    {
      title: 'Update channel',
      details: [
        { label: 'Channel', state: snapshot.updateChannelState },
        { label: 'Rollback', state: snapshot.rollbackState },
        { label: 'Release promotion', state: snapshot.releaseBranchState },
      ],
    },
    {
      title: 'Package lifecycle',
      details: [
        { label: 'Service startup', state: snapshot.packageServiceManagerState },
        { label: 'Health check', state: snapshot.packageHealthProbeState },
        { label: 'Package preview', state: snapshot.packagePreviewState },
      ],
    },
    {
      title: 'Release proof',
      details: [
        { label: 'Signing', state: snapshot.signingState },
        { label: 'Store distribution', state: snapshot.storeDistributionState },
        { label: 'Artifact proof', state: snapshot.artifactProofState },
        { label: 'Actions', state: snapshot.actionsAvailable ? 'reported' : 'unavailable' },
      ],
    },
  ];
}

function sourceCard(snapshot: ParentDesktopDistributionSnapshot): DistributionCard {
  return {
    title: 'Source and custody',
    details: [
      { label: 'Payload source', state: snapshot.payloadSource },
      { label: 'Source custody', state: snapshot.sourceCustodyState },
      { label: 'Product claim', state: snapshot.productClaimState },
      { label: 'Execution claim', state: snapshot.noClaim },
    ],
  };
}

function unavailableCards(kind: 'invalid' | 'missing', isPlatformsRoute: boolean): readonly DistributionCard[] {
  const status = kind === 'invalid' ? 'schema-mismatch' : 'not-reported';
  return [
    {
      title: kind === 'invalid' ? 'Invalid host status' : 'Status unavailable',
      details: [
        { label: 'Package status', state: status },
        { label: 'Update status', state: status },
        { label: 'Host snapshot', state: status },
      ],
    },
    {
      title: isPlatformsRoute ? 'Platform boundary' : 'Update boundary',
      details: [
        { label: 'Installer', state: 'unavailable' },
        { label: 'Updater', state: 'unavailable' },
        { label: 'Rollback', state: 'unavailable' },
        { label: 'Actions', state: 'unavailable' },
      ],
    },
    {
      title: 'Recovery path',
      details: [
        { label: 'Local service', state: 'connect in Start Here' },
        { label: 'Current data', state: 'required before actions' },
        { label: 'Control mode', state: 'read-only until reported' },
      ],
    },
  ];
}

function distributionPanelCopy(kind: ParentDesktopDistributionPanelState['kind']): string {
  return DISTRIBUTION_PANEL_COPY[kind];
}

function humanizeDistributionState(state: string): string {
  return state.replaceAll('-', ' ');
}
