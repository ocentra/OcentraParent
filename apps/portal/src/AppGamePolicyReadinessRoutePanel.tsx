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

const EmptyPolicyReadinessPanel: ParentAppGamePanelSnapshot = {
  eyebrow: 'Rust-owned panel',
  title: 'App/game policy readiness',
  body: 'Rust has not reported an app/game policy readiness panel yet.',
  loadState: 'unavailable',
  summaryDetails: [{ label: PortalDetails.ProductClaim, value: 'Policy readiness has not been reported yet.' }],
  rows: [],
  emptyMessage: 'No app/game policy readiness panel has been reported yet.',
  productClaim: 'Approval workflow, category routing, and adapter dispatch remain unclaimed.',
};

export function shouldRenderAppGamePolicyReadinessRoute(route: ParentRouteId): boolean {
  return isParentAppGameParentSurfaceRoute(route);
}

export function AppGamePolicyReadinessRoutePanel({
  actions,
  commandEnabled,
  panel,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly panel: ParentAppGamePanelSnapshot | null;
}): ReactElement {
  const resolvedPanel = panel ?? EmptyPolicyReadinessPanel;
  return (
    <section
      aria-label={resolvePortalDevText(PortalDevTextToken.AppGamePolicyReadiness)}
      className={PortalDom.Classes.TrackingStatusOverlay}
      data-ocentra-policy-readiness-source="rust-service-read-model"
      data-ocentra-policy-readiness-state={resolvedPanel.loadState}
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
            {resolvePortalDevText(PortalDevTextToken.GetActivityAppGamePolicyReadinessReadModel)}
          </button>
        </header>
        <div
          className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
            PortalDom.Classes.ClassNameSeparator
          )}
        >
          <AppGamePolicyReadinessSummaryCard panel={resolvedPanel} />
          {resolvedPanel.rows.length === 0 ? (
            <AppGamePolicyReadinessEmptyCard panel={resolvedPanel} />
          ) : (
            resolvedPanel.rows.map((row, index) => (
              <AppGamePolicyReadinessRowCard key={`${String(row.title)}:${index}`} row={row} />
            ))
          )}
        </div>
      </div>
    </section>
  );
}

function AppGamePolicyReadinessSummaryCard({ panel }: { readonly panel: ParentAppGamePanelSnapshot }): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{PortalDetails.PolicyReadiness}</h2>
      <AppGamePolicyReadinessDetails details={panel.summaryDetails} />
    </article>
  );
}

function AppGamePolicyReadinessEmptyCard({ panel }: { readonly panel: ParentAppGamePanelSnapshot }): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{panel.loadState}</h2>
      <p>{panel.emptyMessage}</p>
      <AppGamePolicyReadinessDetails details={[{ label: PortalDetails.ProductClaim, value: panel.productClaim }]} />
    </article>
  );
}

function AppGamePolicyReadinessRowCard({ row }: { readonly row: ParentAppGamePanelRowSnapshot }): ReactElement {
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
  readonly details: readonly ParentAppGamePanelDetailSnapshot[];
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
