import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from './family-reference-primitives';
import { FamilyReferenceSchema } from './family-references';
import {
  TrackingNotificationPreferencePreflightReadModelSchema,
  type TrackingNotificationPreferencePreflightReadModel,
  type TrackingNotificationPreferencePreflightRow,
} from './tracking-notification-preference-preflight-proof';
import {
  TrackingNotificationReceiptBoundaryReadModelSchema,
  type TrackingNotificationReceiptBoundaryReadModel,
  type TrackingNotificationReceiptBoundaryRow,
} from './tracking-notification-receipt-boundary-proof';
import {
  TrackingProviderNotificationProofReadModelSchema,
  type TrackingProviderNotificationProofReadModel,
  type TrackingProviderNotificationProofRow,
} from './tracking-provider-notification-proof';

export const TrackingNotificationParentSurfaceHistoryStatus = {
  HistoryIntentReady: 'history-intent-ready',
  ManualActionRequired: 'manual-action-required',
  ProviderUnavailable: 'provider-unavailable',
} as const;

export const RequiredTrackingNotificationParentSurfaceHistoryNonClaims = [
  'no-rendered-parent-notification-ui',
  'no-parent-preference-mutation-runtime',
  'no-parent-frequency-control-ui',
  'no-quiet-hours-timer-runtime',
  'no-provider-delivery-execution',
  'no-provider-receipt-ingestion-runtime',
  'no-provider-credentials',
  'no-cloud-routing',
  'no-child-device-delivery',
  'no-mobile-physical-device-proof',
  'no-authority-proof',
  'no-retry-worker-runtime',
  'no-production-durable-history-storage',
  'no-production-durable-outbox-storage',
  'no-adapter-dispatch',
] as const;

export const TrackingNotificationParentSurfaceHistoryStatusSchema = withParser(
  Schema.Literal(...Object.values(TrackingNotificationParentSurfaceHistoryStatus))
);
export const TrackingNotificationParentSurfaceHistoryNonClaimSchema = withParser(
  Schema.Literal(...RequiredTrackingNotificationParentSurfaceHistoryNonClaims)
);
export const TrackingNotificationParentSurfaceHistoryProofIdSchema = brandedNonEmptyStringSchema(
  'TrackingNotificationParentSurfaceHistoryProofId'
);
export const TrackingNotificationParentSurfaceHistoryReferenceSchema = brandedNonEmptyStringSchema(
  'TrackingNotificationParentSurfaceHistoryReference'
);

const TrackingNotificationParentSurfaceHistoryRowBaseSchema = Schema.Struct({
  historyRowId: TrackingNotificationParentSurfaceHistoryReferenceSchema,
  sourceAlertId: TrackingNotificationParentSurfaceHistoryReferenceSchema,
  sourceProviderNotificationRowId: TrackingNotificationParentSurfaceHistoryReferenceSchema,
  sourceReceiptBoundaryRowId: TrackingNotificationParentSurfaceHistoryReferenceSchema,
  sourcePreferencePreflightRowId: TrackingNotificationParentSurfaceHistoryReferenceSchema,
  status: TrackingNotificationParentSurfaceHistoryStatusSchema,
  sourcePolicyDecisionId: TrackingNotificationParentSurfaceHistoryReferenceSchema,
  evidenceRefs: Schema.Array(TrackingNotificationParentSurfaceHistoryReferenceSchema),
  notificationStatusRefs: Schema.Array(TrackingNotificationParentSurfaceHistoryReferenceSchema),
  reasonCodeRefs: Schema.Array(TrackingNotificationParentSurfaceHistoryReferenceSchema),
  providerStatusEntryRef: TrackingNotificationParentSurfaceHistoryReferenceSchema,
  providerAttemptRef: TrackingNotificationParentSurfaceHistoryReferenceSchema,
  auditRefs: Schema.Array(TrackingNotificationParentSurfaceHistoryReferenceSchema),
  providerPreferenceRefs: Schema.Array(TrackingNotificationParentSurfaceHistoryReferenceSchema),
  parentPreferenceRequirementRefs: Schema.Array(TrackingNotificationParentSurfaceHistoryReferenceSchema),
  quietHoursRequirementRefs: Schema.Array(TrackingNotificationParentSurfaceHistoryReferenceSchema),
  receiptRequirementRefs: Schema.Array(TrackingNotificationParentSurfaceHistoryReferenceSchema),
  manualProofRequirements: Schema.Array(TrackingNotificationParentSurfaceHistoryReferenceSchema),
  drillInRefs: Schema.Array(TrackingNotificationParentSurfaceHistoryReferenceSchema),
  redactedParentSummaryRef: TrackingNotificationParentSurfaceHistoryReferenceSchema,
  renderedParentNotificationUiClaimed: Schema.Literal(false),
  parentPreferenceMutationRuntimeClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  receiptIngestionRuntimeClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  mobilePhysicalDeviceProofClaimed: Schema.Literal(false),
  authorityProofClaimed: Schema.Literal(false),
});

export const TrackingNotificationParentSurfaceHistoryRowSchema = withParser(
  TrackingNotificationParentSurfaceHistoryRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        trackingNotificationParentSurfaceHistoryRowIsHonest(row) ||
        'Expected tracking notification parent-surface history rows to preserve provider, receipt, preference, evidence, and policy refs without claiming UI or delivery runtime'
    )
  )
);

const TrackingNotificationParentSurfaceHistoryReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofId: TrackingNotificationParentSurfaceHistoryProofIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  sourceProviderNotificationProofRef: TrackingNotificationParentSurfaceHistoryReferenceSchema,
  sourceReceiptBoundaryProofRef: TrackingNotificationParentSurfaceHistoryReferenceSchema,
  sourcePreferencePreflightProofRef: TrackingNotificationParentSurfaceHistoryReferenceSchema,
  sourceContractRefs: Schema.Array(TrackingNotificationParentSurfaceHistoryReferenceSchema),
  rows: Schema.Array(TrackingNotificationParentSurfaceHistoryRowSchema),
  historyIntentReadyCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  manualActionRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  providerUnavailableCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  proofNonClaims: Schema.Array(TrackingNotificationParentSurfaceHistoryNonClaimSchema),
  renderedParentNotificationUiClaimed: Schema.Literal(false),
  parentPreferenceMutationRuntimeClaimed: Schema.Literal(false),
  parentFrequencyControlUiClaimed: Schema.Literal(false),
  quietHoursTimerRuntimeClaimed: Schema.Literal(false),
  providerDeliveryRuntimeClaimed: Schema.Literal(false),
  providerReceiptIngestionRuntimeClaimed: Schema.Literal(false),
  providerCredentialsClaimed: Schema.Literal(false),
  cloudRoutingClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  mobilePhysicalDeviceProofClaimed: Schema.Literal(false),
  authorityProofClaimed: Schema.Literal(false),
  retryExecutionRuntimeClaimed: Schema.Literal(false),
  productionDurableHistoryStorageClaimed: Schema.Literal(false),
  productionDurableOutboxStorageClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
});

export const TrackingNotificationParentSurfaceHistoryReadModelSchema = withParser(
  TrackingNotificationParentSurfaceHistoryReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        trackingNotificationParentSurfaceHistoryReadModelIsHonest(readModel) ||
        'Expected tracking notification parent-surface history counts and non-claims to match source proof rows'
    )
  )
);

export type TrackingNotificationParentSurfaceHistoryStatus = Infer<
  typeof TrackingNotificationParentSurfaceHistoryStatusSchema
>;
export type TrackingNotificationParentSurfaceHistoryRow = Infer<
  typeof TrackingNotificationParentSurfaceHistoryRowSchema
>;
export type TrackingNotificationParentSurfaceHistoryReadModel = Infer<
  typeof TrackingNotificationParentSurfaceHistoryReadModelSchema
>;

export type TrackingNotificationParentSurfaceHistoryProofOptions = {
  readonly generatedAt: string;
  readonly proofId: string;
  readonly sourceContractRefs: readonly string[];
};

type HistoryRowInput = Infer<typeof TrackingNotificationParentSurfaceHistoryRowBaseSchema>;
type HistoryReadModelInput = Infer<typeof TrackingNotificationParentSurfaceHistoryReadModelBaseSchema>;

export function buildTrackingNotificationParentSurfaceHistoryReadModel(
  options: TrackingNotificationParentSurfaceHistoryProofOptions,
  providerProof: TrackingProviderNotificationProofReadModel,
  receiptProof: TrackingNotificationReceiptBoundaryReadModel,
  preferenceProof: TrackingNotificationPreferencePreflightReadModel
): TrackingNotificationParentSurfaceHistoryReadModel {
  const parsedProviderProof = TrackingProviderNotificationProofReadModelSchema.parse(providerProof);
  const parsedReceiptProof = TrackingNotificationReceiptBoundaryReadModelSchema.parse(receiptProof);
  const parsedPreferenceProof = TrackingNotificationPreferencePreflightReadModelSchema.parse(preferenceProof);
  const receiptRowsByAlert = rowsByAlert(parsedReceiptProof.rows, (row) => row.sourceAlertId);
  const preferenceRowsByAlert = rowsByAlert(parsedPreferenceProof.rows, (row) => row.sourceAlertId);
  const rows = parsedProviderProof.rows.map((providerRow) =>
    trackingNotificationParentSurfaceHistoryRowForSources(
      providerRow,
      requiredRow(receiptRowsByAlert, providerRow.sourceAlertId, 'receipt boundary'),
      requiredRow(preferenceRowsByAlert, providerRow.sourceAlertId, 'preference preflight')
    )
  );

  return TrackingNotificationParentSurfaceHistoryReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    proofId: options.proofId,
    generatedAt: options.generatedAt,
    family: parsedProviderProof.family,
    sourceProviderNotificationProofRef: parsedProviderProof.proofId,
    sourceReceiptBoundaryProofRef: parsedReceiptProof.proofId,
    sourcePreferencePreflightProofRef: parsedPreferenceProof.preferencePreflightId,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    historyIntentReadyCount: countHistoryStatus(
      rows,
      TrackingNotificationParentSurfaceHistoryStatus.HistoryIntentReady
    ),
    manualActionRequiredCount: countHistoryStatus(
      rows,
      TrackingNotificationParentSurfaceHistoryStatus.ManualActionRequired
    ),
    providerUnavailableCount: countHistoryStatus(
      rows,
      TrackingNotificationParentSurfaceHistoryStatus.ProviderUnavailable
    ),
    proofNonClaims: RequiredTrackingNotificationParentSurfaceHistoryNonClaims,
    renderedParentNotificationUiClaimed: false,
    parentPreferenceMutationRuntimeClaimed: false,
    parentFrequencyControlUiClaimed: false,
    quietHoursTimerRuntimeClaimed: false,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionRuntimeClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    childDeviceDeliveryClaimed: false,
    mobilePhysicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
    retryExecutionRuntimeClaimed: false,
    productionDurableHistoryStorageClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
  });
}

function trackingNotificationParentSurfaceHistoryRowForSources(
  providerRow: TrackingProviderNotificationProofRow,
  receiptRow: TrackingNotificationReceiptBoundaryRow,
  preferenceRow: TrackingNotificationPreferencePreflightRow
): TrackingNotificationParentSurfaceHistoryRow {
  const status = historyStatusFor(providerRow);
  const manualProofRequirements = uniqueRefs([
    ...providerRow.manualProofRequirements,
    ...receiptRow.manualProofRequirements,
    ...receiptRow.receiptIngestionProofRequirements,
    ...preferenceRow.manualProofRequirements,
  ]);

  return TrackingNotificationParentSurfaceHistoryRowSchema.parse({
    historyRowId: `tracking-notification-history-${providerRow.sourceAlertId}`,
    sourceAlertId: providerRow.sourceAlertId,
    sourceProviderNotificationRowId: providerRow.rowId,
    sourceReceiptBoundaryRowId: receiptRow.rowId,
    sourcePreferencePreflightRowId: preferenceRow.preferenceRowId,
    status,
    sourcePolicyDecisionId: providerRow.sourcePolicyDecisionId,
    evidenceRefs: providerRow.evidenceRefs,
    notificationStatusRefs: providerRow.notificationStatusRefs,
    reasonCodeRefs: providerRow.reasonCodeRefs,
    providerStatusEntryRef: providerRow.providerStatusBoundaryEntry.statusEntryId,
    providerAttemptRef: providerRow.providerStatusBoundaryEntry.providerAttemptRef,
    auditRefs: receiptRow.auditRefs,
    providerPreferenceRefs: preferenceRow.providerPreferenceRefs,
    parentPreferenceRequirementRefs: preferenceRow.parentPreferenceRequirementRefs,
    quietHoursRequirementRefs: preferenceRow.quietHoursRequirementRefs,
    receiptRequirementRefs: receiptRow.receiptIngestionProofRequirements,
    manualProofRequirements,
    drillInRefs: [
      `tracking-notification-history-drill-in-${providerRow.sourceAlertId}`,
      providerRow.sourcePolicyDecisionId,
      ...providerRow.evidenceRefs,
    ],
    redactedParentSummaryRef: `tracking-notification-redacted-summary-${providerRow.sourceAlertId}`,
    renderedParentNotificationUiClaimed: false,
    parentPreferenceMutationRuntimeClaimed: false,
    providerDeliveryClaimed: false,
    receiptIngestionRuntimeClaimed: false,
    childDeviceDeliveryClaimed: false,
    mobilePhysicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
  });
}

function historyStatusFor(row: TrackingProviderNotificationProofRow): TrackingNotificationParentSurfaceHistoryStatus {
  if (row.providerStatusKind === 'provider-adapter-required') {
    return TrackingNotificationParentSurfaceHistoryStatus.HistoryIntentReady;
  }
  return row.providerStatusKind === 'unavailable'
    ? TrackingNotificationParentSurfaceHistoryStatus.ProviderUnavailable
    : TrackingNotificationParentSurfaceHistoryStatus.ManualActionRequired;
}

function rowsByAlert<Row>(rows: readonly Row[], alertRefForRow: (row: Row) => string): ReadonlyMap<string, Row> {
  return new Map(rows.map((row) => [alertRefForRow(row), row]));
}

function requiredRow<Row>(rows: ReadonlyMap<string, Row>, sourceAlertId: string, label: string): Row {
  const row = rows.get(sourceAlertId);
  if (row === undefined) {
    throw new Error(`Missing ${label} row for ${sourceAlertId}`);
  }
  return row;
}

function trackingNotificationParentSurfaceHistoryRowIsHonest(row: HistoryRowInput): boolean {
  const refsPresent =
    row.evidenceRefs.length > 0 &&
    row.reasonCodeRefs.length > 0 &&
    row.auditRefs.length > 0 &&
    row.manualProofRequirements.length > 0 &&
    row.drillInRefs.length > 0;
  return refsPresent && rowHistoryClaimsStayFalse(row);
}

function rowHistoryClaimsStayFalse(row: HistoryRowInput): boolean {
  return [
    row.renderedParentNotificationUiClaimed,
    row.parentPreferenceMutationRuntimeClaimed,
    row.providerDeliveryClaimed,
    row.receiptIngestionRuntimeClaimed,
    row.childDeviceDeliveryClaimed,
    row.mobilePhysicalDeviceProofClaimed,
    row.authorityProofClaimed,
  ].every((claim) => claim === false);
}

function trackingNotificationParentSurfaceHistoryReadModelIsHonest(readModel: HistoryReadModelInput): boolean {
  return (
    readModel.historyIntentReadyCount ===
      countHistoryStatus(readModel.rows, TrackingNotificationParentSurfaceHistoryStatus.HistoryIntentReady) &&
    readModel.manualActionRequiredCount ===
      countHistoryStatus(readModel.rows, TrackingNotificationParentSurfaceHistoryStatus.ManualActionRequired) &&
    readModel.providerUnavailableCount ===
      countHistoryStatus(readModel.rows, TrackingNotificationParentSurfaceHistoryStatus.ProviderUnavailable) &&
    RequiredTrackingNotificationParentSurfaceHistoryNonClaims.every((claim) => readModel.proofNonClaims.includes(claim))
  );
}

function countHistoryStatus(
  rows: ReadonlyArray<{ readonly status: TrackingNotificationParentSurfaceHistoryStatus }>,
  status: TrackingNotificationParentSurfaceHistoryStatus
): number {
  return rows.filter((row) => row.status === status).length;
}

function uniqueRefs(refs: readonly string[]): readonly string[] {
  return [...new Set(refs)];
}
