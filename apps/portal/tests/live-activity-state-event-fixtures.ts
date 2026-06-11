import { AgentEventEnvelopeSchema } from '@ocentra-parent/agent-protocol-domain/contracts';

export function emptyBrowserEvidenceEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-browser',
    correlationId: 'cmd-browser',
    sentAt: '2026-05-21T01:00:01Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: 'agent.browser.evidence.recent.reported',
    severity: 'info',
    payload: {
      generatedAt: '2026-05-21T01:00:01Z',
      limit: 10,
      returned: 0,
      latestEventId: null,
      latestObservedAt: null,
      browserEvidenceId: null,
      sourceId: null,
      adapterId: null,
      managedBrowserSessionId: null,
      browserFamily: null,
      activeState: null,
      activeProofSource: null,
      url: null,
      origin: null,
      domain: null,
      title: null,
      capabilityStatus: null,
      custodyLabel: null,
      queryVisibility: null,
    },
    snapshot: null,
  });
}

export function unavailableRecentSummaryEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-recent',
    correlationId: 'cmd-recent',
    sentAt: '2026-05-20T18:45:01Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: 'agent.activity.recent.summary.reported',
    severity: 'error',
    payload: {
      reason: 'Activity store is unavailable.',
    },
    snapshot: null,
  });
}
