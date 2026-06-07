import type { ReactElement } from 'react';
import { AgentCommand, AgentEvent } from '@ocentra-parent/agent-protocol-domain/contracts';
import type { AgentAppGameTimerParentSurfaceResult } from '@ocentra-parent/agent-protocol-domain/app-game-timer-parent-surface-read-model';
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
  createAppGameTimerParentSurfacePanelIntent,
  type AppGameTimerParentSurfacePanelDetail,
  type AppGameTimerParentSurfacePanelIntent,
  type AppGameTimerParentSurfacePanelRow,
} from './app-game-timer-parent-surface-panel';

export function shouldRenderAppGameTimerParentSurfaceRoute(route: PortalRouteValue): boolean {
  return route === PortalRoute.AppGameSessions;
}

export function AppGameTimerParentSurfaceRoutePanel({
  actions,
  commandEnabled,
  readModelResult,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly readModelResult: AgentAppGameTimerParentSurfaceResult | null;
}): ReactElement {
  const intent = createAppGameTimerParentSurfacePanelIntent(readModelResult);
  return (
    <section
      aria-label={PortalText.Resolve(PortalTextToken.AppGameTimerParentSurface)}
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
              actions.selectCommandResult(AgentEvent.ActivityAppGameTimerParentSurfaceReadModelReported);
              actions.sendCommand(AgentCommand.ActivityAppGameTimerParentSurfaceReadModelGet, {});
            }}
          >
            {PortalText.Resolve(PortalTextToken.GetActivityAppGameTimerParentSurfaceReadModel)}
          </button>
        </header>
        <div
          className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
            PortalDom.Classes.ClassNameSeparator
          )}
        >
          <AppGameTimerParentSurfaceSummaryCard intent={intent} />
          {intent.rows.length === 0 ? (
            <AppGameTimerParentSurfaceEmptyCard intent={intent} />
          ) : (
            intent.rows.map((row) => <AppGameTimerParentSurfaceRowCard key={String(row.title)} row={row} />)
          )}
        </div>
      </div>
    </section>
  );
}

function AppGameTimerParentSurfaceSummaryCard({
  intent,
}: {
  readonly intent: AppGameTimerParentSurfacePanelIntent;
}): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{PortalDetails.RuntimeReference}</h2>
      <AppGameTimerParentSurfaceDetails details={intent.summaryDetails} />
    </article>
  );
}

function AppGameTimerParentSurfaceEmptyCard({
  intent,
}: {
  readonly intent: AppGameTimerParentSurfacePanelIntent;
}): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{intent.loadState}</h2>
      <p>{intent.emptyMessage}</p>
      <AppGameTimerParentSurfaceDetails
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

function AppGameTimerParentSurfaceRowCard({ row }: { readonly row: AppGameTimerParentSurfacePanelRow }): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{row.title}</h2>
      <AppGameTimerParentSurfaceDetails details={row.details} />
    </article>
  );
}

function AppGameTimerParentSurfaceDetails({
  details,
}: {
  readonly details: readonly AppGameTimerParentSurfacePanelDetail[];
}): ReactElement {
  return (
    <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
      {details.map((detail) => (
        <AppGameTimerParentSurfaceDetail key={String(detail.label)} label={detail.label} value={detail.value} />
      ))}
    </dl>
  );
}

function AppGameTimerParentSurfaceDetail({
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
