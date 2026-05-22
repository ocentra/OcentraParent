import { describe, expect, it } from 'vitest';
import {
  BrowserActiveTabState,
  BrowserCapabilityStatus,
  BrowserChannel,
  BrowserCustodyLabel,
  BrowserEvidenceReadModelSchema,
  BrowserEvidenceSchemaVersion,
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
  BrowserUnmanagedEnforcementState,
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
      expect(parsed.data.unmanagedBrowserEnforcement).toBe('requires-os-app-control');
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
    reason: 'parent-rule-blocked-video',
    custodyLabel: BrowserCustodyLabel.ChildDeviceLocal,
    queryVisibility: BrowserQueryVisibilityLabel.LiveLocal,
  };
}
