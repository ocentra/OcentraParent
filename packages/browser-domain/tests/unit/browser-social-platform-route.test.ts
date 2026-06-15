import { describe, expect, it } from 'vitest';
import {
  BrowserSocialRouteEvidenceSchema,
  BrowserSocialRouteSchemaVersion,
} from '../../src/browser-social-platform-route-schemas';

describe('browser social platform route contract', () => {
  it('accepts managed browser social route evidence from URL-shape proof', expectManagedBrowserSocialRoute);
  it('accepts unmanaged social bypass as process-only manual-required evidence', expectUnmanagedSocialBypass);
  it('accepts native app social state only as manual-required route evidence', expectNativeManualRequired);
  it('rejects social route rows that claim account, message, feed, AI, policy, or enforcement authority', rejectClaims);
  it('rejects managed social routes without URL-shape proof linkage', rejectManagedRouteWithoutUrlShape);
  it('rejects bypass rows promoted to exact managed route evidence', rejectBypassPromotion);
});

function expectManagedBrowserSocialRoute() {
  const parsed = BrowserSocialRouteEvidenceSchema.safeParse(managedInstagramSignupRoute());

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.sourceKind).toBe('managed-browser-url-shape');
    expect(parsed.data.platform).toBe('instagram');
    expect(parsed.data.routeKind).toBe('account-signup');
    expect(parsed.data.exactManagedBrowserRouteEvidence).toBe(true);
  }
}

function expectUnmanagedSocialBypass() {
  const parsed = BrowserSocialRouteEvidenceSchema.safeParse(unmanagedSocialBypass());

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.proofState).toBe('bypass-only');
    expect(parsed.data.routeKind).toBe('unknown-social-route');
    expect(parsed.data.exactManagedBrowserRouteEvidence).toBe(false);
  }
}

function expectNativeManualRequired() {
  const parsed = BrowserSocialRouteEvidenceSchema.safeParse(nativeManualRequiredRoute());

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.sourceKind).toBe('native-app-manual-required');
    expect(parsed.data.manualRequired).toBe(true);
    expect(parsed.data.nativeAppControlClaimed).toBe(false);
  }
}

function rejectClaims() {
  for (const invalid of claimedAuthorityRows()) {
    const parsed = BrowserSocialRouteEvidenceSchema.safeParse(invalid);

    expect(parsed.success).toBe(false);
  }
}

function rejectManagedRouteWithoutUrlShape() {
  const parsed = BrowserSocialRouteEvidenceSchema.safeParse({
    ...managedInstagramSignupRoute(),
    urlShapeClassificationId: null,
  });

  expect(parsed.success).toBe(false);
}

function rejectBypassPromotion() {
  const parsed = BrowserSocialRouteEvidenceSchema.safeParse({
    ...unmanagedSocialBypass(),
    routeKind: 'feed',
    exactManagedBrowserRouteEvidence: true,
    unmanagedBypassOnly: false,
  });

  expect(parsed.success).toBe(false);
}

function claimedAuthorityRows() {
  const valid = managedInstagramSignupRoute();
  return [
    { ...valid, accountIdentityClaimed: true },
    { ...valid, messageContentClaimed: true },
    { ...valid, feedContentSemanticsClaimed: true },
    { ...valid, aiDecisionClaimed: true },
    { ...valid, policyDecisionClaimed: true },
    { ...valid, enforcementClaimed: true },
    { ...valid, nativeAppControlClaimed: true },
    { ...valid, platformConnectorClaimed: true },
  ];
}

function managedInstagramSignupRoute() {
  return {
    ...baseSocialRoute(),
    socialRouteEvidenceId: 'social-route-instagram-signup',
    sourceEvidenceIds: ['browser-evidence-instagram-signup'],
    urlShapeClassificationId: 'url-shape-instagram-signup',
    urlShapeTargetKind: 'social-upload-post',
    sourceKind: 'managed-browser-url-shape',
    proofState: 'route-evidence',
    platform: 'instagram',
    routeKind: 'account-signup',
    exactManagedBrowserRouteEvidence: true,
  };
}

function unmanagedSocialBypass() {
  return {
    ...baseSocialRoute(),
    socialRouteEvidenceId: 'social-route-unmanaged-bypass',
    sourceEvidenceIds: ['browser-evidence-unmanaged-social-process'],
    sourceKind: 'unmanaged-browser-bypass',
    proofState: 'bypass-only',
    platform: 'unknown-social',
    routeKind: 'unknown-social-route',
    unmanagedBypassOnly: true,
    manualRequired: true,
  };
}

function nativeManualRequiredRoute() {
  return {
    ...baseSocialRoute(),
    socialRouteEvidenceId: 'social-route-native-app-manual-required',
    sourceEvidenceIds: ['app-evidence-native-social-app'],
    sourceKind: 'native-app-manual-required',
    proofState: 'manual-required',
    platform: 'tiktok',
    routeKind: 'unknown-social-route',
    manualRequired: true,
  };
}

function baseSocialRoute() {
  return {
    schemaVersion: BrowserSocialRouteSchemaVersion,
    socialRouteEvidenceId: 'social-route-evidence',
    observedAt: '2026-06-03T01:30:00.000Z',
    sourceEvidenceIds: ['browser-evidence-social-route'],
    urlShapeClassificationId: null,
    urlShapeTargetKind: null,
    sourceKind: 'managed-browser-url-shape',
    proofState: 'route-evidence',
    platform: 'unknown-social',
    routeKind: 'unknown-social-route',
    platformAccountRef: null,
    parentApprovalRequestRef: null,
    exactManagedBrowserRouteEvidence: false,
    unmanagedBypassOnly: false,
    manualRequired: false,
    accountIdentityClaimed: false,
    messageContentClaimed: false,
    feedContentSemanticsClaimed: false,
    aiDecisionClaimed: false,
    policyDecisionClaimed: false,
    enforcementClaimed: false,
    nativeAppControlClaimed: false,
    platformConnectorClaimed: false,
  };
}
