import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AppGameNotificationPayloadField, AppGameNotificationPayloadFieldSchema } from './app-game-notification-intent';
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
} from './reference-primitives';
import { FamilyReferenceSchema } from './references';
import { V3NotificationProviderChannelSchema } from './v3-notification-rule-provider-retry-contract';

const PayloadPreflightText = Schema.String.pipe(Schema.minLength(1));

export const AppGameNotificationPayloadPreflightStatus = {
  MinimalPayloadRequired: 'minimal-payload-required',
  ManualRequired: 'source-manual-required',
  Unavailable: 'source-unavailable',
} as const;

export const AppGameNotificationPayloadSensitiveDetailExclusion = {
  RawChildEvidence: 'raw-child-evidence-excluded',
  RawUrlOrTitle: 'raw-url-title-excluded',
  RawMessageText: 'raw-message-text-excluded',
  ScreenshotOrReport: 'screenshot-report-excluded',
  SensitiveProviderMetadata: 'sensitive-provider-metadata-excluded',
} as const;

export const RequiredAppGameNotificationPayloadPreflightNonClaims = [
  'no-provider-payload-template-runtime',
  'no-sensitive-provider-metadata-storage',
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

const RequiredMinimalPayloadFields = Object.values(AppGameNotificationPayloadField);
const RequiredSensitiveDetailExclusions = Object.values(AppGameNotificationPayloadSensitiveDetailExclusion);

export const AppGameNotificationPayloadPreflightStatusSchema = withParser(
  Schema.Literal(...Object.values(AppGameNotificationPayloadPreflightStatus))
);
export const AppGameNotificationPayloadPreflightNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameNotificationPayloadPreflightNonClaims)
);
export const AppGameNotificationPayloadPreflightIdSchema = PayloadPreflightText.pipe(
  Schema.brand('AppGameNotificationPayloadPreflightId')
);
export const AppGameNotificationPayloadPreflightReferenceSchema = PayloadPreflightText.pipe(
  Schema.brand('AppGameNotificationPayloadPreflightReference')
);
export const AppGameNotificationPayloadSensitiveDetailExclusionSchema = withParser(
  Schema.Literal(...RequiredSensitiveDetailExclusions)
);

const AppGameNotificationPayloadPreflightRowBaseSchema = Schema.Struct({
  payloadPreflightRowId: AppGameNotificationPayloadPreflightReferenceSchema,
  sourceSchedulerBridgeRecordId: AppGameNotificationPayloadPreflightReferenceSchema,
  status: AppGameNotificationPayloadPreflightStatusSchema,
  sourceSchedulerEntryRef: Schema.Union(AppGameNotificationPayloadPreflightReferenceSchema, Schema.Null),
  sourceOutboxRecordRef: Schema.Union(AppGameNotificationPayloadPreflightReferenceSchema, Schema.Null),
  providerChannel: Schema.Union(V3NotificationProviderChannelSchema, Schema.Null),
  reasonCodeRef: Schema.Union(AppGameNotificationPayloadPreflightReferenceSchema, Schema.Null),
  minimalPayloadFields: Schema.Array(AppGameNotificationPayloadFieldSchema),
  sensitiveDetailExclusionRefs: Schema.Array(AppGameNotificationPayloadPreflightReferenceSchema),
  providerTemplateRequirementRefs: Schema.Array(AppGameNotificationPayloadPreflightReferenceSchema),
  payloadProofRequirements: Schema.Array(AppGameNotificationPayloadPreflightReferenceSchema),
});

export const AppGameNotificationPayloadPreflightRowSchema = withParser(
  AppGameNotificationPayloadPreflightRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        payloadPreflightRowIsHonest(row) ||
        'Expected app/game notification payload preflight rows to require minimal payload refs and keep blocked source rows without provider payload claims'
    )
  )
);

const AppGameNotificationPayloadPreflightReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  payloadPreflightId: AppGameNotificationPayloadPreflightIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  sourceSchedulerBridgeId: AppGameNotificationPayloadPreflightReferenceSchema,
  sourceContractRefs: Schema.Array(AppGameNotificationPayloadPreflightReferenceSchema),
  rows: Schema.Array(AppGameNotificationPayloadPreflightRowSchema),
  minimalPayloadRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  manualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  unavailableCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  preflightNonClaims: Schema.Array(AppGameNotificationPayloadPreflightNonClaimSchema),
  providerPayloadTemplateRuntimeClaimed: Schema.Literal(false),
  sensitiveProviderMetadataStored: Schema.Literal(false),
  rawChildEvidenceIncluded: Schema.Literal(false),
  rawUrlOrTitleIncluded: Schema.Literal(false),
  rawMessageTextIncluded: Schema.Literal(false),
  screenshotOrReportIncluded: Schema.Literal(false),
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

export const AppGameNotificationPayloadPreflightReadModelSchema = withParser(
  AppGameNotificationPayloadPreflightReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        payloadPreflightReadModelIsHonest(readModel) ||
        'Expected app/game notification payload preflight counts and non-claims to match minimal manual and unavailable rows'
    )
  )
);

export type AppGameNotificationPayloadPreflightStatus = Infer<typeof AppGameNotificationPayloadPreflightStatusSchema>;
export type AppGameNotificationPayloadPreflightRow = Infer<typeof AppGameNotificationPayloadPreflightRowSchema>;
export type AppGameNotificationPayloadPreflightReadModel = Infer<
  typeof AppGameNotificationPayloadPreflightReadModelSchema
>;

export type AppGameNotificationPayloadPreflightOptions = {
  readonly generatedAt: string;
  readonly payloadPreflightId: string;
};

export function buildAppGameNotificationPayloadPreflightReadModel(
  options: AppGameNotificationPayloadPreflightOptions,
  sourceReadModel: AppGameNotificationSchedulerBridgeReadModel
): AppGameNotificationPayloadPreflightReadModel {
  const parsedSource = AppGameNotificationSchedulerBridgeReadModelSchema.parse(sourceReadModel);
  const rows = parsedSource.rows.map((row) => schedulerRowToPayloadPreflightRow(row));

  return AppGameNotificationPayloadPreflightReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    payloadPreflightId: options.payloadPreflightId,
    generatedAt: options.generatedAt,
    family: parsedSource.family,
    sourceSchedulerBridgeId: parsedSource.schedulerBridgeId,
    sourceContractRefs: [
      parsedSource.schedulerBridgeId,
      parsedSource.sourceOutboxBridgeId,
      parsedSource.schedulerArtifactRootRef,
    ],
    rows,
    minimalPayloadRequiredCount: countRows(rows, AppGameNotificationPayloadPreflightStatus.MinimalPayloadRequired),
    manualRequiredCount: countRows(rows, AppGameNotificationPayloadPreflightStatus.ManualRequired),
    unavailableCount: countRows(rows, AppGameNotificationPayloadPreflightStatus.Unavailable),
    preflightNonClaims: RequiredAppGameNotificationPayloadPreflightNonClaims,
    providerPayloadTemplateRuntimeClaimed: false,
    sensitiveProviderMetadataStored: false,
    rawChildEvidenceIncluded: false,
    rawUrlOrTitleIncluded: false,
    rawMessageTextIncluded: false,
    screenshotOrReportIncluded: false,
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

function schedulerRowToPayloadPreflightRow(
  row: AppGameNotificationSchedulerBridgeRow
): AppGameNotificationPayloadPreflightRow {
  if (row.status === AppGameNotificationSchedulerBridgeStatus.ScheduledLocal) {
    return scheduledRowToPayloadPreflightRow(row);
  }

  return AppGameNotificationPayloadPreflightRowSchema.parse({
    payloadPreflightRowId: `app-game-notification-payload-preflight-${row.schedulerBridgeRecordId}`,
    sourceSchedulerBridgeRecordId: row.schedulerBridgeRecordId,
    status:
      row.status === AppGameNotificationSchedulerBridgeStatus.Unavailable
        ? AppGameNotificationPayloadPreflightStatus.Unavailable
        : AppGameNotificationPayloadPreflightStatus.ManualRequired,
    sourceSchedulerEntryRef: null,
    sourceOutboxRecordRef: null,
    providerChannel: null,
    reasonCodeRef: null,
    minimalPayloadFields: [],
    sensitiveDetailExclusionRefs: [],
    providerTemplateRequirementRefs: [],
    payloadProofRequirements: row.blockedReasonRefs,
  });
}

function scheduledRowToPayloadPreflightRow(
  row: AppGameNotificationSchedulerBridgeRow
): AppGameNotificationPayloadPreflightRow {
  const record = row.schedulerRecord;
  if (record === null) {
    throw new Error(`Missing scheduler record for payload preflight row: ${row.schedulerBridgeRecordId}`);
  }
  const sensitiveRefs = RequiredSensitiveDetailExclusions;
  const templateRefs = [`provider-template-required-${record.providerChannel}-${record.schedulerEntryId}`];

  return AppGameNotificationPayloadPreflightRowSchema.parse({
    payloadPreflightRowId: `app-game-notification-payload-preflight-${row.schedulerBridgeRecordId}`,
    sourceSchedulerBridgeRecordId: row.schedulerBridgeRecordId,
    status: AppGameNotificationPayloadPreflightStatus.MinimalPayloadRequired,
    sourceSchedulerEntryRef: record.schedulerEntryId,
    sourceOutboxRecordRef: record.sourceEntryId,
    providerChannel: record.providerChannel,
    reasonCodeRef: record.reasonCode,
    minimalPayloadFields: RequiredMinimalPayloadFields,
    sensitiveDetailExclusionRefs: sensitiveRefs,
    providerTemplateRequirementRefs: templateRefs,
    payloadProofRequirements: [...RequiredMinimalPayloadFields, ...sensitiveRefs, ...templateRefs],
  });
}

function payloadPreflightRowIsHonest(row: Infer<typeof AppGameNotificationPayloadPreflightRowBaseSchema>): boolean {
  if (row.status === AppGameNotificationPayloadPreflightStatus.MinimalPayloadRequired) {
    return scheduledPayloadPreflightRowIsHonest(row);
  }
  return blockedPayloadPreflightRowIsHonest(row);
}

function scheduledPayloadPreflightRowIsHonest(
  row: Infer<typeof AppGameNotificationPayloadPreflightRowBaseSchema>
): boolean {
  return (
    hasPayloadSourceRefs(row) &&
    includesAll(RequiredMinimalPayloadFields, row.minimalPayloadFields) &&
    includesAll(RequiredSensitiveDetailExclusions, row.sensitiveDetailExclusionRefs) &&
    row.providerTemplateRequirementRefs.length > 0 &&
    row.payloadProofRequirements.length >=
      RequiredMinimalPayloadFields.length + RequiredSensitiveDetailExclusions.length + 1
  );
}

function blockedPayloadPreflightRowIsHonest(
  row: Infer<typeof AppGameNotificationPayloadPreflightRowBaseSchema>
): boolean {
  return (
    hasNoPayloadSourceRefs(row) &&
    row.minimalPayloadFields.length === 0 &&
    row.sensitiveDetailExclusionRefs.length === 0 &&
    row.providerTemplateRequirementRefs.length === 0 &&
    row.payloadProofRequirements.length > 0
  );
}

function hasPayloadSourceRefs(row: Infer<typeof AppGameNotificationPayloadPreflightRowBaseSchema>): boolean {
  return (
    row.sourceSchedulerEntryRef !== null &&
    row.sourceOutboxRecordRef !== null &&
    row.providerChannel !== null &&
    row.reasonCodeRef !== null
  );
}

function hasNoPayloadSourceRefs(row: Infer<typeof AppGameNotificationPayloadPreflightRowBaseSchema>): boolean {
  return (
    row.sourceSchedulerEntryRef === null &&
    row.sourceOutboxRecordRef === null &&
    row.providerChannel === null &&
    row.reasonCodeRef === null
  );
}

function payloadPreflightReadModelIsHonest(
  readModel: Infer<typeof AppGameNotificationPayloadPreflightReadModelBaseSchema>
): boolean {
  return (
    readModel.minimalPayloadRequiredCount ===
      countRows(readModel.rows, AppGameNotificationPayloadPreflightStatus.MinimalPayloadRequired) &&
    readModel.manualRequiredCount ===
      countRows(readModel.rows, AppGameNotificationPayloadPreflightStatus.ManualRequired) &&
    readModel.unavailableCount === countRows(readModel.rows, AppGameNotificationPayloadPreflightStatus.Unavailable) &&
    RequiredAppGameNotificationPayloadPreflightNonClaims.every((claim) => readModel.preflightNonClaims.includes(claim))
  );
}

function includesAll(required: ReadonlyArray<string>, values: ReadonlyArray<string>): boolean {
  const valueSet = new Set(values);
  return required.every((value) => valueSet.has(value));
}

const countRows = (
  rows: ReadonlyArray<{ readonly status: AppGameNotificationPayloadPreflightStatus }>,
  status: AppGameNotificationPayloadPreflightStatus
): number => rows.filter((row) => row.status === status).length;
