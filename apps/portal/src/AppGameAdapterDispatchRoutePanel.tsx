import type { ReactElement } from 'react';
import { AgentCommand, AgentEvent } from '@ocentra-parent/agent-protocol-domain/contracts';
import type { AgentAppGameAdapterDispatchPreflightResult } from '@ocentra-parent/agent-protocol-domain/app-game-adapter-dispatch-preflight';
import type {
  AgentAppGameAdapterDispatchExecute,
  AgentAppGameAdapterDispatchResult,
} from '@ocentra-parent/agent-protocol-domain/app-game-adapter-dispatch-result';
import {
  PortalDetails,
  PortalDom,
  PortalText,
  PortalTextToken,
  isPortalAppGameParentSurfaceRoute,
  type PortalDisplayText,
  type PortalRoute as PortalRouteValue,
} from '@ocentra-parent/portal-domain/contracts';
import type { PortalRenderActions } from './portal-actions';
import {
  createAppGameAdapterDispatchPreflightPanelIntent,
  type AppGameAdapterDispatchPreflightPanelDetail,
  type AppGameAdapterDispatchPreflightPanelIntent,
  type AppGameAdapterDispatchPreflightPanelRow,
} from './app-game-adapter-dispatch-preflight-panel';
import {
  createAppGameAdapterDispatchResultPanelIntent,
  type AppGameAdapterDispatchResultPanelDetail,
  type AppGameAdapterDispatchResultPanelExecuteAction,
  type AppGameAdapterDispatchResultPanelIntent,
  type AppGameAdapterDispatchResultPanelRow,
} from './app-game-adapter-dispatch-result-panel';

export function shouldRenderAppGameAdapterDispatchRoute(route: PortalRouteValue): boolean {
  return isPortalAppGameParentSurfaceRoute(route);
}

export function AppGameAdapterDispatchRoutePanel({
  actions,
  commandEnabled,
  executeResult,
  preflightResult,
  resultReadModel,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly executeResult: AgentAppGameAdapterDispatchExecute | null;
  readonly preflightResult: AgentAppGameAdapterDispatchPreflightResult | null;
  readonly resultReadModel: AgentAppGameAdapterDispatchResult | null;
}): ReactElement {
  const preflightIntent = createAppGameAdapterDispatchPreflightPanelIntent(preflightResult);
  const resultIntent = createAppGameAdapterDispatchResultPanelIntent(resultReadModel, executeResult);
  return (
    <section
      aria-label={PortalText.Resolve(PortalTextToken.ExecuteActivityAppGameAdapterDispatch)}
      className={PortalDom.Classes.TrackingStatusOverlay}
    >
      <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <p className={PortalDom.Classes.ProductEyebrow}>{resultIntent.eyebrow}</p>
          <h2>{resultIntent.title}</h2>
          <p>{resultIntent.body}</p>
          <button
            className={PortalDom.Classes.CommandResultTab}
            disabled={!commandEnabled}
            type={PortalDom.ButtonType.Button}
            onClick={() => {
              actions.selectCommandResult(AgentEvent.ActivityAppGameAdapterDispatchPreflightReadModelReported);
              actions.sendCommand(AgentCommand.ActivityAppGameAdapterDispatchPreflightReadModelGet, {});
            }}
          >
            {PortalText.Resolve(PortalTextToken.GetActivityAppGameAdapterDispatchPreflightReadModel)}
          </button>
          <button
            className={PortalDom.Classes.CommandResultTab}
            disabled={!commandEnabled}
            type={PortalDom.ButtonType.Button}
            onClick={() => {
              actions.selectCommandResult(AgentEvent.ActivityAppGameAdapterDispatchResultReadModelReported);
              actions.sendCommand(AgentCommand.ActivityAppGameAdapterDispatchResultReadModelGet, {});
            }}
          >
            {PortalText.Resolve(PortalTextToken.GetActivityAppGameAdapterDispatchResultReadModel)}
          </button>
          {resultIntent.executeAction === null ? null : (
            <button
              className={PortalDom.Classes.CommandResultTab}
              disabled={!commandEnabled}
              type={PortalDom.ButtonType.Button}
              onClick={() => {
                const action = resultIntent.executeAction;
                if (action === null) {
                  return;
                }
                sendAppGameAdapterDispatchExecuteAction(actions, action);
              }}
            >
              {resultIntent.executeAction.label}
            </button>
          )}
        </header>
        <div
          className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
            PortalDom.Classes.ClassNameSeparator
          )}
        >
          <AppGameAdapterDispatchPreflightSummaryCard intent={preflightIntent} />
          <AppGameAdapterDispatchResultSummaryCard intent={resultIntent} />
          {preflightIntent.rows.map((row, index) => (
            <AppGameAdapterDispatchPreflightRowCard key={`${String(row.title)}:${index}`} row={row} />
          ))}
          {resultIntent.rows.length === 0 ? (
            <AppGameAdapterDispatchResultEmptyCard intent={resultIntent} />
          ) : (
            resultIntent.rows.map((row, index) => (
              <AppGameAdapterDispatchResultRowCard key={`${String(row.title)}:${index}`} row={row} />
            ))
          )}
        </div>
      </div>
    </section>
  );
}

export function sendAppGameAdapterDispatchExecuteAction(
  actions: PortalRenderActions,
  action: AppGameAdapterDispatchResultPanelExecuteAction
): void {
  actions.selectCommandResult(action.resultEvent);
  actions.sendCommand(action.command, {});
}

function AppGameAdapterDispatchPreflightSummaryCard({
  intent,
}: {
  readonly intent: AppGameAdapterDispatchPreflightPanelIntent;
}): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{PortalDetails.Capability}</h2>
      <AppGameAdapterDispatchPreflightDetails details={intent.summaryDetails} />
    </article>
  );
}

function AppGameAdapterDispatchResultSummaryCard({
  intent,
}: {
  readonly intent: AppGameAdapterDispatchResultPanelIntent;
}): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{PortalDetails.AdapterDispatch}</h2>
      <AppGameAdapterDispatchResultDetails details={intent.summaryDetails} />
    </article>
  );
}

function AppGameAdapterDispatchResultEmptyCard({
  intent,
}: {
  readonly intent: AppGameAdapterDispatchResultPanelIntent;
}): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{intent.loadState}</h2>
      <p>{intent.emptyMessage}</p>
      <AppGameAdapterDispatchResultDetails
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

function AppGameAdapterDispatchPreflightRowCard({
  row,
}: {
  readonly row: AppGameAdapterDispatchPreflightPanelRow;
}): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{row.title}</h2>
      <AppGameAdapterDispatchPreflightDetails details={row.details} />
    </article>
  );
}

function AppGameAdapterDispatchResultRowCard({
  row,
}: {
  readonly row: AppGameAdapterDispatchResultPanelRow;
}): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{row.title}</h2>
      <AppGameAdapterDispatchResultDetails details={row.details} />
    </article>
  );
}

function AppGameAdapterDispatchPreflightDetails({
  details,
}: {
  readonly details: readonly AppGameAdapterDispatchPreflightPanelDetail[];
}): ReactElement {
  return (
    <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
      {details.map((detail, index) => (
        <AppGameAdapterDispatchDetail
          key={`${String(detail.label)}:${index}`}
          label={detail.label}
          value={detail.value}
        />
      ))}
    </dl>
  );
}

function AppGameAdapterDispatchResultDetails({
  details,
}: {
  readonly details: readonly AppGameAdapterDispatchResultPanelDetail[];
}): ReactElement {
  return (
    <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
      {details.map((detail, index) => (
        <AppGameAdapterDispatchDetail
          key={`${String(detail.label)}:${index}`}
          label={detail.label}
          value={detail.value}
        />
      ))}
    </dl>
  );
}

function AppGameAdapterDispatchDetail({
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
