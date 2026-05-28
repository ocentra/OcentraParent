import { describe, expect, it } from 'vitest';
import {
  ParentAssistantActionPreviewSchema,
  ParentAssistantAnswerSchema,
  ParentAssistantApiProviderBoundarySchema,
  ParentAssistantGenerateRequestSchema,
} from '../src/parent-assistant';
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
