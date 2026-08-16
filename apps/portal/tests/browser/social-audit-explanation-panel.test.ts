import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { ParentRoute, type ParentBrowserPanelSnapshot } from '../../generated/parent-ui-bridge';
import {
  SocialAuditExplanationRoutePanel,
  shouldRenderSocialAuditExplanationRoute,
} from '../../src/SocialAuditExplanationRoutePanel';

function samplePanel(): ParentBrowserPanelSnapshot {
  return {
    eyebrow: 'Browser route',
    title: 'Social explanations',
    body: 'Schema-backed social explanations show parent-visible evidence, policy, approval, memory, connector, native, manual, and audit refs without raw social content.',
    summary: '1 social explanation rows',
    summaryDetails: [
      { label: 'Rows returned', value: '1' },
      { label: 'Status', value: 'ready' },
      { label: 'Product claim', value: 'Rendered parent explanation surface only.' },
    ],
    rows: [
      {
        key: 'social-explanation-1',
        title: 'Account approval explanation',
        details: [
          { label: 'Status', value: 'manual-required' },
          { label: 'Evidence references', value: 'audit-ref-1' },
        ],
      },
    ],
    emptyMessage: 'No social audit explanation snapshot has been reported yet.',
    productClaim: 'Rendered parent explanation surface only.',
  };
}

describe('social audit explanation portal route panel', () => {
  it('mounts only on the proof-panels route', () => {
    expect(shouldRenderSocialAuditExplanationRoute(ParentRoute.ProofPanels)).toBe(true);
    expect(shouldRenderSocialAuditExplanationRoute(ParentRoute.AppGameSessions)).toBe(false);
  });

  it('renders the Rust-owned browser route snapshot', () => {
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
      createElement(SocialAuditExplanationRoutePanel, {
        actions,
        commandEnabled: true,
        panel: samplePanel(),
      })
    );

    expect(markup).toContain('Social explanations');
    expect(markup).toContain('1 social explanation rows');
    expect(markup).toContain('Account approval explanation');
    expect(markup).toContain('manual-required');
    expect(markup).toContain('audit-ref-1');
  });
});
