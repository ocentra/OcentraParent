import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';

import {
  ParentRoute,
  type ParentPolicyPreviewPanelSnapshot,
} from '../../generated/parent-ui-bridge';
import { PolicyPreviewRoutePanel, shouldRenderPolicyPreviewRoute } from '../../src/PolicyPreviewRoutePanel';

describe('policy preview portal route panel', () => {
  it('attaches only to the policy authoring routes', () => {
    expect(shouldRenderPolicyPreviewRoute(ParentRoute.RuleManagement)).toBe(true);
    expect(shouldRenderPolicyPreviewRoute(ParentRoute.Schedules)).toBe(true);
    expect(shouldRenderPolicyPreviewRoute(ParentRoute.Approvals)).toBe(true);
    expect(shouldRenderPolicyPreviewRoute(ParentRoute.Enforcement)).toBe(true);

    expect(shouldRenderPolicyPreviewRoute(ParentRoute.Overview)).toBe(false);
    expect(shouldRenderPolicyPreviewRoute(ParentRoute.AppGameSessions)).toBe(false);
    expect(shouldRenderPolicyPreviewRoute(ParentRoute.AiRuntime)).toBe(false);
  });

  it('renders the Rust-owned policy preview panel snapshot', () => {
    const markup = renderToStaticMarkup(
      createElement(PolicyPreviewRoutePanel, {
        actions: policyPreviewActions(),
        commandEnabled: true,
        panel: activeControllerPolicyPreviewPanel(),
      })
    );

    expect(markup).toContain('Policy preview parent authoring');
    expect(markup).toContain('Refresh policy decision');
    expect(markup).toContain('https://example.test/latest');
    expect(markup).toContain('Approval authority');
    expect(markup).toContain('Active controller');
    expect(markup).toContain('Delivered is reported, but active enforcement is separate.');
  });

  it('renders observer-only policy authority from the Rust-owned panel snapshot', () => {
    const markup = renderToStaticMarkup(
      createElement(PolicyPreviewRoutePanel, {
        actions: policyPreviewActions(),
        commandEnabled: true,
        panel: observerPolicyPreviewPanel(),
      })
    );

    expect(markup).toContain('Observer only');
    expect(markup).toContain('cannot confirm or save writes');
    expect(markup).toContain('Delivered is reported, but active enforcement is separate.');
  });
});

function activeControllerPolicyPreviewPanel(): ParentPolicyPreviewPanelSnapshot {
  return {
    title: 'Policy preview parent authoring',
    body: 'Preview stays advisory until a parent confirms the request and a child-device contract applies it.',
    summary: 'Preview remains advisory and not enforced.',
    summaryDetails: [
      { label: 'Decision status', value: 'Preview remains advisory and not enforced.' },
      { label: 'Policy check', value: 'policy-preview-latest' },
      { label: 'Parent rule context references', value: '1' },
      { label: 'Parent rule context ref IDs', value: 'parent-rule-context-1' },
      { label: 'Parent access', value: 'Active controller' },
    ],
    cards: [
      {
        title: 'Preview state',
        summary: 'Preview remains advisory and not enforced.',
        details: [
          { label: 'Target value', value: 'https://example.test/latest' },
          { label: 'Request status', value: 'Approved' },
        ],
      },
      {
        title: 'Source lifecycle',
        summary: 'Delivered is reported, but active enforcement is separate.',
        details: [{ label: 'Source status', value: 'Confirmed' }],
      },
      {
        title: 'Approval authority',
        summary: 'Controller confirmation is recorded, but delivery and enforcement remain separate states.',
        details: [
          { label: 'Parent access', value: 'Active controller' },
          {
            label: 'Write authority',
            value: 'Preview-only route; no typed write command is exposed from this surface.',
          },
        ],
      },
    ],
    emptyMessage: 'No policy preview has been reported yet.',
    productClaim:
      'Policy preview is advisory parent-surface state only. It does not claim enforcement, adapter execution, provider delivery, or child-device application.',
  };
}

function observerPolicyPreviewPanel(): ParentPolicyPreviewPanelSnapshot {
  return {
    ...activeControllerPolicyPreviewPanel(),
    summaryDetails: [
      { label: 'Decision status', value: 'Preview remains advisory and not enforced.' },
      { label: 'Policy check', value: 'policy-preview-observer' },
      { label: 'Parent rule context references', value: '1' },
      { label: 'Parent rule context ref IDs', value: 'parent-rule-context-1' },
      { label: 'Parent access', value: 'Observer only' },
    ],
    cards: [
      {
        title: 'Preview state',
        summary: 'Preview remains advisory and not enforced.',
        details: [
          { label: 'Target value', value: 'https://example.test/observer' },
          { label: 'Request status', value: 'Approved' },
        ],
      },
      {
        title: 'Source lifecycle',
        summary: 'Delivered is reported, but active enforcement is separate.',
        details: [{ label: 'Source status', value: 'Delivered' }],
      },
      {
        title: 'Approval authority',
        summary: 'Observer-only parents can review policy explanation but cannot confirm or save writes.',
        details: [
          { label: 'Parent access', value: 'Observer only' },
          {
            label: 'Write authority',
            value: 'Observer scope is read-only and cannot confirm or save policy writes.',
          },
        ],
      },
    ],
  };
}

function policyPreviewActions() {
  return {
    reconnect: () => undefined,
    selectCommandResult: () => undefined,
    sendCommand: async () => null,
    refreshRouteSnapshot: async () => null,
  };
}
