import {
  AgentEvent,
  AgentProtocolDefaults,
  type AgentEventEnvelope,
  type AgentEventName,
  type AgentMessageTarget,
  type AgentWebSocketUrl,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import type { AgentLogSnapshot } from '@ocentra-parent/logging-domain/contracts';
import {
  PortalConnectionState,
  type PortalConnectionState as PortalConnectionStateValue,
} from '@ocentra-parent/portal-domain/contracts';

export interface PortalRuntimeState {
  readonly agentWsUrl: AgentWebSocketUrl;
  readonly target: AgentMessageTarget;
  socket: WebSocket | null;
  connectionState: PortalConnectionStateValue;
  selectedCommandResultEvent: AgentEventName;
  latestSnapshot: AgentLogSnapshot | null;
  readonly events: AgentEventEnvelope[];
}

export function createPortalRuntimeState(agentWsUrl: AgentWebSocketUrl): PortalRuntimeState {
  return {
    agentWsUrl,
    target: resolveAgentTarget(),
    socket: null,
    connectionState: PortalConnectionState.Disconnected,
    selectedCommandResultEvent: AgentEvent.HealthReported,
    latestSnapshot: null,
    events: [],
  };
}

function resolveAgentTarget(): AgentMessageTarget {
  return AgentProtocolDefaults.Target.LocalhostWindowsAgent;
}
