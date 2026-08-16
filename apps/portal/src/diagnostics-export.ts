import { PortalDiagnostics } from '@ocentra-parent/portal-domain/diagnostics';
import {
  ParentAgentEvent as AgentEvent,
  type ParentAgentEventName as AgentEventName,
  ParentAgentProtocolField,
  decodeParentPortalClipboardText,
  ParentRouteDataSource,
  type ParentRouteEventSnapshot,
  type ParentPortalClipboardText,
} from '../generated/parent-ui-bridge';
import { resolveSnapshotLiveActivityState } from './route-live-activity-state';
import type { PortalRuntimeState } from './portal-state';

export function buildDiagnosticsExport(state: PortalRuntimeState): ParentPortalClipboardText {
  const liveActivity = resolveSnapshotLiveActivityState(state.routeSnapshot?.liveActivity ?? null);
  const healthEvent = latestEvent(state.events, AgentEvent.HealthReported);
  const report = {
    [PortalDiagnostics.Field.SchemaVersion]: PortalDiagnostics.SchemaVersion,
    [PortalDiagnostics.Field.Agent]: {
      [PortalDiagnostics.Field.AgentUrl]: state.agentEndpoint,
      [PortalDiagnostics.Field.ConnectionState]: state.connectionState,
      [PortalDiagnostics.Field.Target]: state.routeSnapshot?.dataSource ?? ParentRouteDataSource.Unavailable,
    },
    [PortalDiagnostics.Field.Health]: healthSummary(healthEvent),
    [PortalDiagnostics.Field.Activity]: {
      [PortalDiagnostics.Field.IngestStatus]: liveActivity.ingestStatus,
      [PortalDiagnostics.Field.RecentSummary]: liveActivity.recentSummary,
      [PortalDiagnostics.Field.ActivityMemoryGraphReadModel]: liveActivity.activityMemoryGraphReadModel,
      [PortalDiagnostics.Field.NetworkFlowReadModel]: liveActivity.networkFlowReadModel,
      [PortalDiagnostics.Field.NetworkRuntimeEventChainStream]: liveActivity.networkRuntimeEventChainStream,
      [PortalDiagnostics.Field.NetworkRemoteDeliveryStatus]: liveActivity.networkRemoteDeliveryStatusResult,
      [PortalDiagnostics.Field.NetworkLiveCaptureStatus]: liveActivity.networkLiveCaptureStatusResult,
      [PortalDiagnostics.Field.NetworkLinuxNftablesLabStatus]: liveActivity.networkLinuxNftablesLabStatusResult,
      [PortalDiagnostics.Field.NetworkWindowsFirewallLabStatus]: liveActivity.networkWindowsFirewallLabStatusResult,
      [PortalDiagnostics.Field.NetworkWindowsWfpGateStatus]: liveActivity.networkWindowsWfpGateStatusResult,
      [PortalDiagnostics.Field.NetworkAndroidVpnServiceGateStatus]:
        liveActivity.networkAndroidVpnServiceGateStatusResult,
      [PortalDiagnostics.Field.NetworkAppleNetworkExtensionGateStatus]:
        liveActivity.networkAppleNetworkExtensionGateStatusResult,
    },
    [PortalDiagnostics.Field.Events]: state.events
      .slice(0, PortalDiagnostics.TimelineLimit)
      .map((event) => eventSummary(event)),
    [PortalDiagnostics.Field.DevLog]:
      state.latestSnapshot === null
        ? null
        : {
            [PortalDiagnostics.Field.LogAgent]: state.latestSnapshot.agent,
            [PortalDiagnostics.Field.Entries]: state.latestSnapshot.entries
              .slice(0, PortalDiagnostics.DevLogEntryLimit)
              .map((entry) => ({
                [PortalDiagnostics.Field.EventId]: entry.id,
                [PortalDiagnostics.Field.Timestamp]: entry.timestamp,
                [PortalDiagnostics.Field.Severity]: entry.level,
                [PortalDiagnostics.Field.SourcePeerId]: entry.source,
                [PortalDiagnostics.Field.Event]: entry.message,
              })),
          },
  };

  return decodeParentPortalClipboardText(JSON.stringify(report, null, PortalDiagnostics.JsonIndent));
}

function latestEvent(events: readonly ParentRouteEventSnapshot[], eventName: AgentEventName) {
  return events.find((event) => event.event === eventName) ?? null;
}

function eventSummary(event: ParentRouteEventSnapshot) {
  return {
    [PortalDiagnostics.Field.EventId]: event.eventId,
    [PortalDiagnostics.Field.SentAt]: event.sentAt,
    [PortalDiagnostics.Field.Event]: event.event,
    [PortalDiagnostics.Field.Severity]: event.severity,
    [PortalDiagnostics.Field.CorrelationId]: event.correlationId,
    [PortalDiagnostics.Field.SourcePeerId]: event.sourcePeerId,
    [PortalDiagnostics.Field.TargetPeerId]: event.targetPeerId,
  };
}

function healthSummary(event: ParentRouteEventSnapshot | null) {
  if (event === null) {
    return null;
  }
  const payload = event.payload ?? {};
  return {
    [PortalDiagnostics.Field.EventId]: event.eventId,
    [PortalDiagnostics.Field.SentAt]: event.sentAt,
    [PortalDiagnostics.Field.Online]: payload[ParentAgentProtocolField.Online],
    [PortalDiagnostics.Field.Transport]: payload[ParentAgentProtocolField.Transport],
  };
}
