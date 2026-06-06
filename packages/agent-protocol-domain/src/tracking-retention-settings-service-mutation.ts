import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  AgentCommand,
  AgentEvent,
  AgentProtocolDefaults,
  isAgentProtocolLogText,
  type AgentEventEnvelope,
} from './contracts';

const TrackingRetentionMutationText = Schema.String.pipe(Schema.minLength(1));

export const AgentTrackingRetentionSettingsMutationRequestSchema = withParser(
  Schema.Struct({
    requestId: TrackingRetentionMutationText,
    intentId: TrackingRetentionMutationText,
    settingsKind: TrackingRetentionMutationText,
    writeAction: TrackingRetentionMutationText,
    requestedValue: TrackingRetentionMutationText,
    evidenceReferenceIds: Schema.Array(TrackingRetentionMutationText),
    sourceReadModelProofRefs: Schema.Array(TrackingRetentionMutationText),
    writerBoundaryProofRefs: Schema.Array(TrackingRetentionMutationText),
    auditRefs: Schema.Array(TrackingRetentionMutationText),
  })
);

export const AgentTrackingRetentionSettingsMutationResultSchema = withParser(
  Schema.Struct({
    requestId: TrackingRetentionMutationText,
    mutationId: TrackingRetentionMutationText,
    intentId: TrackingRetentionMutationText,
    settingsKind: TrackingRetentionMutationText,
    writeAction: TrackingRetentionMutationText,
    requestedValue: TrackingRetentionMutationText,
    mutationState: Schema.Literal('accepted', 'rejected'),
    rejectionReason: Schema.Union(TrackingRetentionMutationText, Schema.Null),
    serviceMutationExecuted: Schema.Boolean,
    durablePersistenceClaimed: Schema.Literal(false),
    portalUiClaimed: Schema.Literal(false),
    platformRuntimeClaimed: Schema.Literal(false),
    childDeviceDeliveryClaimed: Schema.Literal(false),
    providerDeliveryClaimed: Schema.Literal(false),
    notificationReceiptClaimed: Schema.Literal(false),
    physicalDeviceClaimed: Schema.Literal(false),
    authorityClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
    evidenceReferenceIds: Schema.Array(TrackingRetentionMutationText),
    sourceReadModelProofRefs: Schema.Array(TrackingRetentionMutationText),
    writerBoundaryProofRefs: Schema.Array(TrackingRetentionMutationText),
    auditRefs: Schema.Array(TrackingRetentionMutationText),
  }).pipe(
    Schema.filter(
      (result) =>
        result.evidenceReferenceIds.length > 0 ||
        result.mutationState === 'rejected' ||
        'Tracking retention settings mutation results need evidence refs'
    )
  )
);

export type AgentTrackingRetentionSettingsMutationRequest = Infer<
  typeof AgentTrackingRetentionSettingsMutationRequestSchema
>;
export type AgentTrackingRetentionSettingsMutationResult = Infer<
  typeof AgentTrackingRetentionSettingsMutationResultSchema
>;

export type AgentTrackingRetentionSettingsMutationFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentTrackingRetentionSettingsMutationParseResult =
  | {
      readonly ok: true;
      readonly value: AgentTrackingRetentionSettingsMutationResult;
    }
  | {
      readonly ok: false;
      readonly reason: AgentTrackingRetentionSettingsMutationFailureReason;
    };

export function createTrackingRetentionSettingsMutationPayload(
  request: AgentTrackingRetentionSettingsMutationRequest
): Record<string, string> {
  return {
    [AgentProtocolDefaults.Field.ActivityTrackingRetentionSettingsMutation]: JSON.stringify(
      AgentTrackingRetentionSettingsMutationRequestSchema.parse(request)
    ),
  };
}

export function trackingRetentionSettingsMutationCommandName(): typeof AgentCommand.ActivityTrackingRetentionSettingsMutate {
  return AgentCommand.ActivityTrackingRetentionSettingsMutate;
}

export function parseTrackingRetentionSettingsMutationEvent(
  event: AgentEventEnvelope
): AgentTrackingRetentionSettingsMutationParseResult {
  if (event.event !== AgentEvent.ActivityTrackingRetentionSettingsMutationReported) {
    return mutationFailure('wrong-event');
  }

  const raw = event.payload[AgentProtocolDefaults.Field.ActivityTrackingRetentionSettingsMutation];
  if (!isAgentProtocolLogText(raw)) {
    return mutationFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return mutationFailure('invalid-json');
  }

  const parsed = AgentTrackingRetentionSettingsMutationResultSchema.safeParse(decoded);
  if (!parsed.success) {
    return mutationFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

function mutationFailure(
  reason: AgentTrackingRetentionSettingsMutationFailureReason
): AgentTrackingRetentionSettingsMutationParseResult {
  return {
    ok: false,
    reason,
  };
}
