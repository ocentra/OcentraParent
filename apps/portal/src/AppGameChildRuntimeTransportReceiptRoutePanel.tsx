import type { ReactElement } from 'react';
import { AgentCommand, AgentEvent } from '@ocentra-parent/agent-protocol-domain/contracts';
import type { AgentAppGameChildRuntimeTransportReceiptResult } from '@ocentra-parent/agent-protocol-domain/app-game-child-runtime-transport-receipt';
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
  createAppGameChildRuntimeTransportReceiptPanelIntent,
  type AppGameChildRuntimeTransportReceiptPanelDetail,
  type AppGameChildRuntimeTransportReceiptPanelIntent,
  type AppGameChildRuntimeTransportReceiptPanelRow,
} from './app-game-child-runtime-transport-receipt-panel';

export function shouldRenderAppGameChildRuntimeTransportReceiptRoute(route: PortalRouteValue): boolean {
  return isPortalAppGameParentSurfaceRoute(route);
}

export function AppGameChildRuntimeTransportReceiptRoutePanel({
  actions,
  commandEnabled,
  readModelResult,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly readModelResult: AgentAppGameChildRuntimeTransportReceiptResult | null;
}): ReactElement {
  const readModel = readModelResult?.ok === true ? readModelResult.value : null;
  const intent = createAppGameChildRuntimeTransportReceiptPanelIntent(readModel);
  return (
    <section
      aria-label={PortalText.Resolve(PortalTextToken.GetActivityAppGameChildRuntimeTransportReceiptReadModel)}
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
              actions.selectCommandResult(AgentEvent.ActivityAppGameChildRuntimeTransportReceiptReadModelReported);
              actions.sendCommand(AgentCommand.ActivityAppGameChildRuntimeTransportReceiptReadModelGet, {});
            }}
          >
            {PortalText.Resolve(PortalTextToken.GetActivityAppGameChildRuntimeTransportReceiptReadModel)}
          </button>
        </header>
        <div
          className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
            PortalDom.Classes.ClassNameSeparator
          )}
        >
          <AppGameChildRuntimeTransportReceiptSummaryCard intent={intent} />
          {intent.rows.length === 0 ? (
            <AppGameChildRuntimeTransportReceiptEmptyCard intent={intent} />
          ) : (
            intent.rows.map((row, index) => (
              <AppGameChildRuntimeTransportReceiptRowCard key={`${String(row.title)}:${index}`} row={row} />
            ))
          )}
        </div>
      </div>
    </section>
  );
}

function AppGameChildRuntimeTransportReceiptSummaryCard({
  intent,
}: {
  readonly intent: AppGameChildRuntimeTransportReceiptPanelIntent;
}): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{PortalDetails.ChildDelivery}</h2>
      <AppGameChildRuntimeTransportReceiptDetails details={intent.summaryDetails} />
    </article>
  );
}

function AppGameChildRuntimeTransportReceiptEmptyCard({
  intent,
}: {
  readonly intent: AppGameChildRuntimeTransportReceiptPanelIntent;
}): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{intent.loadState}</h2>
      <p>{intent.emptyMessage}</p>
      <AppGameChildRuntimeTransportReceiptDetails
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

function AppGameChildRuntimeTransportReceiptRowCard({
  row,
}: {
  readonly row: AppGameChildRuntimeTransportReceiptPanelRow;
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
  readonly details: readonly AppGameChildRuntimeTransportReceiptPanelDetail[];
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
