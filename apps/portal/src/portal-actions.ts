import type {
  AgentCommandName,
  AgentEventName,
  AgentProtocolLogFields,
} from '@ocentra-parent/agent-protocol-domain/contracts';

export interface PortalRenderActions {
  reconnect(): void;
  selectCommandResult(resultEvent: AgentEventName): void;
  sendCommand(command: AgentCommandName, payload: AgentProtocolLogFields): void;
}
