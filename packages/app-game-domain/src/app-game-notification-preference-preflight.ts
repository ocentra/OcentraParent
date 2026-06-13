import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
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
import {
  V3NotificationParentPreferenceStateSchema,
  V3NotificationQuietHoursDecisionSchema,
} from '@ocentra-parent/notification-domain/v3-notification-rule-provider-retry-contract';

export const AppGameNotificationPreferencePreflightStatus = {
  ParentPreferenceRequired: 'parent-preference-required',
  ManualRequired: 'source-manual-required',
  Unavailable: 'source-unavailable',
} as const;

export const RequiredAppGameNotificationPreferencePreflightNonClaims = [
  'no-parent-preference-ui',
  'no-parent-frequency-control-ui',
  'no-quiet-hours-timer-runtime',
  'no-provider-delivery-execution',
  'no-provider-receipt-ingestion',
  'no-provider-credentials',
  'no-cloud-routing',
  'no-child-delivery',
  'no-retry-worker-runtime',
  'no-production-durable-outbox-storage',
  'no-adapter-dispatch',
] as const;

export const AppGameNotificationPreferencePreflightStatusSchema = withParser(
  Schema.Literal(...Object.values(AppGameNotificationPreferencePreflightStatus))
);
export const AppGameNotificationPreferencePreflightNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameNotificationPreferencePreflightNonClaims)
);

// prettier-ignore
export const AppGameNotificationPreferencePreflightIdSchema = brandedNonEmptyStringSchema('AppGameNotificationPreferencePreflightId');
// prettier-ignore
export const AppGameNotificationPreferencePreflightReferenceSchema = brandedNonEmptyStringSchema('AppGameNotificationPreferencePreflightReference');

const AppGameNotificationPreferencePreflightRowBaseSchema = Schema.Struct({
  preferenceRowId: AppGameNotificationPreferencePreflightReferenceSchema,
  sourceSchedulerBridgeRecordId: AppGameNotificationPreferencePreflightReferenceSchema,
  status: AppGameNotificationPreferencePreflightStatusSchema,
  sourceSchedulerEntryRef: Schema.Union(AppGameNotificationPreferencePreflightReferenceSchema, Schema.Null),
  sourceOutboxRecordRef: Schema.Union(AppGameNotificationPreferencePreflightReferenceSchema, Schema.Null),
  providerChannelRef: Schema.Union(AppGameNotificationPreferencePreflightReferenceSchema, Schema.Null),
  reasonCodeRef: Schema.Union(AppGameNotificationPreferencePreflightReferenceSchema, Schema.Null),
  parentPreferenceState: Schema.Union(V3NotificationParentPreferenceStateSchema, Schema.Null),
  quietHoursDecision: Schema.Union(V3NotificationQuietHoursDecisionSchema, Schema.Null),
  parentPreferenceRequirementRefs: Schema.Array(AppGameNotificationPreferencePreflightReferenceSchema),
  quietHoursRequirementRefs: Schema.Array(AppGameNotificationPreferencePreflightReferenceSchema),
  manualProofRequirements: Schema.Array(AppGameNotificationPreferencePreflightReferenceSchema),
});

export const AppGameNotificationPreferencePreflightRowSchema = withParser(
  AppGameNotificationPreferencePreflightRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        preferencePreflightRowIsHonest(row) ||
        'Expected app/game preference preflight rows to require parent preference setup before delivery and keep manual/unavailable source rows blocked'
    )
  )
);

const AppGameNotificationPreferencePreflightReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  preferencePreflightId: AppGameNotificationPreferencePreflightIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  sourceSchedulerBridgeId: AppGameNotificationPreferencePreflightReferenceSchema,
  sourceContractRefs: Schema.Array(AppGameNotificationPreferencePreflightReferenceSchema),
  rows: Schema.Array(AppGameNotificationPreferencePreflightRowSchema),
  parentPreferenceRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  manualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  unavailableCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  preflightNonClaims: Schema.Array(AppGameNotificationPreferencePreflightNonClaimSchema),
  parentPreferenceUiClaimed: Schema.Literal(false),
  parentFrequencyControlUiClaimed: Schema.Literal(false),
  quietHoursTimerRuntimeClaimed: Schema.Literal(false),
  providerDeliveryRuntimeClaimed: Schema.Literal(false),
  providerReceiptIngestionClaimed: Schema.Literal(false),
  providerCredentialsClaimed: Schema.Literal(false),
  cloudRoutingClaimed: Schema.Literal(false),
  childDeliveryClaimed: Schema.Literal(false),
  retryExecutionRuntimeClaimed: Schema.Literal(false),
  productionDurableOutboxStorageClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
});

export const AppGameNotificationPreferencePreflightReadModelSchema = withParser(
  AppGameNotificationPreferencePreflightReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        preferencePreflightReadModelIsHonest(readModel) ||
        'Expected app/game preference preflight counts and non-claims to match parent-preference-required manual and unavailable rows'
    )
  )
);

export type AppGameNotificationPreferencePreflightStatus = Infer<
  typeof AppGameNotificationPreferencePreflightStatusSchema
>;
export type AppGameNotificationPreferencePreflightRow = Infer<typeof AppGameNotificationPreferencePreflightRowSchema>;
export type AppGameNotificationPreferencePreflightReadModel = Infer<
  typeof AppGameNotificationPreferencePreflightReadModelSchema
>;

export type AppGameNotificationPreferencePreflightOptions = {
  readonly generatedAt: string;
  readonly preferencePreflightId: string;
  readonly sourceContractRefs: readonly string[];
};

export function buildAppGameNotificationPreferencePreflightReadModel(
  options: AppGameNotificationPreferencePreflightOptions,
  sourceReadModel: AppGameNotificationSchedulerBridgeReadModel
): AppGameNotificationPreferencePreflightReadModel {
  const parsedSource = AppGameNotificationSchedulerBridgeReadModelSchema.parse(sourceReadModel);
  const rows = parsedSource.rows.map(preferencePreflightRowForSchedulerRow);

  return AppGameNotificationPreferencePreflightReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    preferencePreflightId: options.preferencePreflightId,
    generatedAt: options.generatedAt,
    family: parsedSource.family,
    sourceSchedulerBridgeId: parsedSource.schedulerBridgeId,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    parentPreferenceRequiredCount: countRows(
      rows,
      AppGameNotificationPreferencePreflightStatus.ParentPreferenceRequired
    ),
    manualRequiredCount: countRows(rows, AppGameNotificationPreferencePreflightStatus.ManualRequired),
    unavailableCount: countRows(rows, AppGameNotificationPreferencePreflightStatus.Unavailable),
    preflightNonClaims: RequiredAppGameNotificationPreferencePreflightNonClaims,
    parentPreferenceUiClaimed: false,
    parentFrequencyControlUiClaimed: false,
    quietHoursTimerRuntimeClaimed: false,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    childDeliveryClaimed: false,
    retryExecutionRuntimeClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
  });
}

function preferencePreflightRowForSchedulerRow(
  row: AppGameNotificationSchedulerBridgeRow
): AppGameNotificationPreferencePreflightRow {
  if (row.status === AppGameNotificationSchedulerBridgeStatus.ScheduledLocal && row.schedulerRecord !== null) {
    return scheduledPreferencePreflightRow(row);
  }
  return blockedPreferencePreflightRow(row);
}

function scheduledPreferencePreflightRow(
  row: AppGameNotificationSchedulerBridgeRow
): AppGameNotificationPreferencePreflightRow {
  const record = row.schedulerRecord;
  if (record === null) {
    throw new Error(
      `Missing scheduler record for app/game notification preference preflight: ${row.schedulerBridgeRecordId}`
    );
  }
  const preferenceRefs = [
    `parent-preference-required-${record.providerChannel}-${record.schedulerEntryId}`,
    `notification-frequency-control-required-${record.schedulerEntryId}`,
  ];
  const quietHoursRefs = [`quiet-hours-policy-required-${record.schedulerEntryId}`];

  return AppGameNotificationPreferencePreflightRowSchema.parse({
    preferenceRowId: `app-game-notification-preference-preflight-${row.schedulerBridgeRecordId}`,
    sourceSchedulerBridgeRecordId: row.schedulerBridgeRecordId,
    status: AppGameNotificationPreferencePreflightStatus.ParentPreferenceRequired,
    sourceSchedulerEntryRef: record.schedulerEntryId,
    sourceOutboxRecordRef: row.sourceOutboxRecordRef,
    providerChannelRef: record.providerChannel,
    reasonCodeRef: record.reasonCode,
    parentPreferenceState: 'manual-setup-required',
    quietHoursDecision: 'manual-required',
    parentPreferenceRequirementRefs: preferenceRefs,
    quietHoursRequirementRefs: quietHoursRefs,
    manualProofRequirements: [...preferenceRefs, ...quietHoursRefs],
  });
}

function blockedPreferencePreflightRow(
  row: AppGameNotificationSchedulerBridgeRow
): AppGameNotificationPreferencePreflightRow {
  return AppGameNotificationPreferencePreflightRowSchema.parse({
    preferenceRowId: `app-game-notification-preference-preflight-${row.schedulerBridgeRecordId}`,
    sourceSchedulerBridgeRecordId: row.schedulerBridgeRecordId,
    status:
      row.status === AppGameNotificationSchedulerBridgeStatus.Unavailable
        ? AppGameNotificationPreferencePreflightStatus.Unavailable
        : AppGameNotificationPreferencePreflightStatus.ManualRequired,
    sourceSchedulerEntryRef: null,
    sourceOutboxRecordRef: null,
    providerChannelRef: null,
    reasonCodeRef: null,
    parentPreferenceState: null,
    quietHoursDecision: null,
    parentPreferenceRequirementRefs: row.blockedReasonRefs,
    quietHoursRequirementRefs: row.blockedReasonRefs,
    manualProofRequirements: row.blockedReasonRefs,
  });
}

function preferencePreflightRowIsHonest(
  row: Infer<typeof AppGameNotificationPreferencePreflightRowBaseSchema>
): boolean {
  if (row.status === AppGameNotificationPreferencePreflightStatus.ParentPreferenceRequired) {
    return (
      scheduledPreferenceRefsArePresent(row) &&
      row.parentPreferenceState === 'manual-setup-required' &&
      row.quietHoursDecision === 'manual-required' &&
      row.parentPreferenceRequirementRefs.length >= 2 &&
      row.quietHoursRequirementRefs.length >= 1 &&
      row.manualProofRequirements.length >= 3
    );
  }
  return (
    blockedPreferenceRefsAreEmpty(row) &&
    row.parentPreferenceRequirementRefs.length > 0 &&
    row.quietHoursRequirementRefs.length > 0 &&
    row.manualProofRequirements.length > 0
  );
}

function scheduledPreferenceRefsArePresent(
  row: Infer<typeof AppGameNotificationPreferencePreflightRowBaseSchema>
): boolean {
  return [row.sourceSchedulerEntryRef, row.sourceOutboxRecordRef, row.providerChannelRef, row.reasonCodeRef].every(
    (value) => value !== null
  );
}

function blockedPreferenceRefsAreEmpty(
  row: Infer<typeof AppGameNotificationPreferencePreflightRowBaseSchema>
): boolean {
  return (
    [row.sourceSchedulerEntryRef, row.sourceOutboxRecordRef, row.providerChannelRef, row.reasonCodeRef].every(
      (value) => value === null
    ) &&
    row.parentPreferenceState === null &&
    row.quietHoursDecision === null
  );
}

function preferencePreflightReadModelIsHonest(
  readModel: Infer<typeof AppGameNotificationPreferencePreflightReadModelBaseSchema>
): boolean {
  return (
    readModel.parentPreferenceRequiredCount ===
      countRows(readModel.rows, AppGameNotificationPreferencePreflightStatus.ParentPreferenceRequired) &&
    readModel.manualRequiredCount ===
      countRows(readModel.rows, AppGameNotificationPreferencePreflightStatus.ManualRequired) &&
    readModel.unavailableCount ===
      countRows(readModel.rows, AppGameNotificationPreferencePreflightStatus.Unavailable) &&
    RequiredAppGameNotificationPreferencePreflightNonClaims.every((claim) =>
      readModel.preflightNonClaims.includes(claim)
    )
  );
}

const countRows = (
  rows: ReadonlyArray<{ readonly status: AppGameNotificationPreferencePreflightStatus }>,
  status: AppGameNotificationPreferencePreflightStatus
): number => rows.filter((row) => row.status === status).length;

