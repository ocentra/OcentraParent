import { type Infer, NonEmptyStringSchema, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentEvent, AgentProtocolDefaults, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

export const AppGameTimerParentPreferenceSetupRequestSchema = withParser(
  Schema.Struct({
    requestId: NonEmptyStringSchema,
    requestedAt: NonEmptyStringSchema,
    parentSurfaceIntentReferenceId: NonEmptyStringSchema,
    parentPreferenceSetupReferenceId: NonEmptyStringSchema,
    requestReferenceIds: Schema.Array(NonEmptyStringSchema).pipe(
      Schema.filter((value) => value.length > 0 || 'Expected parent preference setup request references')
    ),
  })
);

export const AppGameTimerParentPreferenceSetupRequestResultSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal('app-game-timer-parent-preference-setup-request-proof'),
    requestId: NonEmptyStringSchema,
    requestedAt: NonEmptyStringSchema,
    acceptedAt: NonEmptyStringSchema,
    requestStatus: Schema.Literal('accepted'),
    parentSurfaceIntentReferenceId: NonEmptyStringSchema,
    parentPreferenceSetupReferenceId: NonEmptyStringSchema,
    requestReferenceIds: Schema.Array(NonEmptyStringSchema).pipe(
      Schema.filter((value) => value.length > 0 || 'Expected parent preference setup request result references')
    ),
    actionResultReferenceId: NonEmptyStringSchema,
    actionResultReferenceIds: Schema.Array(NonEmptyStringSchema).pipe(
      Schema.filter((value) => value.length > 0 || 'Expected parent preference setup request action result references')
    ),
    actionResultPersistenceStatus: Schema.Literal('persisted', 'unavailable'),
    parentPreferenceMutationReceiptId: NonEmptyStringSchema,
    parentPreferenceMutationReceiptIds: Schema.Array(NonEmptyStringSchema).pipe(
      Schema.filter((value) => value.length > 0 || 'Expected parent preference setup mutation receipt references')
    ),
    parentPreferenceMutationReceiptStatus: Schema.Literal('persisted', 'unavailable'),
    parentPreferenceMutationReceiptClaimed: Schema.Boolean,
    childRuntimeDeliveryHandoffId: NonEmptyStringSchema,
    childRuntimeDeliveryHandoffIds: Schema.Array(NonEmptyStringSchema).pipe(
      Schema.filter(
        (value) => value.length > 0 || 'Expected parent preference setup child runtime delivery handoff references'
      )
    ),
    childRuntimeDeliveryHandoffStatus: Schema.Literal('handoff-ready', 'unavailable'),
    childRuntimeDeliveryHandoffClaimed: Schema.Boolean,
    childRuntimeDeliveryQueueId: NonEmptyStringSchema,
    childRuntimeDeliveryQueueIds: Schema.Array(NonEmptyStringSchema).pipe(
      Schema.filter(
        (value) => value.length > 0 || 'Expected parent preference setup child runtime delivery queue references'
      )
    ),
    childRuntimeDeliveryQueueStatus: Schema.Literal('queued', 'unavailable'),
    childRuntimeDeliveryQueueClaimed: Schema.Boolean,
    childRuntimeDeliveryDispatchId: NonEmptyStringSchema,
    childRuntimeDeliveryDispatchIds: Schema.Array(NonEmptyStringSchema).pipe(
      Schema.filter(
        (value) => value.length > 0 || 'Expected parent preference setup child runtime delivery dispatch references'
      )
    ),
    childRuntimeDeliveryDispatchStatus: Schema.Literal('dispatch-ready', 'unavailable'),
    childRuntimeDeliveryDispatchClaimed: Schema.Boolean,
    childRuntimeDeliveryReceiptRequirementId: NonEmptyStringSchema,
    childRuntimeDeliveryReceiptRequirementIds: Schema.Array(NonEmptyStringSchema).pipe(
      Schema.filter(
        (value) =>
          value.length > 0 || 'Expected parent preference setup child runtime delivery receipt requirement references'
      )
    ),
    childRuntimeDeliveryReceiptRequirementStatus: Schema.Literal('receipt-required', 'unavailable'),
    childRuntimeDeliveryReceiptRequirementClaimed: Schema.Boolean,
    childRuntimeDeliveryReceiptPendingId: NonEmptyStringSchema,
    childRuntimeDeliveryReceiptPendingIds: Schema.Array(NonEmptyStringSchema).pipe(
      Schema.filter(
        (value) =>
          value.length > 0 || 'Expected parent preference setup child runtime delivery receipt pending references'
      )
    ),
    childRuntimeDeliveryReceiptPendingStatus: Schema.Literal('receipt-pending', 'unavailable'),
    childRuntimeDeliveryReceiptPendingClaimed: Schema.Boolean,
    childRuntimeDeliveryReceiptIngestedId: NonEmptyStringSchema,
    childRuntimeDeliveryReceiptIngestedIds: Schema.Array(NonEmptyStringSchema).pipe(
      Schema.filter(
        (value) =>
          value.length > 0 || 'Expected parent preference setup child runtime delivery receipt ingested references'
      )
    ),
    childRuntimeDeliveryReceiptIngestedStatus: Schema.Literal('receipt-ingested', 'unavailable'),
    childRuntimeDeliveryReceiptIngestedClaimed: Schema.Boolean,
    durableOutboxRecordId: NonEmptyStringSchema,
    durableOutboxRecordIds: Schema.Array(NonEmptyStringSchema).pipe(
      Schema.filter((value) => value.length > 0 || 'Expected parent preference setup durable outbox references')
    ),
    durableOutboxStatus: Schema.Literal('outbox-recorded', 'unavailable'),
    providerDeliveryReadinessId: NonEmptyStringSchema,
    providerDeliveryReadinessIds: Schema.Array(NonEmptyStringSchema).pipe(
      Schema.filter(
        (value) => value.length > 0 || 'Expected parent preference setup provider delivery readiness references'
      )
    ),
    providerDeliveryReadinessStatus: Schema.Literal('provider-manual-required', 'unavailable'),
    providerDeliveryAttemptId: NonEmptyStringSchema,
    providerDeliveryAttemptIds: Schema.Array(NonEmptyStringSchema).pipe(
      Schema.filter(
        (value) => value.length > 0 || 'Expected parent preference setup provider delivery attempt references'
      )
    ),
    providerDeliveryAttemptStatus: Schema.Literal('provider-delivery-manual-required', 'unavailable'),
    providerDeliveryAdapterRequirementId: NonEmptyStringSchema,
    providerDeliveryAdapterRequirementIds: Schema.Array(NonEmptyStringSchema).pipe(
      Schema.filter(
        (value) =>
          value.length > 0 || 'Expected parent preference setup provider delivery adapter requirement references'
      )
    ),
    providerDeliveryAdapterRequirementStatus: Schema.Literal('provider-adapter-required', 'unavailable'),
    providerDeliveryCredentialRequirementId: NonEmptyStringSchema,
    providerDeliveryCredentialRequirementIds: Schema.Array(NonEmptyStringSchema).pipe(
      Schema.filter(
        (value) =>
          value.length > 0 || 'Expected parent preference setup provider delivery credential requirement references'
      )
    ),
    providerDeliveryCredentialRequirementStatus: Schema.Literal('provider-credential-proof-required', 'unavailable'),
    providerDeliveryQueueId: NonEmptyStringSchema,
    providerDeliveryQueueIds: Schema.Array(NonEmptyStringSchema).pipe(
      Schema.filter(
        (value) => value.length > 0 || 'Expected parent preference setup provider delivery queue references'
      )
    ),
    providerDeliveryQueueStatus: Schema.Literal('provider-delivery-queued', 'unavailable'),
    providerDeliveryReceiptRequirementId: NonEmptyStringSchema,
    providerDeliveryReceiptRequirementIds: Schema.Array(NonEmptyStringSchema).pipe(
      Schema.filter(
        (value) =>
          value.length > 0 || 'Expected parent preference setup provider delivery receipt requirement references'
      )
    ),
    providerDeliveryReceiptRequirementStatus: Schema.Literal('provider-delivery-receipt-required', 'unavailable'),
    providerDeliveryReceiptPendingId: NonEmptyStringSchema,
    providerDeliveryReceiptPendingIds: Schema.Array(NonEmptyStringSchema).pipe(
      Schema.filter(
        (value) => value.length > 0 || 'Expected parent preference setup provider delivery receipt pending references'
      )
    ),
    providerDeliveryReceiptPendingStatus: Schema.Literal('provider-delivery-receipt-pending', 'unavailable'),
    providerDeliveryReceiptIngestedId: NonEmptyStringSchema,
    providerDeliveryReceiptIngestedIds: Schema.Array(NonEmptyStringSchema).pipe(
      Schema.filter(
        (value) => value.length > 0 || 'Expected parent preference setup provider delivery receipt ingested references'
      )
    ),
    providerDeliveryReceiptIngestedStatus: Schema.Literal('provider-delivery-receipt-ingested', 'unavailable'),
    commandBoundaryClaimed: Schema.Literal(true),
    actionResultHandoffClaimed: Schema.Literal(true),
    actionResultPersistenceClaimed: Schema.Boolean,
    parentPreferenceMutationClaimed: Schema.Literal(false),
    notificationRuleMutationClaimed: Schema.Literal(false),
    providerDeliveryReadinessClaimed: Schema.Boolean,
    providerDeliveryAttemptClaimed: Schema.Boolean,
    providerDeliveryAdapterRequirementClaimed: Schema.Boolean,
    providerDeliveryCredentialRequirementClaimed: Schema.Boolean,
    providerDeliveryQueueClaimed: Schema.Boolean,
    providerDeliveryReceiptRequirementClaimed: Schema.Boolean,
    providerDeliveryReceiptPendingClaimed: Schema.Boolean,
    providerDeliveryReceiptIngestedClaimed: Schema.Boolean,
    providerDeliveryClaimed: Schema.Literal(false),
    providerReceiptIngestionClaimed: Schema.Literal(false),
    childRuntimeDeliveryClaimed: Schema.Literal(false),
    durableOutboxClaimed: Schema.Boolean,
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
