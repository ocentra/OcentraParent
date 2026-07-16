import type { ReactElement } from 'react';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import {
  isParentBrowserParentSurfaceRoute,
  type ParentBrowserPanelDetailSnapshot,
  type ParentBrowserPanelRowSnapshot,
  type ParentBrowserPanelSnapshot,
  type ParentRouteId,
} from '../generated/parent-ui-bridge';

export function shouldRenderBrowserParentExplanationRoute(route: ParentRouteId): boolean {
  return isParentBrowserParentSurfaceRoute(route);
}

export function BrowserParentExplanationRoutePanel({
  panel,
}: {
  readonly panel: ParentBrowserPanelSnapshot | null;
}): ReactElement {
  if (panel === null) {
    return (
      <section aria-label="Browser route unavailable" className={PortalDom.Classes.TrackingStatusOverlay}>
        <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
          <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
            <p className={PortalDom.Classes.ProductEyebrow}>Browser route</p>
            <h2>Browser route unavailable</h2>
            <p>Parent Rust snapshot unavailable for the browser parent explanation route.</p>
          </header>
        </div>
      </section>
    );
  }

  return (
    <section aria-label={panel.title} className={PortalDom.Classes.TrackingStatusOverlay}>
      <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <p className={PortalDom.Classes.ProductEyebrow}>{panel.eyebrow}</p>
          <h2>{panel.title}</h2>
          <p>{panel.body}</p>
        </header>
        <div
          className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
            PortalDom.Classes.ClassNameSeparator
          )}
        >
          <BrowserParentExplanationSummaryCard panel={panel} />
          {panel.rows.length === 0 ? (
            <BrowserParentExplanationEmptyCard panel={panel} />
          ) : (
            panel.rows.map((row) => <BrowserParentExplanationRowCard key={row.key} row={row} />)
          )}
        </div>
      </div>
    </section>
  );
}

function BrowserParentExplanationSummaryCard({ panel }: { readonly panel: ParentBrowserPanelSnapshot }): ReactElement {
  return (
    <article className={cardClassName()}>
      <h2>{panel.summary}</h2>
      <BrowserParentExplanationDetails details={panel.summaryDetails} />
    </article>
  );
}

function BrowserParentExplanationEmptyCard({ panel }: { readonly panel: ParentBrowserPanelSnapshot }): ReactElement {
  return (
    <article className={cardClassName()}>
      <h2>{panel.emptyMessage}</h2>
      <BrowserParentExplanationDetails details={panel.summaryDetails} />
    </article>
  );
}

function BrowserParentExplanationRowCard({ row }: { readonly row: ParentBrowserPanelRowSnapshot }): ReactElement {
  return (
    <article className={cardClassName()}>
      <h2>{row.title}</h2>
      <BrowserParentExplanationDetails details={row.details} />
    </article>
  );
}

function BrowserParentExplanationDetails({
  details,
}: {
  readonly details: readonly ParentBrowserPanelDetailSnapshot[];
}): ReactElement {
  return (
    <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
      {details.map((detail, index) => (
        <div key={`${detail.label}:${index}`}>
          <dt>{detail.label}</dt>
          <dd>{detail.value}</dd>
        </div>
      ))}
    </dl>
  );
}

function cardClassName(): string {
  return [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(PortalDom.Classes.ClassNameSeparator);
}
