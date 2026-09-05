import type { ReactElement } from 'react';
import type {
  ParentAppGameAdapterDispatchPanelSnapshot,
  ParentAppGameNotificationParentSurfacePanelSnapshot,
  ParentAppGamePanelSnapshot,
  ParentAppGameTimerParentSurfacePanelSnapshot,
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
    onSelectPanel,
  } satisfies PortalProofPanelsRoutePanelProps);
}
