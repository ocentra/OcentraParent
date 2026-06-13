import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  AppGameNotificationPreferenceStatusHandoffReadModelSchema,
  type AppGameNotificationPreferenceStatusHandoffReadModel,
  type AppGameNotificationPreferenceStatusHandoffRow,
} from './app-game-notification-preference-status-handoff';
import {
  AppGameNotificationProviderStatusHandoffReadModelSchema,
  type AppGameNotificationProviderStatusHandoffReadModel,
  type AppGameNotificationProviderStatusHandoffRow,
} from './app-game-notification-provider-status-handoff';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/family-domain/reference-primitives';
import { FamilyReferenceSchema } from '@ocentra-parent/family-domain/references';
import {
  V08NotificationProviderStatusSchema,
  type V08NotificationProviderStatus,
} from '@ocentra-parent/notification-domain/v0-8-notification-provider-status-boundary';
import {
  V3NotificationDeliveryResultStateSchema,
  V3NotificationParentPreferenceStateSchema,
  V3NotificationProviderChannelSchema,
  V3NotificationQuietHoursDecisionSchema,
} from '@ocentra-parent/notification-domain/v3-notification-rule-provider-retry-contract';

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

// prettier-ignore
export const AppGameNotificationParentSurfaceIntentIdSchema = brandedNonEmptyStringSchema('AppGameNotificationParentSurfaceIntentId');
// prettier-ignore
export const AppGameNotificationParentSurfaceIntentReferenceSchema = brandedNonEmptyStringSchema('AppGameNotificationParentSurfaceIntentReference');

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
    Schema.filter(
      (row) =>
        parentSurfaceIntentRowIsHonest(row) ||
        'Expected app/game notification parent-surface intent rows to preserve refs, expose manual/unavailable status, and keep UI/delivery claims false'
    )
  )
);

const AppGameNotificationParentSurfaceIntentReadModelBaseSchema = Schema.Struct({
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
});

export const AppGameNotificationParentSurfaceIntentReadModelSchema = withParser(
  AppGameNotificationParentSurfaceIntentReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        parentSurfaceIntentReadModelIsHonest(readModel) ||
        'Expected app/game notification parent-surface intent counts and non-claims to match row state'
    )
  )
);

export type AppGameNotificationParentSurfaceIntentRow = Infer<typeof AppGameNotificationParentSurfaceIntentRowSchema>;
export type AppGameNotificationParentSurfaceIntentReadModel = Infer<
  typeof AppGameNotificationParentSurfaceIntentReadModelSchema
>;

type ParentSurfaceIntentRowInput = Infer<typeof AppGameNotificationParentSurfaceIntentRowBaseSchema>;
type ParentSurfaceIntentReadModelInput = Infer<typeof AppGameNotificationParentSurfaceIntentReadModelBaseSchema>;

export type AppGameNotificationParentSurfaceIntentOptions = {
  readonly generatedAt: string;
  readonly intentId: string;
  readonly sourceContractRefs: readonly string[];
};

export function buildAppGameNotificationParentSurfaceIntentReadModel(
  options: AppGameNotificationParentSurfaceIntentOptions,
  providerReadModel: AppGameNotificationProviderStatusHandoffReadModel,
  preferenceReadModel: AppGameNotificationPreferenceStatusHandoffReadModel
): AppGameNotificationParentSurfaceIntentReadModel {
  const parsedProvider = AppGameNotificationProviderStatusHandoffReadModelSchema.parse(providerReadModel);
  const parsedPreference = AppGameNotificationPreferenceStatusHandoffReadModelSchema.parse(preferenceReadModel);
  assertCompatibleInputs(parsedProvider, parsedPreference);

  const rows = parsedProvider.rows.map((providerRow, index) =>
    parentSurfaceIntentRowForStatusRows(providerRow, preferenceRowAt(parsedPreference, index))
  );

  return AppGameNotificationParentSurfaceIntentReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    intentId: options.intentId,
    generatedAt: options.generatedAt,
    family: parsedProvider.family,
    sourceProviderStatusHandoffId: parsedProvider.handoffId,
    sourcePreferenceStatusHandoffId: parsedPreference.handoffId,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    manualActionRequiredCount: countSurfaceStatus(rows, 'manual-action-required'),
    unavailableVisibleCount: countSurfaceStatus(rows, 'unavailable-visible'),
    historyVisibleCount: rows.length,
    preferenceSetupRequiredCount: countPreferenceVisibility(rows, 'preference-setup-required'),
    parentSurfaceNonClaims: RequiredAppGameNotificationParentSurfaceIntentNonClaims,
    parentNotificationUiRendered: false,
    parentPreferenceUiRendered: false,
    parentFrequencyControlUiRendered: false,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    childDeliveryClaimed: false,
    productionRuntimeClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
  });
}

function assertCompatibleInputs(
  providerReadModel: AppGameNotificationProviderStatusHandoffReadModel,
  preferenceReadModel: AppGameNotificationPreferenceStatusHandoffReadModel
): void {
  if (providerReadModel.family.familyId !== preferenceReadModel.family.familyId) {
    throw new Error('Expected app/game notification parent-surface inputs to use the same family ref');
  }
  if (providerReadModel.rows.length !== preferenceReadModel.rows.length) {
    throw new Error('Expected app/game notification parent-surface inputs to have matching row counts');
  }
}

function preferenceRowAt(
  preferenceReadModel: AppGameNotificationPreferenceStatusHandoffReadModel,
  index: number
): AppGameNotificationPreferenceStatusHandoffRow {
  const preferenceRow = preferenceReadModel.rows[index];
  if (preferenceRow === undefined) {
    throw new Error('Expected app/game notification parent-surface preference row to exist');
  }
  return preferenceRow;
}

function parentSurfaceIntentRowForStatusRows(
  providerRow: AppGameNotificationProviderStatusHandoffRow,
  preferenceRow: AppGameNotificationPreferenceStatusHandoffRow
): AppGameNotificationParentSurfaceIntentRow {
  const providerEntry = providerRow.providerStatusBoundaryEntry;
  const preferenceEntry = preferenceRow.notificationPreferenceStatusEntry;

  return AppGameNotificationParentSurfaceIntentRowSchema.parse({
    surfaceRowId: `app-game-notification-parent-surface-${providerRow.handoffRowId}`,
    sourceProviderHandoffRowId: providerRow.handoffRowId,
    sourcePreferenceHandoffRowId: preferenceRow.handoffRowId,
    sourceSchedulerEntryRef: providerRow.sourceSchedulerEntryRef ?? preferenceRow.sourceSchedulerEntryRef,
    sourceOutboxRecordRef: providerRow.sourceOutboxRecordRef ?? preferenceRow.sourceOutboxRecordRef,
    providerStatus: providerEntry.providerStatus,
    deliveryResultState: preferenceEntry.deliveryResultState,
    parentPreferenceState: preferenceEntry.parentPreferenceState,
    quietHoursDecision: preferenceEntry.quietHoursDecision,
    providerChannel: preferenceEntry.providerChannel,
    parentSurfaceStatus:
      providerEntry.providerStatus === 'unavailable' ? 'unavailable-visible' : 'manual-action-required',
    historyVisibility: historyVisibilityFor(providerEntry.providerStatus),
    preferenceVisibility:
      preferenceEntry.parentPreferenceState === 'channel-disabled'
        ? 'preference-disabled-visible'
        : 'preference-setup-required',
    drillInRefs: [providerEntry.notificationStatusRef, preferenceEntry.deliveryResultRef],
    auditRefs: [...providerEntry.auditRefs, ...preferenceEntry.auditRefs],
    manualProofRequirements: [...providerEntry.manualProofRequirements, ...preferenceEntry.manualProofRequirements],
    minimalSurfacePayloadBoundary:
      'Parent surface intent contains status refs and setup requirements only; sensitive app/game evidence stays behind authenticated drill-in.',
    sensitiveDetailIncluded: false,
    providerDeliveryClaimed: false,
    providerReceiptClaimed: false,
    parentPreferenceMutationClaimed: false,
    childDeliveryClaimed: false,
  });
}

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
    row.childDeliveryClaimed === false
  );
}

function parentSurfaceIntentReadModelIsHonest(readModel: ParentSurfaceIntentReadModelInput): boolean {
  return (
    readModel.manualActionRequiredCount === countSurfaceStatus(readModel.rows, 'manual-action-required') &&
    readModel.unavailableVisibleCount === countSurfaceStatus(readModel.rows, 'unavailable-visible') &&
    readModel.historyVisibleCount === readModel.rows.length &&
    readModel.preferenceSetupRequiredCount === countPreferenceVisibility(readModel.rows, 'preference-setup-required') &&
    RequiredAppGameNotificationParentSurfaceIntentNonClaims.every((claim) =>
      readModel.parentSurfaceNonClaims.includes(claim)
    )
  );
}

const countSurfaceStatus = (
  rows: readonly ParentSurfaceIntentRowInput[],
  status: ParentSurfaceIntentRowInput['parentSurfaceStatus']
): number => rows.filter((row) => row.parentSurfaceStatus === status).length;

const countPreferenceVisibility = (
  rows: readonly ParentSurfaceIntentRowInput[],
  visibility: ParentSurfaceIntentRowInput['preferenceVisibility']
): number => rows.filter((row) => row.preferenceVisibility === visibility).length;

