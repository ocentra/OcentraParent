/* generated from crates/browser-core/src/social_schema_generated_alert_report.rs */

import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  RequiredSocialAlertReportProviderStatusHandoffNonClaims,
  SocialAlertReportProviderStatusHandoffReadModelSchema,
  type SocialAlertReportProviderStatusHandoffReadModel,
  type SocialAlertReportProviderStatusHandoffRow,
} from './generated-social-alert-report-provider-status-handoff-proof';
import { V08NotificationProviderStatusSchema } from '@ocentra-parent/schema-domain/v0-8-notification-provider-status-boundary';

export const RequiredSocialAlertReportProviderReceiptBoundaryNonClaims = [
  'no-provider-delivery-execution',
  'no-provider-receipt-ingestion-runtime',
  'no-provider-webhook-runtime',
  'no-provider-credentials',
  'no-cloud-routing',
  'no-parent-notification-ui-delivery',
  'no-report-delivery-execution',
  'no-final-policy-execution',
  'no-connector-native-runtime',
  'no-enforcement',
] as const;

export const SocialAlertReportProviderReceiptBoundaryNonClaimSchema = withParser(
  Schema.Literal(...RequiredSocialAlertReportProviderReceiptBoundaryNonClaims)
);
export const SocialAlertReportProviderReceiptBoundaryIdSchema = withParser(
  brandedNonEmptyStringSchema('SocialAlertReportProviderReceiptBoundaryId')
);
export const SocialAlertReportProviderReceiptBoundaryReferenceSchema = withParser(
  brandedNonEmptyStringSchema('SocialAlertReportProviderReceiptBoundaryReference')
);
export const SocialAlertReportProviderReceiptBoundaryStateSchema = withParser(
  Schema.Literal('provider-dispatch-required', 'manual-receipt-required', 'provider-unavailable')
);

const SocialAlertReportProviderReceiptBoundaryRowBaseSchema = Schema.Struct({
  receiptRowId: SocialAlertReportProviderReceiptBoundaryReferenceSchema,
  sourceProviderStatusHandoffRowRef: SocialAlertReportProviderReceiptBoundaryReferenceSchema,
  sourcePreflightRowRef: SocialAlertReportProviderReceiptBoundaryReferenceSchema,
  sourceIntentRef: SocialAlertReportProviderReceiptBoundaryReferenceSchema,
  sourceLocalOutboxRecordRef: Schema.Union(SocialAlertReportProviderReceiptBoundaryReferenceSchema, Schema.Null),
  sourceProviderStatusEntryRef: SocialAlertReportProviderReceiptBoundaryReferenceSchema,
  sourceProviderStatus: V08NotificationProviderStatusSchema,
  receiptBoundaryState: SocialAlertReportProviderReceiptBoundaryStateSchema,
  providerAttemptRef: SocialAlertReportProviderReceiptBoundaryReferenceSchema,
  providerReceiptRefs: Schema.Array(SocialAlertReportProviderReceiptBoundaryReferenceSchema),
  auditRefs: Schema.Array(SocialAlertReportProviderReceiptBoundaryReferenceSchema),
  readinessRefs: Schema.Array(SocialAlertReportProviderReceiptBoundaryReferenceSchema),
  manualProofRequirements: Schema.Array(SocialAlertReportProviderReceiptBoundaryReferenceSchema),
  receiptProofRequirements: Schema.Array(SocialAlertReportProviderReceiptBoundaryReferenceSchema),
  providerDeliveryClaimed: Schema.Literal(false),
  providerReceiptIngestionClaimed: Schema.Literal(false),
  providerWebhookRuntimeClaimed: Schema.Literal(false),
  providerCredentialClaimed: Schema.Literal(false),
  cloudRoutingClaimed: Schema.Literal(false),
  parentNotificationUiDeliveryClaimed: Schema.Literal(false),
  reportDeliveryExecutionClaimed: Schema.Literal(false),
  finalPolicyExecutionClaimed: Schema.Literal(false),
  connectorNativeRuntimeClaimed: Schema.Literal(false),
  enforcementClaimed: Schema.Literal(false),
});

export const SocialAlertReportProviderReceiptBoundaryRowSchema = withParser(
  SocialAlertReportProviderReceiptBoundaryRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        socialAlertReportProviderReceiptBoundaryRowIsHonest(row) ||
        'Expected social alert/report provider receipt boundary rows to preserve provider status handoff refs and keep delivery/receipt runtime unclaimed'
    )
  )
);

const SocialAlertReportProviderReceiptBoundaryReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  receiptBoundaryId: SocialAlertReportProviderReceiptBoundaryIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceProviderStatusHandoffId: SocialAlertReportProviderReceiptBoundaryReferenceSchema,
  sourceContractRefs: Schema.Array(SocialAlertReportProviderReceiptBoundaryReferenceSchema),
  sourceProviderStatusHandoffNonClaims: Schema.Array(SocialAlertReportProviderReceiptBoundaryReferenceSchema),
  rows: Schema.Array(SocialAlertReportProviderReceiptBoundaryRowSchema),
  providerDispatchRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  manualReceiptRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  providerUnavailableCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  receiptBoundaryNonClaims: Schema.Array(SocialAlertReportProviderReceiptBoundaryNonClaimSchema),
  providerDeliveryRuntimeClaimed: Schema.Literal(false),
  providerReceiptIngestionRuntimeClaimed: Schema.Literal(false),
  providerWebhookRuntimeClaimed: Schema.Literal(false),
  providerCredentialsClaimed: Schema.Literal(false),
  cloudRoutingClaimed: Schema.Literal(false),
  parentNotificationUiDeliveryClaimed: Schema.Literal(false),
  reportDeliveryExecutionClaimed: Schema.Literal(false),
  finalPolicyExecutionClaimed: Schema.Literal(false),
  connectorNativeRuntimeClaimed: Schema.Literal(false),
  enforcementClaimed: Schema.Literal(false),
});

export const SocialAlertReportProviderReceiptBoundaryReadModelSchema = withParser(
  SocialAlertReportProviderReceiptBoundaryReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        socialAlertReportProviderReceiptBoundaryReadModelIsHonest(readModel) ||
        'Expected social alert/report provider receipt boundary counts and non-claims to match provider receipt rows'
    )
  )
);

export type SocialAlertReportProviderReceiptBoundaryState = Infer<
  typeof SocialAlertReportProviderReceiptBoundaryStateSchema
>;
export type SocialAlertReportProviderReceiptBoundaryRow = Infer<
  typeof SocialAlertReportProviderReceiptBoundaryRowSchema
>;
export type SocialAlertReportProviderReceiptBoundaryReadModel = Infer<
  typeof SocialAlertReportProviderReceiptBoundaryReadModelSchema
>;

export type SocialAlertReportProviderReceiptBoundaryOptions = {
  readonly generatedAt: string;
  readonly receiptBoundaryId: string;
  readonly sourceContractRefs: readonly string[];
};

type ReceiptBoundaryRowInput = Infer<typeof SocialAlertReportProviderReceiptBoundaryRowBaseSchema>;
type ReceiptBoundaryReadModelInput = Infer<typeof SocialAlertReportProviderReceiptBoundaryReadModelBaseSchema>;

export function buildSocialAlertReportProviderReceiptBoundaryReadModel(
  options: SocialAlertReportProviderReceiptBoundaryOptions,
  sourceReadModel: SocialAlertReportProviderStatusHandoffReadModel
): SocialAlertReportProviderReceiptBoundaryReadModel {
  const parsedSource = SocialAlertReportProviderStatusHandoffReadModelSchema.parse(sourceReadModel);
  const rows = parsedSource.rows.map(socialAlertReportProviderReceiptBoundaryRowForHandoffRow);

  return SocialAlertReportProviderReceiptBoundaryReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    receiptBoundaryId: options.receiptBoundaryId,
    generatedAt: options.generatedAt,
    sourceProviderStatusHandoffId: parsedSource.handoffId,
    sourceContractRefs: options.sourceContractRefs,
    sourceProviderStatusHandoffNonClaims: RequiredSocialAlertReportProviderStatusHandoffNonClaims,
    rows,
    providerDispatchRequiredCount: countReceiptRows(rows, 'provider-dispatch-required'),
    manualReceiptRequiredCount: countReceiptRows(rows, 'manual-receipt-required'),
    providerUnavailableCount: countReceiptRows(rows, 'provider-unavailable'),
    receiptBoundaryNonClaims: RequiredSocialAlertReportProviderReceiptBoundaryNonClaims,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionRuntimeClaimed: false,
    providerWebhookRuntimeClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    parentNotificationUiDeliveryClaimed: false,
    reportDeliveryExecutionClaimed: false,
    finalPolicyExecutionClaimed: false,
    connectorNativeRuntimeClaimed: false,
    enforcementClaimed: false,
  });
}

export function summarizeSocialAlertReportProviderReceiptBoundary(
  readModel: SocialAlertReportProviderReceiptBoundaryReadModel
) {
  return {
    rows: readModel.rows.length,
    providerDispatchRequiredCount: readModel.providerDispatchRequiredCount,
    manualReceiptRequiredCount: readModel.manualReceiptRequiredCount,
    providerUnavailableCount: readModel.providerUnavailableCount,
    providerDeliveryRuntimeClaimed: readModel.providerDeliveryRuntimeClaimed,
    providerReceiptIngestionRuntimeClaimed: readModel.providerReceiptIngestionRuntimeClaimed,
    finalPolicyExecutionClaimed: readModel.finalPolicyExecutionClaimed,
    enforcementClaimed: readModel.enforcementClaimed,
  };
}

function socialAlertReportProviderReceiptBoundaryRowForHandoffRow(
  row: SocialAlertReportProviderStatusHandoffRow
): SocialAlertReportProviderReceiptBoundaryRow {
  const receiptBoundaryState = receiptBoundaryStateForHandoffRow(row);

  return SocialAlertReportProviderReceiptBoundaryRowSchema.parse({
    receiptRowId: `social-provider-receipt-${row.handoffRowId}`,
    sourceProviderStatusHandoffRowRef: row.handoffRowId,
    sourcePreflightRowRef: row.sourcePreflightRowId,
    sourceIntentRef: row.sourceIntentRef,
    sourceLocalOutboxRecordRef: row.sourceLocalOutboxRecordRef,
    sourceProviderStatusEntryRef: row.providerStatusBoundaryEntry.statusEntryId,
    sourceProviderStatus: row.providerStatusBoundaryEntry.providerStatus,
    receiptBoundaryState,
    providerAttemptRef: row.providerStatusBoundaryEntry.providerAttemptRef,
    providerReceiptRefs: row.providerStatusBoundaryEntry.providerReceiptRefs,
    auditRefs: row.providerStatusBoundaryEntry.auditRefs,
    readinessRefs: row.providerStatusBoundaryEntry.readinessRefs,
    manualProofRequirements: row.manualProofRequirements,
    receiptProofRequirements: receiptProofRequirementsFor(row, receiptBoundaryState),
    providerDeliveryClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerWebhookRuntimeClaimed: false,
    providerCredentialClaimed: false,
    cloudRoutingClaimed: false,
    parentNotificationUiDeliveryClaimed: false,
    reportDeliveryExecutionClaimed: false,
    finalPolicyExecutionClaimed: false,
    connectorNativeRuntimeClaimed: false,
    enforcementClaimed: false,
  });
}

function receiptBoundaryStateForHandoffRow(
  row: SocialAlertReportProviderStatusHandoffRow
): SocialAlertReportProviderReceiptBoundaryState {
  if (row.providerStatusBoundaryEntry.providerStatus === 'unavailable') {
    return 'provider-unavailable';
  }
  if (row.sourceLocalOutboxRecordRef === null) {
    return 'manual-receipt-required';
  }
  return 'provider-dispatch-required';
}

function receiptProofRequirementsFor(
  row: SocialAlertReportProviderStatusHandoffRow,
  state: SocialAlertReportProviderReceiptBoundaryState
): readonly string[] {
  if (state === 'provider-unavailable') {
    return [`social-provider-receipt-provider-unavailable-${row.sourceIntentRef}`];
  }
  if (state === 'manual-receipt-required') {
    return [`social-provider-receipt-manual-provider-setup-${row.sourceIntentRef}`];
  }
  return [
    `social-provider-dispatch-runtime-required-${row.sourceIntentRef}`,
    `social-provider-receipt-ingestion-contract-required-${row.sourceIntentRef}`,
  ];
}

function socialAlertReportProviderReceiptBoundaryRowIsHonest(row: ReceiptBoundaryRowInput): boolean {
  return (
    receiptStateMatchesSource(row) &&
    row.auditRefs.length > 0 &&
    row.manualProofRequirements.length > 0 &&
    row.receiptProofRequirements.length > 0 &&
    row.providerReceiptRefs.length === 0 &&
    socialAlertReportProviderReceiptClaimsStayFalse(row)
  );
}

function receiptStateMatchesSource(row: ReceiptBoundaryRowInput): boolean {
  if (row.receiptBoundaryState === 'provider-unavailable') {
    return row.sourceProviderStatus === 'unavailable' && row.sourceLocalOutboxRecordRef === null;
  }
  if (row.receiptBoundaryState === 'manual-receipt-required') {
    return row.sourceProviderStatus === 'manual-required' && row.sourceLocalOutboxRecordRef === null;
  }
  return row.sourceProviderStatus === 'manual-required' && row.sourceLocalOutboxRecordRef !== null;
}

function socialAlertReportProviderReceiptClaimsStayFalse(row: ReceiptBoundaryRowInput): boolean {
  return [
    row.providerDeliveryClaimed,
    row.providerReceiptIngestionClaimed,
    row.providerWebhookRuntimeClaimed,
    row.providerCredentialClaimed,
    row.cloudRoutingClaimed,
    row.parentNotificationUiDeliveryClaimed,
    row.reportDeliveryExecutionClaimed,
    row.finalPolicyExecutionClaimed,
    row.connectorNativeRuntimeClaimed,
    row.enforcementClaimed,
  ].every((claim) => claim === false);
}

function socialAlertReportProviderReceiptBoundaryReadModelIsHonest(readModel: ReceiptBoundaryReadModelInput): boolean {
  const sourceStatusHandoffNonClaims: readonly string[] = readModel.sourceProviderStatusHandoffNonClaims;

  return (
    readModel.providerDispatchRequiredCount === countReceiptRows(readModel.rows, 'provider-dispatch-required') &&
    readModel.manualReceiptRequiredCount === countReceiptRows(readModel.rows, 'manual-receipt-required') &&
    readModel.providerUnavailableCount === countReceiptRows(readModel.rows, 'provider-unavailable') &&
    RequiredSocialAlertReportProviderStatusHandoffNonClaims.every((claim) =>
      sourceStatusHandoffNonClaims.includes(claim)
    ) &&
    RequiredSocialAlertReportProviderReceiptBoundaryNonClaims.every((claim) =>
      readModel.receiptBoundaryNonClaims.includes(claim)
    )
  );
}

function countReceiptRows(
  rows: ReadonlyArray<{ readonly receiptBoundaryState: SocialAlertReportProviderReceiptBoundaryState }>,
  receiptBoundaryState: SocialAlertReportProviderReceiptBoundaryState
): number {
  return rows.filter((row) => row.receiptBoundaryState === receiptBoundaryState).length;
}
