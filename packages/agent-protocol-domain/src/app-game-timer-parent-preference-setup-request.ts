import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentEvent, AgentProtocolDefaults, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

const NonEmptyTextSchema = Schema.String.pipe(Schema.minLength(1));

export const AppGameTimerParentPreferenceSetupRequestSchema = withParser(
  Schema.Struct({
    requestId: NonEmptyTextSchema,
    requestedAt: NonEmptyTextSchema,
    parentSurfaceIntentReferenceId: NonEmptyTextSchema,
    parentPreferenceSetupReferenceId: NonEmptyTextSchema,
    requestReferenceIds: Schema.Array(NonEmptyTextSchema).pipe(
      Schema.filter((value) => value.length > 0 || 'Expected parent preference setup request references')
    ),
  })
);

export const AppGameTimerParentPreferenceSetupRequestResultSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal('app-game-timer-parent-preference-setup-request-proof'),
    requestId: NonEmptyTextSchema,
    requestedAt: NonEmptyTextSchema,
    acceptedAt: NonEmptyTextSchema,
    requestStatus: Schema.Literal('accepted'),
    parentSurfaceIntentReferenceId: NonEmptyTextSchema,
    parentPreferenceSetupReferenceId: NonEmptyTextSchema,
    requestReferenceIds: Schema.Array(NonEmptyTextSchema).pipe(
      Schema.filter((value) => value.length > 0 || 'Expected parent preference setup request result references')
    ),
    actionResultReferenceId: NonEmptyTextSchema,
    actionResultReferenceIds: Schema.Array(NonEmptyTextSchema).pipe(
      Schema.filter((value) => value.length > 0 || 'Expected parent preference setup request action result references')
    ),
    actionResultPersistenceStatus: Schema.Literal('persisted', 'unavailable'),
    parentPreferenceMutationReceiptId: NonEmptyTextSchema,
    parentPreferenceMutationReceiptIds: Schema.Array(NonEmptyTextSchema).pipe(
      Schema.filter((value) => value.length > 0 || 'Expected parent preference setup mutation receipt references')
    ),
    parentPreferenceMutationReceiptStatus: Schema.Literal('persisted', 'unavailable'),
    parentPreferenceMutationReceiptClaimed: Schema.Boolean,
    childRuntimeDeliveryHandoffId: NonEmptyTextSchema,
    childRuntimeDeliveryHandoffIds: Schema.Array(NonEmptyTextSchema).pipe(
      Schema.filter(
        (value) => value.length > 0 || 'Expected parent preference setup child runtime delivery handoff references'
      )
    ),
    childRuntimeDeliveryHandoffStatus: Schema.Literal('handoff-ready', 'unavailable'),
    childRuntimeDeliveryHandoffClaimed: Schema.Boolean,
    childRuntimeDeliveryQueueId: NonEmptyTextSchema,
    childRuntimeDeliveryQueueIds: Schema.Array(NonEmptyTextSchema).pipe(
      Schema.filter(
        (value) => value.length > 0 || 'Expected parent preference setup child runtime delivery queue references'
      )
    ),
    childRuntimeDeliveryQueueStatus: Schema.Literal('queued', 'unavailable'),
    childRuntimeDeliveryQueueClaimed: Schema.Boolean,
    childRuntimeDeliveryDispatchId: NonEmptyTextSchema,
    childRuntimeDeliveryDispatchIds: Schema.Array(NonEmptyTextSchema).pipe(
      Schema.filter(
        (value) => value.length > 0 || 'Expected parent preference setup child runtime delivery dispatch references'
      )
    ),
    childRuntimeDeliveryDispatchStatus: Schema.Literal('dispatch-ready', 'unavailable'),
    childRuntimeDeliveryDispatchClaimed: Schema.Boolean,
    commandBoundaryClaimed: Schema.Literal(true),
    actionResultHandoffClaimed: Schema.Literal(true),
    actionResultPersistenceClaimed: Schema.Boolean,
    parentPreferenceMutationClaimed: Schema.Literal(false),
    notificationRuleMutationClaimed: Schema.Literal(false),
    providerDeliveryClaimed: Schema.Literal(false),
    providerReceiptIngestionClaimed: Schema.Literal(false),
    childRuntimeDeliveryClaimed: Schema.Literal(false),
    durableOutboxClaimed: Schema.Literal(false),
    adapterDispatchClaimed: Schema.Literal(false),
    broadBlockingClaimed: Schema.Literal(false),
    platformEnforcementClaimed: Schema.Literal(false),
    rawPrivateSourceRowsClaimed: Schema.Literal(false),
    rawTargetValuesClaimed: Schema.Literal(false),
    privateDiagnosticsClaimed: Schema.Literal(false),
  })
);

export type AppGameTimerParentPreferenceSetupRequest = Infer<typeof AppGameTimerParentPreferenceSetupRequestSchema>;
export type AppGameTimerParentPreferenceSetupRequestResult = Infer<
  typeof AppGameTimerParentPreferenceSetupRequestResultSchema
>;

export type AgentAppGameTimerParentPreferenceSetupRequestFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentAppGameTimerParentPreferenceSetupRequestResult =
  | {
      readonly ok: true;
      readonly value: AppGameTimerParentPreferenceSetupRequestResult;
    }
  | {
      readonly ok: false;
      readonly reason: AgentAppGameTimerParentPreferenceSetupRequestFailureReason;
    };

export function parseAgentAppGameTimerParentPreferenceSetupRequestEvent(
  event: AgentEventEnvelope
): AgentAppGameTimerParentPreferenceSetupRequestResult {
  if (event.event !== AgentEvent.ActivityAppGameTimerParentPreferenceSetupRequested) {
    return requestFailure('wrong-event');
  }

  const raw = event.payload[AgentProtocolDefaults.Field.ActivityAppGameTimerParentPreferenceSetupRequest];
  if (!isAgentProtocolLogText(raw)) {
    return requestFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return requestFailure('invalid-json');
  }

  const parsed = AppGameTimerParentPreferenceSetupRequestResultSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return requestFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

function requestFailure(
  reason: AgentAppGameTimerParentPreferenceSetupRequestFailureReason
): AgentAppGameTimerParentPreferenceSetupRequestResult {
  return {
    ok: false,
    reason,
  };
}
