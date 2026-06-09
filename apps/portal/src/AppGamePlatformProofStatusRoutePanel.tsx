import type { ReactElement } from 'react';
import { AgentCommand, AgentEvent } from '@ocentra-parent/agent-protocol-domain/contracts';
import type { AgentAppGamePlatformProofStatusResult } from '@ocentra-parent/agent-protocol-domain/app-game-platform-proof-status';
import {
  PortalDetails,
  PortalDom,
  PortalRoute,
  PortalText,
  PortalTextToken,
  type PortalDisplayText,
  type PortalRoute as PortalRouteValue,
} from '@ocentra-parent/portal-domain/contracts';
import type { PortalRenderActions } from './portal-actions';
import {
  createAppGamePlatformProofStatusPanelIntent,
  type AppGamePlatformProofStatusPanelDetail,
  type AppGamePlatformProofStatusPanelIntent,
  type AppGamePlatformProofStatusPanelRow,
} from './app-game-platform-proof-status-panel';

export function shouldRenderAppGamePlatformProofStatusRoute(route: PortalRouteValue): boolean {
  return route === PortalRoute.AppGameSessions;
}

export function AppGamePlatformProofStatusRoutePanel({
  actions,
  commandEnabled,
  readModelResult,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly readModelResult: AgentAppGamePlatformProofStatusResult | null;
}): ReactElement {
  const readModel = readModelResult?.ok === true ? readModelResult.value : null;
  const intent = createAppGamePlatformProofStatusPanelIntent(readModel);
  return (
    <section
      aria-label={PortalText.Resolve(PortalTextToken.GetActivityAppGamePlatformProofStatusReadModel)}
      className={PortalDom.Classes.TrackingStatusOverlay}
    >
      <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <p className={PortalDom.Classes.ProductEyebrow}>{intent.eyebrow}</p>
          <h2>{intent.title}</h2>
          <p>{intent.body}</p>
          <button
            className={PortalDom.Classes.CommandResultTab}
            disabled={!commandEnabled}
            type={PortalDom.ButtonType.Button}
            onClick={() => {
              actions.selectCommandResult(AgentEvent.ActivityAppGamePlatformProofStatusReadModelReported);
              actions.sendCommand(AgentCommand.ActivityAppGamePlatformProofStatusReadModelGet, {});
            }}
          >
            {PortalText.Resolve(PortalTextToken.GetActivityAppGamePlatformProofStatusReadModel)}
          </button>
        </header>
        <div
          className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
            PortalDom.Classes.ClassNameSeparator
          )}
        >
          <AppGamePlatformProofStatusSummaryCard intent={intent} />
          {intent.rows.length === 0 ? (
            <AppGamePlatformProofStatusEmptyCard intent={intent} />
          ) : (
            intent.rows.map((row) => <AppGamePlatformProofStatusRowCard key={String(row.title)} row={row} />)
          )}
        </div>
      </div>
    </section>
  );
}

function AppGamePlatformProofStatusSummaryCard({
  intent,
}: {
  readonly intent: AppGamePlatformProofStatusPanelIntent;
}): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{PortalDetails.PlatformState}</h2>
      <AppGamePlatformProofStatusDetails details={intent.summaryDetails} />
    </article>
  );
}

function AppGamePlatformProofStatusEmptyCard({
  intent,
}: {
  readonly intent: AppGamePlatformProofStatusPanelIntent;
}): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{intent.loadState}</h2>
      <p>{intent.emptyMessage}</p>
      <AppGamePlatformProofStatusDetails
        details={[
          {
            label: PortalDetails.ProductClaim,
            value: intent.productClaim,
          },
        ]}
      />
    </article>
  );
}

function AppGamePlatformProofStatusRowCard({
  row,
}: {
  readonly row: AppGamePlatformProofStatusPanelRow;
}): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{row.title}</h2>
      <AppGamePlatformProofStatusDetails details={row.details} />
    </article>
  );
}

function AppGamePlatformProofStatusDetails({
  details,
}: {
  readonly details: readonly AppGamePlatformProofStatusPanelDetail[];
}): ReactElement {
  return (
    <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
      {details.map((detail) => (
        <AppGamePlatformProofStatusDetail key={String(detail.label)} label={detail.label} value={detail.value} />
      ))}
    </dl>
  );
}

function AppGamePlatformProofStatusDetail({
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
