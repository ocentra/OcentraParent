import type { PortalRouteEventRecord } from './portal-contract-adapter';
import { PortalCommandButtons } from './commands';
import type { GeneratedPortalAgentEventName as AgentEventName } from './generated-portal-contracts';

export const PortalCommandResultEvents: readonly AgentEventName[] = [
  ...new Set(PortalCommandButtons.map((button) => button.resultEvent)),
];

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
