import type { ReactElement } from 'react';
import {
  PortalDetails,
  PortalDom,
  PortalRoute,
  type PortalRoute as PortalRouteValue,
} from '@ocentra-parent/portal-domain/contracts';
import {
  createSocialDashboardPanelIntent,
  type SocialDashboardPanelDetail,
  type SocialDashboardPanelIntent,
  type SocialDashboardPanelRow,
} from './social-dashboard-panel';

export function shouldRenderSocialDashboardRoute(route: PortalRouteValue): boolean {
  return route === PortalRoute.Browser;
}

export function SocialDashboardRoutePanel({ snapshot }: { readonly snapshot: unknown }): ReactElement {
  const intent = createSocialDashboardPanelIntent(snapshot);
  return (
    <section aria-label={intent.title} className={PortalDom.Classes.TrackingStatusOverlay}>
      <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <p className={PortalDom.Classes.ProductEyebrow}>{intent.eyebrow}</p>
          <h2>{intent.title}</h2>
          <p>{intent.body}</p>
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
