import { type Infer, Schema, withParser, brandedNonEmptyStringSchema, NonEmptyStringSchema } from './effect';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './family-reference-primitives';
import { FamilyReferenceSchema } from './family-references';
import {
  V08NotificationProviderStatusSchema,
  type V08NotificationProviderStatus,
} from './v0-8-notification-provider-status-boundary';
import {
  V3NotificationDeliveryResultStateSchema,
  V3NotificationParentPreferenceStateSchema,
  V3NotificationProviderChannelSchema,
  V3NotificationQuietHoursDecisionSchema,
} from './notification-v3-provider-retry';

export const RequiredAppGameNotificationParentSurfaceIntentNonClaims = [
  'no-parent-notification-ui-rendered',
  'no-parent-preference-ui-rendered',
  'no-parent-frequency-control-ui-rendered',
  'no-provider-delivery-execution',
  'no-provider-receipt-ingestion',
  'no-provider-credentials',
  'no-cloud-routing',
  'no-child-delivery',
  'no-production-runtime',
  'no-production-durable-outbox-storage',
  'no-adapter-dispatch',
] as const;

export const AppGameNotificationParentSurfaceIntentNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameNotificationParentSurfaceIntentNonClaims)
);
export const AppGameNotificationParentSurfaceStatusSchema = withParser(
  Schema.Literal('manual-action-required', 'unavailable-visible')
);
export const AppGameNotificationParentSurfaceHistoryVisibilitySchema = withParser(
  Schema.Literal('history-row-visible', 'manual-review-only', 'unavailable-row-visible')
);
export const AppGameNotificationParentSurfacePreferenceVisibilitySchema = withParser(
  Schema.Literal('preference-setup-required', 'preference-disabled-visible')
);

export const AppGameNotificationParentSurfaceIntentIdSchema = brandedNonEmptyStringSchema(
  'AppGameNotificationParentSurfaceIntentId'
);
export const AppGameNotificationParentSurfaceIntentReferenceSchema = brandedNonEmptyStringSchema(
  'AppGameNotificationParentSurfaceIntentReference'
);

const AppGameNotificationParentSurfaceIntentRowBaseSchema = Schema.Struct({
  surfaceRowId: AppGameNotificationParentSurfaceIntentReferenceSchema,
  sourceProviderHandoffRowId: AppGameNotificationParentSurfaceIntentReferenceSchema,
  sourcePreferenceHandoffRowId: AppGameNotificationParentSurfaceIntentReferenceSchema,
  sourceSchedulerEntryRef: Schema.Union(AppGameNotificationParentSurfaceIntentReferenceSchema, Schema.Null),
  sourceOutboxRecordRef: Schema.Union(AppGameNotificationParentSurfaceIntentReferenceSchema, Schema.Null),
  providerStatus: V08NotificationProviderStatusSchema,
  deliveryResultState: V3NotificationDeliveryResultStateSchema,
  parentPreferenceState: V3NotificationParentPreferenceStateSchema,
  quietHoursDecision: V3NotificationQuietHoursDecisionSchema,
  providerChannel: V3NotificationProviderChannelSchema,
  parentSurfaceStatus: AppGameNotificationParentSurfaceStatusSchema,
  historyVisibility: AppGameNotificationParentSurfaceHistoryVisibilitySchema,
  preferenceVisibility: AppGameNotificationParentSurfacePreferenceVisibilitySchema,
  drillInRefs: Schema.Array(AppGameNotificationParentSurfaceIntentReferenceSchema),
  auditRefs: Schema.Array(AppGameNotificationParentSurfaceIntentReferenceSchema),
  manualProofRequirements: Schema.Array(AppGameNotificationParentSurfaceIntentReferenceSchema),
  minimalSurfacePayloadBoundary: NonEmptyStringSchema,
  sensitiveDetailIncluded: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  providerReceiptClaimed: Schema.Literal(false),
  parentPreferenceMutationClaimed: Schema.Literal(false),
  childDeliveryClaimed: Schema.Literal(false),
});

export const AppGameNotificationParentSurfaceIntentRowSchema = withParser(
  AppGameNotificationParentSurfaceIntentRowBaseSchema.pipe(
    Schema.filter((row) =>
      row.drillInRefs.length > 0 &&
      row.auditRefs.length > 0 &&
      row.manualProofRequirements.length > 0 &&
      row.sensitiveDetailIncluded === false &&
      row.providerDeliveryClaimed === false &&
      row.providerReceiptClaimed === false &&
      row.parentPreferenceMutationClaimed === false &&
      row.childDeliveryClaimed === false
        ? true
        : 'Expected app/game notification parent-surface intent rows to preserve refs, expose manual/unavailable status, and keep UI/delivery claims false'
    )
  )
);

export const AppGameNotificationParentSurfaceIntentReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    intentId: AppGameNotificationParentSurfaceIntentIdSchema,
    generatedAt: ParentTimestampSchema,
    family: FamilyReferenceSchema,
    sourceProviderStatusHandoffId: AppGameNotificationParentSurfaceIntentReferenceSchema,
    sourcePreferenceStatusHandoffId: AppGameNotificationParentSurfaceIntentReferenceSchema,
    sourceContractRefs: Schema.Array(AppGameNotificationParentSurfaceIntentReferenceSchema),
    rows: Schema.Array(AppGameNotificationParentSurfaceIntentRowSchema),
    manualActionRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    unavailableVisibleCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    historyVisibleCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    preferenceSetupRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    parentSurfaceNonClaims: Schema.Array(AppGameNotificationParentSurfaceIntentNonClaimSchema),
    parentNotificationUiRendered: Schema.Literal(false),
    parentPreferenceUiRendered: Schema.Literal(false),
    parentFrequencyControlUiRendered: Schema.Literal(false),
    providerDeliveryRuntimeClaimed: Schema.Literal(false),
    providerReceiptIngestionClaimed: Schema.Literal(false),
    providerCredentialsClaimed: Schema.Literal(false),
    cloudRoutingClaimed: Schema.Literal(false),
    childDeliveryClaimed: Schema.Literal(false),
    productionRuntimeClaimed: Schema.Literal(false),
    productionDurableOutboxStorageClaimed: Schema.Literal(false),
    adapterDispatchClaimed: Schema.Literal(false),
  }).pipe(
    Schema.filter((readModel) =>
      readModel.manualActionRequiredCount ===
        readModel.rows.filter((row) => row.parentSurfaceStatus === 'manual-action-required').length &&
      readModel.unavailableVisibleCount ===
        readModel.rows.filter((row) => row.parentSurfaceStatus === 'unavailable-visible').length &&
      readModel.historyVisibleCount === readModel.rows.length &&
      readModel.preferenceSetupRequiredCount ===
        readModel.rows.filter((row) => row.preferenceVisibility === 'preference-setup-required').length &&
      RequiredAppGameNotificationParentSurfaceIntentNonClaims.every((claim) =>
        readModel.parentSurfaceNonClaims.includes(claim)
      )
        ? true
        : 'Expected app/game notification parent-surface intent counts and non-claims to match row state'
    )
  )
);

export type AppGameNotificationParentSurfaceIntentRow = Infer<typeof AppGameNotificationParentSurfaceIntentRowSchema>;
export type AppGameNotificationParentSurfaceIntentReadModel = Infer<
  typeof AppGameNotificationParentSurfaceIntentReadModelSchema
>;

export function appGameNotificationParentSurfaceHistoryVisibilityFor(
  providerStatus: V08NotificationProviderStatus
): Infer<typeof AppGameNotificationParentSurfaceHistoryVisibilitySchema> {
  return providerStatus === 'unavailable' ? 'unavailable-row-visible' : 'manual-review-only';
}
