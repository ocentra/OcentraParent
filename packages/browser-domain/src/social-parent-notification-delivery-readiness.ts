import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  SocialReportWriterDeliveryProofReadModelSchema,
  SocialReportWriterDeliveryState,
  SocialReportWriterReceiptState,
  type SocialReportWriterDeliveryProofReadModel,
  type SocialReportWriterDeliveryRow,
} from './social-report-writer-delivery-proof';
import { SocialAlertReportReferenceSchema } from './social-alert-report-intent-values';

const RequiredNonClaims = [
  'no-parent-notification-ui-delivery',
  'no-external-runtime-report-delivery',
  'no-provider-delivery',
  'no-provider-receipt-ingestion',
  'no-final-policy-execution',
  'no-enforcement',
] as const;

export const SocialParentNotificationDeliveryReadinessState = {
  ParentReportStatusReady: 'parent-report-status-ready',
  ManualRequired: 'manual-required',
  Unavailable: 'unavailable',
} as const;

export const SocialParentNotificationReportDeliveryExecutionState = {
  ParentOwnedReportReady: 'parent-owned-report-ready',
  ManualRequired: 'manual-required',
  Unavailable: 'unavailable',
} as const;

export const SocialParentNotificationDeliveryReadinessStateSchema = withParser(
  Schema.Literal(...Object.values(SocialParentNotificationDeliveryReadinessState))
);
export const SocialParentNotificationReportDeliveryExecutionStateSchema = withParser(
  Schema.Literal(...Object.values(SocialParentNotificationReportDeliveryExecutionState))
);

export const SocialParentNotificationDeliveryReadinessNonClaimSchema = withParser(Schema.Literal(...RequiredNonClaims));

const SocialParentNotificationDeliveryReadinessRefsSchema = Schema.Array(SocialAlertReportReferenceSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social parent notification delivery refs')
);

const SocialParentNotificationDeliveryReadinessRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  notificationDeliveryReadinessRowId: SocialAlertReportReferenceSchema,
  sourceReportWriterDeliveryRowRef: SocialAlertReportReferenceSchema,
  sourceIntentRef: SocialAlertReportReferenceSchema,
  parentVisibleReportStatusRef: Schema.Union(SocialAlertReportReferenceSchema, Schema.Null),
  parentNotificationUiRef: Schema.Union(SocialAlertReportReferenceSchema, Schema.Null),
  parentLocalDeliveryResultRef: Schema.Union(SocialAlertReportReferenceSchema, Schema.Null),
  parentReportRef: Schema.Union(SocialAlertReportReferenceSchema, Schema.Null),
  reportArtifactRef: Schema.Union(SocialAlertReportReferenceSchema, Schema.Null),
  reportReceiptRef: Schema.Union(SocialAlertReportReferenceSchema, Schema.Null),
  sourceEvidenceRefs: SocialParentNotificationDeliveryReadinessRefsSchema,
  sourcePolicyRefs: SocialParentNotificationDeliveryReadinessRefsSchema,
  sourceAuditRefs: SocialParentNotificationDeliveryReadinessRefsSchema,
  manualProofRequirements: Schema.Array(SocialAlertReportReferenceSchema),
  notificationDeliveryReadinessState: SocialParentNotificationDeliveryReadinessStateSchema,
  reportDeliveryExecutionState: SocialParentNotificationReportDeliveryExecutionStateSchema,
  parentLocalDeliveryResultRecorded: Schema.Boolean,
  parentOwnedReportArtifactWritten: Schema.Boolean,
  parentOwnedReportReceiptRecorded: Schema.Boolean,
  parentNotificationUiDelivered: Schema.Literal(false),
  externalRuntimeReportDeliveryClaimed: Schema.Literal(false),
  providerDeliveryAttempted: Schema.Literal(false),
  providerReceiptIngested: Schema.Literal(false),
  finalPolicyDecisionClaimed: Schema.Literal(false),
  enforcementClaimed: Schema.Literal(false),
  createdAt: ParentTimestampSchema,
});

export const SocialParentNotificationDeliveryReadinessRowSchema = withParser(
  SocialParentNotificationDeliveryReadinessRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        socialParentNotificationDeliveryReadinessRowIsHonest(row) ||
        'Expected parent notification delivery readiness rows to preserve report-writer and runtime non-claim boundaries'
    )
  )
);

const SocialParentNotificationDeliveryReadinessReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  readinessId: SocialAlertReportReferenceSchema,
  generatedAt: ParentTimestampSchema,
  sourceReportWriterProofRef: SocialAlertReportReferenceSchema,
  rows: Schema.Array(SocialParentNotificationDeliveryReadinessRowSchema).pipe(
    Schema.filter((value) => value.length > 0 || 'Expected social parent notification delivery readiness rows')
  ),
  nonClaims: Schema.Array(SocialParentNotificationDeliveryReadinessNonClaimSchema),
  parentReportStatusReadyCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  manualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  unavailableCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  parentLocalDeliveryResultCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  parentNotificationUiDeliveryClaimed: Schema.Literal(false),
  externalRuntimeReportDeliveryClaimed: Schema.Literal(false),
  finalPolicyExecutionClaimed: Schema.Literal(false),
  enforcementClaimed: Schema.Literal(false),
});

export const SocialParentNotificationDeliveryReadinessReadModelSchema = withParser(
  SocialParentNotificationDeliveryReadinessReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        socialParentNotificationDeliveryReadinessReadModelIsHonest(readModel) ||
        'Expected parent notification delivery readiness counts and non-claims to match rows'
    )
  )
);

export type SocialParentNotificationDeliveryReadinessState = Infer<
  typeof SocialParentNotificationDeliveryReadinessStateSchema
>;
export type SocialParentNotificationDeliveryReadinessRow = Infer<
  typeof SocialParentNotificationDeliveryReadinessRowSchema
>;
export type SocialParentNotificationDeliveryReadinessReadModel = Infer<
  typeof SocialParentNotificationDeliveryReadinessReadModelSchema
>;
export type SocialParentNotificationDeliveryReadinessOptions = {
  readonly generatedAt: string;
  readonly readinessId: string;
  readonly sourceReportWriterProofRef: string;
};

type SocialParentNotificationDeliveryReadinessRowInput = Infer<
  typeof SocialParentNotificationDeliveryReadinessRowBaseSchema
>;
type SocialParentNotificationDeliveryReadinessReadModelInput = Infer<
  typeof SocialParentNotificationDeliveryReadinessReadModelBaseSchema
>;

export function buildSocialParentNotificationDeliveryReadinessReadModel(
  options: SocialParentNotificationDeliveryReadinessOptions,
  sourceReadModel: SocialReportWriterDeliveryProofReadModel
): SocialParentNotificationDeliveryReadinessReadModel {
  const parsedSource = SocialReportWriterDeliveryProofReadModelSchema.parse(sourceReadModel);
  const rows = parsedSource.reportWriterDeliveryRows.map((row) =>
    socialParentNotificationDeliveryReadinessRowFromReportWriter(row, options)
  );

  return SocialParentNotificationDeliveryReadinessReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    readinessId: options.readinessId,
    generatedAt: options.generatedAt,
    sourceReportWriterProofRef: options.sourceReportWriterProofRef,
    rows,
    nonClaims: RequiredNonClaims,
    parentReportStatusReadyCount: countRows(
      rows,
      SocialParentNotificationDeliveryReadinessState.ParentReportStatusReady
    ),
    manualRequiredCount: countRows(rows, SocialParentNotificationDeliveryReadinessState.ManualRequired),
    unavailableCount: countRows(rows, SocialParentNotificationDeliveryReadinessState.Unavailable),
    parentLocalDeliveryResultCount: rows.filter((row) => row.parentLocalDeliveryResultRecorded).length,
    parentNotificationUiDeliveryClaimed: false,
    externalRuntimeReportDeliveryClaimed: false,
    finalPolicyExecutionClaimed: false,
    enforcementClaimed: false,
  });
}

export function summarizeSocialParentNotificationDeliveryReadiness(
  readModel: SocialParentNotificationDeliveryReadinessReadModel
) {
  return {
    totalRows: readModel.rows.length,
    parentReportStatusReadyCount: readModel.parentReportStatusReadyCount,
    manualRequiredCount: readModel.manualRequiredCount,
    unavailableCount: readModel.unavailableCount,
    parentLocalDeliveryResultCount: readModel.parentLocalDeliveryResultCount,
    parentNotificationUiDeliveryClaimed: readModel.parentNotificationUiDeliveryClaimed,
    externalRuntimeReportDeliveryClaimed: readModel.externalRuntimeReportDeliveryClaimed,
    finalPolicyExecutionClaimed: readModel.finalPolicyExecutionClaimed,
    enforcementClaimed: readModel.enforcementClaimed,
  };
}

function socialParentNotificationDeliveryReadinessRowFromReportWriter(
  row: SocialReportWriterDeliveryRow,
  options: SocialParentNotificationDeliveryReadinessOptions
): SocialParentNotificationDeliveryReadinessRow {
  const state = notificationDeliveryReadinessStateForReportWriterRow(row);
  const reportDeliveryExecutionState = reportDeliveryExecutionStateFor(state);
  const manualProofRequirements = manualProofRequirementsFor(row, state);

  return SocialParentNotificationDeliveryReadinessRowSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    notificationDeliveryReadinessRowId: `social-parent-notification-delivery-${row.reportWriterDeliveryRowId}`,
    sourceReportWriterDeliveryRowRef: row.reportWriterDeliveryRowId,
    sourceIntentRef: row.sourceIntentRef,
    parentVisibleReportStatusRef: row.parentVisibleReportStatusRef,
    parentNotificationUiRef: null,
    parentLocalDeliveryResultRef:
      state === SocialParentNotificationDeliveryReadinessState.ParentReportStatusReady
        ? `social-parent-local-delivery-result-${row.reportWriterDeliveryRowId}`
        : null,
    parentReportRef: row.parentReportRef,
    reportArtifactRef: row.reportArtifactRef,
    reportReceiptRef: row.reportReceiptRef,
    sourceEvidenceRefs: row.sourceEvidenceRefs,
    sourcePolicyRefs: row.sourcePolicyRefs,
    sourceAuditRefs: row.sourceAuditRefs,
    manualProofRequirements,
    notificationDeliveryReadinessState: state,
    reportDeliveryExecutionState,
    parentLocalDeliveryResultRecorded: state === SocialParentNotificationDeliveryReadinessState.ParentReportStatusReady,
    parentOwnedReportArtifactWritten: row.parentOwnedReportArtifactWritten,
    parentOwnedReportReceiptRecorded: row.parentOwnedReportReceiptRecorded,
    parentNotificationUiDelivered: false,
    externalRuntimeReportDeliveryClaimed: false,
    providerDeliveryAttempted: false,
    providerReceiptIngested: false,
    finalPolicyDecisionClaimed: false,
    enforcementClaimed: false,
    createdAt: options.generatedAt,
  });
}

function notificationDeliveryReadinessStateForReportWriterRow(
  row: SocialReportWriterDeliveryRow
): SocialParentNotificationDeliveryReadinessState {
  if (row.reportWriterDeliveryState === SocialReportWriterDeliveryState.Unavailable) {
    return SocialParentNotificationDeliveryReadinessState.Unavailable;
  }
  if (
    row.reportWriterDeliveryState === SocialReportWriterDeliveryState.ReportDeliveryReady &&
    row.reportWriterReceiptState === SocialReportWriterReceiptState.ParentOwnedReceiptRecorded
  ) {
    return SocialParentNotificationDeliveryReadinessState.ParentReportStatusReady;
  }
  return SocialParentNotificationDeliveryReadinessState.ManualRequired;
}

function reportDeliveryExecutionStateFor(
  state: SocialParentNotificationDeliveryReadinessState
): Infer<typeof SocialParentNotificationReportDeliveryExecutionStateSchema> {
  if (state === SocialParentNotificationDeliveryReadinessState.ParentReportStatusReady) {
    return SocialParentNotificationReportDeliveryExecutionState.ParentOwnedReportReady;
  }
  if (state === SocialParentNotificationDeliveryReadinessState.Unavailable) {
    return SocialParentNotificationReportDeliveryExecutionState.Unavailable;
  }
  return SocialParentNotificationReportDeliveryExecutionState.ManualRequired;
}

function manualProofRequirementsFor(
  row: SocialReportWriterDeliveryRow,
  state: SocialParentNotificationDeliveryReadinessState
): ReadonlyArray<Infer<typeof SocialAlertReportReferenceSchema>> {
  if (state === SocialParentNotificationDeliveryReadinessState.ParentReportStatusReady) {
    return [];
  }
  return [
    ...row.manualProofRequirements,
    SocialAlertReportReferenceSchema.parse(
      `social-parent-notification-ui-delivery-runtime-required-${row.sourceIntentRef}`
    ),
  ];
}

function socialParentNotificationDeliveryReadinessRowIsHonest(
  row: SocialParentNotificationDeliveryReadinessRowInput
): boolean {
  if (
    row.notificationDeliveryReadinessState === SocialParentNotificationDeliveryReadinessState.ParentReportStatusReady
  ) {
    return reportStatusReadyRowIsHonest(row);
  }
  if (row.notificationDeliveryReadinessState === SocialParentNotificationDeliveryReadinessState.Unavailable) {
    return unavailableRowIsHonest(row);
  }
  return manualRequiredRowIsHonest(row);
}

function reportStatusReadyRowIsHonest(row: SocialParentNotificationDeliveryReadinessRowInput): boolean {
  return (
    row.parentVisibleReportStatusRef !== null &&
    row.parentNotificationUiRef === null &&
    row.parentLocalDeliveryResultRef !== null &&
    row.parentReportRef !== null &&
    row.reportArtifactRef !== null &&
    row.reportReceiptRef !== null &&
    row.parentOwnedReportArtifactWritten &&
    row.parentOwnedReportReceiptRecorded &&
    row.parentLocalDeliveryResultRecorded &&
    row.reportDeliveryExecutionState === SocialParentNotificationReportDeliveryExecutionState.ParentOwnedReportReady &&
    row.manualProofRequirements.length === 0 &&
    rowClaimsStayFalse(row)
  );
}

function manualRequiredRowIsHonest(row: SocialParentNotificationDeliveryReadinessRowInput): boolean {
  return (
    row.parentNotificationUiRef === null &&
    row.parentLocalDeliveryResultRef === null &&
    !row.parentLocalDeliveryResultRecorded &&
    row.reportDeliveryExecutionState === SocialParentNotificationReportDeliveryExecutionState.ManualRequired &&
    row.manualProofRequirements.length > 0 &&
    rowClaimsStayFalse(row)
  );
}

function unavailableRowIsHonest(row: SocialParentNotificationDeliveryReadinessRowInput): boolean {
  return (
    row.parentNotificationUiRef === null &&
    row.parentLocalDeliveryResultRef === null &&
    !row.parentLocalDeliveryResultRecorded &&
    row.reportDeliveryExecutionState === SocialParentNotificationReportDeliveryExecutionState.Unavailable &&
    row.parentReportRef === null &&
    row.reportArtifactRef === null &&
    row.reportReceiptRef === null &&
    row.manualProofRequirements.length > 0 &&
    rowClaimsStayFalse(row)
  );
}

function rowClaimsStayFalse(row: SocialParentNotificationDeliveryReadinessRowInput): boolean {
  return [
    row.parentNotificationUiDelivered,
    row.externalRuntimeReportDeliveryClaimed,
    row.providerDeliveryAttempted,
    row.providerReceiptIngested,
    row.finalPolicyDecisionClaimed,
    row.enforcementClaimed,
  ].every((claim) => claim === false);
}

function socialParentNotificationDeliveryReadinessReadModelIsHonest(
  readModel: SocialParentNotificationDeliveryReadinessReadModelInput
): boolean {
  return (
    readModel.parentReportStatusReadyCount ===
      countRows(readModel.rows, SocialParentNotificationDeliveryReadinessState.ParentReportStatusReady) &&
    readModel.manualRequiredCount ===
      countRows(readModel.rows, SocialParentNotificationDeliveryReadinessState.ManualRequired) &&
    readModel.unavailableCount ===
      countRows(readModel.rows, SocialParentNotificationDeliveryReadinessState.Unavailable) &&
    readModel.parentLocalDeliveryResultCount ===
      readModel.rows.filter((row) => row.parentLocalDeliveryResultRecorded).length &&
    RequiredNonClaims.every((claim) => readModel.nonClaims.includes(claim)) &&
    !readModel.parentNotificationUiDeliveryClaimed &&
    !readModel.externalRuntimeReportDeliveryClaimed &&
    !readModel.finalPolicyExecutionClaimed &&
    !readModel.enforcementClaimed
  );
}

function countRows(
  rows: ReadonlyArray<{ readonly notificationDeliveryReadinessState: SocialParentNotificationDeliveryReadinessState }>,
  state: SocialParentNotificationDeliveryReadinessState
): number {
  return rows.filter((row) => row.notificationDeliveryReadinessState === state).length;
}
