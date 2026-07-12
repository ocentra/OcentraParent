import type { ReactElement } from 'react';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/portal-domain/display-text';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalDetails } from '@ocentra-parent/portal-domain/details';
import {
  isParentScreenSummaryRoute,
  type ParentRouteId,
  type ParentScreenSummaryPanelDetailSnapshot,
  type ParentScreenSummaryPanelSnapshot,
} from '../generated/parent-ui-bridge';

export function shouldRenderScreenSummaryRoute(route: ParentRouteId): boolean {
  return isParentScreenSummaryRoute(route);
}

const EMPTY_SCREEN_SUMMARY_PANEL: ParentScreenSummaryPanelSnapshot = {
  eyebrow: 'Activity kind',
  title: 'Screen analysis',
  body: 'Stored activity',
  loadState: 'Unavailable',
  summaryDetails: [
    { label: PortalDetails.Status, value: 'Unavailable' },
    { label: PortalDetails.ProductClaim, value: 'No family setting is configured for this area yet.' },
  ],
  rows: [],
  emptyMessage: 'No recent activity is available yet.',
  productClaim: 'No family setting is configured for this area yet.',
};

export function ScreenSummaryRoutePanel({
  panel,
}: {
  readonly panel: ParentScreenSummaryPanelSnapshot | null;
}): ReactElement {
  const resolvedPanel = panel ?? EMPTY_SCREEN_SUMMARY_PANEL;
  return (
    <section
      aria-label={resolvePortalDevText(PortalDevTextToken.ScreenAnalysis)}
      className={PortalDom.Classes.TrackingStatusOverlay}
    >
      <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <p className={PortalDom.Classes.ProductEyebrow}>{resolvedPanel.eyebrow}</p>
          <h2>{resolvedPanel.title}</h2>
          <p>{resolvedPanel.body}</p>
        </header>
        <ScreenSummaryCards panel={resolvedPanel} />
      </div>
    </section>
  );
}

function ScreenSummaryCards({ panel }: { readonly panel: ParentScreenSummaryPanelSnapshot }): ReactElement {
  return (
    <div
      className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
        PortalDom.Classes.ClassNameSeparator
      )}
    >
      <ScreenSummaryCard title={PortalDetails.Status} details={panel.summaryDetails} />
      {panel.rows.length === 0 ? (
        <ScreenSummaryCard
          title={panel.emptyMessage}
          details={[
            { label: PortalDetails.Status, value: panel.loadState },
            { label: PortalDetails.ProductClaim, value: panel.productClaim },
          ]}
        />
      ) : null}
      {panel.rows.map((row) => (
        <ScreenSummaryCard key={String(row.title)} title={row.title} details={row.details} />
      ))}
    </div>
  );
}

function ScreenSummaryCard({
  details,
  title,
}: {
  readonly details: readonly ParentScreenSummaryPanelDetailSnapshot[];
  readonly title: string;
}): ReactElement {
  return (
    <article className={screenSummaryCardClassName()}>
      <h2>{title}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        {details.map((screenDetail) => (
          <ScreenSummaryDetail key={String(screenDetail.label)} label={screenDetail.label} value={screenDetail.value} />
        ))}
      </dl>
    </article>
  );
}

function ScreenSummaryDetail({ label, value }: { readonly label: string; readonly value: string }): ReactElement {
  return (
    <div>
      <dt>{label}</dt>
      <dd>{value}</dd>
    </div>
  );
}

function screenSummaryCardClassName(): string {
  return [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(PortalDom.Classes.ClassNameSeparator);
}
