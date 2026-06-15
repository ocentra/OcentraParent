import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  AppGameNotificationSchedulerBridgeReadModelSchema,
  AppGameNotificationSchedulerBridgeStatus,
  type AppGameNotificationSchedulerBridgeReadModel,
  type AppGameNotificationSchedulerBridgeRow,
} from './app-game-notification-scheduler-bridge';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/family-domain/reference-primitives';
import { FamilyReferenceSchema } from '@ocentra-parent/family-domain/references';

const PreflightText = Schema.String.pipe(Schema.minLength(1));

export const AppGameNotificationProviderPreflightStatus = {
  ProviderAdapterRequired: 'provider-adapter-required',
  ManualRequired: 'source-manual-required',
  Unavailable: 'source-unavailable',
} as const;

export const RequiredAppGameNotificationProviderPreflightNonClaims = [
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
] as const;

export const AppGameNotificationProviderPreflightStatusSchema = withParser(
  Schema.Literal(...Object.values(AppGameNotificationProviderPreflightStatus))
);
export const AppGameNotificationProviderPreflightNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameNotificationProviderPreflightNonClaims)
);

// prettier-ignore
export const AppGameNotificationProviderPreflightIdSchema = PreflightText.pipe(Schema.brand('AppGameNotificationProviderPreflightId'));
// prettier-ignore
export const AppGameNotificationProviderPreflightReferenceSchema = PreflightText.pipe(Schema.brand('AppGameNotificationProviderPreflightReference'));

const AppGameNotificationProviderPreflightRowBaseSchema = Schema.Struct({
  preflightRowId: AppGameNotificationProviderPreflightReferenceSchema,
  sourceSchedulerBridgeRecordId: AppGameNotificationProviderPreflightReferenceSchema,
  status: AppGameNotificationProviderPreflightStatusSchema,
  sourceSchedulerEntryRef: Schema.Union(AppGameNotificationProviderPreflightReferenceSchema, Schema.Null),
  sourceOutboxRecordRef: Schema.Union(AppGameNotificationProviderPreflightReferenceSchema, Schema.Null),
  schedulerDecisionRef: Schema.Union(AppGameNotificationProviderPreflightReferenceSchema, Schema.Null),
  providerChannelRef: Schema.Union(AppGameNotificationProviderPreflightReferenceSchema, Schema.Null),
  reasonCodeRef: Schema.Union(AppGameNotificationProviderPreflightReferenceSchema, Schema.Null),
  adapterRequirementRefs: Schema.Array(AppGameNotificationProviderPreflightReferenceSchema),
  manualProofRequirements: Schema.Array(AppGameNotificationProviderPreflightReferenceSchema),
});

export const AppGameNotificationProviderPreflightRowSchema = withParser(
  AppGameNotificationProviderPreflightRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        providerPreflightRowIsHonest(row) ||
        'Expected app/game provider preflight rows to require provider setup before delivery and keep manual/unavailable source rows blocked'
    )
  )
);

const AppGameNotificationProviderPreflightReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  providerPreflightId: AppGameNotificationProviderPreflightIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  sourceSchedulerBridgeId: AppGameNotificationProviderPreflightReferenceSchema,
  sourceContractRefs: Schema.Array(AppGameNotificationProviderPreflightReferenceSchema),
  rows: Schema.Array(AppGameNotificationProviderPreflightRowSchema),
  providerAdapterRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  manualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  unavailableCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  preflightNonClaims: Schema.Array(AppGameNotificationProviderPreflightNonClaimSchema),
  providerDeliveryRuntimeClaimed: Schema.Literal(false),
  providerReceiptIngestionClaimed: Schema.Literal(false),
  providerCredentialsClaimed: Schema.Literal(false),
  cloudRoutingClaimed: Schema.Literal(false),
  parentNotificationUiClaimed: Schema.Literal(false),
  childDeliveryClaimed: Schema.Literal(false),
  retryExecutionRuntimeClaimed: Schema.Literal(false),
  quietHoursTimerRuntimeClaimed: Schema.Literal(false),
  productionDurableOutboxStorageClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
});

export const AppGameNotificationProviderPreflightReadModelSchema = withParser(
  AppGameNotificationProviderPreflightReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        providerPreflightReadModelIsHonest(readModel) ||
        'Expected app/game provider preflight counts and non-claims to match adapter-required manual and unavailable rows'
    )
  )
);

export type AppGameNotificationProviderPreflightStatus = Infer<typeof AppGameNotificationProviderPreflightStatusSchema>;
export type AppGameNotificationProviderPreflightRow = Infer<typeof AppGameNotificationProviderPreflightRowSchema>;
export type AppGameNotificationProviderPreflightReadModel = Infer<
  typeof AppGameNotificationProviderPreflightReadModelSchema
>;

export type AppGameNotificationProviderPreflightOptions = {
  readonly generatedAt: string;
  readonly providerPreflightId: string;
  readonly sourceContractRefs: readonly string[];
};

export function buildAppGameNotificationProviderPreflightReadModel(
  options: AppGameNotificationProviderPreflightOptions,
  sourceReadModel: AppGameNotificationSchedulerBridgeReadModel
): AppGameNotificationProviderPreflightReadModel {
  const parsedSource = AppGameNotificationSchedulerBridgeReadModelSchema.parse(sourceReadModel);
  const rows = parsedSource.rows.map(providerPreflightRowForSchedulerRow);

  return AppGameNotificationProviderPreflightReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    providerPreflightId: options.providerPreflightId,
    generatedAt: options.generatedAt,
    family: parsedSource.family,
    sourceSchedulerBridgeId: parsedSource.schedulerBridgeId,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    providerAdapterRequiredCount: countRows(rows, AppGameNotificationProviderPreflightStatus.ProviderAdapterRequired),
    manualRequiredCount: countRows(rows, AppGameNotificationProviderPreflightStatus.ManualRequired),
    unavailableCount: countRows(rows, AppGameNotificationProviderPreflightStatus.Unavailable),
    preflightNonClaims: RequiredAppGameNotificationProviderPreflightNonClaims,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    parentNotificationUiClaimed: false,
    childDeliveryClaimed: false,
    retryExecutionRuntimeClaimed: false,
    quietHoursTimerRuntimeClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
  });
}

function providerPreflightRowForSchedulerRow(
  row: AppGameNotificationSchedulerBridgeRow
): AppGameNotificationProviderPreflightRow {
  if (row.status === AppGameNotificationSchedulerBridgeStatus.ScheduledLocal && row.schedulerRecord !== null) {
    return scheduledProviderPreflightRow(row);
  }
  return blockedProviderPreflightRow(row);
}

function scheduledProviderPreflightRow(
  row: AppGameNotificationSchedulerBridgeRow
): AppGameNotificationProviderPreflightRow {
  const record = row.schedulerRecord;
  if (record === null) {
    throw new Error(`Missing scheduler record for app/game notification preflight: ${row.schedulerBridgeRecordId}`);
  }
  const requirementRefs = [
    `provider-adapter-required-${record.schedulerEntryId}`,
    `provider-credentials-required-${record.schedulerEntryId}`,
    `provider-smoke-proof-required-${record.schedulerEntryId}`,
  ];

  return AppGameNotificationProviderPreflightRowSchema.parse({
    preflightRowId: `app-game-notification-provider-preflight-${row.schedulerBridgeRecordId}`,
    sourceSchedulerBridgeRecordId: row.schedulerBridgeRecordId,
    status: AppGameNotificationProviderPreflightStatus.ProviderAdapterRequired,
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
  row: AppGameNotificationSchedulerBridgeRow
): AppGameNotificationProviderPreflightRow {
  return AppGameNotificationProviderPreflightRowSchema.parse({
    preflightRowId: `app-game-notification-provider-preflight-${row.schedulerBridgeRecordId}`,
    sourceSchedulerBridgeRecordId: row.schedulerBridgeRecordId,
    status:
      row.status === AppGameNotificationSchedulerBridgeStatus.Unavailable
        ? AppGameNotificationProviderPreflightStatus.Unavailable
        : AppGameNotificationProviderPreflightStatus.ManualRequired,
    sourceSchedulerEntryRef: null,
    sourceOutboxRecordRef: null,
    schedulerDecisionRef: null,
    providerChannelRef: null,
    reasonCodeRef: null,
    adapterRequirementRefs: row.blockedReasonRefs,
    manualProofRequirements: row.blockedReasonRefs,
  });
}

function providerPreflightRowIsHonest(row: Infer<typeof AppGameNotificationProviderPreflightRowBaseSchema>): boolean {
  if (row.status === AppGameNotificationProviderPreflightStatus.ProviderAdapterRequired) {
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

function providerSetupRefsArePresent(row: Infer<typeof AppGameNotificationProviderPreflightRowBaseSchema>): boolean {
  return [
    row.sourceSchedulerEntryRef,
    row.sourceOutboxRecordRef,
    row.schedulerDecisionRef,
    row.providerChannelRef,
    row.reasonCodeRef,
  ].every((value) => value !== null);
}

function providerSetupRefsAreBlocked(row: Infer<typeof AppGameNotificationProviderPreflightRowBaseSchema>): boolean {
  return [
    row.sourceSchedulerEntryRef,
    row.sourceOutboxRecordRef,
    row.schedulerDecisionRef,
    row.providerChannelRef,
    row.reasonCodeRef,
  ].every((value) => value === null);
}

function providerPreflightReadModelIsHonest(
  readModel: Infer<typeof AppGameNotificationProviderPreflightReadModelBaseSchema>
): boolean {
  return (
    readModel.providerAdapterRequiredCount ===
      countRows(readModel.rows, AppGameNotificationProviderPreflightStatus.ProviderAdapterRequired) &&
    readModel.manualRequiredCount ===
      countRows(readModel.rows, AppGameNotificationProviderPreflightStatus.ManualRequired) &&
    readModel.unavailableCount === countRows(readModel.rows, AppGameNotificationProviderPreflightStatus.Unavailable) &&
    RequiredAppGameNotificationProviderPreflightNonClaims.every((claim) => readModel.preflightNonClaims.includes(claim))
  );
}

const countRows = (
  rows: ReadonlyArray<{ readonly status: AppGameNotificationProviderPreflightStatus }>,
  status: AppGameNotificationProviderPreflightStatus
): number => rows.filter((row) => row.status === status).length;
