import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { ParentRoute, type ParentBrowserPanelSnapshot } from '../../generated/parent-ui-bridge';
import { SocialDashboardRoutePanel, shouldRenderSocialDashboardRoute } from '../../src/SocialDashboardRoutePanel';

function samplePanel(): ParentBrowserPanelSnapshot {
  return {
    eyebrow: 'Browser route',
    title: 'Social dashboard',
    body: 'Schema-backed social rows show parent-review and manual-required status only; runtime fetch, connector, native app, policy execution, and enforcement remain unclaimed.',
    summary: '2 social dashboard rows',
    summaryDetails: [
      { label: 'Rows returned', value: '2' },
      { label: 'Status', value: 'ready' },
      { label: 'Product claim', value: 'Rendered parent surface only.' },
    ],
    rows: [
      {
        key: 'dashboard-row-1',
        title: 'Account approvals',
        details: [
          { label: 'Status', value: 'ready-for-review' },
          { label: 'Reason', value: 'account-approval-needed' },
        ],
      },
    ],
    emptyMessage: 'No social dashboard snapshot has been reported yet.',
    productClaim: 'Rendered parent surface only.',
  };
}

describe('social dashboard portal route panel', () => {
  it('mounts only on the proof-panels route', () => {
    expect(shouldRenderSocialDashboardRoute(ParentRoute.ProofPanels)).toBe(true);
    expect(shouldRenderSocialDashboardRoute(ParentRoute.AppGameSessions)).toBe(false);
  });

  it('renders the Rust-owned social dashboard snapshot', () => {
    const actions = {
      reconnect() {},
      async refreshRouteSnapshot() {
        return null;
      },
      selectCommandResult() {
        return undefined;
      },
      async sendCommand() {
        return null;
      },
    };
    const markup = renderToStaticMarkup(
      createElement(SocialDashboardRoutePanel, {
        actions,
        commandEnabled: true,
        panel: samplePanel(),
      })
    );

    expect(markup).toContain('Social dashboard');
    expect(markup).toContain('2 social dashboard rows');
    expect(markup).toContain('Account approvals');
    expect(markup).toContain('ready-for-review');
    expect(markup).toContain('account-approval-needed');
  });
});
