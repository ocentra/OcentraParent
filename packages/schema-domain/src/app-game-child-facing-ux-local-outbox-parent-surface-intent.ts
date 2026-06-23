import { type Infer, NonEmptyStringSchema, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import { FamilyReferenceSchema } from './family-references';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './family-reference-primitives';
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

export const RequiredAppGameChildUxLocalOutboxParentSurfaceIntentNonClaims = [
  'no-parent-notification-ui-rendered',
  'no-parent-preference-ui-rendered',
  'no-parent-frequency-control-ui-rendered',
  'no-parent-preference-mutation',
  'no-provider-delivery-execution',
  'no-provider-receipt-ingestion',
  'no-provider-credentials',
  'no-cloud-routing',
  'no-child-delivery',
  'no-production-runtime',
  'no-production-durable-outbox-storage',
  'no-adapter-dispatch',
  'no-platform-enforcement',
  'no-raw-private-source-rows',
] as const;

export const AppGameChildUxLocalOutboxParentSurfaceIntentNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameChildUxLocalOutboxParentSurfaceIntentNonClaims)
);
export const AppGameChildUxLocalOutboxParentSurfaceStatusSchema = withParser(
  Schema.Literal('manual-action-required', 'unavailable-visible')
);
export const AppGameChildUxLocalOutboxParentSurfaceHistoryVisibilitySchema = withParser(
  Schema.Literal('history-row-visible', 'manual-review-only', 'unavailable-row-visible')
);
export const AppGameChildUxLocalOutboxParentSurfacePreferenceVisibilitySchema = withParser(
  Schema.Literal('preference-setup-required', 'preference-disabled-visible')
);
export const AppGameChildUxLocalOutboxParentSurfaceIntentIdSchema = brandedNonEmptyStringSchema(
  'AppGameChildUxLocalOutboxParentSurfaceIntentId'
);
export const AppGameChildUxLocalOutboxParentSurfaceIntentReferenceSchema = brandedNonEmptyStringSchema(
  'AppGameChildUxLocalOutboxParentSurfaceIntentReference'
);

const AppGameChildUxLocalOutboxParentSurfaceIntentRowBaseSchema = Schema.Struct({
  surfaceRowId: AppGameChildUxLocalOutboxParentSurfaceIntentReferenceSchema,
  sourceProviderHandoffRowId: AppGameChildUxLocalOutboxParentSurfaceIntentReferenceSchema,
  sourcePreferenceHandoffRowId: AppGameChildUxLocalOutboxParentSurfaceIntentReferenceSchema,
  sourceSchedulerEntryRef: Schema.Union(AppGameChildUxLocalOutboxParentSurfaceIntentReferenceSchema, Schema.Null),
  sourceOutboxRecordRef: Schema.Union(AppGameChildUxLocalOutboxParentSurfaceIntentReferenceSchema, Schema.Null),
  providerStatus: V08NotificationProviderStatusSchema,
  deliveryResultState: V3NotificationDeliveryResultStateSchema,
  parentPreferenceState: V3NotificationParentPreferenceStateSchema,
  quietHoursDecision: V3NotificationQuietHoursDecisionSchema,
  providerChannel: V3NotificationProviderChannelSchema,
  parentSurfaceStatus: AppGameChildUxLocalOutboxParentSurfaceStatusSchema,
  historyVisibility: AppGameChildUxLocalOutboxParentSurfaceHistoryVisibilitySchema,
  preferenceVisibility: AppGameChildUxLocalOutboxParentSurfacePreferenceVisibilitySchema,
  drillInRefs: Schema.Array(AppGameChildUxLocalOutboxParentSurfaceIntentReferenceSchema),
  auditRefs: Schema.Array(AppGameChildUxLocalOutboxParentSurfaceIntentReferenceSchema),
  manualProofRequirements: Schema.Array(AppGameChildUxLocalOutboxParentSurfaceIntentReferenceSchema),
  minimalSurfacePayloadBoundary: NonEmptyStringSchema,
  childUxLocalOutboxSurfaceClaim: NonEmptyStringSchema,
  sensitiveDetailIncluded: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  providerReceiptClaimed: Schema.Literal(false),
  parentPreferenceMutationClaimed: Schema.Literal(false),
  childDeliveryClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
});

export const AppGameChildUxLocalOutboxParentSurfaceIntentRowSchema = withParser(
  AppGameChildUxLocalOutboxParentSurfaceIntentRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        parentSurfaceIntentRowIsHonest(row) ||
        'Expected child UX parent-surface intent rows to preserve refs, expose manual/unavailable status, and keep UI delivery adapter and platform claims false'
    )
  )
);

const AppGameChildUxLocalOutboxParentSurfaceIntentReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  intentId: AppGameChildUxLocalOutboxParentSurfaceIntentIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  sourceProviderStatusHandoffId: AppGameChildUxLocalOutboxParentSurfaceIntentReferenceSchema,
  sourcePreferenceStatusHandoffId: AppGameChildUxLocalOutboxParentSurfaceIntentReferenceSchema,
  sourceContractRefs: Schema.Array(AppGameChildUxLocalOutboxParentSurfaceIntentReferenceSchema),
  rows: Schema.Array(AppGameChildUxLocalOutboxParentSurfaceIntentRowSchema),
  manualActionRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  unavailableVisibleCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  historyVisibleCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  preferenceSetupRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  parentSurfaceNonClaims: Schema.Array(AppGameChildUxLocalOutboxParentSurfaceIntentNonClaimSchema),
  parentNotificationUiRendered: Schema.Literal(false),
  parentPreferenceUiRendered: Schema.Literal(false),
  parentFrequencyControlUiRendered: Schema.Literal(false),
  parentPreferenceMutationClaimed: Schema.Literal(false),
  providerDeliveryRuntimeClaimed: Schema.Literal(false),
  providerReceiptIngestionClaimed: Schema.Literal(false),
  providerCredentialsClaimed: Schema.Literal(false),
  cloudRoutingClaimed: Schema.Literal(false),
  childDeliveryClaimed: Schema.Literal(false),
  productionRuntimeClaimed: Schema.Literal(false),
  productionDurableOutboxStorageClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
});

export const AppGameChildUxLocalOutboxParentSurfaceIntentReadModelSchema = withParser(
  AppGameChildUxLocalOutboxParentSurfaceIntentReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        parentSurfaceIntentReadModelIsHonest(readModel) ||
        'Expected child UX parent-surface intent counts and non-claims to match row state'
    )
  )
);

export type AppGameChildUxLocalOutboxParentSurfaceIntentRow = Infer<
  typeof AppGameChildUxLocalOutboxParentSurfaceIntentRowSchema
>;
export type AppGameChildUxLocalOutboxParentSurfaceIntentReadModel = Infer<
  typeof AppGameChildUxLocalOutboxParentSurfaceIntentReadModelSchema
>;

type ParentSurfaceIntentRowInput = Infer<typeof AppGameChildUxLocalOutboxParentSurfaceIntentRowBaseSchema>;
type ParentSurfaceIntentReadModelInput = Infer<typeof AppGameChildUxLocalOutboxParentSurfaceIntentReadModelBaseSchema>;

function historyVisibilityFor(
  providerStatus: V08NotificationProviderStatus
): ParentSurfaceIntentRowInput['historyVisibility'] {
  return providerStatus === 'unavailable' ? 'unavailable-row-visible' : 'manual-review-only';
}

function parentSurfaceIntentRowIsHonest(row: ParentSurfaceIntentRowInput): boolean {
  return (
    row.drillInRefs.length > 0 &&
    row.auditRefs.length > 0 &&
    row.manualProofRequirements.length > 0 &&
    row.sensitiveDetailIncluded === false &&
    row.providerDeliveryClaimed === false &&
    row.providerReceiptClaimed === false &&
    row.parentPreferenceMutationClaimed === false &&
    row.childDeliveryClaimed === false &&
    row.adapterDispatchClaimed === false &&
    row.platformEnforcementClaimed === false
  );
}

function parentSurfaceIntentReadModelIsHonest(readModel: ParentSurfaceIntentReadModelInput): boolean {
  return (
    parentSurfaceIntentCountsAreHonest(readModel) &&
    parentSurfaceIntentNonClaimsArePresent(readModel) &&
    parentSurfaceIntentClaimsRemainScoped(readModel)
  );
}

function parentSurfaceIntentCountsAreHonest(readModel: ParentSurfaceIntentReadModelInput): boolean {
  return (
    readModel.manualActionRequiredCount === countSurfaceStatus(readModel.rows, 'manual-action-required') &&
    readModel.unavailableVisibleCount === countSurfaceStatus(readModel.rows, 'unavailable-visible') &&
    readModel.historyVisibleCount === readModel.rows.length &&
    readModel.preferenceSetupRequiredCount === countPreferenceVisibility(readModel.rows, 'preference-setup-required')
  );
}

function parentSurfaceIntentNonClaimsArePresent(readModel: ParentSurfaceIntentReadModelInput): boolean {
  return RequiredAppGameChildUxLocalOutboxParentSurfaceIntentNonClaims.every((claim) =>
    readModel.parentSurfaceNonClaims.includes(claim)
  );
}

function parentSurfaceIntentClaimsRemainScoped(readModel: ParentSurfaceIntentReadModelInput): boolean {
  return [
    readModel.parentNotificationUiRendered,
    readModel.parentPreferenceUiRendered,
    readModel.parentFrequencyControlUiRendered,
    readModel.parentPreferenceMutationClaimed,
    readModel.providerDeliveryRuntimeClaimed,
    readModel.providerReceiptIngestionClaimed,
    readModel.providerCredentialsClaimed,
    readModel.cloudRoutingClaimed,
    readModel.childDeliveryClaimed,
    readModel.productionRuntimeClaimed,
    readModel.productionDurableOutboxStorageClaimed,
    readModel.adapterDispatchClaimed,
    readModel.platformEnforcementClaimed,
    readModel.rawPrivateSourceRowsIncluded,
  ].every((claim) => claim === false);
}

const countSurfaceStatus = (
  rows: readonly ParentSurfaceIntentRowInput[],
  status: ParentSurfaceIntentRowInput['parentSurfaceStatus']
): number => rows.filter((row) => row.parentSurfaceStatus === status).length;

const countPreferenceVisibility = (
  rows: readonly ParentSurfaceIntentRowInput[],
  visibility: ParentSurfaceIntentRowInput['preferenceVisibility']
): number => rows.filter((row) => row.preferenceVisibility === visibility).length;

export { historyVisibilityFor };

export const decodeAppGameChildUxLocalOutboxParentSurfaceIntentReadModel = Schema.decodeUnknownSync(
  AppGameChildUxLocalOutboxParentSurfaceIntentReadModelSchema
);
