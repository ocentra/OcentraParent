import type { ReactElement } from 'react';
import { type PortalRoute as PortalRouteValue } from '@ocentra-parent/schema-domain/portal-contracts';
import { type DisplayText as PortalDisplayText } from '@ocentra-parent/schema-domain/text-contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/schema-domain/text-portal-dev';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import {
  createAppGameAdapterDispatchPreflightPanelIntent,
  type AppGameAdapterDispatchPreflightPanelDetail,
  type AppGameAdapterDispatchPreflightPanelIntent,
  type AppGameAdapterDispatchPreflightPanelRow,
} from '@ocentra-parent/portal-domain/app-game-adapter-dispatch-preflight-panel';
import {
  createAppGameAdapterDispatchResultPanelIntent,
  type AppGameAdapterDispatchResultPanelDetail,
  type AppGameAdapterDispatchResultPanelExecuteAction,
  type AppGameAdapterDispatchResultPanelIntent,
  type AppGameAdapterDispatchResultPanelRow,
} from '@ocentra-parent/portal-domain/app-game-adapter-dispatch-result-panel';
import { PortalDetails } from '@ocentra-parent/portal-domain/details';
import { isPortalAppGameParentSurfaceRoute } from '@ocentra-parent/portal-domain/routes';
import type { PortalRenderActions } from './portal-actions';

type AppGameAdapterDispatchPreflightRouteReadModel = Parameters<
  typeof createAppGameAdapterDispatchPreflightPanelIntent
>[0];
type AppGameAdapterDispatchResultRouteReadModel = Parameters<typeof createAppGameAdapterDispatchResultPanelIntent>[0];
type AppGameAdapterDispatchExecuteRouteResult = Parameters<typeof createAppGameAdapterDispatchResultPanelIntent>[1];

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
  readonly executeResult: AppGameAdapterDispatchExecuteRouteResult;
  readonly preflightResult: AppGameAdapterDispatchPreflightRouteReadModel;
  readonly resultReadModel: AppGameAdapterDispatchResultRouteReadModel;
}): ReactElement {
  const preflightIntent = createAppGameAdapterDispatchPreflightPanelIntent(preflightResult);
  const resultIntent = createAppGameAdapterDispatchResultPanelIntent(resultReadModel, executeResult);
  return (
    <section
      aria-label={resolvePortalDevText(PortalDevTextToken.ExecuteActivityAppGameAdapterDispatch)}
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
            onClick={() => void actions.refreshRouteSnapshot?.()}
          >
            {resolvePortalDevText(PortalDevTextToken.GetActivityAppGameAdapterDispatchPreflightReadModel)}
          </button>
          <button
            className={PortalDom.Classes.CommandResultTab}
            disabled={!commandEnabled}
            type={PortalDom.ButtonType.Button}
            onClick={() => void actions.refreshRouteSnapshot?.()}
          >
            {resolvePortalDevText(PortalDevTextToken.GetActivityAppGameAdapterDispatchResultReadModel)}
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
  _action: AppGameAdapterDispatchResultPanelExecuteAction
): void {
  void actions.requestAppGameAdapterDispatchExecute?.();
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
