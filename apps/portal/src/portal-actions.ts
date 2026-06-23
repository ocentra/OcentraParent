import type {
  AgentCommandName,
  AgentEventName,
  AgentProtocolLogFields,
} from '@ocentra-parent/schema-domain/agent-command-event-contracts';

export interface PortalRenderActions {
  reconnect(): void;
  selectCommandResult(resultEvent: AgentEventName): void;
  sendCommand(command: AgentCommandName, payload: AgentProtocolLogFields): void;
}
