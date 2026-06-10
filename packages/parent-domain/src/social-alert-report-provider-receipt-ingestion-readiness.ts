import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from './reference-primitives';
import {
  RequiredSocialAlertReportProviderReceiptBoundaryNonClaims,
  SocialAlertReportProviderReceiptBoundaryReadModelSchema,
  type SocialAlertReportProviderReceiptBoundaryReadModel,
  type SocialAlertReportProviderReceiptBoundaryRow,
} from './social-alert-report-provider-receipt-boundary-proof';

const ReceiptIngestionText = Schema.String.pipe(Schema.minLength(1));

export const RequiredSocialAlertReportProviderReceiptIngestionReadinessNonClaims = [
  'no-provider-delivery-execution',
  'no-provider-receipt-ingestion-runtime',
  'no-provider-webhook-runtime',
  'no-provider-credentials',
  'no-provider-receipt-observed',
  'no-cloud-routing',
  'no-parent-notification-ui-delivery',
  'no-report-delivery-execution',
  'no-final-policy-execution',
  'no-connector-native-runtime',
  'no-enforcement',
] as const;

export const SocialAlertReportProviderReceiptIngestionReadinessNonClaimSchema = withParser(
  Schema.Literal(...RequiredSocialAlertReportProviderReceiptIngestionReadinessNonClaims)
);
export const SocialAlertReportProviderReceiptIngestionReadinessIdSchema = withParser(
  ReceiptIngestionText.pipe(Schema.brand('SocialAlertReportProviderReceiptIngestionReadinessId'))
);
export const SocialAlertReportProviderReceiptIngestionReadinessReferenceSchema = withParser(
  ReceiptIngestionText.pipe(Schema.brand('SocialAlertReportProviderReceiptIngestionReadinessReference'))
);
export const SocialAlertReportProviderReceiptIngestionReadinessStateSchema = withParser(
  Schema.Literal('ingestion-contract-required', 'manual-receipt-required', 'provider-unavailable')
);

const SocialAlertReportProviderReceiptIngestionReadinessRowBaseSchema = Schema.Struct({
  ingestionRowId: SocialAlertReportProviderReceiptIngestionReadinessReferenceSchema,
  sourceReceiptRowRef: SocialAlertReportProviderReceiptIngestionReadinessReferenceSchema,
  sourceIntentRef: SocialAlertReportProviderReceiptIngestionReadinessReferenceSchema,
  sourceProviderAttemptRef: SocialAlertReportProviderReceiptIngestionReadinessReferenceSchema,
  sourceReceiptBoundaryState: Schema.Literal(
    'provider-dispatch-required',
    'manual-receipt-required',
    'provider-unavailable'
  ),
  ingestionReadinessState: SocialAlertReportProviderReceiptIngestionReadinessStateSchema,
  webhookEndpointRef: Schema.Union(SocialAlertReportProviderReceiptIngestionReadinessReferenceSchema, Schema.Null),
  providerCredentialRef: Schema.Union(SocialAlertReportProviderReceiptIngestionReadinessReferenceSchema, Schema.Null),
  durableReceiptResultRef: Schema.Union(SocialAlertReportProviderReceiptIngestionReadinessReferenceSchema, Schema.Null),
  providerReceiptObservedRefs: Schema.Array(SocialAlertReportProviderReceiptIngestionReadinessReferenceSchema),
  receiptProofRequirements: Schema.Array(SocialAlertReportProviderReceiptIngestionReadinessReferenceSchema),
  ingestionProofRequirements: Schema.Array(SocialAlertReportProviderReceiptIngestionReadinessReferenceSchema),
  providerDeliveryExecutionClaimed: Schema.Literal(false),
  providerReceiptIngestionRuntimeClaimed: Schema.Literal(false),
  providerWebhookRuntimeClaimed: Schema.Literal(false),
  providerCredentialsClaimed: Schema.Literal(false),
  providerReceiptObservedClaimed: Schema.Literal(false),
  cloudRoutingClaimed: Schema.Literal(false),
  parentNotificationUiDeliveryClaimed: Schema.Literal(false),
  reportDeliveryExecutionClaimed: Schema.Literal(false),
  finalPolicyExecutionClaimed: Schema.Literal(false),
  connectorNativeRuntimeClaimed: Schema.Literal(false),
  enforcementClaimed: Schema.Literal(false),
});

export const SocialAlertReportProviderReceiptIngestionReadinessRowSchema = withParser(
  SocialAlertReportProviderReceiptIngestionReadinessRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        socialAlertReportProviderReceiptIngestionReadinessRowIsHonest(row) ||
        'Expected social alert/report provider receipt ingestion readiness rows to keep webhook, credentials, durable receipt, and ingestion runtime unclaimed'
    )
  )
);

const SocialAlertReportProviderReceiptIngestionReadinessReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  readinessId: SocialAlertReportProviderReceiptIngestionReadinessIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceReceiptBoundaryId: SocialAlertReportProviderReceiptIngestionReadinessReferenceSchema,
  sourceContractRefs: Schema.Array(SocialAlertReportProviderReceiptIngestionReadinessReferenceSchema),
  sourceReceiptBoundaryNonClaims: Schema.Array(SocialAlertReportProviderReceiptIngestionReadinessReferenceSchema),
  rows: Schema.Array(SocialAlertReportProviderReceiptIngestionReadinessRowSchema),
  ingestionContractRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  manualReceiptRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  providerUnavailableCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  providerReceiptObservedCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  receiptIngestionReadinessNonClaims: Schema.Array(SocialAlertReportProviderReceiptIngestionReadinessNonClaimSchema),
  providerDeliveryRuntimeClaimed: Schema.Literal(false),
  providerReceiptIngestionRuntimeClaimed: Schema.Literal(false),
  providerWebhookRuntimeClaimed: Schema.Literal(false),
  providerCredentialsClaimed: Schema.Literal(false),
  providerReceiptObservedClaimed: Schema.Literal(false),
  cloudRoutingClaimed: Schema.Literal(false),
  parentNotificationUiDeliveryClaimed: Schema.Literal(false),
  reportDeliveryExecutionClaimed: Schema.Literal(false),
  finalPolicyExecutionClaimed: Schema.Literal(false),
  connectorNativeRuntimeClaimed: Schema.Literal(false),
  enforcementClaimed: Schema.Literal(false),
});

export const SocialAlertReportProviderReceiptIngestionReadinessReadModelSchema = withParser(
  SocialAlertReportProviderReceiptIngestionReadinessReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        socialAlertReportProviderReceiptIngestionReadinessReadModelIsHonest(readModel) ||
        'Expected social alert/report provider receipt ingestion readiness counts and non-claims to match source receipt boundary rows'
    )
  )
);

export type SocialAlertReportProviderReceiptIngestionReadinessState = Infer<
  typeof SocialAlertReportProviderReceiptIngestionReadinessStateSchema
>;
export type SocialAlertReportProviderReceiptIngestionReadinessRow = Infer<
  typeof SocialAlertReportProviderReceiptIngestionReadinessRowSchema
>;
export type SocialAlertReportProviderReceiptIngestionReadinessReadModel = Infer<
  typeof SocialAlertReportProviderReceiptIngestionReadinessReadModelSchema
>;

export type SocialAlertReportProviderReceiptIngestionReadinessOptions = {
  readonly generatedAt: string;
  readonly readinessId: string;
  readonly sourceContractRefs: readonly string[];
};

type ReceiptIngestionReadinessRowInput = Infer<typeof SocialAlertReportProviderReceiptIngestionReadinessRowBaseSchema>;
type ReceiptIngestionReadinessReadModelInput = Infer<
  typeof SocialAlertReportProviderReceiptIngestionReadinessReadModelBaseSchema
>;

export function buildSocialAlertReportProviderReceiptIngestionReadinessReadModel(
  options: SocialAlertReportProviderReceiptIngestionReadinessOptions,
  sourceReadModel: SocialAlertReportProviderReceiptBoundaryReadModel
): SocialAlertReportProviderReceiptIngestionReadinessReadModel {
  const parsedSource = SocialAlertReportProviderReceiptBoundaryReadModelSchema.parse(sourceReadModel);
  const rows = parsedSource.rows.map(socialAlertReportProviderReceiptIngestionReadinessRowForReceiptRow);

  return SocialAlertReportProviderReceiptIngestionReadinessReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    readinessId: options.readinessId,
    generatedAt: options.generatedAt,
    sourceReceiptBoundaryId: parsedSource.receiptBoundaryId,
    sourceContractRefs: options.sourceContractRefs,
    sourceReceiptBoundaryNonClaims: RequiredSocialAlertReportProviderReceiptBoundaryNonClaims,
    rows,
    ingestionContractRequiredCount: countReadinessRows(rows, 'ingestion-contract-required'),
    manualReceiptRequiredCount: countReadinessRows(rows, 'manual-receipt-required'),
    providerUnavailableCount: countReadinessRows(rows, 'provider-unavailable'),
    providerReceiptObservedCount: rows.flatMap((row) => row.providerReceiptObservedRefs).length,
    receiptIngestionReadinessNonClaims: RequiredSocialAlertReportProviderReceiptIngestionReadinessNonClaims,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionRuntimeClaimed: false,
    providerWebhookRuntimeClaimed: false,
    providerCredentialsClaimed: false,
    providerReceiptObservedClaimed: false,
    cloudRoutingClaimed: false,
    parentNotificationUiDeliveryClaimed: false,
    reportDeliveryExecutionClaimed: false,
    finalPolicyExecutionClaimed: false,
    connectorNativeRuntimeClaimed: false,
    enforcementClaimed: false,
  });
}

export function summarizeSocialAlertReportProviderReceiptIngestionReadiness(
  readModel: SocialAlertReportProviderReceiptIngestionReadinessReadModel
) {
  return {
    rows: readModel.rows.length,
    ingestionContractRequiredCount: readModel.ingestionContractRequiredCount,
    manualReceiptRequiredCount: readModel.manualReceiptRequiredCount,
    providerUnavailableCount: readModel.providerUnavailableCount,
    providerReceiptObservedCount: readModel.providerReceiptObservedCount,
    providerReceiptIngestionRuntimeClaimed: readModel.providerReceiptIngestionRuntimeClaimed,
    providerWebhookRuntimeClaimed: readModel.providerWebhookRuntimeClaimed,
    providerCredentialsClaimed: readModel.providerCredentialsClaimed,
    enforcementClaimed: readModel.enforcementClaimed,
  };
}

function socialAlertReportProviderReceiptIngestionReadinessRowForReceiptRow(
  row: SocialAlertReportProviderReceiptBoundaryRow
): SocialAlertReportProviderReceiptIngestionReadinessRow {
  const ingestionReadinessState = ingestionReadinessStateForReceiptRow(row);

  return SocialAlertReportProviderReceiptIngestionReadinessRowSchema.parse({
    ingestionRowId: `social-provider-receipt-ingestion-${row.receiptRowId}`,
    sourceReceiptRowRef: row.receiptRowId,
    sourceIntentRef: row.sourceIntentRef,
    sourceProviderAttemptRef: row.providerAttemptRef,
    sourceReceiptBoundaryState: row.receiptBoundaryState,
    ingestionReadinessState,
    webhookEndpointRef: null,
    providerCredentialRef: null,
    durableReceiptResultRef: null,
    providerReceiptObservedRefs: [],
    receiptProofRequirements: row.receiptProofRequirements,
    ingestionProofRequirements: ingestionProofRequirementsFor(row, ingestionReadinessState),
    providerDeliveryExecutionClaimed: false,
    providerReceiptIngestionRuntimeClaimed: false,
    providerWebhookRuntimeClaimed: false,
    providerCredentialsClaimed: false,
    providerReceiptObservedClaimed: false,
    cloudRoutingClaimed: false,
    parentNotificationUiDeliveryClaimed: false,
    reportDeliveryExecutionClaimed: false,
    finalPolicyExecutionClaimed: false,
    connectorNativeRuntimeClaimed: false,
    enforcementClaimed: false,
  });
}

function ingestionReadinessStateForReceiptRow(
  row: SocialAlertReportProviderReceiptBoundaryRow
): SocialAlertReportProviderReceiptIngestionReadinessState {
  if (row.receiptBoundaryState === 'provider-unavailable') {
    return 'provider-unavailable';
  }
  if (row.receiptBoundaryState === 'manual-receipt-required') {
    return 'manual-receipt-required';
  }
  return 'ingestion-contract-required';
}

function ingestionProofRequirementsFor(
  row: SocialAlertReportProviderReceiptBoundaryRow,
  state: SocialAlertReportProviderReceiptIngestionReadinessState
): readonly string[] {
  if (state === 'provider-unavailable') {
    return [`social-provider-receipt-ingestion-provider-unavailable-${row.sourceIntentRef}`];
  }
  if (state === 'manual-receipt-required') {
    return [`social-provider-receipt-ingestion-manual-provider-setup-${row.sourceIntentRef}`];
  }
  return [
    `social-provider-receipt-webhook-contract-required-${row.sourceIntentRef}`,
    `social-provider-receipt-credential-proof-required-${row.sourceIntentRef}`,
    `social-provider-receipt-durable-store-required-${row.sourceIntentRef}`,
  ];
}

function socialAlertReportProviderReceiptIngestionReadinessRowIsHonest(
  row: ReceiptIngestionReadinessRowInput
): boolean {
  return (
    readinessStateMatchesReceiptBoundary(row) &&
    readinessRefsStayManual(row) &&
    row.receiptProofRequirements.length > 0 &&
    row.ingestionProofRequirements.length > 0 &&
    socialAlertReportProviderReceiptIngestionClaimsStayFalse(row)
  );
}

function readinessStateMatchesReceiptBoundary(row: ReceiptIngestionReadinessRowInput): boolean {
  if (row.ingestionReadinessState === 'provider-unavailable') {
    return row.sourceReceiptBoundaryState === 'provider-unavailable';
  }
  if (row.ingestionReadinessState === 'manual-receipt-required') {
    return row.sourceReceiptBoundaryState === 'manual-receipt-required';
  }
  return row.sourceReceiptBoundaryState === 'provider-dispatch-required';
}

function readinessRefsStayManual(row: ReceiptIngestionReadinessRowInput): boolean {
  return (
    row.webhookEndpointRef === null &&
    row.providerCredentialRef === null &&
    row.durableReceiptResultRef === null &&
    row.providerReceiptObservedRefs.length === 0
  );
}

function socialAlertReportProviderReceiptIngestionClaimsStayFalse(row: ReceiptIngestionReadinessRowInput): boolean {
  return [
    row.providerDeliveryExecutionClaimed,
    row.providerReceiptIngestionRuntimeClaimed,
    row.providerWebhookRuntimeClaimed,
    row.providerCredentialsClaimed,
    row.providerReceiptObservedClaimed,
    row.cloudRoutingClaimed,
    row.parentNotificationUiDeliveryClaimed,
    row.reportDeliveryExecutionClaimed,
    row.finalPolicyExecutionClaimed,
    row.connectorNativeRuntimeClaimed,
    row.enforcementClaimed,
  ].every((claim) => claim === false);
}

function socialAlertReportProviderReceiptIngestionReadinessReadModelIsHonest(
  readModel: ReceiptIngestionReadinessReadModelInput
): boolean {
  const sourceNonClaims: readonly string[] = readModel.sourceReceiptBoundaryNonClaims;

  return (
    readModel.ingestionContractRequiredCount === countReadinessRows(readModel.rows, 'ingestion-contract-required') &&
    readModel.manualReceiptRequiredCount === countReadinessRows(readModel.rows, 'manual-receipt-required') &&
    readModel.providerUnavailableCount === countReadinessRows(readModel.rows, 'provider-unavailable') &&
    readModel.providerReceiptObservedCount === 0 &&
    RequiredSocialAlertReportProviderReceiptBoundaryNonClaims.every((claim) => sourceNonClaims.includes(claim)) &&
    RequiredSocialAlertReportProviderReceiptIngestionReadinessNonClaims.every((claim) =>
      readModel.receiptIngestionReadinessNonClaims.includes(claim)
    )
  );
}

function countReadinessRows(
  rows: ReadonlyArray<{
    readonly ingestionReadinessState: SocialAlertReportProviderReceiptIngestionReadinessState;
  }>,
  ingestionReadinessState: SocialAlertReportProviderReceiptIngestionReadinessState
): number {
  return rows.filter((row) => row.ingestionReadinessState === ingestionReadinessState).length;
}
