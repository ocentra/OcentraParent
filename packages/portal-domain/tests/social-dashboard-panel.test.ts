import { describe, expect, it } from 'vitest';
import { createSocialDashboardPanelIntent } from '../src/social-dashboard-panel';

describe('social dashboard portal panel', () => {
  it('maps schema-backed social dashboard rows into a parent-visible intent', () => {
    const intent = createSocialDashboardPanelIntent(validSnapshot());

    expect(intent.title).toBe('Social review');
    expect(intent.summary).toBe('6 social dashboard rows');
    expect(intent.metrics.map((metric) => [metric.label, metric.value])).toContainEqual(['Rows returned', '6']);
    expect(intent.rows.map((row) => row.title)).toEqual([
      'Account approvals',
      'Feed and video route gates',
      'Native app capability',
      'Connected account boundaries',
      'Remembered decisions',
      'Needs manual proof',
    ]);
    expect(rowPairs(intent.rows[0])).toContainEqual(['Status', 'Ready for parent review']);
    expect(rowPairs(intent.rows[0])).toContainEqual(['Evidence references', 'parent-evidence-account-approval-queue']);
    expect(rowPairs(intent.rows[2])).toContainEqual(['Status', 'Manual proof required']);
    expect(rowPairs(intent.rows[4])).toContainEqual(['Status', 'Contract proof only']);
    expect(intent.productClaim).toContain('policy execution, and enforcement remain unclaimed');
  });

  it('renders invalid or absent input as unavailable without inventing rows', () => {
    const intent = createSocialDashboardPanelIntent({ rows: [] });

    expect(intent.state).toBe('unavailable');
    expect(intent.summary).toBe('0 social dashboard rows');
    expect(intent.rows).toEqual([]);
    expect(intent.emptyMessage).toBe('No social dashboard snapshot has been reported yet.');
    expect(intent.metrics.map((metric) => [metric.label, metric.value])).toContainEqual(['Status', 'not reported']);
  });

  it('rejects snapshots that try to claim runtime UI authority', () => {
    const snapshot = validSnapshot();
    const intent = createSocialDashboardPanelIntent({
      ...snapshot,
      panels: snapshot.panels.map((panel) =>
        panel.panelKind === 'feed-video-gates' ? { ...panel, enforcementClaimed: true } : panel
      ),
    });

    expect(intent.rows).toEqual([]);
    expect(intent.productClaim).toContain('enforcement remain unclaimed');
  });
});

function rowPairs(row: ReturnType<typeof createSocialDashboardPanelIntent>['rows'][number]) {
  return row.details.map((detail) => [detail.label, detail.value]);
}

function validSnapshot() {
  return {
    schemaVersion: 'social-dashboard-ux-contract',
    familyId: 'family-social-dashboard',
    childProfileId: 'child-social-dashboard',
    generatedAt: '2026-06-05T22:12:00.000Z',
    panels: [
      panel('account-approval-queue', 'ready-for-review', 'open-parent-approval', ['parent-review-needed']),
      panel('feed-video-gates', 'ready-for-review', 'review-feed-gate', ['feed-video-gate-candidate']),
      panel('native-app-capability', 'manual-required', 'review-native-capability', ['native-app-manual-required']),
      panel('connector-boundaries', 'manual-required', 'review-connector-boundary', [
        'connector-boundary-manual-required',
      ]),
      panel('decision-memory', 'contract-only', 'review-memory-entry', ['memory-contract-only']),
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
}

function panel(
  panelKind:
    | 'account-approval-queue'
    | 'feed-video-gates'
    | 'native-app-capability'
    | 'connector-boundaries'
    | 'decision-memory'
    | 'manual-required-gaps',
  status: 'ready-for-review' | 'manual-required' | 'contract-only',
  primaryAction:
    | 'open-parent-approval'
    | 'review-feed-gate'
    | 'review-native-capability'
    | 'review-connector-boundary'
    | 'review-memory-entry'
    | 'manual-review',
  reasons: readonly (
    | 'parent-review-needed'
    | 'feed-video-gate-candidate'
    | 'native-app-manual-required'
    | 'connector-boundary-manual-required'
    | 'memory-contract-only'
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

function panelSortOrder(panelKind: ReturnType<typeof validSnapshot>['panels'][number]['panelKind']): number {
  return {
    'account-approval-queue': 0,
    'feed-video-gates': 1,
    'native-app-capability': 2,
    'connector-boundaries': 3,
    'decision-memory': 4,
    'manual-required-gaps': 5,
  }[panelKind];
}
