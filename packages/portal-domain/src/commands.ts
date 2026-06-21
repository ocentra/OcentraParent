import { AgentCommand, AgentEvent } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/text-domain/portal-dev';

export const PortalActivitySurfaceDefaultRequestPayload = {
  [AgentProtocolDefaults.Field.ScopeKind]: 'family',
  [AgentProtocolDefaults.Field.FamilyId]: 'family-local',
  [AgentProtocolDefaults.Field.RequestedAt]: '2026-06-06T00:00:00.000Z',
  [AgentProtocolDefaults.Field.RangeStart]: '2026-06-06T00:00:00.000Z',
  [AgentProtocolDefaults.Field.RangeEnd]: '2026-06-06T23:59:59.999Z',
} as const;

export const PortalOverviewCommands = [
  {
    command: AgentCommand.HealthCheck,
    payload: {},
  },
  {
    command: AgentCommand.LogSnapshotGet,
    payload: {},
  },
  {
    command: AgentCommand.NetworkFlowReadModelGet,
    payload: {},
  },
  {
    command: AgentCommand.LanPairingStatusGet,
    payload: {},
  },
  {
    command: AgentCommand.ActivityIngestStatusGet,
    payload: {},
  },
  {
    command: AgentCommand.ActivityRecentSummaryGet,
    payload: {},
  },
  {
    command: AgentCommand.BrowserEvidenceRecentGet,
    payload: {},
  },
  {
    command: AgentCommand.BrowserManagedBridgePoll,
    payload: {},
  },
  {
    command: AgentCommand.BrowserInventoryReadModelGet,
    payload: {},
  },
  {
    command: AgentCommand.ActivityMemoryGraphGet,
    payload: {},
  },
  {
    command: AgentCommand.ActivityReportHistoryList,
    payload: {},
  },
  {
    command: AgentCommand.ActivityScreenReadModelGet,
    payload: PortalActivitySurfaceDefaultRequestPayload,
  },
  {
    command: AgentCommand.ActivityAppUseReadModelGet,
    payload: {},
  },
  {
    command: AgentCommand.ActivityBrowserReadModelGet,
    payload: {},
  },
  {
    command: AgentCommand.ActivityGamesReadModelGet,
    payload: {},
  },
  {
    command: AgentCommand.ActivityAppGameNotificationReadinessReadModelGet,
    payload: {},
  },
  {
    command: AgentCommand.ActivityAppGameAdapterExecutionReadinessReadModelGet,
    payload: {},
  },
  {
    command: AgentCommand.ActivityAppGamePlatformProofStatusReadModelGet,
    payload: {},
  },
  {
    command: AgentCommand.ActivityAppGameChildRuntimeTransportReceiptReadModelGet,
    payload: {},
  },
  {
    command: AgentCommand.ActivityAppGameAdapterDispatchPreflightReadModelGet,
    payload: {},
  },
  {
    command: AgentCommand.ActivityAppGameAdapterDispatchResultReadModelGet,
    payload: {},
  },
  {
    command: AgentCommand.ActivityNetworkReadModelGet,
    payload: {},
  },
  {
    command: AgentCommand.BrowserInterventionReadModelGet,
    payload: {},
  },
  {
    command: AgentCommand.BrowserRuntimeEventChainStreamGet,
    payload: {},
  },
  {
    command: AgentCommand.NetworkRuntimeEventChainStreamGet,
    payload: {},
  },
  {
    command: AgentCommand.NetworkRemoteDeliveryStatusGet,
    payload: {},
  },
  {
    command: AgentCommand.NetworkLiveCaptureStatusGet,
    payload: {},
  },
  {
    command: AgentCommand.NetworkLinuxNftablesLabStatusGet,
    payload: {},
  },
  {
    command: AgentCommand.NetworkWindowsFirewallLabStatusGet,
    payload: {},
  },
  {
    command: AgentCommand.NetworkWindowsWfpGateStatusGet,
    payload: {},
  },
  {
    command: AgentCommand.NetworkAndroidVpnServiceGateStatusGet,
    payload: {},
  },
  {
    command: AgentCommand.NetworkAppleNetworkExtensionGateStatusGet,
    payload: {},
  },
  {
    command: AgentCommand.ActivityTrackingReadModelGet,
    payload: {},
  },
  {
    command: AgentCommand.LocalAiRuntimeStatusGet,
    payload: {},
  },
  {
    command: AgentCommand.PolicyPreviewReadModelGet,
    payload: {},
  },
] as const;

export const PortalCommandButtons = [
  {
    label: resolvePortalDevText(PortalDevTextToken.CheckHealth),
    command: AgentCommand.HealthCheck,
    resultEvent: AgentEvent.HealthReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetLogSnapshot),
    command: AgentCommand.LogSnapshotGet,
    resultEvent: AgentEvent.LogSnapshotReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.EchoPortalPing),
    command: AgentCommand.DevEcho,
    resultEvent: AgentEvent.DevEchoed,
    payload: {
      [AgentProtocolDefaults.Field.Message]: resolvePortalDevText(PortalDevTextToken.EchoPortalPing),
    },
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetWatcherStatus),
    command: AgentCommand.WatchStatusGet,
    resultEvent: AgentEvent.WatchStatusReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetActivityIngestStatus),
    command: AgentCommand.ActivityIngestStatusGet,
    resultEvent: AgentEvent.ActivityIngestStatusReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetRecentActivitySummary),
    command: AgentCommand.ActivityRecentSummaryGet,
    resultEvent: AgentEvent.ActivityRecentSummaryReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetBrowserEvidenceRecent),
    command: AgentCommand.BrowserEvidenceRecentGet,
    resultEvent: AgentEvent.BrowserEvidenceRecentReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetActivityMemoryGraph),
    command: AgentCommand.ActivityMemoryGraphGet,
    resultEvent: AgentEvent.ActivityMemoryGraphReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetActivityReportDaily),
    command: AgentCommand.ActivityReportDailyGenerate,
    resultEvent: AgentEvent.ActivityReportGenerated,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetActivityReportHistory),
    command: AgentCommand.ActivityReportHistoryList,
    resultEvent: AgentEvent.ActivityReportHistoryReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetActivityScreenReadModel),
    command: AgentCommand.ActivityScreenReadModelGet,
    resultEvent: AgentEvent.ActivityScreenReadModelReported,
    payload: PortalActivitySurfaceDefaultRequestPayload,
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetActivityAppUseReadModel),
    command: AgentCommand.ActivityAppUseReadModelGet,
    resultEvent: AgentEvent.ActivityAppUseReadModelReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetActivityBrowserReadModel),
    command: AgentCommand.ActivityBrowserReadModelGet,
    resultEvent: AgentEvent.ActivityBrowserReadModelReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetActivityGamesReadModel),
    command: AgentCommand.ActivityGamesReadModelGet,
    resultEvent: AgentEvent.ActivityGamesReadModelReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetActivityAppGameAdapterExecutionReadinessReadModel),
    command: AgentCommand.ActivityAppGameAdapterExecutionReadinessReadModelGet,
    resultEvent: AgentEvent.ActivityAppGameAdapterExecutionReadinessReadModelReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetActivityAppGamePlatformProofStatusReadModel),
    command: AgentCommand.ActivityAppGamePlatformProofStatusReadModelGet,
    resultEvent: AgentEvent.ActivityAppGamePlatformProofStatusReadModelReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetActivityAppGameChildRuntimeTransportReceiptReadModel),
    command: AgentCommand.ActivityAppGameChildRuntimeTransportReceiptReadModelGet,
    resultEvent: AgentEvent.ActivityAppGameChildRuntimeTransportReceiptReadModelReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetActivityAppGameAdapterDispatchPreflightReadModel),
    command: AgentCommand.ActivityAppGameAdapterDispatchPreflightReadModelGet,
    resultEvent: AgentEvent.ActivityAppGameAdapterDispatchPreflightReadModelReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetActivityAppGameAdapterDispatchResultReadModel),
    command: AgentCommand.ActivityAppGameAdapterDispatchResultReadModelGet,
    resultEvent: AgentEvent.ActivityAppGameAdapterDispatchResultReadModelReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.ExecuteActivityAppGameAdapterDispatch),
    command: AgentCommand.ActivityAppGameAdapterDispatchExecute,
    resultEvent: AgentEvent.ActivityAppGameAdapterDispatchExecuted,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetActivityNetworkReadModel),
    command: AgentCommand.ActivityNetworkReadModelGet,
    resultEvent: AgentEvent.ActivityNetworkReadModelReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetBrowserInterventionReadModel),
    command: AgentCommand.BrowserInterventionReadModelGet,
    resultEvent: AgentEvent.BrowserInterventionReadModelReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.PollManagedBrowserBridge),
    command: AgentCommand.BrowserManagedBridgePoll,
    resultEvent: AgentEvent.BrowserManagedStatusReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetBrowserRuntimeEventChainStream),
    command: AgentCommand.BrowserRuntimeEventChainStreamGet,
    resultEvent: AgentEvent.BrowserRuntimeEventChainStreamReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetNetworkFlowReadModel),
    command: AgentCommand.NetworkFlowReadModelGet,
    resultEvent: AgentEvent.NetworkFlowReadModelReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetNetworkRuntimeEventChainStream),
    command: AgentCommand.NetworkRuntimeEventChainStreamGet,
    resultEvent: AgentEvent.NetworkRuntimeEventChainStreamReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetNetworkRemoteDeliveryStatus),
    command: AgentCommand.NetworkRemoteDeliveryStatusGet,
    resultEvent: AgentEvent.NetworkRemoteDeliveryStatusReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetNetworkLiveCaptureStatus),
    command: AgentCommand.NetworkLiveCaptureStatusGet,
    resultEvent: AgentEvent.NetworkLiveCaptureStatusReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetNetworkLinuxNftablesLabStatus),
    command: AgentCommand.NetworkLinuxNftablesLabStatusGet,
    resultEvent: AgentEvent.NetworkLinuxNftablesLabStatusReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetNetworkWindowsFirewallLabStatus),
    command: AgentCommand.NetworkWindowsFirewallLabStatusGet,
    resultEvent: AgentEvent.NetworkWindowsFirewallLabStatusReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetNetworkWindowsWfpGateStatus),
    command: AgentCommand.NetworkWindowsWfpGateStatusGet,
    resultEvent: AgentEvent.NetworkWindowsWfpGateStatusReported,
    payload: {},
  },
  {
    label: 'Get network Android VpnService gate status',
    command: AgentCommand.NetworkAndroidVpnServiceGateStatusGet,
    resultEvent: AgentEvent.NetworkAndroidVpnServiceGateStatusReported,
    payload: {},
  },
  {
    label: 'Get network Apple Network Extension gate status',
    command: AgentCommand.NetworkAppleNetworkExtensionGateStatusGet,
    resultEvent: AgentEvent.NetworkAppleNetworkExtensionGateStatusReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetActivityTrackingReadModel),
    command: AgentCommand.ActivityTrackingReadModelGet,
    resultEvent: AgentEvent.ActivityTrackingReadModelReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetLocalAiRuntimeStatus),
    command: AgentCommand.LocalAiRuntimeStatusGet,
    resultEvent: AgentEvent.LocalAiRuntimeStatusReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetPolicyPreviewReadModel),
    command: AgentCommand.PolicyPreviewReadModelGet,
    resultEvent: AgentEvent.PolicyPreviewReadModelReported,
    payload: {},
  },
] as const;
