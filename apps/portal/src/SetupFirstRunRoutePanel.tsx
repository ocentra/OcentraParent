import type { ReactElement } from 'react';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalRoute, type PortalRoute as PortalRouteValue } from '@ocentra-parent/portal-domain/routes';
import {
  createSetupFirstRunPanelIntent,
  type SetupFirstRunPanelCard,
  type SetupFirstRunPanelDetail,
} from '@ocentra-parent/portal-domain/setup-first-run-panel';
import { decodeDisplayText } from '@ocentra-parent/text-domain/contracts';

export function shouldRenderSetupFirstRunRoute(route: PortalRouteValue): boolean {
  return route === PortalRoute.Start;
}

export function SetupFirstRunRoutePanel(): ReactElement {
  const intent = createSetupFirstRunPanelIntent();

  return (
    <section
      aria-label={intent.title}
      className={PortalDom.Classes.TrackingStatusOverlay}
      data-ocentra-setup-proof="first-run-route"
    >
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
          <SetupFirstRunCard
            card={{
              title: decodeDisplayText('State machine summary'),
              summary: intent.summary,
              details: intent.summaryDetails,
            }}
          />
          {intent.cards.map((card, index) => (
            <SetupFirstRunCard key={`${String(card.title)}:${index}`} card={card} />
          ))}
        </div>
      </div>
    </section>
  );
}

function SetupFirstRunCard({
  card,
}: {
  readonly card: SetupFirstRunPanelCard;
}): ReactElement {
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
  readonly details: readonly SetupFirstRunPanelDetail[];
}): ReactElement {
  return (
    <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
      {details.map((detail, index) => (
        <SetupFirstRunDetail key={`${String(detail.label)}:${index}`} detail={detail} />
      ))}
    </dl>
  );
}

function SetupFirstRunDetail({
  detail,
}: {
  readonly detail: SetupFirstRunPanelDetail;
}): ReactElement {
  return (
    <div>
      <dt>{detail.label}</dt>
      <dd>{detail.value}</dd>
    </div>
  );
}

function setupCardClassName(): string {
  return [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
}
