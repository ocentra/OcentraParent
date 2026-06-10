import {
  AgentEvent,
  type AgentEventEnvelope,
  type AgentEventName,
} from '@ocentra-parent/agent-protocol-domain/contracts';

const CommandResultEvents = new Set<AgentEventName>([
  AgentEvent.HealthReported,
  AgentEvent.LogSnapshotReported,
  AgentEvent.DevEchoed,
  AgentEvent.WatchStatusReported,
  AgentEvent.ActivityIngestStatusReported,
  AgentEvent.ActivityRecentSummaryReported,
  AgentEvent.ActivityMemoryGraphReported,
  AgentEvent.ActivityReportGenerated,
  AgentEvent.ActivityReportSaved,
  AgentEvent.ActivityReportHistoryReported,
  AgentEvent.ActivityScreenReadModelReported,
  AgentEvent.ActivityAppUseReadModelReported,
  AgentEvent.ActivityBrowserReadModelReported,
  AgentEvent.ActivityGamesReadModelReported,
  AgentEvent.ActivityAppGameNotificationReadinessReadModelReported,
  AgentEvent.ActivityAppGameTimerParentSurfaceReadModelReported,
  AgentEvent.ActivityAppGameTimerParentPreferenceSetupRequested,
  AgentEvent.ActivityNetworkReadModelReported,
  AgentEvent.BrowserEvidenceRecentReported,
  AgentEvent.BrowserManagedStatusReported,
  AgentEvent.BrowserInterventionReadModelReported,
  AgentEvent.BrowserRuntimeEventChainStreamReported,
  AgentEvent.NetworkFlowReadModelReported,
  AgentEvent.NetworkRuntimeEventChainStreamReported,
  AgentEvent.NetworkRemoteDeliveryStatusReported,
  AgentEvent.NetworkLiveCaptureStatusReported,
  AgentEvent.NetworkLinuxNftablesLabStatusReported,
  AgentEvent.NetworkWindowsFirewallLabStatusReported,
  AgentEvent.NetworkWindowsWfpGateStatusReported,
  AgentEvent.ActivityTrackingReadModelReported,
  AgentEvent.LocalAiRuntimeStatusReported,
  AgentEvent.PolicyPreviewReadModelReported,
  AgentEvent.ActivityAppGamePolicyReadinessReadModelReported,
]);

export function latestCommandResult(
  events: readonly AgentEventEnvelope[],
  eventName: AgentEventName
): AgentEventEnvelope | null {
  return events.find((event) => event.event === eventName) ?? null;
}

export function isCommandResultEvent(eventName: AgentEventName): boolean {
  return CommandResultEvents.has(eventName);
}
