import { describe, expect, it } from 'vitest';
import {
  ParentAssistantActionPreviewSchema,
  ParentAssistantActionPreviewResultSchema,
  ParentAssistantActionConfirmResultSchema,
  ParentAssistantAnswerSchema,
  ParentAssistantApiAuthorizationContextSchema,
  ParentAssistantApiProviderBoundarySchema,
  ParentAssistantGenerateRequestSchema,
  ParentAssistantProviderStatusSchema,
  ParentAssistantRunCancelResultSchema,
  ParentAssistantThreadResponseSchema,
} from '../../src/parent-assistant';
import { ParentAssistantRunStateSchema } from '../../src/parent-assistant-run-state';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
  ParentPlatform,
} from '../../src/reference-primitives';

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
  custodyLabel: 'parent-owned-activity-summary',
  sourceLabel: 'activity-query-store-summary',
  rawChildEvidenceIncluded: false,
  directEnforcementAllowed: false,
} as const;

const SourceRefs = [EvidenceContext.evidence] as const;

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
  accessState: 'not-authorized',
  parentAuthorizationRequired: true,
  evidenceCitationRequired: true,
  custodyLabel: 'parent-authorized-api-ai',
  custodyState: 'parent-owned-citations-only',
  retentionPolicy: 'no-retention-without-parent-authorization',
  retentionState: 'no-retention-without-parent-authorization',
  deletionPolicy: 'delete-provider-cache-on-parent-request',
  deletionState: 'delete-provider-cache-on-parent-request',
  citations: [EvidenceContext],
  providerState: 'unavailable',
  unavailableReason: 'api-ai-provider-not-authorized',
  childSafetyOrEnforcementUseAllowed: false,
} as const;

describe('parent assistant request contracts', () => {
  it('ParentAssistantGenerateRequestSchema: accepts cited evidence context for a local provider request', () => {
    const parsed = ParentAssistantGenerateRequestSchema.parse(Request);

    expect(parsed.evidenceContext[0]?.citationLabel).toBe('Activity summary 1');
    expect(parsed.evidenceContext[0]?.rawChildEvidenceIncluded).toBe(false);
    expect(parsed.evidenceContext[0]?.directEnforcementAllowed).toBe(false);
    expect(parsed.scope.device?.deviceId).toBe('child-device-1');
  });

  it('ParentAssistantGenerateRequestSchema: rejects raw child evidence in MIA context', () => {
    expect(
      ParentAssistantGenerateRequestSchema.safeParse({
        ...Request,
        evidenceContext: [
          {
            ...EvidenceContext,
            rawChildEvidenceIncluded: true,
          },
        ],
      }).success
    ).toBe(false);
  });

  it('ParentAssistantGenerateRequestSchema: rejects empty parent questions', () => {
    expect(
      ParentAssistantGenerateRequestSchema.safeParse({
        ...Request,
        question: '',
      }).success
    ).toBe(false);
  });
});

describe('parent assistant action preview contracts', () => {
  it('ParentAssistantActionPreviewSchema: forbids direct enforcement output', () => {
    expect(ParentAssistantActionPreviewSchema.parse(ActionPreview).enforcementApplied).toBe(false);
    expect(
      ParentAssistantActionPreviewSchema.safeParse({
        ...ActionPreview,
        enforcementApplied: true,
      }).success
    ).toBe(false);
  });

  it('ParentAssistantActionPreviewResultSchema: keeps action drafts non-enforcing and unwritten', () => {
    const parsed = ParentAssistantActionPreviewResultSchema.parse({
      schemaVersion: ParentContractSchemaVersion.V0_6,
      backendState: 'runtime-backed',
      actionIntentId: 'parent-assistant-action-intent-1',
      previewState: 'draft',
      preview: ActionPreview,
      evidenceContext: [EvidenceContext],
      previewRequired: true,
      previewSatisfied: true,
      rawAssistantProseAccepted: false,
      parentConfirmationRequired: true,
      parentConfirmationRecorded: false,
      childAgentValidationState: 'child-agent-contract-required',
      sourceRefs: SourceRefs,
      auditReason: 'Preview generated from cited parent-owned Activity evidence.',
      requiresControllerLease: true,
      childAgentContractRequired: true,
      enforcementApplied: false,
      policyWritten: false,
      reason: 'Preview only. Controller lease and child-agent contract are required before applying this action.',
    });

    expect(parsed.previewState).toBe('draft');
    expect(parsed.evidenceContext[0]?.citationLabel).toBe('Activity summary 1');
    expect(parsed.policyWritten).toBe(false);
    expect(
      ParentAssistantActionPreviewResultSchema.safeParse({
        ...parsed,
        policyWritten: true,
      }).success
    ).toBe(false);
  });

  it('ParentAssistantActionPreviewResultSchema: rejects raw assistant prose as action input', () => {
    expect(
      ParentAssistantActionPreviewResultSchema.safeParse({
        schemaVersion: ParentContractSchemaVersion.V0_6,
        backendState: 'runtime-backed',
        actionIntentId: 'parent-assistant-action-intent-1',
        previewState: 'draft',
        preview: ActionPreview,
        evidenceContext: [EvidenceContext],
        previewRequired: true,
        previewSatisfied: true,
        rawAssistantProseAccepted: true,
        parentConfirmationRequired: true,
        parentConfirmationRecorded: false,
        childAgentValidationState: 'child-agent-contract-required',
        sourceRefs: SourceRefs,
        auditReason: 'Preview generated from cited parent-owned Activity evidence.',
        requiresControllerLease: true,
        childAgentContractRequired: true,
        enforcementApplied: false,
        policyWritten: false,
        reason: 'Preview only. Controller lease and child-agent contract are required before applying this action.',
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
      providerRoute: providerRoute('configured', 'local-provider-ready', 'local'),
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
        providerRoute: providerRoute('configured', 'local-provider-ready', 'local'),
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
      providerRoute: providerRoute('unavailable', 'no-provider-available', 'none'),
      promptVersion: 'parent-assistant-local-v1',
    });

    expect(parsed.unavailableReason).toBe('local-ai-provider-unconfigured');
  });
});

describe('parent assistant API authorization context contracts', () => {
  it('ParentAssistantApiAuthorizationContextSchema: requires explicit parent custody and retention proof', () => {
    const parsed = ParentAssistantApiAuthorizationContextSchema.parse({
      authorizationState: 'authorized',
      parentAuthorizationRequired: true,
      evidenceCitationRequired: true,
      custodyLabel: 'parent-authorized-api-ai',
      retentionState: 'parent-authorized-no-default-retention',
      deletionState: 'delete-provider-cache-on-parent-request',
    });

    expect(parsed.authorizationState).toBe('authorized');
    expect(
      ParentAssistantApiAuthorizationContextSchema.safeParse({
        ...parsed,
        retentionState: 'no-retention-without-parent-authorization',
      }).success
    ).toBe(false);
  });
});

describe('parent assistant API provider boundary contracts', () => {
  it('ParentAssistantApiProviderBoundarySchema: accepts not-authorized unavailable API state with custody and deletion rules', () => {
    const parsed = ParentAssistantApiProviderBoundarySchema.parse(ApiProviderBoundary);

    expect(parsed.accessState).toBe('not-authorized');
    expect(parsed.providerState).toBe('unavailable');
    expect(parsed.parentAuthorizationRequired).toBe(true);
    expect(parsed.evidenceCitationRequired).toBe(true);
    expect(parsed.deletionState).toBe('delete-provider-cache-on-parent-request');
    expect(parsed.childSafetyOrEnforcementUseAllowed).toBe(false);
  });

  it('ParentAssistantApiProviderBoundarySchema: accepts authorized degraded API state without safety decision use', () => {
    const parsed = ParentAssistantApiProviderBoundarySchema.parse({
      ...ApiProviderBoundary,
      providerId: 'api-provider-authorized',
      authorizationState: 'authorized',
      accessState: 'authorized-degraded',
      retentionState: 'parent-authorized-no-default-retention',
      providerState: 'degraded',
      unavailableReason: 'api-ai-provider-authorized-degraded',
    });

    expect(parsed.authorizationState).toBe('authorized');
    expect(parsed.accessState).toBe('authorized-degraded');
    expect(parsed.childSafetyOrEnforcementUseAllowed).toBe(false);
  });

  it('ParentAssistantApiProviderBoundarySchema: rejects API use for child safety or enforcement decisions', () => {
    expect(
      ParentAssistantApiProviderBoundarySchema.safeParse({
        schemaVersion: ParentContractSchemaVersion.V0_6,
        providerId: 'api-provider-not-configured',
        authorizationState: 'not-authorized',
        accessState: 'not-authorized',
        parentAuthorizationRequired: true,
        evidenceCitationRequired: true,
        custodyLabel: 'parent-authorized-api-ai',
        custodyState: 'parent-owned-citations-only',
        retentionPolicy: 'no-retention-without-parent-authorization',
        retentionState: 'no-retention-without-parent-authorization',
        deletionPolicy: 'delete-provider-cache-on-parent-request',
        deletionState: 'delete-provider-cache-on-parent-request',
        citations: [EvidenceContext],
        providerState: 'unavailable',
        unavailableReason: 'api-ai-provider-not-authorized',
        childSafetyOrEnforcementUseAllowed: true,
      }).success
    ).toBe(false);
  });

  it('ParentAssistantApiProviderBoundarySchema: rejects authorized API state without citations', () => {
    expect(
      ParentAssistantApiProviderBoundarySchema.safeParse({
        ...ApiProviderBoundary,
        providerId: 'api-provider-authorized',
        authorizationState: 'authorized',
        accessState: 'authorized-unavailable',
        retentionState: 'parent-authorized-no-default-retention',
        citations: [],
      }).success
    ).toBe(false);
  });
});

describe('parent assistant backend thread contracts', () => {
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
});

describe('parent assistant provider status route contracts', () => {
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
      providerRoute: providerRoute('unavailable', 'no-provider-available', 'none'),
    });

    expect(parsed.apiProviderBoundary.authorizationState).toBe('not-authorized');
    expect(parsed.busy).toBe(false);
    expect(parsed.providerRoute.selectedProvider).toBe('none');
    expect(parsed.providerRoute.routingState).toBe('no-provider-available');
  });

  it('ParentAssistantProviderStatusSchema: rejects provider routes that allow child-safety use', () => {
    expect(
      ParentAssistantProviderStatusSchema.safeParse({
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
        providerRoute: {
          ...providerRoute('unavailable', 'no-provider-available', 'none'),
          childSafetyOrEnforcementUseAllowed: true,
        },
      }).success
    ).toBe(false);
  });

  it('ParentAssistantProviderStatusSchema: rejects provider routes that do not match API custody boundary', () => {
    expect(
      ParentAssistantProviderStatusSchema.safeParse({
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
        providerRoute: {
          ...providerRoute('unavailable', 'no-provider-available', 'none'),
          routingState: 'api-provider-authorized-degraded',
          apiProviderState: 'degraded',
          apiAccessState: 'authorized-degraded',
        },
      }).success
    ).toBe(false);
  });
});

describe('parent assistant runtime result contracts', () => {
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
      previewRequired: true,
      previewSatisfied: true,
      rawAssistantProseAccepted: false,
      parentConfirmationRequired: true,
      parentConfirmationRecorded: false,
      childAgentValidationState: 'child-agent-contract-required',
      sourceRefs: SourceRefs,
      auditReason: 'Parent confirmation cannot write policy until child-agent validation is wired.',
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

  it('ParentAssistantActionConfirmResultSchema: accepts rejected confirmation when preview is missing', () => {
    const parsed = ParentAssistantActionConfirmResultSchema.parse({
      schemaVersion: ParentContractSchemaVersion.V0_6,
      backendState: 'contract-required',
      actionIntentId: 'parent-assistant-action-intent-1',
      previewId: null,
      actionKind: 'policy-suggestion',
      confirmState: 'rejected',
      previewRequired: true,
      previewSatisfied: false,
      rawAssistantProseAccepted: false,
      parentConfirmationRequired: true,
      parentConfirmationRecorded: false,
      childAgentValidationState: 'child-agent-unavailable',
      sourceRefs: SourceRefs,
      auditReason: 'Preview is required before parent confirmation.',
      requiresControllerLease: true,
      childAgentContractRequired: true,
      enforcementApplied: false,
      policyWritten: false,
      reason: 'Action confirmation rejected because no matching preview id was provided.',
    });

    expect(parsed.confirmState).toBe('rejected');
    expect(parsed.previewSatisfied).toBe(false);
    expect(parsed.policyWritten).toBe(false);
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

function providerRoute(
  localProviderState: 'configured' | 'degraded' | 'unavailable',
  routingState:
    | 'local-provider-ready'
    | 'local-provider-degraded'
    | 'local-provider-unavailable'
    | 'api-provider-authorized-unavailable'
    | 'api-provider-authorized-degraded'
    | 'no-provider-available',
  selectedProvider: 'local' | 'api' | 'none'
) {
  return {
    routingState,
    selectedProvider,
    localProviderState,
    apiProviderState: ApiProviderBoundary.providerState,
    apiAccessState: ApiProviderBoundary.accessState,
    evidenceCitationRequired: true,
    remoteAiOptional: true,
    childSafetyOrEnforcementUseAllowed: false,
    reason: 'Local provider routing keeps API AI optional and outside child safety decisions.',
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
