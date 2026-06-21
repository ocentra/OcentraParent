import { describe, expect, it } from 'vitest';
import {
  type BrowserGameDashboardUxSnapshot,
  BrowserGameDashboardUxSnapshotSchema,
} from '@ocentra-parent/schema-domain/browser-game-dashboard-ux';

describe('browser-game dashboard UX contracts', () => {
  it('accepts a contract-only parent browser-game dashboard section snapshot', acceptsHonestSnapshot);
  it('rejects missing required dashboard sections', rejectsMissingSection);
  it('rejects rendered UI, runtime fetch, policy, native, cloud-frame, and enforcement claims', rejectsRuntimeClaims);
  it('rejects unsupported section status and action upgrades', rejectsUnsupportedPanelUpgrades);
  it('rejects approval and mobile panels without required refs', rejectsMissingRequiredRefs);
});

function acceptsHonestSnapshot() {
  const parsed = BrowserGameDashboardUxSnapshotSchema.parse(validSnapshot());

  expect(parsed.schemaVersion).toBe('browser-game-dashboard-ux-contract');
  expect(parsed.panels).toHaveLength(7);
  expect(panelState(parsed, 'detected-game-review')).toEqual({
    status: 'ready-for-review',
    primaryAction: 'review-detected-game',
  });
  expect(panelState(parsed, 'educational-game-allowlist')).toEqual({
    status: 'contract-only',
    primaryAction: 'review-educational-allowlist',
  });
}

function rejectsMissingSection() {
  const snapshot = validSnapshot();

  expect(
    BrowserGameDashboardUxSnapshotSchema.safeParse({
      ...snapshot,
      panels: snapshot.panels.filter((panel) => panel.panelKind !== 'manual-required-gaps'),
    }).success
  ).toBe(false);
}

function rejectsRuntimeClaims() {
  const snapshot = validSnapshot();
  const invalidPanels = [
    { renderedPortalUiClaimed: true },
    { notificationClaimed: true },
    { runtimeDataFetchClaimed: true },
    { finalPolicyDecisionClaimed: true },
    { cloudFrameAnalysisClaimed: true },
    { nativeGameControlClaimed: true },
    { enforcementClaimed: true },
  ];

  for (const invalid of invalidPanels) {
    expect(
      BrowserGameDashboardUxSnapshotSchema.safeParse({
        ...snapshot,
        panels: replacePanel(snapshot, 'detected-game-review', invalid),
      }).success
    ).toBe(false);
  }
}

function rejectsUnsupportedPanelUpgrades() {
  const snapshot = validSnapshot();

  expect(
    BrowserGameDashboardUxSnapshotSchema.safeParse({
      ...snapshot,
      panels: replacePanel(snapshot, 'cloud-gaming-approval', {
        status: 'ready-for-review',
        primaryAction: 'review-detected-game',
      }),
    }).success
  ).toBe(false);

  expect(
    BrowserGameDashboardUxSnapshotSchema.safeParse({
      ...snapshot,
      panels: replacePanel(snapshot, 'educational-game-allowlist', {
        status: 'ready-for-review',
      }),
    }).success
  ).toBe(false);
}

function rejectsMissingRequiredRefs() {
  const snapshot = validSnapshot();

  expect(
    BrowserGameDashboardUxSnapshotSchema.safeParse({
      ...snapshot,
      panels: replacePanel(snapshot, 'unknown-game-approval-queue', {
        approvalRequestRef: null,
      }),
    }).success
  ).toBe(false);

  expect(
    BrowserGameDashboardUxSnapshotSchema.safeParse({
      ...snapshot,
      panels: replacePanel(snapshot, 'mobile-native-capability-gaps', {
        mobileCapabilityRef: null,
      }),
    }).success
  ).toBe(false);
}

function validSnapshot(): BrowserGameDashboardUxSnapshot {
  return {
    schemaVersion: 'browser-game-dashboard-ux-contract',
    familyId: 'family-browser-game-dashboard',
    childProfileId: 'child-browser-game-dashboard',
    generatedAt: '2026-06-03T09:40:00.000Z',
    panels: [
      panel('detected-game-review', 'ready-for-review', 'review-detected-game', ['detected-game-evidence-ready']),
      panel('unknown-game-approval-queue', 'ready-for-review', 'open-parent-approval', [
        'unknown-game-parent-review-needed',
      ]),
      panel('cloud-gaming-approval', 'manual-required', 'review-cloud-gaming', ['cloud-gaming-manual-required']),
      panel('educational-game-allowlist', 'contract-only', 'review-educational-allowlist', [
        'educational-allowlist-contract-only',
      ]),
      panel('game-time-budget-candidates', 'contract-only', 'review-time-budget', ['time-budget-candidate-only']),
      panel('mobile-native-capability-gaps', 'manual-required', 'review-mobile-capability', [
        'mobile-native-proof-gap',
      ]),
      panel('manual-required-gaps', 'manual-required', 'manual-review', ['platform-proof-gap']),
    ],
    claimBoundaries: {
      renderedPortalUi: 'not-claimed',
      notificationDelivery: 'not-claimed',
      runtimeDataFetch: 'not-claimed',
      finalPolicyDecision: 'not-claimed',
      cloudFrameAnalysis: 'not-claimed',
      nativeGameControl: 'not-claimed',
      enforcement: 'not-claimed',
    },
  };
}

function panel(
  panelKind: BrowserGameDashboardUxSnapshot['panels'][number]['panelKind'],
  status: BrowserGameDashboardUxSnapshot['panels'][number]['status'],
  primaryAction: BrowserGameDashboardUxSnapshot['panels'][number]['primaryAction'],
  reasons: BrowserGameDashboardUxSnapshot['panels'][number]['reasons']
): BrowserGameDashboardUxSnapshot['panels'][number] {
  return {
    panelId: `browser-game-dashboard-${panelKind}`,
    panelKind,
    status,
    primaryAction,
    severity: status === 'manual-required' ? 'warning' : 'info',
    sortOrder: panelSortOrder(panelKind),
    sourceEvidenceRefs: [`parent-evidence-${panelKind}`],
    approvalRequestRef: panelKind === 'unknown-game-approval-queue' ? 'approval-request-unknown-game' : null,
    policyCandidateRef: panelKind === 'game-time-budget-candidates' ? 'policy-candidate-game-time' : null,
    mobileCapabilityRef: panelKind === 'mobile-native-capability-gaps' ? 'mobile-capability-game-gap' : null,
    reasons,
    renderedPortalUiClaimed: false,
    notificationClaimed: false,
    runtimeDataFetchClaimed: false,
    finalPolicyDecisionClaimed: false,
    cloudFrameAnalysisClaimed: false,
    nativeGameControlClaimed: false,
    enforcementClaimed: false,
  };
}

function panelSortOrder(panelKind: BrowserGameDashboardUxSnapshot['panels'][number]['panelKind']): number {
  const order = {
    'detected-game-review': 0,
    'unknown-game-approval-queue': 1,
    'cloud-gaming-approval': 2,
    'educational-game-allowlist': 3,
    'game-time-budget-candidates': 4,
    'mobile-native-capability-gaps': 5,
    'manual-required-gaps': 6,
  } as const satisfies Record<BrowserGameDashboardUxSnapshot['panels'][number]['panelKind'], number>;
  return order[panelKind];
}

function panelState(
  snapshot: BrowserGameDashboardUxSnapshot,
  panelKind: BrowserGameDashboardUxSnapshot['panels'][number]['panelKind']
) {
  const panelEntry = snapshot.panels.find((candidate) => candidate.panelKind === panelKind);
  return {
    status: panelEntry?.status,
    primaryAction: panelEntry?.primaryAction,
  };
}

function replacePanel(
  snapshot: BrowserGameDashboardUxSnapshot,
  panelKind: BrowserGameDashboardUxSnapshot['panels'][number]['panelKind'],
  overrides: Partial<BrowserGameDashboardUxSnapshot['panels'][number]>
) {
  return snapshot.panels.map((panelEntry) =>
    panelEntry.panelKind === panelKind ? { ...panelEntry, ...overrides } : panelEntry
  );
}
