import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { ParentRoute, type ParentBrowserPanelSnapshot } from '../../generated/parent-ui-bridge';
import {
  BrowserParentExplanationRoutePanel,
  shouldRenderBrowserParentExplanationRoute,
} from '../../src/BrowserParentExplanationRoutePanel';

function samplePanel(): ParentBrowserPanelSnapshot {
  return {
    eyebrow: 'Browser route',
    title: 'Browser parent explanations',
    body: 'Schema-backed parent explanations show evidence, model, policy, action, child experience, fallback, and audit sections only when a validated browser AI explanation bundle is present.',
    summary: '1 parent explanation rows',
    summaryDetails: [
      { label: 'Rows returned', value: '1' },
      { label: 'Status', value: 'ready' },
      { label: 'Product claim', value: 'Rendered parent explanation surface only.' },
    ],
    rows: [
      {
        key: 'browser-parent-explanation-1',
        title: 'Summary',
        details: [
          { label: 'Evidence references', value: 'source-evidence-1' },
          { label: 'Reason codes', value: 'policy-reason-1' },
        ],
      },
    ],
    emptyMessage: 'No browser parent explanation bundle has been reported yet.',
    productClaim: 'Rendered parent explanation surface only.',
  };
}

describe('browser parent explanation route panel', () => {
  it('mounts only on the proof-panels route', () => {
    expect(shouldRenderBrowserParentExplanationRoute(ParentRoute.ProofPanels)).toBe(true);
    expect(shouldRenderBrowserParentExplanationRoute(ParentRoute.AppGameSessions)).toBe(false);
  });

  it('renders the Rust-owned browser explanation snapshot', () => {
    const markup = renderToStaticMarkup(createElement(BrowserParentExplanationRoutePanel, { panel: samplePanel() }));

    expect(markup).toContain('Browser parent explanations');
    expect(markup).toContain('1 parent explanation rows');
    expect(markup).toContain('Summary');
    expect(markup).toContain('source-evidence-1');
    expect(markup).toContain('policy-reason-1');
  });
});
