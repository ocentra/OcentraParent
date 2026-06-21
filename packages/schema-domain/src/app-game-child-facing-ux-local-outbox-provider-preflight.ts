import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import { FamilyReferenceSchema } from './family-references';
import {
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from './family-reference-primitives';

export const AppGameChildUxLocalOutboxProviderPreflightStatus = {
  ProviderAdapterRequired: 'provider-adapter-required',
  ManualRequired: 'source-manual-required',
  Unavailable: 'source-unavailable',
} as const;

export const RequiredAppGameChildUxLocalOutboxProviderPreflightNonClaims = [
  'no-provider-delivery-execution',
  'no-provider-receipt-ingestion',
  'no-provider-credentials',
  'no-cloud-routing',
  'no-parent-notification-ui',
  'no-child-delivery',
  'no-retry-worker-runtime',
  'no-quiet-hours-timer-runtime',
  'no-production-durable-outbox-storage',
  'no-adapter-dispatch',
  'no-platform-enforcement',
  'no-raw-private-source-rows',
] as const;

export const AppGameChildUxLocalOutboxProviderPreflightStatusSchema = withParser(
  Schema.Literal(...Object.values(AppGameChildUxLocalOutboxProviderPreflightStatus))
);
export const AppGameChildUxLocalOutboxProviderPreflightNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameChildUxLocalOutboxProviderPreflightNonClaims)
);
export const AppGameChildUxLocalOutboxProviderPreflightIdSchema = brandedNonEmptyStringSchema(
  'AppGameChildUxLocalOutboxProviderPreflightId'
);
export const AppGameChildUxLocalOutboxProviderPreflightReferenceSchema = brandedNonEmptyStringSchema(
  'AppGameChildUxLocalOutboxProviderPreflightReference'
);

const AppGameChildUxLocalOutboxProviderPreflightRowBaseSchema = Schema.Struct({
  preflightRowId: AppGameChildUxLocalOutboxProviderPreflightReferenceSchema,
  sourceSchedulerBridgeRecordId: AppGameChildUxLocalOutboxProviderPreflightReferenceSchema,
  status: AppGameChildUxLocalOutboxProviderPreflightStatusSchema,
  sourceSchedulerEntryRef: Schema.Union(AppGameChildUxLocalOutboxProviderPreflightReferenceSchema, Schema.Null),
  sourceOutboxRecordRef: Schema.Union(AppGameChildUxLocalOutboxProviderPreflightReferenceSchema, Schema.Null),
  schedulerDecisionRef: Schema.Union(AppGameChildUxLocalOutboxProviderPreflightReferenceSchema, Schema.Null),
  providerChannelRef: Schema.Union(AppGameChildUxLocalOutboxProviderPreflightReferenceSchema, Schema.Null),
  reasonCodeRef: Schema.Union(AppGameChildUxLocalOutboxProviderPreflightReferenceSchema, Schema.Null),
  adapterRequirementRefs: Schema.Array(AppGameChildUxLocalOutboxProviderPreflightReferenceSchema),
  manualProofRequirements: Schema.Array(AppGameChildUxLocalOutboxProviderPreflightReferenceSchema),
});

export const AppGameChildUxLocalOutboxProviderPreflightRowSchema = withParser(
  AppGameChildUxLocalOutboxProviderPreflightRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        childUxProviderPreflightRowIsHonest(row) ||
        'Expected child UX local outbox provider preflight rows to require provider setup before delivery and keep manual/unavailable source rows blocked'
    )
  )
);

const AppGameChildUxLocalOutboxProviderPreflightReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  providerPreflightId: AppGameChildUxLocalOutboxProviderPreflightIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  sourceSchedulerBridgeId: AppGameChildUxLocalOutboxProviderPreflightReferenceSchema,
  sourceContractRefs: Schema.Array(AppGameChildUxLocalOutboxProviderPreflightReferenceSchema),
  rows: Schema.Array(AppGameChildUxLocalOutboxProviderPreflightRowSchema),
  providerAdapterRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  manualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  unavailableCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  preflightNonClaims: Schema.Array(AppGameChildUxLocalOutboxProviderPreflightNonClaimSchema),
  childDeliveryRuntimeClaimed: Schema.Literal(false),
  providerDeliveryRuntimeClaimed: Schema.Literal(false),
  providerReceiptIngestionClaimed: Schema.Literal(false),
  providerCredentialsClaimed: Schema.Literal(false),
  cloudRoutingClaimed: Schema.Literal(false),
  parentNotificationUiClaimed: Schema.Literal(false),
  retryExecutionRuntimeClaimed: Schema.Literal(false),
  quietHoursTimerRuntimeClaimed: Schema.Literal(false),
  productionDurableOutboxStorageClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
});

export const AppGameChildUxLocalOutboxProviderPreflightReadModelSchema = withParser(
  AppGameChildUxLocalOutboxProviderPreflightReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        childUxProviderPreflightReadModelIsHonest(readModel) ||
        'Expected child UX local outbox provider preflight counts and non-claims to match adapter-required manual and unavailable rows'
    )
  )
);

export type AppGameChildUxLocalOutboxProviderPreflightStatus = Infer<
  typeof AppGameChildUxLocalOutboxProviderPreflightStatusSchema
>;
export type AppGameChildUxLocalOutboxProviderPreflightRow = Infer<
  typeof AppGameChildUxLocalOutboxProviderPreflightRowSchema
>;
export type AppGameChildUxLocalOutboxProviderPreflightReadModel = Infer<
  typeof AppGameChildUxLocalOutboxProviderPreflightReadModelSchema
>;

function childUxProviderPreflightRowIsHonest(
  row: Infer<typeof AppGameChildUxLocalOutboxProviderPreflightRowBaseSchema>
): boolean {
  if (row.status === AppGameChildUxLocalOutboxProviderPreflightStatus.ProviderAdapterRequired) {
    return (
      providerSetupRefsArePresent(row) &&
      row.adapterRequirementRefs.length >= 3 &&
      row.manualProofRequirements.length >= 3
    );
  }
  return (
    providerSetupRefsAreBlocked(row) && row.adapterRequirementRefs.length > 0 && row.manualProofRequirements.length > 0
  );
}

function providerSetupRefsArePresent(
  row: Infer<typeof AppGameChildUxLocalOutboxProviderPreflightRowBaseSchema>
): boolean {
  return [
    row.sourceSchedulerEntryRef,
    row.sourceOutboxRecordRef,
    row.schedulerDecisionRef,
    row.providerChannelRef,
    row.reasonCodeRef,
  ].every((value) => value !== null);
}

function providerSetupRefsAreBlocked(
  row: Infer<typeof AppGameChildUxLocalOutboxProviderPreflightRowBaseSchema>
): boolean {
  return [
    row.sourceSchedulerEntryRef,
    row.sourceOutboxRecordRef,
    row.schedulerDecisionRef,
    row.providerChannelRef,
    row.reasonCodeRef,
  ].every((value) => value === null);
}

function childUxProviderPreflightReadModelIsHonest(
  readModel: Infer<typeof AppGameChildUxLocalOutboxProviderPreflightReadModelBaseSchema>
): boolean {
  return (
    readModel.providerAdapterRequiredCount ===
      countRows(readModel.rows, AppGameChildUxLocalOutboxProviderPreflightStatus.ProviderAdapterRequired) &&
    readModel.manualRequiredCount ===
      countRows(readModel.rows, AppGameChildUxLocalOutboxProviderPreflightStatus.ManualRequired) &&
    readModel.unavailableCount ===
      countRows(readModel.rows, AppGameChildUxLocalOutboxProviderPreflightStatus.Unavailable) &&
    RequiredAppGameChildUxLocalOutboxProviderPreflightNonClaims.every((claim) =>
      readModel.preflightNonClaims.includes(claim)
    ) &&
    !readModel.childDeliveryRuntimeClaimed &&
    !readModel.providerDeliveryRuntimeClaimed &&
    !readModel.providerReceiptIngestionClaimed &&
    !readModel.providerCredentialsClaimed &&
    !readModel.cloudRoutingClaimed &&
    !readModel.parentNotificationUiClaimed &&
    !readModel.retryExecutionRuntimeClaimed &&
    !readModel.quietHoursTimerRuntimeClaimed &&
    !readModel.productionDurableOutboxStorageClaimed &&
    !readModel.adapterDispatchClaimed &&
    !readModel.platformEnforcementClaimed &&
    !readModel.rawPrivateSourceRowsIncluded
  );
}

function countRows(
  rows: ReadonlyArray<{ readonly status: AppGameChildUxLocalOutboxProviderPreflightStatus }>,
  status: AppGameChildUxLocalOutboxProviderPreflightStatus
): number {
  return rows.filter((row) => row.status === status).length;
}

export const decodeAppGameChildUxLocalOutboxProviderPreflightReadModel = Schema.decodeUnknownSync(
  AppGameChildUxLocalOutboxProviderPreflightReadModelSchema
);
