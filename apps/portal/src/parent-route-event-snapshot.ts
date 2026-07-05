import {
  ParentHostBridgeRuntime,
  type ParentRouteEventName,
  type ParentRouteEventSnapshot,
  type ParentRouteSnapshot,
} from '../generated/parent-ui-bridge';

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

export function parentRouteSnapshotTimestampMs(
  snapshot: ParentRouteSnapshot | null | undefined
): number | null {
  return timestampMs(snapshot?.generatedAt) ?? timestampMs(snapshot?.lastUpdated);
}

export function latestParentRouteEventTimestampMs(
  events: readonly ParentRouteEventSnapshot[]
): number | null {
  let latestTimestamp: number | null = null;
  for (const event of events) {
    if (!hasRequiredSnapshotEventIdentity(event)) {
      continue;
    }
    const eventTimestamp = timestampMs(event.sentAt);
    if (eventTimestamp === null) {
      continue;
    }
    latestTimestamp = latestTimestamp === null ? eventTimestamp : Math.max(latestTimestamp, eventTimestamp);
  }
  return latestTimestamp;
}

function timestampMs(value: unknown): number | null {
  if (typeof value !== ParentHostBridgeRuntime.StringType || value.length === 0) {
    return null;
  }
  const parsed = Date.parse(value);
  return Number.isFinite(parsed) ? parsed : null;
}

function hasValue(value: unknown): boolean {
  return value !== undefined && value !== null;
}
