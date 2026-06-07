import { AgentCommand, AgentEvent, type AgentEventEnvelope } from '@ocentra-parent/agent-protocol-domain/contracts';
import { parseAgentSocialAlertReportReadModelEvent } from '@ocentra-parent/agent-protocol-domain/social-alert-report-read-model';
import {
  PortalDetails,
  PortalDom,
  PortalRoute,
  type PortalRoute as PortalRouteValue,
} from '@ocentra-parent/portal-domain/contracts';
import { type ReactElement } from 'react';
import type { PortalRenderActions } from './portal-actions';
import {
  createSocialAlertReportPanelIntent,
  type SocialAlertReportPanelDetail,
  type SocialAlertReportPanelIntent,
  type SocialAlertReportPanelRow,
} from './social-alert-report-panel';

export function shouldRenderSocialAlertReportRoute(route: PortalRouteValue): boolean {
  return route === PortalRoute.Browser;
}

export function SocialAlertReportRoutePanel({
  actions,
  commandEnabled,
  events,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly events: readonly AgentEventEnvelope[];
}): ReactElement {
  const snapshot = latestSocialAlertReportSnapshot(events);
  const intent = createSocialAlertReportPanelIntent(snapshot);
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
            onClick={() => {
              actions.selectCommandResult(AgentEvent.BrowserSocialAlertReportReadModelReported);
              actions.sendCommand(AgentCommand.BrowserSocialAlertReportReadModelGet, {});
            }}
            type={PortalDom.ButtonType.Button}
          >
            {intent.title}
          </button>
        </header>
        <div
          className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
            PortalDom.Classes.ClassNameSeparator
          )}
        >
          <SocialAlertReportSummaryCard intent={intent} />
          {intent.rows.length === 0 ? (
            <SocialAlertReportEmptyCard intent={intent} />
          ) : (
            intent.rows.map((row) => <SocialAlertReportRowCard key={row.key} row={row} />)
          )}
        </div>
      </div>
    </section>
  );
}

function latestSocialAlertReportSnapshot(events: readonly AgentEventEnvelope[]): unknown {
  const event = latestSocialAlertReportEvent(events);
  if (event === null) {
    return null;
  }
  const parsed = parseAgentSocialAlertReportReadModelEvent(event);
  return parsed.ok ? parsed.value : null;
}

function latestSocialAlertReportEvent(events: readonly AgentEventEnvelope[]): AgentEventEnvelope | null {
  let latest: AgentEventEnvelope | null = null;
  let latestTime = Number.NEGATIVE_INFINITY;
  for (let index = 0; index < events.length; index += 1) {
    const event = events[index];
    if (event === undefined || event.event !== AgentEvent.BrowserSocialAlertReportReadModelReported) {
      continue;
    }
    const timestamp = Date.parse(event.sentAt);
    if (Number.isFinite(timestamp) && timestamp >= latestTime) {
      latest = event;
      latestTime = timestamp;
    }
  }
  return latest;
}

function SocialAlertReportSummaryCard({ intent }: { readonly intent: SocialAlertReportPanelIntent }): ReactElement {
  return (
    <article className={cardClassName()}>
      <h2>{intent.summary}</h2>
      <SocialAlertReportDetails details={intent.metrics} />
    </article>
  );
}

function SocialAlertReportEmptyCard({ intent }: { readonly intent: SocialAlertReportPanelIntent }): ReactElement {
  return (
    <article className={cardClassName()}>
      <h2>{intent.emptyMessage}</h2>
      <SocialAlertReportDetails
        details={[
          { label: PortalDetails.Status, value: intent.state },
          { label: PortalDetails.ProductClaim, value: intent.productClaim },
        ]}
      />
    </article>
  );
}

function SocialAlertReportRowCard({ row }: { readonly row: SocialAlertReportPanelRow }): ReactElement {
  return (
    <article className={cardClassName()}>
      <h2>{row.title}</h2>
      <SocialAlertReportDetails details={row.details} />
    </article>
  );
}

function SocialAlertReportDetails({
  details,
}: {
  readonly details: readonly SocialAlertReportPanelDetail[];
}): ReactElement {
  return (
    <dl>
      {details.map((detail) => (
        <div key={detail.label}>
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
