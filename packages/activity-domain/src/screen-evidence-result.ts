import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceRefSchema } from './contracts';
import { ActivityTimestampSchema } from './primitives';
import {
  ScreenCaptureReasonSchema,
  ScreenCaptureScopeSchema,
  ScreenCapabilityStatusSchema,
  ScreenDeletionStateSchema,
  ScreenEvidenceCustodyStateSchema,
  ScreenLocalModelProviderKindSchema,
  ScreenRedactionNoteSchema,
  ScreenRiskSignalSchema,
  ScreenUncertaintyReasonSchema,
  ScreenVisibleCategorySchema,
} from './screen-evidence-states';
import {
  ScreenEvidenceConfidenceSchema,
  ScreenEvidenceImageDigestSchema,
  ScreenEvidenceModelIdSchema,
  ScreenEvidenceModelRuntimeRefSchema,
  ScreenEvidenceOcrSnippetTextSchema,
  ScreenEvidenceQueueJobIdSchema,
  ScreenEvidenceResultIdSchema,
  ScreenEvidenceSchemaVersion,
  ScreenEvidenceSummaryTextSchema,
  ScreenEvidenceTemplateVersionSchema,
} from './screen-evidence-primitives';

const RequiredFalse = Schema.Literal(false);
const PolicyEligibleConfidenceFloor = 0.5;

export const ScreenCategoryCandidateSchema = withParser(
  Schema.Struct({
    category: ScreenVisibleCategorySchema,
    confidence: ScreenEvidenceConfidenceSchema,
    evidenceRefs: Schema.Array(ActivityEvidenceRefSchema),
  })
);

export const ScreenRiskSignalCandidateSchema = withParser(
  Schema.Struct({
    signal: ScreenRiskSignalSchema,
    confidence: ScreenEvidenceConfidenceSchema,
    evidenceRefs: Schema.Array(ActivityEvidenceRefSchema),
  })
);

export const ScreenOcrTextSnippetSchema = withParser(
  Schema.Struct({
    text: ScreenEvidenceOcrSnippetTextSchema,
    confidence: ScreenEvidenceConfidenceSchema,
    evidenceRefs: Schema.Array(ActivityEvidenceRefSchema),
  })
);

const ScreenAnalysisResultBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenEvidenceSchemaVersion),
  screenAnalysisResultId: ScreenEvidenceResultIdSchema,
  queueJobId: ScreenEvidenceQueueJobIdSchema,
  analyzedAt: ActivityTimestampSchema,
  modelRuntimeRef: ScreenEvidenceModelRuntimeRefSchema,
  modelId: ScreenEvidenceModelIdSchema,
  providerKind: ScreenLocalModelProviderKindSchema,
  promptOrTemplateVersion: ScreenEvidenceTemplateVersionSchema,
  captureReason: ScreenCaptureReasonSchema,
  captureScope: ScreenCaptureScopeSchema,
  capabilityStatus: ScreenCapabilityStatusSchema,
  summary: ScreenEvidenceSummaryTextSchema,
  visibleCategoryCandidates: Schema.Array(ScreenCategoryCandidateSchema),
  primaryCategory: Schema.Union(ScreenVisibleCategorySchema, Schema.Null),
  riskSignals: Schema.Array(ScreenRiskSignalCandidateSchema),
  ocrTextSnippets: Schema.Array(ScreenOcrTextSnippetSchema),
  redactionNotes: Schema.Array(ScreenRedactionNoteSchema),
  confidence: ScreenEvidenceConfidenceSchema,
  uncertaintyReason: Schema.Union(ScreenUncertaintyReasonSchema, Schema.Null),
  sourceEvidenceRefs: Schema.Array(ActivityEvidenceRefSchema),
  imageDigest: ScreenEvidenceImageDigestSchema,
  rawImageRetained: RequiredFalse,
  imageDeletionState: ScreenDeletionStateSchema,
  custodyState: ScreenEvidenceCustodyStateSchema,
  policyEligible: Schema.Boolean,
});

export const ScreenAnalysisResultSchema = withParser(
  ScreenAnalysisResultBaseSchema.pipe(
    Schema.filter(
      (value) =>
        !value.policyEligible ||
        (value.sourceEvidenceRefs.length > 0 &&
          value.capabilityStatus === 'ready' &&
          value.confidence >= PolicyEligibleConfidenceFloor &&
          value.primaryCategory !== null &&
          value.primaryCategory !== 'unknown' &&
          value.rawImageRetained === false &&
          (value.imageDeletionState === 'deleted' || value.imageDeletionState === 'expiredDeleted')) ||
        'Expected policy-eligible screen analysis to use ready local evidence, confidence, category, and deleted raw image custody'
    ),
    Schema.filter(
      (value) =>
        (value.primaryCategory !== null && value.primaryCategory !== 'unknown') ||
        (value.uncertaintyReason !== null && !value.policyEligible) ||
        'Expected unknown screen analysis summaries to carry uncertainty and stay policy-ineligible'
    )
  )
);

export type ScreenCategoryCandidate = Infer<typeof ScreenCategoryCandidateSchema>;
export type ScreenRiskSignalCandidate = Infer<typeof ScreenRiskSignalCandidateSchema>;
export type ScreenOcrTextSnippet = Infer<typeof ScreenOcrTextSnippetSchema>;
export type ScreenAnalysisResult = Infer<typeof ScreenAnalysisResultSchema>;
