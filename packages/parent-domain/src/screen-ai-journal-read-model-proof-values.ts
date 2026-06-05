import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  LocalAiConfidenceSchema,
  LocalAiModelIdSchema,
  LocalAiPromptVersionSchema,
  LocalAiRuntimeReferenceIdSchema,
} from './local-ai-primitives';
import {
  PolicyActionSchema,
  PolicyDecisionHandoffStateSchema,
  PolicyDecisionIdSchema,
  PolicyReasonCodeSchema,
  PolicyRuleIdSchema,
} from './policy';
import { ParentEvidenceReferenceIdSchema } from './reference-primitives';

export const ScreenAiJournalTextSchema = Schema.String.pipe(Schema.minLength(1));
export const ScreenAiJournalNonNegativeIntegerSchema = Schema.Number.pipe(Schema.int(), Schema.greaterThanOrEqualTo(0));
export const ScreenAiJournalReadModelRowIdSchema = ScreenAiJournalTextSchema.pipe(
  Schema.brand('ScreenAiJournalReadModelRowId')
);
export const ScreenAiJournalReadModelSnapshotIdSchema = ScreenAiJournalTextSchema.pipe(
  Schema.brand('ScreenAiJournalReadModelSnapshotId')
);
export const ScreenAiJournalEntryIdSchema = ScreenAiJournalTextSchema.pipe(Schema.brand('ScreenAiJournalEntryId'));
export const ScreenAiSqliteRowIdSchema = ScreenAiJournalTextSchema.pipe(Schema.brand('ScreenAiSqliteRowId'));
export const ScreenAiSourceAnalysisRowIdSchema = ScreenAiJournalTextSchema.pipe(
  Schema.brand('ScreenAiSourceAnalysisRowId')
);
export const ScreenAiQueueJobIdSchema = ScreenAiJournalTextSchema.pipe(Schema.brand('ScreenAiQueueJobId'));
export const ScreenAiLocalAiResultIdSchema = ScreenAiJournalTextSchema.pipe(Schema.brand('ScreenAiLocalAiResultId'));
export const ScreenAiImageDigestSchema = ScreenAiJournalTextSchema.pipe(Schema.brand('ScreenAiImageDigest'));
export const ScreenAiCategorySchema = ScreenAiJournalTextSchema.pipe(Schema.brand('ScreenAiCategory'));
export const ScreenAiReadModelEvidenceRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema).pipe(
  Schema.filter((refs) => refs.length > 0 || 'Expected screen AI evidence refs')
);
export const ScreenAiReadModelPolicyReasonCodesSchema = Schema.Array(PolicyReasonCodeSchema).pipe(
  Schema.filter((refs) => refs.length > 0 || 'Expected screen AI policy reason codes')
);
export const ScreenAiReadModelPolicyRuleRefsSchema = Schema.Array(PolicyRuleIdSchema).pipe(
  Schema.filter((refs) => refs.length > 0 || 'Expected screen AI policy rule refs')
);

export const ScreenAiJournalReadModelClaimBoundarySchema = withParser(
  Schema.Struct({
    rawImageRetained: Schema.Literal(false),
    remoteAiUsed: Schema.Literal(false),
    apiAiUsed: Schema.Literal(false),
    policyAuthorityClaimed: Schema.Literal(false),
    portalRuntimeClaimed: Schema.Literal(false),
    enforcementClaimed: Schema.Literal(false),
    runtimeSqliteWriterClaimed: Schema.Literal(false),
  })
);

export const ScreenAiJournalReadModelSourceRowBaseSchema = Schema.Struct({
  analysisRowId: ScreenAiSourceAnalysisRowIdSchema,
  queueJobId: ScreenAiQueueJobIdSchema,
  localAiResultId: ScreenAiLocalAiResultIdSchema,
  modelRuntimeRef: LocalAiRuntimeReferenceIdSchema,
  modelId: LocalAiModelIdSchema,
  promptOrTemplateVersion: LocalAiPromptVersionSchema,
  primaryCategory: ScreenAiCategorySchema,
  confidence: LocalAiConfidenceSchema,
  imageDigest: ScreenAiImageDigestSchema,
  imageDeletionState: Schema.Literal('deleted'),
  rawImageRetained: Schema.Literal(false),
  custodyState: Schema.Literal('child-device-journal'),
  evidenceReferenceIds: ScreenAiReadModelEvidenceRefsSchema,
  policyDecisionRef: PolicyDecisionIdSchema,
  policyAction: PolicyActionSchema,
  policyReasonCodes: ScreenAiReadModelPolicyReasonCodesSchema,
  policyDryRun: Schema.Literal(true),
  enforcementHandoffState: PolicyDecisionHandoffStateSchema,
  parentRuleRefs: ScreenAiReadModelPolicyRuleRefsSchema,
  readModelRowId: ScreenAiSourceAnalysisRowIdSchema,
  readModelRawImageRetained: Schema.Literal(false),
  readModelImageDeletionState: Schema.Literal('deleted'),
});

export type ScreenAiJournalReadModelSourceRowCandidate = Infer<typeof ScreenAiJournalReadModelSourceRowBaseSchema>;
