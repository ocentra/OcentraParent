import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  AppGameChildUxCopyTokenSchema,
  AppGameChildUxTargetKindSchema,
  AppGameChildUxTargetRefSchema,
} from './app-game-child-facing-ux';
import {
  AppGameNotificationAdapterDispatchState,
  AppGameNotificationDeliveryClaimState,
  AppGameNotificationIntentKind,
  AppGameNotificationIntentStatus,
  AppGameNotificationParentCopyToken,
  AppGameNotificationPayloadField,
  AppGameNotificationPriority,
  AppGameNotificationReasonCode,
  appGameNotificationIntentCopyMatchesKind,
  appGameNotificationIntentHasAuditAndEvidence,
  appGameNotificationIntentHasNoRuntimeClaims,
  appGameNotificationIntentKindRefsAreCoherent,
  appGameNotificationIntentPayloadIsMinimal,
  appGameNotificationIntentReasonMatchesKind,
  appGameNotificationIntentStatusIsHonest,
} from './app-game-notification-intent-rules';
import { ParentActionReferenceSchema, ParentDeviceReferenceSchema, ParentEvidenceReferenceSchema } from '@ocentra-parent/family-domain/references';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import { V3NotificationProviderChannelSchema } from '@ocentra-parent/notification-domain/v3-notification-rule-provider-retry-contract';

export const AppGameNotificationIntentIdSchema = brandedNonEmptyStringSchema('AppGameNotificationIntentId');
export const AppGameNotificationReferenceSchema = brandedNonEmptyStringSchema('AppGameNotificationReference');
export const AppGameNotificationParentCopyTokenSchema = withParser(
  Schema.Literal(...Object.values(AppGameNotificationParentCopyToken))
);
export const AppGameNotificationIntentKindSchema = withParser(
  Schema.Literal(...Object.values(AppGameNotificationIntentKind))
);
export const AppGameNotificationIntentStatusSchema = withParser(
  Schema.Literal(...Object.values(AppGameNotificationIntentStatus))
);
export const AppGameNotificationDeliveryClaimStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameNotificationDeliveryClaimState))
);
export const AppGameNotificationPrioritySchema = withParser(
  Schema.Literal(...Object.values(AppGameNotificationPriority))
);
export const AppGameNotificationReasonCodeSchema = withParser(
  Schema.Literal(...Object.values(AppGameNotificationReasonCode))
);
export const AppGameNotificationPayloadFieldSchema = withParser(
  Schema.Literal(...Object.values(AppGameNotificationPayloadField))
);
export const AppGameNotificationAdapterDispatchStateSchema = withParser(
  Schema.Literal(AppGameNotificationAdapterDispatchState.NotDispatched)
);

export const AppGameNotificationIntentSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    notificationIntentId: AppGameNotificationIntentIdSchema,
    intentKind: AppGameNotificationIntentKindSchema,
    intentStatus: AppGameNotificationIntentStatusSchema,
    priority: AppGameNotificationPrioritySchema,
    device: ParentDeviceReferenceSchema,
    targetKind: AppGameChildUxTargetKindSchema,
    targetRef: AppGameChildUxTargetRefSchema,
    notificationReasonCode: AppGameNotificationReasonCodeSchema,
    providerChannelPreference: V3NotificationProviderChannelSchema,
    parentTitleToken: AppGameNotificationParentCopyTokenSchema,
    parentBodyToken: AppGameNotificationParentCopyTokenSchema,
    parentActionToken: AppGameNotificationParentCopyTokenSchema,
    childTitleToken: AppGameChildUxCopyTokenSchema,
    childBodyToken: AppGameChildUxCopyTokenSchema,
    notificationRuleRef: AppGameNotificationReferenceSchema,
    notificationStatusRef: AppGameNotificationReferenceSchema,
    policyRefs: Schema.Array(AppGameNotificationReferenceSchema),
    auditRefs: Schema.Array(AppGameNotificationReferenceSchema),
    evidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
    childReasonReferences: Schema.Array(AppGameNotificationReferenceSchema),
    childStatusReferences: Schema.Array(AppGameNotificationReferenceSchema),
    approvalActionRef: Schema.Union(ParentActionReferenceSchema, Schema.Null),
    timeBudgetDecisionRef: Schema.Union(AppGameNotificationReferenceSchema, Schema.Null),
    unknownCandidateRef: Schema.Union(AppGameNotificationReferenceSchema, Schema.Null),
    localOutboxRecordRef: Schema.Union(AppGameNotificationReferenceSchema, Schema.Null),
    providerAttemptRefs: Schema.Array(AppGameNotificationReferenceSchema),
    providerReceiptRefs: Schema.Array(AppGameNotificationReferenceSchema),
    manualProofRequirements: Schema.Array(AppGameNotificationReferenceSchema),
    minimalPayloadFields: Schema.Array(AppGameNotificationPayloadFieldSchema),
    deliveryClaimState: AppGameNotificationDeliveryClaimStateSchema,
    rawChildEvidenceIncluded: Schema.Boolean,
    rawUrlOrTitleIncluded: Schema.Boolean,
    rawMessageTextIncluded: Schema.Boolean,
    screenshotOrReportIncluded: Schema.Boolean,
    providerDeliveryAttempted: Schema.Boolean,
    providerDeliveryObserved: Schema.Boolean,
    providerReceiptIngested: Schema.Boolean,
    cloudRoutingClaimed: Schema.Boolean,
    parentNotificationUiClaimed: Schema.Boolean,
    adapterDispatchState: AppGameNotificationAdapterDispatchStateSchema,
    adapterActionClaimed: Schema.Boolean,
    createdAt: ParentTimestampSchema,
  })
    .pipe(
      Schema.filter(
        (intent) =>
          appGameNotificationIntentReasonMatchesKind(intent) ||
          'Expected app/game notification reason code to match the intent kind'
      )
    )
    .pipe(
      Schema.filter(
        (intent) =>
          appGameNotificationIntentCopyMatchesKind(intent) ||
          'Expected app/game notification parent and child copy tokens to match the intent kind'
      )
    )
    .pipe(
      Schema.filter(
        (intent) =>
          appGameNotificationIntentHasAuditAndEvidence(intent) ||
          'Expected app/game notification intents to cite evidence, policy, and audit refs'
      )
    )
    .pipe(
      Schema.filter(
        (intent) =>
          appGameNotificationIntentKindRefsAreCoherent(intent) ||
          'Expected app/game notification intents to cite the refs required by their intent kind'
      )
    )
    .pipe(
      Schema.filter(
        (intent) =>
          appGameNotificationIntentStatusIsHonest(intent) ||
          'Expected app/game notification intent status to match local outbox, manual, or unavailable delivery claims'
      )
    )
    .pipe(
      Schema.filter(
        (intent) =>
          appGameNotificationIntentPayloadIsMinimal(intent) ||
          'Expected app/game notification payloads to carry minimal refs and exclude raw child details'
      )
    )
    .pipe(
      Schema.filter(
        (intent) =>
          appGameNotificationIntentHasNoRuntimeClaims(intent) ||
          'Expected app/game notification intents to avoid provider delivery, receipt, cloud, UI, and adapter claims'
      )
    )
);

export type AppGameNotificationIntent = Infer<typeof AppGameNotificationIntentSchema>;

export {
  AppGameNotificationAdapterDispatchState,
  AppGameNotificationDeliveryClaimState,
  AppGameNotificationIntentKind,
  AppGameNotificationIntentStatus,
  AppGameNotificationParentCopyToken,
  AppGameNotificationPayloadField,
  AppGameNotificationPriority,
  AppGameNotificationReasonCode,
};

export const decodeAppGameNotificationIntent = Schema.decodeUnknownSync(AppGameNotificationIntentSchema);

