import type { ReactElement } from 'react';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/portal-domain/display-text';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import {
  isParentSetupFirstRunRoute,
  type ParentRouteId,
  type ParentSetupFirstRunPanelCardSnapshot,
  type ParentSetupFirstRunPanelDetailSnapshot,
  type ParentSetupFirstRunPanelSnapshot,
} from '../generated/parent-ui-bridge';
import type { PortalRenderActions } from './portal-actions';

export function shouldRenderSetupFirstRunRoute(route: ParentRouteId): boolean {
  return isParentSetupFirstRunRoute(route);
}

export function SetupFirstRunRoutePanel({
  actions,
  panel,
}: {
  readonly actions: PortalRenderActions;
  readonly panel: ParentSetupFirstRunPanelSnapshot | null;
}): ReactElement {
  if (panel === null) {
    return (
      <section
        aria-label="Setup status unavailable"
        className={[PortalDom.Classes.TrackingStatusOverlay, PortalDom.Classes.SetupFirstRunRoutePanel].join(
          PortalDom.Classes.ClassNameSeparator
        )}
        data-ocentra-setup-state="unavailable"
      >
        <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
          <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
            <p className={PortalDom.Classes.ProductEyebrow}>Setup status</p>
            <h2>Setup status unavailable</h2>
            <p>Connect the local service to load current setup progress. The setup guide remains available above.</p>
            <button
              className={PortalDom.Classes.CommandResultTab}
              onClick={actions.reconnect}
              type={PortalDom.ButtonType.Button}
            >
              {resolvePortalDevText(PortalDevTextToken.RetryStatus)}
            </button>
          </header>
        </div>
      </section>
    );
  }

  return (
    <section
      aria-label={panel.title}
      className={[PortalDom.Classes.TrackingStatusOverlay, PortalDom.Classes.SetupFirstRunRoutePanel].join(
        PortalDom.Classes.ClassNameSeparator
      )}
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
            <SetupFirstRunDetailCard key={`${String(card.title)}:${index}`} card={card} />
          ))}
        </div>
      </div>
    </section>
  );
}

function SetupFirstRunDetailCard({ card }: { readonly card: ParentSetupFirstRunPanelCardSnapshot }): ReactElement {
  return (
    <details className={PortalDom.Classes.SetupFirstRunDetailCard}>
      <summary>
        <span className={PortalDom.Classes.SetupFirstRunDetailTitle}>{card.title}</span>
        <span className={PortalDom.Classes.SetupFirstRunDetailSummary}>{card.summary}</span>
      </summary>
      <div className={PortalDom.Classes.SetupFirstRunDetailContent}>
        <SetupFirstRunDetails details={card.details} />
      </div>
    </details>
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
