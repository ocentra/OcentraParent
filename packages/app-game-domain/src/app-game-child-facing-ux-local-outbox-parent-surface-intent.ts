import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  AppGameChildUxLocalOutboxPreferenceStatusHandoffReadModelSchema,
  type AppGameChildUxLocalOutboxPreferenceStatusHandoffReadModel,
  type AppGameChildUxLocalOutboxPreferenceStatusHandoffRow,
} from './app-game-child-facing-ux-local-outbox-preference-status-handoff';
import {
  AppGameChildUxLocalOutboxProviderStatusHandoffReadModelSchema,
  type AppGameChildUxLocalOutboxProviderStatusHandoffReadModel,
  type AppGameChildUxLocalOutboxProviderStatusHandoffRow,
} from './app-game-child-facing-ux-local-outbox-provider-status-handoff';
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
export const AppGameChildUxLocalOutboxParentSurfaceIntentIdSchema = brandedNonEmptyStringSchema('AppGameChildUxLocalOutboxParentSurfaceIntentId');
export const AppGameChildUxLocalOutboxParentSurfaceIntentReferenceSchema = brandedNonEmptyStringSchema('AppGameChildUxLocalOutboxParentSurfaceIntentReference');

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

export type AppGameChildUxLocalOutboxParentSurfaceIntentOptions = {
  readonly generatedAt: string;
  readonly intentId: string;
  readonly sourceContractRefs: readonly string[];
};

export function buildAppGameChildUxLocalOutboxParentSurfaceIntentReadModel(
  options: AppGameChildUxLocalOutboxParentSurfaceIntentOptions,
  providerReadModel: AppGameChildUxLocalOutboxProviderStatusHandoffReadModel,
  preferenceReadModel: AppGameChildUxLocalOutboxPreferenceStatusHandoffReadModel
): AppGameChildUxLocalOutboxParentSurfaceIntentReadModel {
  const parsedProvider = AppGameChildUxLocalOutboxProviderStatusHandoffReadModelSchema.parse(providerReadModel);
  const parsedPreference = AppGameChildUxLocalOutboxPreferenceStatusHandoffReadModelSchema.parse(preferenceReadModel);
  assertCompatibleInputs(parsedProvider, parsedPreference);

  const rows = parsedProvider.rows.map((providerRow, index) =>
    parentSurfaceIntentRowForStatusRows(providerRow, preferenceRowAt(parsedPreference, index))
  );

  return AppGameChildUxLocalOutboxParentSurfaceIntentReadModelSchema.parse({
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
    parentSurfaceNonClaims: RequiredAppGameChildUxLocalOutboxParentSurfaceIntentNonClaims,
    parentNotificationUiRendered: false,
    parentPreferenceUiRendered: false,
    parentFrequencyControlUiRendered: false,
    parentPreferenceMutationClaimed: false,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    childDeliveryClaimed: false,
    productionRuntimeClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    rawPrivateSourceRowsIncluded: false,
  });
}

function assertCompatibleInputs(
  providerReadModel: AppGameChildUxLocalOutboxProviderStatusHandoffReadModel,
  preferenceReadModel: AppGameChildUxLocalOutboxPreferenceStatusHandoffReadModel
): void {
  if (providerReadModel.family.familyId !== preferenceReadModel.family.familyId) {
    throw new Error('Expected child UX parent-surface inputs to use the same family ref');
  }
  if (providerReadModel.rows.length !== preferenceReadModel.rows.length) {
    throw new Error('Expected child UX parent-surface inputs to have matching row counts');
  }
}

function preferenceRowAt(
  preferenceReadModel: AppGameChildUxLocalOutboxPreferenceStatusHandoffReadModel,
  index: number
): AppGameChildUxLocalOutboxPreferenceStatusHandoffRow {
  const preferenceRow = preferenceReadModel.rows[index];
  if (preferenceRow === undefined) {
    throw new Error('Expected child UX parent-surface preference row to exist');
  }
  return preferenceRow;
}

function parentSurfaceIntentRowForStatusRows(
  providerRow: AppGameChildUxLocalOutboxProviderStatusHandoffRow,
  preferenceRow: AppGameChildUxLocalOutboxPreferenceStatusHandoffRow
): AppGameChildUxLocalOutboxParentSurfaceIntentRow {
  const providerEntry = providerRow.providerStatusBoundaryEntry;
  const preferenceEntry = preferenceRow.notificationPreferenceStatusEntry;

  return AppGameChildUxLocalOutboxParentSurfaceIntentRowSchema.parse({
    surfaceRowId: `app-game-child-ux-parent-surface-${providerRow.handoffRowId}`,
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
      'Child UX parent surface intent contains setup status refs only; sensitive app/game evidence stays behind authenticated drill-in.',
    childUxLocalOutboxSurfaceClaim:
      'Parent-visible child UX local outbox setup row only; no rendered UI, delivery, preference mutation, adapter dispatch, or platform enforcement is claimed.',
    sensitiveDetailIncluded: false,
    providerDeliveryClaimed: false,
    providerReceiptClaimed: false,
    parentPreferenceMutationClaimed: false,
    childDeliveryClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
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
    row.childDeliveryClaimed === false &&
    row.adapterDispatchClaimed === false &&
    row.platformEnforcementClaimed === false
  );
}

// eslint-disable-next-line complexity -- proof honesty predicates intentionally enumerate required evidence gates.
function parentSurfaceIntentReadModelIsHonest(readModel: ParentSurfaceIntentReadModelInput): boolean {
  return (
    readModel.manualActionRequiredCount === countSurfaceStatus(readModel.rows, 'manual-action-required') &&
    readModel.unavailableVisibleCount === countSurfaceStatus(readModel.rows, 'unavailable-visible') &&
    readModel.historyVisibleCount === readModel.rows.length &&
    readModel.preferenceSetupRequiredCount === countPreferenceVisibility(readModel.rows, 'preference-setup-required') &&
    RequiredAppGameChildUxLocalOutboxParentSurfaceIntentNonClaims.every((claim) =>
      readModel.parentSurfaceNonClaims.includes(claim)
    ) &&
    !readModel.parentNotificationUiRendered &&
    !readModel.parentPreferenceUiRendered &&
    !readModel.parentFrequencyControlUiRendered &&
    !readModel.parentPreferenceMutationClaimed &&
    !readModel.providerDeliveryRuntimeClaimed &&
    !readModel.providerReceiptIngestionClaimed &&
    !readModel.providerCredentialsClaimed &&
    !readModel.cloudRoutingClaimed &&
    !readModel.childDeliveryClaimed &&
    !readModel.productionRuntimeClaimed &&
    !readModel.productionDurableOutboxStorageClaimed &&
    !readModel.adapterDispatchClaimed &&
    !readModel.platformEnforcementClaimed &&
    !readModel.rawPrivateSourceRowsIncluded
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

