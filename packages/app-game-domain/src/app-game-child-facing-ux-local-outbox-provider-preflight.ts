import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  AppGameChildUxLocalOutboxSchedulerBridgeReadModelSchema,
  AppGameChildUxLocalOutboxSchedulerBridgeStatus,
  type AppGameChildUxLocalOutboxSchedulerBridgeReadModel,
  type AppGameChildUxLocalOutboxSchedulerBridgeRow,
} from './app-game-child-facing-ux-local-outbox-scheduler-bridge';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import { FamilyReferenceSchema } from '@ocentra-parent/family-domain/references';

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
export const AppGameChildUxLocalOutboxProviderPreflightIdSchema = brandedNonEmptyStringSchema('AppGameChildUxLocalOutboxProviderPreflightId');
export const AppGameChildUxLocalOutboxProviderPreflightReferenceSchema = brandedNonEmptyStringSchema('AppGameChildUxLocalOutboxProviderPreflightReference');

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

export type AppGameChildUxLocalOutboxProviderPreflightOptions = {
  readonly generatedAt: string;
  readonly providerPreflightId: string;
  readonly sourceContractRefs: readonly string[];
};

export function buildAppGameChildUxLocalOutboxProviderPreflightReadModel(
  options: AppGameChildUxLocalOutboxProviderPreflightOptions,
  sourceReadModel: AppGameChildUxLocalOutboxSchedulerBridgeReadModel
): AppGameChildUxLocalOutboxProviderPreflightReadModel {
  const parsedSource = AppGameChildUxLocalOutboxSchedulerBridgeReadModelSchema.parse(sourceReadModel);
  const rows = parsedSource.rows.map(providerPreflightRowForChildUxSchedulerRow);

  return AppGameChildUxLocalOutboxProviderPreflightReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    providerPreflightId: options.providerPreflightId,
    generatedAt: options.generatedAt,
    family: parsedSource.family,
    sourceSchedulerBridgeId: parsedSource.schedulerBridgeId,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    providerAdapterRequiredCount: countRows(
      rows,
      AppGameChildUxLocalOutboxProviderPreflightStatus.ProviderAdapterRequired
    ),
    manualRequiredCount: countRows(rows, AppGameChildUxLocalOutboxProviderPreflightStatus.ManualRequired),
    unavailableCount: countRows(rows, AppGameChildUxLocalOutboxProviderPreflightStatus.Unavailable),
    preflightNonClaims: RequiredAppGameChildUxLocalOutboxProviderPreflightNonClaims,
    childDeliveryRuntimeClaimed: false,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    parentNotificationUiClaimed: false,
    retryExecutionRuntimeClaimed: false,
    quietHoursTimerRuntimeClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    rawPrivateSourceRowsIncluded: false,
  });
}

function providerPreflightRowForChildUxSchedulerRow(
  row: AppGameChildUxLocalOutboxSchedulerBridgeRow
): AppGameChildUxLocalOutboxProviderPreflightRow {
  if (row.status === AppGameChildUxLocalOutboxSchedulerBridgeStatus.ScheduledLocal && row.schedulerRecord !== null) {
    return scheduledProviderPreflightRow(row);
  }
  return blockedProviderPreflightRow(row);
}

function scheduledProviderPreflightRow(
  row: AppGameChildUxLocalOutboxSchedulerBridgeRow
): AppGameChildUxLocalOutboxProviderPreflightRow {
  const record = row.schedulerRecord;
  if (record === null) {
    throw new Error(`Missing scheduler record for child UX provider preflight: ${row.schedulerBridgeRecordId}`);
  }
  const requirementRefs = [
    `child-ux-provider-adapter-required-${record.schedulerEntryId}`,
    `child-ux-provider-credentials-required-${record.schedulerEntryId}`,
    `child-ux-provider-smoke-proof-required-${record.schedulerEntryId}`,
  ];

  return AppGameChildUxLocalOutboxProviderPreflightRowSchema.parse({
    preflightRowId: `app-game-child-ux-local-outbox-provider-preflight-${row.schedulerBridgeRecordId}`,
    sourceSchedulerBridgeRecordId: row.schedulerBridgeRecordId,
    status: AppGameChildUxLocalOutboxProviderPreflightStatus.ProviderAdapterRequired,
    sourceSchedulerEntryRef: record.schedulerEntryId,
    sourceOutboxRecordRef: row.sourceOutboxRecordRef,
    schedulerDecisionRef: record.schedulerDecisionRef,
    providerChannelRef: record.providerChannel,
    reasonCodeRef: record.reasonCode,
    adapterRequirementRefs: requirementRefs,
    manualProofRequirements: requirementRefs,
  });
}

function blockedProviderPreflightRow(
  row: AppGameChildUxLocalOutboxSchedulerBridgeRow
): AppGameChildUxLocalOutboxProviderPreflightRow {
  return AppGameChildUxLocalOutboxProviderPreflightRowSchema.parse({
    preflightRowId: `app-game-child-ux-local-outbox-provider-preflight-${row.schedulerBridgeRecordId}`,
    sourceSchedulerBridgeRecordId: row.schedulerBridgeRecordId,
    status:
      row.status === AppGameChildUxLocalOutboxSchedulerBridgeStatus.Unavailable
        ? AppGameChildUxLocalOutboxProviderPreflightStatus.Unavailable
        : AppGameChildUxLocalOutboxProviderPreflightStatus.ManualRequired,
    sourceSchedulerEntryRef: null,
    sourceOutboxRecordRef: null,
    schedulerDecisionRef: null,
    providerChannelRef: null,
    reasonCodeRef: null,
    adapterRequirementRefs: row.blockedReasonRefs,
    manualProofRequirements: row.blockedReasonRefs,
  });
}

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

