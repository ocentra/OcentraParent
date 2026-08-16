import type { ReactElement } from 'react';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/portal-domain/display-text';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import {
  isParentAppGameParentSurfaceRoute,
  type ParentAppGameAdapterDispatchPanelSnapshot,
  type ParentAppGamePanelDetailSnapshot,
  type ParentAppGamePanelRowSnapshot,
  type ParentAppGamePanelSnapshot,
  type ParentRouteId,
} from '../generated/parent-ui-bridge';
import { PortalDetails } from '@ocentra-parent/portal-domain/details';
import type { PortalRenderActions } from './portal-actions';

const EmptyAdapterDispatchPreflightPanel: ParentAppGamePanelSnapshot = {
  eyebrow: 'Rust-owned panel',
  title: 'Adapter dispatch preflight',
  body: 'Rust has not reported an adapter dispatch preflight panel yet.',
  loadState: 'unavailable',
  summaryDetails: [
    { label: PortalDetails.ProductClaim, value: 'Adapter dispatch preflight has not been reported yet.' },
  ],
  rows: [],
  emptyMessage: 'No adapter dispatch preflight rows have been reported yet.',
  productClaim: 'Scoped adapter dispatch readiness has not been reported yet.',
};

const EmptyAdapterDispatchResultPanel: ParentAppGamePanelSnapshot = {
  eyebrow: 'Rust-owned panel',
  title: 'Adapter dispatch result',
  body: 'Rust has not reported an adapter dispatch result panel yet.',
  loadState: 'unavailable',
  summaryDetails: [{ label: PortalDetails.ProductClaim, value: 'Adapter dispatch result has not been reported yet.' }],
  rows: [],
  emptyMessage: 'No adapter dispatch result rows have been reported yet.',
  productClaim: 'Scoped adapter dispatch execution has not been reported yet.',
};

const EmptyAdapterDispatchPanel: ParentAppGameAdapterDispatchPanelSnapshot = {
  eyebrow: 'Rust-owned panel',
  title: 'App/game adapter dispatch',
  body: 'Rust has not reported an app/game adapter dispatch panel yet.',
  preflightPanel: EmptyAdapterDispatchPreflightPanel,
  resultPanel: EmptyAdapterDispatchResultPanel,
  executeActionLabel: null,
};

export function shouldRenderAppGameAdapterDispatchRoute(route: ParentRouteId): boolean {
  return isParentAppGameParentSurfaceRoute(route);
}

export function AppGameAdapterDispatchRoutePanel({
  actions,
  commandEnabled,
  panel,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly panel: ParentAppGameAdapterDispatchPanelSnapshot | null;
}): ReactElement {
  const resolvedPanel = panel ?? EmptyAdapterDispatchPanel;
  const preflightPanel = resolvedPanel.preflightPanel;
  const resultPanel = resolvedPanel.resultPanel;
  return (
    <section
      aria-label={resolvePortalDevText(PortalDevTextToken.ExecuteActivityAppGameAdapterDispatch)}
      className={PortalDom.Classes.TrackingStatusOverlay}
    >
      <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <p className={PortalDom.Classes.ProductEyebrow}>{resolvedPanel.eyebrow}</p>
          <h2>{resolvedPanel.title}</h2>
          <p>{resolvedPanel.body}</p>
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
          {resolvedPanel.executeActionLabel === null || resolvedPanel.executeActionLabel === undefined ? null : (
            <button
              className={PortalDom.Classes.CommandResultTab}
              disabled={!commandEnabled}
              type={PortalDom.ButtonType.Button}
              onClick={() => sendAppGameAdapterDispatchExecuteAction(actions)}
            >
              {resolvedPanel.executeActionLabel}
            </button>
          )}
        </header>
        <div
          className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
            PortalDom.Classes.ClassNameSeparator
          )}
        >
          <AppGameAdapterDispatchSummaryCard
            heading={PortalDetails.Capability}
            details={preflightPanel.summaryDetails}
          />
          <AppGameAdapterDispatchSummaryCard
            heading={PortalDetails.AdapterDispatch}
            details={resultPanel.summaryDetails}
          />
          {preflightPanel.rows.length === 0 ? (
            <AppGameAdapterDispatchEmptyCard panel={preflightPanel} />
          ) : (
            preflightPanel.rows.map((row, index) => (
              <AppGameAdapterDispatchRowCard key={`${String(row.title)}:preflight:${index}`} row={row} />
            ))
          )}
          {resultPanel.rows.length === 0 ? (
            <AppGameAdapterDispatchEmptyCard panel={resultPanel} />
          ) : (
            resultPanel.rows.map((row, index) => (
              <AppGameAdapterDispatchRowCard key={`${String(row.title)}:result:${index}`} row={row} />
            ))
          )}
        </div>
      </div>
    </section>
  );
}

export function sendAppGameAdapterDispatchExecuteAction(actions: PortalRenderActions): void {
  void actions.requestAppGameAdapterDispatchExecute?.();
}

function AppGameAdapterDispatchSummaryCard({
  heading,
  details,
}: {
  readonly heading: string;
  readonly details: readonly ParentAppGamePanelDetailSnapshot[];
}): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{heading}</h2>
      <AppGameAdapterDispatchDetails details={details} />
    </article>
  );
}

function AppGameAdapterDispatchEmptyCard({ panel }: { readonly panel: ParentAppGamePanelSnapshot }): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{panel.loadState}</h2>
      <p>{panel.emptyMessage}</p>
      <AppGameAdapterDispatchDetails details={[{ label: PortalDetails.ProductClaim, value: panel.productClaim }]} />
    </article>
  );
}

function AppGameAdapterDispatchRowCard({ row }: { readonly row: ParentAppGamePanelRowSnapshot }): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{row.title}</h2>
      <AppGameAdapterDispatchDetails details={row.details} />
    </article>
  );
}

function AppGameAdapterDispatchDetails({
  details,
}: {
  readonly details: readonly ParentAppGamePanelDetailSnapshot[];
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
  readonly label: string;
  readonly value: string;
}): ReactElement {
  return (
    <div>
      <dt>{label}</dt>
      <dd>{value}</dd>
    </div>
  );
}
