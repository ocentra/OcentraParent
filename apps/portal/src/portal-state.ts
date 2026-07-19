import type {
  GeneratedAgentLogEntry,
  GeneratedAgentLogSnapshot as AgentLogSnapshot,
} from '@ocentra-parent/logging-domain/generated/logging-contracts';
import { PortalFormatting } from '@ocentra-parent/portal-domain/formatting';
import {
  ParentAgentEvent as AgentEvent,
  type ParentAgentEventName as AgentEventName,
  ParentBridgeConnectionState,
  ParentHostBridgeRuntime,
  type ParentBridgeConnectionState as ParentBridgeConnectionStateValue,
  type ParentRouteEventSnapshot,
  type ParentRouteAgentEndpoint,
  type ParentRouteId,
  type ParentRouteSnapshot,
  type ParentSubscriptionEvent,
  type ParentUiDisplayText,
} from '../generated/parent-ui-bridge';
import {
  hasRequiredSnapshotEventIdentity,
  latestParentRouteEventTimestampMs,
  parentRouteSnapshotTimestampMs,
} from './parent-route-event-snapshot';
import { isReplayBatchBoundToSnapshot } from './lan-replay-snapshot-binding';

const MAX_BUFFERED_PORTAL_EVENTS = 128;

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

export function beginParentRouteLoad(state: PortalRuntimeState, route: ParentRouteId): void {
  if (state.routeSnapshot?.route !== route) {
    state.routeSnapshot = null;
  }
  state.connectionState = ParentBridgeConnectionState.Connecting;
  state.commandEnabled = false;
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
  const boundedSnapshots = snapshots.slice(-MAX_BUFFERED_PORTAL_EVENTS);
  for (const snapshot of boundedSnapshots) {
    if (!hasRequiredSnapshotEventIdentity(snapshot)) {
      continue;
    }
    const key = portalEventBufferKey(snapshot);
    if (bufferedEventKeys.has(key)) {
      continue;
    }
    bufferedEventKeys.add(key);
    state.events.unshift(snapshot);
    if (snapshot.event === AgentEvent.LogSnapshotReported) {
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

export function applyParentSubscriptionEvent(state: PortalRuntimeState, event: ParentSubscriptionEvent): void {
  if (
    isReplayBatchBoundToSnapshot(event.events ?? [], event.snapshot) &&
    !isStaleIncomingEventBatch(state, event.events ?? [])
  ) {
    applyParentRouteEvents(state, event.events ?? []);
  }
  if (event.snapshot.route === event.route && !isStaleIncomingRouteSnapshot(state, event.snapshot)) {
    applyParentRouteSnapshot(state, event.snapshot);
  }
}

function portalEventBufferKey(event: ParentRouteEventSnapshot) {
  return [event.eventId, event.correlationId, event.sentAt, event.event, event.sourcePeerId, event.targetPeerId].join(
    PortalFormatting.EventDetailSeparator
  );
}

function isStaleIncomingEventBatch(state: PortalRuntimeState, snapshots: readonly ParentRouteEventSnapshot[]): boolean {
  const latestBufferedTimestamp = latestParentRouteEventTimestampMs(state.events);
  const latestIncomingTimestamp = latestParentRouteEventTimestampMs(snapshots);
  return (
    latestBufferedTimestamp !== null &&
    latestIncomingTimestamp !== null &&
    latestIncomingTimestamp < latestBufferedTimestamp
  );
}

function isStaleIncomingRouteSnapshot(state: PortalRuntimeState, snapshot: ParentRouteSnapshot): boolean {
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
  if (Object(value) !== value || value === null) {
    return false;
  }
  const candidate = value as {
    schemaVersion?: unknown;
    agent?: unknown;
    entries?: unknown;
  };
  return (
    Number.isFinite(candidate.schemaVersion) &&
    Object(candidate.agent) === candidate.agent &&
    candidate.agent !== null &&
    !Array.isArray(candidate.agent) &&
    Array.isArray(candidate.entries)
  );
}
