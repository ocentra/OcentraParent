import type { ReactElement } from 'react';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalDetails } from '@ocentra-parent/portal-domain/details';
import {
  createSocialDashboardPanelIntent,
  type SocialDashboardPanelDetail,
  type SocialDashboardPanelIntent,
  type SocialDashboardPanelRow,
} from '@ocentra-parent/portal-domain/social-dashboard-panel';
import { isPortalBrowserParentSurfaceRoute } from '@ocentra-parent/portal-domain/routes';
import { type PortalRoute as PortalRouteValue } from '@ocentra-parent/schema-domain/portal-contracts';
import type { PortalRenderActions } from './portal-actions';

export function shouldRenderSocialDashboardRoute(route: PortalRouteValue): boolean {
  return isPortalBrowserParentSurfaceRoute(route);
}

export function SocialDashboardRoutePanel({
  actions,
  commandEnabled,
  snapshot,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly snapshot: unknown | null;
}): ReactElement {
  const intent = createSocialDashboardPanelIntent(snapshot);
  return (
    <section aria-label={intent.title} className={PortalDom.Classes.TrackingStatusOverlay}>
      <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <p className={PortalDom.Classes.ProductEyebrow}>{intent.eyebrow}</p>
          <h2>{intent.title}</h2>
          <p>{intent.body}</p>
          <button
            className={PortalDom.Classes.CommandResultTab}
            disabled={!commandEnabled}
            type={PortalDom.ButtonType.Button}
            onClick={() => void actions.refreshRouteSnapshot?.()}
          >
            {intent.title}
          </button>
        </header>
        <div
          className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
            PortalDom.Classes.ClassNameSeparator
          )}
        >
          <SocialDashboardSummaryCard intent={intent} />
          {intent.rows.length === 0 ? (
            <SocialDashboardEmptyCard intent={intent} />
          ) : (
            intent.rows.map((row) => <SocialDashboardRowCard key={row.key} row={row} />)
          )}
        </div>
      </div>
    </section>
  );
}

function SocialDashboardSummaryCard({ intent }: { readonly intent: SocialDashboardPanelIntent }): ReactElement {
  return (
    <article className={cardClassName()}>
      <h2>{intent.summary}</h2>
      <SocialDashboardDetails details={intent.metrics} />
    </article>
  );
}

function SocialDashboardEmptyCard({ intent }: { readonly intent: SocialDashboardPanelIntent }): ReactElement {
  return (
    <article className={cardClassName()}>
      <h2>{intent.emptyMessage}</h2>
      <SocialDashboardDetails
        details={[
          { label: PortalDetails.Status, value: intent.state },
          { label: PortalDetails.ProductClaim, value: intent.productClaim },
        ]}
      />
    </article>
  );
}

function SocialDashboardRowCard({ row }: { readonly row: SocialDashboardPanelRow }): ReactElement {
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
  readonly details: readonly SocialDashboardPanelDetail[];
}): ReactElement {
  return (
    <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
      {details.map((detail) => (
        <div key={`${detail.label}:${detail.value}`}>
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
