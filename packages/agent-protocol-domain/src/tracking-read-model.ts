import { ActivityEvidenceRefSchema } from '@ocentra-parent/activity-domain/contracts';
import { ActivityQuerySchemaVersion } from '@ocentra-parent/activity-domain/query';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentEvent, AgentProtocolDefaults, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

const TrackingProtocolText = Schema.String.pipe(Schema.minLength(1));
const TrackingProtocolCount = Schema.Number.pipe(Schema.nonNegative(), Schema.int());
const NullableTrackingProtocolText = Schema.Union(TrackingProtocolText, Schema.Null);

export const AgentActivityTrackingReadModelCountSchema = withParser(
  Schema.Struct({
    value: TrackingProtocolText,
    count: TrackingProtocolCount,
  })
);

export const AgentActivityTrackingReadModelRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ActivityQuerySchemaVersion),
    eventId: TrackingProtocolText,
    observedAt: TrackingProtocolText,
    deviceId: TrackingProtocolText,
    platform: TrackingProtocolText,
    observer: TrackingProtocolText,
    kind: TrackingProtocolText,
    subjectKind: TrackingProtocolText,
    subjectId: TrackingProtocolText,
    subjectDisplayName: NullableTrackingProtocolText,
    capabilityStatus: NullableTrackingProtocolText,
    queryVisibility: TrackingProtocolText,
    deletedAt: NullableTrackingProtocolText,
    evidenceReferenceIds: Schema.Array(TrackingProtocolText),
    deletedEvidenceReferenceIds: Schema.Array(TrackingProtocolText),
    evidence: Schema.Array(ActivityEvidenceRefSchema),
  })
);

export const AgentActivityTrackingReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ActivityQuerySchemaVersion),
    generatedAt: TrackingProtocolText,
    custodyLabel: TrackingProtocolText,
    limit: TrackingProtocolCount,
    returned: TrackingProtocolCount,
    activeRows: TrackingProtocolCount,
    tombstoneRows: TrackingProtocolCount,
    capabilityStatus: TrackingProtocolText,
    latestEventId: NullableTrackingProtocolText,
    latestObservedAt: NullableTrackingProtocolText,
    latestActiveEventId: Schema.optionalWith(NullableTrackingProtocolText, { default: () => null }),
    latestActiveObservedAt: Schema.optionalWith(NullableTrackingProtocolText, { default: () => null }),
    latestTombstoneEventId: NullableTrackingProtocolText,
    latestTombstoneObservedAt: NullableTrackingProtocolText,
    activeKindCounts: Schema.optionalWith(Schema.Array(AgentActivityTrackingReadModelCountSchema), {
      default: () => [],
    }),
    activeDeviceCounts: Schema.optionalWith(Schema.Array(AgentActivityTrackingReadModelCountSchema), {
      default: () => [],
    }),
    activeCapabilityStatusCounts: Schema.optionalWith(Schema.Array(AgentActivityTrackingReadModelCountSchema), {
      default: () => [],
    }),
    deletedEvidenceReferenceIds: Schema.Array(TrackingProtocolText),
    rows: Schema.Array(AgentActivityTrackingReadModelRowSchema),
  })
);

export type AgentActivityTrackingReadModelCount = Infer<typeof AgentActivityTrackingReadModelCountSchema>;
export type AgentActivityTrackingReadModelRow = Infer<typeof AgentActivityTrackingReadModelRowSchema>;
export type AgentActivityTrackingReadModel = Infer<typeof AgentActivityTrackingReadModelSchema>;
export type AgentActivityTrackingEvidenceReferenceIds = AgentActivityTrackingReadModelRow['evidenceReferenceIds'];

export type AgentActivityTrackingReadModelFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentActivityTrackingReadModelResult =
  | {
      readonly ok: true;
      readonly value: AgentActivityTrackingReadModel;
    }
  | {
      readonly ok: false;
      readonly reason: AgentActivityTrackingReadModelFailureReason;
    };

export function parseAgentActivityTrackingReadModelEvent(
  event: AgentEventEnvelope
): AgentActivityTrackingReadModelResult {
  if (event.event !== AgentEvent.ActivityTrackingReadModelReported) {
    return adapterFailure('wrong-event');
  }

  const raw = event.payload[AgentProtocolDefaults.Field.ActivityTrackingReadModel];
  if (!isAgentProtocolLogText(raw)) {
    return adapterFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return adapterFailure('invalid-json');
  }

  const parsed = AgentActivityTrackingReadModelSchema.safeParse(decoded);
  if (!parsed.success) {
    return adapterFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

function adapterFailure(reason: AgentActivityTrackingReadModelFailureReason): AgentActivityTrackingReadModelResult {
  return {
    ok: false,
    reason,
  };
}
