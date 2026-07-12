import { PortalDevTextToken, resolvePortalDevText } from './display-text';
import {
  GeneratedPortalAgentActivitySurfaceAdapterOperationId,
  GeneratedPortalAgentActivitySurfaceAdapterOperationManifest,
  GeneratedPortalAgentActivitySurfaceScopeKind,
  GeneratedPortalAgentCommand as AgentCommand,
  type GeneratedPortalAgentCommandName as AgentCommandName,
  GeneratedPortalAgentEvent as AgentEvent,
  type GeneratedPortalAgentEventName as AgentEventName,
  GeneratedPortalAgentProtocolField,
  type GeneratedPortalAgentProtocolPayload as AgentProtocolPayload,
  type GeneratedPortalAgentActivitySurfaceAdapterOperation as ActivitySurfaceOperation,
} from './generated-portal-contracts';

export type PortalOverviewCommand = {
  readonly command: AgentCommandName;
  readonly payload: AgentProtocolPayload;
};

export type PortalCommandButton = PortalOverviewCommand & {
  readonly label: string;
  readonly resultEvent: AgentEventName;
};

const EmptyPayload = {} as const satisfies AgentProtocolPayload;

const ActivitySurfaceOperations = new Map(
  GeneratedPortalAgentActivitySurfaceAdapterOperationManifest.map(
    (operation) => [operation.operationId, operation] as const
  )
);

function activitySurfaceOperation(operationId: ActivitySurfaceOperation['operationId']): ActivitySurfaceOperation {
  const operation = ActivitySurfaceOperations.get(operationId);
  if (operation === undefined) {
    throw new TypeError(`Missing generated activity surface operation: ${operationId}`);
  }
  return operation;
}

function overviewCommand(
  command: AgentCommandName,
  payload: AgentProtocolPayload = EmptyPayload
): PortalOverviewCommand {
  return { command, payload };
}

function overviewCommandFromActivity(
  operationId: ActivitySurfaceOperation['operationId'],
  payload: AgentProtocolPayload = EmptyPayload
): PortalOverviewCommand {
  const operation = activitySurfaceOperation(operationId);
  return overviewCommand(operation.command, payload);
}

function commandButton(
  label: string,
  command: AgentCommandName,
  resultEvent: AgentEventName,
  payload: AgentProtocolPayload = EmptyPayload
): PortalCommandButton {
  return { label, command, resultEvent, payload };
}

function commandButtonFromActivity(
  label: string,
  operationId: ActivitySurfaceOperation['operationId'],
  payload: AgentProtocolPayload = EmptyPayload
): PortalCommandButton {
  const operation = activitySurfaceOperation(operationId);
  return commandButton(label, operation.command, operation.successEvent, payload);
}

export const PortalActivitySurfaceDefaultRequestPayload = {
  [GeneratedPortalAgentProtocolField.ScopeKind]: GeneratedPortalAgentActivitySurfaceScopeKind.Family,
  [GeneratedPortalAgentProtocolField.FamilyId]: 'family-local',
  [GeneratedPortalAgentProtocolField.RequestedAt]: '2026-06-06T00:00:00.000Z',
  [GeneratedPortalAgentProtocolField.RangeStart]: '2026-06-06T00:00:00.000Z',
  [GeneratedPortalAgentProtocolField.RangeEnd]: '2026-06-06T23:59:59.999Z',
} as const satisfies AgentProtocolPayload;

const PortalOverviewActivityOperations = [
  GeneratedPortalAgentActivitySurfaceAdapterOperationId.ListHistoricalReports,
  GeneratedPortalAgentActivitySurfaceAdapterOperationId.GetScreenActivity,
  GeneratedPortalAgentActivitySurfaceAdapterOperationId.GetAppUseActivity,
  GeneratedPortalAgentActivitySurfaceAdapterOperationId.GetBrowserActivity,
  GeneratedPortalAgentActivitySurfaceAdapterOperationId.GetGamesActivity,
] as const satisfies readonly ActivitySurfaceOperation['operationId'][];

export const PortalOverviewCommands = [
  overviewCommand(AgentCommand.HealthCheck),
  overviewCommand(AgentCommand.LogSnapshotGet),
  overviewCommand(AgentCommand.NetworkFlowReadModelGet),
  overviewCommand(AgentCommand.LanPairingStatusGet),
  overviewCommand(AgentCommand.ActivityIngestStatusGet),
  overviewCommand(AgentCommand.ActivityRecentSummaryGet),
  overviewCommand(AgentCommand.BrowserEvidenceRecentGet),
  overviewCommand(AgentCommand.BrowserManagedBridgePoll),
  overviewCommand(AgentCommand.BrowserInventoryReadModelGet),
  overviewCommand(AgentCommand.ActivityMemoryGraphGet),
  ...PortalOverviewActivityOperations.map((operationId) =>
    operationId === GeneratedPortalAgentActivitySurfaceAdapterOperationId.GetScreenActivity
      ? overviewCommandFromActivity(operationId, PortalActivitySurfaceDefaultRequestPayload)
      : overviewCommandFromActivity(operationId)
  ),
  overviewCommand(AgentCommand.ActivityAppGameNotificationReadinessReadModelGet),
  overviewCommand(AgentCommand.ActivityAppGameAdapterExecutionReadinessReadModelGet),
  overviewCommand(AgentCommand.ActivityAppGamePlatformProofStatusReadModelGet),
  overviewCommand(AgentCommand.ActivityAppGameChildRuntimeTransportReceiptReadModelGet),
  overviewCommand(AgentCommand.ActivityAppGameAdapterDispatchPreflightReadModelGet),
  overviewCommand(AgentCommand.ActivityAppGameAdapterDispatchResultReadModelGet),
  overviewCommandFromActivity(GeneratedPortalAgentActivitySurfaceAdapterOperationId.GetNetworkActivity),
  overviewCommand(AgentCommand.BrowserInterventionReadModelGet),
  overviewCommand(AgentCommand.BrowserRuntimeEventChainStreamGet),
  overviewCommand(AgentCommand.NetworkRuntimeEventChainStreamGet),
  overviewCommand(AgentCommand.NetworkRemoteDeliveryStatusGet),
  overviewCommand(AgentCommand.NetworkLiveCaptureStatusGet),
  overviewCommand(AgentCommand.NetworkLinuxNftablesLabStatusGet),
  overviewCommand(AgentCommand.NetworkWindowsFirewallLabStatusGet),
  overviewCommand(AgentCommand.NetworkWindowsWfpGateStatusGet),
  overviewCommand(AgentCommand.NetworkAndroidVpnServiceGateStatusGet),
  overviewCommand(AgentCommand.NetworkAppleNetworkExtensionGateStatusGet),
  overviewCommand(AgentCommand.ActivityTrackingReadModelGet),
  overviewCommand(AgentCommand.LocalAiRuntimeStatusGet),
  overviewCommand(AgentCommand.PolicyPreviewReadModelGet),
] as const satisfies readonly PortalOverviewCommand[];

// The portal only chooses which Rust-owned commands to surface and how to label them.
export const PortalCommandButtons = [
  commandButton(
    resolvePortalDevText(PortalDevTextToken.CheckHealth),
    AgentCommand.HealthCheck,
    AgentEvent.HealthReported
  ),
  commandButton(
    resolvePortalDevText(PortalDevTextToken.GetLogSnapshot),
    AgentCommand.LogSnapshotGet,
    AgentEvent.LogSnapshotReported
  ),
  commandButton(resolvePortalDevText(PortalDevTextToken.EchoPortalPing), AgentCommand.DevEcho, AgentEvent.DevEchoed, {
    [GeneratedPortalAgentProtocolField.Message]: resolvePortalDevText(PortalDevTextToken.EchoPortalPing),
  }),
  commandButton(
    resolvePortalDevText(PortalDevTextToken.GetWatcherStatus),
    AgentCommand.WatchStatusGet,
    AgentEvent.WatchStatusReported
  ),
  commandButton(
    resolvePortalDevText(PortalDevTextToken.GetActivityIngestStatus),
    AgentCommand.ActivityIngestStatusGet,
    AgentEvent.ActivityIngestStatusReported
  ),
  commandButton(
    resolvePortalDevText(PortalDevTextToken.GetRecentActivitySummary),
    AgentCommand.ActivityRecentSummaryGet,
    AgentEvent.ActivityRecentSummaryReported
  ),
  commandButton(
    resolvePortalDevText(PortalDevTextToken.GetBrowserEvidenceRecent),
    AgentCommand.BrowserEvidenceRecentGet,
    AgentEvent.BrowserEvidenceRecentReported
  ),
  commandButton(
    resolvePortalDevText(PortalDevTextToken.GetActivityMemoryGraph),
    AgentCommand.ActivityMemoryGraphGet,
    AgentEvent.ActivityMemoryGraphReported
  ),
  commandButtonFromActivity(
    resolvePortalDevText(PortalDevTextToken.GetActivityReportDaily),
    GeneratedPortalAgentActivitySurfaceAdapterOperationId.GetDailyReport
  ),
  commandButtonFromActivity(
    resolvePortalDevText(PortalDevTextToken.GetActivityReportHistory),
    GeneratedPortalAgentActivitySurfaceAdapterOperationId.ListHistoricalReports
  ),
  commandButtonFromActivity(
    resolvePortalDevText(PortalDevTextToken.GetActivityScreenReadModel),
    GeneratedPortalAgentActivitySurfaceAdapterOperationId.GetScreenActivity,
    PortalActivitySurfaceDefaultRequestPayload
  ),
  commandButtonFromActivity(
    resolvePortalDevText(PortalDevTextToken.GetActivityAppUseReadModel),
    GeneratedPortalAgentActivitySurfaceAdapterOperationId.GetAppUseActivity
  ),
  commandButtonFromActivity(
    resolvePortalDevText(PortalDevTextToken.GetActivityBrowserReadModel),
    GeneratedPortalAgentActivitySurfaceAdapterOperationId.GetBrowserActivity
  ),
  commandButtonFromActivity(
    resolvePortalDevText(PortalDevTextToken.GetActivityGamesReadModel),
    GeneratedPortalAgentActivitySurfaceAdapterOperationId.GetGamesActivity
  ),
  commandButton(
    resolvePortalDevText(PortalDevTextToken.GetActivityAppGameAdapterExecutionReadinessReadModel),
    AgentCommand.ActivityAppGameAdapterExecutionReadinessReadModelGet,
    AgentEvent.ActivityAppGameAdapterExecutionReadinessReadModelReported
  ),
  commandButton(
    resolvePortalDevText(PortalDevTextToken.GetActivityAppGamePlatformProofStatusReadModel),
    AgentCommand.ActivityAppGamePlatformProofStatusReadModelGet,
    AgentEvent.ActivityAppGamePlatformProofStatusReadModelReported
  ),
  commandButton(
    resolvePortalDevText(PortalDevTextToken.GetActivityAppGameChildRuntimeTransportReceiptReadModel),
    AgentCommand.ActivityAppGameChildRuntimeTransportReceiptReadModelGet,
    AgentEvent.ActivityAppGameChildRuntimeTransportReceiptReadModelReported
  ),
  commandButton(
    resolvePortalDevText(PortalDevTextToken.GetActivityAppGameAdapterDispatchPreflightReadModel),
    AgentCommand.ActivityAppGameAdapterDispatchPreflightReadModelGet,
    AgentEvent.ActivityAppGameAdapterDispatchPreflightReadModelReported
  ),
  commandButton(
    resolvePortalDevText(PortalDevTextToken.GetActivityAppGameAdapterDispatchResultReadModel),
    AgentCommand.ActivityAppGameAdapterDispatchResultReadModelGet,
    AgentEvent.ActivityAppGameAdapterDispatchResultReadModelReported
  ),
  commandButton(
    resolvePortalDevText(PortalDevTextToken.ExecuteActivityAppGameAdapterDispatch),
    AgentCommand.ActivityAppGameAdapterDispatchExecute,
    AgentEvent.ActivityAppGameAdapterDispatchExecuted
  ),
  commandButtonFromActivity(
    resolvePortalDevText(PortalDevTextToken.GetActivityNetworkReadModel),
    GeneratedPortalAgentActivitySurfaceAdapterOperationId.GetNetworkActivity
  ),
  commandButton(
    resolvePortalDevText(PortalDevTextToken.GetBrowserInterventionReadModel),
    AgentCommand.BrowserInterventionReadModelGet,
    AgentEvent.BrowserInterventionReadModelReported
  ),
  commandButton(
    resolvePortalDevText(PortalDevTextToken.PollManagedBrowserBridge),
    AgentCommand.BrowserManagedBridgePoll,
    AgentEvent.BrowserManagedStatusReported
  ),
  commandButton(
    resolvePortalDevText(PortalDevTextToken.GetBrowserRuntimeEventChainStream),
    AgentCommand.BrowserRuntimeEventChainStreamGet,
    AgentEvent.BrowserRuntimeEventChainStreamReported
  ),
  commandButton(
    resolvePortalDevText(PortalDevTextToken.GetNetworkFlowReadModel),
    AgentCommand.NetworkFlowReadModelGet,
    AgentEvent.NetworkFlowReadModelReported
  ),
  commandButton(
    resolvePortalDevText(PortalDevTextToken.GetLanPairingStatus),
    AgentCommand.LanPairingStatusGet,
    AgentEvent.LanPairingStatusReported
  ),
  commandButton(
    resolvePortalDevText(PortalDevTextToken.GetNetworkRuntimeEventChainStream),
    AgentCommand.NetworkRuntimeEventChainStreamGet,
    AgentEvent.NetworkRuntimeEventChainStreamReported
  ),
  commandButton(
    resolvePortalDevText(PortalDevTextToken.GetNetworkRemoteDeliveryStatus),
    AgentCommand.NetworkRemoteDeliveryStatusGet,
    AgentEvent.NetworkRemoteDeliveryStatusReported
  ),
  commandButton(
    resolvePortalDevText(PortalDevTextToken.GetNetworkLiveCaptureStatus),
    AgentCommand.NetworkLiveCaptureStatusGet,
    AgentEvent.NetworkLiveCaptureStatusReported
  ),
  commandButton(
    resolvePortalDevText(PortalDevTextToken.GetNetworkLinuxNftablesLabStatus),
    AgentCommand.NetworkLinuxNftablesLabStatusGet,
    AgentEvent.NetworkLinuxNftablesLabStatusReported
  ),
  commandButton(
    resolvePortalDevText(PortalDevTextToken.GetNetworkWindowsFirewallLabStatus),
    AgentCommand.NetworkWindowsFirewallLabStatusGet,
    AgentEvent.NetworkWindowsFirewallLabStatusReported
  ),
  commandButton(
    resolvePortalDevText(PortalDevTextToken.GetNetworkWindowsWfpGateStatus),
    AgentCommand.NetworkWindowsWfpGateStatusGet,
    AgentEvent.NetworkWindowsWfpGateStatusReported
  ),
  commandButton(
    'Get network Android VpnService gate status',
    AgentCommand.NetworkAndroidVpnServiceGateStatusGet,
    AgentEvent.NetworkAndroidVpnServiceGateStatusReported
  ),
  commandButton(
    'Get network Apple Network Extension gate status',
    AgentCommand.NetworkAppleNetworkExtensionGateStatusGet,
    AgentEvent.NetworkAppleNetworkExtensionGateStatusReported
  ),
  commandButton(
    resolvePortalDevText(PortalDevTextToken.GetActivityTrackingReadModel),
    AgentCommand.ActivityTrackingReadModelGet,
    AgentEvent.ActivityTrackingReadModelReported
  ),
  commandButton(
    resolvePortalDevText(PortalDevTextToken.GetLocalAiRuntimeStatus),
    AgentCommand.LocalAiRuntimeStatusGet,
    AgentEvent.LocalAiRuntimeStatusReported
  ),
  commandButton(
    resolvePortalDevText(PortalDevTextToken.GetPolicyPreviewReadModel),
    AgentCommand.PolicyPreviewReadModelGet,
    AgentEvent.PolicyPreviewReadModelReported
  ),
] as const satisfies readonly PortalCommandButton[];
