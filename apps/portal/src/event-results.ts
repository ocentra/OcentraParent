import {
  AgentEvent,
  type AgentEventEnvelope,
  type AgentEventName,
} from '@ocentra-parent/agent-protocol-domain/contracts';

const PortalResultEvents = new Set<AgentEventName>([
  AgentEvent.ConnectionReady,
  AgentEvent.CommandRejected,
  AgentEvent.HealthReported,
  AgentEvent.LogSnapshotReported,
  AgentEvent.DevEchoed,
  AgentEvent.WatchStatusReported,
]);

const CommandResultEvents = new Set<AgentEventName>([
  AgentEvent.HealthReported,
  AgentEvent.LogSnapshotReported,
  AgentEvent.DevEchoed,
  AgentEvent.WatchStatusReported,
]);

export function latestPortalEvents(events: readonly AgentEventEnvelope[]): AgentEventEnvelope[] {
  return latestMatchingEvents(events, PortalResultEvents);
}

export function latestCommandResults(events: readonly AgentEventEnvelope[]): AgentEventEnvelope[] {
  return latestMatchingEvents(events, CommandResultEvents);
}

function latestMatchingEvents(
  events: readonly AgentEventEnvelope[],
  allowedEvents: ReadonlySet<AgentEventName>
): AgentEventEnvelope[] {
  const visibleEvents: AgentEventEnvelope[] = [];
  const seenEvents = new Set<AgentEventName>();

  for (const event of events) {
    if (!allowedEvents.has(event.event) || seenEvents.has(event.event)) {
      continue;
    }

    visibleEvents.push(event);
    seenEvents.add(event.event);

    if (seenEvents.size === allowedEvents.size) {
      return visibleEvents;
    }
  }

  return visibleEvents;
}
