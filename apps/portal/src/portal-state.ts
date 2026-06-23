import {
  AgentEvent,
  type AgentEventEnvelope,
  type AgentEventName,
} from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import { type AgentMessageTarget, type AgentWebSocketUrl } from '@ocentra-parent/schema-domain/event-primitives';
import type { AgentLogSnapshot } from '@ocentra-parent/schema-domain/logging-contracts';
import {
  PortalConnectionState,
  type PortalConnectionState as PortalConnectionStateValue,
} from '@ocentra-parent/schema-domain/portal-contracts';

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
    selectedCommandResultEvent: AgentEvent.LogSnapshotReported,
    latestSnapshot: null,
    events: [],
  };
}

function resolveAgentTarget(): AgentMessageTarget {
  return AgentProtocolDefaults.Target.LocalhostWindowsAgent;
}
