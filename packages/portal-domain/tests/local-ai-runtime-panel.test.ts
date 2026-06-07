import { describe, expect, it } from 'vitest';
import {
  AgentEvent,
  AgentEventEnvelopeSchema,
  AgentProtocolDefaults,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import { createLocalAiRuntimePanelIntent, PortalDetails } from '../src/contracts';

describe('local AI runtime panel intent', () => {
  it('renders runtime and household job rows from real agent event envelopes', () => {
    const intent = createLocalAiRuntimePanelIntent(localAiRuntimeStatusEvent(), lanAiJobEvent());

    expect(intent.title).toBe('AI jobs and runtime activity');
    expect(intent.summaryDetails).toContainEqual({
      label: PortalDetails.Status,
      value: 'reported',
    });
    expect(intent.summaryDetails).toContainEqual({
      label: PortalDetails.ProductClaim,
      value: 'no-model-quality-or-enforcement-claim',
    });
    expect(intent.cards.map((card) => card.title)).toEqual(['Local AI runtime status', 'Household AI job activity']);
    expect(intent.cards[0]?.details).toContainEqual({
      label: PortalDetails.Model,
      value: 'screen-local-vlm-v1',
    });
    expect(intent.cards[1]?.details).toContainEqual({
      label: PortalDetails.Status,
      value: 'claimed',
    });
  });

  it('keeps missing runtime/job events visible as no-data rather than success', () => {
    const intent = createLocalAiRuntimePanelIntent(null, null);

    expect(intent.cards).toEqual([]);
    expect(intent.emptyMessage).toBe('No local AI runtime or job event has been reported yet.');
    expect(intent.summaryDetails).toContainEqual({
      label: PortalDetails.Status,
      value: 'not-reported',
    });
  });
});

function localAiRuntimeStatusEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-local-ai-runtime',
    correlationId: 'cmd-local-ai-runtime',
    sentAt: '2026-06-07T19:15:00Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: AgentEvent.LocalAiRuntimeStatusReported,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.LocalAiRuntimeReferenceId]: 'runtime-child-device-1',
      [AgentProtocolDefaults.Field.LocalAiProviderId]: 'local-provider-1',
      [AgentProtocolDefaults.Field.LocalAiModelId]: 'screen-local-vlm-v1',
      [AgentProtocolDefaults.Field.LoadState]: 'loaded',
      [AgentProtocolDefaults.Field.LocalAiCapabilityFlags]: 'ocr,vision',
      [AgentProtocolDefaults.Field.LocalAiResourceClass]: 'gpu',
      [AgentProtocolDefaults.Field.LocalAiDegradedState]: 'ready',
      [AgentProtocolDefaults.Field.LocalAiPrivacyMode]: 'local-only',
      [AgentProtocolDefaults.Field.LocalAiExecutionState]: 'ready',
    },
    snapshot: null,
  });
}

function lanAiJobEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-lan-ai-job',
    correlationId: 'cmd-lan-ai-job',
    sentAt: '2026-06-07T19:15:02Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: AgentEvent.LanAiJobReported,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.LanAiJobId]: 'lan-ai-job-1',
      [AgentProtocolDefaults.Field.LanAiJobStatus]: 'claimed',
      [AgentProtocolDefaults.Field.LanAiJobState]: 'worker-running',
      [AgentProtocolDefaults.Field.LocalAiProviderId]: 'household-desktop-provider',
      [AgentProtocolDefaults.Field.LanAiProviderCustodyLabel]: 'local-lan-redacted',
      [AgentProtocolDefaults.Field.LocalAiExecutionState]: 'running',
    },
    snapshot: null,
  });
}
