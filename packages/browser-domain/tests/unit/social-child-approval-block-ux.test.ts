import { describe, expect, it } from 'vitest';
import {
  type SocialChildApprovalBlockUxSnapshot,
  SocialChildApprovalBlockUxSnapshotSchema,
} from '@ocentra-parent/schema-domain/social-child-approval-block-ux';

describe('social child approval and block UX contracts', () => {
  it('accepts a contract-only child approval and block state snapshot', acceptsHonestSnapshot);
  it('rejects missing required child-facing states', rejectsMissingState);
  it(
    'rejects rendered UI, notification, browser block, policy, native, connector, and enforcement claims',
    rejectsClaims
  );
  it('rejects unsupported child-facing state upgrades', rejectsUnsupportedStateUpgrades);
});

function acceptsHonestSnapshot() {
  const parsed = SocialChildApprovalBlockUxSnapshotSchema.parse(validSnapshot());

  expect(parsed.schemaVersion).toBe('social-child-approval-block-ux-contract');
  expect(parsed.surfaces).toHaveLength(6);
  expect(surfaceState(parsed, 'approval-request-pending')).toEqual({
    state: 'waiting-parent',
    primaryAction: 'wait-for-parent',
  });
  expect(surfaceState(parsed, 'blocked-social-route-candidate')).toEqual({
    state: 'blocked-contract-only',
    primaryAction: 'open-safe-back',
  });
}

function rejectsMissingState() {
  const snapshot = validSnapshot();

  expect(
    SocialChildApprovalBlockUxSnapshotSchema.safeParse({
      ...snapshot,
      surfaces: snapshot.surfaces.filter((surface) => surface.surfaceKind !== 'native-app-unavailable'),
    }).success
  ).toBe(false);
}

function rejectsClaims() {
  const snapshot = validSnapshot();
  const invalidSurfaces = [
    { renderedChildUiClaimed: true },
    { notificationDeliveredClaimed: true },
    { browserNavigationBlockedClaimed: true },
    { blockPageRenderedClaimed: true },
    { timeLimitAppliedClaimed: true },
    { finalPolicyDecisionClaimed: true },
    { connectorAuthorizationClaimed: true },
    { nativeAppControlClaimed: true },
    { enforcementClaimed: true },
  ];

  for (const invalid of invalidSurfaces) {
    expect(
      SocialChildApprovalBlockUxSnapshotSchema.safeParse({
        ...snapshot,
        surfaces: replaceSurface(snapshot, 'blocked-social-route-candidate', invalid),
      }).success
    ).toBe(false);
  }
}

function rejectsUnsupportedStateUpgrades() {
  const snapshot = validSnapshot();

  expect(
    SocialChildApprovalBlockUxSnapshotSchema.safeParse({
      ...snapshot,
      surfaces: replaceSurface(snapshot, 'approval-request-pending', {
        parentApprovalRequestRef: null,
      }),
    }).success
  ).toBe(false);

  expect(
    SocialChildApprovalBlockUxSnapshotSchema.safeParse({
      ...snapshot,
      surfaces: replaceSurface(snapshot, 'time-limit-candidate', {
        timeLimitAppliedClaimed: true,
      }),
    }).success
  ).toBe(false);
}

function validSnapshot(): SocialChildApprovalBlockUxSnapshot {
  return {
    schemaVersion: 'social-child-approval-block-ux-contract',
    familyId: 'family-social-child-ux',
    childProfileId: 'child-social-child-ux',
    deviceId: 'device-social-child-ux',
    generatedAt: '2026-06-03T08:21:00.000Z',
    surfaces: [
      surface('approval-request-pending', 'waiting-parent', 'wait-for-parent', ['parent-approval-needed'], {
        parentApprovalRequestRef: 'parent-approval-request-social',
      }),
      surface('blocked-social-route-candidate', 'blocked-contract-only', 'open-safe-back', ['route-block-candidate']),
      surface('warning-social-route-candidate', 'child-readable', 'acknowledge-warning', ['route-warning-candidate']),
      surface('manual-review-required', 'manual-required', 'manual-review', ['manual-review-needed']),
      surface('time-limit-candidate', 'child-readable', 'acknowledge-warning', ['time-limit-not-applied']),
      surface('native-app-unavailable', 'unavailable', 'no-action', ['native-app-proof-unavailable']),
    ],
    claimBoundaries: {
      renderedChildUi: 'not-claimed',
      notificationDelivery: 'not-claimed',
      browserNavigationBlock: 'not-claimed',
      blockPageRender: 'not-claimed',
      timeLimitApply: 'not-claimed',
      finalPolicyDecision: 'not-claimed',
      connectorAuthorization: 'not-claimed',
      nativeAppControl: 'not-claimed',
      enforcement: 'not-claimed',
    },
  };
}

function surface(
  surfaceKind: SocialChildApprovalBlockUxSnapshot['surfaces'][number]['surfaceKind'],
  state: SocialChildApprovalBlockUxSnapshot['surfaces'][number]['state'],
  primaryAction: SocialChildApprovalBlockUxSnapshot['surfaces'][number]['primaryAction'],
  reasons: SocialChildApprovalBlockUxSnapshot['surfaces'][number]['reasons'],
  overrides: Partial<SocialChildApprovalBlockUxSnapshot['surfaces'][number]> = {}
): SocialChildApprovalBlockUxSnapshot['surfaces'][number] {
  return {
    surfaceId: `social-child-ux-${surfaceKind}`,
    surfaceKind,
    state,
    primaryAction,
    sourceEvidenceRefs: [`parent-evidence-${surfaceKind}`],
    parentApprovalRequestRef: null,
    gatePlanRef: surfaceKind === 'blocked-social-route-candidate' ? 'parent-gate-plan-social-route' : null,
    reasons,
    renderedChildUiClaimed: false,
    notificationDeliveredClaimed: false,
    browserNavigationBlockedClaimed: false,
    blockPageRenderedClaimed: false,
    timeLimitAppliedClaimed: false,
    finalPolicyDecisionClaimed: false,
    connectorAuthorizationClaimed: false,
    nativeAppControlClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}

function surfaceState(
  snapshot: SocialChildApprovalBlockUxSnapshot,
  surfaceKind: SocialChildApprovalBlockUxSnapshot['surfaces'][number]['surfaceKind']
) {
  const surfaceEntry = snapshot.surfaces.find((candidate) => candidate.surfaceKind === surfaceKind);
  return {
    state: surfaceEntry?.state,
    primaryAction: surfaceEntry?.primaryAction,
  };
}

function replaceSurface(
  snapshot: SocialChildApprovalBlockUxSnapshot,
  surfaceKind: SocialChildApprovalBlockUxSnapshot['surfaces'][number]['surfaceKind'],
  overrides: Partial<SocialChildApprovalBlockUxSnapshot['surfaces'][number]>
) {
  return snapshot.surfaces.map((surfaceEntry) =>
    surfaceEntry.surfaceKind === surfaceKind ? { ...surfaceEntry, ...overrides } : surfaceEntry
  );
}
