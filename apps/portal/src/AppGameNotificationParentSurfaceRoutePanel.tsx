import type { ReactElement } from 'react';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalDetails } from '@ocentra-parent/portal-domain/details';
import {
  createAppGameNotificationParentSurfacePanelIntent,
  type AppGameNotificationParentSurfaceDetail,
  type AppGameNotificationParentSurfacePanelIntent,
  type AppGameNotificationParentSurfacePanelRow,
} from '@ocentra-parent/portal-domain/app-game-notification-parent-surface-panel';
import {
  isPortalAppGameParentSurfaceRoute,
  type PortalRoute as PortalRouteValue,
} from '@ocentra-parent/portal-domain/routes';

export function shouldRenderAppGameNotificationParentSurfaceRoute(route: PortalRouteValue): boolean {
  return isPortalAppGameParentSurfaceRoute(route);
}

export function AppGameNotificationParentSurfaceRoutePanel({
  readModel,
}: {
  readonly readModel: unknown;
}): ReactElement {
  const intent = createAppGameNotificationParentSurfacePanelIntent(readModel);
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
          <AppGameNotificationParentSurfaceSummaryCard intent={intent} />
          {intent.rows.length === 0 ? (
            <AppGameNotificationParentSurfaceEmptyCard intent={intent} />
          ) : (
            intent.rows.map((row) => <AppGameNotificationParentSurfaceRowCard key={row.key} row={row} />)
          )}
        </div>
      </div>
    </section>
  );
}

function AppGameNotificationParentSurfaceSummaryCard({
  intent,
}: {
  readonly intent: AppGameNotificationParentSurfacePanelIntent;
}): ReactElement {
  const details = [
    { label: PortalDetails.Status, value: intent.state },
    { label: PortalDetails.ProductClaim, value: intent.productClaim },
    ...intent.metrics,
  ];
  return (
    <article className={cardClassName()}>
      <h2>{intent.summary}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        {details.map((detail) => (
          <AppGameNotificationParentSurfaceDetail key={`${detail.label}:${detail.value}`} detail={detail} />
        ))}
      </dl>
    </article>
  );
}

function AppGameNotificationParentSurfaceEmptyCard({
  intent,
}: {
  readonly intent: AppGameNotificationParentSurfacePanelIntent;
}): ReactElement {
  return (
    <article className={cardClassName()}>
      <h2>{intent.emptyMessage}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <AppGameNotificationParentSurfaceDetail
          detail={{ label: PortalDetails.RuntimeReference, value: intent.metrics[2]?.value ?? intent.state }}
        />
        <AppGameNotificationParentSurfaceDetail
          detail={{ label: PortalDetails.ProductClaim, value: intent.productClaim }}
        />
      </dl>
    </article>
  );
}

function AppGameNotificationParentSurfaceRowCard({
  row,
}: {
  readonly row: AppGameNotificationParentSurfacePanelRow;
}): ReactElement {
  return (
    <article className={cardClassName()}>
      <h2>{row.title}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        {row.details.map((detail) => (
          <AppGameNotificationParentSurfaceDetail key={`${row.key}:${detail.label}`} detail={detail} />
        ))}
      </dl>
    </article>
  );
}

function AppGameNotificationParentSurfaceDetail({
  detail,
}: {
  readonly detail: AppGameNotificationParentSurfaceDetail;
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
