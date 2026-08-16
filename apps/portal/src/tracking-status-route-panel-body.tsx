import type { ReactElement } from 'react';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import {
  type ParentTrackingStatusPanelCardSnapshot,
  type ParentTrackingStatusPanelDetailSnapshot,
  type ParentTrackingStatusPanelSnapshot,
} from '../generated/parent-ui-bridge';

export function renderTrackingStatusRoutePanelBody(panel: ParentTrackingStatusPanelSnapshot): ReactElement {
  const cards = [...panel.summaryCards, ...panel.cards];
  if (cards.length === 0) {
    return renderTrackingStatusRoutePanelCard(createTrackingStatusEmptyCard(panel));
  }
  return (
    <div
      className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
        PortalDom.Classes.ClassNameSeparator
      )}
    >
      {renderTrackingStatusCards(cards)}
    </div>
  );
}

function renderTrackingStatusCards(cards: readonly ParentTrackingStatusPanelCardSnapshot[]): ReactElement[] {
  const renderedCards: ReactElement[] = [];
  for (const card of cards) {
    renderedCards.push(renderTrackingStatusRoutePanelCard(card));
  }
  return renderedCards;
}

function renderTrackingStatusRoutePanelCard(card: ParentTrackingStatusPanelCardSnapshot): ReactElement {
  return (
    <article className={trackingStatusCardClassName()}>
      <h2>{card.title}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        {renderTrackingStatusRoutePanelCardDetails(card.details)}
      </dl>
    </article>
  );
}

function renderTrackingStatusRoutePanelCardDetails(
  details: readonly ParentTrackingStatusPanelDetailSnapshot[]
): ReactElement[] {
  const renderedDetails: ReactElement[] = [];
  for (const detail of details) {
    renderedDetails.push(renderTrackingStatusRoutePanelCardDetail(detail));
  }
  return renderedDetails;
}

function renderTrackingStatusRoutePanelCardDetail(detail: ParentTrackingStatusPanelDetailSnapshot): ReactElement {
  return (
    <div>
      <dt>{detail.label}</dt>
      <dd>{detail.value}</dd>
    </div>
  );
}

function createTrackingStatusEmptyCard(
  panel: ParentTrackingStatusPanelSnapshot
): ParentTrackingStatusPanelCardSnapshot {
  return {
    key: 'tracking-empty',
    title: panel.emptyMessage,
    details: [{ label: 'Product claim', value: panel.productClaim }],
  };
}

function trackingStatusCardClassName(): string {
  return [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(PortalDom.Classes.ClassNameSeparator);
}
