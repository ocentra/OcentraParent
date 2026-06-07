import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
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
} from './reference-primitives';
import { FamilyReferenceSchema } from './references';
import {
  V3NotificationParentPreferenceStateSchema,
  V3NotificationQuietHoursDecisionSchema,
} from './v3-notification-rule-provider-retry-contract';

const ChildUxPreferencePreflightText = Schema.String.pipe(Schema.minLength(1));

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
export const AppGameChildUxLocalOutboxPreferencePreflightIdSchema = ChildUxPreferencePreflightText.pipe(
  Schema.brand('AppGameChildUxLocalOutboxPreferencePreflightId')
);
export const AppGameChildUxLocalOutboxPreferencePreflightReferenceSchema = ChildUxPreferencePreflightText.pipe(
  Schema.brand('AppGameChildUxLocalOutboxPreferencePreflightReference')
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

export type AppGameChildUxLocalOutboxPreferencePreflightOptions = {
  readonly generatedAt: string;
  readonly preferencePreflightId: string;
  readonly sourceContractRefs: readonly string[];
};

export function buildAppGameChildUxLocalOutboxPreferencePreflightReadModel(
  options: AppGameChildUxLocalOutboxPreferencePreflightOptions,
  sourceReadModel: AppGameChildUxLocalOutboxSchedulerBridgeReadModel
): AppGameChildUxLocalOutboxPreferencePreflightReadModel {
  const parsedSource = AppGameChildUxLocalOutboxSchedulerBridgeReadModelSchema.parse(sourceReadModel);
  const rows = parsedSource.rows.map(preferencePreflightRowForChildUxSchedulerRow);

  return AppGameChildUxLocalOutboxPreferencePreflightReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    preferencePreflightId: options.preferencePreflightId,
    generatedAt: options.generatedAt,
    family: parsedSource.family,
    sourceSchedulerBridgeId: parsedSource.schedulerBridgeId,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    parentPreferenceRequiredCount: countRows(
      rows,
      AppGameChildUxLocalOutboxPreferencePreflightStatus.ParentPreferenceRequired
    ),
    manualRequiredCount: countRows(rows, AppGameChildUxLocalOutboxPreferencePreflightStatus.ManualRequired),
    unavailableCount: countRows(rows, AppGameChildUxLocalOutboxPreferencePreflightStatus.Unavailable),
    preflightNonClaims: RequiredAppGameChildUxLocalOutboxPreferencePreflightNonClaims,
    parentPreferenceUiClaimed: false,
    parentFrequencyControlUiClaimed: false,
    parentNotificationUiClaimed: false,
    quietHoursTimerRuntimeClaimed: false,
    childDeliveryRuntimeClaimed: false,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    retryExecutionRuntimeClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    rawPrivateSourceRowsIncluded: false,
  });
}

function preferencePreflightRowForChildUxSchedulerRow(
  row: AppGameChildUxLocalOutboxSchedulerBridgeRow
): AppGameChildUxLocalOutboxPreferencePreflightRow {
  if (row.status === AppGameChildUxLocalOutboxSchedulerBridgeStatus.ScheduledLocal && row.schedulerRecord !== null) {
    return scheduledPreferencePreflightRow(row);
  }
  return blockedPreferencePreflightRow(row);
}

function scheduledPreferencePreflightRow(
  row: AppGameChildUxLocalOutboxSchedulerBridgeRow
): AppGameChildUxLocalOutboxPreferencePreflightRow {
  const record = row.schedulerRecord;
  if (record === null) {
    throw new Error(`Missing scheduler record for child UX preference preflight: ${row.schedulerBridgeRecordId}`);
  }
  const parentPreferenceRefs = [
    `child-ux-parent-preference-required-${record.providerChannel}-${record.schedulerEntryId}`,
    `child-ux-notification-frequency-control-required-${record.schedulerEntryId}`,
  ];
  const quietHoursRefs = [`child-ux-quiet-hours-policy-required-${record.schedulerEntryId}`];

  return AppGameChildUxLocalOutboxPreferencePreflightRowSchema.parse({
    preferenceRowId: `app-game-child-ux-local-outbox-preference-preflight-${row.schedulerBridgeRecordId}`,
    sourceSchedulerBridgeRecordId: row.schedulerBridgeRecordId,
    status: AppGameChildUxLocalOutboxPreferencePreflightStatus.ParentPreferenceRequired,
    sourceSchedulerEntryRef: record.schedulerEntryId,
    sourceOutboxRecordRef: row.sourceOutboxRecordRef,
    schedulerDecisionRef: record.schedulerDecisionRef,
    providerChannelRef: record.providerChannel,
    reasonCodeRef: record.reasonCode,
    parentPreferenceState: 'manual-setup-required',
    quietHoursDecision: 'manual-required',
    parentPreferenceRequirementRefs: parentPreferenceRefs,
    quietHoursRequirementRefs: quietHoursRefs,
    manualProofRequirements: [...parentPreferenceRefs, ...quietHoursRefs],
  });
}

function blockedPreferencePreflightRow(
  row: AppGameChildUxLocalOutboxSchedulerBridgeRow
): AppGameChildUxLocalOutboxPreferencePreflightRow {
  return AppGameChildUxLocalOutboxPreferencePreflightRowSchema.parse({
    preferenceRowId: `app-game-child-ux-local-outbox-preference-preflight-${row.schedulerBridgeRecordId}`,
    sourceSchedulerBridgeRecordId: row.schedulerBridgeRecordId,
    status:
      row.status === AppGameChildUxLocalOutboxSchedulerBridgeStatus.Unavailable
        ? AppGameChildUxLocalOutboxPreferencePreflightStatus.Unavailable
        : AppGameChildUxLocalOutboxPreferencePreflightStatus.ManualRequired,
    sourceSchedulerEntryRef: null,
    sourceOutboxRecordRef: null,
    schedulerDecisionRef: null,
    providerChannelRef: null,
    reasonCodeRef: null,
    parentPreferenceState: null,
    quietHoursDecision: null,
    parentPreferenceRequirementRefs: row.blockedReasonRefs,
    quietHoursRequirementRefs: row.blockedReasonRefs,
    manualProofRequirements: row.blockedReasonRefs,
  });
}

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
    readModel.parentPreferenceRequiredCount ===
      countRows(readModel.rows, AppGameChildUxLocalOutboxPreferencePreflightStatus.ParentPreferenceRequired) &&
    readModel.manualRequiredCount ===
      countRows(readModel.rows, AppGameChildUxLocalOutboxPreferencePreflightStatus.ManualRequired) &&
    readModel.unavailableCount ===
      countRows(readModel.rows, AppGameChildUxLocalOutboxPreferencePreflightStatus.Unavailable) &&
    RequiredAppGameChildUxLocalOutboxPreferencePreflightNonClaims.every((claim) =>
      readModel.preflightNonClaims.includes(claim)
    ) &&
    !readModel.parentPreferenceUiClaimed &&
    !readModel.parentFrequencyControlUiClaimed &&
    !readModel.parentNotificationUiClaimed &&
    !readModel.quietHoursTimerRuntimeClaimed &&
    !readModel.childDeliveryRuntimeClaimed &&
    !readModel.providerDeliveryRuntimeClaimed &&
    !readModel.providerReceiptIngestionClaimed &&
    !readModel.providerCredentialsClaimed &&
    !readModel.cloudRoutingClaimed &&
    !readModel.retryExecutionRuntimeClaimed &&
    !readModel.productionDurableOutboxStorageClaimed &&
    !readModel.adapterDispatchClaimed &&
    !readModel.platformEnforcementClaimed &&
    !readModel.rawPrivateSourceRowsIncluded
  );
}

function countRows(
  rows: ReadonlyArray<{ readonly status: AppGameChildUxLocalOutboxPreferencePreflightStatus }>,
  status: AppGameChildUxLocalOutboxPreferencePreflightStatus
): number {
  return rows.filter((row) => row.status === status).length;
}
