import { describe, expect, it } from 'vitest';
import { AgentEvent, AgentProtocolDefaults, type AgentEventEnvelope } from '../../src/contracts';
import { AgentProtocolSchemaVersion } from '../../src/primitives';
import { parseAgentSocialDashboardReadModelEvent } from '../../src/social-dashboard-read-model';

const Source = {
  peerId: 'agent-service',
  role: 'agent-service',
} as const;

const Target = {
  peerId: 'portal-dev',
  role: 'portal',
} as const;

const SocialDashboardSnapshot = {
  schemaVersion: 'social-dashboard-ux-contract',
  familyId: 'family-social-dashboard',
  childProfileId: 'child-social-dashboard',
  generatedAt: '2026-06-06T03:58:00.000Z',
  panels: [
    panel('account-approval-queue', 'ready-for-review', 'open-parent-approval', ['parent-review-needed']),
    panel('feed-video-gates', 'ready-for-review', 'review-feed-gate', ['feed-video-gate-candidate']),
    panel('native-app-capability', 'manual-required', 'review-native-capability', ['native-app-manual-required']),
    panel('connector-boundaries', 'manual-required', 'review-connector-boundary', [
      'connector-boundary-manual-required',
    ]),
    panel('decision-memory', 'contract-only', 'review-memory-entry', ['memory-contract-only']),
    panel('settings-custody', 'manual-required', 'review-settings-custody', ['settings-custody-runtime-gap']),
    panel('manual-required-gaps', 'manual-required', 'manual-review', ['platform-proof-gap']),
  ],
  claimBoundaries: {
    renderedPortalUi: 'not-claimed',
    notificationDelivery: 'not-claimed',
    runtimeDataFetch: 'not-claimed',
    policyDecision: 'not-claimed',
    nativeAppControl: 'not-claimed',
    connectorAuthorization: 'not-claimed',
    enforcement: 'not-claimed',
  },
} as const;

describe('agent social dashboard read-model parser', () => {
  it('parses the dedicated social dashboard read-model event payload', () => {
    const parsed = parseAgentSocialDashboardReadModelEvent(
      socialDashboardEvent(JSON.stringify(SocialDashboardSnapshot))
    );

    expect(parsed).toEqual({
      ok: true,
      value: SocialDashboardSnapshot,
    });
  });

  it('rejects wrong events, invalid json, and hidden runtime claims', () => {
    expect(
      parseAgentSocialDashboardReadModelEvent({
        ...socialDashboardEvent(JSON.stringify(SocialDashboardSnapshot)),
        event: AgentEvent.HealthReported,
      })
    ).toEqual({
      ok: false,
      reason: 'wrong-event',
    });
    expect(parseAgentSocialDashboardReadModelEvent(socialDashboardEvent('{'))).toEqual({
      ok: false,
      reason: 'invalid-json',
    });
    expect(
      parseAgentSocialDashboardReadModelEvent(
        socialDashboardEvent(
          JSON.stringify({
            ...SocialDashboardSnapshot,
            panels: SocialDashboardSnapshot.panels.map((row) =>
              row.panelKind === 'feed-video-gates' ? { ...row, policyDecisionClaimed: true } : row
            ),
          })
        )
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-payload',
    });
  });
});

function socialDashboardEvent(serializedReadModel: string): AgentEventEnvelope {
  return {
    schemaVersion: AgentProtocolSchemaVersion,
    eventId: 'browser-social-dashboard-event',
    correlationId: 'browser-social-dashboard-command',
    sentAt: '2026-06-06T03:58:01.000Z',
    source: Source,
    target: Target,
    event: AgentEvent.BrowserSocialDashboardReadModelReported,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.BrowserSocialDashboardReadModel]: serializedReadModel,
    },
    snapshot: null,
  };
}

function panel(
  panelKind:
    | 'account-approval-queue'
    | 'feed-video-gates'
    | 'native-app-capability'
    | 'connector-boundaries'
    | 'decision-memory'
    | 'settings-custody'
    | 'manual-required-gaps',
  status: 'ready-for-review' | 'manual-required' | 'contract-only',
  primaryAction:
    | 'open-parent-approval'
    | 'review-feed-gate'
    | 'review-native-capability'
    | 'review-connector-boundary'
    | 'review-memory-entry'
    | 'review-settings-custody'
    | 'manual-review',
  reasons: readonly (
    | 'parent-review-needed'
    | 'feed-video-gate-candidate'
    | 'native-app-manual-required'
    | 'connector-boundary-manual-required'
    | 'memory-contract-only'
    | 'settings-custody-runtime-gap'
    | 'platform-proof-gap'
  )[]
) {
  return {
    panelId: `social-dashboard-${panelKind}`,
    panelKind,
    status,
    primaryAction,
    severity: status === 'manual-required' ? 'warning' : 'info',
    sortOrder: panelSortOrder(panelKind),
    sourceEvidenceRefs: [`parent-evidence-${panelKind}`],
    reasons,
    renderedUiClaimed: false,
    notificationClaimed: false,
    runtimeDataFetchClaimed: false,
    policyDecisionClaimed: false,
    nativeAppControlClaimed: false,
    connectorAuthorizationClaimed: false,
    enforcementClaimed: false,
  } as const;
}

function panelSortOrder(
  panelKind:
    | 'account-approval-queue'
    | 'feed-video-gates'
    | 'native-app-capability'
    | 'connector-boundaries'
    | 'decision-memory'
    | 'settings-custody'
    | 'manual-required-gaps'
): number {
  return {
    'account-approval-queue': 0,
    'feed-video-gates': 1,
    'native-app-capability': 2,
    'connector-boundaries': 3,
    'decision-memory': 4,
    'settings-custody': 5,
    'manual-required-gaps': 6,
  }[panelKind];
}
