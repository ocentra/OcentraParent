import type { ReactElement } from 'react';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import {
  isParentSetupFirstRunRoute,
  type ParentRouteId,
  type ParentSetupFirstRunPanelCardSnapshot,
  type ParentSetupFirstRunPanelDetailSnapshot,
  type ParentSetupFirstRunPanelSnapshot,
} from '../generated/parent-ui-bridge';

export function shouldRenderSetupFirstRunRoute(route: ParentRouteId): boolean {
  return isParentSetupFirstRunRoute(route);
}

export function SetupFirstRunRoutePanel({
  panel,
}: {
  readonly panel: ParentSetupFirstRunPanelSnapshot | null;
}): ReactElement {
  if (panel === null) {
    return (
      <section aria-label="Start route unavailable" className={PortalDom.Classes.TrackingStatusOverlay}>
        <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
          <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
            <p className={PortalDom.Classes.ProductEyebrow}>Setup route</p>
            <h2>Start route unavailable</h2>
            <p>Parent Rust snapshot unavailable for the setup-first-run route.</p>
          </header>
        </div>
      </section>
    );
  }

  return (
    <section
      aria-label={panel.title}
      className={PortalDom.Classes.TrackingStatusOverlay}
      data-ocentra-setup-proof="first-run-route"
    >
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
          <SetupFirstRunCard
            card={{
              title: panel.summaryCardTitle,
              summary: panel.summary,
              details: panel.summaryDetails,
            }}
          />
          {panel.cards.map((card, index) => (
            <SetupFirstRunCard key={`${String(card.title)}:${index}`} card={card} />
          ))}
        </div>
      </div>
    </section>
  );
}

function SetupFirstRunCard({ card }: { readonly card: ParentSetupFirstRunPanelCardSnapshot }): ReactElement {
  return (
    <article className={setupCardClassName()}>
      <h2>{card.title}</h2>
      <p>{card.summary}</p>
      <SetupFirstRunDetails details={card.details} />
    </article>
  );
}

function SetupFirstRunDetails({
  details,
}: {
  readonly details: readonly ParentSetupFirstRunPanelDetailSnapshot[];
}): ReactElement {
  return (
    <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
      {details.map((detail, index) => (
        <SetupFirstRunDetail key={`${String(detail.label)}:${index}`} detail={detail} />
      ))}
    </dl>
  );
}

function SetupFirstRunDetail({ detail }: { readonly detail: ParentSetupFirstRunPanelDetailSnapshot }): ReactElement {
  return (
    <div>
      <dt>{detail.label}</dt>
      <dd>{detail.value}</dd>
    </div>
  );
}

function setupCardClassName(): string {
  return [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(PortalDom.Classes.ClassNameSeparator);
}
