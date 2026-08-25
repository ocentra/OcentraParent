import type {
  GeneratedPortalAgentActivitySurfaceAdapterFailureReason,
  GeneratedPortalActivityHistoricalReportListSnapshot,
  GeneratedPortalActivityIngestStatusSnapshot,
  GeneratedPortalActivityReadModelState,
  GeneratedPortalActivityRecentSummarySnapshot,
  GeneratedPortalActivityReportDocumentSnapshot,
  GeneratedPortalActivitySurfaceReadModelSnapshot,
  GeneratedPortalBrowserEvidenceReadModelSnapshot,
  GeneratedPortalBrowserInterventionReadModelSnapshot,
  GeneratedPortalBrowserInterventionRowSnapshot,
  GeneratedPortalBrowserInventoryReadModelSnapshot,
  GeneratedPortalBrowserManagedSessionStatusSnapshot,
  GeneratedPortalNetworkLiveCaptureStatusSnapshot,
  GeneratedPortalNetworkPlatformGateStatusSnapshot,
  GeneratedPortalNetworkRemoteDeliveryStatusSnapshot,
  GeneratedPortalNetworkRuntimeEventPayload,
  GeneratedPortalNetworkRuntimeEventType,
} from './generated-portal-contracts';
import type { PortalActivityMemoryGraphReadModel } from './activity-memory-graph';
import type { PortalLanAddDeviceReadModel } from './live-activity-lan-add-device';
import type { networkEvidenceDrawerSummary } from './network-evidence-drawer';
import type { PortalRouteEventRecord } from './portal-contract-adapter';
import type { ParsedPayloadResult, ReadModelResult } from './read-model-result';
import { GeneratedPortalTrackingContracts as GeneratedPortalTrackingContractsValue } from './generated-portal-contracts';

type ActivitySurfaceReadModel = GeneratedPortalActivitySurfaceReadModelSnapshot;
void GeneratedPortalTrackingContractsValue;
type PortalActivitySurfaceAdapterState = GeneratedPortalActivityReadModelState;
type PortalActivitySurfaceAdapterResult<TValue> =
  | {
      readonly ok: true;
      readonly state: PortalActivitySurfaceAdapterState;
      readonly value: TValue;
    }
  | {
      readonly ok: false;
      readonly state: PortalActivitySurfaceAdapterState;
      readonly reason: string;
    };
type PortalActivityServiceUiSpine = {
  readonly dataOwner: 'rust-service-read-model';
  readonly uiConsumer: 'c-owned-activity-ui';
  readonly viteDataOwner: false;
  readonly currentState: PortalActivitySurfaceAdapterState;
  readonly report: PortalActivitySurfaceAdapterResult<GeneratedPortalActivityReportDocumentSnapshot> | null;
  readonly reportHistory: PortalActivitySurfaceAdapterResult<GeneratedPortalActivityHistoricalReportListSnapshot> | null;
  readonly screen: PortalActivitySurfaceAdapterResult<ActivitySurfaceReadModel> | null;
  readonly appUse: PortalActivitySurfaceAdapterResult<ActivitySurfaceReadModel> | null;
  readonly browser: PortalActivitySurfaceAdapterResult<ActivitySurfaceReadModel> | null;
  readonly games: PortalActivitySurfaceAdapterResult<ActivitySurfaceReadModel> | null;
  readonly network: PortalActivitySurfaceAdapterResult<ActivitySurfaceReadModel> | null;
};
type PortalNetworkRuntimeEventResult =
  | {
      readonly ok: true;
      readonly eventType: GeneratedPortalNetworkRuntimeEventType;
      readonly value: GeneratedPortalNetworkRuntimeEventPayload;
    }
  | {
      readonly ok: false;
      readonly reason: string;
    };
type PortalStatusParseResult<TStatus> =
  | {
      readonly ok: true;
      readonly status: TStatus;
    }
  | {
      readonly ok: false;
      readonly reason: string;
    };
type PortalNetworkFlowReadModel = NonNullable<Parameters<typeof networkEvidenceDrawerSummary>[0]>;
type PortalActivityTrackingReadModelSnapshot = NonNullable<
  ReturnType<typeof GeneratedPortalTrackingContractsValue.ActivityTrackingReadModel.decode>
>;
type PortalTrackingRetentionSettingsWriteResultSnapshot = NonNullable<
  ReturnType<typeof GeneratedPortalTrackingContractsValue.RetentionSettingsWrite.Result.decode>
>;
type PortalTrackingRetentionSettingsWriteResultParseResult =
  ParsedPayloadResult<PortalTrackingRetentionSettingsWriteResultSnapshot>;

export interface PortalNetworkRuntimeEventChainStream {
  readonly streamedEventCount: number | null;
  readonly events: readonly PortalNetworkRuntimeEventResult[];
  readonly invalidEventCount: number;
}

export type PortalActivityTrackingReadModelFailureReason = GeneratedPortalAgentActivitySurfaceAdapterFailureReason;

export type PortalActivityTrackingReadModelResult = ReadModelResult<
  PortalActivityTrackingReadModelSnapshot,
  PortalActivityTrackingReadModelFailureReason
>;

export type PortalBrowserInterventionReadModel = GeneratedPortalBrowserInterventionReadModelSnapshot;
export type PortalBrowserInterventionRow = GeneratedPortalBrowserInterventionRowSnapshot;

export interface PortalLiveActivityState {
  readonly activityServiceUiSpine: PortalActivityServiceUiSpine;
  readonly ingestEvent: PortalRouteEventRecord | null;
  readonly ingestStatus: GeneratedPortalActivityIngestStatusSnapshot | null;
  readonly recentSummaryEvent: PortalRouteEventRecord | null;
  readonly recentSummary: GeneratedPortalActivityRecentSummarySnapshot | null;
  readonly browserEvidenceEvent: PortalRouteEventRecord | null;
  readonly browserEvidenceReadModel: GeneratedPortalBrowserEvidenceReadModelSnapshot | null;
  readonly browserInventoryEvent: PortalRouteEventRecord | null;
  readonly browserInventoryReadModel: GeneratedPortalBrowserInventoryReadModelSnapshot | null;
  readonly browserManagedEvent: PortalRouteEventRecord | null;
  readonly browserManagedStatus: GeneratedPortalBrowserManagedSessionStatusSnapshot | null;
  readonly localAiRuntimeStatusEvent: PortalRouteEventRecord | null;
  readonly lanAiJobEvent: PortalRouteEventRecord | null;
  readonly parentAssistantBoundaryEvent: PortalRouteEventRecord | null;
  readonly activityMemoryGraphEvent: PortalRouteEventRecord | null;
  readonly activityMemoryGraphReadModel: PortalActivityMemoryGraphReadModel | null;
  readonly activityReportEvent: PortalRouteEventRecord | null;
  readonly activityReport: PortalActivitySurfaceAdapterResult<GeneratedPortalActivityReportDocumentSnapshot> | null;
  readonly activityReportHistoryEvent: PortalRouteEventRecord | null;
  readonly activityReportHistory: PortalActivitySurfaceAdapterResult<GeneratedPortalActivityHistoricalReportListSnapshot> | null;
  readonly activityScreenReadModelEvent: PortalRouteEventRecord | null;
  readonly activityScreenReadModel: PortalActivitySurfaceAdapterResult<ActivitySurfaceReadModel> | null;
  readonly activityAppUseReadModelEvent: PortalRouteEventRecord | null;
  readonly activityAppUseReadModel: PortalActivitySurfaceAdapterResult<ActivitySurfaceReadModel> | null;
  readonly activityBrowserReadModelEvent: PortalRouteEventRecord | null;
  readonly activityBrowserReadModel: PortalActivitySurfaceAdapterResult<ActivitySurfaceReadModel> | null;
  readonly activityGamesReadModelEvent: PortalRouteEventRecord | null;
  readonly activityGamesReadModel: PortalActivitySurfaceAdapterResult<ActivitySurfaceReadModel> | null;
  readonly activityAppGamePlatformExtensionReadModel: unknown | null;
  readonly appGameNotificationParentSurfacePanel: unknown | null;
  readonly appGamePlatformProofStatusPanel: unknown | null;
  readonly appGameChildRuntimeTransportReceiptPanel: unknown | null;
  readonly activityNetworkReadModelEvent: PortalRouteEventRecord | null;
  readonly activityNetworkReadModel: PortalActivitySurfaceAdapterResult<ActivitySurfaceReadModel> | null;
  readonly browserInterventionEvent: PortalRouteEventRecord | null;
  readonly browserInterventionReadModel: GeneratedPortalBrowserInterventionReadModelSnapshot | null;
  readonly networkFlowEvent: PortalRouteEventRecord | null;
  readonly networkFlowReadModel: PortalNetworkFlowReadModel | null;
  readonly networkRuntimeEventChainEvent: PortalRouteEventRecord | null;
  readonly networkRuntimeEventChainStream: PortalNetworkRuntimeEventChainStream | null;
  readonly networkRemoteDeliveryStatusEvent: PortalRouteEventRecord | null;
  readonly networkRemoteDeliveryStatusResult: PortalStatusParseResult<GeneratedPortalNetworkRemoteDeliveryStatusSnapshot> | null;
  readonly networkLiveCaptureStatusEvent: PortalRouteEventRecord | null;
  readonly networkLiveCaptureStatusResult: PortalStatusParseResult<GeneratedPortalNetworkLiveCaptureStatusSnapshot> | null;
  readonly networkLinuxNftablesLabStatusEvent: PortalRouteEventRecord | null;
  readonly networkLinuxNftablesLabStatusResult: PortalStatusParseResult<GeneratedPortalNetworkPlatformGateStatusSnapshot> | null;
  readonly networkWindowsFirewallLabStatusEvent: PortalRouteEventRecord | null;
  readonly networkWindowsFirewallLabStatusResult: PortalStatusParseResult<GeneratedPortalNetworkPlatformGateStatusSnapshot> | null;
  readonly networkWindowsWfpGateStatusEvent: PortalRouteEventRecord | null;
  readonly networkWindowsWfpGateStatusResult: PortalStatusParseResult<GeneratedPortalNetworkPlatformGateStatusSnapshot> | null;
  readonly networkAndroidVpnServiceGateStatusEvent: PortalRouteEventRecord | null;
  readonly networkAndroidVpnServiceGateStatusResult: PortalStatusParseResult<GeneratedPortalNetworkPlatformGateStatusSnapshot> | null;
  readonly networkAppleNetworkExtensionGateStatusEvent: PortalRouteEventRecord | null;
  readonly networkAppleNetworkExtensionGateStatusResult: PortalStatusParseResult<GeneratedPortalNetworkPlatformGateStatusSnapshot> | null;
  readonly activityTrackingReadModelEvent: PortalRouteEventRecord | null;
  readonly activityTrackingReadModel: PortalActivityTrackingReadModelResult | null;
  readonly activityTrackingPanel: unknown | null;
  readonly activityTrackingRetentionSettingsWriteEvent: PortalRouteEventRecord | null;
  readonly activityTrackingRetentionSettingsWriteResult: PortalTrackingRetentionSettingsWriteResultParseResult | null;
  readonly lanPairingStatusEvent: PortalRouteEventRecord | null;
  readonly lanPairingBrowserDiscoveryEvent: PortalRouteEventRecord | null;
  readonly lanAddDeviceReadModel: PortalLanAddDeviceReadModel | null;
  readonly appGamePolicyReadinessPanel: unknown | null;
}

export const EMPTY_PORTAL_LIVE_ACTIVITY_STATE = {
  activityServiceUiSpine: {
    dataOwner: 'rust-service-read-model',
    uiConsumer: 'c-owned-activity-ui',
    viteDataOwner: false,
    currentState: 'unavailable',
    report: null,
    reportHistory: null,
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
  activityAppGamePlatformExtensionReadModel: null,
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
} satisfies PortalLiveActivityState;
