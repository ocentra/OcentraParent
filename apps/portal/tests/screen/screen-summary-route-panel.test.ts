import { readFileSync } from 'node:fs';
import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { ParentRoute, type ParentScreenSummaryPanelSnapshot } from '../../generated/parent-ui-bridge';
import { ScreenSummaryRoutePanel, shouldRenderScreenSummaryRoute } from '../../src/ScreenSummaryRoutePanel';

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
    const markup = renderToStaticMarkup(createElement(ScreenSummaryRoutePanel, { panel: SAMPLE_SCREEN_SUMMARY_PANEL }));

    expect(markup).toContain('Screen analysis');
    expect(markup).toContain('screen-ready-row');
    expect(markup).toContain('screen-ready-row-1');
    expect(markup).toContain('Kept');
  });

  it('renders the Rust-owned empty-state fallback when no snapshot is present', () => {
    const markup = renderToStaticMarkup(createElement(ScreenSummaryRoutePanel, { panel: null }));

    expect(markup).toContain('Unavailable');
    expect(markup).toContain('No recent activity is available yet.');
    expect(markup).toContain('No family setting is configured for this area yet.');
  });

  it('keeps the route on generated snapshots instead of TS business intent', () => {
    const panelSource = readFileSync(new URL('../../src/ScreenSummaryRoutePanel.tsx', import.meta.url), 'utf8');
    const appSource = readFileSync(new URL('../../src/PortalApp.tsx', import.meta.url), 'utf8');

    expect(panelSource).not.toContain('createScreenSummaryPanelIntent');
    expect(panelSource).not.toContain('activityScreenReadModel');
    expect(appSource).toContain('route === ParentRoute.ScreenAnalysis');
    expect(appSource).toContain('ScreenSummaryRoutePanel');
  });
});
