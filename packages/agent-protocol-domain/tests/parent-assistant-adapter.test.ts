import { describe, expect, it } from 'vitest';
import { createParentAssistantRuntimeCommand, parseParentAssistantAnswerEvent } from '../src/parent-assistant-adapter';
import { AgentEvent, AgentProtocolDefaults } from '../src/contracts';

const Source = {
  peerId: 'portal-dev',
  role: 'portal',
} as const;

const Target = {
  deviceId: 'local-dev-agent',
  platform: 'windows',
  route: 'localhost',
} as const;

describe('parent assistant adapter boundary', () => {
  it('creates runtime commands for portal message and action-preview handoff', () => {
    const message = createParentAssistantRuntimeCommand('message-send', commandInput());
    const preview = createParentAssistantRuntimeCommand('action-preview', commandInput());

    expect(message.command).toBe('agent.parent-assistant.message.send');
    expect(preview.command).toBe('agent.parent-assistant.action.preview');
    expect(message.payload[AgentProtocolDefaults.Field.ParentAssistantQuestion]).toBe(
      'Suggest a policy rule from recent activity.'
    );
    expect(message.payload[AgentProtocolDefaults.Field.ParentAssistantEvidenceSummary]).toBe(
      'Recent Activity evidence is available.'
    );
  });

  it('parses full answer payloads with citations, preview, and API custody boundary', () => {
    const parsed = parseParentAssistantAnswerEvent(
      eventEnvelope(AgentEvent.ParentAssistantAnswerReported, {
        [AgentProtocolDefaults.Field.ParentAssistantAnswer]: JSON.stringify(answerPayload()),
      })
    );

    expect(parsed.ok).toBe(true);
    expect(parsed.ok ? parsed.value.answerState : null).toBe('unavailable');
    expect(parsed.ok ? parsed.value.citations.length : 0).toBe(1);
    expect(parsed.ok ? parsed.value.actionPreview.enforcementApplied : true).toBe(false);
    expect(parsed.ok ? parsed.value.apiProviderBoundary.authorizationState : null).toBe('not-authorized');
  });

  it('rejects wrong events and invalid answer JSON', () => {
    const wrong = parseParentAssistantAnswerEvent(eventEnvelope(AgentEvent.HealthReported, {}));
    const invalid = parseParentAssistantAnswerEvent(
      eventEnvelope(AgentEvent.ParentAssistantAnswerReported, {
        [AgentProtocolDefaults.Field.ParentAssistantAnswer]: '{',
      })
    );

    expect(wrong.ok).toBe(false);
    expect(invalid.ok).toBe(false);
  });
});

function commandInput() {
  return {
    messageId: 'cmd-parent-assistant-1',
    sentAt: '2026-05-28T14:55:00Z',
    source: Source,
    target: Target,
    requestId: 'parent-assistant-request-local',
    question: 'Suggest a policy rule from recent activity.',
    evidenceSummary: 'Recent Activity evidence is available.',
    maxOutputTokens: 120,
    timeoutMs: 1000,
  } as const;
}

function answerPayload() {
  const citation = {
    evidence: {
      evidenceReferenceId: 'activityDigest',
      kind: 'query-store-summary',
      observedAt: '2026-05-28T14:55:00Z',
    },
    citationLabel: 'Recent activity',
    allowedSummary: 'Recent Activity evidence is available.',
  };

  return {
    schemaVersion: 'v0.6',
    requestId: 'parent-assistant-request-local',
    threadId: 'parent-assistant-thread-local',
    messageId: 'parent-assistant-message-local',
    answeredAt: '2026-05-28T14:55:01Z',
    providerId: 'local-llama-cli',
    modelId: 'gemma-4-default',
    providerState: 'unavailable',
    answerState: 'unavailable',
    schedulerJobStatus: 'unavailable',
    degradedState: 'provider-unavailable',
    unavailableReason: 'local-ai-runtime-unconfigured',
    localAiResultId: null,
    answerText: null,
    citations: [citation],
    actionPreview: {
      previewId: 'parent-assistant-action-preview-local',
      actionKind: 'policy-suggestion',
      summary:
        'Policy suggestion preview only. Controller lease and child-agent contract execution are required before any rule changes.',
      actionReference: null,
      requiresControllerLease: true,
      childAgentContractRequired: true,
      enforcementApplied: false,
    },
    apiProviderBoundary: {
      schemaVersion: 'v0.6',
      providerId: 'api-provider-not-authorized',
      authorizationState: 'not-authorized',
      custodyLabel: 'parent-authorized-api-ai',
      retentionPolicy: 'no-retention-without-parent-authorization',
      deletionPolicy: 'delete-provider-cache-on-parent-request',
      citations: [citation],
      providerState: 'unavailable',
      unavailableReason: 'api-ai-provider-not-authorized',
      childSafetyOrEnforcementUseAllowed: false,
    },
    promptVersion: 'parent-assistant-local-v1',
  } as const;
}

function eventEnvelope(event: (typeof AgentEvent)[keyof typeof AgentEvent], payload: Record<string, unknown>) {
  return {
    schemaVersion: 1,
    eventId: 'parent-assistant-event-1',
    correlationId: 'cmd-parent-assistant-1',
    sentAt: '2026-05-28T14:55:01Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: Source,
    event,
    severity: 'warn',
    payload,
    snapshot: null,
  } as const;
}
