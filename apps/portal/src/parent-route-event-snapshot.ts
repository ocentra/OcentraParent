import {
  type ParentRouteEventSnapshot,
  type ParentRouteSnapshot,
} from '../generated/parent-ui-bridge';

export function latestParentRouteEventSnapshot(
  events: readonly ParentRouteEventSnapshot[],
  eventName: string
): ParentRouteEventSnapshot | null {
  return events.find((event) => event.event === eventName) ?? null;
}

export function hasRequiredSnapshotEventIdentity(
  snapshot: ParentRouteEventSnapshot
): snapshot is ParentRouteEventSnapshot & {
  readonly event: NonNullable<ParentRouteEventSnapshot['event']>;
  readonly eventId: NonNullable<ParentRouteEventSnapshot['eventId']>;
  readonly sentAt: NonNullable<ParentRouteEventSnapshot['sentAt']>;
  readonly sourcePeerId: NonNullable<ParentRouteEventSnapshot['sourcePeerId']>;
  readonly sourceRole: NonNullable<ParentRouteEventSnapshot['sourceRole']>;
  readonly targetPeerId: NonNullable<ParentRouteEventSnapshot['targetPeerId']>;
  readonly targetRole: NonNullable<ParentRouteEventSnapshot['targetRole']>;
  readonly severity: NonNullable<ParentRouteEventSnapshot['severity']>;
} {
  return (
    snapshot.event !== undefined &&
    snapshot.event !== null &&
    snapshot.eventId !== undefined &&
    snapshot.eventId !== null &&
    snapshot.sentAt !== undefined &&
    snapshot.sentAt !== null &&
    snapshot.sourcePeerId !== undefined &&
    snapshot.sourcePeerId !== null &&
    snapshot.sourceRole !== undefined &&
    snapshot.sourceRole !== null &&
    snapshot.targetPeerId !== undefined &&
    snapshot.targetPeerId !== null &&
    snapshot.targetRole !== undefined &&
    snapshot.targetRole !== null &&
    snapshot.severity !== undefined &&
    snapshot.severity !== null
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

function timestampMs(value: string | null | undefined): number | null {
  if (typeof value !== 'string' || value.length === 0) {
    return null;
  }
  const parsed = Date.parse(value);
  return Number.isFinite(parsed) ? parsed : null;
}
