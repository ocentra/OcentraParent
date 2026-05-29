import { describe, expect, it } from 'vitest';
import {
  ParentAssistantActionPreviewSchema,
  ParentAssistantActionConfirmResultSchema,
  ParentAssistantAnswerSchema,
  ParentAssistantApiProviderBoundarySchema,
  ParentAssistantGenerateRequestSchema,
  ParentAssistantProviderStatusSchema,
  ParentAssistantRunCancelResultSchema,
  ParentAssistantThreadResponseSchema,
} from '../src/parent-assistant';
import { ParentAssistantRunStateSchema } from '../src/parent-assistant-run-state';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
  ParentPlatform,
} from '../src/reference-primitives';

const Family = {
  familyId: 'family-local-1',
} as const;

const Device = {
  deviceId: 'child-device-1',
  childProfileId: 'child-profile-1',
  label: 'Kitchen laptop',
  platform: ParentPlatform.Windows,
} as const;

const EvidenceContext = {
  evidence: {
    evidenceReferenceId: 'activity-summary-1',
    kind: ParentEvidenceReferenceKind.QueryStoreSummary,
    observedAt: '2026-05-27T06:30:00Z',
  },
  citationLabel: 'Activity summary 1',
  allowedSummary: 'App use was higher than the daily baseline.',
} as const;

const Request = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  requestId: 'parent-assistant-request-1',
  threadId: 'parent-assistant-thread-1',
  messageId: 'parent-assistant-message-1',
  askedAt: '2026-05-27T06:31:00Z',
  actor: {
    actorId: 'parent-actor-1',
    role: ParentActorRole.Parent,
  },
  scope: {
    family: Family,
    device: Device,
  },
  question: 'Why did app use increase today?',
  evidenceContext: [EvidenceContext],
  modelId: 'local-gguf-chat-model',
  maxOutputTokens: 320,
  timeoutMs: 15000,
} as const;

const ActionPreview = {
  previewId: 'parent-assistant-preview-1',
  actionKind: 'time-limit-change',
  summary: 'Preview a shorter evening game window.',
  actionReference: null,
  requiresControllerLease: true,
  childAgentContractRequired: true,
  enforcementApplied: false,
} as const;

const ApiProviderBoundary = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  providerId: 'api-provider-not-authorized',
  authorizationState: 'not-authorized',
  custodyLabel: 'parent-authorized-api-ai',
  retentionPolicy: 'no-retention-without-parent-authorization',
  deletionPolicy: 'delete-provider-cache-on-parent-request',
  citations: [EvidenceContext],
  providerState: 'unavailable',
  unavailableReason: 'api-ai-provider-not-authorized',
  childSafetyOrEnforcementUseAllowed: false,
} as const;

describe('parent assistant request contracts', () => {
  it('ParentAssistantGenerateRequestSchema: accepts cited evidence context for a local provider request', () => {
    const parsed = ParentAssistantGenerateRequestSchema.parse(Request);

    expect(parsed.evidenceContext[0]?.citationLabel).toBe('Activity summary 1');
    expect(parsed.scope.device?.deviceId).toBe('child-device-1');
  });

  it('ParentAssistantGenerateRequestSchema: rejects empty parent questions', () => {
    expect(
      ParentAssistantGenerateRequestSchema.safeParse({
        ...Request,
        question: '',
      }).success
    ).toBe(false);
  });

  it('ParentAssistantActionPreviewSchema: forbids direct enforcement output', () => {
    expect(ParentAssistantActionPreviewSchema.parse(ActionPreview).enforcementApplied).toBe(false);
    expect(
      ParentAssistantActionPreviewSchema.safeParse({
        ...ActionPreview,
        enforcementApplied: true,
      }).success
    ).toBe(false);
  });
});

describe('parent assistant answer contracts', () => {
  it('ParentAssistantAnswerSchema: requires citations for configured answered state', () => {
    const parsed = ParentAssistantAnswerSchema.parse({
      schemaVersion: ParentContractSchemaVersion.V0_6,
      requestId: 'parent-assistant-request-1',
      threadId: 'parent-assistant-thread-1',
      messageId: 'parent-assistant-message-1',
      answeredAt: '2026-05-27T06:31:02Z',
      providerId: 'local-provider-llama-cli',
      modelId: 'local-gguf-chat-model',
      providerState: 'configured',
      answerState: 'answered',
      runState: 'completed',
      schedulerJobStatus: 'complete',
      degradedState: 'none',
      unavailableReason: null,
      localAiResultId: 'local-ai-result-parent-assistant-request-1',
      answerText: 'App use increased because the recent activity window shows a longer game session.',
      citations: [EvidenceContext],
      actionPreview: ActionPreview,
      apiProviderBoundary: ApiProviderBoundary,
      promptVersion: 'parent-assistant-local-v1',
    });

    expect(parsed.citations[0]?.allowedSummary).toBe('App use was higher than the daily baseline.');
  });

  it('ParentAssistantAnswerSchema: rejects answered state without citations', () => {
    expect(
      ParentAssistantAnswerSchema.safeParse({
        schemaVersion: ParentContractSchemaVersion.V0_6,
        requestId: 'parent-assistant-request-1',
        threadId: 'parent-assistant-thread-1',
        messageId: 'parent-assistant-message-1',
        answeredAt: '2026-05-27T06:31:02Z',
        providerId: 'local-provider-llama-cli',
        modelId: 'local-gguf-chat-model',
        providerState: 'configured',
        answerState: 'answered',
        runState: 'completed',
        schedulerJobStatus: 'complete',
        degradedState: 'none',
        unavailableReason: null,
        localAiResultId: 'local-ai-result-parent-assistant-request-1',
        answerText: 'Answer without evidence should not pass.',
        citations: [],
        actionPreview: ActionPreview,
        apiProviderBoundary: ApiProviderBoundary,
        promptVersion: 'parent-assistant-local-v1',
      }).success
    ).toBe(false);
  });
});

describe('parent assistant unavailable answer contracts', () => {
  it('ParentAssistantAnswerSchema: accepts unavailable local provider state with typed reason', () => {
    const parsed = ParentAssistantAnswerSchema.parse({
      schemaVersion: ParentContractSchemaVersion.V0_6,
      requestId: 'parent-assistant-request-2',
      threadId: 'parent-assistant-thread-1',
      messageId: 'parent-assistant-message-2',
      answeredAt: '2026-05-27T06:32:02Z',
      providerId: 'local-provider-unconfigured',
      modelId: 'safety-model-unconfigured',
      providerState: 'unavailable',
      answerState: 'unavailable',
      runState: 'unavailable',
      schedulerJobStatus: 'unavailable',
      degradedState: 'provider-unavailable',
      unavailableReason: 'local-ai-provider-unconfigured',
      localAiResultId: null,
      answerText: null,
      citations: [],
      actionPreview: {
        previewId: null,
        actionKind: 'none',
        summary: null,
        actionReference: null,
        requiresControllerLease: false,
        childAgentContractRequired: true,
        enforcementApplied: false,
      },
      apiProviderBoundary: ApiProviderBoundary,
      promptVersion: 'parent-assistant-local-v1',
    });

    expect(parsed.unavailableReason).toBe('local-ai-provider-unconfigured');
  });
});

describe('parent assistant API provider boundary contracts', () => {
  it('ParentAssistantApiProviderBoundarySchema: accepts not-authorized unavailable API state with custody and deletion rules', () => {
    const parsed = ParentAssistantApiProviderBoundarySchema.parse(ApiProviderBoundary);

    expect(parsed.providerState).toBe('unavailable');
    expect(parsed.childSafetyOrEnforcementUseAllowed).toBe(false);
  });

  it('ParentAssistantApiProviderBoundarySchema: rejects API use for child safety or enforcement decisions', () => {
    expect(
      ParentAssistantApiProviderBoundarySchema.safeParse({
        schemaVersion: ParentContractSchemaVersion.V0_6,
        providerId: 'api-provider-not-configured',
        authorizationState: 'not-authorized',
        custodyLabel: 'parent-authorized-api-ai',
        retentionPolicy: 'no-retention-without-parent-authorization',
        deletionPolicy: 'delete-provider-cache-on-parent-request',
        citations: [EvidenceContext],
        providerState: 'unavailable',
        unavailableReason: 'api-ai-provider-not-authorized',
        childSafetyOrEnforcementUseAllowed: true,
      }).success
    ).toBe(false);
  });
});

describe('parent assistant backend runtime contracts', () => {
  it('ParentAssistantRunStateSchema: accepts scheduler-backed run states and rejects non-contract states', () => {
    expect(ParentAssistantRunStateSchema.parse('queued')).toBe('queued');
    expect(ParentAssistantRunStateSchema.parse('unavailable')).toBe('unavailable');
    expect(ParentAssistantRunStateSchema.safeParse('blocked').success).toBe(false);
  });

  it('ParentAssistantThreadResponseSchema: accepts durable local thread lifecycle state', () => {
    const parsed = ParentAssistantThreadResponseSchema.parse({
      schemaVersion: ParentContractSchemaVersion.V0_6,
      backendState: 'durable-local',
      activeThread: threadRecord('open'),
      threads: [threadRecord('open')],
      reason: 'Thread state is persisted in the local Parent Assistant store.',
    });

    expect(parsed.activeThread?.state).toBe('open');
    expect(parsed.threads[0]?.backendState).toBe('durable-local');
  });

  it('ParentAssistantProviderStatusSchema: exposes local provider and API custody boundaries', () => {
    const parsed = ParentAssistantProviderStatusSchema.parse({
      schemaVersion: ParentContractSchemaVersion.V0_6,
      backendState: 'runtime-backed',
      providerId: 'local-provider-llama-cli',
      modelId: 'local-gguf-chat-model',
      providerState: 'unavailable',
      runState: 'unavailable',
      schedulerJobStatus: 'unavailable',
      schedulerStatus: schedulerStatus('unavailable'),
      degradedState: 'provider-unavailable',
      unavailableReason: 'local-ai-provider-unconfigured',
      queueDepth: 0,
      busy: false,
      apiProviderBoundary: ApiProviderBoundary,
    });

    expect(parsed.apiProviderBoundary.authorizationState).toBe('not-authorized');
    expect(parsed.busy).toBe(false);
  });

  it('ParentAssistantRunCancelResultSchema: reports no-active-run without pretending to stop a runtime', () => {
    const parsed = ParentAssistantRunCancelResultSchema.parse({
      schemaVersion: ParentContractSchemaVersion.V0_6,
      backendState: 'runtime-backed',
      threadId: 'parent-assistant-thread-1',
      runId: 'parent-assistant-run-1',
      cancelState: 'not-running',
      runState: 'completed',
      providerState: 'unavailable',
      unavailableReason: 'parent-assistant-run-not-running',
    });

    expect(parsed.cancelState).toBe('not-running');
  });

  it('ParentAssistantActionConfirmResultSchema: forbids direct policy writes and enforcement', () => {
    const parsed = ParentAssistantActionConfirmResultSchema.parse({
      schemaVersion: ParentContractSchemaVersion.V0_6,
      backendState: 'contract-required',
      actionIntentId: 'parent-assistant-action-intent-1',
      previewId: 'parent-assistant-preview-1',
      actionKind: 'policy-suggestion',
      confirmState: 'contract-required',
      requiresControllerLease: true,
      childAgentContractRequired: true,
      enforcementApplied: false,
      policyWritten: false,
      reason: 'Controller lease and child-agent policy contract are required before applying this action.',
    });

    expect(parsed.policyWritten).toBe(false);
    expect(
      ParentAssistantActionConfirmResultSchema.safeParse({
        ...parsed,
        enforcementApplied: true,
      }).success
    ).toBe(false);
  });
});

function threadRecord(state: 'open' | 'archived') {
  return {
    schemaVersion: ParentContractSchemaVersion.V0_6,
    threadId: 'parent-assistant-thread-1',
    title: 'Recent activity questions',
    state,
    backendState: 'durable-local',
    createdAt: '2026-05-28T17:20:00Z',
    updatedAt: '2026-05-28T17:20:01Z',
    messageCount: 0,
  } as const;
}

function schedulerStatus(lifecycleState: 'idle' | 'running' | 'queued' | 'degraded' | 'unavailable') {
  return {
    physicalDeviceId: 'physical-device-local',
    singletonScope: 'physical-device',
    providerId: 'local-provider-llama-cli',
    runtimeReferenceId: 'local-runtime-llama-cli',
    modelId: 'local-gguf-chat-model',
    modelReference: 'local-model-reference',
    resourceClass: 'cpu',
    lifecycleState,
    currentJobClass: null,
    queue: {
      childSafetyQueued: 0,
      parentAssistantQueued: 0,
      parentReportQueued: 0,
    },
    duplicateRuntimeBlocked: false,
    degradedState: 'provider-unavailable',
    unavailableReason: 'local-ai-provider-unconfigured',
    lastCheckedAt: '2026-05-28T17:20:01Z',
  } as const;
}
