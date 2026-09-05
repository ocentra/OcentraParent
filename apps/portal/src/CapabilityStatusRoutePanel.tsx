import type { ReactElement } from 'react';
import {
  CAPABILITY_STATUS_TEXT,
  buildCapabilityStatusRouteModel,
  capabilityStatusCardState,
  type CapabilityStatusCardModel,
} from '@ocentra-parent/portal-domain/capability-status-route-model';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/portal-domain/display-text';
import { ParentRoute, type ParentPortalShellStatusSnapshot, type ParentRouteId } from '../generated/parent-ui-bridge';
import type { PortalLiveActivityState } from './live-activity-state';
import type { PortalRenderActions } from './portal-actions';

export function shouldRenderCapabilityStatusRoute(route: ParentRouteId): boolean {
  return route === ParentRoute.CapabilityStatus;
}

export function CapabilityStatusRoutePanel({
  actions,
  commandEnabled,
  liveActivity,
  shellStatus,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly liveActivity: PortalLiveActivityState;
  readonly shellStatus: ParentPortalShellStatusSnapshot | null;
}): ReactElement {
  const { cards, reported } = buildCapabilityStatusRouteModel(shellStatus, liveActivity);
  const routeAction = capabilityStatusRouteAction(actions, commandEnabled);
  return (
    <section
      aria-label={CAPABILITY_STATUS_TEXT.ariaLabel}
      className={PortalDom.Classes.TrackingStatusOverlay}
      data-ocentra-capability-status-state={reported ? 'reported' : 'unavailable'}
      data-ocentra-capability-status-surface="product"
    >
      <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <p className={PortalDom.Classes.ProductEyebrow}>{CAPABILITY_STATUS_TEXT.eyebrow}</p>
          <h2>{reported ? CAPABILITY_STATUS_TEXT.title : CAPABILITY_STATUS_TEXT.unavailableTitle}</h2>
          <p>{reported ? CAPABILITY_STATUS_TEXT.body : CAPABILITY_STATUS_TEXT.unavailableBody}</p>
          <button
            className={PortalDom.Classes.CommandResultTab}
            onClick={routeAction.run}
            type={PortalDom.ButtonType.Button}
          >
            {routeAction.label}
          </button>
        </header>
        <div
          aria-label={CAPABILITY_STATUS_TEXT.capabilityDomains}
          className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
            PortalDom.Classes.ClassNameSeparator
          )}
        >
          {cards.map((card) => (
            <CapabilityStatusCard card={card} key={card.id} />
          ))}
        </div>
      </div>
    </section>
  );
}

function CapabilityStatusCard({ card }: { readonly card: CapabilityStatusCardModel }): ReactElement {
  return (
    <article
      className={capabilityStatusCardClassName()}
      data-ocentra-capability-card-state={capabilityStatusCardState(card.status)}
    >
      <h3>{card.title}</h3>
      <p>{card.reason}</p>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <div>
          <dt>{CAPABILITY_STATUS_TEXT.status}</dt>
          <dd>{card.status}</dd>
        </div>
        <div>
          <dt>{CAPABILITY_STATUS_TEXT.source}</dt>
          <dd>{card.source}</dd>
        </div>
      </dl>
    </article>
  );
}

function capabilityStatusRouteAction(
  actions: PortalRenderActions,
  commandEnabled: boolean
): { readonly label: string; readonly run: () => void } {
  if (!commandEnabled || actions.refreshRouteSnapshot === undefined) {
    return { label: resolvePortalDevText(PortalDevTextToken.RetryStatus), run: actions.reconnect };
  }
  return {
    label: CAPABILITY_STATUS_TEXT.refresh,
    run: () => {
      void actions.refreshRouteSnapshot?.();
    },
  };
}

function capabilityStatusCardClassName(): string {
  return [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(PortalDom.Classes.ClassNameSeparator);
}
