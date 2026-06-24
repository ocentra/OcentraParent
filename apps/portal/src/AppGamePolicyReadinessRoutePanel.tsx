import type { ReactElement } from 'react';
import { type PortalRoute as PortalRouteValue } from '@ocentra-parent/schema-domain/portal-contracts';
import { type DisplayText as PortalDisplayText } from '@ocentra-parent/schema-domain/text-contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/schema-domain/text-portal-dev';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import {
  createAppGamePolicyReadinessPanelIntent,
  type AppGamePolicyReadinessPanelDetail,
  type AppGamePolicyReadinessPanelIntent,
  type AppGamePolicyReadinessPanelRow,
} from '@ocentra-parent/portal-domain/app-game-policy-readiness-panel';
import { PortalDetails } from '@ocentra-parent/portal-domain/details';
import { isPortalAppGameParentSurfaceRoute } from '@ocentra-parent/portal-domain/routes';
import type { PortalRenderActions } from './portal-actions';

type AppGamePolicyReadinessRouteReadModelResult = Parameters<typeof createAppGamePolicyReadinessPanelIntent>[0];

export function shouldRenderAppGamePolicyReadinessRoute(route: PortalRouteValue): boolean {
  return isPortalAppGameParentSurfaceRoute(route);
}

export function AppGamePolicyReadinessRoutePanel({
  actions,
  commandEnabled,
  readModelResult,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly readModelResult: AppGamePolicyReadinessRouteReadModelResult;
}): ReactElement {
  const intent = createAppGamePolicyReadinessPanelIntent(readModelResult);
  return (
    <section
      aria-label={resolvePortalDevText(PortalDevTextToken.AppGamePolicyReadiness)}
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
            {resolvePortalDevText(PortalDevTextToken.GetActivityAppGamePolicyReadinessReadModel)}
          </button>
        </header>
        <div
          className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
            PortalDom.Classes.ClassNameSeparator
          )}
        >
          <AppGamePolicyReadinessSummaryCard intent={intent} />
          {intent.rows.length === 0 ? (
            <AppGamePolicyReadinessEmptyCard intent={intent} />
          ) : (
            intent.rows.map((row, index) => (
              <AppGamePolicyReadinessRowCard key={`${String(row.title)}:${index}`} row={row} />
            ))
          )}
        </div>
      </div>
    </section>
  );
}

function AppGamePolicyReadinessSummaryCard({
  intent,
}: {
  readonly intent: AppGamePolicyReadinessPanelIntent;
}): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{PortalDetails.PolicyReadiness}</h2>
      <AppGamePolicyReadinessDetails details={intent.summaryDetails} />
    </article>
  );
}

function AppGamePolicyReadinessEmptyCard({
  intent,
}: {
  readonly intent: AppGamePolicyReadinessPanelIntent;
}): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{intent.loadState}</h2>
      <p>{intent.emptyMessage}</p>
      <AppGamePolicyReadinessDetails
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

function AppGamePolicyReadinessRowCard({ row }: { readonly row: AppGamePolicyReadinessPanelRow }): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{row.title}</h2>
      <AppGamePolicyReadinessDetails details={row.details} />
    </article>
  );
}

function AppGamePolicyReadinessDetails({
  details,
}: {
  readonly details: readonly AppGamePolicyReadinessPanelDetail[];
}): ReactElement {
  return (
    <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
      {details.map((detail, index) => (
        <AppGamePolicyReadinessDetail
          key={`${String(detail.label)}:${index}`}
          label={detail.label}
          value={detail.value}
        />
      ))}
    </dl>
  );
}

function AppGamePolicyReadinessDetail({
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
