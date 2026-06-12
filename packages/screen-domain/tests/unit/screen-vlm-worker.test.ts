import { describe, expect, it } from 'vitest';
import {
  ScreenVlmWorkerJobSchema,
  ScreenVlmWorkerMaxImagePixels,
  ScreenVlmWorkerModelId,
  ScreenVlmWorkerProofSchema,
  ScreenVlmWorkerResultSchema,
  ScreenVlmWorkerRuntimeRef,
  ScreenVlmWorkerSchemaVersion,
  ScreenVlmWorkerTemplateVersion,
  screenVlmWorkerPromptIsOpenEnded,
  screenVlmWorkerResultToAnalysisResult,
} from '../../src/screen-vlm-worker';

const EvidenceRef = {
  evidenceId: 'screen-vlm-worker-evidence',
  kind: 'journal-entry',
  digest: 'sha256:screen-vlm-worker-image',
  uri: null,
} as const;

const VlmJob = {
  schemaVersion: ScreenVlmWorkerSchemaVersion,
  queueJobId: 'screen-vlm-worker-job',
  createdAt: '2026-06-05T23:18:00.000Z',
  captureReason: 'managedBrowserUrlChange',
  captureScope: 'selectedWindow',
  capabilityStatus: 'ready',
  sourceEvidenceRefs: [EvidenceRef],
  imageDigest: 'sha256:screen-vlm-worker-image',
  encryptedImageRef: 'encrypted-temp-screen-vlm-worker-image',
  modelRuntimeRef: ScreenVlmWorkerRuntimeRef,
  modelId: ScreenVlmWorkerModelId,
  promptOrTemplateVersion: ScreenVlmWorkerTemplateVersion,
  prompt: 'Classify the visible screen category and risk signals from the local encrypted image only.',
  maxImagePixels: ScreenVlmWorkerMaxImagePixels,
  providerKind: 'localVision',
  custodyState: 'child-device-temp-queue',
  localOnly: true,
  remoteAiUsed: false,
  rawImageRetained: false,
} as const;

const VlmResult = {
  schemaVersion: ScreenVlmWorkerSchemaVersion,
  vlmResultId: 'screen-vlm-worker-result',
  queueJobId: VlmJob.queueJobId,
  analyzedAt: '2026-06-05T23:18:05.000Z',
  modelRuntimeRef: ScreenVlmWorkerRuntimeRef,
  modelId: ScreenVlmWorkerModelId,
  promptOrTemplateVersion: ScreenVlmWorkerTemplateVersion,
  providerKind: 'localVision',
  captureReason: VlmJob.captureReason,
  captureScope: VlmJob.captureScope,
  capabilityStatus: 'ready',
  modelOutput: {
    primary_category: 'game',
    confidence: 0.86,
    visible_text: 'A browser game board is visible in the selected window.',
    risk_signals: [],
  },
  summary: 'The selected-window image is a browser game screen.',
  visibleCategoryCandidates: [
    {
      category: 'game',
      confidence: 0.86,
      evidenceRefs: [EvidenceRef],
    },
  ],
  primaryCategory: 'game',
  riskSignals: [],
  redactionNotes: [],
  confidence: 0.86,
  uncertaintyReason: null,
  sourceEvidenceRefs: [EvidenceRef],
  imageDigest: VlmJob.imageDigest,
  rawImageRetained: false,
  imageDeletionState: 'deleted',
  custodyState: 'child-device-query-store',
  policyEligible: true,
  localOnly: true,
  remoteAiUsed: false,
} as const;

const RiskResult = {
  ...VlmResult,
  vlmResultId: 'screen-vlm-worker-risk-result',
  queueJobId: 'screen-vlm-worker-risk-job',
  modelOutput: {
    primary_category: 'bypassTool',
    confidence: 0.91,
    visible_text: 'A proxy bypass tool surface is visible in the selected window.',
    risk_signals: ['possibleBypassTool'],
  },
  summary: 'The selected-window image shows a bypass tool surface.',
  visibleCategoryCandidates: [
    {
      category: 'bypassTool',
      confidence: 0.91,
      evidenceRefs: [EvidenceRef],
    },
  ],
  primaryCategory: 'bypassTool',
  riskSignals: [
    {
      signal: 'possibleBypassTool',
      confidence: 0.91,
      evidenceRefs: [EvidenceRef],
    },
  ],
  confidence: 0.91,
} as const;

const VlmProof = {
  schemaVersion: ScreenVlmWorkerSchemaVersion,
  proofId: 'screen-vlm-worker-contract-proof',
  proofTier: 'P3_CONTRACT_LOCAL_VLM_WORKER',
  scenarios: [VlmResult, RiskResult],
  localOnly: true,
  rawImageRetained: false,
  remoteAiUsed: false,
  rawImageRemoteUploadEnabled: false,
} as const;

describe('screen VLM worker parsing', () => {
  it('parses a guided local VLM job and converts the result into screen analysis evidence', () => {
    const job = ScreenVlmWorkerJobSchema.parse(VlmJob);
    const result = ScreenVlmWorkerResultSchema.parse(VlmResult);
    const analysis = screenVlmWorkerResultToAnalysisResult(result);

    expect(job.custodyState).toBe('child-device-temp-queue');
    expect(result.providerKind).toBe('localVision');
    expect(analysis.providerKind).toBe('localVision');
    expect(analysis.primaryCategory).toBe('game');
    expect(analysis.imageDeletionState).toBe('deleted');
    expect(analysis.rawImageRetained).toBe(false);
  });
});

describe('screen VLM worker job guards', () => {
  it('rejects ready VLM jobs without source evidence, local custody, or bounded local image input', () => {
    const missingEvidence = ScreenVlmWorkerJobSchema.safeParse({
      ...VlmJob,
      sourceEvidenceRefs: [],
    });
    const wrongCustody = ScreenVlmWorkerJobSchema.safeParse({
      ...VlmJob,
      custodyState: 'parent-device-cache',
    });
    const remoteAi = ScreenVlmWorkerJobSchema.safeParse({
      ...VlmJob,
      remoteAiUsed: true,
    });
    const retainedRawImage = ScreenVlmWorkerJobSchema.safeParse({
      ...VlmJob,
      rawImageRetained: true,
    });
    const oversizedImage = ScreenVlmWorkerJobSchema.safeParse({
      ...VlmJob,
      maxImagePixels: ScreenVlmWorkerMaxImagePixels + 1,
    });

    expect(missingEvidence.success).toBe(false);
    expect(wrongCustody.success).toBe(false);
    expect(remoteAi.success).toBe(false);
    expect(retainedRawImage.success).toBe(false);
    expect(oversizedImage.success).toBe(false);
  });

  it('rejects open-ended VLM description prompts before local worker handoff', () => {
    const openEndedPrompt = ScreenVlmWorkerJobSchema.safeParse({
      ...VlmJob,
      prompt: 'Describe the screen in detail.',
    });

    expect(screenVlmWorkerPromptIsOpenEnded('Describe this screen')).toBe(true);
    expect(screenVlmWorkerPromptIsOpenEnded(VlmJob.prompt)).toBe(false);
    expect(openEndedPrompt.success).toBe(false);
  });
});

describe('screen VLM worker result guards', () => {
  it('rejects VLM results that drift from schema-bound model output or become policy-eligible before deletion', () => {
    const categoryDrift = ScreenVlmWorkerResultSchema.safeParse({
      ...VlmResult,
      primaryCategory: 'video',
    });
    const confidenceDrift = ScreenVlmWorkerResultSchema.safeParse({
      ...VlmResult,
      confidence: 0.9,
    });
    const policyBeforeDeletion = ScreenVlmWorkerResultSchema.safeParse({
      ...VlmResult,
      imageDeletionState: 'deletePending',
    });
    const remoteAi = ScreenVlmWorkerResultSchema.safeParse({
      ...VlmResult,
      remoteAiUsed: true,
    });

    expect(categoryDrift.success).toBe(false);
    expect(confidenceDrift.success).toBe(false);
    expect(policyBeforeDeletion.success).toBe(false);
    expect(remoteAi.success).toBe(false);
  });
});

describe('screen VLM worker proof guards', () => {
  it('requires proof rows to be local-only VLM rows with no raw retention or remote upload', () => {
    const proof = ScreenVlmWorkerProofSchema.parse(VlmProof);
    const remoteProof = ScreenVlmWorkerProofSchema.safeParse({
      ...proof,
      remoteAiUsed: true,
    });

    expect(proof.scenarios).toHaveLength(2);
    expect(remoteProof.success).toBe(false);
  });
});
