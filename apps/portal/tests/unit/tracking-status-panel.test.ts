import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { TrackingStatusRoutePanel } from '../../src/TrackingStatusRoutePanel';
import { EMPTY_ROUTE_LIVE_ACTIVITY_STATE } from '../../src/route-live-activity-state';
import type { ParentTrackingStatusPanelSnapshot } from '../../generated/parent-ui-bridge';

const trackingProductClaim =
  'Shows only records reported by the local Rust service. It does not prove child delivery, live device execution, provider delivery, or permission to take actions.';

describe('tracking status route panel', () => {
  registerReportedTrackingTests();
  registerUnavailableTrackingTests();
});

function registerReportedTrackingTests(): void {
  it('renders the Rust-backed proof panel when the route snapshot provides one', () => {
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

  it('renders the Rust-owned family dashboard, activity coverage, and unavailable action boundaries', () => {
    const markup = renderToStaticMarkup(
      createElement(TrackingStatusRoutePanel, {
        actions: trackingActions(),
        commandEnabled: true,
        liveActivity: {
          ...EMPTY_ROUTE_LIVE_ACTIVITY_STATE,
          activityTrackingPanel: trackingLivePanelSnapshot(),
        },
        surface: 'product',
      })
    );

    expect(markup).toContain('aria-label="Tracking status"');
    expect(markup).toContain('data-ocentra-tracking-surface="product"');
    expect(markup).toContain('aria-label="Tracking overview"');
    expect(markup).toContain('aria-label="Tracking details and availability"');
    expect(markup).toContain('Current child tracking history, service coverage, custody');
    expect(markup).toContain('Family dashboard tracking rollup');
    expect(markup).toContain('Current child status');
    expect(markup).toContain('Last known location');
    expect(markup).toContain('Tracking activity coverage');
    expect(markup).toContain('Custody and retention');
    expect(markup).toContain('Child tracking surface');
    expect(markup).toContain('Tracking controls');
    expect(markup).toContain('Expected-place status · citation 1');
    expect(markup).toContain('Observer source</dt><dd>tracking-engine');
    expect(markup).toContain('Subject ID</dt><dd>expected-place-school');
    expect(markup).toContain('Subject name</dt><dd>School');
    expect(markup).toContain('Capability status</dt><dd>recent');
    expect(markup).toContain('Query visibility</dt><dd>active');
    expect(markup).toContain('Deleted at</dt><dd>Not reported');
    expect(markup).toContain('Deleted evidence refs</dt><dd>Not reported');
    expect(markup).toContain('Tracking boundary');
    expect(markup).toContain('data-ocentra-tracking-card-state="recent"');
    expect(markup).toContain('data-ocentra-tracking-card-state="unavailable"');
    expect(markup.split(trackingProductClaim)).toHaveLength(2);
    expect(markup).not.toContain('Proof tier');
    expect(markup).not.toContain('ui-fixture');
  });
}

function registerUnavailableTrackingTests(): void {
  registerReportedUnavailableTrackingTest();
  registerMissingTrackingPanelTests();
  registerInvalidTrackingPanelTest();
  registerMissingRefreshActionTest();
}

function registerReportedUnavailableTrackingTest(): void {
  it('keeps the Rust-owned unavailable product state visible with a real reconnect action', () => {
    const markup = renderToStaticMarkup(
      createElement(TrackingStatusRoutePanel, {
        actions: trackingActions(),
        commandEnabled: false,
        liveActivity: {
          ...EMPTY_ROUTE_LIVE_ACTIVITY_STATE,
          activityTrackingPanel: trackingUnavailablePanelSnapshot(),
        },
        surface: 'product',
      })
    );

    expect(markup).toContain('aria-label="Tracking status"');
    expect(markup).toContain('data-ocentra-tracking-surface="product"');
    expect(markup).toContain('>Retry status</button>');
    expect(markup).not.toContain('disabled=""');
    expect(markup).toContain('Unavailable');
    expect(markup).toContain('Rows returned</dt><dd>0');
    expect(markup).toContain('Last observed</dt><dd>Not reported');
    expect(markup).toContain('Tracking boundary');
    expect(markup.split(trackingProductClaim)).toHaveLength(2);
    expect(markup).not.toContain('Family dashboard tracking rollup');
    expect(markup).not.toContain('Proof tier');
    expect(markup).not.toContain('ui-fixture');
  });
}

function registerMissingTrackingPanelTests(): void {
  it('renders nothing on the proof surface when Rust does not provide a tracking panel snapshot', () => {
    const markup = renderToStaticMarkup(
      createElement(TrackingStatusRoutePanel, {
        actions: trackingActions(),
        commandEnabled: true,
        liveActivity: EMPTY_ROUTE_LIVE_ACTIVITY_STATE,
      })
    );

    expect(markup).toBe('');
  });

  it('renders an explicit unavailable proof surface when its developer host requests one', () => {
    const markup = renderToStaticMarkup(
      createElement(TrackingStatusRoutePanel, {
        actions: trackingActions(),
        commandEnabled: false,
        liveActivity: EMPTY_ROUTE_LIVE_ACTIVITY_STATE,
        showUnavailable: true,
      })
    );

    expect(markup).toContain('data-ocentra-tracking-route-state="unavailable"');
    expect(markup).toContain('data-ocentra-tracking-surface="proof"');
    expect(markup).toContain('Tracking status unavailable');
    expect(markup).toContain('Tracking is not connected to the local service.');
    expect(markup).toContain('Location and devices');
    expect(markup).toContain('Tracking controls');
  });

  it('renders an explicit unavailable product surface when Rust does not provide a tracking panel snapshot', () => {
    const markup = renderToStaticMarkup(
      createElement(TrackingStatusRoutePanel, {
        actions: trackingActions(),
        commandEnabled: false,
        liveActivity: EMPTY_ROUTE_LIVE_ACTIVITY_STATE,
        surface: 'product',
      })
    );

    expect(markup).toContain('aria-label="Tracking status"');
    expect(markup).toContain('data-ocentra-tracking-route-state="unavailable"');
    expect(markup).toContain('data-ocentra-tracking-surface="product"');
    expect(markup).toContain('Tracking status unavailable');
    expect(markup).toContain('Tracking is not connected to the local service.');
    expect(markup).toContain('Retry status to load the Rust-owned tracking read model');
    expect(markup).toContain('Location and devices');
    expect(markup).toContain('No location, accuracy, device freshness, or child status is displayed');
    expect(markup).toContain('Tracking controls');
    expect(markup).toContain('actions stay unavailable until the service supplies owner-authorized inputs');
    expect(markup).toContain('>Retry status</button>');
    expect(markup).not.toContain('disabled=""');
    expect(markup).not.toContain('ui-fixture');
  });
}

function registerInvalidTrackingPanelTest(): void {
  it('rejects malformed service data without rendering it as trusted product state', () => {
    const markup = renderToStaticMarkup(
      createElement(TrackingStatusRoutePanel, {
        actions: trackingActions(),
        commandEnabled: true,
        liveActivity: {
          ...EMPTY_ROUTE_LIVE_ACTIVITY_STATE,
          activityTrackingPanel: {
            ...trackingLivePanelSnapshot(),
            title: 'Caller-forged tracking state',
            portalAuthority: true,
          },
        },
        surface: 'product',
      })
    );

    expect(markup).toContain('data-ocentra-tracking-route-state="invalid-contract"');
    expect(markup).toContain('Tracking status data rejected');
    expect(markup).toContain('does not match the Rust-owned contract');
    expect(markup).toContain('>Refresh tracking status</button>');
    expect(markup).not.toContain('disabled=""');
    expect(markup).not.toContain('Caller-forged tracking state');
    expect(markup).not.toContain('citation 1');
  });
}

function registerMissingRefreshActionTest(): void {
  it('falls back to reconnect when the host does not provide a route refresh action', () => {
    const markup = renderToStaticMarkup(
      createElement(TrackingStatusRoutePanel, {
        actions: trackingActionsWithoutRefresh(),
        commandEnabled: true,
        liveActivity: {
          ...EMPTY_ROUTE_LIVE_ACTIVITY_STATE,
          activityTrackingPanel: trackingLivePanelSnapshot(),
        },
        surface: 'product',
      })
    );

    expect(markup).toContain('>Retry status</button>');
    expect(markup).not.toContain('>Refresh tracking status</button>');
  });
}

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
    productClaim: trackingProductClaim,
  };
}

function trackingLivePanelSnapshot(): ParentTrackingStatusPanelSnapshot {
  return {
    eyebrow: 'Family tracking',
    title: 'Tracking status',
    body: 'Current child tracking history, service coverage, custody, and honest connection gaps from the local Rust service.',
    summaryCards: trackingLiveSummaryCards(),
    cards: trackingLiveCards(),
    emptyMessage: 'No tracking history has been reported by the local service yet.',
    productClaim: trackingProductClaim,
  };
}

function trackingLiveSummaryCards(): ParentTrackingStatusPanelSnapshot['summaryCards'] {
  return [
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
  ];
}

function trackingLiveCards(): ParentTrackingStatusPanelSnapshot['cards'] {
  return [
    {
      key: 'family-dashboard-rollup',
      title: 'Family dashboard tracking rollup',
      details: [
        { label: 'Status', value: 'ready' },
        { label: 'Visible devices', value: '1' },
      ],
    },
    {
      key: 'tracking-current-device',
      title: 'Current child status',
      details: [
        { label: 'Status', value: 'recent' },
        { label: 'Device', value: 'child-device-1' },
      ],
    },
    {
      key: 'tracking-location-surface',
      title: 'Last known location',
      details: [
        { label: 'Status', value: 'Unavailable' },
        { label: 'Accuracy', value: 'Not supplied' },
      ],
    },
    {
      key: 'tracking-event-coverage',
      title: 'Tracking activity coverage',
      details: [
        { label: 'Status', value: 'Reported' },
        { label: 'Expected-place states', value: '1 reported' },
      ],
    },
    {
      key: 'tracking-retention-custody',
      title: 'Custody and retention',
      details: [
        { label: 'Status', value: 'Read-only' },
        { label: 'Custody', value: 'child-device-query-store' },
      ],
    },
    {
      key: 'tracking-child-surface',
      title: 'Child tracking surface',
      details: [
        { label: 'Status', value: 'Unavailable' },
        { label: 'Authenticated delivery', value: 'Not supplied' },
      ],
    },
    {
      key: 'tracking-action-readiness',
      title: 'Tracking controls',
      details: [
        { label: 'Status', value: 'Unavailable' },
        { label: 'Exception editor', value: 'No owner-authorized mutation input' },
      ],
    },
    {
      key: 'tracking-citation-0',
      title: 'Expected-place status · citation 1',
      details: [
        { label: 'Observer source', value: 'tracking-engine' },
        { label: 'Subject ID', value: 'expected-place-school' },
        { label: 'Subject name', value: 'School' },
        { label: 'Capability status', value: 'recent' },
        { label: 'Query visibility', value: 'active' },
        { label: 'Deleted at', value: 'Not reported' },
        { label: 'Evidence refs', value: 'location-evidence-live-1' },
        { label: 'Deleted evidence refs', value: 'Not reported' },
      ],
    },
  ];
}

function trackingUnavailablePanelSnapshot(): ParentTrackingStatusPanelSnapshot {
  return {
    eyebrow: 'Family tracking',
    title: 'Tracking status',
    body: 'Current child tracking history, service coverage, custody, and honest connection gaps from the local Rust service.',
    summaryCards: [
      {
        key: 'tracking-live-summary',
        title: 'Tracking live summary',
        details: [
          { label: 'Status', value: 'Unavailable' },
          { label: 'Rows returned', value: '0' },
          { label: 'Last observed', value: 'Not reported' },
          { label: 'Event ID', value: 'Not reported' },
          { label: 'Capability', value: 'Unavailable' },
          { label: 'Custody', value: 'Unavailable' },
          { label: 'Evidence refs', value: 'Not reported' },
          { label: 'Product claim', value: trackingProductClaim },
        ],
      },
      {
        key: 'tracking-service-data-coverage',
        title: 'Tracking service data coverage',
        details: [
          { label: 'Status', value: 'Unavailable' },
          { label: 'Rows returned', value: '0' },
          { label: 'Deleted evidence', value: '0' },
          { label: 'Product claim', value: trackingProductClaim },
        ],
      },
    ],
    cards: [],
    emptyMessage: 'No tracking history has been reported by the local service yet.',
    productClaim: trackingProductClaim,
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

function trackingActionsWithoutRefresh() {
  return {
    reconnect: () => undefined,
    selectCommandResult: () => undefined,
    sendCommand: async () => null,
    requestTrackingRetentionSettingsWrite: async () => null,
  };
}
