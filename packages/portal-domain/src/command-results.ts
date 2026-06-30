import { AgentEvent, type AgentEventName } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import type { PortalRouteEventRecord } from './portal-contract-adapter';

export const PortalCommandResultEvents = [
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
  AgentEvent.LanPairingStatusReported,
  AgentEvent.NetworkRuntimeEventChainStreamReported,
  AgentEvent.NetworkRemoteDeliveryStatusReported,
  AgentEvent.NetworkLiveCaptureStatusReported,
  AgentEvent.NetworkLinuxNftablesLabStatusReported,
  AgentEvent.NetworkWindowsFirewallLabStatusReported,
  AgentEvent.NetworkWindowsWfpGateStatusReported,
  AgentEvent.ActivityTrackingReadModelReported,
  AgentEvent.ActivityTrackingRetentionSettingsWriteReported,
  AgentEvent.LocalAiRuntimeStatusReported,
  AgentEvent.PolicyPreviewReadModelReported,
  AgentEvent.ActivityAppGamePolicyReadinessReadModelReported,
] as const satisfies readonly AgentEventName[];

const CommandResultEvents = new Set<AgentEventName>(PortalCommandResultEvents);

export function latestCommandResult(
  events: readonly PortalRouteEventRecord[],
  eventName: AgentEventName
): PortalRouteEventRecord | null {
  return events.find((event) => event.event === eventName) ?? null;
}

export function isCommandResultEvent(eventName: AgentEventName): boolean {
  return CommandResultEvents.has(eventName);
}
