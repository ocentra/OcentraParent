import { ActivityEvidenceRefSchema } from '@ocentra-parent/activity-domain/contracts';
import { ActivityQuerySchemaVersion } from '@ocentra-parent/activity-domain/query';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentEvent, AgentProtocolDefaults, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

const TrackingProtocolText = Schema.String.pipe(Schema.minLength(1));
const TrackingProtocolCount = Schema.Number.pipe(Schema.nonNegative(), Schema.int());
const NullableTrackingProtocolText = Schema.Union(TrackingProtocolText, Schema.Null);

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
    evidenceReferenceIds: Schema.Array(TrackingProtocolText),
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
    capabilityStatus: TrackingProtocolText,
    latestEventId: NullableTrackingProtocolText,
    latestObservedAt: NullableTrackingProtocolText,
    evidenceReferenceIds: Schema.Array(TrackingProtocolText),
    retentionTombstoneCount: TrackingProtocolCount,
    retentionTombstoneEvidenceReferenceIds: Schema.Array(TrackingProtocolText),
    rows: Schema.Array(AgentActivityTrackingReadModelRowSchema),
  })
);

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
