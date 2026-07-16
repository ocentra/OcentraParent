import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { ParentRoute, type ParentAppGamePanelSnapshot } from '../../generated/parent-ui-bridge';
import type { PortalRenderActions } from '../../src/portal-actions';
import {
  AppGamePolicyReadinessRoutePanel,
  shouldRenderAppGamePolicyReadinessRoute,
} from '../../src/AppGamePolicyReadinessRoutePanel';

const PolicyReadinessPanel: ParentAppGamePanelSnapshot = {
  eyebrow: 'Rust-owned panel',
  title: 'App/game policy readiness',
  body: 'Rust-owned policy readiness rows are rendered directly in the portal.',
  loadState: 'ready',
  summaryDetails: [
    { label: 'Capability', value: 'Not claimed' },
    { label: 'Adapter dispatch', value: 'Not claimed' },
    { label: 'Manual review', value: 'Manual required' },
    { label: 'Evidence claim rows', value: '1' },
    { label: 'Approval action result rows', value: '0' },
    { label: 'AI classifier rows', value: '0' },
    { label: 'Category candidate rows', value: '0' },
    { label: 'Unknown review rows', value: '1' },
    { label: 'Category routing', value: 'Ready' },
    { label: 'Unknown review', value: 'Manual required' },
  ],
  rows: [
    {
      title: 'Policy evidence',
      details: [
        { label: 'Reason', value: 'Ready' },
        { label: 'Evidence references', value: 'claim-1, identity-1' },
      ],
    },
    {
      title: 'AI classifier context',
      details: [
        { label: 'Reason', value: 'AI classifier context requires manual review' },
        { label: 'Evidence references', value: 'Not reported' },
      ],
    },
  ],
  emptyMessage: 'No app/game policy readiness panel has been reported yet.',
  productClaim: 'Approval workflow, category routing, and adapter dispatch remain unclaimed.',
};

describe('app-game policy readiness portal route panel', () => {
  it('attaches the renderer only to App/Game Sessions', () => {
    expect(shouldRenderAppGamePolicyReadinessRoute(ParentRoute.AppGameSessions)).toBe(true);
    expect(shouldRenderAppGamePolicyReadinessRoute(ParentRoute.Overview)).toBe(false);
  });

  it('renders the Rust-owned policy readiness panel without TS read-model reconstruction', () => {
    const html = renderToStaticMarkup(
      createElement(AppGamePolicyReadinessRoutePanel, {
        actions: testActions(),
        commandEnabled: true,
        panel: PolicyReadinessPanel,
      })
    );

    expect(html).toContain('App/game policy readiness');
    expect(html).toContain('Capability');
    expect(html).toContain('Not claimed');
    expect(html).toContain('Adapter dispatch');
    expect(html).toContain('Manual review');
    expect(html).toContain('Category candidate rows');
    expect(html).toContain('Unknown review rows');
    expect(html).toContain('Category routing');
    expect(html).toContain('Unknown review');
    expect(html).toContain('Policy evidence');
    expect(html).toContain('AI classifier context');
    expect(html).toContain('AI classifier context requires manual review');
    expect(html).toContain('Not reported');
  });

  it('keeps the Rust-owned empty state explicit when the panel snapshot is absent', () => {
    const html = renderToStaticMarkup(
      createElement(AppGamePolicyReadinessRoutePanel, {
        actions: testActions(),
        commandEnabled: false,
        panel: null,
      })
    );

    expect(html).toContain('No app/game policy readiness panel has been reported yet.');
    expect(html).toContain('Approval workflow, category routing, and adapter dispatch remain unclaimed.');
    expect(html).toContain('disabled=""');
  });
});

function testActions(): PortalRenderActions {
  return {
    reconnect() {},
    selectCommandResult() {},
    async sendCommand() {
      return null;
    },
    async refreshRouteSnapshot() {
      return null;
    },
  };
}
