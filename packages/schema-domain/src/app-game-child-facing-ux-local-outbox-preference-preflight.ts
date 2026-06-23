import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import { FamilyReferenceSchema } from './family-references';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './family-reference-primitives';
import {
  V3NotificationParentPreferenceStateSchema,
  V3NotificationQuietHoursDecisionSchema,
} from './notification-v3-provider-retry';

export const AppGameChildUxLocalOutboxPreferencePreflightStatus = {
  ParentPreferenceRequired: 'parent-preference-required',
  ManualRequired: 'source-manual-required',
  Unavailable: 'source-unavailable',
} as const;

export const RequiredAppGameChildUxLocalOutboxPreferencePreflightNonClaims = [
  'no-parent-preference-ui',
  'no-parent-frequency-control-ui',
  'no-parent-notification-ui',
  'no-quiet-hours-timer-runtime',
  'no-provider-delivery-execution',
  'no-provider-receipt-ingestion',
  'no-provider-credentials',
  'no-cloud-routing',
  'no-child-delivery',
  'no-retry-worker-runtime',
  'no-production-durable-outbox-storage',
  'no-adapter-dispatch',
  'no-platform-enforcement',
  'no-raw-private-source-rows',
] as const;

export const AppGameChildUxLocalOutboxPreferencePreflightStatusSchema = withParser(
  Schema.Literal(...Object.values(AppGameChildUxLocalOutboxPreferencePreflightStatus))
);
export const AppGameChildUxLocalOutboxPreferencePreflightNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameChildUxLocalOutboxPreferencePreflightNonClaims)
);
export const AppGameChildUxLocalOutboxPreferencePreflightIdSchema = brandedNonEmptyStringSchema(
  'AppGameChildUxLocalOutboxPreferencePreflightId'
);
export const AppGameChildUxLocalOutboxPreferencePreflightReferenceSchema = brandedNonEmptyStringSchema(
  'AppGameChildUxLocalOutboxPreferencePreflightReference'
);

const AppGameChildUxLocalOutboxPreferencePreflightRowBaseSchema = Schema.Struct({
  preferenceRowId: AppGameChildUxLocalOutboxPreferencePreflightReferenceSchema,
  sourceSchedulerBridgeRecordId: AppGameChildUxLocalOutboxPreferencePreflightReferenceSchema,
  status: AppGameChildUxLocalOutboxPreferencePreflightStatusSchema,
  sourceSchedulerEntryRef: Schema.Union(AppGameChildUxLocalOutboxPreferencePreflightReferenceSchema, Schema.Null),
  sourceOutboxRecordRef: Schema.Union(AppGameChildUxLocalOutboxPreferencePreflightReferenceSchema, Schema.Null),
  schedulerDecisionRef: Schema.Union(AppGameChildUxLocalOutboxPreferencePreflightReferenceSchema, Schema.Null),
  providerChannelRef: Schema.Union(AppGameChildUxLocalOutboxPreferencePreflightReferenceSchema, Schema.Null),
  reasonCodeRef: Schema.Union(AppGameChildUxLocalOutboxPreferencePreflightReferenceSchema, Schema.Null),
  parentPreferenceState: Schema.Union(V3NotificationParentPreferenceStateSchema, Schema.Null),
  quietHoursDecision: Schema.Union(V3NotificationQuietHoursDecisionSchema, Schema.Null),
  parentPreferenceRequirementRefs: Schema.Array(AppGameChildUxLocalOutboxPreferencePreflightReferenceSchema),
  quietHoursRequirementRefs: Schema.Array(AppGameChildUxLocalOutboxPreferencePreflightReferenceSchema),
  manualProofRequirements: Schema.Array(AppGameChildUxLocalOutboxPreferencePreflightReferenceSchema),
});

export const AppGameChildUxLocalOutboxPreferencePreflightRowSchema = withParser(
  AppGameChildUxLocalOutboxPreferencePreflightRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        childUxPreferencePreflightRowIsHonest(row) ||
        'Expected child UX local outbox preference preflight rows to require parent preference and quiet-hours setup before delivery and keep manual/unavailable source rows blocked'
    )
  )
);

const AppGameChildUxLocalOutboxPreferencePreflightReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  preferencePreflightId: AppGameChildUxLocalOutboxPreferencePreflightIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  sourceSchedulerBridgeId: AppGameChildUxLocalOutboxPreferencePreflightReferenceSchema,
  sourceContractRefs: Schema.Array(AppGameChildUxLocalOutboxPreferencePreflightReferenceSchema),
  rows: Schema.Array(AppGameChildUxLocalOutboxPreferencePreflightRowSchema),
  parentPreferenceRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  manualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  unavailableCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  preflightNonClaims: Schema.Array(AppGameChildUxLocalOutboxPreferencePreflightNonClaimSchema),
  parentPreferenceUiClaimed: Schema.Literal(false),
  parentFrequencyControlUiClaimed: Schema.Literal(false),
  parentNotificationUiClaimed: Schema.Literal(false),
  quietHoursTimerRuntimeClaimed: Schema.Literal(false),
  childDeliveryRuntimeClaimed: Schema.Literal(false),
  providerDeliveryRuntimeClaimed: Schema.Literal(false),
  providerReceiptIngestionClaimed: Schema.Literal(false),
  providerCredentialsClaimed: Schema.Literal(false),
  cloudRoutingClaimed: Schema.Literal(false),
  retryExecutionRuntimeClaimed: Schema.Literal(false),
  productionDurableOutboxStorageClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
});

export const AppGameChildUxLocalOutboxPreferencePreflightReadModelSchema = withParser(
  AppGameChildUxLocalOutboxPreferencePreflightReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        childUxPreferencePreflightReadModelIsHonest(readModel) ||
        'Expected child UX local outbox preference preflight counts and non-claims to match parent-preference-required manual and unavailable rows'
    )
  )
);

export type AppGameChildUxLocalOutboxPreferencePreflightStatus = Infer<
  typeof AppGameChildUxLocalOutboxPreferencePreflightStatusSchema
>;
export type AppGameChildUxLocalOutboxPreferencePreflightRow = Infer<
  typeof AppGameChildUxLocalOutboxPreferencePreflightRowSchema
>;
export type AppGameChildUxLocalOutboxPreferencePreflightReadModel = Infer<
  typeof AppGameChildUxLocalOutboxPreferencePreflightReadModelSchema
>;

type PreferencePreflightRowInput = Infer<typeof AppGameChildUxLocalOutboxPreferencePreflightRowBaseSchema>;

function childUxPreferencePreflightRowIsHonest(row: PreferencePreflightRowInput): boolean {
  if (row.status === AppGameChildUxLocalOutboxPreferencePreflightStatus.ParentPreferenceRequired) {
    return (
      preferenceSetupRefsArePresent(row) &&
      row.parentPreferenceState === 'manual-setup-required' &&
      row.quietHoursDecision === 'manual-required' &&
      row.parentPreferenceRequirementRefs.length >= 2 &&
      row.quietHoursRequirementRefs.length >= 1 &&
      row.manualProofRequirements.length >= 3
    );
  }
  return (
    preferenceSetupRefsAreBlocked(row) &&
    row.parentPreferenceRequirementRefs.length > 0 &&
    row.quietHoursRequirementRefs.length > 0 &&
    row.manualProofRequirements.length > 0
  );
}

function preferenceSetupRefsArePresent(row: PreferencePreflightRowInput): boolean {
  return [
    row.sourceSchedulerEntryRef,
    row.sourceOutboxRecordRef,
    row.schedulerDecisionRef,
    row.providerChannelRef,
    row.reasonCodeRef,
  ].every((value) => value !== null);
}

function preferenceSetupRefsAreBlocked(row: PreferencePreflightRowInput): boolean {
  return (
    [
      row.sourceSchedulerEntryRef,
      row.sourceOutboxRecordRef,
      row.schedulerDecisionRef,
      row.providerChannelRef,
      row.reasonCodeRef,
      row.parentPreferenceState,
      row.quietHoursDecision,
    ].every((value) => value === null) &&
    row.status !== AppGameChildUxLocalOutboxPreferencePreflightStatus.ParentPreferenceRequired
  );
}

function childUxPreferencePreflightReadModelIsHonest(
  readModel: Infer<typeof AppGameChildUxLocalOutboxPreferencePreflightReadModelBaseSchema>
): boolean {
  return (
    childUxPreferencePreflightCountsAreHonest(readModel) &&
    childUxPreferencePreflightNonClaimsArePresent(readModel) &&
    childUxPreferencePreflightClaimsRemainScoped(readModel)
  );
}

function childUxPreferencePreflightCountsAreHonest(
  readModel: Infer<typeof AppGameChildUxLocalOutboxPreferencePreflightReadModelBaseSchema>
): boolean {
  return (
    readModel.parentPreferenceRequiredCount ===
      countRows(readModel.rows, AppGameChildUxLocalOutboxPreferencePreflightStatus.ParentPreferenceRequired) &&
    readModel.manualRequiredCount ===
      countRows(readModel.rows, AppGameChildUxLocalOutboxPreferencePreflightStatus.ManualRequired) &&
    readModel.unavailableCount ===
      countRows(readModel.rows, AppGameChildUxLocalOutboxPreferencePreflightStatus.Unavailable)
  );
}

function childUxPreferencePreflightNonClaimsArePresent(
  readModel: Infer<typeof AppGameChildUxLocalOutboxPreferencePreflightReadModelBaseSchema>
): boolean {
  return RequiredAppGameChildUxLocalOutboxPreferencePreflightNonClaims.every((claim) =>
    readModel.preflightNonClaims.includes(claim)
  );
}

function childUxPreferencePreflightClaimsRemainScoped(
  readModel: Infer<typeof AppGameChildUxLocalOutboxPreferencePreflightReadModelBaseSchema>
): boolean {
  return [
    readModel.parentPreferenceUiClaimed,
    readModel.parentFrequencyControlUiClaimed,
    readModel.parentNotificationUiClaimed,
    readModel.quietHoursTimerRuntimeClaimed,
    readModel.childDeliveryRuntimeClaimed,
    readModel.providerDeliveryRuntimeClaimed,
    readModel.providerReceiptIngestionClaimed,
    readModel.providerCredentialsClaimed,
    readModel.cloudRoutingClaimed,
    readModel.retryExecutionRuntimeClaimed,
    readModel.productionDurableOutboxStorageClaimed,
    readModel.adapterDispatchClaimed,
    readModel.platformEnforcementClaimed,
    readModel.rawPrivateSourceRowsIncluded,
  ].every((claim) => claim === false);
}

function countRows(
  rows: ReadonlyArray<{ readonly status: AppGameChildUxLocalOutboxPreferencePreflightStatus }>,
  status: AppGameChildUxLocalOutboxPreferencePreflightStatus
): number {
  return rows.filter((row) => row.status === status).length;
}

export const decodeAppGameChildUxLocalOutboxPreferencePreflightReadModel = Schema.decodeUnknownSync(
  AppGameChildUxLocalOutboxPreferencePreflightReadModelSchema
);
