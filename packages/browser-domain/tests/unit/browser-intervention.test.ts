import { describe, expect, it } from 'vitest';
import {
  BrowserCustodyLabel,
  BrowserQueryVisibilityLabel,
} from '@ocentra-parent/schema-domain/browser-values';
import {
  BrowserBoundaryState,
  BrowserExactUrlClaimState,
  BrowserInterventionAction,
  BrowserInterventionCapabilityState,
  BrowserInterventionDecisionSource,
  BrowserInterventionDeliveryState,
  BrowserInterventionMechanism,
  BrowserInterventionOutcome,
  BrowserInterventionReadModelSchema,
  BrowserInterventionSchemaVersion,
  BrowserInterventionTargetType,
  BrowserUnmanagedDetectionState,
  BrowserUnmanagedEnforcementState,
  BrowserUnmanagedFallbackActionState,
} from '@ocentra-parent/schema-domain/browser-intervention-schemas';

describe('browser intervention contract', () => {
  registerManagedInterventionCases();
  registerUnmanagedFallbackCases();
  registerInvalidInterventionCases();
});

function registerManagedInterventionCases() {
  it('accepts managed social and browser-game intervention rows with action audit and evidence refs', () => {
    const parsed = BrowserInterventionReadModelSchema.safeParse({
      schemaVersion: BrowserInterventionSchemaVersion,
      generatedAt: '2026-06-02T22:59:00Z',
      limit: 10,
      returned: 2,
      latestEventId: 'activity-browser-intervention-applied-2',
      latestObservedAt: '2026-06-02T22:58:30Z',
      managedSessionInterventionCapability: BrowserInterventionCapabilityState.Ready,
      unmanagedBrowserEnforcement: BrowserUnmanagedEnforcementState.RequiresOsAppControl,
      rows: [
        interventionRow({
          browserInterventionId: 'browser-intervention-social-feed',
          interventionAction: BrowserInterventionAction.Warn,
          interventionTargetType: BrowserInterventionTargetType.SocialShortVideoFeed,
          interventionMechanism: BrowserInterventionMechanism.ManagedBlockPage,
          interventionOutcome: BrowserInterventionOutcome.Warned,
          childDeliveryState: BrowserInterventionDeliveryState.WarnPageRendered,
        }),
        interventionRow({
          browserInterventionId: 'browser-intervention-game-purchase',
          interventionAction: BrowserInterventionAction.ApprovalHold,
          interventionTargetType: BrowserInterventionTargetType.GamePurchase,
          interventionMechanism: BrowserInterventionMechanism.ApprovalHoldPage,
          interventionOutcome: BrowserInterventionOutcome.ApprovalRequired,
          childDeliveryState: BrowserInterventionDeliveryState.ApprovalHoldRendered,
        }),
      ],
    });

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.rows[0]?.interventionTargetType).toBe('social-short-video-feed');
      expect(parsed.data.rows[0]?.childDeliveryState).toBe('warn-page-rendered');
      expect(parsed.data.rows[1]?.interventionTargetType).toBe('game-purchase');
      expect(parsed.data.rows[1]?.interventionActionId).toBe('browser-action-1');
      expect(parsed.data.rows[1]?.interventionAuditId).toBe('browser-audit-1');
      expect(parsed.data.rows[1]?.evidenceReferenceIds).toEqual(['browser-evidence-1']);
    }
  });
}

function registerUnmanagedFallbackCases() {
  it('accepts unmanaged fallback actions without exact URL claims', () => {
    const parsed = BrowserInterventionReadModelSchema.safeParse({
      schemaVersion: BrowserInterventionSchemaVersion,
      generatedAt: '2026-06-02T23:15:00Z',
      limit: 10,
      returned: 2,
      latestEventId: 'activity-browser-intervention-applied-4',
      latestObservedAt: '2026-06-02T23:14:30Z',
      managedSessionInterventionCapability: BrowserInterventionCapabilityState.NeedsManagedSession,
      unmanagedBrowserEnforcement: BrowserUnmanagedEnforcementState.TerminateProcess,
      unmanagedFallbackAction: BrowserUnmanagedFallbackActionState.TerminateProcess,
      rows: [
        unmanagedFallbackRow({
          browserInterventionId: 'browser-intervention-unmanaged-social',
          interventionAction: BrowserInterventionAction.AskParent,
          interventionTargetType: BrowserInterventionTargetType.SocialAccountCreation,
          unmanagedFallbackAction: BrowserUnmanagedFallbackActionState.AskParent,
        }),
        unmanagedFallbackRow({
          browserInterventionId: 'browser-intervention-unmanaged-game',
          interventionAction: BrowserInterventionAction.RelaunchManaged,
          interventionOutcome: BrowserInterventionOutcome.RelaunchStarted,
          interventionTargetType: BrowserInterventionTargetType.CloudGaming,
          unmanagedFallbackAction: BrowserUnmanagedFallbackActionState.RelaunchManagedBrowser,
        }),
      ],
    });

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.unmanagedFallbackAction).toBe('terminate-process');
      expect(parsed.data.rows[0]?.requestedUrl).toBeNull();
      expect(parsed.data.rows[0]?.exactUrlClaimState).toBe('not-claimed');
      expect(parsed.data.rows[0]?.unmanagedFallbackAction).toBe('parent-review');
      expect(parsed.data.rows[1]?.unmanagedFallbackAction).toBe('relaunch-managed-browser');
    }
  });
}

function registerInvalidInterventionCases() {
  it('rejects intervention rows with invalid delivery state', () => {
    const parsed = BrowserInterventionReadModelSchema.safeParse({
      schemaVersion: BrowserInterventionSchemaVersion,
      generatedAt: '2026-06-02T22:59:00Z',
      limit: 10,
      returned: 1,
      latestEventId: 'activity-browser-intervention-applied-1',
      latestObservedAt: '2026-06-02T22:58:00Z',
      managedSessionInterventionCapability: BrowserInterventionCapabilityState.Ready,
      unmanagedBrowserEnforcement: BrowserUnmanagedEnforcementState.RequiresOsAppControl,
      rows: [
        {
          ...interventionRow({}),
          childDeliveryState: 'silently-blocked',
        },
      ],
    });

    expect(parsed.success).toBe(false);
  });

  it('rejects intervention rows with invalid unmanaged fallback action', () => {
    const parsed = BrowserInterventionReadModelSchema.safeParse({
      schemaVersion: BrowserInterventionSchemaVersion,
      generatedAt: '2026-06-02T23:15:00Z',
      limit: 10,
      returned: 1,
      latestEventId: 'activity-browser-intervention-applied-5',
      latestObservedAt: '2026-06-02T23:14:30Z',
      managedSessionInterventionCapability: BrowserInterventionCapabilityState.NeedsManagedSession,
      unmanagedBrowserEnforcement: BrowserUnmanagedEnforcementState.ReportOnly,
      unmanagedFallbackAction: 'recover-exact-url',
      rows: [unmanagedFallbackRow({})],
    });

    expect(parsed.success).toBe(false);
  });

  it('rejects unmanaged fallback rows that attach exact URL evidence', () => {
    const parsed = BrowserInterventionReadModelSchema.safeParse({
      schemaVersion: BrowserInterventionSchemaVersion,
      generatedAt: '2026-06-02T23:15:00Z',
      limit: 10,
      returned: 1,
      latestEventId: 'activity-browser-intervention-applied-6',
      latestObservedAt: '2026-06-02T23:14:30Z',
      managedSessionInterventionCapability: BrowserInterventionCapabilityState.NeedsManagedSession,
      unmanagedBrowserEnforcement: BrowserUnmanagedEnforcementState.AskParent,
      unmanagedFallbackAction: BrowserUnmanagedFallbackActionState.AskParent,
      rows: [
        unmanagedFallbackRow({
          requestedUrl: 'https://social.example.test/signup',
          observedUrl: 'https://social.example.test/signup',
          exactUrlClaimState: BrowserExactUrlClaimState.ExactUrlProven,
        }),
      ],
    });

    expect(parsed.success).toBe(false);
  });
}

function unmanagedFallbackRow(overrides: Partial<Record<string, unknown>>): Record<string, unknown> {
  return interventionRow({
    browserFamily: 'chrome',
    browserChannel: 'stable',
    managedBrowserSessionId: null,
    profileId: null,
    processId: 4242,
    interventionAction: BrowserInterventionAction.TerminateProcess,
    interventionTargetType: BrowserInterventionTargetType.BrowserProcess,
    interventionTargetValue: 'chrome.exe',
    requestedUrl: null,
    observedUrl: null,
    interventionMechanism: BrowserInterventionMechanism.OsAppControl,
    interventionOutcome: BrowserInterventionOutcome.Terminated,
    browserBoundaryState: BrowserBoundaryState.UnmanagedBrowserProcess,
    exactUrlClaimState: BrowserExactUrlClaimState.NotClaimed,
    unmanagedDetectionState: BrowserUnmanagedDetectionState.Terminated,
    unmanagedFallbackAction: BrowserUnmanagedFallbackActionState.TerminateProcess,
    childDeliveryState: BrowserInterventionDeliveryState.ManualRequired,
    reason: 'managed-browser-unmanaged-process',
    ...overrides,
  });
}

function interventionRow(overrides: Partial<Record<string, unknown>>): Record<string, unknown> {
  return {
    schemaVersion: BrowserInterventionSchemaVersion,
    browserInterventionId: 'browser-intervention-1',
    observedAt: '2026-06-02T22:58:00Z',
    sourceId: 'managed-browser-intervention',
    deviceId: 'local-dev-agent',
    browserFamily: 'chrome',
    browserChannel: 'stable',
    managedBrowserSessionId: 'managed-browser-session-dev',
    profileId: 'managed-browser-profile-dev',
    processId: 4242,
    interventionActionId: 'browser-action-1',
    interventionAuditId: 'browser-audit-1',
    evidenceReferenceIds: ['browser-evidence-1'],
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
    childDeliveryState: BrowserInterventionDeliveryState.BlockPageRendered,
    reason: 'parent-rule-blocked-video',
    custodyLabel: BrowserCustodyLabel.ChildDeviceLocal,
    queryVisibility: BrowserQueryVisibilityLabel.LiveLocal,
    ...overrides,
  };
}
