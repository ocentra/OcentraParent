import { type ReactElement } from 'react';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import {
  isParentBrowserParentSurfaceRoute,
  type ParentBrowserPanelDetailSnapshot,
  type ParentBrowserPanelRowSnapshot,
  type ParentBrowserPanelSnapshot,
  type ParentRouteId,
} from '../generated/parent-ui-bridge';
import type { PortalRenderActions } from './portal-actions';

export function shouldRenderSocialAlertReportRoute(route: ParentRouteId): boolean {
  return isParentBrowserParentSurfaceRoute(route);
}

export function SocialAlertReportRoutePanel({
  actions,
  commandEnabled,
  socialAlertReportPanel,
  socialAlertReportParentSurfacePanel,
  socialParentNotificationDeliveryPanel,
  browserActionIntentStreamStatusPanel,
  browserSocialProviderReceiptStreamStatusPanel,
  browserSocialProviderReceiptIngestionReadinessStatusPanel,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly socialAlertReportPanel: ParentBrowserPanelSnapshot | null;
  readonly socialAlertReportParentSurfacePanel: ParentBrowserPanelSnapshot | null;
  readonly socialParentNotificationDeliveryPanel: ParentBrowserPanelSnapshot | null;
  readonly browserActionIntentStreamStatusPanel: ParentBrowserPanelSnapshot | null;
  readonly browserSocialProviderReceiptStreamStatusPanel: ParentBrowserPanelSnapshot | null;
  readonly browserSocialProviderReceiptIngestionReadinessStatusPanel: ParentBrowserPanelSnapshot | null;
}): ReactElement {
  if (socialAlertReportPanel === null) {
    return <SocialAlertReportUnavailableSection />;
  }

  return (
    <section aria-label={socialAlertReportPanel.title} className={PortalDom.Classes.TrackingStatusOverlay}>
      <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
        <SocialAlertReportHeader
          actions={actions}
          commandEnabled={commandEnabled}
          panel={socialAlertReportPanel}
          parentSurfacePanel={socialAlertReportParentSurfacePanel}
          notificationPanel={socialParentNotificationDeliveryPanel}
        />
        <div
          className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
            PortalDom.Classes.ClassNameSeparator
          )}
        >
          <SocialAlertReportSummaryCard panel={socialAlertReportPanel} />
          {socialAlertReportPanel.rows.length === 0 ? (
            <SocialAlertReportEmptyCard panel={socialAlertReportPanel} />
          ) : (
            socialAlertReportPanel.rows.map((row) => <SocialAlertReportRowCard key={row.key} row={row} />)
          )}
          <SupplementalBrowserPanelCards panel={socialParentNotificationDeliveryPanel} />
          <SupplementalBrowserPanelCards panel={socialAlertReportParentSurfacePanel} />
          <SupplementalBrowserPanelCards panel={browserActionIntentStreamStatusPanel} />
          <SupplementalBrowserPanelCards panel={browserSocialProviderReceiptStreamStatusPanel} />
          <SupplementalBrowserPanelCards panel={browserSocialProviderReceiptIngestionReadinessStatusPanel} />
        </div>
      </div>
    </section>
  );
}

function SocialAlertReportUnavailableSection(): ReactElement {
  return (
    <section aria-label="Social alerts and reports unavailable" className={PortalDom.Classes.TrackingStatusOverlay}>
      <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <p className={PortalDom.Classes.ProductEyebrow}>Browser route</p>
          <h2>Social alerts and reports unavailable</h2>
          <p>Parent Rust snapshot unavailable for the social alert/report route.</p>
        </header>
      </div>
    </section>
  );
}

function SocialAlertReportHeader({
  actions,
  commandEnabled,
  panel,
  parentSurfacePanel,
  notificationPanel,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly panel: ParentBrowserPanelSnapshot;
  readonly parentSurfacePanel: ParentBrowserPanelSnapshot | null;
  readonly notificationPanel: ParentBrowserPanelSnapshot | null;
}): ReactElement {
  return (
    <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
      <p className={PortalDom.Classes.ProductEyebrow}>{panel.eyebrow}</p>
      <h2>{panel.title}</h2>
      <p>{panel.body}</p>
      <SocialAlertReportRefreshButton actions={actions} commandEnabled={commandEnabled} label={panel.title} />
      <SocialAlertReportRefreshButton
        actions={actions}
        commandEnabled={commandEnabled}
        label={notificationPanel?.title ?? 'Social parent notification delivery readiness'}
      />
      <SocialAlertReportRefreshButton
        actions={actions}
        commandEnabled={commandEnabled}
        label={parentSurfacePanel?.title ?? 'Social parent surface status'}
      />
    </header>
  );
}

function SocialAlertReportRefreshButton({
  actions,
  commandEnabled,
  label,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly label: string;
}): ReactElement {
  return (
    <button
      className={PortalDom.Classes.CommandResultTab}
      disabled={!commandEnabled}
      onClick={() => void actions.refreshRouteSnapshot?.()}
      type={PortalDom.ButtonType.Button}
    >
      {label}
    </button>
  );
}

function SocialAlertReportSummaryCard({ panel }: { readonly panel: ParentBrowserPanelSnapshot }): ReactElement {
  return (
    <article className={cardClassName()}>
      <h2>{panel.summary}</h2>
      <SocialAlertReportDetails details={panel.summaryDetails} />
    </article>
  );
}

function SocialAlertReportEmptyCard({ panel }: { readonly panel: ParentBrowserPanelSnapshot }): ReactElement {
  return (
    <article className={cardClassName()}>
      <h2>{panel.emptyMessage}</h2>
      <SocialAlertReportDetails details={panel.summaryDetails} />
    </article>
  );
}

function SocialAlertReportRowCard({ row }: { readonly row: ParentBrowserPanelRowSnapshot }): ReactElement {
  return (
    <article className={cardClassName()}>
      <h2>{row.title}</h2>
      <SocialAlertReportDetails details={row.details} />
    </article>
  );
}

function SupplementalBrowserPanelCards({ panel }: { readonly panel: ParentBrowserPanelSnapshot | null }): ReactElement {
  if (panel === null) {
    return <></>;
  }

  return (
    <>
      <article className={cardClassName()}>
        <h2>{panel.title}</h2>
        <p>{panel.summary}</p>
        <SocialAlertReportDetails details={panel.summaryDetails} />
      </article>
      {panel.rows.map((row) => (
        <article className={cardClassName()} key={row.key}>
          <h2>{row.title}</h2>
          <SocialAlertReportDetails details={row.details} />
        </article>
      ))}
    </>
  );
}

function SocialAlertReportDetails({
  details,
}: {
  readonly details: readonly ParentBrowserPanelDetailSnapshot[];
}): ReactElement {
  return (
    <dl>
      {details.map((detail, index) => (
        <div key={`${detail.label}-${index}`}>
          <dt>{detail.label}</dt>
          <dd>{detail.value}</dd>
        </div>
      ))}
    </dl>
  );
}

function cardClassName(): string {
  return [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(PortalDom.Classes.ClassNameSeparator);
}
