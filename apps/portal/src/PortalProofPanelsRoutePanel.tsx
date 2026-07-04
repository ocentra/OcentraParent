import type { ReactElement } from 'react';
import type {
  ParentAppGameAdapterDispatchPanelSnapshot,
  ParentAppGameNotificationParentSurfacePanelSnapshot,
  ParentAppGamePanelSnapshot,
  ParentAppGameTimerParentSurfacePanelSnapshot,
  ParentBrowserPanelSnapshot,
  ParentNetworkEvidenceSummarySnapshot,
  ParentPolicyPreviewPanelSnapshot,
} from '../generated/parent-ui-bridge';
import type { PortalRenderActions } from './portal-actions';
import type { PortalLiveActivityState } from './live-activity-state';
import {
  renderPortalProofPanelsRoutePanel,
  usePortalProofPanelId,
  type PortalProofPanelsRoutePanelProps,
} from './portal-proof-panels-renderers';

export function PortalProofPanelsRoutePanel({
  actions,
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
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly liveActivity: PortalLiveActivityState;
  readonly networkEvidenceSummary: ParentNetworkEvidenceSummarySnapshot | null;
  readonly policyPreviewPanel: ParentPolicyPreviewPanelSnapshot | null;
  readonly appGameNotificationParentSurfacePanel: ParentAppGameNotificationParentSurfacePanelSnapshot | null;
  readonly appGamePolicyReadinessPanel: ParentAppGamePanelSnapshot | null;
  readonly appGamePlatformProofStatusPanel: ParentAppGamePanelSnapshot | null;
  readonly appGameChildRuntimeTransportReceiptPanel: ParentAppGamePanelSnapshot | null;
  readonly appGameAdapterDispatchPanel: ParentAppGameAdapterDispatchPanelSnapshot | null;
  readonly appGameTimerParentSurfacePanel: ParentAppGameTimerParentSurfacePanelSnapshot | null;
  readonly browserParentExplanationPanel: ParentBrowserPanelSnapshot | null;
  readonly socialAuditExplanationPanel: ParentBrowserPanelSnapshot | null;
  readonly socialDashboardPanel: ParentBrowserPanelSnapshot | null;
  readonly socialAlertReportPanel: ParentBrowserPanelSnapshot | null;
  readonly socialAlertReportParentSurfacePanel: ParentBrowserPanelSnapshot | null;
  readonly socialParentNotificationDeliveryPanel: ParentBrowserPanelSnapshot | null;
  readonly browserActionIntentStreamStatusPanel: ParentBrowserPanelSnapshot | null;
  readonly browserSocialProviderReceiptStreamStatusPanel: ParentBrowserPanelSnapshot | null;
  readonly browserSocialProviderReceiptIngestionReadinessStatusPanel: ParentBrowserPanelSnapshot | null;
}): ReactElement {
  const [activePanel, onSelectPanel] = usePortalProofPanelId();
  return renderPortalProofPanelsRoutePanel({
    actions,
    activePanel,
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
    browserParentExplanationPanel,
    socialAuditExplanationPanel,
    socialDashboardPanel,
    socialAlertReportPanel,
    socialAlertReportParentSurfacePanel,
    socialParentNotificationDeliveryPanel,
    browserActionIntentStreamStatusPanel,
    browserSocialProviderReceiptStreamStatusPanel,
    browserSocialProviderReceiptIngestionReadinessStatusPanel,
    onSelectPanel,
  } satisfies PortalProofPanelsRoutePanelProps);
}
