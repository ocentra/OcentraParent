import { describe, expect, it } from 'vitest';
import {
  ScreenOcrWorkerJobSchema,
  ScreenOcrWorkerModelId,
  ScreenOcrWorkerProofSchema,
  ScreenOcrWorkerResultSchema,
  ScreenOcrWorkerRuntimeRef,
  ScreenOcrWorkerSchemaVersion,
  ScreenOcrWorkerTemplateVersion,
  screenOcrWorkerResultToAnalysisResult,
} from '../../src/screen-ocr-worker';

const EvidenceRef = {
  evidenceId: 'screen-ocr-worker-evidence',
  kind: 'journal-entry',
  digest: 'sha256:screen-ocr-worker-image',
  uri: null,
} as const;

const OcrJob = {
  schemaVersion: ScreenOcrWorkerSchemaVersion,
  queueJobId: 'screen-ocr-worker-job',
  createdAt: '2026-06-05T09:25:00.000Z',
  captureReason: 'manualParentTestCapture',
  captureScope: 'selectedWindow',
  capabilityStatus: 'ready',
  sourceEvidenceRefs: [EvidenceRef],
  imageDigest: 'sha256:screen-ocr-worker-image',
  encryptedImageRef: 'encrypted-temp-screen-ocr-worker-image',
  ocrEngine: 'winRtOcr',
  custodyState: 'child-device-temp-queue',
  rawImageRetained: false,
} as const;

const OcrResult = {
  schemaVersion: ScreenOcrWorkerSchemaVersion,
  ocrResultId: 'screen-ocr-worker-result',
  queueJobId: OcrJob.queueJobId,
  analyzedAt: '2026-06-05T09:25:03.000Z',
  ocrEngine: 'winRtOcr',
  modelRuntimeRef: ScreenOcrWorkerRuntimeRef,
  modelId: ScreenOcrWorkerModelId,
  promptOrTemplateVersion: ScreenOcrWorkerTemplateVersion,
  captureReason: OcrJob.captureReason,
  captureScope: OcrJob.captureScope,
  capabilityStatus: 'ready',
  textLines: [
    {
      text: 'Khan Academy math lesson',
      confidence: 0.91,
      boundingBoxRef: 'line-1',
    },
  ],
  ocrTextSnippets: [
    {
      text: 'Khan Academy math lesson',
      confidence: 0.91,
      evidenceRefs: [EvidenceRef],
    },
  ],
  summary: 'WinRT OCR extracted school lesson text from the captured window.',
  visibleCategoryCandidates: [
    {
      category: 'school',
      confidence: 0.91,
      evidenceRefs: [EvidenceRef],
    },
  ],
  primaryCategory: 'school',
  riskSignals: [],
  redactionNotes: [],
  confidence: 0.91,
  uncertaintyReason: null,
  sourceEvidenceRefs: [EvidenceRef],
  imageDigest: OcrJob.imageDigest,
  rawImageRetained: false,
  imageDeletionState: 'deleted',
  custodyState: 'child-device-query-store',
  policyEligible: true,
  lineCount: 1,
} as const;

const OverflowTextLines = [
  ...OcrResult.textLines,
  { text: 'second line', confidence: 0.8, boundingBoxRef: 'line-2' },
  { text: 'third line', confidence: 0.8, boundingBoxRef: 'line-3' },
  { text: 'fourth line', confidence: 0.8, boundingBoxRef: 'line-4' },
  { text: 'fifth line', confidence: 0.8, boundingBoxRef: 'line-5' },
  { text: 'sixth line', confidence: 0.8, boundingBoxRef: 'line-6' },
] as const;

const NativeOcrResult = {
  ...OcrResult,
  ocrResultId: 'screen-ocr-worker-result-native',
  queueJobId: 'screen-ocr-worker-job-native',
  captureReason: 'nativeAppForegroundStart',
  captureScope: 'activeWindow',
  primaryCategory: 'productivity',
  visibleCategoryCandidates: [
    {
      category: 'productivity',
      confidence: 0.88,
      evidenceRefs: [EvidenceRef],
    },
  ],
} as const;

const OcrProof = {
  schemaVersion: ScreenOcrWorkerSchemaVersion,
  proofId: 'screen-winrt-ocr-worker-proof',
  proofTier: 'P3_REAL_CAPTURE_LOCAL_OCR',
  scenarios: [OcrResult, NativeOcrResult],
  localOnly: true,
  rawImageRetained: false,
  remoteAiUsed: false,
  rawImageRemoteUploadEnabled: false,
} as const;

describe('screen OCR worker contracts', () => {
  it('parses a local WinRT OCR job and converts the OCR result into screen analysis evidence', () => {
    const job = ScreenOcrWorkerJobSchema.parse(OcrJob);
    const result = ScreenOcrWorkerResultSchema.parse(OcrResult);
    const analysis = screenOcrWorkerResultToAnalysisResult(result);

    expect(job.custodyState).toBe('child-device-temp-queue');
    expect(result.ocrEngine).toBe('winRtOcr');
    expect(analysis.providerKind).toBe('localOcr');
    expect(analysis.primaryCategory).toBe('school');
    expect(analysis.imageDeletionState).toBe('deleted');
    expect(analysis.rawImageRetained).toBe(false);
  });

  it('rejects ready OCR jobs without source evidence or child temp-queue custody', () => {
    const missingEvidence = ScreenOcrWorkerJobSchema.safeParse({
      ...OcrJob,
      sourceEvidenceRefs: [],
    });
    const wrongCustody = ScreenOcrWorkerJobSchema.safeParse({
      ...OcrJob,
      custodyState: 'parent-device-cache',
    });
    const retainedRawImage = ScreenOcrWorkerJobSchema.safeParse({
      ...OcrJob,
      rawImageRetained: true,
    });

    expect(missingEvidence.success).toBe(false);
    expect(wrongCustody.success).toBe(false);
    expect(retainedRawImage.success).toBe(false);
  });

  it('rejects OCR results that are unbounded, mismatched, or policy-eligible before deletion', () => {
    const tooManyLines = ScreenOcrWorkerResultSchema.safeParse({
      ...OcrResult,
      lineCount: OverflowTextLines.length,
      textLines: OverflowTextLines,
    });
    const mismatchedLineCount = ScreenOcrWorkerResultSchema.safeParse({
      ...OcrResult,
      lineCount: 2,
    });
    const mismatchedSnippetCount = ScreenOcrWorkerResultSchema.safeParse({
      ...OcrResult,
      ocrTextSnippets: [],
    });
    const policyBeforeDeletion = ScreenOcrWorkerResultSchema.safeParse({
      ...OcrResult,
      imageDeletionState: 'deletePending',
    });

    expect(tooManyLines.success).toBe(false);
    expect(mismatchedLineCount.success).toBe(false);
    expect(mismatchedSnippetCount.success).toBe(false);
    expect(policyBeforeDeletion.success).toBe(false);
  });

  it('requires proof rows to be local-only WinRT OCR rows with no raw retention or remote AI', () => {
    const proof = ScreenOcrWorkerProofSchema.parse(OcrProof);
    const remoteAi = ScreenOcrWorkerProofSchema.safeParse({
      ...proof,
      remoteAiUsed: true,
    });

    expect(proof.scenarios).toHaveLength(2);
    expect(remoteAi.success).toBe(false);
  });
});
