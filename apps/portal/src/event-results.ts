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
