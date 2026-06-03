import { describe, expect, it } from 'vitest';
import {
  BrowserActiveProofSource,
  BrowserActiveTabState,
  BrowserBoundaryState,
  BrowserCapabilityStatus,
  BrowserChannel,
  BrowserCustodyLabel,
  BrowserEvidenceReadModelSchema,
  BrowserEvidenceSchemaVersion,
  BrowserExactUrlClaimState,
  BrowserFamily,
  BrowserInterventionAction,
  BrowserInterventionCapabilityState,
  BrowserInterventionDecisionSource,
  BrowserInterventionMechanism,
  BrowserInterventionOutcome,
  BrowserInterventionReadModelSchema,
  BrowserInterventionSchemaVersion,
  BrowserInterventionTargetType,
  BrowserQueryVisibilityLabel,
  BrowserUnmanagedDetectionState,
  BrowserUnmanagedEnforcementState,
  BrowserUnmanagedFallbackActionState,
} from '../src/browser';

describe('browser evidence read model contracts', () => {
  it('accepts browser evidence read models with tab-list-only active certainty', () => {
    const parsed = BrowserEvidenceReadModelSchema.safeParse({
      schemaVersion: BrowserEvidenceSchemaVersion,
      generatedAt: '2026-05-21T01:00:01Z',
      limit: 10,
      returned: 1,
      latestEventId: 'activity-url-observed-1',
      latestObservedAt: '2026-05-21T01:00:00Z',
      capabilityStatus: BrowserCapabilityStatus.TabListOnly,
      custodyLabel: BrowserCustodyLabel.ChildDeviceLocal,
      queryVisibility: BrowserQueryVisibilityLabel.LiveLocal,
      rows: [browserTabEvidence()],
    });

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.rows[0].activeState).toBe('unknown');
      expect(parsed.data.rows[0].capabilityStatus).toBe('tab-list-only');
    }
  });
});

describe('browser intervention read model contracts', () => {
  it('accepts managed browser intervention read models without coupling to a decision engine', () => {
    const parsed = BrowserInterventionReadModelSchema.safeParse({
      schemaVersion: BrowserInterventionSchemaVersion,
      generatedAt: '2026-05-21T01:01:01Z',
      limit: 10,
      returned: 1,
      latestEventId: 'activity-browser-intervention-applied-1',
      latestObservedAt: '2026-05-21T01:01:00Z',
      managedSessionInterventionCapability: BrowserInterventionCapabilityState.Ready,
      unmanagedBrowserEnforcement: BrowserUnmanagedEnforcementState.RequiresOsAppControl,
      rows: [browserInterventionRow()],
    });

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.rows[0].decisionSource).toBe('parent-rule');
      expect(parsed.data.rows[0].interventionMechanism).toBe('chromium-cdp-fetch');
      expect(parsed.data.rows[0].browserBoundaryState).toBe('managed-session');
      expect(parsed.data.rows[0].exactUrlClaimState).toBe('exact-url-proven');
      expect(parsed.data.rows[0].unmanagedFallbackAction).toBe('unavailable');
      expect(parsed.data.unmanagedBrowserEnforcement).toBe('requires-os-app-control');
    }
  });

  it('accepts unmanaged browser intervention evidence without exact URL claims', () => {
    const parsed = BrowserInterventionReadModelSchema.safeParse({
      schemaVersion: BrowserInterventionSchemaVersion,
      generatedAt: '2026-05-21T01:02:01Z',
      limit: 10,
      returned: 1,
      latestEventId: 'activity-browser-intervention-applied-2',
      latestObservedAt: '2026-05-21T01:02:00Z',
      managedSessionInterventionCapability: BrowserInterventionCapabilityState.NeedsManagedSession,
      unmanagedBrowserEnforcement: BrowserUnmanagedEnforcementState.ReadyToBlock,
      rows: [unmanagedBrowserInterventionRow()],
    });

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.rows[0].managedBrowserSessionId).toBeNull();
      expect(parsed.data.rows[0].requestedUrl).toBeNull();
      expect(parsed.data.rows[0].observedUrl).toBeNull();
      expect(parsed.data.rows[0].browserBoundaryState).toBe('unmanaged-browser-process');
      expect(parsed.data.rows[0].exactUrlClaimState).toBe('not-claimed');
      expect(parsed.data.rows[0].unmanagedDetectionState).toBe('terminated');
      expect(parsed.data.rows[0].unmanagedFallbackAction).toBe('terminate-process');
    }
  });

  it('does not overclaim exact URL proof when browser intervention fields are omitted', () => {
    const legacyRow = browserInterventionRow();
    delete (legacyRow as Partial<typeof legacyRow>).browserBoundaryState;
    delete (legacyRow as Partial<typeof legacyRow>).exactUrlClaimState;
    delete (legacyRow as Partial<typeof legacyRow>).unmanagedDetectionState;

    const parsed = BrowserInterventionReadModelSchema.safeParse({
      schemaVersion: BrowserInterventionSchemaVersion,
      generatedAt: '2026-05-21T01:03:01Z',
      limit: 10,
      returned: 1,
      latestEventId: 'activity-browser-intervention-applied-3',
      latestObservedAt: '2026-05-21T01:03:00Z',
      managedSessionInterventionCapability: BrowserInterventionCapabilityState.Ready,
      unmanagedBrowserEnforcement: BrowserUnmanagedEnforcementState.RequiresOsAppControl,
      rows: [legacyRow],
    });

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.rows[0].browserBoundaryState).toBe('unknown');
      expect(parsed.data.rows[0].exactUrlClaimState).toBe('not-claimed');
      expect(parsed.data.rows[0].unmanagedDetectionState).toBe('unavailable');
    }
  });
});

function browserTabEvidence() {
  return {
    schemaVersion: BrowserEvidenceSchemaVersion,
    browserEvidenceId: 'browser-evidence-1',
    observedAt: '2026-05-21T01:00:00Z',
    freshUntil: '2026-05-21T01:00:30Z',
    sourceId: 'managed-chromium-devtools',
    adapterId: 'managed-chromium-devtools-adapter',
    deviceId: 'local-dev-agent',
    browserFamily: BrowserFamily.Chrome,
    browserChannel: BrowserChannel.Stable,
    managedBrowserSessionId: 'managed-browser-session-1',
    profileId: 'managed-profile-child',
    processId: 4242,
    windowId: null,
    tabId: null,
    targetId: 'target-1',
    activeState: BrowserActiveTabState.Unknown,
    activeProofSource: BrowserActiveProofSource.TargetListOnly,
    url: 'https://example.test/learn',
    origin: 'https://example.test',
    domain: 'example.test',
    title: 'Example learning page',
    capabilityStatus: BrowserCapabilityStatus.TabListOnly,
    degradedReason: null,
    staleAt: '2026-05-21T01:00:30Z',
    custodyLabel: BrowserCustodyLabel.ChildDeviceLocal,
    queryVisibility: BrowserQueryVisibilityLabel.LiveLocal,
  };
}

function browserInterventionRow() {
  return {
    schemaVersion: BrowserInterventionSchemaVersion,
    browserInterventionId: 'browser-intervention-1',
    observedAt: '2026-05-21T01:01:00Z',
    sourceId: 'managed-browser-intervention',
    deviceId: 'local-dev-agent',
    browserFamily: BrowserFamily.Chrome,
    browserChannel: BrowserChannel.Stable,
    managedBrowserSessionId: 'managed-browser-session-1',
    profileId: 'managed-profile-child',
    processId: 4242,
    policyDecisionId: 'policy-decision-1',
    decisionSource: BrowserInterventionDecisionSource.ParentRule,
    interventionAction: BrowserInterventionAction.Block,
    interventionTargetType: BrowserInterventionTargetType.Video,
    interventionTargetValue: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ',
    requestedUrl: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ',
    observedUrl: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ',
    interventionMechanism: BrowserInterventionMechanism.ChromiumCdpFetch,
    interventionOutcome: BrowserInterventionOutcome.Blocked,
    browserBoundaryState: BrowserBoundaryState.ManagedSession,
    exactUrlClaimState: BrowserExactUrlClaimState.ExactUrlProven,
    unmanagedDetectionState: BrowserUnmanagedDetectionState.None,
    reason: 'parent-rule-blocked-video',
    custodyLabel: BrowserCustodyLabel.ChildDeviceLocal,
    queryVisibility: BrowserQueryVisibilityLabel.LiveLocal,
  };
}

function unmanagedBrowserInterventionRow() {
  return {
    schemaVersion: BrowserInterventionSchemaVersion,
    browserInterventionId: 'browser-intervention-2',
    observedAt: '2026-05-21T01:02:00Z',
    sourceId: 'managed-browser-intervention',
    deviceId: 'local-dev-agent',
    browserFamily: BrowserFamily.Chrome,
    browserChannel: BrowserChannel.Stable,
    managedBrowserSessionId: null,
    profileId: null,
    processId: 5150,
    policyDecisionId: 'policy-decision-2',
    decisionSource: BrowserInterventionDecisionSource.ParentRule,
    interventionAction: BrowserInterventionAction.Block,
    interventionTargetType: BrowserInterventionTargetType.BrowserProcess,
    interventionTargetValue: 'chrome.exe',
    requestedUrl: null,
    observedUrl: null,
    interventionMechanism: BrowserInterventionMechanism.OsAppControl,
    interventionOutcome: BrowserInterventionOutcome.Blocked,
    browserBoundaryState: BrowserBoundaryState.UnmanagedBrowserProcess,
    exactUrlClaimState: BrowserExactUrlClaimState.NotClaimed,
    unmanagedDetectionState: BrowserUnmanagedDetectionState.Terminated,
    unmanagedFallbackAction: BrowserUnmanagedFallbackActionState.TerminateProcess,
    reason: 'managed-browser-unmanaged-process',
    custodyLabel: BrowserCustodyLabel.ChildDeviceLocal,
    queryVisibility: BrowserQueryVisibilityLabel.LiveLocal,
  };
}
