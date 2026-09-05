import type { ReactElement } from 'react';
import {
  decodeDisplayText,
  PortalDevTextToken,
  resolvePortalDevText,
} from '@ocentra-parent/portal-domain/display-text';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalDetails } from '@ocentra-parent/portal-domain/details';
import {
  isParentScreenSummaryRoute,
  type ParentRouteId,
  type ParentScreenSummaryPanelDetailSnapshot,
  type ParentScreenSummaryPanelSnapshot,
} from '../generated/parent-ui-bridge';
import type { PortalRenderActions } from './portal-actions';

const SCREEN_SUMMARY_TEXT = {
  refresh: decodeDisplayText('Refresh screen activity'),
  unavailableTitle: decodeDisplayText('Screen activity unavailable'),
  unavailableBody: decodeDisplayText(
    'No screen summary read model has been reported. No screen content, activity, or capability state is inferred.'
  ),
  unavailableActivityTitle: decodeDisplayText('Activity rows'),
  unavailableActivityBody: decodeDisplayText(
    'No screen activity rows, totals, or observation time are shown without a service-owned read model.'
  ),
  unavailableCapabilityTitle: decodeDisplayText('Analysis capability'),
  unavailableCapabilityBody: decodeDisplayText(
    'No capture, analysis, visibility, or platform capability is inferred from an unavailable snapshot.'
  ),
  unavailableEvidenceTitle: decodeDisplayText('Evidence custody'),
  unavailableEvidenceBody: decodeDisplayText(
    'No evidence reference, retention state, custody label, or policy result has been reported.'
  ),
  notReported: decodeDisplayText('Not reported'),
} as const;

export function shouldRenderScreenSummaryRoute(route: ParentRouteId): boolean {
  return isParentScreenSummaryRoute(route);
}

export function ScreenSummaryRoutePanelMount({
  actions,
  commandEnabled,
  panel,
  route,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly panel: ParentScreenSummaryPanelSnapshot | null;
  readonly route: ParentRouteId;
}): ReactElement | null {
  return shouldRenderScreenSummaryRoute(route) ? (
    <ScreenSummaryRoutePanel actions={actions} commandEnabled={commandEnabled} panel={panel} />
  ) : null;
}

const EMPTY_SCREEN_SUMMARY_PANEL: ParentScreenSummaryPanelSnapshot = {
  eyebrow: 'Local service read model',
  title: 'Screen analysis',
  body: SCREEN_SUMMARY_TEXT.unavailableBody,
  loadState: 'Unavailable',
  summaryDetails: [],
  rows: [],
  emptyMessage: SCREEN_SUMMARY_TEXT.unavailableTitle,
  productClaim: 'No screen summary read model has been reported.',
};

export function ScreenSummaryRoutePanel({
  actions,
  commandEnabled,
  panel,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly panel: ParentScreenSummaryPanelSnapshot | null;
}): ReactElement {
  const resolvedPanel = panel ?? EMPTY_SCREEN_SUMMARY_PANEL;
  const empty = resolvedPanel.rows.length === 0;
  const reported = panel !== null;
  const routeAction = screenSummaryRouteAction(actions, commandEnabled);
  return (
    <section
      aria-label={resolvePortalDevText(PortalDevTextToken.ScreenAnalysis)}
      className={PortalDom.Classes.TrackingStatusOverlay}
      data-ocentra-screen-summary-surface="product"
      data-ocentra-screen-summary-empty={String(empty)}
      data-ocentra-screen-summary-panel=""
      data-ocentra-screen-summary-reported={String(reported)}
      data-ocentra-screen-summary-state={resolvedPanel.loadState}
    >
      <ScreenSummaryContent panel={resolvedPanel} reported={reported} routeAction={routeAction} />
    </section>
  );
}

function ScreenSummaryContent({
  panel,
  reported,
  routeAction,
}: {
  readonly panel: ParentScreenSummaryPanelSnapshot;
  readonly reported: boolean;
  readonly routeAction: Readonly<{ label: string; run(): void }>;
}): ReactElement {
  return (
    <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
      <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
        <p className={PortalDom.Classes.ProductEyebrow}>{panel.eyebrow}</p>
        <h2>{panel.title}</h2>
        <p>{panel.body}</p>
        <button
          className={PortalDom.Classes.CommandResultTab}
          onClick={routeAction.run}
          type={PortalDom.ButtonType.Button}
        >
          {routeAction.label}
        </button>
      </header>
      <ScreenSummaryCards panel={panel} reported={reported} />
    </div>
  );
}

function ScreenSummaryCards({
  panel,
  reported,
}: {
  readonly panel: ParentScreenSummaryPanelSnapshot;
  readonly reported: boolean;
}): ReactElement {
  return (
    <div
      className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
        PortalDom.Classes.ClassNameSeparator
      )}
    >
      {reported ? (
        <>
          <ScreenSummaryCard title={PortalDetails.Status} details={panel.summaryDetails} />
          {panel.rows.length === 0 ? (
            <ScreenSummaryCard
              title={panel.emptyMessage}
              details={[
                { label: PortalDetails.Status, value: panel.loadState },
                { label: PortalDetails.ProductClaim, value: panel.productClaim },
              ]}
            />
          ) : null}
          {panel.rows.map((row) => (
            <ScreenSummaryCard key={String(row.title)} title={row.title} details={row.details} />
          ))}
        </>
      ) : (
        <ScreenSummaryUnavailableCards panel={panel} />
      )}
    </div>
  );
}

function ScreenSummaryUnavailableCards({ panel }: { readonly panel: ParentScreenSummaryPanelSnapshot }): ReactElement {
  return (
    <>
      <ScreenSummaryCard
        title={SCREEN_SUMMARY_TEXT.unavailableActivityTitle}
        body={SCREEN_SUMMARY_TEXT.unavailableActivityBody}
        details={[
          { label: PortalDetails.Status, value: panel.loadState },
          { label: PortalDetails.ReadModelRows, value: '0' },
        ]}
      />
      <ScreenSummaryCard
        title={SCREEN_SUMMARY_TEXT.unavailableCapabilityTitle}
        body={SCREEN_SUMMARY_TEXT.unavailableCapabilityBody}
        details={[
          { label: PortalDetails.Capability, value: SCREEN_SUMMARY_TEXT.notReported },
          { label: PortalDetails.ProductClaim, value: panel.productClaim },
        ]}
      />
      <ScreenSummaryCard
        title={SCREEN_SUMMARY_TEXT.unavailableEvidenceTitle}
        body={SCREEN_SUMMARY_TEXT.unavailableEvidenceBody}
        details={[
          { label: PortalDetails.Custody, value: SCREEN_SUMMARY_TEXT.notReported },
          { label: PortalDetails.EvidenceReferences, value: SCREEN_SUMMARY_TEXT.notReported },
        ]}
      />
    </>
  );
}

function screenSummaryRouteAction(
  actions: PortalRenderActions,
  commandEnabled: boolean
): { readonly label: string; readonly run: () => void } {
  if (!commandEnabled || actions.refreshRouteSnapshot === undefined) {
    return { label: resolvePortalDevText(PortalDevTextToken.RetryStatus), run: actions.reconnect };
  }
  return {
    label: SCREEN_SUMMARY_TEXT.refresh,
    run: () => {
      void actions.refreshRouteSnapshot?.();
    },
  };
}

function ScreenSummaryCard({
  body,
  details,
  title,
}: {
  readonly body?: string;
  readonly details: readonly ParentScreenSummaryPanelDetailSnapshot[];
  readonly title: string;
}): ReactElement {
  return (
    <article className={screenSummaryCardClassName()}>
      <h2>{title}</h2>
      {body === undefined ? null : <p>{body}</p>}
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        {details.map((screenDetail) => (
          <ScreenSummaryDetail key={String(screenDetail.label)} label={screenDetail.label} value={screenDetail.value} />
        ))}
      </dl>
    </article>
  );
}

function ScreenSummaryDetail({ label, value }: { readonly label: string; readonly value: string }): ReactElement {
  return (
    <div>
      <dt>{label}</dt>
      <dd>{value}</dd>
    </div>
  );
}

function screenSummaryCardClassName(): string {
  return [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(PortalDom.Classes.ClassNameSeparator);
}
