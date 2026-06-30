import React, { type ReactElement } from 'react';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/schema-domain/text-portal-dev';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import {
  isParentTrackingStatusRoute,
  type ParentRouteId,
  type ParentTrackingStatusPanelCardSnapshot,
  type ParentTrackingStatusPanelDetailSnapshot,
  type ParentTrackingStatusPanelSnapshot,
} from '../generated/parent-ui-bridge';
import type { PortalLiveActivityState } from './live-activity-state';
import type { PortalRenderActions } from './portal-actions';

export function shouldRenderTrackingStatusRoute(route: ParentRouteId): boolean {
  return isParentTrackingStatusRoute(route);
}

export function TrackingStatusRoutePanel({
  actions,
  commandEnabled,
  liveActivity,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly liveActivity: PortalLiveActivityState;
}): ReactElement | null {
  const panel = liveActivity.activityTrackingPanel as ParentTrackingStatusPanelSnapshot | null;
  if (panel == null) {
    return null;
  }
  const cards = [...panel.summaryCards, ...panel.cards];
  return (
    <section
      aria-label={resolvePortalDevText(PortalDevTextToken.TrackingStatusSurface)}
      className={PortalDom.Classes.TrackingStatusOverlay}
    >
      <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <p className={PortalDom.Classes.ProductEyebrow}>{panel.eyebrow}</p>
          <h2>{panel.title}</h2>
          <p>{panel.body}</p>
          <button
            className={PortalDom.Classes.CommandResultTab}
            disabled={!commandEnabled}
            type={PortalDom.ButtonType.Button}
            onClick={() => void actions.refreshRouteSnapshot?.()}
          >
            {resolvePortalDevText(PortalDevTextToken.GetActivityTrackingReadModel)}
          </button>
        </header>
        <div
          className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
            PortalDom.Classes.ClassNameSeparator
          )}
        >
          {cards.length === 0 ? (
            <TrackingStatusCard
              card={{
                key: 'tracking-empty',
                title: panel.emptyMessage,
                details: [{ label: 'Product claim', value: panel.productClaim }],
              }}
            />
          ) : (
            cards.map((card) => <TrackingStatusCard key={card.key} card={card} />)
          )}
        </div>
      </div>
    </section>
  );
}

function TrackingStatusCard({
  card,
}: {
  readonly card: ParentTrackingStatusPanelCardSnapshot;
}): ReactElement {
  return (
    <article className={trackingStatusCardClassName()}>
      <h2>{card.title}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        {card.details.map((detail) => (
          <TrackingStatusDetail key={`${detail.label}:${detail.value}`} detail={detail} />
        ))}
      </dl>
    </article>
  );
}

function TrackingStatusDetail({
  detail,
}: {
  readonly detail: ParentTrackingStatusPanelDetailSnapshot;
}): ReactElement {
  return (
    <div>
      <dt>{detail.label}</dt>
      <dd>{detail.value}</dd>
    </div>
  );
}

function trackingStatusCardClassName(): string {
  return [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(PortalDom.Classes.ClassNameSeparator);
}
