import type { ReactElement } from 'react';
import { ParentRoute } from '../generated/parent-ui-bridge';
import { NetworkEvidenceDrawerRoutePanel } from './NetworkEvidenceDrawerRoutePanel';
import { PolicyPreviewRoutePanel } from './PolicyPreviewRoutePanel';
import { TrackingStatusRoutePanel } from './TrackingStatusRoutePanel';
import { renderAppGameProofPanel } from './portal-proof-panels-app-game-renderers';
import type { PortalProofPanelsRoutePanelProps } from './portal-proof-panels-renderers';

export function renderPortalProofPanelContent({
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
}: Omit<PortalProofPanelsRoutePanelProps, 'onSelectPanel'>): ReactElement {
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
        actions={actions}
        commandEnabled={commandEnabled}
        liveActivity={liveActivity}
        networkEvidenceSummary={networkEvidenceSummary}
        route={ParentRoute.ProofPanels}
      />
    );
  }
  if (activePanel === 'policy-preview') {
    return (
      <PolicyPreviewRoutePanel
        actions={actions}
        commandEnabled={commandEnabled}
        panel={policyPreviewPanel}
        route={ParentRoute.ProofPanels}
      />
    );
  }
  return (
    <TrackingStatusRoutePanel
      actions={actions}
      commandEnabled={commandEnabled}
      liveActivity={liveActivity}
      showUnavailable
    />
  );
}
