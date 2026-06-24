import {
  AgentEvent,
  type AgentEventEnvelope,
  type AgentEventName,
} from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import type { AgentLogSnapshot } from '@ocentra-parent/schema-domain/logging-contracts';
import {
  PortalConnectionState,
  type PortalConnectionState as PortalConnectionStateValue,
} from '@ocentra-parent/schema-domain/portal-contracts';
import type { ParentRouteSnapshot } from './generated/parent-ui-bridge';

export interface PortalRuntimeState {
  agentEndpoint: string;
  connectionState: PortalConnectionStateValue;
  commandEnabled: boolean;
  selectedCommandResultEvent: AgentEventName;
  latestSnapshot: AgentLogSnapshot | null;
  routeSnapshot: ParentRouteSnapshot | null;
  lastHostMessage: string | null;
  readonly events: AgentEventEnvelope[];
}

export function createPortalRuntimeState(): PortalRuntimeState {
  return {
    agentEndpoint: 'host-bridge://pending',
    connectionState: PortalConnectionState.Disconnected,
    commandEnabled: false,
    selectedCommandResultEvent: AgentEvent.LogSnapshotReported,
    latestSnapshot: null,
    routeSnapshot: null,
    lastHostMessage: null,
    events: [],
  };
}

export function applyParentRouteSnapshot(state: PortalRuntimeState, snapshot: ParentRouteSnapshot): void {
  state.routeSnapshot = snapshot;
  state.agentEndpoint = snapshot.agentEndpoint;
  state.connectionState = snapshot.connectionState;
  state.commandEnabled = snapshot.commandEnabled;
  state.lastHostMessage = snapshot.summary.routeCapability;
}
