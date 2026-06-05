import { AppGameSchemaVersion } from '@ocentra-parent/activity-domain/app-game';
import { ActivityEvidenceRefSchema } from '@ocentra-parent/activity-domain/contracts';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentEvent, AgentProtocolDefaults, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

const NotificationReadinessText = Schema.String.pipe(Schema.minLength(1));
const NotificationReadinessCount = Schema.Number.pipe(Schema.nonNegative(), Schema.int());

export const AgentAppGameNotificationReadinessReason = {
  TimeLimitExceeded: 'time-limit-exceeded',
  ApprovalRequest: 'approval-request',
  SuspiciousUnknown: 'suspicious-unknown',
  ManualRequired: 'manual-required',
  CapabilityUnavailable: 'capability-unavailable',
} as const;

export const AgentAppGameNotificationReadinessState = {
  ReadyForLocalIntent: 'ready-for-local-intent',
  ManualRequired: 'manual-required',
  Unavailable: 'unavailable',
} as const;

export const AgentAppGameNotificationReadinessRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AppGameSchemaVersion),
    rowId: NotificationReadinessText,
    reason: Schema.Literal(
      AgentAppGameNotificationReadinessReason.TimeLimitExceeded,
      AgentAppGameNotificationReadinessReason.ApprovalRequest,
      AgentAppGameNotificationReadinessReason.SuspiciousUnknown,
      AgentAppGameNotificationReadinessReason.ManualRequired,
      AgentAppGameNotificationReadinessReason.CapabilityUnavailable
    ),
    readinessState: Schema.Literal(
      AgentAppGameNotificationReadinessState.ReadyForLocalIntent,
      AgentAppGameNotificationReadinessState.ManualRequired,
      AgentAppGameNotificationReadinessState.Unavailable
    ),
    rowCount: NotificationReadinessCount,
    minimalPayloadRef: NotificationReadinessText,
    evidenceReferenceIds: Schema.Array(NotificationReadinessText),
    evidence: Schema.Array(ActivityEvidenceRefSchema),
  })
);

export const AgentAppGameNotificationReadinessReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AppGameSchemaVersion),
    generatedAt: NotificationReadinessText,
    custodyLabel: NotificationReadinessText,
    capabilityStatus: NotificationReadinessText,
    returned: NotificationReadinessCount,
    readyIntentCount: NotificationReadinessCount,
    manualRequiredCount: NotificationReadinessCount,
    unavailableCount: NotificationReadinessCount,
    providerDeliveryClaimed: Schema.Literal(false),
    providerReceiptIngestionClaimed: Schema.Literal(false),
    localOutboxRuntimeClaimed: Schema.Literal(false),
    schedulerRuntimeClaimed: Schema.Literal(false),
    adapterDispatchClaimed: Schema.Literal(false),
    parentUiClaimed: Schema.Literal(false),
    childDeliveryClaimed: Schema.Literal(false),
    rows: Schema.Array(AgentAppGameNotificationReadinessRowSchema),
  })
);

export type AgentAppGameNotificationReadinessReason = Infer<
  typeof AgentAppGameNotificationReadinessRowSchema
>['reason'];
export type AgentAppGameNotificationReadinessState = Infer<
  typeof AgentAppGameNotificationReadinessRowSchema
>['readinessState'];
export type AgentAppGameNotificationReadinessRow = Infer<typeof AgentAppGameNotificationReadinessRowSchema>;
export type AgentAppGameNotificationReadinessReadModel = Infer<typeof AgentAppGameNotificationReadinessReadModelSchema>;

export type AgentAppGameNotificationReadinessFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentAppGameNotificationReadinessResult =
  | {
      readonly ok: true;
      readonly value: AgentAppGameNotificationReadinessReadModel;
    }
  | {
      readonly ok: false;
      readonly reason: AgentAppGameNotificationReadinessFailureReason;
    };

export function parseAgentAppGameNotificationReadinessEvent(
  event: AgentEventEnvelope
): AgentAppGameNotificationReadinessResult {
  if (event.event !== AgentEvent.ActivityAppGameNotificationReadinessReadModelReported) {
    return adapterFailure('wrong-event');
  }

  const raw = event.payload[AgentProtocolDefaults.Field.ActivityAppGameNotificationReadinessReadModel];
  if (!isAgentProtocolLogText(raw)) {
    return adapterFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return adapterFailure('invalid-json');
  }

  const parsed = AgentAppGameNotificationReadinessReadModelSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return adapterFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

function adapterFailure(
  reason: AgentAppGameNotificationReadinessFailureReason
): AgentAppGameNotificationReadinessResult {
  return {
    ok: false,
    reason,
  };
}
