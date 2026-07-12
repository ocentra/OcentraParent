import type { ReactElement } from 'react';
import { ParentRoute } from '../generated/parent-ui-bridge';
import { NetworkEvidenceDrawerRoutePanel } from './NetworkEvidenceDrawerRoutePanel';
import { PolicyPreviewRoutePanel } from './PolicyPreviewRoutePanel';
import { TrackingStatusRoutePanel } from './TrackingStatusRoutePanel';
import { renderAppGameProofPanel } from './portal-proof-panels-app-game-renderers';
import { renderPortalProofPanelSocial } from './portal-proof-panels-social-renderers';
import type { PortalProofPanelsRoutePanelProps } from './portal-proof-panels-renderers';

export function renderPortalProofPanelContent({
  actions,
  activePanel,
  browserActionIntentStreamStatusPanel,
  browserParentExplanationPanel,
  browserSocialProviderReceiptIngestionReadinessStatusPanel,
  browserSocialProviderReceiptStreamStatusPanel,
  commandEnabled,
  liveActivity,
  networkEvidenceSummary,
  policyPreviewPanel,
  appGameNotificationParentSurfacePanel,
  appGamePolicyReadinessPanel,
  appGamePlatformProofStatusPanel,
  appGameChildRuntimeTransportReceiptPanel,
  appGameAdapterDispatchPanel,
  appGameTimerParentSurfacePanel,
  socialAlertReportPanel,
  socialAlertReportParentSurfacePanel,
  socialAuditExplanationPanel,
  socialDashboardPanel,
  socialParentNotificationDeliveryPanel,
}: Omit<PortalProofPanelsRoutePanelProps, 'onSelectPanel'>): ReactElement {
  if (
    activePanel === 'browser-parent-explanation' ||
    activePanel === 'social-audit-explanation' ||
    activePanel === 'social-dashboard' ||
    activePanel === 'social-alert-report'
  ) {
    return renderPortalProofPanelSocial({
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
    });
  }
  if (
    activePanel === 'app-game-notification-parent-surface' ||
    activePanel === 'app-game-policy-readiness' ||
    activePanel === 'app-game-platform-proof-status' ||
    activePanel === 'app-game-child-runtime-transport-receipt' ||
    activePanel === 'app-game-adapter-dispatch' ||
    activePanel === 'app-game-timer-parent-surface'
  ) {
    return renderAppGameProofPanel({
      actions,
      activePanel,
      commandEnabled,
      appGameNotificationParentSurfacePanel,
      appGamePolicyReadinessPanel,
      appGamePlatformProofStatusPanel,
      appGameChildRuntimeTransportReceiptPanel,
      appGameAdapterDispatchPanel,
      appGameTimerParentSurfacePanel,
    });
  }
  if (activePanel === 'network-activity') {
    return (
      <NetworkEvidenceDrawerRoutePanel
        liveActivity={liveActivity}
        networkEvidenceSummary={networkEvidenceSummary}
        route={ParentRoute.ProofPanels}
      />
    );
  }
  if (activePanel === 'policy-preview') {
    return <PolicyPreviewRoutePanel actions={actions} commandEnabled={commandEnabled} panel={policyPreviewPanel} />;
  }
  return <TrackingStatusRoutePanel actions={actions} commandEnabled={commandEnabled} liveActivity={liveActivity} />;
}
