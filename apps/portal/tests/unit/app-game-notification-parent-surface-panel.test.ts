import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import {
  ParentRoute,
  type ParentAppGameNotificationParentSurfacePanelSnapshot,
} from '../../generated/parent-ui-bridge';
import {
  AppGameNotificationParentSurfaceRoutePanel,
  shouldRenderAppGameNotificationParentSurfaceRoute,
} from '../../src/AppGameNotificationParentSurfaceRoutePanel';

const NotificationParentSurfacePanel: ParentAppGameNotificationParentSurfacePanelSnapshot = {
  eyebrow: 'Rust-owned panel',
  title: 'App/game notification parent surface',
  body: 'Rust-owned notification parent-surface rows are rendered directly in the portal.',
  state: 'manual-required',
  summary: '1 manual action',
  productClaim: 'provider delivery, preference mutation, child delivery, and runtime dispatch remain unclaimed',
  metrics: [
    { label: 'Rows returned', value: '2' },
    { label: 'Status', value: '1 manual action' },
    { label: 'History visibility', value: '2 history rows' },
  ],
  rows: [
    {
      key: 'app-game-notification-parent-surface-time-limit',
      title: 'app-game-notification-parent-surface-time-limit',
      details: [
        {
          label: 'Evidence references',
          value: 'provider-status-ref-time-limit, preference-result-time-limit',
        },
        {
          label: 'Runtime reference',
          value: 'scheduler-entry-app-game-time-limit, outbox-record-app-game-time-limit',
        },
      ],
    },
    {
      key: 'app-game-notification-parent-surface-unavailable',
      title: 'app-game-notification-parent-surface-unavailable',
      details: [{ label: 'Status', value: 'unavailable-visible' }],
    },
  ],
  emptyMessage: 'No app/game notification parent-surface panel has been reported yet.',
};

describe('app/game notification parent surface panel', () => {
  it('renders Rust-owned parent-surface panel rows without TS intent reconstruction', () => {
    const html = renderToStaticMarkup(
      createElement(AppGameNotificationParentSurfaceRoutePanel, { panel: NotificationParentSurfacePanel })
    );

    expect(html).toContain('App/game notification parent surface');
    expect(html).toContain('1 manual action');
    expect(html).toContain(
      'provider delivery, preference mutation, child delivery, and runtime dispatch remain unclaimed'
    );
    expect(html).toContain('app-game-notification-parent-surface-time-limit');
    expect(html).toContain('provider-status-ref-time-limit, preference-result-time-limit');
    expect(html).toContain('scheduler-entry-app-game-time-limit, outbox-record-app-game-time-limit');
    expect(html).toContain('app-game-notification-parent-surface-unavailable');
    expect(html).toContain('unavailable-visible');
  });

  it('keeps the Rust-owned empty state explicit when the panel snapshot is absent', () => {
    const html = renderToStaticMarkup(createElement(AppGameNotificationParentSurfaceRoutePanel, { panel: null }));

    expect(html).toContain('No app/game notification parent-surface panel has been reported yet.');
    expect(html).toContain('service event not reported');
  });

  it('mounts only on the App/Game Sessions route', () => {
    expect(shouldRenderAppGameNotificationParentSurfaceRoute(ParentRoute.AppGameSessions)).toBe(true);
    expect(shouldRenderAppGameNotificationParentSurfaceRoute(ParentRoute.Notifications)).toBe(false);
  });
});
