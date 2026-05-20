import {
  AgentEvent,
  AgentProtocolDefaults,
  type AgentEventEnvelope,
  type AgentEventName,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import {
  PortalDiagnostics,
  decodePortalClipboardText,
  type PortalClipboardText,
} from '@ocentra-parent/portal-domain/contracts';
import { resolveLiveActivityState } from './live-activity-state';
import type { PortalRuntimeState } from './portal-state';

export function buildDiagnosticsExport(state: PortalRuntimeState): PortalClipboardText {
  const liveActivity = resolveLiveActivityState(state.events);
  const healthEvent = latestEvent(state.events, AgentEvent.HealthReported);
  const report = {
    [PortalDiagnostics.Field.SchemaVersion]: PortalDiagnostics.SchemaVersion,
    [PortalDiagnostics.Field.Agent]: {
      [PortalDiagnostics.Field.AgentUrl]: state.agentWsUrl,
      [PortalDiagnostics.Field.ConnectionState]: state.connectionState,
      [PortalDiagnostics.Field.Target]: state.target,
    },
    [PortalDiagnostics.Field.Health]: healthSummary(healthEvent),
    [PortalDiagnostics.Field.Activity]: {
      [PortalDiagnostics.Field.IngestStatus]: liveActivity.ingestStatus,
      [PortalDiagnostics.Field.RecentSummary]: liveActivity.recentSummary,
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

  return decodePortalClipboardText(JSON.stringify(report, null, PortalDiagnostics.JsonIndent));
}

function latestEvent(events: readonly AgentEventEnvelope[], eventName: AgentEventName) {
  return events.find((event) => event.event === eventName) ?? null;
}

function eventSummary(event: AgentEventEnvelope) {
  return {
    [PortalDiagnostics.Field.EventId]: event.eventId,
    [PortalDiagnostics.Field.SentAt]: event.sentAt,
    [PortalDiagnostics.Field.Event]: event.event,
    [PortalDiagnostics.Field.Severity]: event.severity,
    [PortalDiagnostics.Field.CorrelationId]: event.correlationId,
    [PortalDiagnostics.Field.SourcePeerId]: event.source.peerId,
    [PortalDiagnostics.Field.TargetPeerId]: event.target.peerId,
  };
}

function healthSummary(event: AgentEventEnvelope | null) {
  if (event === null) {
    return null;
  }
  return {
    [PortalDiagnostics.Field.EventId]: event.eventId,
    [PortalDiagnostics.Field.SentAt]: event.sentAt,
    [PortalDiagnostics.Field.Online]: event.payload[AgentProtocolDefaults.Field.Online],
    [PortalDiagnostics.Field.Transport]: event.payload[AgentProtocolDefaults.Field.Transport],
  };
}
