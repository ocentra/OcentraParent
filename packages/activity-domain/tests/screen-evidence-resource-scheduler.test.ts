import { describe, expect, it } from 'vitest';
import {
  ScreenLocalAiResourceDecisionSchema,
  ScreenLocalAiResourceProofSchema,
  screenLocalAiResourceSchedulerDecisions,
  screenLocalAiResourceSchedulerProof,
  screenLocalAiResourceSchedulerProofSummary,
} from '../src/screen-evidence';

describe('screen evidence local AI resource scheduler contracts', () => {
  it('accepts screen OCR and VLM jobs with one heavy runtime lane per device', acceptsSchedulerProof);
  it('rejects more than one active heavy screen AI job on the same device', rejectsDuplicateHeavyJobs);
  it('rejects background admission ahead of policy-blocking work', rejectsBadAdmissionOrder);
  it('rejects queued heavy jobs without duplicate runtime blocking', rejectsQueuedJobsWithoutSingletonBlocking);
  it('rejects unsafe raw retention, remote AI, pixel caps, and OCR snippet caps', rejectsUnsafeCapsAndCustody);
});

function acceptsSchedulerProof() {
  const proof = ScreenLocalAiResourceProofSchema.parse(screenLocalAiResourceSchedulerProof);

  expect(proof.queueSnapshot.activeHeavyJobCount).toBe(1);
  expect(proof.queueSnapshot.maxActiveHeavyJobs).toBe(1);
  expect(proof.admissionOrder).toEqual([
    'screen-policy-vlm-running',
    'screen-foreground-ocr-complete',
    'screen-cadence-vlm-queued',
    'screen-background-vlm-queued',
  ]);
  expect(screenLocalAiResourceSchedulerProofSummary(proof.decisions)).toEqual({
    totalJobs: 7,
    heavyJobs: 4,
    queuedJobs: 2,
    timedOutJobs: 1,
    skippedOrDegradedJobs: 1,
    policyBlockingJobs: 1,
    remoteAiAllowed: false,
    rawImageRetained: false,
  });
}

function rejectsDuplicateHeavyJobs() {
  expect(
    ScreenLocalAiResourceProofSchema.safeParse({
      ...screenLocalAiResourceSchedulerProof,
      queueSnapshot: {
        ...screenLocalAiResourceSchedulerProof.queueSnapshot,
        activeHeavyJobCount: 2,
      },
    }).success
  ).toBe(false);
}

function rejectsBadAdmissionOrder() {
  expect(
    ScreenLocalAiResourceProofSchema.safeParse({
      ...screenLocalAiResourceSchedulerProof,
      admissionOrder: [
        'screen-background-vlm-queued',
        'screen-foreground-ocr-complete',
        'screen-policy-vlm-running',
        'screen-cadence-vlm-queued',
      ],
    }).success
  ).toBe(false);
}

function rejectsQueuedJobsWithoutSingletonBlocking() {
  const queued = screenLocalAiResourceSchedulerDecisions.find((decision) => decision.queuePosition === 1);

  expect(queued?.jobState).toBe('queued');
  expect(
    ScreenLocalAiResourceDecisionSchema.safeParse({
      ...queued,
      duplicateRuntimeBlocked: false,
    }).success
  ).toBe(false);
}

function rejectsUnsafeCapsAndCustody() {
  const completeOcr = screenLocalAiResourceSchedulerDecisions.find((decision) => decision.jobKind === 'ocrText');

  expect(completeOcr?.jobState).toBe('complete');
  expect(ScreenLocalAiResourceDecisionSchema.safeParse({ ...completeOcr, rawImageRetained: true }).success).toBe(false);
  expect(ScreenLocalAiResourceDecisionSchema.safeParse({ ...completeOcr, remoteAiAllowed: true }).success).toBe(false);
  expect(
    ScreenLocalAiResourceDecisionSchema.safeParse({
      ...completeOcr,
      caps: { ...completeOcr?.caps, maxImagePixels: 2073601 },
    }).success
  ).toBe(false);
  expect(
    ScreenLocalAiResourceDecisionSchema.safeParse({
      ...completeOcr,
      caps: { ...completeOcr?.caps, ocrSnippetCharLimit: 241 },
    }).success
  ).toBe(false);
}
