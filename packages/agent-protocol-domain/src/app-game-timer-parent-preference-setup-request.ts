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
    commandBoundaryClaimed: Schema.Literal(true),
    parentPreferenceMutationClaimed: Schema.Literal(false),
    notificationRuleMutationClaimed: Schema.Literal(false),
    providerDeliveryClaimed: Schema.Literal(false),
    durableOutboxClaimed: Schema.Literal(false),
    adapterDispatchClaimed: Schema.Literal(false),
    platformEnforcementClaimed: Schema.Literal(false),
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
