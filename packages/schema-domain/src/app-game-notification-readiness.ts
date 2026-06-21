import { AppGameSchemaVersion } from './app-game-primitives';
import { ActivityEvidenceRefSchema } from './evidence-contracts';
import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';

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
    rowId: NonEmptyStringSchema,
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
    minimalPayloadRef: NonEmptyStringSchema,
    evidenceReferenceIds: Schema.Array(NonEmptyStringSchema),
    evidence: Schema.Array(ActivityEvidenceRefSchema),
  })
);

export const AgentAppGameNotificationReadinessReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AppGameSchemaVersion),
    generatedAt: NonEmptyStringSchema,
    custodyLabel: NonEmptyStringSchema,
    capabilityStatus: NonEmptyStringSchema,
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
