import { describe, expect, it } from 'vitest';
import { AgentEventEnvelopeSchema } from '@ocentra-parent/agent-protocol-domain/contracts';
import { resolveLiveActivityState } from '../src/live-activity-state';

describe('portal live browser status', () => {
  it('parses managed browser status without exposing raw bridge endpoints', () => {
    const state = resolveLiveActivityState([browserManagedStatusEvent()]);

    expect(state.browserManagedStatus?.managedState).toBe('bridge-connected');
    expect(state.browserManagedStatus?.bridgeEndpointRef).toBe('managed-loopback-devtools-redacted');
    expect(state.browserManagedStatus?.queryVisibility).toBe('live-local');
  });

  it('parses unmanaged browser status as supported install with unmanaged capability', () => {
    const state = resolveLiveActivityState([unmanagedBrowserStatusEvent()]);

    expect(state.browserManagedStatus?.managedBrowserSessionId).toBeNull();
    expect(state.browserManagedStatus?.managedState).toBe('installed-supported');
    expect(state.browserManagedStatus?.capabilityStatus).toBe('unmanaged-browser');
    expect(state.browserManagedStatus?.degradedReason).toBe('managed-browser-unmanaged-process');
  });

  it('parses browser intervention readiness and latest decision-neutral row', () => {
    const state = resolveLiveActivityState([browserInterventionEvent()]);

    expect(state.browserInterventionReadModel?.managedSessionInterventionCapability).toBe('ready');
    expect(state.browserInterventionReadModel?.unmanagedBrowserEnforcement).toBe('requires-os-app-control');
    const latestRow = state.browserInterventionReadModel?.rows[0];
    expect(latestRow?.decisionSource).toBe('parent-rule');
    expect(latestRow?.interventionOutcome).toBe('blocked');
  });
});

function browserManagedStatusEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-browser-managed',
    correlationId: 'cmd-browser-managed',
    sentAt: '2026-05-21T03:30:01Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: 'agent.browser.managed.status.reported',
    severity: 'info',
    payload: {
      checkedAt: '2026-05-21T03:30:00Z',
      managedBrowserSessionId: 'managed-browser-session-dev',
      browserFamily: 'unknown-chromium',
      browserChannel: 'unknown',
      browserVersion: 'Chrome/125.0.0.0',
      profileId: 'managed-browser-profile-dev',
      profilePathRef: 'managed-profile-redacted',
      processId: null,
      bridgeKind: 'chromium-devtools-protocol',
      bridgeEndpointRef: 'managed-loopback-devtools-redacted',
      managedState: 'bridge-connected',
      capabilityStatus: 'tab-list-only',
      reason: null,
      startedAt: null,
      custodyLabel: 'child-device-local',
      queryVisibility: 'live-local',
    },
    snapshot: null,
  });
}

function unmanagedBrowserStatusEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-browser-unmanaged',
    correlationId: 'cmd-browser-managed',
    sentAt: '2026-05-21T03:30:01Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: 'agent.browser.managed.status.reported',
    severity: 'warn',
    payload: {
      checkedAt: '2026-05-21T03:30:00Z',
      managedBrowserSessionId: null,
      browserFamily: 'chrome',
      browserChannel: 'stable',
      browserVersion: null,
      profileId: null,
      profilePathRef: null,
      processId: 5150,
      bridgeKind: null,
      bridgeEndpointRef: null,
      managedState: 'installed-supported',
      capabilityStatus: 'unmanaged-browser',
      reason: 'managed-browser-unmanaged-process',
      startedAt: null,
      custodyLabel: 'child-device-local',
      queryVisibility: 'unavailable',
    },
    snapshot: null,
  });
}

function browserInterventionEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-browser-intervention',
    correlationId: 'cmd-browser-intervention',
    sentAt: '2026-05-21T03:31:01Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: 'agent.browser.intervention.read-model.reported',
    severity: 'info',
    payload: {
      generatedAt: '2026-05-21T03:31:01Z',
      limit: 10,
      returned: 1,
      latestEventId: 'activity-browser-intervention-applied-1',
      latestObservedAt: '2026-05-21T03:31:00Z',
      managedSessionInterventionCapability: 'ready',
      unmanagedBrowserEnforcement: 'requires-os-app-control',
      browserInterventionId: 'browser-intervention-1',
      sourceId: 'managed-browser-intervention',
      browserFamily: 'chrome',
      browserChannel: 'stable',
      managedBrowserSessionId: 'managed-browser-session-dev',
      profileId: 'managed-browser-profile-dev',
      processId: 4242,
      policyDecisionId: 'policy-decision-1',
      decisionSource: 'parent-rule',
      interventionAction: 'block',
      interventionTargetType: 'video',
      interventionTargetValue: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ',
      requestedUrl: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ',
      observedUrl: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ',
      interventionMechanism: 'chromium-cdp-fetch',
      interventionOutcome: 'blocked',
      reason: 'parent-rule-blocked-video',
      custodyLabel: 'child-device-local',
      queryVisibility: 'live-local',
    },
    snapshot: null,
  });
}
