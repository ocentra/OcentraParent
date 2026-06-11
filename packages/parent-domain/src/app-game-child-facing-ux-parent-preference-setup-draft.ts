import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  AppGameChildUxLocalOutboxParentSurfaceIntentReadModelSchema,
  type AppGameChildUxLocalOutboxParentSurfaceIntentReadModel,
  type AppGameChildUxLocalOutboxParentSurfaceIntentRow,
} from './app-game-child-facing-ux-local-outbox-parent-surface-intent';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from './reference-primitives';
import { FamilyReferenceSchema } from './references';
import {
  V3NotificationParentPreferenceStateSchema,
  V3NotificationProviderChannelSchema,
  V3NotificationQuietHoursDecisionSchema,
} from './v3-notification-rule-provider-retry-contract';

const PreferenceSetupDraftText = Schema.String.pipe(Schema.minLength(1));

export const AppGameChildUxParentPreferenceSetupDraftStatus = {
  DraftReady: 'draft-ready',
  UnavailableVisible: 'unavailable-visible',
} as const;
type AppGameChildUxParentPreferenceSetupDraftStatusValue =
  (typeof AppGameChildUxParentPreferenceSetupDraftStatus)[keyof typeof AppGameChildUxParentPreferenceSetupDraftStatus];

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
export const AppGameChildUxParentPreferenceSetupDraftIdSchema = PreferenceSetupDraftText.pipe(
  Schema.brand('AppGameChildUxParentPreferenceSetupDraftId')
);
export const AppGameChildUxParentPreferenceSetupDraftReferenceSchema = PreferenceSetupDraftText.pipe(
  Schema.brand('AppGameChildUxParentPreferenceSetupDraftReference')
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

export type AppGameChildUxParentPreferenceSetupDraftRow = Infer<
  typeof AppGameChildUxParentPreferenceSetupDraftRowSchema
>;
export type AppGameChildUxParentPreferenceSetupDraftReadModel = Infer<
  typeof AppGameChildUxParentPreferenceSetupDraftReadModelSchema
>;

type DraftRowInput = Infer<typeof AppGameChildUxParentPreferenceSetupDraftRowBaseSchema>;

export type AppGameChildUxParentPreferenceSetupDraftOptions = {
  readonly generatedAt: string;
  readonly draftId: string;
  readonly sourceContractRefs: readonly string[];
};

export function buildAppGameChildUxParentPreferenceSetupDraftReadModel(
  options: AppGameChildUxParentPreferenceSetupDraftOptions,
  sourceReadModel: AppGameChildUxLocalOutboxParentSurfaceIntentReadModel
): AppGameChildUxParentPreferenceSetupDraftReadModel {
  const parsedSource = AppGameChildUxLocalOutboxParentSurfaceIntentReadModelSchema.parse(sourceReadModel);
  const rows = parsedSource.rows.map(parentPreferenceSetupDraftRow);

  return AppGameChildUxParentPreferenceSetupDraftReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    draftId: options.draftId,
    generatedAt: options.generatedAt,
    family: parsedSource.family,
    sourceParentSurfaceIntentId: parsedSource.intentId,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    draftReadyCount: countDraftStatus(rows, AppGameChildUxParentPreferenceSetupDraftStatus.DraftReady),
    unavailableVisibleCount: countDraftStatus(rows, AppGameChildUxParentPreferenceSetupDraftStatus.UnavailableVisible),
    draftNonClaims: RequiredAppGameChildUxParentPreferenceSetupDraftNonClaims,
    parentPreferenceUiRendered: false,
    parentFrequencyControlUiRendered: false,
    parentPreferenceMutationClaimed: false,
    notificationRuleMutationClaimed: false,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    childDeliveryClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    rawPrivateSourceRowsIncluded: false,
  });
}

function parentPreferenceSetupDraftRow(
  sourceRow: AppGameChildUxLocalOutboxParentSurfaceIntentRow
): AppGameChildUxParentPreferenceSetupDraftRow {
  return AppGameChildUxParentPreferenceSetupDraftRowSchema.parse({
    draftRowId: `app-game-child-ux-parent-preference-setup-draft-${sourceRow.surfaceRowId}`,
    sourceParentSurfaceRowId: sourceRow.surfaceRowId,
    sourceSchedulerEntryRef: sourceRow.sourceSchedulerEntryRef,
    sourceOutboxRecordRef: sourceRow.sourceOutboxRecordRef,
    providerChannel: sourceRow.providerChannel,
    parentPreferenceState: sourceRow.parentPreferenceState,
    quietHoursDecision: sourceRow.quietHoursDecision,
    draftStatus: draftStatusForSourceRow(sourceRow),
    preferenceRequirementRefs: preferenceRequirementRefs(sourceRow),
    quietHoursRequirementRefs: quietHoursRequirementRefs(sourceRow),
    manualProofRequirements: sourceRow.manualProofRequirements,
    parentSafeDrillInRefs: sourceRow.drillInRefs,
    parentPreferenceUiRendered: false,
    parentFrequencyControlUiRendered: false,
    parentPreferenceMutationClaimed: false,
    notificationRuleMutationClaimed: false,
    providerDeliveryClaimed: false,
    childDeliveryClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    rawPrivateSourceRowsIncluded: false,
  });
}

function draftStatusForSourceRow(
  sourceRow: AppGameChildUxLocalOutboxParentSurfaceIntentRow
): AppGameChildUxParentPreferenceSetupDraftStatusValue {
  if (sourceRow.preferenceVisibility === 'preference-setup-required') {
    return AppGameChildUxParentPreferenceSetupDraftStatus.DraftReady;
  }
  return AppGameChildUxParentPreferenceSetupDraftStatus.UnavailableVisible;
}

function preferenceRequirementRefs(sourceRow: AppGameChildUxLocalOutboxParentSurfaceIntentRow): readonly string[] {
  return sourceRow.manualProofRequirements.filter((reference) => reference.includes('parent-preference'));
}

function quietHoursRequirementRefs(sourceRow: AppGameChildUxLocalOutboxParentSurfaceIntentRow): readonly string[] {
  return sourceRow.manualProofRequirements.filter((reference) => reference.includes('quiet-hours'));
}

function countDraftStatus(
  rows: readonly AppGameChildUxParentPreferenceSetupDraftRow[],
  status: AppGameChildUxParentPreferenceSetupDraftStatusValue
): number {
  return rows.filter((row) => row.draftStatus === status).length;
}

// eslint-disable-next-line complexity -- proof honesty predicates intentionally enumerate required evidence gates.
function parentPreferenceSetupDraftReadModelIsHonest(
  readModel: Infer<typeof AppGameChildUxParentPreferenceSetupDraftReadModelBaseSchema>
): boolean {
  return (
    readModel.draftReadyCount ===
      countDraftStatus(readModel.rows, AppGameChildUxParentPreferenceSetupDraftStatus.DraftReady) &&
    readModel.unavailableVisibleCount ===
      countDraftStatus(readModel.rows, AppGameChildUxParentPreferenceSetupDraftStatus.UnavailableVisible) &&
    requiredNonClaimsPresent(readModel.draftNonClaims) &&
    readModel.parentPreferenceUiRendered === false &&
    readModel.parentFrequencyControlUiRendered === false &&
    readModel.parentPreferenceMutationClaimed === false &&
    readModel.notificationRuleMutationClaimed === false &&
    readModel.providerDeliveryRuntimeClaimed === false &&
    readModel.providerReceiptIngestionClaimed === false &&
    readModel.childDeliveryClaimed === false &&
    readModel.productionDurableOutboxStorageClaimed === false &&
    readModel.adapterDispatchClaimed === false &&
    readModel.platformEnforcementClaimed === false &&
    readModel.rawPrivateSourceRowsIncluded === false
  );
}

function parentPreferenceSetupDraftRowIsHonest(row: DraftRowInput): boolean {
  return (
    parentPreferenceRefsMatch(row) &&
    quietHoursRefsMatch(row) &&
    row.parentPreferenceUiRendered === false &&
    row.parentFrequencyControlUiRendered === false &&
    row.parentPreferenceMutationClaimed === false &&
    row.notificationRuleMutationClaimed === false &&
    row.providerDeliveryClaimed === false &&
    row.childDeliveryClaimed === false &&
    row.adapterDispatchClaimed === false &&
    row.platformEnforcementClaimed === false &&
    row.rawPrivateSourceRowsIncluded === false
  );
}

function parentPreferenceRefsMatch(row: DraftRowInput): boolean {
  if (row.draftStatus === AppGameChildUxParentPreferenceSetupDraftStatus.DraftReady) {
    return row.preferenceRequirementRefs.length > 0;
  }
  return row.preferenceRequirementRefs.length === 0;
}

function quietHoursRefsMatch(row: DraftRowInput): boolean {
  if (row.draftStatus === AppGameChildUxParentPreferenceSetupDraftStatus.DraftReady) {
    return row.quietHoursRequirementRefs.length > 0;
  }
  return row.quietHoursRequirementRefs.length === 0;
}

function requiredNonClaimsPresent(
  claims: readonly (typeof RequiredAppGameChildUxParentPreferenceSetupDraftNonClaims)[number][]
): boolean {
  return RequiredAppGameChildUxParentPreferenceSetupDraftNonClaims.every((claim) => claims.includes(claim));
}
