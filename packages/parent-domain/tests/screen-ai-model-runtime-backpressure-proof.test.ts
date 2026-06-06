import { describe, expect, it } from 'vitest';
import {
  ScreenAiModelRuntimeBackpressureProofSchema,
  buildScreenAiModelRuntimeBackpressureProof,
  screenAiModelRuntimeBackpressureSummary,
} from '../src/screen-ai-model-runtime-backpressure-proof';

const generatedAt = '2026-06-06T01:20:00.000Z';
const physicalDeviceId = 'child-laptop-physical-1';
const runtimeReferenceId = 'runtime:screen-child-safety-vlm';
const modelId = 'screen-child-safety-vlm';

const runningRow = {
  jobId: 'screen-model-job-1',
  physicalDeviceId,
  sourceEncryptedQueueRef: 'queue:encrypted-frame-1',
  captureDigestRef: 'digest:frame-1',
  priority: 'policy-blocking',
  requestedAt: generatedAt,
  modelId,
  runtimeReferenceId,
  providerDecision: {
    physicalDeviceId,
    jobClass: 'child-safety',
    jobStatus: 'running',
    selectedRuntimeReferenceId: runtimeReferenceId,
    queuePosition: null,
    unavailableReason: null,
    duplicateRuntimeBlocked: true,
  },
  jobState: 'running',
  queuePosition: null,
  maxQueueDepth: 2,
  activeHeavyRuntimeCount: 1,
  queuedHeavyJobCount: 0,
  backpressureAction: 'run-now',
  degradedState: 'none',
  unavailableReason: null,
  policyEligible: false,
  remoteProviderUsed: false,
  rawImageRetained: false,
} as const;

const queuedRow = {
  ...runningRow,
  jobId: 'screen-model-job-2',
  sourceEncryptedQueueRef: 'queue:encrypted-frame-2',
  captureDigestRef: 'digest:frame-2',
  priority: 'foreground',
  providerDecision: {
    ...runningRow.providerDecision,
    jobStatus: 'queued',
    queuePosition: 1,
  },
  jobState: 'queued',
  queuePosition: 1,
  queuedHeavyJobCount: 1,
  backpressureAction: 'enqueue',
} as const;

const overflowRow = {
  ...runningRow,
  jobId: 'screen-model-job-4',
  sourceEncryptedQueueRef: 'queue:encrypted-frame-4',
  captureDigestRef: 'digest:frame-4',
  priority: 'cadence',
  providerDecision: {
    ...runningRow.providerDecision,
    jobStatus: 'degraded',
  },
  jobState: 'overflow-degraded',
  queuedHeavyJobCount: 2,
  backpressureAction: 'reject-overload',
  degradedState: 'overloaded',
} as const;

const readyProof = {
  schemaVersion: 'v0.6',
  proofId: 'screen-ai-model-runtime-backpressure-proof',
  generatedAt,
  maxQueueDepth: 2,
  rows: [runningRow, queuedRow, overflowRow],
} as const;

function parsesWith(overrides: Record<string, unknown>): boolean {
  return ScreenAiModelRuntimeBackpressureProofSchema.safeParse({
    ...readyProof,
    ...overrides,
  }).success;
}
describe('screen AI model runtime backpressure accepted state', () => {
  it('accepts one active heavy screen model runtime with bounded queued work and overload rejection', () => {
    const proof = buildScreenAiModelRuntimeBackpressureProof(readyProof);
    const summary = screenAiModelRuntimeBackpressureSummary(proof);

    expect(summary).toMatchObject({
      totalJobs: 3,
      maxQueueDepth: 2,
      activeHeavyRuntimeCount: 1,
      queuedHeavyJobCount: 2,
      overflowRejectedCount: 1,
      singleActiveHeavyRuntime: true,
      boundedQueueDepth: true,
      overflowRowsPolicyIneligible: true,
    });
  });
});

describe('screen AI model runtime backpressure rejection states', () => {
  it('rejects duplicate active heavy model runtimes on one physical device', () => {
    expect(
      parsesWith({
        rows: [
          {
            ...runningRow,
            activeHeavyRuntimeCount: 2,
          },
          queuedRow,
          overflowRow,
        ],
      })
    ).toBe(false);
  });

  it('rejects queued work beyond the model runtime queue cap', () => {
    expect(
      parsesWith({
        rows: [
          runningRow,
          {
            ...queuedRow,
            queuedHeavyJobCount: 3,
          },
          overflowRow,
        ],
      })
    ).toBe(false);
  });

  it('rejects overload rows that become policy eligible before analysis exists', () => {
    expect(
      parsesWith({
        rows: [
          runningRow,
          queuedRow,
          {
            ...overflowRow,
            policyEligible: true,
          },
        ],
      })
    ).toBe(false);
  });

  it('rejects remote provider fallback or raw image retention during flood control', () => {
    expect(
      parsesWith({
        rows: [
          {
            ...runningRow,
            remoteProviderUsed: true,
          },
          queuedRow,
          overflowRow,
        ],
      })
    ).toBe(false);
    expect(
      parsesWith({
        rows: [
          runningRow,
          queuedRow,
          {
            ...overflowRow,
            rawImageRetained: true,
          },
        ],
      })
    ).toBe(false);
  });
});
