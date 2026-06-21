import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from './effect';
import { ParentTimestampSchema } from './family-reference-primitives';
import { buildTrackingAiProviderRoutingProofRows } from './tracking-ai-provider-routing-proof';
import {
  TrackingEvidenceTraceSchema,
  TrackingLocationAiAnalysisInputSchema,
  TrackingPolicySchemaVersion,
} from './tracking-location-policy';
import { TrackingPolicyAuditRefSchema, TrackingPolicyReasonCodeSchema } from './tracking-location-policy-primitives';

export const TrackingAiStoredRefConsumerRowIdSchema = brandedNonEmptyStringSchema('TrackingAiStoredRefConsumerRowId');

export const TrackingAiStoredRefConsumerProofRefSchema = brandedNonEmptyStringSchema('TrackingAiStoredRefConsumerProofRef');

export const TrackingAiStoredRefConsumerJournalRefSchema = brandedNonEmptyStringSchema('TrackingAiStoredRefConsumerJournalRef');

export const TrackingAiStoredRefConsumerReadModelRefSchema = brandedNonEmptyStringSchema('TrackingAiStoredRefConsumerReadModelRef');

export const TrackingAiStoredRefConsumerKindSchema = withParser(
  Schema.Literal('ai-parent-report-context', 'ai-policy-drill-in-context', 'ai-metadata-fallback-context')
);

export const TrackingAiStoredRefConsumerStateSchema = withParser(
  Schema.Literal('stored-ref-consumer-ready', 'manual-required')
);

export const TrackingAiStoredRefConsumerTierSchema = withParser(
  Schema.Literal('P1_FIXTURE_SIMULATION', 'P2_HOSTED_CI', 'P3_LOCAL_DEV_MACHINE')
);

export const TrackingAiStoredRefConsumerRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    rowId: TrackingAiStoredRefConsumerRowIdSchema,
    consumerKind: TrackingAiStoredRefConsumerKindSchema,
    readinessState: TrackingAiStoredRefConsumerStateSchema,
    requiredProofTier: TrackingAiStoredRefConsumerTierSchema,
    currentProofTier: TrackingAiStoredRefConsumerTierSchema,
    generatedAt: ParentTimestampSchema,
    sourceProofRefs: Schema.Array(TrackingAiStoredRefConsumerProofRefSchema),
    aiProviderRouteProofRefs: Schema.Array(TrackingAiStoredRefConsumerProofRefSchema),
    reportPolicyConsumerProofRefs: Schema.Array(TrackingAiStoredRefConsumerProofRefSchema),
    storedJournalRefs: Schema.Array(TrackingAiStoredRefConsumerJournalRefSchema),
    storedReadModelRowRefs: Schema.Array(TrackingAiStoredRefConsumerReadModelRefSchema),
    analysisInput: TrackingLocationAiAnalysisInputSchema,
    evidenceReferences: Schema.Array(TrackingEvidenceTraceSchema),
    reasonCodes: Schema.Array(TrackingPolicyReasonCodeSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    aiStoredRefConsumerClaimed: Schema.Literal(true),
    modelExecutionClaimed: Schema.Literal(false),
    assistantPolicyWriteClaimed: Schema.Literal(false),
    assistantEnforcementClaimed: Schema.Literal(false),
    childDeviceRuntimeClaimed: Schema.Literal(false),
    providerDeliveryClaimed: Schema.Literal(false),
    notificationReceiptClaimed: Schema.Literal(false),
    physicalDeviceClaimed: Schema.Literal(false),
    authorityClaimed: Schema.Literal(false),
    productionBehaviorClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  })
    .pipe(Schema.filter((row) => row.sourceProofRefs.length > 0 || 'Tracking AI consumers need source proof refs'))
    .pipe(
      Schema.filter(
        (row) => row.aiProviderRouteProofRefs.length > 0 || 'Tracking AI consumers need provider route proof refs'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.reportPolicyConsumerProofRefs.length > 0 || 'Tracking AI consumers need report/policy consumer proof refs'
      )
    )
    .pipe(Schema.filter((row) => row.storedJournalRefs.length > 0 || 'Tracking AI consumers need stored journal refs'))
    .pipe(
      Schema.filter(
        (row) => row.storedReadModelRowRefs.length > 0 || 'Tracking AI consumers need stored read-model refs'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.analysisInput.evidenceReferences.length === row.evidenceReferences.length ||
          'Tracking AI analysis inputs must carry the same evidence refs as the consumer row'
      )
    )
);

export const TrackingAiStoredRefConsumerProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-ai-stored-ref-consumer-proof'),
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(TrackingAiStoredRefConsumerRowSchema),
    productClaims: Schema.Struct({
      productClaimReady: Schema.Literal(false),
      modelExecutionClaimed: Schema.Literal(false),
      assistantPolicyWriteClaimed: Schema.Literal(false),
      assistantEnforcementClaimed: Schema.Literal(false),
      childDeviceRuntimeClaimed: Schema.Literal(false),
      providerDeliveryClaimed: Schema.Literal(false),
      notificationReceiptClaimed: Schema.Literal(false),
      physicalDeviceClaimed: Schema.Literal(false),
      authorityClaimed: Schema.Literal(false),
      productionBehaviorClaimed: Schema.Literal(false),
    }),
  }).pipe(
    Schema.filter((proof) => proof.rows.length >= 3 || 'Tracking AI stored-ref consumer proof needs all consumer rows')
  )
);

export type TrackingAiStoredRefConsumerKind = Infer<typeof TrackingAiStoredRefConsumerKindSchema>;
export type TrackingAiStoredRefConsumerRow = Infer<typeof TrackingAiStoredRefConsumerRowSchema>;
export type TrackingAiStoredRefConsumerProof = Infer<typeof TrackingAiStoredRefConsumerProofSchema>;
type TrackingAiStoredRefConsumerEvidence = Infer<typeof TrackingEvidenceTraceSchema>;

export function buildTrackingAiStoredRefConsumerProof(generatedAt: string): TrackingAiStoredRefConsumerProof {
  return TrackingAiStoredRefConsumerProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-ai-stored-ref-consumer-proof',
    generatedAt,
    rows: [
      row({
        rowId: 'tracking-ai-stored-ref-row-parent-report-context',
        consumerKind: 'ai-parent-report-context',
        generatedAt,
        providerRouteMode: 'parent-local',
        evidenceReferences: [
          evidence('tracking-ai-stored-ref-evidence-parent-report-context', 'query-store-summary', generatedAt),
        ],
        storedJournalRefs: ['tracking-journal-row-ai-parent-report-context'],
        storedReadModelRowRefs: ['tracking-read-model-row-ai-parent-report-context'],
        reasonCodes: ['tracking-ai-parent-report-context-stored-ref-ready'],
        auditRefs: ['tracking-ai-stored-ref-audit-parent-report-context'],
      }),
      row({
        rowId: 'tracking-ai-stored-ref-row-policy-drill-in-context',
        consumerKind: 'ai-policy-drill-in-context',
        generatedAt,
        providerRouteMode: 'child-local',
        evidenceReferences: [
          evidence('tracking-ai-stored-ref-evidence-policy-drill-in-context', 'policy-decision', generatedAt),
        ],
        storedJournalRefs: ['tracking-journal-row-ai-policy-drill-in-context'],
        storedReadModelRowRefs: ['tracking-read-model-row-ai-policy-drill-in-context'],
        reasonCodes: ['tracking-ai-policy-drill-in-context-stored-ref-ready'],
        auditRefs: ['tracking-ai-stored-ref-audit-policy-drill-in-context'],
      }),
      row({
        rowId: 'tracking-ai-stored-ref-row-metadata-fallback-context',
        consumerKind: 'ai-metadata-fallback-context',
        generatedAt,
        providerRouteMode: 'metadata-only',
        evidenceReferences: [
          evidence('tracking-ai-stored-ref-evidence-metadata-fallback-context', 'query-store-summary', generatedAt),
        ],
        storedJournalRefs: ['tracking-journal-row-ai-metadata-fallback-context'],
        storedReadModelRowRefs: ['tracking-read-model-row-ai-metadata-fallback-context'],
        reasonCodes: ['tracking-ai-metadata-fallback-context-stored-ref-ready'],
        auditRefs: ['tracking-ai-stored-ref-audit-metadata-fallback-context'],
      }),
    ],
    productClaims: {
      productClaimReady: false,
      modelExecutionClaimed: false,
      assistantPolicyWriteClaimed: false,
      assistantEnforcementClaimed: false,
      childDeviceRuntimeClaimed: false,
      providerDeliveryClaimed: false,
      notificationReceiptClaimed: false,
      physicalDeviceClaimed: false,
      authorityClaimed: false,
      productionBehaviorClaimed: false,
    },
  });
}

function row(input: {
  readonly rowId: string;
  readonly consumerKind: TrackingAiStoredRefConsumerKind;
  readonly generatedAt: string;
  readonly providerRouteMode: TrackingAiStoredRefConsumerRow['analysisInput']['providerRouteId'] extends `${infer Mode}-tracking-ai-route`
    ? Mode
    : string;
  readonly evidenceReferences: readonly TrackingAiStoredRefConsumerEvidence[];
  readonly storedJournalRefs: readonly string[];
  readonly storedReadModelRowRefs: readonly string[];
  readonly reasonCodes: readonly string[];
  readonly auditRefs: readonly string[];
}): TrackingAiStoredRefConsumerRow {
  const routeRow = buildTrackingAiProviderRoutingProofRows().find(
    (entry) => entry.route.mode === input.providerRouteMode
  );
  if (routeRow === undefined) {
    throw new Error(`Missing tracking AI provider route row: ${input.providerRouteMode}`);
  }

  return TrackingAiStoredRefConsumerRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    rowId: input.rowId,
    consumerKind: input.consumerKind,
    readinessState: 'stored-ref-consumer-ready',
    requiredProofTier: 'P2_HOSTED_CI',
    currentProofTier: 'P2_HOSTED_CI',
    generatedAt: input.generatedAt,
    sourceProofRefs: [
      'output/tracking-plan-proof/24-ai-provider-routing/18-ai-provider-routing-custody-proof.json',
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/21-product-surface-summary-proof.json',
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/22-report-policy-consumer-proof.json',
    ],
    aiProviderRouteProofRefs: [
      'output/tracking-plan-proof/24-ai-provider-routing/18-ai-provider-routing-custody-proof.json',
    ],
    reportPolicyConsumerProofRefs: [
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/22-report-policy-consumer-proof.json',
    ],
    storedJournalRefs: input.storedJournalRefs,
    storedReadModelRowRefs: input.storedReadModelRowRefs,
    analysisInput: {
      schemaVersion: TrackingPolicySchemaVersion,
      analysisId: `${input.rowId}-analysis-input`,
      requestedAt: input.generatedAt,
      evidenceReferences: input.evidenceReferences,
      policyVersion: 'tracking-ai-stored-ref-policy-v1',
      providerRouteId: routeRow.route.providerRouteId,
    },
    evidenceReferences: input.evidenceReferences,
    reasonCodes: input.reasonCodes,
    auditRefs: input.auditRefs,
    aiStoredRefConsumerClaimed: true,
    modelExecutionClaimed: false,
    assistantPolicyWriteClaimed: false,
    assistantEnforcementClaimed: false,
    childDeviceRuntimeClaimed: false,
    providerDeliveryClaimed: false,
    notificationReceiptClaimed: false,
    physicalDeviceClaimed: false,
    authorityClaimed: false,
    productionBehaviorClaimed: false,
    productClaimReady: false,
  });
}

function evidence(
  evidenceReferenceId: string,
  kind: TrackingAiStoredRefConsumerEvidence['kind'],
  observedAt: string
): TrackingAiStoredRefConsumerEvidence {
  return TrackingEvidenceTraceSchema.parse({
    evidenceReferenceId,
    kind,
    observedAt,
  });
}

