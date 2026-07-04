import { ParentAgentEvent as AgentEvent, type ParentRouteEventSnapshot } from '../../generated/parent-ui-bridge';

export function emptyBrowserEvidenceEvent(): ParentRouteEventSnapshot {
  return {
    eventId: 'evt-browser',
    correlationId: 'cmd-browser',
    sentAt: '2026-05-21T01:00:01Z',
    sourcePeerId: 'local-dev-agent',
    sourceRole: 'agent-service',
    targetPeerId: 'portal-dev',
    targetRole: 'portal',
    event: AgentEvent.BrowserEvidenceRecentReported,
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
  };
}

export function unavailableRecentSummaryEvent(): ParentRouteEventSnapshot {
  return {
    eventId: 'evt-recent',
    correlationId: 'cmd-recent',
    sentAt: '2026-05-20T18:45:01Z',
    sourcePeerId: 'local-dev-agent',
    sourceRole: 'agent-service',
    targetPeerId: 'portal-dev',
    targetRole: 'portal',
    event: AgentEvent.ActivityRecentSummaryReported,
    severity: 'error',
    payload: {
      reason: 'Activity store is unavailable.',
    },
    snapshot: null,
  };
}
