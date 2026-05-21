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
