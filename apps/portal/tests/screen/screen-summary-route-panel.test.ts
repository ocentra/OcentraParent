import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { ParentRoute, type ParentScreenSummaryPanelSnapshot } from '../../generated/parent-ui-bridge';
import {
  ScreenSummaryRoutePanel,
  ScreenSummaryRoutePanelMount,
  shouldRenderScreenSummaryRoute,
} from '../../src/ScreenSummaryRoutePanel';
import type { PortalRenderActions } from '../../src/portal-actions';

const actions: PortalRenderActions = {
  reconnect() {},
  selectCommandResult() {},
  async sendCommand() {
    return null;
  },
};

const SAMPLE_SCREEN_SUMMARY_PANEL: ParentScreenSummaryPanelSnapshot = {
  eyebrow: 'Activity kind',
  title: 'Screen analysis',
  body: 'Stored activity',
  loadState: 'Ready',
  summaryDetails: [
    { label: 'Status', value: 'Ready' },
    { label: 'Generated at', value: '2026-06-27T17:40:00Z' },
    { label: 'Rows returned', value: '1' },
    { label: 'Product claim', value: 'No family setting is configured for this area yet.' },
  ],
  rows: [
    {
      title: 'screen-ready-row',
      details: [
        { label: 'Event ID', value: 'screen-ready-row-1' },
        { label: 'Capability', value: 'Kept' },
      ],
    },
  ],
  emptyMessage: 'No recent activity is available yet.',
  productClaim: 'No family setting is configured for this area yet.',
};

describe('screen summary route panel', () => {
  it('renders only on the Screen Analysis route', () => {
    expect(shouldRenderScreenSummaryRoute(ParentRoute.ScreenAnalysis)).toBe(true);
    expect(shouldRenderScreenSummaryRoute(ParentRoute.Activity)).toBe(false);
    expect(shouldRenderScreenSummaryRoute(ParentRoute.Overview)).toBe(false);
  });

  it('renders the Rust-owned screen summary snapshot directly', () => {
    const markup = renderToStaticMarkup(
      createElement(ScreenSummaryRoutePanel, {
        actions,
        commandEnabled: false,
        panel: SAMPLE_SCREEN_SUMMARY_PANEL,
      })
    );

    expect(markup).toContain('Screen analysis');
    expect(markup).toContain('screen-ready-row');
    expect(markup).toContain('screen-ready-row-1');
    expect(markup).toContain('Kept');
    expect(markup).toContain('data-ocentra-screen-summary-empty="false"');
    expect(markup).toContain('data-ocentra-screen-summary-state="Ready"');
  });

  it('renders the Rust-owned empty-state fallback when no snapshot is present', () => {
    const markup = renderToStaticMarkup(
      createElement(ScreenSummaryRoutePanel, { actions, commandEnabled: false, panel: null })
    );

    expect(markup).toContain('Unavailable');
    expect(markup).toContain('No screen summary read model has been reported.');
    expect(markup).toContain('<h2>Activity rows</h2>');
    expect(markup).toContain('<h2>Analysis capability</h2>');
    expect(markup).toContain('<h2>Evidence custody</h2>');
    expect(markup.match(/<article class="summary product-status-card">/gu)).toHaveLength(3);
    expect(markup).toContain('Retry status');
    expect(markup).toContain('data-ocentra-screen-summary-empty="true"');
    expect(markup).toContain('data-ocentra-screen-summary-reported="false"');
    expect(markup).toContain('data-ocentra-screen-summary-state="Unavailable"');
    expect(markup).not.toContain('<details');
  });

  it('mounts the generated snapshot only for Screen Analysis', () => {
    const screenMarkup = renderToStaticMarkup(
      createElement(ScreenSummaryRoutePanelMount, {
        actions,
        commandEnabled: false,
        panel: SAMPLE_SCREEN_SUMMARY_PANEL,
        route: ParentRoute.ScreenAnalysis,
      })
    );
    const activityMarkup = renderToStaticMarkup(
      createElement(ScreenSummaryRoutePanelMount, {
        actions,
        commandEnabled: false,
        panel: SAMPLE_SCREEN_SUMMARY_PANEL,
        route: ParentRoute.Activity,
      })
    );

    expect(screenMarkup).toContain('screen-ready-row-1');
    expect(activityMarkup).toBe('');
  });
});
