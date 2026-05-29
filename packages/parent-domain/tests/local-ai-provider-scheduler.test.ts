import { describe, expect, it } from 'vitest';
import {
  LocalAiProviderSchedulerDecisionSchema,
  LocalAiProviderSchedulerStatusSchema,
} from '../src/local-ai-provider-scheduler';

const idleSchedulerStatus = {
  physicalDeviceId: 'device-local-1',
  singletonScope: 'physical-device',
  providerId: 'local-provider-llama-cli',
  runtimeReferenceId: 'local-ai-runtime-local-llama-cli',
  modelId: 'local-gguf-chat-model',
  modelReference: 'artifact:local_gguf_chat_model',
  resourceClass: 'cpu',
  lifecycleState: 'idle',
  currentJobClass: null,
  queue: {
    childSafetyQueued: 0,
    parentAssistantQueued: 0,
    parentReportQueued: 0,
  },
  duplicateRuntimeBlocked: false,
  degradedState: 'none',
  unavailableReason: null,
  lastCheckedAt: '2026-05-27T01:57:00.000Z',
} as const;

const queuedStatus = {
  ...idleSchedulerStatus,
  lifecycleState: 'queued',
  currentJobClass: 'child-safety',
  queue: {
    childSafetyQueued: 1,
    parentAssistantQueued: 1,
    parentReportQueued: 0,
  },
  duplicateRuntimeBlocked: true,
} as const;

const unavailableStatus = {
  ...idleSchedulerStatus,
  lifecycleState: 'unavailable',
  providerId: 'local-provider-unconfigured',
  runtimeReferenceId: 'local-ai-runtime-dev-unconfigured',
  modelId: 'safety-model-unconfigured',
  modelReference: 'local-model-cache-unconfigured',
  degradedState: 'provider-unavailable',
  unavailableReason: 'local-ai-provider-unconfigured',
} as const;

const idleStatusWithQueuedReport = {
  ...idleSchedulerStatus,
  queue: {
    childSafetyQueued: 0,
    parentAssistantQueued: 0,
    parentReportQueued: 1,
  },
} as const;

const queuedDecision = {
  physicalDeviceId: 'device-local-1',
  jobClass: 'parent-assistant',
  jobStatus: 'queued',
  selectedRuntimeReferenceId: 'local-ai-runtime-local-llama-cli',
  queuePosition: 2,
  unavailableReason: null,
  duplicateRuntimeBlocked: true,
} as const;

const unavailableDecisionWithoutReason = {
  physicalDeviceId: 'device-local-1',
  jobClass: 'child-safety',
  jobStatus: 'unavailable',
  selectedRuntimeReferenceId: null,
  queuePosition: null,
  unavailableReason: null,
  duplicateRuntimeBlocked: false,
} as const;

const queuedDecisionWithoutRuntimeLaneBlock = {
  ...queuedDecision,
  duplicateRuntimeBlocked: false,
} as const;

const runningDecisionWithoutRuntimeLaneBlock = {
  ...queuedDecision,
  jobStatus: 'running',
  queuePosition: null,
  duplicateRuntimeBlocked: false,
} as const;

describe('local AI provider scheduler contracts', () => {
  it('accepts idle singleton provider state for one physical device', () => {
    expect(LocalAiProviderSchedulerStatusSchema.parse(idleSchedulerStatus)).toEqual(idleSchedulerStatus);
  });

  it('accepts child-safety priority queue while parent assistant waits on the same runtime', () => {
    const parsed = LocalAiProviderSchedulerStatusSchema.parse(queuedStatus);

    expect(parsed.currentJobClass).toBe('child-safety');
    expect(parsed.queue.childSafetyQueued).toBe(1);
    expect(parsed.queue.parentAssistantQueued).toBe(1);
    expect(parsed.duplicateRuntimeBlocked).toBe(true);
  });

  it('accepts unavailable lifecycle only with a typed unavailable reason', () => {
    expect(LocalAiProviderSchedulerStatusSchema.parse(unavailableStatus).unavailableReason).toBe(
      'local-ai-provider-unconfigured'
    );
  });

  it('rejects unavailable lifecycle without unavailable reason', () => {
    expect(
      LocalAiProviderSchedulerStatusSchema.safeParse({
        ...idleSchedulerStatus,
        lifecycleState: 'unavailable',
        degradedState: 'provider-unavailable',
      }).success
    ).toBe(false);
  });

  it('rejects idle lifecycle when a queued parent report remains', () => {
    expect(LocalAiProviderSchedulerStatusSchema.safeParse(idleStatusWithQueuedReport).success).toBe(false);
  });

  it('accepts queued decision with selected singleton runtime and queue position', () => {
    expect(LocalAiProviderSchedulerDecisionSchema.parse(queuedDecision)).toEqual(queuedDecision);
  });

  it('rejects queued decision that does not block duplicate runtime loading', () => {
    expect(LocalAiProviderSchedulerDecisionSchema.safeParse(queuedDecisionWithoutRuntimeLaneBlock).success).toBe(false);
  });

  it('rejects running decision that does not prove singleton runtime ownership', () => {
    expect(LocalAiProviderSchedulerDecisionSchema.safeParse(runningDecisionWithoutRuntimeLaneBlock).success).toBe(
      false
    );
  });

  it('rejects unavailable decision without a reason', () => {
    expect(LocalAiProviderSchedulerDecisionSchema.safeParse(unavailableDecisionWithoutReason).success).toBe(false);
  });
});
