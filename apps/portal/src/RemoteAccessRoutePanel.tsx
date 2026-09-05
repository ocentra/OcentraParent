import type { ReactElement } from 'react';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { ParentRoute, parentRouteHashPath, type ParentRouteId } from '../generated/parent-ui-bridge';

const REMOTE_ACCESS_DESTINATIONS = [
  { label: 'Open Start Here', route: ParentRoute.Start },
  { label: 'Open devices', route: ParentRoute.Devices },
  { label: 'Review remote screen policy', route: ParentRoute.PolicyRemoteScreen },
] as const;

const REMOTE_ACCESS_STATUS_CARDS = [
  {
    label: 'Remote session',
    value: 'Not reported',
    body: 'No owner-backed live-view session or child permission state is connected.',
  },
  {
    label: 'Trusted target',
    value: 'Not reported',
    body: 'No current device, route, reachability, or selected-target proof has been reported.',
  },
  {
    label: 'Control authority',
    value: 'Manual required',
    body: 'Remote actions stay locked until the service supplies current parent authority and transport custody.',
  },
] as const;

export function shouldRenderRemoteAccessRoute(route: ParentRouteId): boolean {
  return route === ParentRoute.RemoteAccess;
}

export function RemoteAccessRoutePanel({
  onNavigate,
}: {
  readonly onNavigate: (routePath: string) => boolean | void;
}): ReactElement {
  return (
    <section
      aria-label="Remote access unavailable"
      className={PortalDom.Classes.TrackingStatusOverlay}
      data-ocentra-remote-access-surface="product"
      data-ocentra-remote-access-state="manual-required"
    >
      <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <p className={PortalDom.Classes.ProductEyebrow}>Remote access</p>
          <h2>Remote access unavailable</h2>
          <p>
            No owner-backed remote session, trusted target, transport route, or current authority is connected. Choose a
            real recovery or review surface without creating a local command draft.
          </p>
          <nav aria-label="Remote access recovery" className={PortalDom.Classes.RouteTabs}>
            {REMOTE_ACCESS_DESTINATIONS.map((destination) => (
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
        </header>
        <div
          className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
            PortalDom.Classes.ClassNameSeparator
          )}
        >
          {REMOTE_ACCESS_STATUS_CARDS.map((card) => (
            <article
              className={[PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
                PortalDom.Classes.ClassNameSeparator
              )}
              key={card.label}
            >
              <p className={PortalDom.Classes.ProductEyebrow}>{card.label}</p>
              <h3>{card.value}</h3>
              <p>{card.body}</p>
            </article>
          ))}
        </div>
      </div>
    </section>
  );
}
