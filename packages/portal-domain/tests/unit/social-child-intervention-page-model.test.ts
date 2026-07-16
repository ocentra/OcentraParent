import { describe, expect, it } from 'vitest';
import { renderBrowserChildInterventionPage } from '../../src/browser-child-intervention-page';
import {
  createSocialChildInterventionPageModels,
  type SocialChildApprovalBlockUxSnapshot,
} from '../../src/social-child-intervention-page-model';

describe('social child intervention page model', () => {
  it('maps every social child approval/block UX surface to the shared child intervention page model', () => {
    const result = createSocialChildInterventionPageModels(validSnapshot(), {
      requestedUrlForSurface: (surface) => `https://proof.example.invalid/${surface.surfaceKind}`,
    });

    expect(result.state).toBe('renderable');
    expect(result.models.map((model) => [model.action, model.deliveryState, model.outcome])).toEqual([
      ['approval-hold', 'approval-hold-rendered', 'approval-required'],
      ['block', 'block-page-rendered', 'blocked'],
      ['warn', 'warn-page-rendered', 'warned'],
      ['parent-review', 'manual-required', 'manual-required'],
      ['time-limit', 'time-limit-candidate-rendered', 'time-limit-candidate'],
      ['parent-review', 'native-app-unavailable', 'manual-required'],
    ]);
    expect(result.models.map((model) => model.targetType)).toEqual([
      'social-route',
      'social-route',
      'social-route',
      'social-route',
      'social-route',
      'social-route',
    ]);
    expect(result.models[1]?.requestedUrl).toBe('https://proof.example.invalid/blocked-social-route-candidate');
  });

  it('renders mapped social child states through the shared intervention page html', () => {
    const result = createSocialChildInterventionPageModels(validSnapshot());
    expect(result.state).toBe('renderable');

    const html = result.models.map((model) => renderBrowserChildInterventionPage(model)).join('\n');

    expect(html).toContain('OCENTRA_MANAGED_BROWSER_BLOCKED');
    expect(html).toContain('Parent approval required for this social account action');
    expect(html).toContain('Blocked social route candidate');
    expect(html).toContain('Social route warning');
    expect(html).toContain('Manual parent review required');
    expect(html).toContain('Social route time limit candidate');
    expect(html).toContain('Native social app proof unavailable');
    expect(html).toContain('ocentra-child-approval-request');
  });

  it('does not render dishonest snapshots that claim runtime child UI or enforcement', () => {
    const snapshot = validSnapshot();
    const invalid = {
      ...snapshot,
      surfaces: snapshot.surfaces.map((surface) =>
        surface.surfaceKind === 'blocked-social-route-candidate'
          ? { ...surface, blockPageRenderedClaimed: true }
          : surface
      ),
    };

    const result = createSocialChildInterventionPageModels(invalid);

    expect(result).toEqual({
      models: [],
      reason: 'invalid-social-child-ux-snapshot',
      state: 'unavailable',
    });
  });
});

function validSnapshot(): SocialChildApprovalBlockUxSnapshot {
  return {
    schemaVersion: 'social-child-approval-block-ux-contract',
    familyId: 'family-social-child-ux',
    childProfileId: 'child-social-child-ux',
    deviceId: 'device-social-child-ux',
    generatedAt: '2026-06-06T04:45:00.000Z',
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
