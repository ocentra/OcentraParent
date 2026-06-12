import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceRefSchema } from '@ocentra-parent/evidence-domain/contracts';
import { ActivityTimestampSchema } from '@ocentra-parent/evidence-domain/primitives';
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
import { ScreenAnalysisResultSchema, ScreenOcrTextSnippetSchema } from './screen-evidence-result';
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

export const ScreenOcrWorkerSchemaVersion = 1;
export const ScreenOcrWorkerTemplateVersion = 'screen-ocr-worker-winrt-v1';
export const ScreenOcrWorkerModelId = 'windows-winrt-ocr';
export const ScreenOcrWorkerRuntimeRef = 'windows-winrt-ocr-local-runtime';
export const ScreenOcrWorkerMaxSnippetCount = 5;

const RequiredFalse = Schema.Literal(false);
const PositiveInteger = Schema.Number.pipe(Schema.int(), Schema.positive());
const NonEmptyText = Schema.String.pipe(Schema.minLength(1));
const SupportedOcrEngine = Schema.Literal('winRtOcr');

export const ScreenOcrWorkerJobSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ScreenOcrWorkerSchemaVersion),
    queueJobId: ScreenEvidenceQueueJobIdSchema,
    createdAt: ActivityTimestampSchema,
    captureReason: ScreenCaptureReasonSchema,
    captureScope: ScreenCaptureScopeSchema,
    capabilityStatus: ScreenCapabilityStatusSchema,
    sourceEvidenceRefs: Schema.Array(ActivityEvidenceRefSchema),
    imageDigest: ScreenEvidenceImageDigestSchema,
    encryptedImageRef: NonEmptyText,
    ocrEngine: SupportedOcrEngine,
    custodyState: ScreenEvidenceCustodyStateSchema,
    rawImageRetained: RequiredFalse,
  }).pipe(
    Schema.filter(
      (value) =>
        value.capabilityStatus !== 'ready' ||
        (value.sourceEvidenceRefs.length > 0 &&
          value.custodyState === 'child-device-temp-queue' &&
          value.rawImageRetained === false) ||
        'Expected ready OCR jobs to cite source evidence and stay in encrypted child-device temp custody'
    )
  )
);

export const ScreenOcrWorkerTextLineSchema = withParser(
  Schema.Struct({
    text: ScreenEvidenceOcrSnippetTextSchema,
    confidence: ScreenEvidenceConfidenceSchema,
    boundingBoxRef: NonEmptyText,
  })
);

export const ScreenOcrWorkerResultSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ScreenOcrWorkerSchemaVersion),
    ocrResultId: ScreenEvidenceResultIdSchema,
    queueJobId: ScreenEvidenceQueueJobIdSchema,
    analyzedAt: ActivityTimestampSchema,
    ocrEngine: SupportedOcrEngine,
    modelRuntimeRef: ScreenEvidenceModelRuntimeRefSchema,
    modelId: ScreenEvidenceModelIdSchema,
    promptOrTemplateVersion: ScreenEvidenceTemplateVersionSchema,
    captureReason: ScreenCaptureReasonSchema,
    captureScope: ScreenCaptureScopeSchema,
    capabilityStatus: ScreenCapabilityStatusSchema,
    textLines: Schema.Array(ScreenOcrWorkerTextLineSchema),
    ocrTextSnippets: Schema.Array(ScreenOcrTextSnippetSchema),
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
    lineCount: PositiveInteger,
  }).pipe(
    Schema.filter(
      (value) =>
        value.textLines.length <= ScreenOcrWorkerMaxSnippetCount ||
        'Expected OCR worker proof to keep bounded retained snippets'
    ),
    Schema.filter(
      (value) =>
        value.textLines.length === value.lineCount || 'Expected OCR line count to match retained OCR line evidence'
    ),
    Schema.filter(
      (value) =>
        value.ocrTextSnippets.length === value.textLines.length ||
        'Expected OCR snippets to match retained OCR line evidence'
    ),
    Schema.filter(
      (value) =>
        !value.policyEligible ||
        (value.primaryCategory !== null &&
          value.primaryCategory !== 'unknown' &&
          value.confidence >= 0.5 &&
          value.imageDeletionState === 'deleted' &&
          value.custodyState === 'child-device-query-store') ||
        'Expected policy-eligible OCR results to be categorized, confident, deleted, and query-store bound'
    )
  )
);

export const ScreenOcrWorkerProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ScreenOcrWorkerSchemaVersion),
    proofId: NonEmptyText,
    proofTier: Schema.Literal('P3_REAL_CAPTURE_LOCAL_OCR'),
    scenarios: Schema.Array(ScreenOcrWorkerResultSchema),
    localOnly: Schema.Literal(true),
    rawImageRetained: RequiredFalse,
    remoteAiUsed: RequiredFalse,
    rawImageRemoteUploadEnabled: RequiredFalse,
  }).pipe(
    Schema.filter(
      (value) => value.scenarios.length >= 2 || 'Expected OCR proof to cover browser and native app captures'
    ),
    Schema.filter(
      (value) =>
        value.scenarios.every((scenario) => scenario.ocrEngine === 'winRtOcr') || 'Expected real WinRT OCR rows'
    )
  )
);

export function screenOcrWorkerResultToAnalysisResult(result: ScreenOcrWorkerResult) {
  return ScreenAnalysisResultSchema.parse({
    schemaVersion: ScreenEvidenceSchemaVersion,
    screenAnalysisResultId: result.ocrResultId,
    queueJobId: result.queueJobId,
    analyzedAt: result.analyzedAt,
    modelRuntimeRef: result.modelRuntimeRef,
    modelId: result.modelId,
    providerKind: 'localOcr',
    promptOrTemplateVersion: result.promptOrTemplateVersion,
    captureReason: result.captureReason,
    captureScope: result.captureScope,
    capabilityStatus: result.capabilityStatus,
    summary: result.summary,
    visibleCategoryCandidates: result.visibleCategoryCandidates,
    primaryCategory: result.primaryCategory,
    riskSignals: result.riskSignals,
    ocrTextSnippets: result.ocrTextSnippets,
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

export type ScreenOcrWorkerJob = Infer<typeof ScreenOcrWorkerJobSchema>;
export type ScreenOcrWorkerTextLine = Infer<typeof ScreenOcrWorkerTextLineSchema>;
export type ScreenOcrWorkerResult = Infer<typeof ScreenOcrWorkerResultSchema>;
export type ScreenOcrWorkerProof = Infer<typeof ScreenOcrWorkerProofSchema>;
