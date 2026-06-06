import { describe, expect, it } from 'vitest';
import {
  ScreenVlmExecutionReadinessProofSchema,
  ScreenVlmExecutionReadinessProofTier,
  ScreenVlmExecutionReadinessSchemaVersion,
  ScreenVlmExecutionStatusRowSchema,
  ScreenVlmQueueHandoffSchema,
  screenVlmCompletedStatusFromResult,
  screenVlmManualRequiredStatus,
  screenVlmQueueHandoffFromJob,
  screenVlmQueuedStatusFromHandoff,
} from '../src/screen-vlm-execution-readiness';
import {
  ScreenVlmWorkerJobSchema,
  ScreenVlmWorkerMaxImagePixels,
  ScreenVlmWorkerModelId,
  ScreenVlmWorkerResultSchema,
  ScreenVlmWorkerRuntimeRef,
  ScreenVlmWorkerSchemaVersion,
  ScreenVlmWorkerTemplateVersion,
} from '../src/screen-vlm-worker';

const EvidenceRef = {
  evidenceId: 'screen-vlm-execution-readiness-evidence',
  kind: 'journal-entry',
  digest: 'sha256:screen-vlm-execution-readiness-image',
  uri: null,
} as const;

const VlmJob = ScreenVlmWorkerJobSchema.parse({
  schemaVersion: ScreenVlmWorkerSchemaVersion,
  queueJobId: 'screen-vlm-execution-readiness-job',
  createdAt: '2026-06-06T00:20:00.000Z',
  captureReason: 'managedBrowserUrlChange',
  captureScope: 'selectedWindow',
  capabilityStatus: 'ready',
  sourceEvidenceRefs: [EvidenceRef],
  imageDigest: EvidenceRef.digest,
  encryptedImageRef: 'encrypted-temp-screen-vlm-execution-readiness-image',
  modelRuntimeRef: ScreenVlmWorkerRuntimeRef,
  modelId: ScreenVlmWorkerModelId,
  promptOrTemplateVersion: ScreenVlmWorkerTemplateVersion,
  prompt: 'Prepare a local VLM analysis handoff for this encrypted selected-window capture.',
  maxImagePixels: ScreenVlmWorkerMaxImagePixels,
  providerKind: 'localVision',
  custodyState: 'child-device-temp-queue',
  localOnly: true,
  remoteAiUsed: false,
  rawImageRetained: false,
});

const VlmResult = ScreenVlmWorkerResultSchema.parse({
  schemaVersion: ScreenVlmWorkerSchemaVersion,
  vlmResultId: 'screen-vlm-execution-readiness-result',
  queueJobId: VlmJob.queueJobId,
  analyzedAt: '2026-06-06T00:20:05.000Z',
  modelRuntimeRef: ScreenVlmWorkerRuntimeRef,
  modelId: ScreenVlmWorkerModelId,
  promptOrTemplateVersion: ScreenVlmWorkerTemplateVersion,
  providerKind: 'localVision',
  captureReason: VlmJob.captureReason,
  captureScope: VlmJob.captureScope,
  capabilityStatus: 'ready',
  modelOutput: {
    primary_category: 'video',
    confidence: 0.84,
    visible_text: 'A local video page is visible in the selected window.',
    risk_signals: [],
  },
  summary: 'The local VLM result classified the selected-window capture as video.',
  visibleCategoryCandidates: [
    {
      category: 'video',
      confidence: 0.84,
      evidenceRefs: [EvidenceRef],
    },
  ],
  primaryCategory: 'video',
  riskSignals: [],
  redactionNotes: [],
  confidence: 0.84,
  uncertaintyReason: null,
  sourceEvidenceRefs: [EvidenceRef],
  imageDigest: EvidenceRef.digest,
  rawImageRetained: false,
  imageDeletionState: 'deleted',
  custodyState: 'child-device-query-store',
  policyEligible: true,
  localOnly: true,
  remoteAiUsed: false,
});

const Handoff = screenVlmQueueHandoffFromJob({
  job: VlmJob,
  handoffId: 'screen-vlm-execution-readiness-handoff',
  queuedAt: '2026-06-06T00:20:01.000Z',
  acceptedAt: '2026-06-06T00:20:02.000Z',
  statusReason: 'The encrypted local capture is ready for VLM worker execution.',
});

const QueuedStatus = screenVlmQueuedStatusFromHandoff({
  handoff: Handoff,
  statusId: 'screen-vlm-execution-readiness-queued',
  updatedAt: '2026-06-06T00:20:02.000Z',
});

const CompletedStatus = screenVlmCompletedStatusFromResult({
  result: VlmResult,
  statusId: 'screen-vlm-execution-readiness-completed',
});

const ManualRequiredStatus = screenVlmManualRequiredStatus({
  queueJobId: 'screen-vlm-execution-readiness-manual-job',
  statusId: 'screen-vlm-execution-readiness-manual',
  updatedAt: '2026-06-06T00:20:03.000Z',
  statusReason: 'The local VLM runtime is unavailable, so manual review remains required.',
  degradedReasons: ['local-vlm-runtime-unavailable'],
});

describe('screen VLM execution readiness contracts', () => {
  it('builds an accepted queue handoff from the local VLM worker job without claiming execution', () => {
    const handoff = ScreenVlmQueueHandoffSchema.parse(Handoff);

    expect(handoff.queueAccepted).toBe(true);
    expect(handoff.job.custodyState).toBe('child-device-temp-queue');
    expect(handoff.modelRuntimeRef).toBe(ScreenVlmWorkerRuntimeRef);
    expect(handoff.nonClaims.liveModelExecutionClaimed).toBe(false);
    expect(handoff.nonClaims.remoteAiUsed).toBe(false);
  });

  it('keeps queued rows in temp custody and completed rows in deleted query-store custody', () => {
    const queued = ScreenVlmExecutionStatusRowSchema.parse(QueuedStatus);
    const completed = ScreenVlmExecutionStatusRowSchema.parse(CompletedStatus);

    expect(queued.status).toBe('queued');
    expect(queued.result).toBeNull();
    expect(queued.custodyState).toBe('child-device-temp-queue');
    expect(completed.status).toBe('completed');
    expect(completed.result?.imageDeletionState).toBe('deleted');
    expect(completed.custodyState).toBe('child-device-query-store');
  });

  it('rejects completed rows before deletion and any proof claiming live execution', () => {
    const beforeDeletion = ScreenVlmExecutionStatusRowSchema.safeParse({
      ...CompletedStatus,
      result: {
        ...VlmResult,
        imageDeletionState: 'deletePending',
      },
    });
    const liveExecutionClaim = ScreenVlmExecutionReadinessProofSchema.safeParse({
      schemaVersion: ScreenVlmExecutionReadinessSchemaVersion,
      proofId: 'screen-vlm-execution-readiness-proof',
      proofTier: ScreenVlmExecutionReadinessProofTier,
      handoffs: [Handoff],
      statusRows: [QueuedStatus, CompletedStatus, ManualRequiredStatus],
      localOnly: true,
      remoteAiUsed: false,
      rawImageRetained: false,
      liveModelExecutionClaimed: true,
      productionVlmQualityClaimed: false,
      portalRuntimeClaimed: false,
      enforcementClaimed: false,
    });

    expect(beforeDeletion.success).toBe(false);
    expect(liveExecutionClaim.success).toBe(false);
  });

  it('proves queued, completed, and manual-required status rows with explicit non-claims', () => {
    const proof = ScreenVlmExecutionReadinessProofSchema.parse({
      schemaVersion: ScreenVlmExecutionReadinessSchemaVersion,
      proofId: 'screen-vlm-execution-readiness-proof',
      proofTier: ScreenVlmExecutionReadinessProofTier,
      handoffs: [Handoff],
      statusRows: [QueuedStatus, CompletedStatus, ManualRequiredStatus],
      localOnly: true,
      remoteAiUsed: false,
      rawImageRetained: false,
      liveModelExecutionClaimed: false,
      productionVlmQualityClaimed: false,
      portalRuntimeClaimed: false,
      enforcementClaimed: false,
    });

    expect(proof.statusRows.map((row) => row.status)).toEqual(['queued', 'completed', 'manual-required']);
    expect(proof.statusRows.every((row) => !row.nonClaims.liveModelExecutionClaimed)).toBe(true);
    expect(proof.statusRows.every((row) => !row.nonClaims.rawImageRetained)).toBe(true);
  });
});
