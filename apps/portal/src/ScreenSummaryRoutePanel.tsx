import type { ReactElement } from 'react';
import {
  PortalDetails,
  PortalDom,
  PortalRoute,
  PortalText,
  PortalTextToken,
  type PortalDisplayText,
  type PortalRoute as PortalRouteValue,
} from '@ocentra-parent/portal-domain/contracts';
import {
  createScreenSummaryPanelIntent,
  type ScreenSummaryPanelDetail,
  type ScreenSummaryPanelIntent,
} from '@ocentra-parent/portal-domain/screen-summary-panel';
import type { PortalLiveActivityState } from './live-activity-state';

export function shouldRenderScreenSummaryRoute(route: PortalRouteValue): boolean {
  return route === PortalRoute.ScreenAnalysis;
}

export function ScreenSummaryRoutePanel({
  liveActivity,
}: {
  readonly liveActivity: PortalLiveActivityState;
}): ReactElement {
  const intent = createScreenSummaryPanelIntent(liveActivity.activityScreenReadModel);
  return (
    <section
      aria-label={PortalText.Resolve(PortalTextToken.ScreenAnalysis)}
      className={PortalDom.Classes.TrackingStatusOverlay}
    >
      <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <p className={PortalDom.Classes.ProductEyebrow}>{intent.eyebrow}</p>
          <h2>{intent.title}</h2>
          <p>{intent.body}</p>
        </header>
        <ScreenSummaryCards intent={intent} />
      </div>
    </section>
  );
}

function ScreenSummaryCards({ intent }: { readonly intent: ScreenSummaryPanelIntent }): ReactElement {
  return (
    <div
      className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
        PortalDom.Classes.ClassNameSeparator
      )}
    >
      <ScreenSummaryCard title={PortalDetails.Status} details={intent.summaryDetails} />
      {intent.rows.length === 0 ? (
        <ScreenSummaryCard
          title={intent.emptyMessage}
          details={[
            { label: PortalDetails.Status, value: intent.loadState },
            { label: PortalDetails.ProductClaim, value: intent.productClaim },
          ]}
        />
      ) : null}
      {intent.rows.map((row) => (
        <ScreenSummaryCard key={String(row.title)} title={row.title} details={row.details} />
      ))}
    </div>
  );
}

function ScreenSummaryCard({
  details,
  title,
}: {
  readonly details: readonly ScreenSummaryPanelDetail[];
  readonly title: PortalDisplayText;
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

function ScreenSummaryDetail({
  label,
  value,
}: {
  readonly label: PortalDisplayText;
  readonly value: PortalDisplayText;
}): ReactElement {
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
