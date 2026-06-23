import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './family-reference-primitives';
import { FamilyReferenceSchema } from './family-references';
import {
  V3NotificationParentPreferenceStateSchema,
  V3NotificationProviderChannelSchema,
  V3NotificationQuietHoursDecisionSchema,
} from './notification-v3-provider-retry';

export const AppGameChildUxParentPreferenceSetupDraftStatus = {
  DraftReady: 'draft-ready',
  UnavailableVisible: 'unavailable-visible',
} as const;

export const RequiredAppGameChildUxParentPreferenceSetupDraftNonClaims = [
  'no-parent-preference-ui-rendered',
  'no-parent-frequency-control-ui-rendered',
  'no-parent-preference-mutation',
  'no-notification-rule-mutation',
  'no-provider-delivery-execution',
  'no-provider-receipt-ingestion',
  'no-child-delivery',
  'no-production-durable-outbox-storage',
  'no-adapter-dispatch',
  'no-platform-enforcement',
  'no-raw-private-source-rows',
] as const;

export const AppGameChildUxParentPreferenceSetupDraftStatusSchema = withParser(
  Schema.Literal(...Object.values(AppGameChildUxParentPreferenceSetupDraftStatus))
);
export const AppGameChildUxParentPreferenceSetupDraftNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameChildUxParentPreferenceSetupDraftNonClaims)
);
export const AppGameChildUxParentPreferenceSetupDraftIdSchema = brandedNonEmptyStringSchema(
  'AppGameChildUxParentPreferenceSetupDraftId'
);
export const AppGameChildUxParentPreferenceSetupDraftReferenceSchema = brandedNonEmptyStringSchema(
  'AppGameChildUxParentPreferenceSetupDraftReference'
);

const AppGameChildUxParentPreferenceSetupDraftRowBaseSchema = Schema.Struct({
  draftRowId: AppGameChildUxParentPreferenceSetupDraftReferenceSchema,
  sourceParentSurfaceRowId: AppGameChildUxParentPreferenceSetupDraftReferenceSchema,
  sourceSchedulerEntryRef: Schema.Union(AppGameChildUxParentPreferenceSetupDraftReferenceSchema, Schema.Null),
  sourceOutboxRecordRef: Schema.Union(AppGameChildUxParentPreferenceSetupDraftReferenceSchema, Schema.Null),
  providerChannel: V3NotificationProviderChannelSchema,
  parentPreferenceState: V3NotificationParentPreferenceStateSchema,
  quietHoursDecision: V3NotificationQuietHoursDecisionSchema,
  draftStatus: AppGameChildUxParentPreferenceSetupDraftStatusSchema,
  preferenceRequirementRefs: Schema.Array(AppGameChildUxParentPreferenceSetupDraftReferenceSchema),
  quietHoursRequirementRefs: Schema.Array(AppGameChildUxParentPreferenceSetupDraftReferenceSchema),
  manualProofRequirements: Schema.Array(AppGameChildUxParentPreferenceSetupDraftReferenceSchema),
  parentSafeDrillInRefs: Schema.Array(AppGameChildUxParentPreferenceSetupDraftReferenceSchema),
  parentPreferenceUiRendered: Schema.Literal(false),
  parentFrequencyControlUiRendered: Schema.Literal(false),
  parentPreferenceMutationClaimed: Schema.Literal(false),
  notificationRuleMutationClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  childDeliveryClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
});

export const AppGameChildUxParentPreferenceSetupDraftRowSchema = withParser(
  AppGameChildUxParentPreferenceSetupDraftRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        parentPreferenceSetupDraftRowIsHonest(row) ||
        'Expected parent preference setup draft rows to keep setup refs parent-safe and avoid mutation delivery adapter and platform claims'
    )
  )
);

const AppGameChildUxParentPreferenceSetupDraftReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  draftId: AppGameChildUxParentPreferenceSetupDraftIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  sourceParentSurfaceIntentId: AppGameChildUxParentPreferenceSetupDraftReferenceSchema,
  sourceContractRefs: Schema.Array(AppGameChildUxParentPreferenceSetupDraftReferenceSchema),
  rows: Schema.Array(AppGameChildUxParentPreferenceSetupDraftRowSchema),
  draftReadyCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  unavailableVisibleCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  draftNonClaims: Schema.Array(AppGameChildUxParentPreferenceSetupDraftNonClaimSchema),
  parentPreferenceUiRendered: Schema.Literal(false),
  parentFrequencyControlUiRendered: Schema.Literal(false),
  parentPreferenceMutationClaimed: Schema.Literal(false),
  notificationRuleMutationClaimed: Schema.Literal(false),
  providerDeliveryRuntimeClaimed: Schema.Literal(false),
  providerReceiptIngestionClaimed: Schema.Literal(false),
  childDeliveryClaimed: Schema.Literal(false),
  productionDurableOutboxStorageClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
});

export const AppGameChildUxParentPreferenceSetupDraftReadModelSchema = withParser(
  AppGameChildUxParentPreferenceSetupDraftReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        parentPreferenceSetupDraftReadModelIsHonest(readModel) ||
        'Expected parent preference setup draft counts and non-claims to match draft rows'
    )
  )
);

export type AppGameChildUxParentPreferenceSetupDraftStatusValue = Infer<
  typeof AppGameChildUxParentPreferenceSetupDraftStatusSchema
>;
export type AppGameChildUxParentPreferenceSetupDraftRow = Infer<
  typeof AppGameChildUxParentPreferenceSetupDraftRowSchema
>;
export type AppGameChildUxParentPreferenceSetupDraftReadModel = Infer<
  typeof AppGameChildUxParentPreferenceSetupDraftReadModelSchema
>;
type ParentPreferenceSetupDraftRowInput = Infer<typeof AppGameChildUxParentPreferenceSetupDraftRowBaseSchema>;
type ParentPreferenceSetupDraftReadModelInput = Infer<
  typeof AppGameChildUxParentPreferenceSetupDraftReadModelBaseSchema
>;

function parentPreferenceSetupDraftRowIsHonest(row: ParentPreferenceSetupDraftRowInput): boolean {
  return (
    parentPreferenceSetupDraftRequirementsMatchStatus(row) &&
    [
      row.parentPreferenceUiRendered,
      row.parentFrequencyControlUiRendered,
      row.parentPreferenceMutationClaimed,
      row.notificationRuleMutationClaimed,
      row.providerDeliveryClaimed,
      row.childDeliveryClaimed,
      row.adapterDispatchClaimed,
      row.platformEnforcementClaimed,
      row.rawPrivateSourceRowsIncluded,
    ].every((claim) => claim === false)
  );
}

function parentPreferenceSetupDraftRequirementsMatchStatus(row: ParentPreferenceSetupDraftRowInput): boolean {
  return row.draftStatus === AppGameChildUxParentPreferenceSetupDraftStatus.DraftReady
    ? row.preferenceRequirementRefs.length > 0 && row.quietHoursRequirementRefs.length > 0
    : row.preferenceRequirementRefs.length === 0 && row.quietHoursRequirementRefs.length === 0;
}

function parentPreferenceSetupDraftReadModelIsHonest(readModel: ParentPreferenceSetupDraftReadModelInput): boolean {
  return (
    parentPreferenceSetupDraftCountsAreHonest(readModel) &&
    parentPreferenceSetupDraftNonClaimsArePresent(readModel) &&
    parentPreferenceSetupDraftClaimsRemainScoped(readModel)
  );
}

function parentPreferenceSetupDraftCountsAreHonest(readModel: ParentPreferenceSetupDraftReadModelInput): boolean {
  return (
    readModel.draftReadyCount ===
      readModel.rows.filter((row) => row.draftStatus === AppGameChildUxParentPreferenceSetupDraftStatus.DraftReady)
        .length &&
    readModel.unavailableVisibleCount ===
      readModel.rows.filter(
        (row) => row.draftStatus === AppGameChildUxParentPreferenceSetupDraftStatus.UnavailableVisible
      ).length
  );
}

function parentPreferenceSetupDraftNonClaimsArePresent(readModel: ParentPreferenceSetupDraftReadModelInput): boolean {
  return RequiredAppGameChildUxParentPreferenceSetupDraftNonClaims.every((claim) =>
    readModel.draftNonClaims.includes(claim)
  );
}

function parentPreferenceSetupDraftClaimsRemainScoped(readModel: ParentPreferenceSetupDraftReadModelInput): boolean {
  return [
    readModel.parentPreferenceUiRendered,
    readModel.parentFrequencyControlUiRendered,
    readModel.parentPreferenceMutationClaimed,
    readModel.notificationRuleMutationClaimed,
    readModel.providerDeliveryRuntimeClaimed,
    readModel.providerReceiptIngestionClaimed,
    readModel.childDeliveryClaimed,
    readModel.productionDurableOutboxStorageClaimed,
    readModel.adapterDispatchClaimed,
    readModel.platformEnforcementClaimed,
    readModel.rawPrivateSourceRowsIncluded,
  ].every((claim) => claim === false);
}
