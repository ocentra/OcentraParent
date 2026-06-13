import { ActivityEvidenceRefSchema } from '@ocentra-parent/evidence-domain/contracts';
import { ActivityQuerySchemaVersion } from '@ocentra-parent/activity-domain/query';
import {
  type Infer,
  NonEmptyStringSchema,
  Schema,
  withParser,
} from '@ocentra-parent/schema-domain/effect';
import { AgentEvent, AgentProtocolDefaults, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

const TrackingProtocolCount = Schema.Number.pipe(Schema.nonNegative(), Schema.int());
const NullableTrackingProtocolText = Schema.Union(NonEmptyStringSchema, Schema.Null);

export const AgentActivityTrackingReadModelCountSchema = withParser(
  Schema.Struct({
    value: NonEmptyStringSchema,
    count: TrackingProtocolCount,
  })
);

export const AgentActivityTrackingReadModelRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ActivityQuerySchemaVersion),
    eventId: NonEmptyStringSchema,
    observedAt: NonEmptyStringSchema,
    deviceId: NonEmptyStringSchema,
    platform: NonEmptyStringSchema,
    observer: NonEmptyStringSchema,
    kind: NonEmptyStringSchema,
    subjectKind: NonEmptyStringSchema,
    subjectId: NonEmptyStringSchema,
    subjectDisplayName: NullableTrackingProtocolText,
    capabilityStatus: NullableTrackingProtocolText,
    queryVisibility: NonEmptyStringSchema,
    deletedAt: NullableTrackingProtocolText,
    evidenceReferenceIds: Schema.Array(NonEmptyStringSchema),
    deletedEvidenceReferenceIds: Schema.Array(NonEmptyStringSchema),
    evidence: Schema.Array(ActivityEvidenceRefSchema),
  })
);

export const AgentActivityTrackingReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ActivityQuerySchemaVersion),
    generatedAt: NonEmptyStringSchema,
    custodyLabel: NonEmptyStringSchema,
    limit: TrackingProtocolCount,
    returned: TrackingProtocolCount,
    activeRows: TrackingProtocolCount,
    tombstoneRows: TrackingProtocolCount,
    capabilityStatus: NonEmptyStringSchema,
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
    deletedEvidenceReferenceIds: Schema.Array(NonEmptyStringSchema),
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
