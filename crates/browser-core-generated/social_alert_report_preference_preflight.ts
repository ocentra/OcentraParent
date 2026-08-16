/* generated from crates/browser-core/src/social_alert_report_preference_preflight.rs */

import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import { FamilyReferenceSchema } from '@ocentra-parent/schema-domain/family-references';
import {
  SocialAlertReportSchedulerBridgeReadModelSchema,
  SocialAlertReportSchedulerBridgeStatus,
  type SocialAlertReportSchedulerBridgeReadModel,
  type SocialAlertReportSchedulerBridgeRow,
} from './social-alert-report-scheduler-bridge';
import {
  V3NotificationParentPreferenceStateSchema,
  V3NotificationQuietHoursDecisionSchema,
} from '@ocentra-parent/schema-domain/notification-v3-provider-retry';

export const SocialAlertReportPreferencePreflightStatus = {
  ParentPreferenceRequired: 'parent-preference-required',
  ManualRequired: 'source-manual-required',
  Unavailable: 'source-unavailable',
} as const;

export const RequiredSocialAlertReportPreferencePreflightNonClaims = [
  'no-parent-notification-preference-ui',
  'no-parent-notification-history-ui',
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
  'no-report-delivery-execution',
  'no-final-policy-execution',
  'no-enforcement',
] as const;

export const SocialAlertReportPreferencePreflightStatusSchema = withParser(
  Schema.Literal(...Object.values(SocialAlertReportPreferencePreflightStatus))
);
export const SocialAlertReportPreferencePreflightNonClaimSchema = withParser(
  Schema.Literal(...RequiredSocialAlertReportPreferencePreflightNonClaims)
);
export const SocialAlertReportPreferencePreflightIdSchema = brandedNonEmptyStringSchema(
  'SocialAlertReportPreferencePreflightId'
);
export const SocialAlertReportPreferencePreflightReferenceSchema = brandedNonEmptyStringSchema(
  'SocialAlertReportPreferencePreflightReference'
);

const SocialAlertReportPreferencePreflightRowBaseSchema = Schema.Struct({
  preferenceRowId: SocialAlertReportPreferencePreflightReferenceSchema,
  sourceSchedulerBridgeRecordId: SocialAlertReportPreferencePreflightReferenceSchema,
  status: SocialAlertReportPreferencePreflightStatusSchema,
  sourceSchedulerEntryRef: Schema.Union(SocialAlertReportPreferencePreflightReferenceSchema, Schema.Null),
  sourceOutboxRecordRef: Schema.Union(SocialAlertReportPreferencePreflightReferenceSchema, Schema.Null),
  providerChannelRef: Schema.Union(SocialAlertReportPreferencePreflightReferenceSchema, Schema.Null),
  reasonCodeRef: Schema.Union(SocialAlertReportPreferencePreflightReferenceSchema, Schema.Null),
  schedulerDecisionRef: Schema.Union(SocialAlertReportPreferencePreflightReferenceSchema, Schema.Null),
  parentPreferenceState: Schema.Union(V3NotificationParentPreferenceStateSchema, Schema.Null),
  quietHoursDecision: Schema.Union(V3NotificationQuietHoursDecisionSchema, Schema.Null),
  parentPreferenceRequirementRefs: Schema.Array(SocialAlertReportPreferencePreflightReferenceSchema),
  quietHoursRequirementRefs: Schema.Array(SocialAlertReportPreferencePreflightReferenceSchema),
  manualProofRequirements: Schema.Array(SocialAlertReportPreferencePreflightReferenceSchema),
});

export const SocialAlertReportPreferencePreflightRowSchema = withParser(
  SocialAlertReportPreferencePreflightRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        preferencePreflightRowIsHonest(row) ||
        'Expected social alert/report preference preflight rows to require parent notification preferences and quiet-hours proof before delivery'
    )
  )
);

const SocialAlertReportPreferencePreflightReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  preferencePreflightId: SocialAlertReportPreferencePreflightIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  sourceSchedulerBridgeId: SocialAlertReportPreferencePreflightReferenceSchema,
  sourceContractRefs: Schema.Array(SocialAlertReportPreferencePreflightReferenceSchema),
  rows: Schema.Array(SocialAlertReportPreferencePreflightRowSchema),
  parentPreferenceRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  manualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  unavailableCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  preflightNonClaims: Schema.Array(SocialAlertReportPreferencePreflightNonClaimSchema),
  parentNotificationPreferenceUiClaimed: Schema.Literal(false),
  parentNotificationHistoryUiClaimed: Schema.Literal(false),
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
  reportDeliveryExecutionClaimed: Schema.Literal(false),
  finalPolicyExecutionClaimed: Schema.Literal(false),
  enforcementClaimed: Schema.Literal(false),
});

export const SocialAlertReportPreferencePreflightReadModelSchema = withParser(
  SocialAlertReportPreferencePreflightReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        preferencePreflightReadModelIsHonest(readModel) ||
        'Expected social alert/report preference preflight counts and non-claims to match scheduled manual and unavailable rows'
    )
  )
);

export type SocialAlertReportPreferencePreflightStatus = Infer<typeof SocialAlertReportPreferencePreflightStatusSchema>;
export type SocialAlertReportPreferencePreflightRow = Infer<typeof SocialAlertReportPreferencePreflightRowSchema>;
export type SocialAlertReportPreferencePreflightReadModel = Infer<
  typeof SocialAlertReportPreferencePreflightReadModelSchema
>;
export type SocialAlertReportPreferencePreflightOptions = {
  readonly generatedAt: string;
  readonly preferencePreflightId: string;
  readonly sourceContractRefs: readonly string[];
};

type PreferenceRowInput = Infer<typeof SocialAlertReportPreferencePreflightRowBaseSchema>;
type PreferenceReadModelInput = Infer<typeof SocialAlertReportPreferencePreflightReadModelBaseSchema>;

export function buildSocialAlertReportPreferencePreflightReadModel(
  options: SocialAlertReportPreferencePreflightOptions,
  sourceReadModel: SocialAlertReportSchedulerBridgeReadModel
): SocialAlertReportPreferencePreflightReadModel {
  const parsedSource = SocialAlertReportSchedulerBridgeReadModelSchema.parse(sourceReadModel);
  const rows = parsedSource.rows.map(preferencePreflightRowForSchedulerRow);

  return SocialAlertReportPreferencePreflightReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    preferencePreflightId: options.preferencePreflightId,
    generatedAt: options.generatedAt,
    family: parsedSource.family,
    sourceSchedulerBridgeId: parsedSource.schedulerBridgeId,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    parentPreferenceRequiredCount: countRows(rows, SocialAlertReportPreferencePreflightStatus.ParentPreferenceRequired),
    manualRequiredCount: countRows(rows, SocialAlertReportPreferencePreflightStatus.ManualRequired),
    unavailableCount: countRows(rows, SocialAlertReportPreferencePreflightStatus.Unavailable),
    preflightNonClaims: RequiredSocialAlertReportPreferencePreflightNonClaims,
    parentNotificationPreferenceUiClaimed: false,
    parentNotificationHistoryUiClaimed: false,
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
    reportDeliveryExecutionClaimed: false,
    finalPolicyExecutionClaimed: false,
    enforcementClaimed: false,
  });
}

function preferencePreflightRowForSchedulerRow(
  row: SocialAlertReportSchedulerBridgeRow
): SocialAlertReportPreferencePreflightRow {
  if (row.status === SocialAlertReportSchedulerBridgeStatus.ScheduledLocal && row.schedulerRecord !== null) {
    return scheduledPreferencePreflightRow(row);
  }
  return blockedPreferencePreflightRow(row);
}

function scheduledPreferencePreflightRow(
  row: SocialAlertReportSchedulerBridgeRow
): SocialAlertReportPreferencePreflightRow {
  const record = row.schedulerRecord;
  if (record === null) {
    throw new Error(`Missing social alert/report scheduler record for preference row: ${row.schedulerBridgeRecordId}`);
  }
  const preferenceRefs = [
    `social-parent-notification-preference-required-${record.schedulerEntryId}`,
    `social-notification-frequency-control-required-${record.schedulerEntryId}`,
  ];
  const quietHoursRefs = [`social-quiet-hours-policy-required-${record.schedulerEntryId}`];

  return SocialAlertReportPreferencePreflightRowSchema.parse({
    preferenceRowId: `social-alert-report-preference-preflight-${row.schedulerBridgeRecordId}`,
    sourceSchedulerBridgeRecordId: row.schedulerBridgeRecordId,
    status: SocialAlertReportPreferencePreflightStatus.ParentPreferenceRequired,
    sourceSchedulerEntryRef: record.schedulerEntryId,
    sourceOutboxRecordRef: row.sourceOutboxRecordRef,
    providerChannelRef: record.providerChannel,
    reasonCodeRef: record.reasonCode,
    schedulerDecisionRef: record.schedulerDecisionRef,
    parentPreferenceState: 'manual-setup-required',
    quietHoursDecision: 'manual-required',
    parentPreferenceRequirementRefs: preferenceRefs,
    quietHoursRequirementRefs: quietHoursRefs,
    manualProofRequirements: [...preferenceRefs, ...quietHoursRefs],
  });
}

function blockedPreferencePreflightRow(
  row: SocialAlertReportSchedulerBridgeRow
): SocialAlertReportPreferencePreflightRow {
  return SocialAlertReportPreferencePreflightRowSchema.parse({
    preferenceRowId: `social-alert-report-preference-preflight-${row.schedulerBridgeRecordId}`,
    sourceSchedulerBridgeRecordId: row.schedulerBridgeRecordId,
    status:
      row.status === SocialAlertReportSchedulerBridgeStatus.Unavailable
        ? SocialAlertReportPreferencePreflightStatus.Unavailable
        : SocialAlertReportPreferencePreflightStatus.ManualRequired,
    sourceSchedulerEntryRef: null,
    sourceOutboxRecordRef: null,
    providerChannelRef: null,
    reasonCodeRef: null,
    schedulerDecisionRef: null,
    parentPreferenceState: null,
    quietHoursDecision: null,
    parentPreferenceRequirementRefs: row.blockedReasonRefs,
    quietHoursRequirementRefs: row.blockedReasonRefs,
    manualProofRequirements: row.blockedReasonRefs,
  });
}

function preferencePreflightRowIsHonest(row: PreferenceRowInput): boolean {
  if (row.status === SocialAlertReportPreferencePreflightStatus.ParentPreferenceRequired) {
    return (
      scheduledRefsArePresent(row) &&
      row.parentPreferenceState === 'manual-setup-required' &&
      row.quietHoursDecision === 'manual-required' &&
      row.parentPreferenceRequirementRefs.length >= 2 &&
      row.quietHoursRequirementRefs.length >= 1 &&
      row.manualProofRequirements.length >= 3
    );
  }
  return (
    blockedRefsAreEmpty(row) &&
    row.parentPreferenceRequirementRefs.length > 0 &&
    row.quietHoursRequirementRefs.length > 0 &&
    row.manualProofRequirements.length > 0
  );
}

function scheduledRefsArePresent(row: PreferenceRowInput): boolean {
  return [
    row.sourceSchedulerEntryRef,
    row.sourceOutboxRecordRef,
    row.providerChannelRef,
    row.reasonCodeRef,
    row.schedulerDecisionRef,
  ].every((value) => value !== null);
}

function blockedRefsAreEmpty(row: PreferenceRowInput): boolean {
  return (
    [
      row.sourceSchedulerEntryRef,
      row.sourceOutboxRecordRef,
      row.providerChannelRef,
      row.reasonCodeRef,
      row.schedulerDecisionRef,
    ].every((value) => value === null) &&
    row.parentPreferenceState === null &&
    row.quietHoursDecision === null
  );
}

function preferencePreflightReadModelIsHonest(readModel: PreferenceReadModelInput): boolean {
  return (
    readModel.parentPreferenceRequiredCount ===
      countRows(readModel.rows, SocialAlertReportPreferencePreflightStatus.ParentPreferenceRequired) &&
    readModel.manualRequiredCount ===
      countRows(readModel.rows, SocialAlertReportPreferencePreflightStatus.ManualRequired) &&
    readModel.unavailableCount === countRows(readModel.rows, SocialAlertReportPreferencePreflightStatus.Unavailable) &&
    RequiredSocialAlertReportPreferencePreflightNonClaims.every((claim) => readModel.preflightNonClaims.includes(claim))
  );
}

function countRows(
  rows: ReadonlyArray<{ readonly status: SocialAlertReportPreferencePreflightStatus }>,
  status: SocialAlertReportPreferencePreflightStatus
): number {
  return rows.filter((row) => row.status === status).length;
}
