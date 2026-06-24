import type { ReactElement } from 'react';
import { type PortalRoute as PortalRouteValue } from '@ocentra-parent/schema-domain/portal-contracts';
import { type DisplayText as PortalDisplayText } from '@ocentra-parent/schema-domain/text-contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/schema-domain/text-portal-dev';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalDetails } from '@ocentra-parent/portal-domain/details';
import { isPortalAppGameParentSurfaceRoute } from '@ocentra-parent/portal-domain/routes';
import type { PortalRenderActions } from './portal-actions';
import {
  createAppGameChildRuntimeTransportReceiptPanelIntent,
  type AppGameChildRuntimeTransportReceiptPanelDetail,
  type AppGameChildRuntimeTransportReceiptPanelIntent,
  type AppGameChildRuntimeTransportReceiptPanelRow,
} from '@ocentra-parent/portal-domain/app-game-child-runtime-transport-receipt-panel';

type AppGameChildRuntimeTransportReceiptReadModel = Exclude<
  Parameters<typeof createAppGameChildRuntimeTransportReceiptPanelIntent>[0],
  null
>;
type AppGameChildRuntimeTransportReceiptRouteReadModelResult =
  | {
      readonly ok: true;
      readonly value: AppGameChildRuntimeTransportReceiptReadModel;
    }
  | {
      readonly ok: false;
    };

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
  readonly readModelResult: AppGameChildRuntimeTransportReceiptRouteReadModelResult | null;
}): ReactElement {
  const readModel = readModelResult?.ok === true ? readModelResult.value : null;
  const intent = createAppGameChildRuntimeTransportReceiptPanelIntent(readModel);
  return (
    <section
      aria-label={resolvePortalDevText(PortalDevTextToken.GetActivityAppGameChildRuntimeTransportReceiptReadModel)}
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
