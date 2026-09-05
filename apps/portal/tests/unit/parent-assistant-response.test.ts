import { describe, expect, it } from 'vitest';
import {
  ParentAgentEvent as AgentEvent,
  ParentAgentProtocolField as ProtocolField,
  type ParentRouteEventSnapshot,
} from '../../generated/parent-ui-bridge';
import { latestParentAssistantResponse } from '@ocentra-parent/portal-domain/parent-portal-data';

describe('parent assistant response projection', () => {
  it('projects the newest real answer text from the service event buffer', () => {
    const response = latestParentAssistantResponse([
      event('answer-2', AgentEvent.ParentAssistantAnswerReported, {
        [ProtocolField.ParentAssistantAnswerState]: 'answered',
        [ProtocolField.ParentAssistantAnswerText]: 'The current report has two manual-required items.',
      }),
      event('answer-1', AgentEvent.ParentAssistantAnswerReported, {
        [ProtocolField.ParentAssistantAnswerState]: 'answered',
        [ProtocolField.ParentAssistantAnswerText]: 'Older answer',
      }),
    ]);

    expect(response).toEqual({
      eventId: 'answer-2',
      kind: 'answer',
      state: 'answered',
      text: 'The current report has two manual-required items.',
    });
  });

  it('projects provider and error failures without fabricating an answer', () => {
    const unavailable = latestParentAssistantResponse([
      event('degraded-1', AgentEvent.ParentAssistantProviderDegraded, {
        [ProtocolField.LocalAiUnavailableReason]: 'No local provider is configured.',
      }),
    ]);
    const failed = latestParentAssistantResponse([
      event('error-1', AgentEvent.ParentAssistantErrorReported, {
        [ProtocolField.Reason]: 'The authenticated command route closed.',
      }),
    ]);

    expect(unavailable).toMatchObject({ kind: 'unavailable', text: 'No local provider is configured.' });
    expect(failed).toMatchObject({ kind: 'error', text: 'The authenticated command route closed.' });
  });

  it('ignores unrelated or identity-less events', () => {
    const response = latestParentAssistantResponse([
      event('', AgentEvent.ParentAssistantAnswerReported, {
        [ProtocolField.ParentAssistantAnswerText]: 'must not render',
      }),
      event('health-1', AgentEvent.HealthReported, {
        [ProtocolField.Message]: 'healthy',
      }),
    ]);

    expect(response).toBeNull();
  });
});

function event(
  eventId: string,
  eventName: NonNullable<ParentRouteEventSnapshot['event']>,
  payload: Readonly<Record<string, unknown>>
): ParentRouteEventSnapshot {
  return {
    event: eventName,
    eventId,
    correlationId: `correlation-${eventId || 'missing'}`,
    sentAt: '2026-08-30T00:00:00.000Z',
    sourcePeerId: 'agent-service',
    sourceRole: 'agent-service',
    targetPeerId: 'portal',
    targetRole: 'portal',
    severity: 'info',
    payload,
    snapshot: null,
  };
}
