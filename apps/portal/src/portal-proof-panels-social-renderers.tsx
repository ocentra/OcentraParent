import type { ReactElement } from 'react';
import type { PortalProofPanelsRoutePanelProps } from './portal-proof-panels-renderers';
import { BrowserParentExplanationRoutePanel } from './BrowserParentExplanationRoutePanel';
import { SocialAlertReportRoutePanel } from './SocialAlertReportRoutePanel';
import { SocialAuditExplanationRoutePanel } from './SocialAuditExplanationRoutePanel';
import { SocialDashboardRoutePanel } from './SocialDashboardRoutePanel';

export function renderPortalProofPanelSocial({
  actions,
  activePanel,
  commandEnabled,
  browserActionIntentStreamStatusPanel,
  browserParentExplanationPanel,
  browserSocialProviderReceiptIngestionReadinessStatusPanel,
  browserSocialProviderReceiptStreamStatusPanel,
  socialAlertReportPanel,
  socialAlertReportParentSurfacePanel,
  socialAuditExplanationPanel,
  socialDashboardPanel,
  socialParentNotificationDeliveryPanel,
}: Pick<
  PortalProofPanelsRoutePanelProps,
  | 'actions'
  | 'activePanel'
  | 'commandEnabled'
  | 'browserActionIntentStreamStatusPanel'
  | 'browserParentExplanationPanel'
  | 'browserSocialProviderReceiptIngestionReadinessStatusPanel'
  | 'browserSocialProviderReceiptStreamStatusPanel'
  | 'socialAlertReportPanel'
  | 'socialAlertReportParentSurfacePanel'
  | 'socialAuditExplanationPanel'
  | 'socialDashboardPanel'
  | 'socialParentNotificationDeliveryPanel'
>): ReactElement {
  switch (activePanel) {
    case 'browser-parent-explanation':
      return <BrowserParentExplanationRoutePanel panel={browserParentExplanationPanel} />;
    case 'social-audit-explanation':
      return (
        <SocialAuditExplanationRoutePanel
          actions={actions}
          commandEnabled={commandEnabled}
          panel={socialAuditExplanationPanel}
        />
      );
    case 'social-dashboard':
      return (
        <SocialDashboardRoutePanel actions={actions} commandEnabled={commandEnabled} panel={socialDashboardPanel} />
      );
    case 'social-alert-report':
    default:
      return (
        <SocialAlertReportRoutePanel
          actions={actions}
          commandEnabled={commandEnabled}
          socialAlertReportPanel={socialAlertReportPanel}
          socialAlertReportParentSurfacePanel={socialAlertReportParentSurfacePanel}
          socialParentNotificationDeliveryPanel={socialParentNotificationDeliveryPanel}
          browserActionIntentStreamStatusPanel={browserActionIntentStreamStatusPanel}
          browserSocialProviderReceiptStreamStatusPanel={browserSocialProviderReceiptStreamStatusPanel}
          browserSocialProviderReceiptIngestionReadinessStatusPanel={
            browserSocialProviderReceiptIngestionReadinessStatusPanel
          }
        />
      );
  }
}
