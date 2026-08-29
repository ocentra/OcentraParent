import type { ReactElement } from 'react';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/portal-domain/display-text';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import {
  isParentAppGameParentSurfaceRoute,
  type ParentAppGameActionRowSnapshot,
  type ParentAppGamePanelDetailSnapshot,
  type ParentAppGamePanelRowSnapshot,
  type ParentAppGameTimerParentSurfacePanelSnapshot,
  type ParentRouteId,
  type ParentUiActionPayload,
} from '../generated/parent-ui-bridge';
import { PortalDetails } from '@ocentra-parent/portal-domain/details';
import type { PortalRenderActions } from './portal-actions';

const EmptyTimerParentSurfacePanel: ParentAppGameTimerParentSurfacePanelSnapshot = {
  eyebrow: 'Rust-owned panel',
  title: 'App/game timer parent surface',
  body: 'Rust has not reported an app/game timer parent surface panel yet.',
  loadState: 'unavailable',
  summaryDetails: [{ label: PortalDetails.ProductClaim, value: 'Timer parent surface has not been reported yet.' }],
  parentActionRows: [],
  parentPreferenceSetupRows: [],
  localHandoffArtifactRows: [],
  rows: [],
  emptyMessage: 'No timer parent surface rows have been reported yet.',
  productClaim: 'Timer runtime, child handoff, and preference setup remain unreported.',
};

export function shouldRenderAppGameTimerParentSurfaceRoute(route: ParentRouteId): boolean {
  return isParentAppGameParentSurfaceRoute(route);
}

export function AppGameTimerParentSurfaceRoutePanel({
  actions,
  commandEnabled,
  panel,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly panel: ParentAppGameTimerParentSurfacePanelSnapshot | null;
}): ReactElement {
  const resolvedPanel = panel ?? EmptyTimerParentSurfacePanel;
  return (
    <section
      aria-label={resolvePortalDevText(PortalDevTextToken.AppGameTimerParentSurface)}
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
            {resolvePortalDevText(PortalDevTextToken.GetActivityAppGameTimerParentSurfaceReadModel)}
          </button>
        </header>
        <div
          className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
            PortalDom.Classes.ClassNameSeparator
          )}
        >
          <AppGameTimerParentSurfaceSummaryCard panel={resolvedPanel} />
          {resolvedPanel.parentActionRows.map((row, index) => (
            <AppGameTimerParentSurfaceRowCard key={`${String(row.title)}:parent-action:${index}`} row={row} />
          ))}
          {resolvedPanel.parentPreferenceSetupRows.map((row, index) => (
            <AppGameTimerParentSurfaceActionRowCard
              actions={actions}
              commandEnabled={commandEnabled}
              key={`${String(row.title)}:parent-preference:${index}`}
              row={row}
            />
          ))}
          {resolvedPanel.localHandoffArtifactRows.map((row, index) => (
            <AppGameTimerParentSurfaceRowCard key={`${String(row.title)}:local-artifact:${index}`} row={row} />
          ))}
          {resolvedPanel.rows.length === 0 ? (
            <AppGameTimerParentSurfaceEmptyCard panel={resolvedPanel} />
          ) : (
            resolvedPanel.rows.map((row, index) => (
              <AppGameTimerParentSurfaceRowCard key={`${String(row.title)}:service:${index}`} row={row} />
            ))
          )}
        </div>
      </div>
    </section>
  );
}

export function sendAppGameTimerParentPreferenceSetupAction(
  actions: PortalRenderActions,
  payload: ParentUiActionPayload
): void {
  void actions.requestAppGameTimerParentPreferenceSetup?.(payload);
}

function AppGameTimerParentSurfaceSummaryCard({
  panel,
}: {
  readonly panel: ParentAppGameTimerParentSurfacePanelSnapshot;
}): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{PortalDetails.RuntimeReference}</h2>
      <AppGameTimerParentSurfaceDetails details={panel.summaryDetails} />
    </article>
  );
}

function AppGameTimerParentSurfaceEmptyCard({
  panel,
}: {
  readonly panel: ParentAppGameTimerParentSurfacePanelSnapshot;
}): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{panel.loadState}</h2>
      <p>{panel.emptyMessage}</p>
      <AppGameTimerParentSurfaceDetails
        details={[
          {
            label: PortalDetails.ProductClaim,
            value: panel.productClaim,
          },
        ]}
      />
    </article>
  );
}

function AppGameTimerParentSurfaceRowCard({ row }: { readonly row: ParentAppGamePanelRowSnapshot }): ReactElement {
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

function AppGameTimerParentSurfaceActionRowCard({
  actions,
  commandEnabled = false,
  row,
}: {
  readonly actions?: PortalRenderActions;
  readonly commandEnabled?: boolean;
  readonly row: ParentAppGameActionRowSnapshot;
}): ReactElement {
  const className = [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
    PortalDom.Classes.ClassNameSeparator
  );
  return (
    <article className={className}>
      <h2>{row.title}</h2>
      <AppGameTimerParentSurfaceDetails details={row.details} />
      {row.actionLabel === null ||
      row.actionLabel === undefined ||
      row.actionPayload === null ||
      row.actionPayload === undefined ||
      actions === undefined ||
      actions.requestAppGameTimerParentPreferenceSetup === undefined ? null : (
        <button
          className={PortalDom.Classes.CommandResultTab}
          disabled={!commandEnabled}
          type={PortalDom.ButtonType.Button}
          onClick={() =>
            sendAppGameTimerParentPreferenceSetupAction(actions, row.actionPayload as ParentUiActionPayload)
          }
        >
          {row.actionLabel}
        </button>
      )}
    </article>
  );
}

function AppGameTimerParentSurfaceDetails({
  details,
}: {
  readonly details: readonly ParentAppGamePanelDetailSnapshot[];
}): ReactElement {
  return (
    <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
      {details.map((detail, index) => (
        <AppGameTimerParentSurfaceDetail
          key={`${String(detail.label)}:${index}`}
          label={detail.label}
          value={detail.value}
        />
      ))}
    </dl>
  );
}

function AppGameTimerParentSurfaceDetail({
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
