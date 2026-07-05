import type { ReactElement } from 'react';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/portal-domain/display-text';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalDetails } from '@ocentra-parent/portal-domain/details';
import {
  isParentAppGameParentSurfaceRoute,
  type ParentAppGamePanelDetailSnapshot,
  type ParentAppGamePanelRowSnapshot,
  type ParentAppGamePanelSnapshot,
  type ParentRouteId,
} from '../generated/parent-ui-bridge';
import type { PortalRenderActions } from './portal-actions';

const EmptyChildRuntimeTransportReceiptPanel: ParentAppGamePanelSnapshot = {
  eyebrow: 'Rust-owned panel',
  title: 'App/game child runtime transport receipt',
  body: 'Rust has not reported an app/game child runtime transport receipt panel yet.',
  loadState: 'unavailable',
  summaryDetails: [
    { label: PortalDetails.ProductClaim, value: 'Child runtime transport status has not been reported yet.' },
  ],
  rows: [],
  emptyMessage: 'No app/game child runtime transport receipt panel has been reported yet.',
  productClaim: 'Runtime transport, runtime receipt, and provider delivery remain unclaimed.',
};

export function shouldRenderAppGameChildRuntimeTransportReceiptRoute(route: ParentRouteId): boolean {
  return isParentAppGameParentSurfaceRoute(route);
}

export function AppGameChildRuntimeTransportReceiptRoutePanel({
  actions,
  commandEnabled,
  panel,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly panel: ParentAppGamePanelSnapshot | null;
}): ReactElement {
  const resolvedPanel = panel ?? EmptyChildRuntimeTransportReceiptPanel;
  return (
    <section
      aria-label={resolvePortalDevText(PortalDevTextToken.GetActivityAppGameChildRuntimeTransportReceiptReadModel)}
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
            {resolvePortalDevText(PortalDevTextToken.GetActivityAppGameChildRuntimeTransportReceiptReadModel)}
          </button>
        </header>
        <div
          className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
            PortalDom.Classes.ClassNameSeparator
          )}
        >
          <AppGameChildRuntimeTransportReceiptSummaryCard panel={resolvedPanel} />
          {resolvedPanel.rows.length === 0 ? (
            <AppGameChildRuntimeTransportReceiptEmptyCard panel={resolvedPanel} />
          ) : (
            resolvedPanel.rows.map((row, index) => (
              <AppGameChildRuntimeTransportReceiptRowCard key={`${String(row.title)}:${index}`} row={row} />
            ))
          )}
        </div>
      </div>
    </section>
  );
}

function AppGameChildRuntimeTransportReceiptSummaryCard({
  panel,
}: {
  readonly panel: ParentAppGamePanelSnapshot;
}): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{PortalDetails.ChildDelivery}</h2>
      <AppGameChildRuntimeTransportReceiptDetails details={panel.summaryDetails} />
    </article>
  );
}

function AppGameChildRuntimeTransportReceiptEmptyCard({
  panel,
}: {
  readonly panel: ParentAppGamePanelSnapshot;
}): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{panel.loadState}</h2>
      <p>{panel.emptyMessage}</p>
      <AppGameChildRuntimeTransportReceiptDetails
        details={[{ label: PortalDetails.ProductClaim, value: panel.productClaim }]}
      />
    </article>
  );
}

function AppGameChildRuntimeTransportReceiptRowCard({
  row,
}: {
  readonly row: ParentAppGamePanelRowSnapshot;
}): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{row.title}</h2>
      <AppGameChildRuntimeTransportReceiptDetails details={row.details} />
    </article>
  );
}

function AppGameChildRuntimeTransportReceiptDetails({
  details,
}: {
  readonly details: readonly ParentAppGamePanelDetailSnapshot[];
}): ReactElement {
  return (
    <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
      {details.map((detail, index) => (
        <AppGameChildRuntimeTransportReceiptDetail key={`${String(detail.label)}:${index}`} {...detail} />
      ))}
    </dl>
  );
}

function AppGameChildRuntimeTransportReceiptDetail({
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
