/* generated support for crates/browser-core/src/social_alert_report_provider_dispatch_execution.rs */

import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import {
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';

const RequiredSocialAlertReportProviderStatusHandoffNonClaims = [
  'no-provider-delivery-execution',
  'no-provider-receipt-ingestion',
  'no-provider-credentials',
  'no-cloud-routing',
  'no-parent-notification-ui-delivery',
  'no-report-delivery-execution',
  'no-final-policy-execution',
  'no-connector-native-runtime',
  'no-enforcement',
] as const;

const V08NotificationProviderStatusSchema = withParser(
  Schema.Literal('queued', 'delivered', 'failed', 'unavailable', 'manual-required')
);

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

const SocialAlertReportProviderReceiptBoundaryReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  receiptBoundaryId: SocialAlertReportProviderReceiptBoundaryIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceProviderStatusHandoffId: SocialAlertReportProviderReceiptBoundaryReferenceSchema,
  sourceContractRefs: Schema.Array(SocialAlertReportProviderReceiptBoundaryReferenceSchema),
  sourceProviderStatusHandoffNonClaims: Schema.Array(SocialAlertReportProviderReceiptBoundaryReferenceSchema),
  rows: Schema.Array(Schema.suspend(() => SocialAlertReportProviderReceiptBoundaryRowSchema)),
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

type ReceiptBoundaryRowInput = Infer<typeof SocialAlertReportProviderReceiptBoundaryRowBaseSchema>;
type ReceiptBoundaryReadModelInput = Infer<typeof SocialAlertReportProviderReceiptBoundaryReadModelBaseSchema>;

export const SocialAlertReportProviderReceiptBoundaryRowSchema = withParser(
  SocialAlertReportProviderReceiptBoundaryRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        socialAlertReportProviderReceiptBoundaryRowIsHonest(row) ||
        'Expected social alert/report provider receipt boundary rows to preserve provider status handoff refs and keep delivery/receipt runtime unclaimed'
    )
  )
);

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
