import { describe, expect, it } from 'vitest';
import {
  type SocialDashboardUxSnapshot,
  SocialDashboardUxSnapshotSchema,
} from '@ocentra-parent/schema-domain/social-dashboard-ux';

describe('social dashboard UX contracts', () => {
  it('accepts a contract-only parent social dashboard section snapshot', acceptsHonestSnapshot);
  it('rejects missing required dashboard sections', rejectsMissingSection);
  it('rejects rendered UI, runtime, policy, connector, native, and enforcement claims', rejectsRuntimeClaims);
  it('rejects unsupported section status and action upgrades', rejectsUnsupportedPanelUpgrades);
});

function acceptsHonestSnapshot() {
  const parsed = SocialDashboardUxSnapshotSchema.parse(validSnapshot());

  expect(parsed.schemaVersion).toBe('social-dashboard-ux-contract');
  expect(parsed.panels).toHaveLength(7);
  expect(panelState(parsed, 'account-approval-queue')).toEqual({
    status: 'ready-for-review',
    primaryAction: 'open-parent-approval',
  });
  expect(panelState(parsed, 'decision-memory')).toEqual({
    status: 'contract-only',
    primaryAction: 'review-memory-entry',
  });
  expect(panelState(parsed, 'settings-custody')).toEqual({
    status: 'manual-required',
    primaryAction: 'review-settings-custody',
  });
}

function rejectsMissingSection() {
  const snapshot = validSnapshot();

  expect(
    SocialDashboardUxSnapshotSchema.safeParse({
      ...snapshot,
      panels: snapshot.panels.filter((panel) => panel.panelKind !== 'connector-boundaries'),
    }).success
  ).toBe(false);
}

function rejectsRuntimeClaims() {
  const snapshot = validSnapshot();
  const invalidPanels = [
    { renderedUiClaimed: true },
    { notificationClaimed: true },
    { runtimeDataFetchClaimed: true },
    { policyDecisionClaimed: true },
    { nativeAppControlClaimed: true },
    { connectorAuthorizationClaimed: true },
    { enforcementClaimed: true },
  ];

  for (const invalid of invalidPanels) {
    expect(
      SocialDashboardUxSnapshotSchema.safeParse({
        ...snapshot,
        panels: replacePanel(snapshot, 'feed-video-gates', invalid),
      }).success
    ).toBe(false);
  }
}

function rejectsUnsupportedPanelUpgrades() {
  const snapshot = validSnapshot();

  expect(
    SocialDashboardUxSnapshotSchema.safeParse({
      ...snapshot,
      panels: replacePanel(snapshot, 'native-app-capability', {
        status: 'ready-for-review',
        primaryAction: 'review-feed-gate',
      }),
    }).success
  ).toBe(false);

  expect(
    SocialDashboardUxSnapshotSchema.safeParse({
      ...snapshot,
      panels: replacePanel(snapshot, 'decision-memory', {
        status: 'ready-for-review',
      }),
    }).success
  ).toBe(false);
}

function validSnapshot(): SocialDashboardUxSnapshot {
  return {
    schemaVersion: 'social-dashboard-ux-contract',
    familyId: 'family-social-dashboard',
    childProfileId: 'child-social-dashboard',
    generatedAt: '2026-06-03T08:15:00.000Z',
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
  };
}

function panel(
  panelKind: SocialDashboardUxSnapshot['panels'][number]['panelKind'],
  status: SocialDashboardUxSnapshot['panels'][number]['status'],
  primaryAction: SocialDashboardUxSnapshot['panels'][number]['primaryAction'],
  reasons: SocialDashboardUxSnapshot['panels'][number]['reasons']
): SocialDashboardUxSnapshot['panels'][number] {
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
  };
}

function panelSortOrder(panelKind: SocialDashboardUxSnapshot['panels'][number]['panelKind']): number {
  const order = {
    'account-approval-queue': 0,
    'feed-video-gates': 1,
    'native-app-capability': 2,
    'connector-boundaries': 3,
    'decision-memory': 4,
    'settings-custody': 5,
    'manual-required-gaps': 6,
  } as const satisfies Record<SocialDashboardUxSnapshot['panels'][number]['panelKind'], number>;
  return order[panelKind];
}

function panelState(
  snapshot: SocialDashboardUxSnapshot,
  panelKind: SocialDashboardUxSnapshot['panels'][number]['panelKind']
) {
  const panelEntry = snapshot.panels.find((candidate) => candidate.panelKind === panelKind);
  return {
    status: panelEntry?.status,
    primaryAction: panelEntry?.primaryAction,
  };
}

function replacePanel(
  snapshot: SocialDashboardUxSnapshot,
  panelKind: SocialDashboardUxSnapshot['panels'][number]['panelKind'],
  overrides: Partial<SocialDashboardUxSnapshot['panels'][number]>
) {
  return snapshot.panels.map((panelEntry) =>
    panelEntry.panelKind === panelKind ? { ...panelEntry, ...overrides } : panelEntry
  );
}
