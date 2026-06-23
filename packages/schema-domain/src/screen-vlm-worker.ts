import { type Infer, Schema, withParser, NonEmptyStringSchema } from './effect';
import { ActivityEvidenceRefSchema } from './evidence-contracts';
import { ActivityTimestampSchema } from './evidence-primitives';
import {
  ScreenEvidenceConfidenceSchema,
  ScreenEvidenceImageDigestSchema,
  ScreenEvidenceModelIdSchema,
  ScreenEvidenceModelRuntimeRefSchema,
  ScreenEvidenceQueueJobIdSchema,
  ScreenEvidenceResultIdSchema,
  ScreenEvidenceSchemaVersion,
  ScreenEvidenceSummaryTextSchema,
  ScreenEvidenceTemplateVersionSchema,
} from './screen-evidence-primitives';
import { ScreenAnalysisResultSchema, ScreenLocalModelOutputSchema } from './screen-evidence-result';
import {
  ScreenCapabilityStatusSchema,
  ScreenCaptureReasonSchema,
  ScreenCaptureScopeSchema,
  ScreenDeletionStateSchema,
  ScreenEvidenceCustodyStateSchema,
  ScreenRedactionNoteSchema,
  ScreenRiskSignalSchema,
  ScreenUncertaintyReasonSchema,
  ScreenVisibleCategorySchema,
} from './screen-evidence-states';

export const ScreenVlmWorkerSchemaVersion = 1;
export const ScreenVlmWorkerTemplateVersion = 'screen-vlm-worker-guided-v1';
export const ScreenVlmWorkerModelId = 'screen-local-vlm-safety-model';
export const ScreenVlmWorkerRuntimeRef = 'screen-local-vlm-runtime';
export const ScreenVlmWorkerMaxPromptCharacters = 1200;
export const ScreenVlmWorkerMaxImagePixels = 2073600;
export const ScreenVlmWorkerRejectedOpenEndedPromptTerms = ['describe the screen', 'describe this screen'] as const;

const RequiredFalse = Schema.Literal(false);
const RequiredTrue = Schema.Literal(true);
const PositiveInteger = Schema.Number.pipe(Schema.int(), Schema.positive());
const PromptText = NonEmptyStringSchema.pipe(Schema.maxLength(ScreenVlmWorkerMaxPromptCharacters));
const SupportedVlmProvider = Schema.Literal('localVision', 'localMultimodal');

export const ScreenVlmWorkerJobSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ScreenVlmWorkerSchemaVersion),
    queueJobId: ScreenEvidenceQueueJobIdSchema,
    createdAt: ActivityTimestampSchema,
    captureReason: ScreenCaptureReasonSchema,
    captureScope: ScreenCaptureScopeSchema,
    capabilityStatus: ScreenCapabilityStatusSchema,
    sourceEvidenceRefs: Schema.Array(ActivityEvidenceRefSchema),
    imageDigest: ScreenEvidenceImageDigestSchema,
    encryptedImageRef: NonEmptyStringSchema,
    modelRuntimeRef: ScreenEvidenceModelRuntimeRefSchema,
    modelId: ScreenEvidenceModelIdSchema,
    promptOrTemplateVersion: ScreenEvidenceTemplateVersionSchema,
    prompt: PromptText,
    maxImagePixels: PositiveInteger,
    providerKind: SupportedVlmProvider,
    custodyState: ScreenEvidenceCustodyStateSchema,
    localOnly: RequiredTrue,
    remoteAiUsed: RequiredFalse,
    rawImageRetained: RequiredFalse,
  }).pipe(
    Schema.filter(
      (value) =>
        value.maxImagePixels <= ScreenVlmWorkerMaxImagePixels ||
        'Expected VLM worker jobs to stay within the local image-pixel budget'
    ),
    Schema.filter(
      (value) =>
        !screenVlmWorkerPromptIsOpenEnded(value.prompt) ||
        'Expected VLM worker prompts to use guided classifier templates instead of open-ended screen descriptions'
    ),
    Schema.filter(
      (value) =>
        value.capabilityStatus !== 'ready' ||
        (value.sourceEvidenceRefs.length > 0 &&
          value.custodyState === 'child-device-temp-queue' &&
          value.localOnly &&
          !value.remoteAiUsed &&
          !value.rawImageRetained) ||
        'Expected ready VLM jobs to cite source evidence and stay local in encrypted temp custody'
    )
  )
);

export function screenVlmWorkerPromptIsOpenEnded(prompt: string) {
  const normalizedPrompt = prompt.toLowerCase();
  return ScreenVlmWorkerRejectedOpenEndedPromptTerms.some((term) => normalizedPrompt.includes(term));
}

export const ScreenVlmWorkerResultSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ScreenVlmWorkerSchemaVersion),
    vlmResultId: ScreenEvidenceResultIdSchema,
    queueJobId: ScreenEvidenceQueueJobIdSchema,
    analyzedAt: ActivityTimestampSchema,
    modelRuntimeRef: ScreenEvidenceModelRuntimeRefSchema,
    modelId: ScreenEvidenceModelIdSchema,
    promptOrTemplateVersion: ScreenEvidenceTemplateVersionSchema,
    providerKind: SupportedVlmProvider,
    captureReason: ScreenCaptureReasonSchema,
    captureScope: ScreenCaptureScopeSchema,
    capabilityStatus: ScreenCapabilityStatusSchema,
    modelOutput: ScreenLocalModelOutputSchema,
    summary: ScreenEvidenceSummaryTextSchema,
    visibleCategoryCandidates: Schema.Array(
      Schema.Struct({
        category: ScreenVisibleCategorySchema,
        confidence: ScreenEvidenceConfidenceSchema,
        evidenceRefs: Schema.Array(ActivityEvidenceRefSchema),
      })
    ),
    primaryCategory: Schema.Union(ScreenVisibleCategorySchema, Schema.Null),
    riskSignals: Schema.Array(
      Schema.Struct({
        signal: ScreenRiskSignalSchema,
        confidence: ScreenEvidenceConfidenceSchema,
        evidenceRefs: Schema.Array(ActivityEvidenceRefSchema),
      })
    ),
    redactionNotes: Schema.Array(ScreenRedactionNoteSchema),
    confidence: ScreenEvidenceConfidenceSchema,
    uncertaintyReason: Schema.Union(ScreenUncertaintyReasonSchema, Schema.Null),
    sourceEvidenceRefs: Schema.Array(ActivityEvidenceRefSchema),
    imageDigest: ScreenEvidenceImageDigestSchema,
    rawImageRetained: RequiredFalse,
    imageDeletionState: ScreenDeletionStateSchema,
    custodyState: ScreenEvidenceCustodyStateSchema,
    policyEligible: Schema.Boolean,
    localOnly: RequiredTrue,
    remoteAiUsed: RequiredFalse,
  }).pipe(
    Schema.filter(
      (value) =>
        value.primaryCategory === value.modelOutput.primary_category ||
        'Expected VLM worker primary category to match schema-bound model output'
    ),
    Schema.filter(
      (value) =>
        value.confidence === value.modelOutput.confidence ||
        'Expected VLM worker confidence to match schema-bound model output'
    ),
    Schema.filter(
      (value) =>
        !value.policyEligible ||
        (value.primaryCategory !== null &&
          value.primaryCategory !== 'unknown' &&
          value.confidence >= 0.5 &&
          value.sourceEvidenceRefs.length > 0 &&
          value.imageDeletionState === 'deleted' &&
          value.custodyState === 'child-device-query-store' &&
          value.localOnly &&
          !value.remoteAiUsed &&
          !value.rawImageRetained) ||
        'Expected policy-eligible VLM results to be categorized, confident, deleted, local, and query-store bound'
    )
  )
);

export const ScreenVlmWorkerProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ScreenVlmWorkerSchemaVersion),
    proofId: NonEmptyStringSchema,
    proofTier: Schema.Literal('P3_CONTRACT_LOCAL_VLM_WORKER'),
    scenarios: Schema.Array(ScreenVlmWorkerResultSchema),
    localOnly: RequiredTrue,
    rawImageRetained: RequiredFalse,
    remoteAiUsed: RequiredFalse,
    rawImageRemoteUploadEnabled: RequiredFalse,
  }).pipe(
    Schema.filter((value) => value.scenarios.length >= 2 || 'Expected VLM worker proof to cover visual and risk rows'),
    Schema.filter(
      (value) =>
        value.scenarios.every((scenario) => scenario.providerKind === 'localVision') ||
        'Expected this guided VLM worker proof to use local vision rows only'
    )
  )
);

export function screenVlmWorkerResultToAnalysisResult(result: ScreenVlmWorkerResult) {
  return ScreenAnalysisResultSchema.parse({
    schemaVersion: ScreenEvidenceSchemaVersion,
    screenAnalysisResultId: result.vlmResultId,
    queueJobId: result.queueJobId,
    analyzedAt: result.analyzedAt,
    modelRuntimeRef: result.modelRuntimeRef,
    modelId: result.modelId,
    providerKind: result.providerKind,
    promptOrTemplateVersion: result.promptOrTemplateVersion,
    captureReason: result.captureReason,
    captureScope: result.captureScope,
    capabilityStatus: result.capabilityStatus,
    summary: result.summary,
    visibleCategoryCandidates: result.visibleCategoryCandidates,
    primaryCategory: result.primaryCategory,
    riskSignals: result.riskSignals,
    ocrTextSnippets: [],
    redactionNotes: result.redactionNotes,
    confidence: result.confidence,
    uncertaintyReason: result.uncertaintyReason,
    sourceEvidenceRefs: result.sourceEvidenceRefs,
    imageDigest: result.imageDigest,
    rawImageRetained: result.rawImageRetained,
    imageDeletionState: result.imageDeletionState,
    custodyState: result.custodyState,
    policyEligible: result.policyEligible,
  });
}

export type ScreenVlmWorkerJob = Infer<typeof ScreenVlmWorkerJobSchema>;
export type ScreenVlmWorkerResult = Infer<typeof ScreenVlmWorkerResultSchema>;
export type ScreenVlmWorkerProof = Infer<typeof ScreenVlmWorkerProofSchema>;
