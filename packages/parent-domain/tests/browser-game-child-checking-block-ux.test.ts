import { describe, expect, it } from 'vitest';
import {
  type BrowserGameChildCheckingBlockUxSnapshot,
  BrowserGameChildCheckingBlockUxSnapshotSchema,
} from '../src/browser-game-child-checking-block-ux';

describe('browser-game child checking and block UX contracts', () => {
  it('accepts a contract-only child game checking and block snapshot', acceptsHonestSnapshot);
  it('rejects missing required browser-game child states', rejectsMissingState);
  it('rejects raw copy, rendered UI, browser block, native, cloud-frame, and enforcement claims', rejectsClaims);
  it('rejects state and text-token mismatches', rejectsTokenMismatch);
  it('rejects unsupported child-facing state upgrades', rejectsUnsupportedStateUpgrades);
});

function acceptsHonestSnapshot() {
  const parsed = BrowserGameChildCheckingBlockUxSnapshotSchema.parse(validSnapshot());

  expect(parsed.schemaVersion).toBe('browser-game-child-checking-block-ux-contract');
  expect(parsed.surfaces).toHaveLength(7);
  expect(surfaceState(parsed, 'checking-unknown-game')).toEqual({
    state: 'checking-contract-only',
    primaryAction: 'wait-for-classification',
    primaryTextToken: 'browser-game.child.checking.title',
  });
  expect(surfaceState(parsed, 'blocked-game-candidate')).toEqual({
    state: 'blocked-contract-only',
    primaryAction: 'open-safe-back',
    primaryTextToken: 'browser-game.child.blocked.title',
  });
}

function rejectsMissingState() {
  const snapshot = validSnapshot();

  expect(
    BrowserGameChildCheckingBlockUxSnapshotSchema.safeParse({
      ...snapshot,
      surfaces: snapshot.surfaces.filter((surface) => surface.surfaceKind !== 'cloud-gaming-manual-required'),
    }).success
  ).toBe(false);
}

function rejectsClaims() {
  const snapshot = validSnapshot();
  const invalidSurfaces = [
    { rawChildCopyClaimed: true },
    { renderedChildUiClaimed: true },
    { notificationDeliveredClaimed: true },
    { browserNavigationBlockedClaimed: true },
    { blockPageRenderedClaimed: true },
    { timeLimitAppliedClaimed: true },
    { finalPolicyDecisionClaimed: true },
    { cloudFrameAnalysisClaimed: true },
    { nativeGameControlClaimed: true },
    { enforcementClaimed: true },
  ];

  for (const invalid of invalidSurfaces) {
    expect(
      BrowserGameChildCheckingBlockUxSnapshotSchema.safeParse({
        ...snapshot,
        surfaces: replaceSurface(snapshot, 'blocked-game-candidate', invalid),
      }).success
    ).toBe(false);
  }
}

function rejectsTokenMismatch() {
  const snapshot = validSnapshot();

  expect(
    BrowserGameChildCheckingBlockUxSnapshotSchema.safeParse({
      ...snapshot,
      surfaces: replaceSurface(snapshot, 'educational-game-allowed', {
        primaryTextToken: 'browser-game.child.blocked.title',
      }),
    }).success
  ).toBe(false);
}

function rejectsUnsupportedStateUpgrades() {
  const snapshot = validSnapshot();

  expect(
    BrowserGameChildCheckingBlockUxSnapshotSchema.safeParse({
      ...snapshot,
      surfaces: replaceSurface(snapshot, 'checking-unknown-game', {
        analysisRef: null,
      }),
    }).success
  ).toBe(false);

  expect(
    BrowserGameChildCheckingBlockUxSnapshotSchema.safeParse({
      ...snapshot,
      surfaces: replaceSurface(snapshot, 'approval-required-game', {
        parentApprovalRequestRef: null,
      }),
    }).success
  ).toBe(false);
}

function validSnapshot(): BrowserGameChildCheckingBlockUxSnapshot {
  return {
    schemaVersion: 'browser-game-child-checking-block-ux-contract',
    familyId: 'family-browser-game-child-ux',
    childProfileId: 'child-browser-game-child-ux',
    deviceId: 'device-browser-game-child-ux',
    generatedAt: '2026-06-03T09:31:00.000Z',
    surfaces: [
      surface(
        'checking-unknown-game',
        'checking-contract-only',
        'wait-for-classification',
        'browser-game.child.checking.title',
        ['unknown-game-needs-classification'],
        { analysisRef: 'browser-game-analysis-unknown-game' }
      ),
      surface(
        'approval-required-game',
        'waiting-parent',
        'wait-for-parent',
        'browser-game.child.approval.title',
        ['parent-approval-needed'],
        { parentApprovalRequestRef: 'browser-game-parent-approval-request' }
      ),
      surface(
        'blocked-game-candidate',
        'blocked-contract-only',
        'open-safe-back',
        'browser-game.child.blocked.title',
        ['game-block-candidate'],
        { policyCandidateRef: 'browser-game-policy-candidate-block' }
      ),
      surface(
        'educational-game-allowed',
        'child-readable',
        'acknowledge',
        'browser-game.child.educational-allowed.title',
        ['educational-game-allowed-contract']
      ),
      surface('game-time-limit-candidate', 'child-readable', 'acknowledge', 'browser-game.child.time-limited.title', [
        'time-limit-not-applied',
      ]),
      surface('cloud-gaming-manual-required', 'manual-required', 'manual-review', 'browser-game.child.manual.title', [
        'cloud-gaming-proof-manual-required',
      ]),
      surface('native-game-control-unavailable', 'unavailable', 'no-action', 'browser-game.child.unavailable.title', [
        'native-game-proof-unavailable',
      ]),
    ],
    claimBoundaries: {
      rawChildCopy: 'not-claimed',
      renderedChildUi: 'not-claimed',
      notificationDelivery: 'not-claimed',
      browserNavigationBlock: 'not-claimed',
      blockPageRender: 'not-claimed',
      timeLimitApply: 'not-claimed',
      finalPolicyDecision: 'not-claimed',
      cloudFrameAnalysis: 'not-claimed',
      nativeGameControl: 'not-claimed',
      enforcement: 'not-claimed',
    },
  };
}

function surface(
  surfaceKind: BrowserGameChildCheckingBlockUxSnapshot['surfaces'][number]['surfaceKind'],
  state: BrowserGameChildCheckingBlockUxSnapshot['surfaces'][number]['state'],
  primaryAction: BrowserGameChildCheckingBlockUxSnapshot['surfaces'][number]['primaryAction'],
  primaryTextToken: BrowserGameChildCheckingBlockUxSnapshot['surfaces'][number]['primaryTextToken'],
  reasons: BrowserGameChildCheckingBlockUxSnapshot['surfaces'][number]['reasons'],
  overrides: Partial<BrowserGameChildCheckingBlockUxSnapshot['surfaces'][number]> = {}
): BrowserGameChildCheckingBlockUxSnapshot['surfaces'][number] {
  return {
    surfaceId: `browser-game-child-ux-${surfaceKind}`,
    surfaceKind,
    state,
    primaryAction,
    primaryTextToken,
    sourceEvidenceRefs: [`parent-evidence-${surfaceKind}`],
    gameEvidenceRef: `browser-game-evidence-${surfaceKind}`,
    analysisRef: null,
    policyCandidateRef: null,
    parentApprovalRequestRef: null,
    adapterProofRef: null,
    reasons,
    rawChildCopyClaimed: false,
    renderedChildUiClaimed: false,
    notificationDeliveredClaimed: false,
    browserNavigationBlockedClaimed: false,
    blockPageRenderedClaimed: false,
    timeLimitAppliedClaimed: false,
    finalPolicyDecisionClaimed: false,
    cloudFrameAnalysisClaimed: false,
    nativeGameControlClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}

function surfaceState(
  snapshot: BrowserGameChildCheckingBlockUxSnapshot,
  surfaceKind: BrowserGameChildCheckingBlockUxSnapshot['surfaces'][number]['surfaceKind']
) {
  const surfaceEntry = snapshot.surfaces.find((candidate) => candidate.surfaceKind === surfaceKind);
  return {
    state: surfaceEntry?.state,
    primaryAction: surfaceEntry?.primaryAction,
    primaryTextToken: surfaceEntry?.primaryTextToken,
  };
}

function replaceSurface(
  snapshot: BrowserGameChildCheckingBlockUxSnapshot,
  surfaceKind: BrowserGameChildCheckingBlockUxSnapshot['surfaces'][number]['surfaceKind'],
  overrides: Partial<BrowserGameChildCheckingBlockUxSnapshot['surfaces'][number]>
) {
  return snapshot.surfaces.map((surfaceEntry) =>
    surfaceEntry.surfaceKind === surfaceKind ? { ...surfaceEntry, ...overrides } : surfaceEntry
  );
}
