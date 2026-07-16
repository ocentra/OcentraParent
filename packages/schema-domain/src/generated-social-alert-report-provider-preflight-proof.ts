/* generated from crates/browser-core/src/social_schema_generated_alert_report.rs */

import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import {
  decodeSocialAlertReportIntent,
  SocialAlertReportIntentStatus,
  type SocialAlertReportIntent,
} from './generated-social-alert-report-intent';
import {
  SocialAlertReportIntentIdSchema,
  SocialAlertReportReferenceSchema,
} from '@ocentra-parent/schema-domain/social-alert-report-intent-values';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';

const RequiredNonClaims = [
  'no-provider-delivery-execution',
  'no-provider-receipt-ingestion',
  'no-provider-credentials',
  'no-cloud-routing',
  'no-parent-notification-ui-delivery',
  'no-report-delivery-execution',
  'no-final-policy-execution',
  'no-enforcement',
] as const;

export const SocialAlertReportProviderPreflightStatus = {
  ProviderAdapterRequired: 'provider-adapter-required',
  ManualRequired: 'manual-required',
  Unavailable: 'unavailable',
} as const;

export const SocialAlertReportProviderPreflightStatusSchema = withParser(
  Schema.Literal(...Object.values(SocialAlertReportProviderPreflightStatus))
);
export const SocialAlertReportProviderPreflightNonClaimSchema = withParser(Schema.Literal(...RequiredNonClaims));
export const SocialAlertReportProviderPreflightIdSchema = withParser(
  brandedNonEmptyStringSchema('SocialAlertReportProviderPreflightId')
);

const SocialAlertReportProviderPreflightRowBaseSchema = Schema.Struct({
  preflightRowId: SocialAlertReportReferenceSchema,
  sourceIntentRef: SocialAlertReportIntentIdSchema,
  status: SocialAlertReportProviderPreflightStatusSchema,
  sourceLocalOutboxRecordRef: Schema.Union(SocialAlertReportReferenceSchema, Schema.Null),
  providerChannelRef: Schema.Union(SocialAlertReportReferenceSchema, Schema.Null),
  reasonCodeRef: Schema.Union(SocialAlertReportReferenceSchema, Schema.Null),
  evidenceRefs: Schema.Array(SocialAlertReportReferenceSchema),
  policyRefs: Schema.Array(SocialAlertReportReferenceSchema),
  auditRefs: Schema.Array(SocialAlertReportReferenceSchema),
  adapterRequirementRefs: Schema.Array(SocialAlertReportReferenceSchema),
  manualProofRequirements: Schema.Array(SocialAlertReportReferenceSchema),
});

export const SocialAlertReportProviderPreflightRowSchema = withParser(
  SocialAlertReportProviderPreflightRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        socialProviderPreflightRowIsHonest(row) ||
        'Expected social alert/report provider preflight rows to require adapter proof before delivery'
    )
  )
);

const SocialAlertReportProviderPreflightReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  providerPreflightId: SocialAlertReportProviderPreflightIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(SocialAlertReportReferenceSchema),
  rows: Schema.Array(SocialAlertReportProviderPreflightRowSchema),
  providerAdapterRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  manualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  unavailableCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  preflightNonClaims: Schema.Array(SocialAlertReportProviderPreflightNonClaimSchema),
  providerDeliveryRuntimeClaimed: Schema.Literal(false),
  providerReceiptIngestionClaimed: Schema.Literal(false),
  providerCredentialsClaimed: Schema.Literal(false),
  cloudRoutingClaimed: Schema.Literal(false),
  parentNotificationUiDeliveryClaimed: Schema.Literal(false),
  reportDeliveryExecutionClaimed: Schema.Literal(false),
  finalPolicyExecutionClaimed: Schema.Literal(false),
  enforcementClaimed: Schema.Literal(false),
});

export const SocialAlertReportProviderPreflightReadModelSchema = withParser(
  SocialAlertReportProviderPreflightReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        socialProviderPreflightReadModelIsHonest(readModel) ||
        'Expected social provider preflight counts and non-claims to match parsed rows'
    )
  )
);

export type SocialAlertReportProviderPreflightStatus = Infer<typeof SocialAlertReportProviderPreflightStatusSchema>;
export type SocialAlertReportProviderPreflightRow = Infer<typeof SocialAlertReportProviderPreflightRowSchema>;
export type SocialAlertReportProviderPreflightReadModel = Infer<
  typeof SocialAlertReportProviderPreflightReadModelSchema
>;

export type SocialAlertReportProviderPreflightOptions = {
  readonly generatedAt: string;
  readonly providerPreflightId: string;
  readonly sourceContractRefs: readonly string[];
};

export function buildSocialAlertReportProviderPreflightReadModel(
  options: SocialAlertReportProviderPreflightOptions,
  intents: ReadonlyArray<unknown>
): SocialAlertReportProviderPreflightReadModel {
  const parsedIntents = intents.map((intent) => decodeSocialAlertReportIntent(intent));
  const rows = parsedIntents.map(socialProviderPreflightRowForIntent);

  return SocialAlertReportProviderPreflightReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    providerPreflightId: options.providerPreflightId,
    generatedAt: options.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    providerAdapterRequiredCount: countRows(rows, SocialAlertReportProviderPreflightStatus.ProviderAdapterRequired),
    manualRequiredCount: countRows(rows, SocialAlertReportProviderPreflightStatus.ManualRequired),
    unavailableCount: countRows(rows, SocialAlertReportProviderPreflightStatus.Unavailable),
    preflightNonClaims: RequiredNonClaims,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    parentNotificationUiDeliveryClaimed: false,
    reportDeliveryExecutionClaimed: false,
    finalPolicyExecutionClaimed: false,
    enforcementClaimed: false,
  });
}

export function summarizeSocialAlertReportProviderPreflight(readModel: SocialAlertReportProviderPreflightReadModel) {
  return {
    totalRows: readModel.rows.length,
    providerAdapterRequiredCount: readModel.providerAdapterRequiredCount,
    manualRequiredCount: readModel.manualRequiredCount,
    unavailableCount: readModel.unavailableCount,
    providerDeliveryRuntimeClaimed: readModel.providerDeliveryRuntimeClaimed,
    finalPolicyExecutionClaimed: readModel.finalPolicyExecutionClaimed,
    enforcementClaimed: readModel.enforcementClaimed,
  };
}

function socialProviderPreflightRowForIntent(intent: SocialAlertReportIntent): SocialAlertReportProviderPreflightRow {
  if (intent.intentStatus === SocialAlertReportIntentStatus.LocalOutboxEligible) {
    return SocialAlertReportProviderPreflightRowSchema.parse({
      preflightRowId: `social-provider-preflight-${intent.alertReportIntentId}`,
      sourceIntentRef: intent.alertReportIntentId,
      status: SocialAlertReportProviderPreflightStatus.ProviderAdapterRequired,
      sourceLocalOutboxRecordRef: intent.localOutboxRecordRef,
      providerChannelRef: `social-provider-channel-${intent.providerChannelPreference}`,
      reasonCodeRef: intent.notificationReasonCode,
      evidenceRefs: intent.evidenceReferences.map(
        (ref: SocialAlertReportIntent['evidenceReferences'][number]) => ref.evidenceReferenceId
      ),
      policyRefs: intent.policyRefs,
      auditRefs: intent.auditRefs,
      adapterRequirementRefs: [
        `provider-adapter-required-${intent.alertReportIntentId}`,
        `provider-credentials-required-${intent.alertReportIntentId}`,
        `provider-smoke-proof-required-${intent.alertReportIntentId}`,
      ],
      manualProofRequirements: [
        `provider-adapter-required-${intent.alertReportIntentId}`,
        `provider-credentials-required-${intent.alertReportIntentId}`,
        `provider-smoke-proof-required-${intent.alertReportIntentId}`,
      ],
    });
  }

  return SocialAlertReportProviderPreflightRowSchema.parse({
    preflightRowId: `social-provider-preflight-${intent.alertReportIntentId}`,
    sourceIntentRef: intent.alertReportIntentId,
    status:
      intent.intentStatus === SocialAlertReportIntentStatus.Unavailable
        ? SocialAlertReportProviderPreflightStatus.Unavailable
        : SocialAlertReportProviderPreflightStatus.ManualRequired,
    sourceLocalOutboxRecordRef: null,
    providerChannelRef: null,
    reasonCodeRef: null,
    evidenceRefs: intent.evidenceReferences.map(
      (ref: SocialAlertReportIntent['evidenceReferences'][number]) => ref.evidenceReferenceId
    ),
    policyRefs: intent.policyRefs,
    auditRefs: intent.auditRefs,
    adapterRequirementRefs: intent.manualProofRequirements,
    manualProofRequirements: intent.manualProofRequirements,
  });
}

function socialProviderPreflightRowIsHonest(
  row: Infer<typeof SocialAlertReportProviderPreflightRowBaseSchema>
): boolean {
  if (row.evidenceRefs.length === 0 || row.policyRefs.length === 0 || row.auditRefs.length === 0) {
    return false;
  }

  if (row.status === SocialAlertReportProviderPreflightStatus.ProviderAdapterRequired) {
    return socialProviderPreflightAdapterRequiredRowIsHonest(row);
  }

  return socialProviderPreflightManualRowIsHonest(row);
}

function socialProviderPreflightAdapterRequiredRowIsHonest(
  row: Infer<typeof SocialAlertReportProviderPreflightRowBaseSchema>
): boolean {
  return (
    row.sourceLocalOutboxRecordRef !== null &&
    row.providerChannelRef !== null &&
    row.reasonCodeRef !== null &&
    row.adapterRequirementRefs.length >= 3 &&
    row.manualProofRequirements.length >= 3
  );
}

function socialProviderPreflightManualRowIsHonest(
  row: Infer<typeof SocialAlertReportProviderPreflightRowBaseSchema>
): boolean {
  return (
    row.sourceLocalOutboxRecordRef === null &&
    row.providerChannelRef === null &&
    row.reasonCodeRef === null &&
    row.adapterRequirementRefs.length > 0 &&
    row.manualProofRequirements.length > 0
  );
}

function socialProviderPreflightReadModelIsHonest(
  readModel: Infer<typeof SocialAlertReportProviderPreflightReadModelBaseSchema>
): boolean {
  return (
    readModel.providerAdapterRequiredCount ===
      countRows(readModel.rows, SocialAlertReportProviderPreflightStatus.ProviderAdapterRequired) &&
    readModel.manualRequiredCount ===
      countRows(readModel.rows, SocialAlertReportProviderPreflightStatus.ManualRequired) &&
    readModel.unavailableCount === countRows(readModel.rows, SocialAlertReportProviderPreflightStatus.Unavailable) &&
    RequiredNonClaims.every((claim) => readModel.preflightNonClaims.includes(claim))
  );
}

function countRows(
  rows: ReadonlyArray<{ readonly status: SocialAlertReportProviderPreflightStatus }>,
  status: SocialAlertReportProviderPreflightStatus
): number {
  return rows.filter((row) => row.status === status).length;
}
