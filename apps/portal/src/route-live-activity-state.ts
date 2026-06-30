import type { PortalLiveActivityState } from '@ocentra-parent/portal-domain/live-activity-state';
import type {
  ParentAppGameAdapterDispatchPanelSnapshot,
  ParentAppGameNotificationParentSurfacePanelSnapshot,
  ParentAppGamePanelSnapshot,
  ParentAppGameTimerParentSurfacePanelSnapshot,
  ParentNetworkEvidenceSummarySnapshot,
  ParentPolicyPreviewPanelSnapshot,
  ParentRouteLiveActivitySnapshot,
  ParentScreenSummaryPanelSnapshot,
} from '../generated/parent-ui-bridge';

type ResolvedPortalLiveActivityState = PortalLiveActivityState & {
  readonly screenSummaryPanel?: ParentScreenSummaryPanelSnapshot | null;
  readonly networkEvidenceSummary?: ParentNetworkEvidenceSummarySnapshot | null;
  readonly policyPreviewPanel?: ParentPolicyPreviewPanelSnapshot | null;
  readonly appGameNotificationParentSurfacePanel: ParentAppGameNotificationParentSurfacePanelSnapshot | null;
  readonly appGamePolicyReadinessPanel: ParentAppGamePanelSnapshot | null;
  readonly appGamePlatformProofStatusPanel: ParentAppGamePanelSnapshot | null;
  readonly appGameChildRuntimeTransportReceiptPanel: ParentAppGamePanelSnapshot | null;
  readonly appGameAdapterDispatchPanel?: ParentAppGameAdapterDispatchPanelSnapshot | null;
  readonly appGameTimerParentSurfacePanel?: ParentAppGameTimerParentSurfacePanelSnapshot | null;
};

export const EMPTY_ROUTE_LIVE_ACTIVITY_STATE = {
  activityServiceUiSpine: {
    dataOwner: 'rust-service-read-model',
    uiConsumer: 'c-owned-activity-ui',
    viteDataOwner: false,
    currentState: 'unavailable',
    report: null,
    reportHistory: null,
    familyAggregation: null,
    screen: null,
    appUse: null,
    browser: null,
    games: null,
    network: null,
  },
  ingestEvent: null,
  ingestStatus: null,
  recentSummaryEvent: null,
  recentSummary: null,
  browserEvidenceEvent: null,
  browserEvidenceReadModel: null,
  browserInventoryEvent: null,
  browserInventoryReadModel: null,
  browserManagedEvent: null,
  browserManagedStatus: null,
  localAiRuntimeStatusEvent: null,
  lanAiJobEvent: null,
  parentAssistantBoundaryEvent: null,
  activityMemoryGraphEvent: null,
  activityMemoryGraphReadModel: null,
  activityReportEvent: null,
  activityReport: null,
  activityReportHistoryEvent: null,
  activityReportHistory: null,
  activityScreenReadModelEvent: null,
  activityScreenReadModel: null,
  activityAppUseReadModelEvent: null,
  activityAppUseReadModel: null,
  activityBrowserReadModelEvent: null,
  activityBrowserReadModel: null,
  activityGamesReadModelEvent: null,
  activityGamesReadModel: null,
  appGameNotificationParentSurfacePanel: null,
  appGamePlatformProofStatusPanel: null,
  appGameChildRuntimeTransportReceiptPanel: null,
  activityNetworkReadModelEvent: null,
  activityNetworkReadModel: null,
  browserInterventionEvent: null,
  browserInterventionReadModel: null,
  networkFlowEvent: null,
  networkFlowReadModel: null,
  networkRuntimeEventChainEvent: null,
  networkRuntimeEventChainStream: null,
  networkRemoteDeliveryStatusEvent: null,
  networkRemoteDeliveryStatusResult: null,
  networkLiveCaptureStatusEvent: null,
  networkLiveCaptureStatusResult: null,
  networkLinuxNftablesLabStatusEvent: null,
  networkLinuxNftablesLabStatusResult: null,
  networkWindowsFirewallLabStatusEvent: null,
  networkWindowsFirewallLabStatusResult: null,
  networkWindowsWfpGateStatusEvent: null,
  networkWindowsWfpGateStatusResult: null,
  networkAndroidVpnServiceGateStatusEvent: null,
  networkAndroidVpnServiceGateStatusResult: null,
  networkAppleNetworkExtensionGateStatusEvent: null,
  networkAppleNetworkExtensionGateStatusResult: null,
  activityTrackingReadModelEvent: null,
  activityTrackingReadModel: null,
  activityTrackingPanel: null,
  activityTrackingRetentionSettingsWriteEvent: null,
  activityTrackingRetentionSettingsWriteResult: null,
  lanPairingStatusEvent: null,
  lanPairingBrowserDiscoveryEvent: null,
  lanAddDeviceReadModel: null,
  appGamePolicyReadinessPanel: null,
} satisfies ResolvedPortalLiveActivityState;

export function resolveSnapshotLiveActivityState(
  snapshot?: ParentRouteLiveActivitySnapshot | null
): ResolvedPortalLiveActivityState {
  if (snapshot === null || snapshot === undefined) {
    return EMPTY_ROUTE_LIVE_ACTIVITY_STATE;
  }
  // The Rust-owned bridge snapshot is structurally compatible here even when
  // the generated TS surface widens some fields to unknown records.
  const resolvedSnapshot = snapshot as Partial<ResolvedPortalLiveActivityState>;
  return {
    ...EMPTY_ROUTE_LIVE_ACTIVITY_STATE,
    ...resolvedSnapshot,
  };
}
