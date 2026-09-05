import type { ReactElement } from 'react';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import {
  type ParentTrackingStatusPanelCardSnapshot,
  type ParentTrackingStatusPanelDetailSnapshot,
  type ParentTrackingStatusPanelSnapshot,
} from '../generated/parent-ui-bridge';

export function renderTrackingStatusRoutePanelBody(panel: ParentTrackingStatusPanelSnapshot): ReactElement {
  if (panel.summaryCards.length === 0 && panel.cards.length === 0) {
    return renderTrackingStatusRoutePanelCard(createTrackingStatusEmptyCard(panel));
  }
  const summaryCards = panel.summaryCards.map((card) => withoutRepeatedProductClaim(card, panel.productClaim));
  const detailCards = panel.cards.map((card) => withoutRepeatedProductClaim(card, panel.productClaim));
  detailCards.push(createTrackingStatusBoundaryCard(panel));
  return (
    <div className={PortalDom.Classes.TrackingStatusOverlayGrid}>
      {renderTrackingCardGroup('Tracking overview', summaryCards)}
      {renderTrackingCardGroup('Tracking details and availability', detailCards)}
    </div>
  );
}

function renderTrackingCardGroup(
  label: string,
  cards: readonly ParentTrackingStatusPanelCardSnapshot[]
): ReactElement | null {
  if (cards.length === 0) return null;
  return (
    <section aria-label={label} className={PortalDom.Classes.ProductDashboard}>
      {renderTrackingStatusCards(cards)}
    </section>
  );
}

function withoutRepeatedProductClaim(
  card: ParentTrackingStatusPanelCardSnapshot,
  productClaim: string
): ParentTrackingStatusPanelCardSnapshot {
  return {
    ...card,
    details: card.details.filter((detail) => detail.label !== 'Product claim' || detail.value !== productClaim),
  };
}

function createTrackingStatusBoundaryCard(
  panel: ParentTrackingStatusPanelSnapshot
): ParentTrackingStatusPanelCardSnapshot {
  return {
    key: 'tracking-product-boundary',
    title: 'Tracking boundary',
    details: [{ label: 'Product claim', value: panel.productClaim }],
  };
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
    <article
      key={card.key}
      className={trackingStatusCardClassName()}
      data-ocentra-tracking-card-state={trackingStatusCardState(card)}
    >
      <h2>{card.title}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        {renderTrackingStatusRoutePanelCardDetails(card.details)}
      </dl>
    </article>
  );
}

function trackingStatusCardState(card: ParentTrackingStatusPanelCardSnapshot): string {
  const status = card.details.find((detail) => detail.label === 'Status')?.value ?? 'reported';
  return status
    .trim()
    .toLocaleLowerCase('en-US')
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-|-$/g, '');
}

function renderTrackingStatusRoutePanelCardDetails(
  details: readonly ParentTrackingStatusPanelDetailSnapshot[]
): ReactElement[] {
  const renderedDetails: ReactElement[] = [];
  for (const [index, detail] of details.entries()) {
    renderedDetails.push(renderTrackingStatusRoutePanelCardDetail(detail, index));
  }
  return renderedDetails;
}

function renderTrackingStatusRoutePanelCardDetail(
  detail: ParentTrackingStatusPanelDetailSnapshot,
  index: number
): ReactElement {
  return (
    <div key={`${detail.label}:${index}`}>
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
