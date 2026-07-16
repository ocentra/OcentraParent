import type { ReactElement } from 'react';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import {
  isParentBrowserParentSurfaceRoute,
  type ParentBrowserPanelDetailSnapshot,
  type ParentBrowserPanelRowSnapshot,
  type ParentBrowserPanelSnapshot,
  type ParentRouteId,
} from '../generated/parent-ui-bridge';
import type { PortalRenderActions } from './portal-actions';

export function shouldRenderSocialDashboardRoute(route: ParentRouteId): boolean {
  return isParentBrowserParentSurfaceRoute(route);
}

export function SocialDashboardRoutePanel({
  actions,
  commandEnabled,
  panel,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly panel: ParentBrowserPanelSnapshot | null;
}): ReactElement {
  if (panel === null) {
    return (
      <section aria-label="Social dashboard unavailable" className={PortalDom.Classes.TrackingStatusOverlay}>
        <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
          <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
            <p className={PortalDom.Classes.ProductEyebrow}>Browser route</p>
            <h2>Social dashboard unavailable</h2>
            <p>Parent Rust snapshot unavailable for the social dashboard route.</p>
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
          <button
            className={PortalDom.Classes.CommandResultTab}
            disabled={!commandEnabled}
            type={PortalDom.ButtonType.Button}
            onClick={() => void actions.refreshRouteSnapshot?.()}
          >
            {panel.title}
          </button>
        </header>
        <div
          className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
            PortalDom.Classes.ClassNameSeparator
          )}
        >
          <SocialDashboardSummaryCard panel={panel} />
          {panel.rows.length === 0 ? (
            <SocialDashboardEmptyCard panel={panel} />
          ) : (
            panel.rows.map((row) => <SocialDashboardRowCard key={row.key} row={row} />)
          )}
        </div>
      </div>
    </section>
  );
}

function SocialDashboardSummaryCard({ panel }: { readonly panel: ParentBrowserPanelSnapshot }): ReactElement {
  return (
    <article className={cardClassName()}>
      <h2>{panel.summary}</h2>
      <SocialDashboardDetails details={panel.summaryDetails} />
    </article>
  );
}

function SocialDashboardEmptyCard({ panel }: { readonly panel: ParentBrowserPanelSnapshot }): ReactElement {
  return (
    <article className={cardClassName()}>
      <h2>{panel.emptyMessage}</h2>
      <SocialDashboardDetails details={panel.summaryDetails} />
    </article>
  );
}

function SocialDashboardRowCard({ row }: { readonly row: ParentBrowserPanelRowSnapshot }): ReactElement {
  return (
    <article className={cardClassName()}>
      <h2>{row.title}</h2>
      <SocialDashboardDetails details={row.details} />
    </article>
  );
}

function SocialDashboardDetails({
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
