import { useState, type ReactElement } from 'react';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { ParentRoute, type ParentBrowserPanelSnapshot, type ParentRouteId } from '../generated/parent-ui-bridge';
import { BrowserParentExplanationRoutePanel } from './BrowserParentExplanationRoutePanel';
import { SocialAlertReportRoutePanel } from './SocialAlertReportRoutePanel';
import { SocialAuditExplanationRoutePanel } from './SocialAuditExplanationRoutePanel';
import { SocialDashboardRoutePanel } from './SocialDashboardRoutePanel';
import type { PortalRenderActions } from './portal-actions';

const BrowserReviewPanel = {
  BrowserExplanation: 'browser-explanation',
  SocialDashboard: 'social-dashboard',
  SocialExplanations: 'social-explanations',
  SocialAlerts: 'social-alerts',
} as const;

type BrowserReviewPanelId = (typeof BrowserReviewPanel)[keyof typeof BrowserReviewPanel];

const BrowserReviewTabs: ReadonlyArray<{
  readonly id: BrowserReviewPanelId;
  readonly label: string;
}> = [
  { id: BrowserReviewPanel.SocialDashboard, label: 'Social dashboard' },
  { id: BrowserReviewPanel.SocialExplanations, label: 'Explanations' },
  { id: BrowserReviewPanel.SocialAlerts, label: 'Alerts & delivery' },
  { id: BrowserReviewPanel.BrowserExplanation, label: 'Browser explanation' },
];

const BROWSER_REVIEW_ROUTE_PANEL_CLASSES = {
  RoutePanel: 'browser-review-route-panel',
  Tabs: 'browser-review-route-tabs',
  Content: 'browser-review-route-content',
} as const;

export type BrowserReviewRoutePanelProps = {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly browserParentExplanationPanel: ParentBrowserPanelSnapshot | null;
  readonly socialAuditExplanationPanel: ParentBrowserPanelSnapshot | null;
  readonly socialDashboardPanel: ParentBrowserPanelSnapshot | null;
  readonly socialAlertReportPanel: ParentBrowserPanelSnapshot | null;
  readonly socialAlertReportParentSurfacePanel: ParentBrowserPanelSnapshot | null;
  readonly socialParentNotificationDeliveryPanel: ParentBrowserPanelSnapshot | null;
  readonly browserActionIntentStreamStatusPanel: ParentBrowserPanelSnapshot | null;
  readonly browserSocialProviderReceiptStreamStatusPanel: ParentBrowserPanelSnapshot | null;
  readonly browserSocialProviderReceiptIngestionReadinessStatusPanel: ParentBrowserPanelSnapshot | null;
};

export function shouldRenderBrowserReviewRoute(route: ParentRouteId): boolean {
  return route === ParentRoute.Browser;
}

export function BrowserReviewRoutePanel(props: BrowserReviewRoutePanelProps): ReactElement {
  const [activePanel, setActivePanel] = useState<BrowserReviewPanelId>(BrowserReviewPanel.SocialDashboard);
  return (
    <section aria-label="Social review" className={BROWSER_REVIEW_ROUTE_PANEL_CLASSES.RoutePanel}>
      <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
        <p className={PortalDom.Classes.ProductEyebrow}>Browser</p>
        <h2>Browser &amp; social review</h2>
        <p>Review browser evidence, social decisions, alerts, and delivery readiness from the parent service.</p>
      </header>
      <div aria-label="Browser review sections" className={BROWSER_REVIEW_ROUTE_PANEL_CLASSES.Tabs} role="toolbar">
        {BrowserReviewTabs.map((tab) => (
          <button
            aria-pressed={tab.id === activePanel}
            className={PortalDom.Classes.CommandResultTab}
            data-active={tab.id === activePanel ? PortalDom.Attributes.True : undefined}
            key={tab.id}
            onClick={() => setActivePanel(tab.id)}
            type={PortalDom.ButtonType.Button}
          >
            {tab.label}
          </button>
        ))}
      </div>
      <div className={BROWSER_REVIEW_ROUTE_PANEL_CLASSES.Content}>{renderBrowserReviewPanel(activePanel, props)}</div>
    </section>
  );
}

function renderBrowserReviewPanel(
  activePanel: BrowserReviewPanelId,
  props: BrowserReviewRoutePanelProps
): ReactElement {
  if (activePanel === BrowserReviewPanel.BrowserExplanation) {
    return <BrowserParentExplanationRoutePanel panel={props.browserParentExplanationPanel} />;
  }
  if (activePanel === BrowserReviewPanel.SocialExplanations) {
    return (
      <SocialAuditExplanationRoutePanel
        actions={props.actions}
        commandEnabled={props.commandEnabled}
        panel={props.socialAuditExplanationPanel}
      />
    );
  }
  if (activePanel === BrowserReviewPanel.SocialAlerts) {
    return (
      <SocialAlertReportRoutePanel
        actions={props.actions}
        commandEnabled={props.commandEnabled}
        socialAlertReportPanel={props.socialAlertReportPanel}
        socialAlertReportParentSurfacePanel={props.socialAlertReportParentSurfacePanel}
        socialParentNotificationDeliveryPanel={props.socialParentNotificationDeliveryPanel}
        browserActionIntentStreamStatusPanel={props.browserActionIntentStreamStatusPanel}
        browserSocialProviderReceiptStreamStatusPanel={props.browserSocialProviderReceiptStreamStatusPanel}
        browserSocialProviderReceiptIngestionReadinessStatusPanel={
          props.browserSocialProviderReceiptIngestionReadinessStatusPanel
        }
      />
    );
  }
  return (
    <SocialDashboardRoutePanel
      actions={props.actions}
      commandEnabled={props.commandEnabled}
      panel={props.socialDashboardPanel}
    />
  );
}
