import { ActivityQuerySchemaVersion } from './activity-query';
import { ActivityEvidenceRefSchema } from './evidence-contracts';
import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';

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
