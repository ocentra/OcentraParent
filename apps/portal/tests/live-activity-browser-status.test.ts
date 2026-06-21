import { describe, expect, it } from 'vitest';
import { AgentEventEnvelopeSchema } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { resolveLiveActivityState } from '../src/live-activity-state';

type PortalBrowserInterventionReadModel = NonNullable<
  ReturnType<typeof resolveLiveActivityState>['browserInterventionReadModel']
>;
type PortalBrowserInterventionRow = PortalBrowserInterventionReadModel['rows'][number];

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
    expect(state.browserManagedStatus?.unmanagedProcessName).toBe('chrome.exe');
    expect(state.browserManagedStatus?.unmanagedProcessKind).toBe('supported-browser');
    expect(state.browserManagedStatus?.unmanagedDetectionConfidence).toBe('high');
    expect(state.browserManagedStatus?.unmanagedDetectionReason).toBe('supported-browser-outside-managed-session');
  });

  it('parses browser inventory read-model events without upgrading unmanaged browser claims', () => {
    const state = resolveLiveActivityState([browserInventoryReadModelEvent()]);
    const latestRow = state.browserInventoryReadModel?.rows[0];

    expect(state.browserInventoryEvent?.event).toBe('agent.browser.inventory.read-model.reported');
    expect(state.browserInventoryReadModel?.returned).toBe(1);
    expect(latestRow?.managementTier).toBe('unmanaged');
    expect(latestRow?.supportTier).toBe('unmanaged-process-only');
    expect(latestRow?.exactUrlCapability).toBe('not-claimed');
    expect(latestRow?.activeTabCapability).toBe('not-claimed');
    expect(latestRow?.unmanagedFallbackCapability).toBe('report-only');
    expect(latestRow?.publisherSignatureRef).toBe('chrome-publisher-signature-ref');
    expect(latestRow?.fileHashRef).toBe('chrome-file-hash-ref');
    expect(latestRow?.reasonCode).toBe('managed-browser-unmanaged-process');
  });

  it('parses browser intervention readiness and latest decision-neutral row', () => {
    const state = resolveLiveActivityState([browserInterventionEvent()]);

    expectBrowserInterventionReadModel(state.browserInterventionReadModel);
  });
});

function expectBrowserInterventionReadModel(
  readModel: ReturnType<typeof resolveLiveActivityState>['browserInterventionReadModel']
) {
  expect(readModel).not.toBeNull();
  if (readModel === null) {
    return;
  }
  expect(readModel.managedSessionInterventionCapability).toBe('ready');
  expect(readModel.unmanagedBrowserEnforcement).toBe('requires-os-app-control');
  expect(readModel.unmanagedFallbackAction).toBe('unavailable');
  expectBrowserInterventionLatestRow(readModel.rows[0]);
}

function expectBrowserInterventionLatestRow(latestRow: PortalBrowserInterventionRow | undefined) {
  expect(latestRow?.browserInterventionId).toBe('browser-intervention-1');
  if (latestRow === undefined) {
    return;
  }
  expect(latestRow.decisionSource).toBe('parent-rule');
  expect(latestRow.interventionOutcome).toBe('blocked');
  expect(latestRow.interventionActionId).toBe('browser-intervention-action-1');
  expect(latestRow.interventionAuditId).toBe('browser-intervention-audit-1');
  expect(latestRow.evidenceReferenceIds).toEqual(['browser-evidence-1']);
  expect(latestRow.browserBoundaryState).toBe('managed-session');
  expect(latestRow.exactUrlClaimState).toBe('exact-url-proven');
  expect(latestRow.unmanagedDetectionState).toBe('none');
  expect(latestRow.unmanagedFallbackAction).toBe('unavailable');
  expect(latestRow.childDeliveryState).toBe('block-page-rendered');
}

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
      unmanagedProcessName: 'chrome.exe',
      unmanagedExecutablePathRef: 'windows-browser-executable-redacted',
      unmanagedSignatureRef: null,
      unmanagedProcessHashRef: null,
      unmanagedProcessKind: 'supported-browser',
      unmanagedDetectionConfidence: 'high',
      unmanagedDetectionReason: 'supported-browser-outside-managed-session',
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

function browserInventoryReadModelEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-browser-inventory',
    correlationId: 'cmd-browser-inventory',
    sentAt: '2026-05-21T03:32:01Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: 'agent.browser.inventory.read-model.reported',
    severity: 'warn',
    payload: {
      generatedAt: '2026-05-21T03:32:01Z',
      limit: 20,
      returned: 1,
      latestObservedAt: '2026-05-21T03:32:00Z',
      capabilityStatus: 'unmanaged-browser',
      custodyLabel: 'child-device-local',
      queryVisibility: 'unavailable',
      browserInventoryRowId: 'browser-inventory-unmanaged-chrome',
      scannedAt: '2026-05-21T03:32:00Z',
      browserFamily: 'chrome',
      browserChannel: 'stable',
      productName: 'Chrome',
      browserVersion: null,
      installState: 'candidate-running',
      runningState: 'running-unmanaged',
      managementTier: 'unmanaged',
      supportTier: 'unmanaged-process-only',
      exactUrlCapability: 'not-claimed',
      activeTabCapability: 'not-claimed',
      managedProfileState: 'not-applicable',
      unmanagedFallbackCapability: 'report-only',
      executablePathRef: null,
      publisherSignatureRef: 'chrome-publisher-signature-ref',
      fileHashRef: 'chrome-file-hash-ref',
      profileId: null,
      processId: 5150,
      reason: 'managed-browser-unmanaged-process',
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
      unmanagedFallbackAction: 'unavailable',
      browserInterventionId: 'browser-intervention-1',
      sourceId: 'managed-browser-intervention',
      browserFamily: 'chrome',
      browserChannel: 'stable',
      managedBrowserSessionId: 'managed-browser-session-dev',
      profileId: 'managed-browser-profile-dev',
      processId: 4242,
      browserInterventionActionId: 'browser-intervention-action-1',
      browserInterventionAuditId: 'browser-intervention-audit-1',
      evidenceReferenceIds: 'browser-evidence-1',
      policyDecisionId: 'policy-decision-1',
      decisionSource: 'parent-rule',
      interventionAction: 'block',
      interventionTargetType: 'video',
      interventionTargetValue: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ',
      requestedUrl: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ',
      observedUrl: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ',
      interventionMechanism: 'chromium-cdp-fetch',
      interventionOutcome: 'blocked',
      browserBoundaryState: 'managed-session',
      exactUrlClaimState: 'exact-url-proven',
      unmanagedDetectionState: 'none',
      childDeliveryState: 'block-page-rendered',
      reason: 'parent-rule-blocked-video',
      custodyLabel: 'child-device-local',
      queryVisibility: 'live-local',
    },
    snapshot: null,
  });
}
