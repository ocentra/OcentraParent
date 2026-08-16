import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { TrackingStatusRoutePanel } from '../../src/TrackingStatusRoutePanel';
import { EMPTY_ROUTE_LIVE_ACTIVITY_STATE } from '../../src/route-live-activity-state';
import type { ParentTrackingStatusPanelSnapshot } from '../../generated/parent-ui-bridge';

describe('tracking status route panel', () => {
  it('renders the Rust-backed tracking panel when the route snapshot provides one', () => {
    const markup = renderToStaticMarkup(
      createElement(TrackingStatusRoutePanel, {
        actions: trackingActions(),
        commandEnabled: true,
        liveActivity: {
          ...EMPTY_ROUTE_LIVE_ACTIVITY_STATE,
          activityTrackingPanel: trackingPanelSnapshot(),
        },
      })
    );

    expect(markup).toContain('Rust-backed tracking proof surface');
    expect(markup).toContain('Tracking live summary');
    expect(markup).toContain('Tracking service data coverage');
    expect(markup).toContain('Family dashboard tracking rollup');
    expect(markup).toContain('Notification history intent UI');
    expect(markup).toContain('Child check-in request');
  });

  it('renders nothing when Rust does not provide a tracking panel snapshot', () => {
    const markup = renderToStaticMarkup(
      createElement(TrackingStatusRoutePanel, {
        actions: trackingActions(),
        commandEnabled: true,
        liveActivity: EMPTY_ROUTE_LIVE_ACTIVITY_STATE,
      })
    );

    expect(markup).toBe('');
  });
});

function trackingPanelSnapshot(): ParentTrackingStatusPanelSnapshot {
  return {
    eyebrow: 'First target',
    title: 'Tracking status',
    body: 'Rust-backed tracking proof surface.',
    summaryCards: [
      {
        key: 'tracking-live-summary',
        title: 'Tracking live summary',
        details: [
          { label: 'Status', value: 'Available' },
          { label: 'Rows returned', value: '1' },
        ],
      },
      {
        key: 'tracking-service-data-coverage',
        title: 'Tracking service data coverage',
        details: [
          { label: 'Status', value: 'Available' },
          { label: 'Rows returned', value: '1' },
        ],
      },
    ],
    cards: [
      {
        key: 'family-dashboard-rollup',
        title: 'Family dashboard tracking rollup',
        details: [{ label: 'Status', value: 'Read-only' }],
      },
      {
        key: 'notification-history-intent-ui',
        title: 'Notification history intent UI',
        details: [{ label: 'Status', value: 'Read-only' }],
      },
      {
        key: 'child-check-in-request',
        title: 'Child check-in request',
        details: [{ label: 'Status', value: 'Read-only' }],
      },
    ],
    emptyMessage: 'No tracking activity is available yet.',
    productClaim:
      'Tracking status is Rust-read-model-backed UI only; provider delivery, child delivery, physical-device execution, and authority actions remain unclaimed unless an explicit proof row states otherwise.',
  };
}

function trackingActions() {
  return {
    reconnect: () => undefined,
    selectCommandResult: () => undefined,
    sendCommand: async () => null,
    refreshRouteSnapshot: async () => null,
    requestTrackingRetentionSettingsWrite: async () => null,
  };
}
