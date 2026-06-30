import type {
  ActivitySurfaceAdapterResult,
  ActivityServiceUiSpine,
} from '@ocentra-parent/agent-protocol-domain/activity-surface-adapter';
import type { AgentNetworkAndroidVpnServiceGateStatusParseResult } from '@ocentra-parent/agent-protocol-domain/network-android-vpnservice-gate-status';
import type { AgentNetworkAppleNetworkExtensionGateStatusParseResult } from '@ocentra-parent/agent-protocol-domain/network-apple-network-extension-gate-status';
import type { AgentNetworkLinuxNftablesLabStatusParseResult } from '@ocentra-parent/agent-protocol-domain/network-linux-nftables-lab-status';
import type { AgentNetworkLiveCaptureStatusParseResult } from '@ocentra-parent/agent-protocol-domain/network-live-capture-status';
import type { AgentNetworkRemoteDeliveryStatusParseResult } from '@ocentra-parent/agent-protocol-domain/network-remote-delivery-status';
import type { AgentNetworkRuntimeEventResult } from '@ocentra-parent/agent-protocol-domain/network-runtime-events';
import type { AgentNetworkWindowsFirewallLabStatusParseResult } from '@ocentra-parent/agent-protocol-domain/network-windows-firewall-lab-status';
import type { AgentNetworkWindowsWfpGateStatusParseResult } from '@ocentra-parent/agent-protocol-domain/network-windows-wfp-gate-status';
import type { AgentTrackingRetentionSettingsWriteResultParseResult } from '@ocentra-parent/agent-protocol-domain/tracking-retention-settings-write-command';
import type { AgentActivityTrackingReadModel } from '@ocentra-parent/schema-domain/agent-tracking-read-model';
import type { ActivityIngestStatus, ActivityRecentSummary } from '@ocentra-parent/schema-domain/activity-query';
import type {
  ActivityAppUseReadModel,
  ActivityBrowserReadModel,
  ActivityGamesReadModel,
  ActivityHistoricalReportList,
  ActivityNetworkReadModel,
  ActivityReportDocument,
  ActivityScreenReadModel,
} from '@ocentra-parent/schema-domain/activity-surface';
import type {
  BrowserEvidenceReadModel,
  BrowserManagedSessionStatus,
} from '@ocentra-parent/schema-domain/browser-schemas';
import type { BrowserInterventionReadModel } from '@ocentra-parent/schema-domain/browser-intervention-schemas';
import type { BrowserInventoryReadModel } from '@ocentra-parent/schema-domain/browser-inventory-schemas';
import type { ActivityNetworkFlowReadModel } from '@ocentra-parent/schema-domain/network-flow';
import type { PortalActivityMemoryGraphReadModel } from './activity-memory-graph';
import type { PortalLanAddDeviceReadModel } from './live-activity-lan-add-device';
import type { PortalRouteEventRecord } from './portal-contract-adapter';

type ActivitySurfaceReadModel =
  | ActivityScreenReadModel
  | ActivityAppUseReadModel
  | ActivityBrowserReadModel
  | ActivityGamesReadModel
  | ActivityNetworkReadModel;

export interface PortalNetworkRuntimeEventChainStream {
  readonly streamedEventCount: number | null;
  readonly events: readonly AgentNetworkRuntimeEventResult[];
  readonly invalidEventCount: number;
}

export type PortalActivityTrackingReadModelFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type PortalActivityTrackingReadModelResult =
  | {
      readonly ok: true;
      readonly value: AgentActivityTrackingReadModel;
    }
  | {
      readonly ok: false;
      readonly reason: PortalActivityTrackingReadModelFailureReason;
    };

export interface PortalLiveActivityState {
  readonly activityServiceUiSpine: ActivityServiceUiSpine;
  readonly ingestEvent: PortalRouteEventRecord | null;
  readonly ingestStatus: ActivityIngestStatus | null;
  readonly recentSummaryEvent: PortalRouteEventRecord | null;
  readonly recentSummary: ActivityRecentSummary | null;
  readonly browserEvidenceEvent: PortalRouteEventRecord | null;
  readonly browserEvidenceReadModel: BrowserEvidenceReadModel | null;
  readonly browserInventoryEvent: PortalRouteEventRecord | null;
  readonly browserInventoryReadModel: BrowserInventoryReadModel | null;
  readonly browserManagedEvent: PortalRouteEventRecord | null;
  readonly browserManagedStatus: BrowserManagedSessionStatus | null;
  readonly localAiRuntimeStatusEvent: PortalRouteEventRecord | null;
  readonly lanAiJobEvent: PortalRouteEventRecord | null;
  readonly parentAssistantBoundaryEvent: PortalRouteEventRecord | null;
  readonly activityMemoryGraphEvent: PortalRouteEventRecord | null;
  readonly activityMemoryGraphReadModel: PortalActivityMemoryGraphReadModel | null;
  readonly activityReportEvent: PortalRouteEventRecord | null;
  readonly activityReport: ActivitySurfaceAdapterResult<ActivityReportDocument> | null;
  readonly activityReportHistoryEvent: PortalRouteEventRecord | null;
  readonly activityReportHistory: ActivitySurfaceAdapterResult<ActivityHistoricalReportList> | null;
  readonly activityScreenReadModelEvent: PortalRouteEventRecord | null;
  readonly activityScreenReadModel: ActivitySurfaceAdapterResult<ActivitySurfaceReadModel> | null;
  readonly activityAppUseReadModelEvent: PortalRouteEventRecord | null;
  readonly activityAppUseReadModel: ActivitySurfaceAdapterResult<ActivitySurfaceReadModel> | null;
  readonly activityBrowserReadModelEvent: PortalRouteEventRecord | null;
  readonly activityBrowserReadModel: ActivitySurfaceAdapterResult<ActivitySurfaceReadModel> | null;
  readonly activityGamesReadModelEvent: PortalRouteEventRecord | null;
  readonly activityGamesReadModel: ActivitySurfaceAdapterResult<ActivitySurfaceReadModel> | null;
  readonly appGameNotificationParentSurfacePanel: unknown | null;
  readonly appGamePlatformProofStatusPanel: unknown | null;
  readonly appGameChildRuntimeTransportReceiptPanel: unknown | null;
  readonly activityNetworkReadModelEvent: PortalRouteEventRecord | null;
  readonly activityNetworkReadModel: ActivitySurfaceAdapterResult<ActivitySurfaceReadModel> | null;
  readonly browserInterventionEvent: PortalRouteEventRecord | null;
  readonly browserInterventionReadModel: BrowserInterventionReadModel | null;
  readonly networkFlowEvent: PortalRouteEventRecord | null;
  readonly networkFlowReadModel: ActivityNetworkFlowReadModel | null;
  readonly networkRuntimeEventChainEvent: PortalRouteEventRecord | null;
  readonly networkRuntimeEventChainStream: PortalNetworkRuntimeEventChainStream | null;
  readonly networkRemoteDeliveryStatusEvent: PortalRouteEventRecord | null;
  readonly networkRemoteDeliveryStatusResult: AgentNetworkRemoteDeliveryStatusParseResult | null;
  readonly networkLiveCaptureStatusEvent: PortalRouteEventRecord | null;
  readonly networkLiveCaptureStatusResult: AgentNetworkLiveCaptureStatusParseResult | null;
  readonly networkLinuxNftablesLabStatusEvent: PortalRouteEventRecord | null;
  readonly networkLinuxNftablesLabStatusResult: AgentNetworkLinuxNftablesLabStatusParseResult | null;
  readonly networkWindowsFirewallLabStatusEvent: PortalRouteEventRecord | null;
  readonly networkWindowsFirewallLabStatusResult: AgentNetworkWindowsFirewallLabStatusParseResult | null;
  readonly networkWindowsWfpGateStatusEvent: PortalRouteEventRecord | null;
  readonly networkWindowsWfpGateStatusResult: AgentNetworkWindowsWfpGateStatusParseResult | null;
  readonly networkAndroidVpnServiceGateStatusEvent: PortalRouteEventRecord | null;
  readonly networkAndroidVpnServiceGateStatusResult: AgentNetworkAndroidVpnServiceGateStatusParseResult | null;
  readonly networkAppleNetworkExtensionGateStatusEvent: PortalRouteEventRecord | null;
  readonly networkAppleNetworkExtensionGateStatusResult: AgentNetworkAppleNetworkExtensionGateStatusParseResult | null;
  readonly activityTrackingReadModelEvent: PortalRouteEventRecord | null;
  readonly activityTrackingReadModel: PortalActivityTrackingReadModelResult | null;
  readonly activityTrackingPanel: unknown | null;
  readonly activityTrackingRetentionSettingsWriteEvent: PortalRouteEventRecord | null;
  readonly activityTrackingRetentionSettingsWriteResult: AgentTrackingRetentionSettingsWriteResultParseResult | null;
  readonly lanPairingStatusEvent: PortalRouteEventRecord | null;
  readonly lanPairingBrowserDiscoveryEvent: PortalRouteEventRecord | null;
  readonly lanAddDeviceReadModel: PortalLanAddDeviceReadModel | null;
  readonly appGamePolicyReadinessPanel: unknown | null;
}
