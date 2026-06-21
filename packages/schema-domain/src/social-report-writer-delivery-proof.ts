import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  SocialAlertReportDeliveryClaimState,
  SocialAlertReportIntentStatus,
} from '@ocentra-parent/schema-domain/social-alert-report-intent';
import {
  SocialAlertReportDeliveryClaimStateSchema,
  SocialAlertReportIntentIdSchema,
  SocialAlertReportIntentStatusSchema,
  SocialAlertReportReferenceSchema,
} from '@ocentra-parent/schema-domain/social-alert-report-intent-values';
import {
  SocialAlertReportProviderReceiptIngestionReadinessReadModelSchema,
  type SocialAlertReportProviderReceiptIngestionReadinessReadModel,
  type SocialAlertReportProviderReceiptIngestionReadinessRow,
} from './social-alert-report-provider-receipt-ingestion-readiness';

const SocialReportWriterRowsSchema = Schema.Array(SocialAlertReportReferenceSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social report writer row refs')
);
const RequiredNonClaims = {
  NoExternalRuntimeReportDelivery: SocialAlertReportReferenceSchema.parse('no-external-runtime-report-delivery'),
  NoProviderDelivery: SocialAlertReportReferenceSchema.parse('no-provider-delivery'),
  NoFinalPolicyExecution: SocialAlertReportReferenceSchema.parse('no-final-policy-execution'),
  NoEnforcement: SocialAlertReportReferenceSchema.parse('no-enforcement'),
} as const;

export const SocialReportWriterDeliveryState = {
  ReportDeliveryReady: 'report-delivery-ready',
  ManualRequired: 'manual-required',
  Unavailable: 'unavailable',
} as const;

export const SocialReportWriterReceiptState = {
  ParentOwnedReceiptRecorded: 'parent-owned-report-receipt-recorded',
  NotRecorded: 'not-recorded',
  ManualRequired: 'manual-required',
} as const;

export const SocialReportWriterDeliveryStateSchema = withParser(
  Schema.Literal(...Object.values(SocialReportWriterDeliveryState))
);
export const SocialReportWriterReceiptStateSchema = withParser(
  Schema.Literal(...Object.values(SocialReportWriterReceiptState))
);

export const SocialReportWriterDeliveryRowSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    reportWriterDeliveryRowId: SocialAlertReportReferenceSchema,
    sourceIntentRef: SocialAlertReportIntentIdSchema,
    sourceIntentStatus: SocialAlertReportIntentStatusSchema,
    sourceDeliveryClaimState: SocialAlertReportDeliveryClaimStateSchema,
    parentReportRef: Schema.Union(SocialAlertReportReferenceSchema, Schema.Null),
    reportArtifactRef: Schema.Union(SocialAlertReportReferenceSchema, Schema.Null),
    reportReceiptRef: Schema.Union(SocialAlertReportReferenceSchema, Schema.Null),
    parentVisibleReportStatusRef: Schema.Union(SocialAlertReportReferenceSchema, Schema.Null),
    sourceEvidenceRefs: SocialReportWriterRowsSchema,
    sourcePolicyRefs: SocialReportWriterRowsSchema,
    sourceAuditRefs: SocialReportWriterRowsSchema,
    manualProofRequirements: Schema.Array(SocialAlertReportReferenceSchema),
    reportWriterDeliveryState: SocialReportWriterDeliveryStateSchema,
    reportWriterReceiptState: SocialReportWriterReceiptStateSchema,
    parentOwnedReportArtifactWritten: Schema.Boolean,
    parentOwnedReportReceiptRecorded: Schema.Boolean,
    externalRuntimeReportDeliveryClaimed: Schema.Literal(false),
    providerDeliveryAttempted: Schema.Literal(false),
    providerReceiptIngested: Schema.Literal(false),
    rawAccountDataIncluded: Schema.Literal(false),
    rawVideoContentIncluded: Schema.Literal(false),
    rawMessageContentIncluded: Schema.Literal(false),
    screenshotIncluded: Schema.Literal(false),
    finalPolicyDecisionClaimed: Schema.Literal(false),
    enforcementClaimed: Schema.Literal(false),
    createdAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        socialReportWriterDeliveryStateIsCoherent(row) ||
        'Expected social report writer rows to match delivery receipt and manual states'
    )
  )
);

export const SocialReportWriterDeliveryProofReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    proofId: SocialAlertReportReferenceSchema,
    sourceAlertReportIntentProofRef: SocialAlertReportReferenceSchema,
    reportWriterDeliveryRows: Schema.Array(SocialReportWriterDeliveryRowSchema).pipe(
      Schema.filter((value) => value.length > 0 || 'Expected social report writer delivery rows')
    ),
    nonClaims: SocialReportWriterRowsSchema,
    generatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (readModel) =>
        socialReportWriterReadModelHasRequiredNonClaims(readModel) ||
        'Expected social report writer read model to preserve external delivery and enforcement non-claims'
    )
  )
);

export type SocialReportWriterDeliveryRow = Infer<typeof SocialReportWriterDeliveryRowSchema>;
export type SocialReportWriterDeliveryProofReadModel = Infer<typeof SocialReportWriterDeliveryProofReadModelSchema>;
export type SocialReportWriterDeliveryProofFromReceiptIngestionOptions = {
  readonly generatedAt: string;
  readonly proofId: string;
  readonly sourceAlertReportIntentProofRef: string;
};
type SocialReportWriterReference = Infer<typeof SocialAlertReportReferenceSchema>;
type SocialReportWriterDeliveryCandidate = {
  readonly sourceIntentStatus: Infer<typeof SocialAlertReportIntentStatusSchema>;
  readonly sourceDeliveryClaimState: Infer<typeof SocialAlertReportDeliveryClaimStateSchema>;
  readonly parentReportRef: SocialReportWriterReference | null;
  readonly reportArtifactRef: SocialReportWriterReference | null;
  readonly reportReceiptRef: SocialReportWriterReference | null;
  readonly parentVisibleReportStatusRef: SocialReportWriterReference | null;
  readonly manualProofRequirements: ReadonlyArray<SocialReportWriterReference>;
  readonly reportWriterDeliveryState: Infer<typeof SocialReportWriterDeliveryStateSchema>;
  readonly reportWriterReceiptState: Infer<typeof SocialReportWriterReceiptStateSchema>;
  readonly parentOwnedReportArtifactWritten: boolean;
  readonly parentOwnedReportReceiptRecorded: boolean;
};

export const SocialReportWriterDeliveryProofReadModel = SocialReportWriterDeliveryProofReadModelSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  proofId: 'social-report-writer-delivery-proof',
  sourceAlertReportIntentProofRef: 'social-alert-report-intent-proof',
  reportWriterDeliveryRows: [
    {
      schemaVersion: ParentContractSchemaVersion.V0_6,
      reportWriterDeliveryRowId: 'social-report-delivery-weekly-summary',
      sourceIntentRef: 'social-alert-report-weekly-summary',
      sourceIntentStatus: SocialAlertReportIntentStatus.IntentOnly,
      sourceDeliveryClaimState: SocialAlertReportDeliveryClaimState.NotClaimed,
      parentReportRef: 'parent-report-social-weekly-summary',
      reportArtifactRef: 'parent-owned-social-weekly-report-artifact',
      reportReceiptRef: 'parent-owned-social-weekly-report-receipt',
      parentVisibleReportStatusRef: 'parent-visible-social-weekly-report-status',
      sourceEvidenceRefs: ['evidence-social-decision-memory'],
      sourcePolicyRefs: ['policy-ref-social-weekly-summary'],
      sourceAuditRefs: ['audit-ref-social-report-writer'],
      manualProofRequirements: [],
      reportWriterDeliveryState: SocialReportWriterDeliveryState.ReportDeliveryReady,
      reportWriterReceiptState: SocialReportWriterReceiptState.ParentOwnedReceiptRecorded,
      parentOwnedReportArtifactWritten: true,
      parentOwnedReportReceiptRecorded: true,
      externalRuntimeReportDeliveryClaimed: false,
      providerDeliveryAttempted: false,
      providerReceiptIngested: false,
      rawAccountDataIncluded: false,
      rawVideoContentIncluded: false,
      rawMessageContentIncluded: false,
      screenshotIncluded: false,
      finalPolicyDecisionClaimed: false,
      enforcementClaimed: false,
      createdAt: '2026-06-07T04:28:00Z',
    },
    {
      schemaVersion: ParentContractSchemaVersion.V0_6,
      reportWriterDeliveryRowId: 'social-report-delivery-manual-required',
      sourceIntentRef: 'social-alert-report-manual-required',
      sourceIntentStatus: SocialAlertReportIntentStatus.ManualRequired,
      sourceDeliveryClaimState: SocialAlertReportDeliveryClaimState.ManualRequired,
      parentReportRef: null,
      reportArtifactRef: null,
      reportReceiptRef: null,
      parentVisibleReportStatusRef: 'parent-visible-social-report-manual-required',
      sourceEvidenceRefs: ['evidence-social-native-manual-required'],
      sourcePolicyRefs: ['policy-ref-social-manual-required'],
      sourceAuditRefs: ['audit-ref-social-report-manual-required'],
      manualProofRequirements: ['manual-proof-social-provider-report-runtime-required'],
      reportWriterDeliveryState: SocialReportWriterDeliveryState.ManualRequired,
      reportWriterReceiptState: SocialReportWriterReceiptState.ManualRequired,
      parentOwnedReportArtifactWritten: false,
      parentOwnedReportReceiptRecorded: false,
      externalRuntimeReportDeliveryClaimed: false,
      providerDeliveryAttempted: false,
      providerReceiptIngested: false,
      rawAccountDataIncluded: false,
      rawVideoContentIncluded: false,
      rawMessageContentIncluded: false,
      screenshotIncluded: false,
      finalPolicyDecisionClaimed: false,
      enforcementClaimed: false,
      createdAt: '2026-06-07T04:28:00Z',
    },
  ],
  nonClaims: [
    'no-external-runtime-report-delivery',
    'no-provider-delivery',
    'no-provider-receipt-ingestion',
    'no-raw-social-content',
    'no-final-policy-execution',
    'no-enforcement',
  ],
  generatedAt: '2026-06-07T04:28:00Z',
});

export function summarizeSocialReportWriterDeliveryProof(readModel: SocialReportWriterDeliveryProofReadModel) {
  return {
    totalRows: readModel.reportWriterDeliveryRows.length,
    reportDeliveryReadyRows: readModel.reportWriterDeliveryRows.filter(
      (row) => row.reportWriterDeliveryState === SocialReportWriterDeliveryState.ReportDeliveryReady
    ).length,
    manualRequiredRows: readModel.reportWriterDeliveryRows.filter(
      (row) => row.reportWriterDeliveryState === SocialReportWriterDeliveryState.ManualRequired
    ).length,
    unavailableRows: readModel.reportWriterDeliveryRows.filter(
      (row) => row.reportWriterDeliveryState === SocialReportWriterDeliveryState.Unavailable
    ).length,
    externalRuntimeReportDeliveryClaimed: readModel.reportWriterDeliveryRows.some(
      (row) => row.externalRuntimeReportDeliveryClaimed
    ),
    providerDeliveryAttempted: readModel.reportWriterDeliveryRows.some((row) => row.providerDeliveryAttempted),
    enforcementClaimed: readModel.reportWriterDeliveryRows.some((row) => row.enforcementClaimed),
  };
}

export function buildSocialReportWriterDeliveryProofFromReceiptIngestionReadiness(
  options: SocialReportWriterDeliveryProofFromReceiptIngestionOptions,
  sourceReadModel: SocialAlertReportProviderReceiptIngestionReadinessReadModel
): SocialReportWriterDeliveryProofReadModel {
  const parsedSource = SocialAlertReportProviderReceiptIngestionReadinessReadModelSchema.parse(sourceReadModel);
  const rows = parsedSource.rows.map((row) => socialReportWriterDeliveryRowFromReceiptIngestion(row, options));

  return SocialReportWriterDeliveryProofReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    proofId: options.proofId,
    sourceAlertReportIntentProofRef: options.sourceAlertReportIntentProofRef,
    reportWriterDeliveryRows: rows,
    nonClaims: [
      'no-external-runtime-report-delivery',
      'no-provider-delivery',
      'no-provider-receipt-ingestion',
      'no-raw-social-content',
      'no-final-policy-execution',
      'no-enforcement',
    ],
    generatedAt: options.generatedAt,
  });
}

function socialReportWriterDeliveryRowFromReceiptIngestion(
  row: SocialAlertReportProviderReceiptIngestionReadinessRow,
  options: SocialReportWriterDeliveryProofFromReceiptIngestionOptions
): SocialReportWriterDeliveryRow {
  const unavailable = row.ingestionReadinessState === 'provider-unavailable';
  const manualProofRequirements = [
    ...row.receiptProofRequirements,
    ...row.ingestionProofRequirements,
    `social-report-writer-provider-receipt-runtime-required-${row.sourceIntentRef}`,
  ];

  return SocialReportWriterDeliveryRowSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    reportWriterDeliveryRowId: `social-report-writer-${row.ingestionRowId}`,
    sourceIntentRef: row.sourceIntentRef,
    sourceIntentStatus: unavailable
      ? SocialAlertReportIntentStatus.Unavailable
      : SocialAlertReportIntentStatus.ManualRequired,
    sourceDeliveryClaimState: SocialAlertReportDeliveryClaimState.ManualRequired,
    parentReportRef: null,
    reportArtifactRef: null,
    reportReceiptRef: null,
    parentVisibleReportStatusRef: `parent-visible-social-report-writer-${row.ingestionRowId}`,
    sourceEvidenceRefs: [row.sourceReceiptRowRef],
    sourcePolicyRefs: [`social-report-writer-source-policy-${row.sourceIntentRef}`],
    sourceAuditRefs: [row.ingestionRowId],
    manualProofRequirements,
    reportWriterDeliveryState: unavailable
      ? SocialReportWriterDeliveryState.Unavailable
      : SocialReportWriterDeliveryState.ManualRequired,
    reportWriterReceiptState: unavailable
      ? SocialReportWriterReceiptState.NotRecorded
      : SocialReportWriterReceiptState.ManualRequired,
    parentOwnedReportArtifactWritten: false,
    parentOwnedReportReceiptRecorded: false,
    externalRuntimeReportDeliveryClaimed: false,
    providerDeliveryAttempted: false,
    providerReceiptIngested: false,
    rawAccountDataIncluded: false,
    rawVideoContentIncluded: false,
    rawMessageContentIncluded: false,
    screenshotIncluded: false,
    finalPolicyDecisionClaimed: false,
    enforcementClaimed: false,
    createdAt: options.generatedAt,
  });
}

function socialReportWriterDeliveryStateIsCoherent(row: SocialReportWriterDeliveryCandidate): boolean {
  if (row.reportWriterDeliveryState === SocialReportWriterDeliveryState.ReportDeliveryReady) {
    return socialReportWriterDeliveryReadyStateIsCoherent(row);
  }

  if (socialReportWriterDeliveryStateNeedsManualProof(row)) {
    return socialReportWriterDeliveryManualStateIsCoherent(row);
  }

  return false;
}

function socialReportWriterDeliveryReadyStateIsCoherent(row: SocialReportWriterDeliveryCandidate): boolean {
  return (
    row.sourceIntentStatus === SocialAlertReportIntentStatus.IntentOnly &&
    row.sourceDeliveryClaimState === SocialAlertReportDeliveryClaimState.NotClaimed &&
    row.parentReportRef !== null &&
    row.reportArtifactRef !== null &&
    row.reportReceiptRef !== null &&
    row.parentVisibleReportStatusRef !== null &&
    row.parentOwnedReportArtifactWritten &&
    row.parentOwnedReportReceiptRecorded &&
    row.reportWriterReceiptState === SocialReportWriterReceiptState.ParentOwnedReceiptRecorded &&
    row.manualProofRequirements.length === 0
  );
}

function socialReportWriterDeliveryStateNeedsManualProof(row: SocialReportWriterDeliveryCandidate): boolean {
  return (
    row.reportWriterDeliveryState === SocialReportWriterDeliveryState.ManualRequired ||
    row.reportWriterDeliveryState === SocialReportWriterDeliveryState.Unavailable
  );
}

function socialReportWriterDeliveryManualStateIsCoherent(row: SocialReportWriterDeliveryCandidate): boolean {
  return (
    row.parentReportRef === null &&
    row.reportArtifactRef === null &&
    row.reportReceiptRef === null &&
    !row.parentOwnedReportArtifactWritten &&
    !row.parentOwnedReportReceiptRecorded &&
    row.reportWriterReceiptState !== SocialReportWriterReceiptState.ParentOwnedReceiptRecorded &&
    row.manualProofRequirements.length > 0
  );
}

function socialReportWriterReadModelHasRequiredNonClaims(readModel: {
  readonly nonClaims: ReadonlyArray<SocialReportWriterReference>;
}): boolean {
  const nonClaims = new Set(readModel.nonClaims);
  return (
    nonClaims.has(RequiredNonClaims.NoExternalRuntimeReportDelivery) &&
    nonClaims.has(RequiredNonClaims.NoProviderDelivery) &&
    nonClaims.has(RequiredNonClaims.NoFinalPolicyExecution) &&
    nonClaims.has(RequiredNonClaims.NoEnforcement)
  );
}

