import type { ReactElement } from 'react';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalDetails } from '@ocentra-parent/portal-domain/details';
import {
  isParentAppGameParentSurfaceRoute,
  type ParentAppGameNotificationParentSurfacePanelRowSnapshot,
  type ParentAppGameNotificationParentSurfacePanelSnapshot,
  type ParentAppGamePanelDetailSnapshot,
  type ParentRouteId,
} from '../generated/parent-ui-bridge';

const EmptyNotificationParentSurfacePanel: ParentAppGameNotificationParentSurfacePanelSnapshot = {
  eyebrow: 'Rust-owned panel',
  title: 'App/game notification parent surface',
  body: 'Rust has not reported a notification parent-surface panel yet.',
  state: 'unavailable',
  summary: 'No notification rows reported',
  productClaim: 'Provider delivery, preference mutation, child delivery, and runtime dispatch remain unclaimed.',
  metrics: [
    { label: PortalDetails.Status, value: 'unavailable' },
    { label: 'Rows returned', value: '0' },
    { label: 'Runtime reference', value: 'service event not reported' },
  ],
  rows: [],
  emptyMessage: 'No app/game notification parent-surface panel has been reported yet.',
};

export function shouldRenderAppGameNotificationParentSurfaceRoute(route: ParentRouteId): boolean {
  return isParentAppGameParentSurfaceRoute(route);
}

export function AppGameNotificationParentSurfaceRoutePanel({
  panel,
}: {
  readonly panel: ParentAppGameNotificationParentSurfacePanelSnapshot | null;
}): ReactElement {
  const resolvedPanel = panel ?? EmptyNotificationParentSurfacePanel;
  return (
    <section aria-label={resolvedPanel.title} className={PortalDom.Classes.TrackingStatusOverlay}>
      <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <p className={PortalDom.Classes.ProductEyebrow}>{resolvedPanel.eyebrow}</p>
          <h2>{resolvedPanel.title}</h2>
          <p>{resolvedPanel.body}</p>
        </header>
        <div
          className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
            PortalDom.Classes.ClassNameSeparator
          )}
        >
          <AppGameNotificationParentSurfaceSummaryCard panel={resolvedPanel} />
          {resolvedPanel.rows.length === 0 ? (
            <AppGameNotificationParentSurfaceEmptyCard panel={resolvedPanel} />
          ) : (
            resolvedPanel.rows.map((row) => <AppGameNotificationParentSurfaceRowCard key={row.key} row={row} />)
          )}
        </div>
      </div>
    </section>
  );
}

function AppGameNotificationParentSurfaceSummaryCard({
  panel,
}: {
  readonly panel: ParentAppGameNotificationParentSurfacePanelSnapshot;
}): ReactElement {
  const summaryMetrics = panel.metrics.filter(
    (detail) => detail.label !== PortalDetails.Status && detail.label !== PortalDetails.ProductClaim
  );
  const details = [
    { label: PortalDetails.Status, value: panel.state },
    { label: PortalDetails.ProductClaim, value: panel.productClaim },
    ...summaryMetrics,
  ];
  return (
    <article className={cardClassName()}>
      <h2>{panel.summary}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        {details.map((detail, index) => (
          <AppGameNotificationParentSurfaceDetail
            key={`${String(index)}:${detail.label}:${detail.value}`}
            detail={detail}
          />
        ))}
      </dl>
    </article>
  );
}

function AppGameNotificationParentSurfaceEmptyCard({
  panel,
}: {
  readonly panel: ParentAppGameNotificationParentSurfacePanelSnapshot;
}): ReactElement {
  const runtimeReference =
    panel.metrics.find((detail) => detail.label === PortalDetails.RuntimeReference)?.value ?? panel.state;
  return (
    <article className={cardClassName()}>
      <h2>{panel.emptyMessage}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <AppGameNotificationParentSurfaceDetail
          detail={{ label: PortalDetails.RuntimeReference, value: runtimeReference }}
        />
        <AppGameNotificationParentSurfaceDetail
          detail={{ label: PortalDetails.ProductClaim, value: panel.productClaim }}
        />
      </dl>
    </article>
  );
}

function AppGameNotificationParentSurfaceRowCard({
  row,
}: {
  readonly row: ParentAppGameNotificationParentSurfacePanelRowSnapshot;
}): ReactElement {
  return (
    <article className={cardClassName()}>
      <h2>{row.title}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        {row.details.map((detail, index) => (
          <AppGameNotificationParentSurfaceDetail key={`${row.key}:${String(index)}:${detail.label}`} detail={detail} />
        ))}
      </dl>
    </article>
  );
}

function AppGameNotificationParentSurfaceDetail({
  detail,
}: {
  readonly detail: ParentAppGamePanelDetailSnapshot;
}): ReactElement {
  return (
    <div>
      <dt>{detail.label}</dt>
      <dd>{detail.value}</dd>
    </div>
  );
}

function cardClassName() {
  return [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(PortalDom.Classes.ClassNameSeparator);
}
