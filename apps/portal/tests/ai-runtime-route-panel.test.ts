import { describe, expect, it } from 'vitest';
import {
  AgentEvent,
  AgentEventEnvelopeSchema,
  AgentProtocolDefaults,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import { PortalRoute } from '@ocentra-parent/portal-domain/contracts';
import { shouldRenderAiRuntimeRoute } from '../src/AiRuntimeRoutePanel';
import { resolveLiveActivityState } from '../src/live-activity-state';

describe('AI runtime route panel', () => {
  it('renders only on the AI runtime route', () => {
    expect(shouldRenderAiRuntimeRoute(PortalRoute.AiRuntime)).toBe(true);
    expect(shouldRenderAiRuntimeRoute(PortalRoute.Memory)).toBe(false);
    expect(shouldRenderAiRuntimeRoute(PortalRoute.Overview)).toBe(false);
  });

  it('selects real local AI runtime and household AI job events from live state', () => {
    const state = resolveLiveActivityState([localAiRuntimeStatusEvent(), lanAiJobEvent()]);

    expect(state.localAiRuntimeStatusEvent?.event).toBe(AgentEvent.LocalAiRuntimeStatusReported);
    expect(state.localAiRuntimeStatusEvent?.payload[AgentProtocolDefaults.Field.LocalAiModelId]).toBe(
      'screen-local-vlm-v1'
    );
    expect(state.lanAiJobEvent?.event).toBe(AgentEvent.LanAiJobReported);
    expect(state.lanAiJobEvent?.payload[AgentProtocolDefaults.Field.LanAiJobStatus]).toBe('claimed');
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
    },
    snapshot: null,
  });
}
