import {
  AgentEvent,
  type AgentEventName,
} from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import type {
  GeneratedAgentLogEntry,
  GeneratedAgentLogSnapshot as AgentLogSnapshot,
} from '@ocentra-parent/schema-domain/generated/logging-contracts';
import {
  ParentBridgeConnectionState,
  ParentHostBridgeRuntime,
  type ParentBridgeConnectionState as ParentBridgeConnectionStateValue,
  type ParentRouteEventSnapshot,
  type ParentRouteAgentEndpoint,
  type ParentRouteSnapshot,
  type ParentSubscriptionEvent,
  type ParentUiDisplayText,
} from '../generated/parent-ui-bridge';
import {
  hasRequiredSnapshotEventIdentity,
  latestParentRouteEventTimestampMs,
  parentRouteSnapshotTimestampMs,
} from './parent-route-event-snapshot';

const MAX_BUFFERED_PORTAL_EVENTS = 128;
const LOG_SNAPSHOT_REPORTED_EVENT = 'agent.log.snapshot.reported';

export interface PortalRuntimeState {
  agentEndpoint: ParentRouteAgentEndpoint;
  connectionState: ParentBridgeConnectionStateValue;
  commandEnabled: boolean;
  selectedCommandResultEvent: AgentEventName;
  latestSnapshot: AgentLogSnapshot | null;
  routeSnapshot: ParentRouteSnapshot | null;
  lastHostMessage: ParentUiDisplayText | null;
  readonly events: ParentRouteEventSnapshot[];
}

export function createPortalRuntimeState(): PortalRuntimeState {
  return {
    agentEndpoint: ParentHostBridgeRuntime.AgentEndpointPending,
    connectionState: ParentBridgeConnectionState.Disconnected,
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

export function applyParentRouteEvents(
  state: PortalRuntimeState,
  snapshots: readonly ParentRouteEventSnapshot[]
): void {
  if (snapshots.length === 0) {
    return;
  }

  const bufferedEventKeys = new Set(state.events.map(portalEventBufferKey));
  for (const snapshot of snapshots) {
    if (!hasRequiredSnapshotEventIdentity(snapshot)) {
      continue;
    }
    const key = portalEventBufferKey(snapshot);
    if (bufferedEventKeys.has(key)) {
      continue;
    }
    bufferedEventKeys.add(key);
    state.events.unshift(snapshot);
    if (snapshot.event === LOG_SNAPSHOT_REPORTED_EVENT) {
      const latestSnapshot = toGeneratedAgentLogSnapshot(snapshot.snapshot);
      if (latestSnapshot !== null) {
        state.latestSnapshot = latestSnapshot;
      }
    }
  }

  if (state.events.length > MAX_BUFFERED_PORTAL_EVENTS) {
    state.events.length = MAX_BUFFERED_PORTAL_EVENTS;
  }
}

export function applyParentSubscriptionEvent(
  state: PortalRuntimeState,
  event: ParentSubscriptionEvent
): void {
  if (!isStaleIncomingEventBatch(state, event.events ?? [])) {
    applyParentRouteEvents(state, event.events ?? []);
  }
  if (!isStaleIncomingRouteSnapshot(state, event.snapshot)) {
    applyParentRouteSnapshot(state, event.snapshot);
  }
}

function portalEventBufferKey(event: ParentRouteEventSnapshot): string {
  return [
    event.eventId,
    event.correlationId,
    event.sentAt,
    event.event,
    event.sourcePeerId,
    event.targetPeerId,
  ].join('|');
}

function isStaleIncomingEventBatch(
  state: PortalRuntimeState,
  snapshots: readonly ParentRouteEventSnapshot[]
): boolean {
  const latestBufferedTimestamp = latestParentRouteEventTimestampMs(state.events);
  const latestIncomingTimestamp = latestParentRouteEventTimestampMs(snapshots);
  return (
    latestBufferedTimestamp !== null &&
    latestIncomingTimestamp !== null &&
    latestIncomingTimestamp < latestBufferedTimestamp
  );
}

function isStaleIncomingRouteSnapshot(
  state: PortalRuntimeState,
  snapshot: ParentRouteSnapshot
): boolean {
  const currentTimestamp = parentRouteSnapshotTimestampMs(state.routeSnapshot);
  const incomingTimestamp = parentRouteSnapshotTimestampMs(snapshot);
  return currentTimestamp !== null && incomingTimestamp !== null && incomingTimestamp < currentTimestamp;
}

function toGeneratedAgentLogSnapshot(snapshot: unknown): AgentLogSnapshot | null {
  if (!isGeneratedAgentLogSnapshot(snapshot)) {
    return null;
  }
  return {
    schemaVersion: snapshot.schemaVersion,
    agent: { ...snapshot.agent },
    entries: snapshot.entries.map(
      (entry): GeneratedAgentLogEntry => ({
        ...entry,
        fields: { ...entry.fields },
      })
    ),
  };
}

function isGeneratedAgentLogSnapshot(value: unknown): value is AgentLogSnapshot {
  if (typeof value !== 'object' || value === null) {
    return false;
  }
  const candidate = value as {
    schemaVersion?: unknown;
    agent?: unknown;
    entries?: unknown;
  };
  return (
    typeof candidate.schemaVersion === 'number' &&
    typeof candidate.agent === 'object' &&
    candidate.agent !== null &&
    !Array.isArray(candidate.agent) &&
    Array.isArray(candidate.entries)
  );
}
