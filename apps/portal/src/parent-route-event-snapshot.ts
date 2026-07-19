import {
  ParentHostBridgeRuntime,
  type ParentLanDiscoveryEventRowSnapshot,
  type ParentRouteEventName,
  type ParentRouteEventSnapshot,
  type ParentRouteSnapshot,
} from '../generated/parent-ui-bridge';

export interface LanReplayPayloadCandidate {
  readonly eventId?: unknown;
  readonly eventKind?: unknown;
  readonly occurredAt?: unknown;
  readonly previousEventId?: unknown;
}

export function latestParentRouteEventSnapshot(
  events: readonly ParentRouteEventSnapshot[],
  eventName: ParentRouteEventName
): ParentRouteEventSnapshot | null {
  return events.find((event) => event.event === eventName) ?? null;
}

export function hasRequiredSnapshotEventIdentity(snapshot: ParentRouteEventSnapshot): boolean {
  return (
    hasValue(snapshot.event) &&
    hasValue(snapshot.eventId) &&
    hasValue(snapshot.sentAt) &&
    hasValue(snapshot.sourcePeerId) &&
    hasValue(snapshot.sourceRole) &&
    hasValue(snapshot.targetPeerId) &&
    hasValue(snapshot.targetRole) &&
    hasValue(snapshot.severity)
  );
}

export function parentRouteSnapshotTimestampMs(snapshot: ParentRouteSnapshot | null | undefined): number | null {
  return parentRouteTimestampMs(snapshot?.generatedAt) ?? parentRouteTimestampMs(snapshot?.lastUpdated);
}

export function latestParentRouteEventTimestampMs(events: readonly ParentRouteEventSnapshot[]): number | null {
  let latestTimestamp: number | null = null;
  for (const event of events) {
    if (!hasRequiredSnapshotEventIdentity(event)) {
      continue;
    }
    const eventTimestamp = parentRouteTimestampMs(event.sentAt);
    if (eventTimestamp === null) {
      continue;
    }
    latestTimestamp = latestTimestamp === null ? eventTimestamp : Math.max(latestTimestamp, eventTimestamp);
  }
  return latestTimestamp;
}

export function parentRouteTimestampMs(value: unknown): number | null {
  if (typeof value !== ParentHostBridgeRuntime.StringType) {
    return null;
  }
  const timestamp = String(value);
  if (timestamp.length === 0) {
    return null;
  }
  const parsed = Date.parse(timestamp);
  return Number.isFinite(parsed) ? parsed : null;
}

export function isLanReplaySnapshot(
  snapshot: ParentRouteEventSnapshot,
  historyRows: readonly ParentLanDiscoveryEventRowSnapshot[]
): boolean {
  if (lanReplayPayloadCandidate(snapshot.payload) !== null) {
    return true;
  }
  return historyRows.some((row) => row.eventId === snapshot.eventId);
}

export function lanReplayPayloadCandidate(value: unknown): LanReplayPayloadCandidate | null {
  if (!isRecord(value)) {
    return null;
  }
  const candidate = value as LanReplayPayloadCandidate;
  const hasNoReplayFields = [
    candidate.eventId,
    candidate.eventKind,
    candidate.occurredAt,
    candidate.previousEventId,
  ].every((field) => field === undefined);
  return hasNoReplayFields ? null : candidate;
}

function isRecord(value: unknown): value is Readonly<Record<PropertyKey, unknown>> {
  if (value === null || Array.isArray(value)) {
    return false;
  }
  return Object(value) === value;
}

function hasValue(value: unknown): boolean {
  return value !== undefined && value !== null;
}
