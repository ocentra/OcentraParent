import type { GeneratedPortalRouteEventSnapshot } from './generated-portal-contracts';
import type { PortalRouteEventRecord } from './portal-contract-adapter';

export function decodePortalRouteEvent(
  value: GeneratedPortalRouteEventSnapshot | null | undefined
): PortalRouteEventRecord | null {
  if (value === null || value === undefined) return null;

  const event: PortalRouteEventRecord = {
    event: optionalSnapshotValue(value.event),
    eventId: optionalSnapshotValue(value.eventId),
    correlationId: optionalSnapshotValue(value.correlationId),
    sentAt: optionalSnapshotValue(value.sentAt),
    sourcePeerId: optionalSnapshotValue(value.sourcePeerId),
    sourceRole: optionalSnapshotValue(value.sourceRole),
    targetPeerId: optionalSnapshotValue(value.targetPeerId),
    targetRole: optionalSnapshotValue(value.targetRole),
    severity: optionalSnapshotValue(value.severity),
    snapshot: optionalSnapshotValue(value.snapshot),
    ...(value.payload === null || value.payload === undefined ? {} : { payload: value.payload }),
  };

  return event;
}

function optionalSnapshotValue<TValue>(value: TValue | null | undefined): TValue | null {
  return value ?? null;
}
